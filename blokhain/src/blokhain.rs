use itertools::Itertools;

use crate::block::Block;

#[derive(Debug)]
pub struct Blokhain {
    pub chain: Vec<Block>
}

impl Blokhain {
    pub fn new(genesis: Option<Block>) -> Self {
        let chain = [genesis.unwrap_or_else(|| Block::genesis())].to_vec();

        Blokhain { chain }
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
                let wrong_block_sequence = next_block.last_hash != prev_block.hash;
                let wrong_hash_in_block = next_block.hash != next_block.recalculate_hash();
                if wrong_block_sequence || wrong_hash_in_block {
                    return false
                }
            }
        }
        true
    }

    fn replace_chain(&mut self, new_chain: Blokhain) -> Result<(), String> {
        if new_chain.chain.len() <= self.chain.len() { return Err("New chain is NOT longer than the current one".to_string()) }
        if !new_chain.is_valid_chain() { return Err("New chain is NOT valid".to_string()) }
        self.chain = new_chain.chain;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::block::Block;

    use super::Blokhain;

    // TODO: redo these tests as prop-based
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

    #[test]
    fn test_replace_chain() {
        let mut bc1 = Blokhain::new(None);
        let mut bc2 = Blokhain::new(None);

        bc2.add_block(1);

        assert!(bc1.replace_chain(bc2).is_ok());
    }

    #[test]
    fn test_replace_rejects_invalid_chain() {
        let mut bc1 = Blokhain::new(None);
        let bc2 = Blokhain {
            chain: [
                Block::genesis(),
                Block::mine_block(Block::genesis(), 1),
                Block::mine_block(Block::genesis(), 2),
            ].to_vec()
        };

        let expected = Err("New chain is NOT valid".to_string());
        assert_eq!(expected, bc1.replace_chain(bc2));
    }

    #[test]
    fn test_replace_rejects_short_chain() {
        let mut bc1 = Blokhain::new(None);
        let bc2 = Blokhain::new(None);

        bc1.add_block(1);

        let expected = Err("New chain is NOT longer than the current one".to_string());
        assert_eq!(expected, bc1.replace_chain(bc2));
    }
}
