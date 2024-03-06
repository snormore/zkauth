pub mod memory;

use anyhow::Result;
use uuid::Uuid;
use zkauth::{Element, Scalar};

/// User data for the authentication protocol.
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub y1: Element,
    pub y2: Element,
}

/// Challenge data for the authentication protocol.
#[derive(Debug, Clone, PartialEq)]
pub struct Challenge {
    pub user: String,
    pub c: Scalar,
    pub r1: Element,
    pub r2: Element,
}

/// Session data for the authentication protocol.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Session {
    pub id: Uuid,
}

/// Store trait for the authentication protocol.
pub trait Store: Sync + Send {
    /// Inserts a user into the store.
    fn insert_user(&self, username: &str, user: User) -> Result<()>;

    /// Returns an Option<User> for the given username.
    fn get_user(&self, username: &str) -> Result<Option<User>>;

    /// Inserts a challenge into the store.
    fn insert_challenge(&self, id: Uuid, challenge: Challenge) -> Result<()>;

    /// Returns an Option<Challenge> for the given id.
    fn get_challenge(&self, id: Uuid) -> Result<Option<Challenge>>;

    /// Inserts a session into the store.
    fn insert_session(&self, id: &str, session: Session) -> Result<()>;

    /// Returns an Option<Session> for the given id.
    fn get_session(&self, id: &str) -> Result<Option<Session>>;
}
