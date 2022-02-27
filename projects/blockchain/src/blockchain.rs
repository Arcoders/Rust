// use time;
// use serde;
// use serde_json;
// use sha2::{Sha256, Digest};
// use std::fmt::Write;


#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    reciever: String,
    amount: f32
}

#[derive(Debug, Serialize)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkel: String,
    difficulty: u32,
}

pub struct Block {
    header: Blockheader,
    count: u32,
    transactions: Transaction
}

pub struct Chain {
    chain: Vec<Block>,
    curr_trans: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32
}

impl Chain {
    pub fn new(miner_addr: String, difficulty: u32) -> Self {
        let mut chain = Chain {
            chain: Vec::new(),
            curr_trans: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
        };

        // chain.generate_new_block();
        chain
    }

    pub fn new_transaction(&mut self, sender: String, reciever: String, amount: f32) -> bool {
        self.curr_trans.push(Transaction { sender, reciever, amount});
        true
    }

    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap()
        };
        Chain::hash(&block.header)
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }
}