use std::time::SystemTime;

use sha2::{Digest, Sha256};

#[derive(Debug)]
struct Block {
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

        let since_the_epoch = now
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
            .to_string();

        let hash = Sha256::new()
            .chain(since_the_epoch.as_bytes())
            .chain(last_hash.clone())
            .chain([data])
            .finalize()
            .to_vec();

        Block::new(now, last_hash, hash, 0)
    }
}


#[cfg(test)]
mod tests {
    use std::time::SystemTime;
    use super::Block;

    #[test]
    fn test_block_creation() {
        let block = Block::new(SystemTime::now(),0,0,0);
        assert_eq!(0, block.data);
    }
}
