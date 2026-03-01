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
    timeout_seconds: f64,
    poll_interval_seconds: f64,
}

impl SyncConfig {
    /// Create a new `SyncConfig` with validated parameters.
    ///
    /// # Errors
    ///
    /// Returns [`MqRestError::InvalidConfig`] if either value is not positive.
    pub fn new(timeout_seconds: f64, poll_interval_seconds: f64) -> Result<Self> {
        if timeout_seconds <= 0.0 {
            return Err(MqRestError::InvalidConfig {
                message: format!(
                    "timeout_seconds must be positive, got {timeout_seconds}"
                ),
            });
        }
        if poll_interval_seconds <= 0.0 {
            return Err(MqRestError::InvalidConfig {
                message: format!(
                    "poll_interval_seconds must be positive, got {poll_interval_seconds}"
                ),
            });
        }
        Ok(Self {
            timeout_seconds,
            poll_interval_seconds,
        })
    }

    /// Maximum wall-clock seconds to wait for the target state.
    #[must_use]
    pub fn timeout_seconds(&self) -> f64 {
        self.timeout_seconds
    }

    /// Seconds to sleep between status polls.
    #[must_use]
    pub fn poll_interval_seconds(&self) -> f64 {
        self.poll_interval_seconds
    }
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
        thread::sleep(Duration::from_secs_f64(sync_config.poll_interval_seconds()));
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
        if elapsed >= sync_config.timeout_seconds() {
            return Err(MqRestError::Timeout {
                name: name.into(),
                operation: "start".into(),
                elapsed,
                message: format!(
                    "{} '{}' did not reach RUNNING within {}s",
                    obj_config.start_qualifier, name, sync_config.timeout_seconds()
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
        thread::sleep(Duration::from_secs_f64(sync_config.poll_interval_seconds()));
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
        if elapsed >= sync_config.timeout_seconds() {
            return Err(MqRestError::Timeout {
                name: name.into(),
                operation: "stop".into(),
                elapsed,
                message: format!(
                    "{} '{}' did not reach STOPPED within {}s",
                    obj_config.stop_qualifier, name, sync_config.timeout_seconds()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{
        MockTransport, empty_success_response, mock_session, success_response,
    };
    use serde_json::json;

    fn fast_config() -> SyncConfig {
        SyncConfig::new(0.5, 0.01).unwrap()
    }

    fn status_response(key: &str, value: &str) -> crate::transport::TransportResponse {
        let mut params = HashMap::new();
        params.insert(key.into(), json!(value));
        success_response(vec![params])
    }

    // ---- SyncConfig::default ----

    #[test]
    fn sync_config_default_values() {
        let config = SyncConfig::default();
        assert!((config.timeout_seconds() - 30.0).abs() < f64::EPSILON);
        assert!((config.poll_interval_seconds() - 1.0).abs() < f64::EPSILON);
    }

    // ---- has_status ----

    #[test]
    fn has_status_match_first_key() {
        let mut row = HashMap::new();
        row.insert("channel_status".into(), json!("RUNNING"));
        assert!(has_status(
            &[row],
            &["channel_status", "STATUS"],
            &["RUNNING"]
        ));
    }

    #[test]
    fn has_status_match_second_key() {
        let mut row = HashMap::new();
        row.insert("STATUS".into(), json!("STOPPED"));
        assert!(has_status(
            &[row],
            &["channel_status", "STATUS"],
            &["STOPPED"]
        ));
    }

    #[test]
    fn has_status_no_match() {
        let mut row = HashMap::new();
        row.insert("STATUS".into(), json!("STARTING"));
        assert!(!has_status(
            &[row],
            &["channel_status", "STATUS"],
            &["RUNNING"]
        ));
    }

    #[test]
    fn has_status_empty_rows() {
        assert!(!has_status(&[], &["STATUS"], &["RUNNING"]));
    }

    #[test]
    fn has_status_non_string_value() {
        let mut row = HashMap::new();
        row.insert("STATUS".into(), json!(42));
        assert!(!has_status(&[row], &["STATUS"], &["RUNNING"]));
    }

    // ---- start_channel_sync ----

    #[test]
    fn start_channel_sync_first_poll_running() {
        let transport = MockTransport::new(vec![
            empty_success_response(),
            status_response("channel_status", "RUNNING"),
        ]);
        let mut session = mock_session(transport);
        let result = session
            .start_channel_sync("MY.CH", Some(fast_config()))
            .unwrap();
        assert_eq!(result.operation, SyncOperation::Started);
        assert!(result.polls >= 1);
    }

    #[test]
    fn start_channel_sync_timeout() {
        let transport = MockTransport::new(vec![
            empty_success_response(),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
            status_response("channel_status", "STARTING"),
        ]);
        let mut session = mock_session(transport);
        let result = session.start_channel_sync("MY.CH", Some(fast_config()));
        assert!(format!("{:?}", result.unwrap_err()).starts_with("Timeout"));
    }

    // ---- stop_channel_sync ----

    #[test]
    fn stop_channel_sync_returns_stopped_via_status() {
        let transport = MockTransport::new(vec![
            empty_success_response(),
            status_response("STATUS", "STOPPED"),
        ]);
        let mut session = mock_session(transport);
        let result = session
            .stop_channel_sync("MY.CH", Some(fast_config()))
            .unwrap();
        assert_eq!(result.operation, SyncOperation::Stopped);
    }

    #[test]
    fn stop_channel_sync_empty_means_stopped() {
        let transport =
            MockTransport::new(vec![empty_success_response(), empty_success_response()]);
        let mut session = mock_session(transport);
        let result = session
            .stop_channel_sync("MY.CH", Some(fast_config()))
            .unwrap();
        assert_eq!(result.operation, SyncOperation::Stopped);
    }

    // ---- stop_listener_sync ----

    #[test]
    fn stop_listener_sync_empty_rows_not_stopped() {
        // Listeners have empty_means_stopped=false, so empty rows mean timeout
        let mut responses = vec![empty_success_response()]; // STOP command
        for _ in 0..60 {
            responses.push(empty_success_response()); // poll returns empty
        }
        let transport = MockTransport::new(responses);
        let mut session = mock_session(transport);
        let result = session.stop_listener_sync("MY.LIS", Some(fast_config()));
        assert!(format!("{:?}", result.unwrap_err()).starts_with("Timeout"));
    }

    #[test]
    fn stop_listener_sync_stopped_status() {
        let transport = MockTransport::new(vec![
            empty_success_response(),
            status_response("status", "STOPPED"),
        ]);
        let mut session = mock_session(transport);
        let result = session
            .stop_listener_sync("MY.LIS", Some(fast_config()))
            .unwrap();
        assert_eq!(result.operation, SyncOperation::Stopped);
    }

    // ---- restart_channel ----

    #[test]
    fn restart_channel_both_phases_succeed() {
        let transport = MockTransport::new(vec![
            empty_success_response(),                     // STOP
            empty_success_response(),                     // poll → empty (stopped for channel)
            empty_success_response(),                     // START
            status_response("channel_status", "RUNNING"), // poll → RUNNING
        ]);
        let mut session = mock_session(transport);
        let result = session
            .restart_channel("MY.CH", Some(fast_config()))
            .unwrap();
        assert_eq!(result.operation, SyncOperation::Restarted);
        assert!(result.polls >= 2);
    }

    #[test]
    fn restart_channel_stop_phase_fails() {
        let transport = MockTransport::new(vec![]);
        let mut session = mock_session(transport);
        let result = session.restart_channel("MY.CH", Some(fast_config()));
        assert!(result.is_err());
    }

    #[test]
    fn restart_channel_start_phase_fails() {
        let transport = MockTransport::new(vec![
            empty_success_response(), // STOP
            empty_success_response(), // poll → empty (stopped for channel)
                                      // START fails - no response
        ]);
        let mut session = mock_session(transport);
        let result = session.restart_channel("MY.CH", Some(fast_config()));
        assert!(result.is_err());
    }

    // ---- Macro-generated per-method tests ----

    macro_rules! test_start_sync {
        ($method:ident) => {
            paste::paste! {
                #[test]
                fn [<test_ $method _ok>]() {
                    let transport = MockTransport::new(vec![
                        empty_success_response(),
                        status_response("status", "RUNNING"),
                    ]);
                    let mut session = mock_session(transport);
                    let result = session.$method("OBJ", Some(fast_config())).unwrap();
                    assert_eq!(result.operation, SyncOperation::Started);
                }
            }
        };
    }

    macro_rules! test_stop_sync {
        ($method:ident) => {
            paste::paste! {
                #[test]
                fn [<test_ $method _ok>]() {
                    let transport = MockTransport::new(vec![
                        empty_success_response(),
                        status_response("status", "STOPPED"),
                    ]);
                    let mut session = mock_session(transport);
                    let result = session.$method("OBJ", Some(fast_config())).unwrap();
                    assert_eq!(result.operation, SyncOperation::Stopped);
                }
            }
        };
    }

    macro_rules! test_restart {
        ($method:ident) => {
            paste::paste! {
                #[test]
                fn [<test_ $method _ok>]() {
                    let transport = MockTransport::new(vec![
                        empty_success_response(),
                        status_response("status", "STOPPED"),
                        empty_success_response(),
                        status_response("status", "RUNNING"),
                    ]);
                    let mut session = mock_session(transport);
                    let result = session.$method("OBJ", Some(fast_config())).unwrap();
                    assert_eq!(result.operation, SyncOperation::Restarted);
                }
            }
        };
    }

    macro_rules! test_start_sync_channel {
        ($method:ident) => {
            paste::paste! {
                #[test]
                fn [<test_ $method _ok>]() {
                    let transport = MockTransport::new(vec![
                        empty_success_response(),
                        status_response("channel_status", "RUNNING"),
                    ]);
                    let mut session = mock_session(transport);
                    let result = session.$method("OBJ", Some(fast_config())).unwrap();
                    assert_eq!(result.operation, SyncOperation::Started);
                }
            }
        };
    }

    macro_rules! test_stop_sync_channel {
        ($method:ident) => {
            paste::paste! {
                #[test]
                fn [<test_ $method _ok>]() {
                    let transport = MockTransport::new(vec![
                        empty_success_response(),
                        status_response("channel_status", "STOPPED"),
                    ]);
                    let mut session = mock_session(transport);
                    let result = session.$method("OBJ", Some(fast_config())).unwrap();
                    assert_eq!(result.operation, SyncOperation::Stopped);
                }
            }
        };
    }

    macro_rules! test_restart_channel {
        ($method:ident) => {
            paste::paste! {
                #[test]
                fn [<test_ $method _ok>]() {
                    let transport = MockTransport::new(vec![
                        empty_success_response(),
                        status_response("channel_status", "STOPPED"),
                        empty_success_response(),
                        status_response("channel_status", "RUNNING"),
                    ]);
                    let mut session = mock_session(transport);
                    let result = session.$method("OBJ", Some(fast_config())).unwrap();
                    assert_eq!(result.operation, SyncOperation::Restarted);
                }
            }
        };
    }

    test_start_sync_channel!(start_channel_sync);
    test_start_sync!(start_listener_sync);
    test_start_sync!(start_service_sync);

    test_stop_sync_channel!(stop_channel_sync);
    test_stop_sync!(stop_listener_sync);
    test_stop_sync!(stop_service_sync);

    test_restart_channel!(restart_channel);
    test_restart!(restart_listener);
    test_restart!(restart_service);

    #[test]
    fn start_channel_sync_start_command_fails() {
        let transport = MockTransport::new(vec![]);
        let mut session = mock_session(transport);
        let result = session.start_channel_sync("MY.CH", Some(fast_config()));
        assert!(result.is_err());
    }

    #[test]
    fn start_channel_sync_poll_fails() {
        // START succeeds but poll DISPLAY fails
        let transport = MockTransport::new(vec![
            empty_success_response(), // START ok
                                      // poll fails - no response
        ]);
        let mut session = mock_session(transport);
        let result = session.start_channel_sync("MY.CH", Some(fast_config()));
        assert!(result.is_err());
    }

    #[test]
    fn stop_channel_sync_stop_command_fails() {
        let transport = MockTransport::new(vec![]);
        let mut session = mock_session(transport);
        let result = session.stop_channel_sync("MY.CH", Some(fast_config()));
        assert!(result.is_err());
    }

    #[test]
    fn stop_channel_sync_poll_fails() {
        let transport = MockTransport::new(vec![
            empty_success_response(), // STOP ok
                                      // poll fails - no response
        ]);
        let mut session = mock_session(transport);
        let result = session.stop_channel_sync("MY.CH", Some(fast_config()));
        assert!(result.is_err());
    }

    // ---- SyncConfig validation ----

    #[test]
    fn sync_config_new_valid() {
        let config = SyncConfig::new(10.0, 0.5).unwrap();
        assert!((config.timeout_seconds() - 10.0).abs() < f64::EPSILON);
        assert!((config.poll_interval_seconds() - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn sync_config_new_zero_timeout_rejected() {
        let err = SyncConfig::new(0.0, 1.0).unwrap_err();
        assert!(
            format!("{err}").contains("timeout_seconds must be positive"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn sync_config_new_negative_timeout_rejected() {
        let err = SyncConfig::new(-1.0, 1.0).unwrap_err();
        assert!(
            format!("{err}").contains("timeout_seconds must be positive"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn sync_config_new_zero_poll_interval_rejected() {
        let err = SyncConfig::new(30.0, 0.0).unwrap_err();
        assert!(
            format!("{err}").contains("poll_interval_seconds must be positive"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn sync_config_new_negative_poll_interval_rejected() {
        let err = SyncConfig::new(30.0, -1.0).unwrap_err();
        assert!(
            format!("{err}").contains("poll_interval_seconds must be positive"),
            "unexpected error: {err}"
        );
    }
}
