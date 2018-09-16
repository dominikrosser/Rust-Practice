
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use num_bigint::BigUint;
use num_traits::One;
use chrono::prelude::*;

use mining_error::MiningError;

const HASH_BYTE_SIZE: usize = 32;
pub type Sha256Hash = [u8; HASH_BYTE_SIZE];

// Miner difficulty
pub const DIFFICULTY: usize = 5;
pub const MAX_NONCE: u64 = 3_000_000;

#[derive(Debug)]
pub struct Block {
    // Header
    timestamp: i64,
    hash: Sha256Hash,
    prev_hash: Sha256Hash,
    nonce: u64,

    // Body
    // here, instead of transaction, blocks contain data
    data: Vec<u8>,
}

impl Block {
    /// Creates a genesis block, which is a block with no parent
    /// The 'prev_hash'' field is set to all zeroes
    pub fn genesis() -> Result<Self, MiningError> {
        Self::new("Genesis block", Sha256Hash::default())
    }

    pub fn new(data: &str, prev_hash: Sha256Hash) -> Result<Self, MiningError> {
        let mut s = Self {
            prev_hash,
            data: data.to_owned().into(),
            timestamp: Utc::now().timestamp(),
            nonce: 0,
            hash: Sha256Hash::default(),
        };

        s.try_hash()
            .ok_or(MiningError::Iteration)
            .and_then(|(nonce, hash)| {
                s.nonce = nonce;
                s.hash = hash;

                Ok(s)
            })
    }

    fn try_hash(&self) -> Option<(u64, Sha256Hash)> {
        // The target is a number we compare the hash to. It is a 256bit binary with
        // leading zeroes
        let target = BigUint::one() << (256 - 4 * DIFFICULTY);

        for nonce in 0..MAX_NONCE {
            let hash = calculate_hash(&self, nonce);
            let hash_int = BigUint::from_bytes_be(&hash);

            if hash_int < target {
                return Some((nonce, hash));
            }
        }
        None
    }

    pub fn hash(&self) -> Sha256Hash {
        self.hash
    }

    pub fn prev_hash(&self) -> Sha256Hash {
        self.prev_hash
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn pretty_data(&self) -> String {
        let mut s = String::new();
        for c in &self.data {
            s.push(*c as char);
        }
        s
    }

    pub fn headers(&self) -> Vec<u8> {
        let mut vec = Vec::new();

        vec.extend(&util::convert_u64_to_u8_array(self.timestamp as u64));
        vec.extend_from_slice(&self.prev_hash);

        vec
    }
}

mod util {
    // This transforms a u64 into a little endian array of u8
    pub fn convert_u64_to_u8_array(val: u64) -> [u8; 8] {
        [
            val as u8,
            (val >> 8) as u8,
            (val >> 16) as u8,
            (val >> 24) as u8,
            (val >> 32) as u8,
            (val >> 40) as u8,
            (val >> 48) as u8,
            (val >> 56) as u8,
        ]
    }
}

pub fn calculate_hash(block: &Block, nonce: u64) -> Sha256Hash {
    let mut headers = block.headers();
    headers.extend_from_slice(&util::convert_u64_to_u8_array(nonce));

    let mut hasher = Sha256::new();
    hasher.input(&headers);
    let mut hash = Sha256Hash::default();

    hasher.result(&mut hash);

    hash
}
