use dp_blockchain::{self, BlockChain};
//use sha2;

fn main() {
    let mut blockchain = BlockChain::new_blockchain();
    blockchain.add_block(String::from("Send 1 BTC to Ivan"));
    blockchain.add_block(String::from("Send 2 more BTC to Ivan"));

    blockchain.show_blockchain();
}