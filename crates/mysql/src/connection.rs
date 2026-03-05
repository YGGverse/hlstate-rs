use crate::table::*;
use mysql::{Error, Pool, PooledConn, prelude::Queryable};

/// Safe, read-only operations
pub struct Connection {
    conn: PooledConn,
}

impl Connection {
    pub fn create(pool: &Pool) -> Result<Self, Error> {
        Ok(Self {
            conn: pool.get_conn()?,
        })
    }

    pub fn server(&mut self, id: u64) -> Result<Option<Server>, Error> {
        self.conn.exec_first(
            "SELECT `id`,
                    `added`,
                    `updated`,
                    `host`,
                    `port`,
                    `name`,
                    `description` FROM `server` WHERE `id` = ?",
            (id,),
        )
    }

    pub fn servers(
        &mut self,
        sort: Option<Sort>,
        start: Option<usize>,
        limit: Option<usize>,
    ) -> Result<Vec<Server>, Error> {
        self.conn.exec(
            format!(
                "SELECT `id`,
                    `added`,
                    `updated`,
                    `host`,
                    `port`,
                    `name`,
                    `description` FROM `server`
                    ORDER BY `id` {} LIMIT {},{}",
                sort.unwrap_or_default(),
                start.unwrap_or(0),
                limit.unwrap_or(DEFAULT_LIMIT)
            ),
            (),
        )
    }
}

const DEFAULT_LIMIT: usize = 100;
