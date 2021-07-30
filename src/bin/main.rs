use dp_blockchain::{Block, BlockChainDb, BlockchainIterator};
//use sha2;

fn main() {
    let mut blockchain = BlockChainDb::new_blockchain();
    blockchain.add_block(String::from("Send 1 BTC to Ivan"));

    let mut blockchainiterator = BlockchainIterator::new(blockchain);

    loop {
        if let Some(block) = blockchainiterator.next() {
            //println!("{:?}", block)
            Block::show_block(&block);
        }
        else {
            break;
        }
    };
    //blockchain.add_block(String::from("Send 1 BTC to Ivan"));
    //blockchain.add_block(String::from("Send 2 more BTC to Ivan"));

    //blockchain.show_blockchain();

    // let x: &[u8] =&[1,2,3];
    // println!("{:?}", x);
    println!("hello world!");
}
