#![no_std]
extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FrontendCommand {
    Ready,
    RequestRandomNumber { request_id: u32 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FrontendEvent {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BackendCommand {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BackendEvent {
    Connected,
    RandomNumber { request_id: u32, value: u32 },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FrontendMessage {
    Command(FrontendCommand),
    Event(FrontendEvent),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BackendMessage {
    Command(BackendCommand),
    Event(BackendEvent),
}

impl FrontendMessage {
    pub fn to_bytes(&self) -> Result<Vec<u8>, postcard::Error> {
        postcard::to_allocvec(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        postcard::from_bytes(bytes).ok()
    }

    pub fn to_base64(&self) -> Option<String> {
        use base64::Engine;
        let bytes = self.to_bytes().ok()?;
        Some(base64::engine::general_purpose::STANDARD.encode(&bytes))
    }

    pub fn from_base64(encoded: &str) -> Option<Self> {
        use base64::Engine;
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .ok()?;
        Self::from_bytes(&bytes)
    }
}

impl BackendMessage {
    pub fn to_bytes(&self) -> Result<Vec<u8>, postcard::Error> {
        postcard::to_allocvec(self)
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        postcard::from_bytes(bytes).ok()
    }

    pub fn to_base64(&self) -> Option<String> {
        use base64::Engine;
        let bytes = self.to_bytes().ok()?;
        Some(base64::engine::general_purpose::STANDARD.encode(&bytes))
    }

    pub fn from_base64(encoded: &str) -> Option<Self> {
        use base64::Engine;
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .ok()?;
        Self::from_bytes(&bytes)
    }
}
