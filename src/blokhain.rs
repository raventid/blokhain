use crate::block::Block;

pub struct Blokhain {
    chain: Vec<Block>
}

impl Blokhain {
    fn new() -> Self {
        Blokhain {
            chain: [Block::genesis()].to_vec()
        }
    }

    fn add_block(&mut self, data: u8) {
        let block = Block::mine_block(self.chain.last().expect("genesis block is always here").clone(), data);
        self.chain.push(block);
    }
}
