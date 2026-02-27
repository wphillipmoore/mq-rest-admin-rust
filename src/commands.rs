//! MQSC command methods for MqRestSession.

use std::collections::HashMap;

use serde_json::Value;

use crate::error::Result;
use crate::session::MqRestSession;

impl MqRestSession {
    // -----------------------------------------------------------------------
    // Pattern 1 -- Singleton DISPLAY (no name, returns first item or None)
    // -----------------------------------------------------------------------

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

    // -----------------------------------------------------------------------
    // Pattern 2 -- List DISPLAY (name + where, returns Vec)
    // -----------------------------------------------------------------------

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

    // -----------------------------------------------------------------------
    // Pattern 3a -- Mutating with required name (&str)
    // -----------------------------------------------------------------------

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

    // -----------------------------------------------------------------------
    // Pattern 3a -- Mutating with optional name (Option<&str>)
    // -----------------------------------------------------------------------

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
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "QALIAS",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE QLOCAL` command.
    pub fn delete_qlocal(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "QLOCAL",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE QMODEL` command.
    pub fn delete_qmodel(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "QMODEL",
            name,
            request_parameters,
            response_parameters,
            None,
        )?;
        Ok(())
    }

    /// Execute the MQSC `DELETE QREMOTE` command.
    pub fn delete_qremote(
        &mut self,
        name: Option<&str>,
        request_parameters: Option<&HashMap<String, Value>>,
        response_parameters: Option<&[&str]>,
    ) -> Result<()> {
        self.mqsc_command(
            "DELETE",
            "QREMOTE",
            name,
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

    // -----------------------------------------------------------------------
    // Pattern 3b -- Mutating without name (passes None)
    // -----------------------------------------------------------------------

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
}
