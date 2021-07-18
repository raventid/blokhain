use std::time::SystemTime;

use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Block {
    pub timestamp : SystemTime,
    pub last_hash: Vec<u8>,
    pub hash: Vec<u8>,
    pub data: u8,
}

impl Block {
    pub fn new(timestamp : SystemTime, last_hash: Vec<u8>, hash: Vec<u8>, data: u8) -> Self {
        Block {
            timestamp,
            last_hash,
            hash,
            data,
        }
    }

    // Genesis function return the Genesis block
    // which is the first block in our blockchain.
    pub fn genesis() -> Self {
        Self::new(SystemTime::now(), [0].to_vec(), [0].to_vec(), 0)
    }

    pub fn mine_block(last_block: Block, data: u8) -> Block {
        let now = SystemTime::now();
        let last_hash = last_block.hash;
        let hash = Self::calculate_hash(&last_hash, now, data);

        Block::new(now, last_hash, hash, data)
    }

    pub fn recalculate_hash(&self) -> Vec<u8> {
        let timestamp = self.timestamp;
        let last_hash = self.last_hash.clone();
        let data = self.data;

        Self::calculate_hash(&last_hash, timestamp, data)
    }

    fn calculate_hash(last_hash: &Vec<u8>, timestamp: SystemTime, data: u8) -> Vec<u8> {
        let millis = timestamp
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
            .to_string();

        Sha256::new()
            .chain(millis.as_bytes())
            .chain(last_hash.clone())
            .chain([data])
            .finalize()
            .to_vec()
    }
}


#[cfg(test)]
mod tests {
    use super::Block;
    use quickcheck as qc;
    use quickcheck_macros::quickcheck;

    impl quickcheck::Arbitrary for Block {
      fn arbitrary(_g: &mut qc::Gen) -> Self {
          Block::genesis()
      }
    }

    #[test]
    fn test_block_creation() {
        let genesis = Block::genesis();
        let block = Block::mine_block(genesis.clone(), 9);
        assert_eq!(genesis.hash, block.last_hash);
    }

    #[quickcheck]
    fn new_block_hash_is_a_hash_from_timestamp_and_previous_hash_and_data(previous_block: Block) -> bool {
        let data = 1;
        let next_block = Block::mine_block(previous_block.clone(), data);

        next_block.recalculate_hash() == next_block.hash
    }
}
