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
}




#[cfg(test)]
mod tests {
    use super::Block;

    #[test]
    fn test_block_creation() {
        let block = Block::new(0,0,0,0);
        assert_eq!(0, block.data);
    }
}
