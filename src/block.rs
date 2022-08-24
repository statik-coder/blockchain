// # Block
// This is the main blockchain block structure

use crate::{difficulty_bytes_as_u128, now, u128_bytes, u64_bytes, Hashable};
use std::fmt::{Debug, Formatter};

// Public block types

pub type Hash = Vec<u8>;
pub type Nonce = u64;
pub type Timestamp = u128;
pub type BlockIndex = u64;
pub type BlockPayload = String;
pub type Difficulty = u128;

/// # Block structure
///
/// The block structure publicly available with main field to interact with block

pub struct Block {
    pub index: BlockIndex,
    pub hash: Hash,
    pub prev_block_hash: Hash,
    pub nonce: Nonce,
    pub payload: BlockPayload,
    pub timestamp: Timestamp,
    pub difficulty: Difficulty,
}

impl Block {
    pub fn new_genesis() -> Self {
        Self::new(
            0,
            vec![0, 64],
            0,
            String::from("Genesis block!"),
            0x00000000000000000000000000000000,
        )
    }

    /// Creates new Block instance with required fields
    pub fn new(
        index: BlockIndex,
        prev_block_hash: Hash,
        nonce: Nonce,
        payload: BlockPayload,
        difficulty: Difficulty,
    ) -> Block {
        Block {
            index,
            hash: vec![0, 32],
            prev_block_hash,
            nonce,
            payload,
            timestamp: now(),
            difficulty,
        }
    }
}

// Traits implementations

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(u64_bytes(&self.index));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(u64_bytes(&self.nonce));
        bytes.extend(self.payload.as_bytes());
        bytes.extend(u128_bytes(&self.timestamp));
        bytes
    }
}

// Structure function implementations

impl Block {
    /// Generated `SHA256` hash and adds it to the `hash` field
    pub fn create_hash(self: &mut Self) -> () {
        self.hash = self.hash()
    }

    /// Check current hash matching with required difficulty
    pub fn check_difficulty(hash: &Hash, difficulty: u128) -> bool {
        difficulty > difficulty_bytes_as_u128(&hash)
    }

    /// Mine block hash
    pub fn mine(self: &mut Self) -> () {
        for attempt in 0..(u64::max_value()) {
            self.nonce = attempt;
            let mined = self.hash();
            println!("Mining... {}", hex::encode(&mined));
            if Block::check_difficulty(&mined, self.difficulty) {
                self.hash = mined;
                return;
            }
        }
    }
}

// Other implementations (helpful implementation)

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "Block[{}] \n --- Hash: {} \n --- Prev block timestamp: {} \n --- Timestamp: {} \n --- Payload: {} \n --- Nonce {}",
            &self.index,
            &hex::encode(&self.hash),
            &hex::encode(&self.prev_block_hash),
            &self.timestamp,
            &self.payload,
            &self.nonce
        )
    }
}
