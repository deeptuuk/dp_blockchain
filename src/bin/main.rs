use dp_blockchain::{self, Block};
//use sha2;

fn main() {
    dp_blockchain::test_work();
    println!("hello, world!");

    let first_block_data = String::from("This is a test data");
    let first_block = Block::new(20210729, first_block_data, [0u8; 32]);

    println!("{:?}", first_block)

    // let mut block = Block {
    //     timestamp: 20210728,
    //     data: String::from("This is a test data"),
    //     prev_block_hash: [0u8; 32],
    //     hash: [0u8; 32],
    // };

    // block.set_hash();
}