//! Synchronous start/stop/restart wrappers for MQ objects.

use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};

use serde_json::Value;

use crate::error::{MqRestError, Result};
use crate::session::MqRestSession;

/// Configuration for synchronous polling operations.
#[derive(Debug, Clone, Copy)]
pub struct SyncConfig {
    /// Maximum wall-clock seconds to wait for the target state.
    pub timeout_seconds: f64,
    /// Seconds to sleep between status polls.
    pub poll_interval_seconds: f64,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30.0,
            poll_interval_seconds: 1.0,
        }
    }
}

/// Operation performed by a synchronous wrapper.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncOperation {
    /// The object was started and confirmed running.
    Started,
    /// The object was stopped and confirmed stopped.
    Stopped,
    /// The object was stopped then started.
    Restarted,
}

/// Result of a synchronous start/stop/restart operation.
#[derive(Debug, Clone)]
pub struct SyncResult {
    /// The operation that was performed.
    pub operation: SyncOperation,
    /// Total number of status polls issued.
    pub polls: u32,
    /// Total wall-clock seconds from command to target state confirmation.
    pub elapsed_seconds: f64,
}

struct ObjectTypeConfig {
    start_qualifier: &'static str,
    stop_qualifier: &'static str,
    status_qualifier: &'static str,
    status_keys: &'static [&'static str],
    empty_means_stopped: bool,
}

const CHANNEL_CONFIG: ObjectTypeConfig = ObjectTypeConfig {
    start_qualifier: "CHANNEL",
    stop_qualifier: "CHANNEL",
    status_qualifier: "CHSTATUS",
    status_keys: &["channel_status", "STATUS"],
    empty_means_stopped: true,
};

const LISTENER_CONFIG: ObjectTypeConfig = ObjectTypeConfig {
    start_qualifier: "LISTENER",
    stop_qualifier: "LISTENER",
    status_qualifier: "LSSTATUS",
    status_keys: &["status", "STATUS"],
    empty_means_stopped: false,
};

const SERVICE_CONFIG: ObjectTypeConfig = ObjectTypeConfig {
    start_qualifier: "SERVICE",
    stop_qualifier: "SERVICE",
    status_qualifier: "SVSTATUS",
    status_keys: &["status", "STATUS"],
    empty_means_stopped: false,
};

const RUNNING_VALUES: &[&str] = &["RUNNING", "running"];
const STOPPED_VALUES: &[&str] = &["STOPPED", "stopped"];

impl MqRestSession {
    // ---- Channel ----

    /// Start a channel and wait until it is running.
    ///
    /// # Errors
    ///
    /// Returns an error if the START command fails or the channel does not
    /// reach RUNNING within the timeout.
    pub fn start_channel_sync(
        &mut self,
        name: &str,
        config: Option<SyncConfig>,
    ) -> Result<SyncResult> {
        start_and_poll(self, name, &CHANNEL_CONFIG, config)
    }

    /// Stop a channel and wait until it is stopped.
    ///
    /// # Errors
    ///
    /// Returns an error if the STOP command fails or the channel does not
    /// reach STOPPED within the timeout.
    pub fn stop_channel_sync(
        &mut self,
        name: &str,
        config: Option<SyncConfig>,
    ) -> Result<SyncResult> {
        stop_and_poll(self, name, &CHANNEL_CONFIG, config)
    }

    /// Stop then start a channel, waiting for each phase.
    ///
    /// # Errors
    ///
    /// Returns an error if either the stop or start phase fails or times out.
    pub fn restart_channel(
        &mut self,
        name: &str,
        config: Option<SyncConfig>,
    ) -> Result<SyncResult> {
        restart(self, name, &CHANNEL_CONFIG, config)
    }

    // ---- Listener ----

    /// Start a listener and wait until it is running.
    ///
    /// # Errors
    ///
    /// Returns an error if the START command fails or the listener does not
    /// reach RUNNING within the timeout.
    pub fn start_listener_sync(
        &mut self,
        name: &str,
        config: Option<SyncConfig>,
    ) -> Result<SyncResult> {
        start_and_poll(self, name, &LISTENER_CONFIG, config)
    }

    /// Stop a listener and wait until it is stopped.
    ///
    /// # Errors
    ///
    /// Returns an error if the STOP command fails or the listener does not
    /// reach STOPPED within the timeout.
    pub fn stop_listener_sync(
        &mut self,
        name: &str,
        config: Option<SyncConfig>,
    ) -> Result<SyncResult> {
        stop_and_poll(self, name, &LISTENER_CONFIG, config)
    }

    /// Stop then start a listener, waiting for each phase.
    ///
    /// # Errors
    ///
    /// Returns an error if either the stop or start phase fails or times out.
    pub fn restart_listener(
        &mut self,
        name: &str,
        config: Option<SyncConfig>,
    ) -> Result<SyncResult> {
        restart(self, name, &LISTENER_CONFIG, config)
    }

    // ---- Service ----

    /// Start a service and wait until it is running.
    ///
    /// # Errors
    ///
    /// Returns an error if the START command fails or the service does not
    /// reach RUNNING within the timeout.
    pub fn start_service_sync(
        &mut self,
        name: &str,
        config: Option<SyncConfig>,
    ) -> Result<SyncResult> {
        start_and_poll(self, name, &SERVICE_CONFIG, config)
    }

    /// Stop a service and wait until it is stopped.
    ///
    /// # Errors
    ///
    /// Returns an error if the STOP command fails or the service does not
    /// reach STOPPED within the timeout.
    pub fn stop_service_sync(
        &mut self,
        name: &str,
        config: Option<SyncConfig>,
    ) -> Result<SyncResult> {
        stop_and_poll(self, name, &SERVICE_CONFIG, config)
    }

    /// Stop then start a service, waiting for each phase.
    ///
    /// # Errors
    ///
    /// Returns an error if either the stop or start phase fails or times out.
    pub fn restart_service(
        &mut self,
        name: &str,
        config: Option<SyncConfig>,
    ) -> Result<SyncResult> {
        restart(self, name, &SERVICE_CONFIG, config)
    }
}

fn start_and_poll(
    session: &mut MqRestSession,
    name: &str,
    obj_config: &ObjectTypeConfig,
    config: Option<SyncConfig>,
) -> Result<SyncResult> {
    let sync_config = config.unwrap_or_default();
    session.mqsc_command(
        "START",
        obj_config.start_qualifier,
        Some(name),
        None,
        None,
        None,
    )?;
    let mut polls = 0u32;
    let start_time = Instant::now();
    loop {
        thread::sleep(Duration::from_secs_f64(sync_config.poll_interval_seconds));
        let all_params: &[&str] = &["all"];
        let status_rows = session.mqsc_command(
            "DISPLAY",
            obj_config.status_qualifier,
            Some(name),
            None,
            Some(all_params),
            None,
        )?;
        polls += 1;
        if has_status(&status_rows, obj_config.status_keys, RUNNING_VALUES) {
            let elapsed = start_time.elapsed().as_secs_f64();
            return Ok(SyncResult {
                operation: SyncOperation::Started,
                polls,
                elapsed_seconds: elapsed,
            });
        }
        let elapsed = start_time.elapsed().as_secs_f64();
        if elapsed >= sync_config.timeout_seconds {
            return Err(MqRestError::Timeout {
                name: name.into(),
                operation: "start".into(),
                elapsed,
                message: format!(
                    "{} '{}' did not reach RUNNING within {}s",
                    obj_config.start_qualifier, name, sync_config.timeout_seconds
                ),
            });
        }
    }
}

fn stop_and_poll(
    session: &mut MqRestSession,
    name: &str,
    obj_config: &ObjectTypeConfig,
    config: Option<SyncConfig>,
) -> Result<SyncResult> {
    let sync_config = config.unwrap_or_default();
    session.mqsc_command(
        "STOP",
        obj_config.stop_qualifier,
        Some(name),
        None,
        None,
        None,
    )?;
    let mut polls = 0u32;
    let start_time = Instant::now();
    loop {
        thread::sleep(Duration::from_secs_f64(sync_config.poll_interval_seconds));
        let all_params: &[&str] = &["all"];
        let status_rows = session.mqsc_command(
            "DISPLAY",
            obj_config.status_qualifier,
            Some(name),
            None,
            Some(all_params),
            None,
        )?;
        polls += 1;
        if obj_config.empty_means_stopped && status_rows.is_empty() {
            let elapsed = start_time.elapsed().as_secs_f64();
            return Ok(SyncResult {
                operation: SyncOperation::Stopped,
                polls,
                elapsed_seconds: elapsed,
            });
        }
        if has_status(&status_rows, obj_config.status_keys, STOPPED_VALUES) {
            let elapsed = start_time.elapsed().as_secs_f64();
            return Ok(SyncResult {
                operation: SyncOperation::Stopped,
                polls,
                elapsed_seconds: elapsed,
            });
        }
        let elapsed = start_time.elapsed().as_secs_f64();
        if elapsed >= sync_config.timeout_seconds {
            return Err(MqRestError::Timeout {
                name: name.into(),
                operation: "stop".into(),
                elapsed,
                message: format!(
                    "{} '{}' did not reach STOPPED within {}s",
                    obj_config.stop_qualifier, name, sync_config.timeout_seconds
                ),
            });
        }
    }
}

fn restart(
    session: &mut MqRestSession,
    name: &str,
    obj_config: &ObjectTypeConfig,
    config: Option<SyncConfig>,
) -> Result<SyncResult> {
    let stop_result = stop_and_poll(session, name, obj_config, config)?;
    let start_result = start_and_poll(session, name, obj_config, config)?;
    Ok(SyncResult {
        operation: SyncOperation::Restarted,
        polls: stop_result.polls + start_result.polls,
        elapsed_seconds: stop_result.elapsed_seconds + start_result.elapsed_seconds,
    })
}

fn has_status(
    rows: &[HashMap<String, Value>],
    status_keys: &[&str],
    target_values: &[&str],
) -> bool {
    for row in rows {
        for key in status_keys {
            if let Some(Value::String(value)) = row.get(*key)
                && target_values.contains(&value.as_str())
            {
                return true;
            }
        }
    }
    false
}
