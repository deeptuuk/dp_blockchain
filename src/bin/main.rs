use dp_blockchain::{Block, BlockChainDb, BlockchainIterator, Transaction, Pool};
use clap::{Arg, App, SubCommand};

fn main() {
    let mut blockchain = BlockChainDb::new_blockchain(String::from("Ivan"));
    //blockchain.add_block(vec![Transaction::new_coinbase_tx(String::from("Ivan"))]);
    // let mut pool = Pool::new_pool();
    // pool.add_transaction_to_pool(&blockchain, "Ivan", "Test", 5);
    //blockchain.add_block(vec![Transaction::new_coinbase_tx(String::from("Ivan"))]);
    //println!("{:?}", blockchain.find_spendable_transaction_output_index(&pool, "Ivan", 60));
    //println!("{:?}", blockchain.find_unspent_transactions(&pool, "Ivan"));
    // println!("------------------------------------------------------");

    // blockchain.add_block(vec![Transaction::new_utxo_transaction(&blockchain, "Ivan", "Test", 5).unwrap()]);
    // //blockchain.add_block(vec![Transaction::new_utxo_transaction(&blockchain, "Ivan", "Test", 5).unwrap()]);
    //blockchain.show_blockchain();

    // println!("{:?}", blockchain.find_unspent_transactions("Ivan"));

    //println!("------------------------------------------------------");

    //println!("{:?}", blockchain.find_unspent_transactions("Test"));
    //println!("{:?}", blockchain.find_spendable_outputs("Ivan", 5));
    let matches = App::new("DP_BlockChain")
                          .version("1.0")
                          .author("Deep Tuuk. <zhouy2048@gmail.com>")
                          .about("Manage the Block Chain")
                          .arg(Arg::with_name("p")
                               .short("p")
                               .long("--print")
                               .help("Print the Block Chain"))              
                          .arg(Arg::with_name("b")
                               .short("b")
                               .long("--balance")
                               .value_name("Address")
                               .help("Print the Balance")
                               .takes_value(true))    
                          .arg(Arg::with_name("from")
                               .short("f")
                               .long("--from")
                               .value_name("Address")
                               .help("Send from")
                               .takes_value(true))             
                          .arg(Arg::with_name("to")
                               .short("t")
                               .long("--to")
                               .value_name("Address")
                               .help("Send to")
                               .takes_value(true))              
                          .arg(Arg::with_name("amount")
                               .short("a")
                               .long("--amount")
                               .value_name("amount")
                               .help("Send amount")
                               .takes_value(true))                                                                                      
                          .get_matches();

    if matches.is_present("p") {
        blockchain.show_blockchain();
    }
    
    if let Some(message) = matches.value_of("b") {
        println!("Balance of '{}': {}", message, blockchain.get_balance(message));
    }    

    if let Some(from) = matches.value_of("from") {
        if let Some(to) = matches.value_of("to") {
            if let Some(amount) = matches.value_of("amount") {

                //blockchain.add_block(vec![Transaction::new_coinbase_tx(String::from("Ivan"))]);

                let mut pool = Pool::new_pool();
                pool.add_transaction_to_pool(&blockchain, from, to, amount.parse::<i64>().unwrap());
                blockchain.add_block(pool.pool);

                println!("{:?}", blockchain.find_unspent_transactions(&Pool::new_pool(), "Ivan"));
            }
        }
    }
}