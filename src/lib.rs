use serde::{Serialize, Deserialize};
use sodiumoxide::crypto::box_::{PublicKey as SodiumPublicKey, SecretKey as SodiumSecretKey, Nonce as SodiumNonce};
use sodiumoxide::crypto::box_;

use paillier::*;
use paillier::{
    EncryptionKey as PaillierEncryptionKey,
    encoding::EncodedCiphertext as PaillierEncodedCiphertext
};

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
    /*RecipientToSenderRound1 {
        paillier_key: PaillierEncryptionKey,
        enc_ab_pairs: [(PaillierEncodedCiphertext<usize>, PaillierEncodedCiphertext<usize>); 128],
        a_shares: [PaillierEncodedCiphertext<usize>; 128],
        b_shares: [PaillierEncodedCiphertext<usize>; 128],
    },
    CourierToSenderRound1 {
        paillier_key: PaillierEncryptionKey,
        enc_ab_pairs: [(PaillierEncodedCiphertext<usize>, PaillierEncodedCiphertext<usize>); 128],
        a_shares: [PaillierEncodedCiphertext<usize>; 128],
        b_shares: [PaillierEncodedCiphertext<usize>; 128]
    }*/
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ProtocolMessage {
    ACheck,
    PCheck(PCheckMessage),
}
