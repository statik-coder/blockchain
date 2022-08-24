use std::vec;

use crate::{
    block::{BlockIndex, Difficulty},
    Block,
};

type ChainOfBlocks = Vec<Block>;

const BLOCK_MINE_DIFFICULTY: Difficulty = 0x000fffffffffffffffffffffffffffff;

pub struct Blockchain {
    _difficulty: Difficulty,
    _index: BlockIndex,
    _stop_index: usize,
    chain: ChainOfBlocks,
}

impl Blockchain {
    fn get_latest_chain_block(self: &Self) -> &Block {
        let chain_count = self.chain.len();
        self.chain.get(chain_count - 1).unwrap()
    }

    pub fn bootstrap() -> Self {
        let genesis = Block::new_genesis();
        Blockchain {
            _difficulty: BLOCK_MINE_DIFFICULTY,
            _index: 0,
            _stop_index: 12,
            chain: vec![genesis],
        }
    }

    pub fn get_chain(self: Self) -> ChainOfBlocks {
        self.chain
    }

    pub fn start_mining(self: &mut Self) {
        loop {
            let last_hash = &self.get_latest_chain_block().hash;
            let mut block = Block::new(
                self._index,
                last_hash.to_vec(),
                0,
                String::from("Another block"),
                self._difficulty,
            );
            block.mine();
            println!(
                "Block has mined successfully. Block[{}]: {} at {} with {}.",
                &block.index,
                hex::encode(&block.hash),
                &block.timestamp,
                &block.payload
            );
            self.chain.push(block);
            self._index += 1;
            if self._stop_index == self._index.try_into().unwrap() {
                break;
            }
        }
    }
}
