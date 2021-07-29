use sha2::{Sha256, Digest};
use std::str;
use std::convert::TryInto;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Debug)]
pub struct Block {
    timestamp: u128,
    data: String,
    prev_block_hash: [u8; 32],
    hash: [u8; 32],
}

//sha2::digest::generic_array::GenericArray<u8, <Sha256 as Digest>::OutputSize>

impl Block {
    pub fn new(timestamp: u128, data: String, prev_block_hash: [u8; 32]) -> Block {

        let mut temp = Block {
            timestamp: timestamp,
            data: data,
            prev_block_hash: prev_block_hash,
            hash: [0u8; 32],
        };
        
        temp.set_hash();
        temp
    }


    pub fn set_hash(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update((self.timestamp.to_string() + &self.data + str::from_utf8(&self.prev_block_hash).unwrap()).as_bytes());
        let result = hasher.finalize();
        self.hash = result.as_slice().try_into().expect("slice with incorrect length");
        println!("{:?}", self.hash)
    }
}

pub fn test_work() {
    println!("This is a test")
}
