use crate::block::Block;

struct Blokhain {
    chain: Vec<Block>
}

impl Blokhain {
    fn new() -> Self {
        Blokhain {
            chain: [Block::genesis()].to_vec()
        }
    }
}
