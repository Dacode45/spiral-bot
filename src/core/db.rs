// Note that this requires the `v4` feature enabled in the uuid crate.

use redis::Commands;
use uuid::Uuid;

use std::collections::HashMap;

use crate::core::*;

pub const HT_UPDATES: &str = "spiralbot:updates";
pub const ZS_SORTED_UPDATES: &str = "spiralbot:sorted_updates";

pub const F_USER_ID: &str = "user_id";
pub const F_MESSAGE: &str = "message";
pub const F_TIME: &str = "time";

pub struct Update {
    pub user_id: u64,
    pub message: String,
    pub time: u64,
}

impl Update {
    pub fn new(user_id: u64, message: impl std::string::ToString) -> Self {
        Self {
            user_id,
            message: message.to_string(),
            time: 0,
        }
    }
}

pub struct DB {
    client: redis::Client,
}

impl DB {
    pub fn new(url: &str) -> AppResult<Self> {
        let client = redis::Client::open(url)?;
        Ok(Self { client })
    }

    pub fn list_updates(&self) -> AppResult<Vec<Update>> {
        let mut con = self.client.get_connection()?;

        let ids: Vec<String> = redis::cmd("ZRANGE")
            .arg(ZS_SORTED_UPDATES)
            .arg(0i32)
            .arg(-1i32)
            .query(&mut con)?;

        let mut updates: AppResult<Vec<Update>> = ids
            .iter()
            .map(|id| {
                let mut con = self.client.get_connection()?;

                let map: HashMap<String, String> = con.hgetall(id)?;

                let mut update = Update::new(
                    map.get(&F_USER_ID.to_owned()).unwrap().parse()?,
                    map.get(&F_MESSAGE.to_owned()).unwrap().to_string(),
                );
                update.time = map.get(&F_TIME.to_owned()).unwrap().parse()?;

                Ok(update)
            })
            .collect();
        return updates;
    }

    pub fn update(&self, update: Update) -> AppResult<()> {
        let mut con = self.client.get_connection()?;

        let id = format!("{}:{}", HT_UPDATES, uuid::Uuid::new_v4());
        let now = get_time();
        // Update to messages
        redis::cmd("HSET")
            .arg(&id)
            .arg(&F_USER_ID)
            .arg(&update.user_id)
            .arg(&F_MESSAGE)
            .arg(&update.message)
            .arg(&F_TIME)
            .arg(now.as_secs())
            .query(&mut con)?;

        // Add to rank table
        redis::cmd("ZADD")
            .arg(ZS_SORTED_UPDATES)
            .arg(now.as_secs())
            .arg(&id)
            .query(&mut con)?;

        Ok(())
    }
}
