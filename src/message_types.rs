use std::time::{SystemTime};

use sodiumoxide::crypto::box_::{
    PublicKey,
};

pub struct RequestForACheck {
    pub from: String,
    pub timestamp: SystemTime,
    pub public_key: PublicKey,
    pub signature: Vec<u8>,
}
