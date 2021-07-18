use itertools::Itertools;

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

    fn is_valid_chain(&self) -> bool {
        if self.chain.first().expect("we should have a genesis block") != &Block::genesis() {
            return false
        } else {
            for (prev_block, next_block) in self.chain.iter().skip(1).tuple_windows() {
                if next_block.last_hash != prev_block.hash || next_block.hash != next_block.recalculate_hash() {
                    return false
                }
            }
        }
        true
    }
}
