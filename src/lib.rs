use serde::{Serialize, Deserialize};
use sodiumoxide::crypto::box_::{PublicKey as SodiumPublicKey, SecretKey as SodiumSecretKey, Nonce as SodiumNonce};
use sodiumoxide::crypto::box_;

use paillier::*;
use paillier::{
    EncryptionKey as PaillierEncryptionKey,
    encoding::EncodedCiphertext as PaillierEncodedCiphertext
};

use arrayvec::ArrayVec;

#[derive(Serialize, Deserialize, Debug)]
pub enum GhostmatesMessage {
    Identify {
        ghostmates_address: String,
        pubkey: SodiumPublicKey,
    },
    SuccesfulIdentify,
    FailedIdentify,
    SuccesfulLookup {
        pubkey: SodiumPublicKey,
        ghostmates_address: String,
    },
    FailedLookup {
        ghostmates_address: String,
    },
    Lookup {
        dest_address: String,
    },
    DirectMessage {
        dest_address: String,
        encrypted_message: Vec<u8>,
        nonce: SodiumNonce
    },
    IncomingMessage {
        from_address: String,
        encrypted_message: Vec<u8>,
        nonce: SodiumNonce,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PCheckMessage {
    RequestForPCheck {
        recipient_ghost_address: String,
        courier_ghost_address: String
    },
    CourierRequestRecipient {
        sender_ghost_address: String,
    },
    RecipientToSenderRound1 {
        courier_address: String,
        paillier_key: PaillierEncryptionKey,
        // Lengths of all these vectors is 128... don't know how to fix that
        enc_ab_pairs: Vec<(PaillierEncodedCiphertext<u64>, PaillierEncodedCiphertext<u64>)>,
        a_shares: Vec<PaillierEncodedCiphertext<u64>>,
        b_shares: Vec<PaillierEncodedCiphertext<u64>>,
    },
    RecipientToCourierRound1 {
        sender_address: String,
        paillier_key: PaillierEncryptionKey,
        // Lengths of all these vectors is 128... don't know how to fix that
        enc_ab_pairs: Vec<(PaillierEncodedCiphertext<u64>, PaillierEncodedCiphertext<u64>)>,
        a_shares: Vec<PaillierEncodedCiphertext<u64>>,
        b_shares: Vec<PaillierEncodedCiphertext<u64>>,
    },
    CourierToSenderRound1 {
        recipient_address: String,
        paillier_key: PaillierEncryptionKey,
        // Lengths of all these vectors is 128... don't know how to fix that
        enc_ab_pairs: Vec<(PaillierEncodedCiphertext<u64>, PaillierEncodedCiphertext<u64>)>,
        a_shares: Vec<PaillierEncodedCiphertext<u64>>,
        b_shares: Vec<PaillierEncodedCiphertext<u64>>,
    },
    CourierToRecipientRound1 {
        sender_address: String,
        courier_to_recipient_t_values: Vec<PaillierEncodedCiphertext<u64>>,
    },
    SenderToCourierRound1 {
        recipient_address: String,
        sender_to_courier_t_values: Vec<PaillierEncodedCiphertext<u64>>,
    },
    SenderToRecipientRound1{
        courier_address: String,
        sender_to_recipient_t_values: Vec<PaillierEncodedCiphertext<u64>>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProtocolMessage {
    ACheck,
    PCheck(PCheckMessage),
}
