use dp_blockchain::{Block, BlockChainDb, BlockchainIterator};
use clap::{Arg, App, SubCommand};

fn main() {
    let mut blockchain = BlockChainDb::new_blockchain();
    // //blockchain.add_block(String::from("Send 1 BTC to Ivan"));
    // //blockchain.add_block(String::from("Send 2 more BTC to Ivan"));
    // //blockchain.add_block(String::from("Send 1 BTC to Ivan"));

    // let mut blockchainiterator = BlockchainIterator::new(blockchain);

    // loop {
    //     if let Some(block) = blockchainiterator.next() {
    //         //println!("{:?}", block)
    //         Block::show_block(&block);
    //     }
    //     else {
    //         break;
    //     }
    // };
    // //blockchain.add_block(String::from("Send 1 BTC to Ivan"));
    // //blockchain.add_block(String::from("Send 2 more BTC to Ivan"));

    // //blockchain.show_blockchain();

    // // let x: &[u8] =&[1,2,3];
    // // println!("{:?}", x);
    // println!("hello world!");

    let matches = App::new("DP_BlockChain")
                          .version("1.0")
                          .author("Deep Tuuk. <zhouy2048@gmail.com>")
                          .about("Manage the Block Chain")
                          .arg(Arg::with_name("p")
                               .short("p")
                               .long("--print")
                               .help("Print the Block Chain"))                
                          .arg(Arg::with_name("a")
                               .short("a")
                               .long("--add")
                               .value_name("String")
                               .help("Add a Block")
                               .takes_value(true))                                         
                          .get_matches();

    if matches.is_present("p") {
        let mut blockchainiterator = BlockchainIterator::new(&blockchain);

        loop {
            if let Some(block) = blockchainiterator.next() {
                //println!("{:?}", block)
                Block::show_block(&block);
            }
            else {
                break;
            }
        };
    }

    if let Some(message) = matches.value_of("a") {
        blockchain.add_block(String::from(message));
    }
}
