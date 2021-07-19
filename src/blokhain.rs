use itertools::Itertools;

use crate::block::Block;

#[derive(Debug)]
pub struct Blokhain {
    chain: Vec<Block>
}

impl Blokhain {
    fn new(genesis: Option<Block>) -> Self {
        let chain = [genesis.unwrap_or_else(|| Block::genesis())].to_vec();

        Blokhain { chain }
    }

    fn add_block(&mut self, data: u8) {
        let block = Block::mine_block(self.chain.last().expect("genesis block is always here").clone(), data);
        self.chain.push(block);
    }

    fn is_valid_chain(&self) -> bool {
        if self.chain.first().expect("we should have a genesis block") != &Block::genesis() {
                dbg!("raventid");
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

#[cfg(test)]
mod tests {
    use crate::block::Block;

    use super::Blokhain;

    #[test]
    fn test_is_valid_chain() {
       assert!(Blokhain::new(None).is_valid_chain())
    }

    #[test]
    fn test_chain_is_not_valid_if_genesis_block_is_wrong() {
       let genesis = Block::genesis();
       let not_genesis = Block::mine_block(genesis, 1);

       assert!(!Blokhain::new(Some(not_genesis)).is_valid_chain())
    }

    #[test]
    fn test_chain_is_not_valid_if_some_part_of_chain_is_wrong() {
       let genesis = Block::genesis();
       let second_block = Block::mine_block(genesis.clone(), 1);
       let alternative_second_block = Block::mine_block(genesis.clone(), 2);
       let chain = Blokhain {
            chain: [genesis, second_block, alternative_second_block].to_vec()
       };

       assert!(!chain.is_valid_chain())
    }
}
