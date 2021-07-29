use sha2::{Sha256, Digest};
use std::convert::TryInto;
use std::time::{SystemTime, Duration};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Debug)]
struct Block {
    timestamp: Duration,
    data: String,
    prev_block_hash: [u8; 32],
    hash: [u8; 32],
}

//sha2::digest::generic_array::GenericArray<u8, <Sha256 as Digest>::OutputSize>

impl Block {
    pub fn new_block(data: String, prev_block_hash: [u8; 32]) -> Block {

        let mut temp = Block {
            timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(),
            data: data,
            prev_block_hash: prev_block_hash,
            hash: [0u8; 32],
        };
        
        temp.set_hash();
        temp
    }


    fn set_hash(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update( [self.timestamp.as_nanos().to_string().as_bytes(), self.data.as_bytes(), &self.prev_block_hash ].concat() );
        let result = hasher.finalize();
        self.hash = result.as_slice().try_into().expect("slice with incorrect length");
    }
}

#[derive(Debug)]
pub struct BlockChain {
    blocks: Vec<Block>
}

impl BlockChain {
    pub fn new_blockchain() -> BlockChain {
        let genesis_block = Block::new_block(String::from("Genesis Block"), [0u8; 32]);
        let mut blockchain =  BlockChain {
            blocks: Vec::new(),
        };
        blockchain.blocks.push(genesis_block);
        blockchain
    }
    pub fn add_block(&mut self, data: String) {
        self.blocks.push(Block::new_block(data, self.blocks[self.blocks.len() - 1].hash));
    }

    pub fn show_blockchain(&self) {
        for i in &self.blocks {
            println!("TimeStamp: {:?}", i.timestamp);
            println!("Prev_hash: {:02x?}", i.prev_block_hash);
            println!("Data     : {}", i.data);
            println!("Hash     : {:02x?}", i.hash);
            println!("");
        }
    }
}

pub fn test_work() {
    println!("This is a test")
}
