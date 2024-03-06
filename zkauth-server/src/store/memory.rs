use anyhow::Result;
use dashmap::DashMap;
use moka::sync::Cache;
use std::time::Duration;
use uuid::Uuid;

use crate::store::{Challenge, Session, Store, User};

/// MemoryStore is an in-memory implementation of the Store trait.
pub struct MemoryStore {
    users: DashMap<String, User>,
    challenges: Cache<String, Challenge>,
    sessions: Cache<String, Session>,
}

/// Implement the MemoryStore.
impl MemoryStore {
    pub fn new(challenges_ttl: Duration, sessions_ttl: Duration) -> Self {
        Self {
            users: DashMap::new(),
            challenges: Cache::builder().time_to_live(challenges_ttl).build(),
            sessions: Cache::builder().time_to_live(sessions_ttl).build(),
        }
    }
}

/// Implement the Default trait for the MemoryStore.
impl Default for MemoryStore {
    fn default() -> Self {
        Self::new(Duration::from_secs(300), Duration::from_secs(3600))
    }
}

/// Implement the Store trait for the MemoryStore.
impl Store for MemoryStore {
    fn insert_user(&self, username: &str, user: User) -> Result<()> {
        self.users.insert(username.to_string(), user);
        Ok(())
    }

    /// The get_user method returns an Option<User> for the given username.
    fn get_user(&self, username: &str) -> Result<Option<User>> {
        Ok(self.users.get(username).map(|u| u.value().clone()))
    }

    /// The insert_challenge method inserts a challenge into the store.
    fn insert_challenge(&self, id: Uuid, challenge: Challenge) -> Result<()> {
        self.challenges.insert(id.to_string(), challenge);
        Ok(())
    }

    /// The get_challenge method returns an Option<Challenge> for the given id.
    fn get_challenge(&self, id: Uuid) -> Result<Option<Challenge>> {
        Ok(self.challenges.get(&id.to_string()))
    }

    /// The insert_session method inserts a session into the store.
    fn insert_session(&self, id: &str, session: Session) -> Result<()> {
        self.sessions.insert(id.to_string(), session);
        Ok(())
    }

    /// The get_session method returns an Option<Session> for the given id.
    fn get_session(&self, id: &str) -> Result<Option<Session>> {
        Ok(self.sessions.get(&id.to_string()))
    }
}
