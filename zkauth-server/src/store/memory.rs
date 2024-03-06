use anyhow::Result;
use dashmap::DashMap;
use moka::sync::Cache;
use std::time::Duration;
use uuid::Uuid;

use crate::store::{Challenge, Session, Store, User};

pub struct MemoryStore {
    users: DashMap<String, User>,
    challenges: Cache<String, Challenge>,
    sessions: Cache<String, Session>,
}

impl MemoryStore {
    pub fn new(challenges_ttl: Duration, sessions_ttl: Duration) -> Self {
        Self {
            users: DashMap::new(),
            challenges: Cache::builder().time_to_live(challenges_ttl).build(),
            sessions: Cache::builder().time_to_live(sessions_ttl).build(),
        }
    }
}

impl Default for MemoryStore {
    fn default() -> Self {
        Self::new(Duration::from_secs(300), Duration::from_secs(3600))
    }
}

impl Store for MemoryStore {
    fn insert_user(&self, username: &str, user: User) -> Result<()> {
        self.users.insert(username.to_string(), user);
        Ok(())
    }

    fn get_user(&self, username: &str) -> Result<Option<User>> {
        Ok(self.users.get(username).map(|u| u.value().clone()))
    }

    fn insert_challenge(&self, id: Uuid, challenge: Challenge) -> Result<()> {
        self.challenges.insert(id.to_string(), challenge);
        Ok(())
    }

    fn get_challenge(&self, id: Uuid) -> Result<Option<Challenge>> {
        Ok(self.challenges.get(&id.to_string()))
    }

    fn insert_session(&self, id: &str, session: Session) -> Result<()> {
        self.sessions.insert(id.to_string(), session);
        Ok(())
    }

    fn get_session(&self, id: &str) -> Result<Option<Session>> {
        Ok(self.sessions.get(&id.to_string()))
    }
}
