use num_bigint::BigUint;
use sha2::{Digest, Sha256};
use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};
use serde_json::Result;

use nut::DBBuilder;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

const TARGET_BITS: u64 = 16;

#[derive(Serialize, Deserialize, Debug)]
struct Block {
    timestamp: Duration,
    data: String,
    prev_block_hash: BigUint,
    hash: BigUint,
    nonce: u128,
}

//sha2::digest::generic_array::GenericArray<u8, <Sha256 as Digest>::OutputSize>

impl Block {
    pub fn new_block(data: String, prev_block_hash: BigUint) -> Block {
        let mut temp = Block {
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap(),
            data: data,
            prev_block_hash: prev_block_hash,
            hash: BigUint::new(Vec::new()),
            nonce: 0,
        };

        temp.proof_of_work();
        while !temp.validate_work() {
            temp.timestamp = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap();
            temp.nonce = 0;
            temp.proof_of_work();
        }
        temp
    }

    fn validate_work(&mut self) -> bool {
        let mut target_big = BigUint::new(Vec::new());
        target_big.set_bit(256 - TARGET_BITS, true);

        let mut hasher = Sha256::new();
        hasher.update(
            [
                self.timestamp.as_nanos().to_string().as_bytes(),
                self.data.as_bytes(),
                &self.prev_block_hash.to_bytes_le()[..],
                &TARGET_BITS.to_le_bytes(),
                &self.nonce.to_le_bytes(),
            ]
            .concat(),
        );
        let result = hasher.finalize();

        let temp = BigUint::from_bytes_le(result.as_slice());

        if temp < target_big {
            return true;
        } else {
            return false;
        }
    }
    // fn set_hash(&mut self) {
    //     let mut hasher = Sha256::new();
    //     hasher.update( [self.timestamp.as_nanos().to_string().as_bytes(), self.data.as_bytes(), &self.prev_block_hash ].concat() );
    //     let result = hasher.finalize();
    //     self.hash = result.as_slice().try_into().expect("slice with incorrect length");
    // }

    fn proof_of_work(&mut self) {
        let mut hasher = Sha256::new();
        let mut result = hasher.finalize_reset();

        //let v: Vec<u32> = Vec::new();
        let mut target_big = BigUint::new(Vec::new());
        target_big.set_bit(256 - TARGET_BITS, true);
        //println!("target_big = {:064x}", target_big);

        while self.nonce < u128::MAX {
            hasher.update(
                [
                    self.timestamp.as_nanos().to_string().as_bytes(),
                    self.data.as_bytes(),
                    &self.prev_block_hash.to_bytes_le()[..],
                    &TARGET_BITS.to_le_bytes(),
                    &self.nonce.to_le_bytes(),
                ]
                .concat(),
            );

            result = hasher.finalize_reset();
            //self.hash = result.as_slice().try_into().expect("slice with incorrect length");

            //let temp = BigUint::from_bytes_le(&self.hash);
            self.hash = BigUint::from_bytes_le(result.as_slice());

            if self.hash < target_big {
                break;
            } else {
                self.nonce = self.nonce + 1;
            }
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }

    pub fn deserialize(block: &mut Vec<u8>) -> Block {
        let temp: Block = serde_json::from_slice(block).unwrap();
        temp
    }
}

#[derive(Debug)]
pub struct BlockChain {
    blocks: Vec<Block>,
}

impl BlockChain {
    pub fn new_blockchain() -> BlockChain {

        // let tip: &[u8];

        // let mut db = DBBuilder::new("test.db").build().unwrap();
        // let mut tx = db.begin_rw_tx().unwrap();

        // match tx.bucket(b"blocksBucket") {
        //     Ok(blocks) => {
        //         tip = blocks.get(b"l").unwrap();
        //     },
        //     Err(_)   => {

        //     },
        // }

        let genesis_block =
            Block::new_block(String::from("Genesis Block"), BigUint::new(vec![0u32; 8]));
        let mut blockchain = BlockChain { blocks: Vec::new() };
        blockchain.blocks.push(genesis_block);
        blockchain
    }
    pub fn add_block(&mut self, data: String) {
        // let temp = Block::new_block(data, self.blocks[self.blocks.len() - 1].hash.clone());
        // let j = serde_json::to_string(&temp).unwrap();
        // println!("{}", j);
        //self.blocks.push(temp);
        self.blocks.push(Block::new_block(
            data,
            self.blocks[self.blocks.len() - 1].hash.clone(),
        ))
    }

    pub fn show_blockchain(&self) {
        for i in &self.blocks {
            println!("TimeStamp: {:?}", i.timestamp);
            println!("Prev_hash: {:064x}", i.prev_block_hash);
            println!("Data     : {}", i.data);
            println!("Hash     : {:064x}", i.hash);
            println!("Nonce    : {}", i.nonce);
            println!("");
        }
    }
}

pub fn test_work() {
    println!("This is a test")
}
