use std::time::SystemTime;

#[derive(Debug)]
struct Block {
    pub timestamp : SystemTime,
    pub last_hash: u8,
    pub hash: u8,
    pub data: u8,
}

impl Block {
    pub fn new(timestamp : SystemTime, last_hash: u8, hash: u8, data: u8) -> Self {
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
        Self::new(SystemTime::now(),0,0,0)
    }

    pub fn mine_block(last_block: Block, data: u8) -> Block {
        let now = SystemTime::now();
        let last_hash = last_block.hash;
        let hash = 1;

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
