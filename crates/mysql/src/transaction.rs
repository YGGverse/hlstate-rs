use crate::table::*;
use mysql::{Error, Pool, TxOpts, prelude::Queryable};

/// Safe, optimized read/write operations
/// * all members implementation requires `commit` action
pub struct Transaction {
    tx: mysql::Transaction<'static>,
}

impl Transaction {
    pub fn create(pool: &Pool) -> Result<Self, Error> {
        Ok(Self {
            tx: pool.start_transaction(TxOpts::default())?,
        })
    }

    pub fn commit(self) -> Result<(), Error> {
        self.tx.commit()
    }

    pub fn rollback(self) -> Result<(), Error> {
        self.tx.rollback()
    }

    pub fn server_id_by_host_port(&mut self, host: &str, port: u32) -> Result<Option<u64>, Error> {
        self.tx.exec_first(
            "SELECT `id` FROM `server` WHERE `host` = ? AND `port` = ? LIMIT 1",
            (host, port),
        )
    }
}
