pub mod memory;

use anyhow::Result;
use uuid::Uuid;
use zkauth::{Element, Scalar};

/// User data for the authentication protocol.
#[derive(Debug, Clone)]
pub struct User {
    pub y1: Element,
    pub y2: Element,
}

/// Challenge data for the authentication protocol.
#[derive(Debug, Clone)]
pub struct Challenge {
    pub user: String,
    pub c: Scalar,
    pub r1: Element,
    pub r2: Element,
}

/// Session data for the authentication protocol.
#[derive(Debug, Clone, Copy)]
pub struct Session {
    pub id: Uuid,
}

/// Store trait for the authentication protocol.
pub trait Store: Sync + Send {
    fn insert_user(&self, username: &str, user: User) -> Result<()>;
    fn get_user(&self, username: &str) -> Result<Option<User>>;

    fn insert_challenge(&self, id: Uuid, challenge: Challenge) -> Result<()>;
    fn get_challenge(&self, id: Uuid) -> Result<Option<Challenge>>;

    fn insert_session(&self, id: &str, session: Session) -> Result<()>;
    fn get_session(&self, id: &str) -> Result<Option<Session>>;
}
