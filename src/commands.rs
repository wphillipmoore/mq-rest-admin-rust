//! MQSC command methods for `MqRestSession`.
#![allow(clippy::missing_errors_doc)]

use std::collections::HashMap;

use serde_json::Value;

use crate::error::Result;
use crate::session::MqRestSession;

impl MqRestSession {
    // BEGIN GENERATED MQSC METHODS
    /// Execute the MQSC `ALTER AUTHINFO` command.
    pub fn alter_authinfo(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "ALTER",
            "AUTHINFO",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `ALTER BUFFPOOL` command.
    pub fn alter_buffpool(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "ALTER",
            "BUFFPOOL",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `ALTER CFSTRUCT` command.
    pub fn alter_cfstruct(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "ALTER",
            "CFSTRUCT",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `ALTER CHANNEL` command.
    pub fn alter_channel(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "ALTER",
            "CHANNEL",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `ALTER COMMINFO` command.
    pub fn alter_comminfo(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "ALTER",
            "COMMINFO",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `ALTER LISTENER` command.
    pub fn alter_listener(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "ALTER",
            "LISTENER",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `ALTER NAMELIST` command.
    pub fn alter_namelist(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "ALTER",
            "NAMELIST",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `ALTER PROCESS` command.
    pub fn alter_process(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "ALTER",
            "PROCESS",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `ALTER PSID` command.
    pub fn alter_psid(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "ALTER",
            "PSID",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `ALTER QALIAS` command.
    pub fn alter_qalias(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "ALTER",
            "QALIAS",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `ALTER QLOCAL` command.
    pub fn alter_qlocal(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "ALTER",
            "QLOCAL",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `ALTER QMGR` command.
    pub fn alter_qmgr(
        &mut self,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "ALTER",
            "QMGR",
            None,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `ALTER QMODEL` command.
    pub fn alter_qmodel(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "ALTER",
            "QMODEL",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `ALTER QREMOTE` command.
    pub fn alter_qremote(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "ALTER",
            "QREMOTE",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `ALTER SECURITY` command.
    pub fn alter_security(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "ALTER",
            "SECURITY",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `ALTER SERVICE` command.
    pub fn alter_service(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "ALTER",
            "SERVICE",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `ALTER SMDS` command.
    pub fn alter_smds(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "ALTER",
            "SMDS",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `ALTER STGCLASS` command.
    pub fn alter_stgclass(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "ALTER",
            "STGCLASS",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `ALTER SUB` command.
    pub fn alter_sub(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "ALTER",
            "SUB",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `ALTER TOPIC` command.
    pub fn alter_topic(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "ALTER",
            "TOPIC",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `ALTER TRACE` command.
    pub fn alter_trace(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "ALTER",
            "TRACE",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `ARCHIVE LOG` command.
    pub fn archive_log(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "ARCHIVE",
            "LOG",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `BACKUP CFSTRUCT` command.
    pub fn backup_cfstruct(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "BACKUP",
            "CFSTRUCT",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `CLEAR QLOCAL` command.
    pub fn clear_qlocal(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "CLEAR",
            "QLOCAL",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `CLEAR TOPICSTR` command.
    pub fn clear_topicstr(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "CLEAR",
            "TOPICSTR",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DEFINE AUTHINFO` command.
    pub fn define_authinfo(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DEFINE",
            "AUTHINFO",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DEFINE BUFFPOOL` command.
    pub fn define_buffpool(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DEFINE",
            "BUFFPOOL",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DEFINE CFSTRUCT` command.
    pub fn define_cfstruct(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DEFINE",
            "CFSTRUCT",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DEFINE CHANNEL` command.
    pub fn define_channel(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DEFINE",
            "CHANNEL",
            Some(name),
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DEFINE COMMINFO` command.
    pub fn define_comminfo(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DEFINE",
            "COMMINFO",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DEFINE LISTENER` command.
    pub fn define_listener(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DEFINE",
            "LISTENER",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DEFINE LOG` command.
    pub fn define_log(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DEFINE",
            "LOG",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DEFINE MAXSMSGS` command.
    pub fn define_maxsmsgs(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DEFINE",
            "MAXSMSGS",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DEFINE NAMELIST` command.
    pub fn define_namelist(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DEFINE",
            "NAMELIST",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DEFINE PROCESS` command.
    pub fn define_process(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DEFINE",
            "PROCESS",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DEFINE PSID` command.
    pub fn define_psid(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DEFINE",
            "PSID",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DEFINE QALIAS` command.
    pub fn define_qalias(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DEFINE",
            "QALIAS",
            Some(name),
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DEFINE QLOCAL` command.
    pub fn define_qlocal(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DEFINE",
            "QLOCAL",
            Some(name),
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DEFINE QMODEL` command.
    pub fn define_qmodel(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DEFINE",
            "QMODEL",
            Some(name),
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DEFINE QREMOTE` command.
    pub fn define_qremote(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DEFINE",
            "QREMOTE",
            Some(name),
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DEFINE SERVICE` command.
    pub fn define_service(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DEFINE",
            "SERVICE",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DEFINE STGCLASS` command.
    pub fn define_stgclass(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DEFINE",
            "STGCLASS",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DEFINE SUB` command.
    pub fn define_sub(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DEFINE",
            "SUB",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DEFINE TOPIC` command.
    pub fn define_topic(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DEFINE",
            "TOPIC",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE AUTHINFO` command.
    pub fn delete_authinfo(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "AUTHINFO",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE AUTHREC` command.
    pub fn delete_authrec(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "AUTHREC",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE BUFFPOOL` command.
    pub fn delete_buffpool(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "BUFFPOOL",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE CFSTRUCT` command.
    pub fn delete_cfstruct(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "CFSTRUCT",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE CHANNEL` command.
    pub fn delete_channel(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "CHANNEL",
            Some(name),
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE COMMINFO` command.
    pub fn delete_comminfo(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "COMMINFO",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE LISTENER` command.
    pub fn delete_listener(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "LISTENER",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE NAMELIST` command.
    pub fn delete_namelist(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "NAMELIST",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE POLICY` command.
    pub fn delete_policy(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "POLICY",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE PROCESS` command.
    pub fn delete_process(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "PROCESS",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE PSID` command.
    pub fn delete_psid(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "PSID",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE QALIAS` command.
    pub fn delete_qalias(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "QALIAS",
            Some(name),
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE QLOCAL` command.
    pub fn delete_qlocal(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "QLOCAL",
            Some(name),
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE QMODEL` command.
    pub fn delete_qmodel(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "QMODEL",
            Some(name),
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE QREMOTE` command.
    pub fn delete_qremote(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "QREMOTE",
            Some(name),
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE QUEUE` command.
    pub fn delete_queue(
        &mut self,
        name: &str,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "QUEUE",
            Some(name),
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE SERVICE` command.
    pub fn delete_service(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "SERVICE",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE STGCLASS` command.
    pub fn delete_stgclass(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "STGCLASS",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE SUB` command.
    pub fn delete_sub(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "SUB",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE TOPIC` command.
    pub fn delete_topic(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "TOPIC",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DISPLAY APSTATUS` command.
    pub fn display_apstatus(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "APSTATUS",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY ARCHIVE` command.
    pub fn display_archive(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "ARCHIVE",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY AUTHINFO` command.
    pub fn display_authinfo(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "AUTHINFO",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY AUTHREC` command.
    pub fn display_authrec(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "AUTHREC",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY AUTHSERV` command.
    pub fn display_authserv(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "AUTHSERV",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY CFSTATUS` command.
    pub fn display_cfstatus(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "CFSTATUS",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY CFSTRUCT` command.
    pub fn display_cfstruct(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "CFSTRUCT",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY CHANNEL` command.
    pub fn display_channel(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "CHANNEL",
            Some(name.unwrap_or("*")),
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY CHINIT` command.
    pub fn display_chinit(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "CHINIT",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY CHLAUTH` command.
    pub fn display_chlauth(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "CHLAUTH",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY CHSTATUS` command.
    pub fn display_chstatus(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "CHSTATUS",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY CLUSQMGR` command.
    pub fn display_clusqmgr(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "CLUSQMGR",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY CMDSERV` command.
    pub fn display_cmdserv(
        &mut self,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<Option<HashMap<String, Value>>> {
        let objects = self.mqsc_command(
            "DISPLAY",
            "CMDSERV",
            None,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(objects.into_iter().next())
    }

    /// Execute the MQSC `DISPLAY COMMINFO` command.
    pub fn display_comminfo(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "COMMINFO",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY CONN` command.
    pub fn display_conn(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "CONN",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY ENTAUTH` command.
    pub fn display_entauth(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "ENTAUTH",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY GROUP` command.
    pub fn display_group(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "GROUP",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY LISTENER` command.
    pub fn display_listener(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "LISTENER",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY LOG` command.
    pub fn display_log(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "LOG",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY LSSTATUS` command.
    pub fn display_lsstatus(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "LSSTATUS",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY MAXSMSGS` command.
    pub fn display_maxsmsgs(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "MAXSMSGS",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY NAMELIST` command.
    pub fn display_namelist(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "NAMELIST",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY POLICY` command.
    pub fn display_policy(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "POLICY",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY PROCESS` command.
    pub fn display_process(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "PROCESS",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY PUBSUB` command.
    pub fn display_pubsub(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "PUBSUB",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY QMGR` command.
    pub fn display_qmgr(
        &mut self,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<Option<HashMap<String, Value>>> {
        let objects = self.mqsc_command(
            "DISPLAY",
            "QMGR",
            None,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(objects.into_iter().next())
    }

    /// Execute the MQSC `DISPLAY QMSTATUS` command.
    pub fn display_qmstatus(
        &mut self,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<Option<HashMap<String, Value>>> {
        let objects = self.mqsc_command(
            "DISPLAY",
            "QMSTATUS",
            None,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(objects.into_iter().next())
    }

    /// Execute the MQSC `DISPLAY QSTATUS` command.
    pub fn display_qstatus(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "QSTATUS",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY QUEUE` command.
    pub fn display_queue(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "QUEUE",
            Some(name.unwrap_or("*")),
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY SBSTATUS` command.
    pub fn display_sbstatus(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "SBSTATUS",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY SECURITY` command.
    pub fn display_security(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "SECURITY",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY SERVICE` command.
    pub fn display_service(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "SERVICE",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY SMDS` command.
    pub fn display_smds(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "SMDS",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY SMDSCONN` command.
    pub fn display_smdsconn(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "SMDSCONN",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY STGCLASS` command.
    pub fn display_stgclass(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "STGCLASS",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY SUB` command.
    pub fn display_sub(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "SUB",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY SVSTATUS` command.
    pub fn display_svstatus(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "SVSTATUS",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY SYSTEM` command.
    pub fn display_system(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "SYSTEM",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY TCLUSTER` command.
    pub fn display_tcluster(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "TCLUSTER",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY THREAD` command.
    pub fn display_thread(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "THREAD",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY TOPIC` command.
    pub fn display_topic(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "TOPIC",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY TPSTATUS` command.
    pub fn display_tpstatus(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "TPSTATUS",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY TRACE` command.
    pub fn display_trace(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "TRACE",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `DISPLAY USAGE` command.
    pub fn display_usage(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
        where_clause: Option<&str>,
    ) -> Result<Vec<HashMap<String, Value>>> {
        self.mqsc_command(
            "DISPLAY",
            "USAGE",
            name,
            request_parameters,
            response_parameters,
            where_clause,
        )
    }

    /// Execute the MQSC `MOVE QLOCAL` command.
    pub fn move_qlocal(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "MOVE",
            "QLOCAL",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `PING CHANNEL` command.
    pub fn ping_channel(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "PING",
            "CHANNEL",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `PING QMGR` command.
    pub fn ping_qmgr(
        &mut self,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "PING",
            "QMGR",
            None,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `PURGE CHANNEL` command.
    pub fn purge_channel(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "PURGE",
            "CHANNEL",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `RECOVER BSDS` command.
    pub fn recover_bsds(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "RECOVER",
            "BSDS",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `RECOVER CFSTRUCT` command.
    pub fn recover_cfstruct(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "RECOVER",
            "CFSTRUCT",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `REFRESH CLUSTER` command.
    pub fn refresh_cluster(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "REFRESH",
            "CLUSTER",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `REFRESH QMGR` command.
    pub fn refresh_qmgr(
        &mut self,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "REFRESH",
            "QMGR",
            None,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `REFRESH SECURITY` command.
    pub fn refresh_security(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "REFRESH",
            "SECURITY",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `RESET CFSTRUCT` command.
    pub fn reset_cfstruct(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "RESET",
            "CFSTRUCT",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `RESET CHANNEL` command.
    pub fn reset_channel(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "RESET",
            "CHANNEL",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `RESET CLUSTER` command.
    pub fn reset_cluster(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "RESET",
            "CLUSTER",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `RESET QMGR` command.
    pub fn reset_qmgr(
        &mut self,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "RESET",
            "QMGR",
            None,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `RESET QSTATS` command.
    pub fn reset_qstats(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "RESET",
            "QSTATS",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `RESET SMDS` command.
    pub fn reset_smds(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "RESET",
            "SMDS",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `RESET TPIPE` command.
    pub fn reset_tpipe(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "RESET",
            "TPIPE",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `RESOLVE CHANNEL` command.
    pub fn resolve_channel(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "RESOLVE",
            "CHANNEL",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `RESOLVE INDOUBT` command.
    pub fn resolve_indoubt(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "RESOLVE",
            "INDOUBT",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `RESUME QMGR` command.
    pub fn resume_qmgr(
        &mut self,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "RESUME",
            "QMGR",
            None,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `RVERIFY SECURITY` command.
    pub fn rverify_security(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "RVERIFY",
            "SECURITY",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `SET ARCHIVE` command.
    pub fn set_archive(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "SET",
            "ARCHIVE",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `SET AUTHREC` command.
    pub fn set_authrec(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "SET",
            "AUTHREC",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `SET CHLAUTH` command.
    pub fn set_chlauth(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "SET",
            "CHLAUTH",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `SET LOG` command.
    pub fn set_log(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "SET",
            "LOG",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `SET POLICY` command.
    pub fn set_policy(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "SET",
            "POLICY",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `SET SYSTEM` command.
    pub fn set_system(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "SET",
            "SYSTEM",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `START CHANNEL` command.
    pub fn start_channel(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "START",
            "CHANNEL",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `START CHINIT` command.
    pub fn start_chinit(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "START",
            "CHINIT",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `START CMDSERV` command.
    pub fn start_cmdserv(
        &mut self,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "START",
            "CMDSERV",
            None,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `START LISTENER` command.
    pub fn start_listener(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "START",
            "LISTENER",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `START QMGR` command.
    pub fn start_qmgr(
        &mut self,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "START",
            "QMGR",
            None,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `START SERVICE` command.
    pub fn start_service(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "START",
            "SERVICE",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `START SMDSCONN` command.
    pub fn start_smdsconn(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "START",
            "SMDSCONN",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `START TRACE` command.
    pub fn start_trace(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "START",
            "TRACE",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `STOP CHANNEL` command.
    pub fn stop_channel(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "STOP",
            "CHANNEL",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `STOP CHINIT` command.
    pub fn stop_chinit(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "STOP",
            "CHINIT",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `STOP CMDSERV` command.
    pub fn stop_cmdserv(
        &mut self,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "STOP",
            "CMDSERV",
            None,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `STOP CONN` command.
    pub fn stop_conn(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "STOP",
            "CONN",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `STOP LISTENER` command.
    pub fn stop_listener(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "STOP",
            "LISTENER",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `STOP QMGR` command.
    pub fn stop_qmgr(
        &mut self,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "STOP",
            "QMGR",
            None,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `STOP SERVICE` command.
    pub fn stop_service(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "STOP",
            "SERVICE",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `STOP SMDSCONN` command.
    pub fn stop_smdsconn(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "STOP",
            "SMDSCONN",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `STOP TRACE` command.
    pub fn stop_trace(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "STOP",
            "TRACE",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `SUSPEND QMGR` command.
    pub fn suspend_qmgr(
        &mut self,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "SUSPEND",
            "QMGR",
            None,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    // END GENERATED MQSC METHODS
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{
        MockTransport, empty_success_response, mock_session, success_response,
    };
    use serde_json::json;

    // -----------------------------------------------------------------
    // Macro: Pattern 1 — Singleton DISPLAY (returns Option)
    // -----------------------------------------------------------------
    macro_rules! test_singleton_display {
        ($method:ident) => {
            paste::paste! {
                #[test]
                fn [<test_ $method _returns_option>]() {
                    let transport = MockTransport::new(vec![empty_success_response()]);
                    let mut session = mock_session(transport);
                    let result = session.$method(None, None).unwrap();
                    assert!(result.is_none());
                }

                #[test]
                fn [<test_ $method _error_propagates>]() {
                    let transport = MockTransport::new(vec![]);
                    let mut session = mock_session(transport);
                    assert!(session.$method(None, None).is_err());
                }
            }
        };
    }

    test_singleton_display!(display_qmgr);
    test_singleton_display!(display_qmstatus);
    test_singleton_display!(display_cmdserv);

    // -----------------------------------------------------------------
    // Macro: Pattern 2 — List DISPLAY (returns Vec)
    // -----------------------------------------------------------------
    macro_rules! test_list_display {
        ($method:ident) => {
            paste::paste! {
                #[test]
                fn [<test_ $method _returns_vec>]() {
                    let transport = MockTransport::new(vec![empty_success_response()]);
                    let mut session = mock_session(transport);
                    let result = session.$method(None, None, None, None).unwrap();
                    assert!(result.is_empty());
                }

                #[test]
                fn [<test_ $method _error_propagates>]() {
                    let transport = MockTransport::new(vec![]);
                    let mut session = mock_session(transport);
                    assert!(session.$method(None, None, None, None).is_err());
                }
            }
        };
    }

    test_list_display!(display_queue);
    test_list_display!(display_channel);
    test_list_display!(display_apstatus);
    test_list_display!(display_archive);
    test_list_display!(display_authinfo);
    test_list_display!(display_authrec);
    test_list_display!(display_authserv);
    test_list_display!(display_cfstatus);
    test_list_display!(display_cfstruct);
    test_list_display!(display_chinit);
    test_list_display!(display_chlauth);
    test_list_display!(display_chstatus);
    test_list_display!(display_clusqmgr);
    test_list_display!(display_comminfo);
    test_list_display!(display_conn);
    test_list_display!(display_entauth);
    test_list_display!(display_group);
    test_list_display!(display_listener);
    test_list_display!(display_log);
    test_list_display!(display_lsstatus);
    test_list_display!(display_maxsmsgs);
    test_list_display!(display_namelist);
    test_list_display!(display_policy);
    test_list_display!(display_process);
    test_list_display!(display_pubsub);
    test_list_display!(display_qstatus);
    test_list_display!(display_sbstatus);
    test_list_display!(display_security);
    test_list_display!(display_service);
    test_list_display!(display_smds);
    test_list_display!(display_smdsconn);
    test_list_display!(display_stgclass);
    test_list_display!(display_sub);
    test_list_display!(display_svstatus);
    test_list_display!(display_system);
    test_list_display!(display_tcluster);
    test_list_display!(display_thread);
    test_list_display!(display_topic);
    test_list_display!(display_tpstatus);
    test_list_display!(display_trace);
    test_list_display!(display_usage);

    // -----------------------------------------------------------------
    // Macro: Pattern 3a — Mutating with required name (&str)
    // -----------------------------------------------------------------
    macro_rules! test_mutating_required_name {
        ($method:ident) => {
            paste::paste! {
                #[test]
                fn [<test_ $method _ok>]() {
                    let transport = MockTransport::new(vec![empty_success_response()]);
                    let mut session = mock_session(transport);
                    session.$method("OBJ1", None, None).unwrap();
                }

                #[test]
                fn [<test_ $method _error_propagates>]() {
                    let transport = MockTransport::new(vec![]);
                    let mut session = mock_session(transport);
                    assert!(session.$method("OBJ1", None, None).is_err());
                }
            }
        };
    }

    test_mutating_required_name!(define_qlocal);
    test_mutating_required_name!(define_qremote);
    test_mutating_required_name!(define_qalias);
    test_mutating_required_name!(define_qmodel);
    test_mutating_required_name!(define_channel);
    test_mutating_required_name!(delete_channel);
    test_mutating_required_name!(delete_qalias);
    test_mutating_required_name!(delete_qlocal);
    test_mutating_required_name!(delete_qmodel);
    test_mutating_required_name!(delete_qremote);
    test_mutating_required_name!(delete_queue);

    // -----------------------------------------------------------------
    // Macro: Pattern 3a — Mutating with optional name (Option<&str>)
    // -----------------------------------------------------------------
    macro_rules! test_mutating_optional_name {
        ($method:ident) => {
            paste::paste! {
                #[test]
                fn [<test_ $method _ok>]() {
                    let transport = MockTransport::new(vec![empty_success_response()]);
                    let mut session = mock_session(transport);
                    session.$method(Some("OBJ1"), None, None).unwrap();
                }

                #[test]
                fn [<test_ $method _error_propagates>]() {
                    let transport = MockTransport::new(vec![]);
                    let mut session = mock_session(transport);
                    assert!(session.$method(Some("OBJ1"), None, None).is_err());
                }
            }
        };
    }

    test_mutating_optional_name!(alter_authinfo);
    test_mutating_optional_name!(alter_buffpool);
    test_mutating_optional_name!(alter_cfstruct);
    test_mutating_optional_name!(alter_channel);
    test_mutating_optional_name!(alter_comminfo);
    test_mutating_optional_name!(alter_listener);
    test_mutating_optional_name!(alter_namelist);
    test_mutating_optional_name!(alter_process);
    test_mutating_optional_name!(alter_psid);
    test_mutating_optional_name!(alter_security);
    test_mutating_optional_name!(alter_service);
    test_mutating_optional_name!(alter_smds);
    test_mutating_optional_name!(alter_stgclass);
    test_mutating_optional_name!(alter_sub);
    test_mutating_optional_name!(alter_topic);
    test_mutating_optional_name!(alter_qalias);
    test_mutating_optional_name!(alter_qlocal);
    test_mutating_optional_name!(alter_qmodel);
    test_mutating_optional_name!(alter_qremote);
    test_mutating_optional_name!(alter_trace);
    test_mutating_optional_name!(archive_log);
    test_mutating_optional_name!(backup_cfstruct);
    test_mutating_optional_name!(clear_qlocal);
    test_mutating_optional_name!(clear_topicstr);
    test_mutating_optional_name!(define_authinfo);
    test_mutating_optional_name!(define_buffpool);
    test_mutating_optional_name!(define_cfstruct);
    test_mutating_optional_name!(define_comminfo);
    test_mutating_optional_name!(define_listener);
    test_mutating_optional_name!(define_log);
    test_mutating_optional_name!(define_maxsmsgs);
    test_mutating_optional_name!(define_namelist);
    test_mutating_optional_name!(define_process);
    test_mutating_optional_name!(define_psid);
    test_mutating_optional_name!(define_service);
    test_mutating_optional_name!(define_stgclass);
    test_mutating_optional_name!(define_sub);
    test_mutating_optional_name!(define_topic);
    test_mutating_optional_name!(delete_authinfo);
    test_mutating_optional_name!(delete_authrec);
    test_mutating_optional_name!(delete_buffpool);
    test_mutating_optional_name!(delete_cfstruct);
    test_mutating_optional_name!(delete_comminfo);
    test_mutating_optional_name!(delete_listener);
    test_mutating_optional_name!(delete_namelist);
    test_mutating_optional_name!(delete_policy);
    test_mutating_optional_name!(delete_process);
    test_mutating_optional_name!(delete_psid);
    test_mutating_optional_name!(delete_service);
    test_mutating_optional_name!(delete_stgclass);
    test_mutating_optional_name!(delete_sub);
    test_mutating_optional_name!(delete_topic);
    test_mutating_optional_name!(move_qlocal);
    test_mutating_optional_name!(ping_channel);
    test_mutating_optional_name!(purge_channel);
    test_mutating_optional_name!(recover_bsds);
    test_mutating_optional_name!(recover_cfstruct);
    test_mutating_optional_name!(refresh_cluster);
    test_mutating_optional_name!(refresh_security);
    test_mutating_optional_name!(reset_cfstruct);
    test_mutating_optional_name!(reset_channel);
    test_mutating_optional_name!(reset_cluster);
    test_mutating_optional_name!(reset_qstats);
    test_mutating_optional_name!(reset_smds);
    test_mutating_optional_name!(reset_tpipe);
    test_mutating_optional_name!(resolve_channel);
    test_mutating_optional_name!(resolve_indoubt);
    test_mutating_optional_name!(rverify_security);
    test_mutating_optional_name!(set_archive);
    test_mutating_optional_name!(set_authrec);
    test_mutating_optional_name!(set_chlauth);
    test_mutating_optional_name!(set_log);
    test_mutating_optional_name!(set_policy);
    test_mutating_optional_name!(set_system);
    test_mutating_optional_name!(start_channel);
    test_mutating_optional_name!(start_chinit);
    test_mutating_optional_name!(start_listener);
    test_mutating_optional_name!(start_service);
    test_mutating_optional_name!(start_smdsconn);
    test_mutating_optional_name!(start_trace);
    test_mutating_optional_name!(stop_channel);
    test_mutating_optional_name!(stop_chinit);
    test_mutating_optional_name!(stop_conn);
    test_mutating_optional_name!(stop_listener);
    test_mutating_optional_name!(stop_service);
    test_mutating_optional_name!(stop_smdsconn);
    test_mutating_optional_name!(stop_trace);

    // -----------------------------------------------------------------
    // Macro: Pattern 3b — Mutating without name
    // -----------------------------------------------------------------
    macro_rules! test_mutating_no_name {
        ($method:ident) => {
            paste::paste! {
                #[test]
                fn [<test_ $method _ok>]() {
                    let transport = MockTransport::new(vec![empty_success_response()]);
                    let mut session = mock_session(transport);
                    session.$method(None, None).unwrap();
                }

                #[test]
                fn [<test_ $method _error_propagates>]() {
                    let transport = MockTransport::new(vec![]);
                    let mut session = mock_session(transport);
                    assert!(session.$method(None, None).is_err());
                }
            }
        };
    }

    test_mutating_no_name!(alter_qmgr);
    test_mutating_no_name!(ping_qmgr);
    test_mutating_no_name!(refresh_qmgr);
    test_mutating_no_name!(reset_qmgr);
    test_mutating_no_name!(resume_qmgr);
    test_mutating_no_name!(start_cmdserv);
    test_mutating_no_name!(start_qmgr);
    test_mutating_no_name!(stop_cmdserv);
    test_mutating_no_name!(stop_qmgr);
    test_mutating_no_name!(suspend_qmgr);

    // -----------------------------------------------------------------
    // Hand-crafted tests for command/qualifier verification
    // -----------------------------------------------------------------

    #[test]
    fn display_queue_sends_correct_command() {
        let mut params = HashMap::new();
        params.insert("DESCR".into(), json!("test"));
        let transport = MockTransport::new(vec![success_response(vec![params])]);
        let mut session = mock_session(transport);
        session.display_queue(Some("Q1"), None, None, None).unwrap();
        let payload = session.last_command_payload.unwrap();
        assert_eq!(payload["command"], json!("DISPLAY"));
        assert_eq!(payload["qualifier"], json!("QUEUE"));
        assert_eq!(payload["name"], json!("Q1"));
    }

    #[test]
    fn define_qlocal_sends_correct_command() {
        let transport = MockTransport::new(vec![empty_success_response()]);
        let mut session = mock_session(transport);
        session.define_qlocal("MY.Q", None, None).unwrap();
        let payload = session.last_command_payload.unwrap();
        assert_eq!(payload["command"], json!("DEFINE"));
        assert_eq!(payload["qualifier"], json!("QLOCAL"));
        assert_eq!(payload["name"], json!("MY.Q"));
    }

    #[test]
    fn alter_qmgr_sends_correct_command() {
        let transport = MockTransport::new(vec![empty_success_response()]);
        let mut session = mock_session(transport);
        session.alter_qmgr(None, None).unwrap();
        let payload = session.last_command_payload.unwrap();
        assert_eq!(payload["command"], json!("ALTER"));
        assert_eq!(payload["qualifier"], json!("QMGR"));
        assert!(!payload.contains_key("name"));
    }

    #[test]
    fn display_queue_default_name_wildcard() {
        let transport = MockTransport::new(vec![empty_success_response()]);
        let mut session = mock_session(transport);
        session.display_queue(None, None, None, None).unwrap();
        let payload = session.last_command_payload.unwrap();
        assert_eq!(payload["name"], json!("*"));
    }

    #[test]
    fn singleton_display_returns_some_when_present() {
        let mut params = HashMap::new();
        params.insert("QMNAME".into(), json!("QM1"));
        let transport = MockTransport::new(vec![success_response(vec![params])]);
        let mut session = mock_session(transport);
        let result = session.display_qmgr(None, None).unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap()["QMNAME"], json!("QM1"));
    }
}
