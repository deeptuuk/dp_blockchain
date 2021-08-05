use num_bigint::BigUint;
use sha2::{Digest, Sha256};
use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};
use nut::{DBBuilder, DB};
use std::collections::HashMap;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

const TARGET_BITS: u64 = 12;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXInput {
    txid: BigUint,
    vout_index: usize,
    script_sig: String,
}

impl TXInput {
    fn can_unlock_output_with(&self, unlock_data: &str) -> bool {
        return self.script_sig == unlock_data;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TXOutput {
    value: i64,
    script_pubkey: String,
}

impl TXOutput {
    fn can_be_unlocked_with(&self, unlock_data: &str) -> bool {
        return self.script_pubkey == unlock_data;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    timestamp: Duration,
    id: BigUint,
    vin: Vec<TXInput>,
    vout: Vec<TXOutput>,
}

impl Transaction {
    pub fn new_coinbase_tx(to: String) -> Transaction {
        let mut tx = Transaction {
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap(),            
            id: BigUint::new(vec![0u32; 8]),
            vin: vec![TXInput {
                txid: BigUint::new(vec![0u32; 8]),
                vout_index: 0,
                script_sig: String::from(format!("Reward to {}", to)),
            }],
            vout: vec![TXOutput {
                value: 50,
                script_pubkey: to,
            }],
        };

        Transaction::set_id(&mut tx);
        tx
    }

    fn serialize(temp: &Transaction) -> Vec<u8> {
        serde_json::to_vec(temp).unwrap()
    }    
    
    fn set_id(temp: &mut Transaction) {
        let mut hasher = Sha256::new();
        hasher.update(Transaction::serialize(temp));
        //hasher.update();
        let result = hasher.finalize();
        temp.id = BigUint::from_bytes_le(result.as_slice());
    }

    fn is_coinbase(&self) -> bool {
        return self.vin[0].txid == BigUint::new(vec![0u32; 8])
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pool {
    pub pool: Vec<Transaction>,
}

impl Pool {
    pub fn new_pool() -> Pool {
        Pool {
            pool: Vec::new()
        }
    }

    pub fn add_transaction_to_pool(&mut self, bc: &BlockChainDb, from: &str, to: &str, amount: i64) {

        let (from_available, tx_table) = bc.find_spendable_transaction_output_index(self, from, amount);

        if from_available < amount {
            println!("{} not have enough money", from);
            return
        }

        let mut inputs: Vec<TXInput> = Vec::new();
        for (transaction_id, vout_index_collect) in tx_table {
            for vout_index in vout_index_collect {
                inputs.push(TXInput {
                    txid: transaction_id.clone(),
                    vout_index: vout_index,
                    script_sig: String::from(from),
                });
            }
        }

        let mut outputs: Vec<TXOutput> = Vec::new();
        outputs.push(TXOutput {
            value: amount,
            script_pubkey: String::from(to),
        });

        if from_available > amount {
            outputs.push(TXOutput {
                value: from_available - amount,
                script_pubkey: String::from(from),
            });            
        }

        let mut transaction = Transaction {
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap(),            
            id: BigUint::new(vec![0u32; 8]),
            vin: inputs,
            vout: outputs,
        };

        Transaction::set_id(&mut transaction);

        self.pool.push(transaction);
    }

    pub fn add_coninbase_to_pool(&mut self, address: &str) {
        self.pool.push(Transaction::new_coinbase_tx(String::from(address)));
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    timestamp: Duration,
    transactions: Vec<Transaction>,
    prev_block_hash: BigUint,
    hash: BigUint,
    nonce: u128,
}



impl Block {
    pub fn new_genesis_block(coinbase: Transaction) -> Result<Block, ()> {

        let mut block = Block {
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap(),
            transactions: vec![coinbase],
            prev_block_hash: BigUint::new(vec![0u32; 8]),
            hash: BigUint::new(vec![0u32; 8]),
            nonce: 0,
        };
        
        match block.proof_of_work() {
            Ok(_) => return Ok(block),
            Err(_) => return Err(()),
        }
    }

    pub fn new_block(transactions: Vec<Transaction>, prev_block_hash: BigUint) -> Result<Block, ()> {
        let mut block = Block {
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap(),
            transactions: transactions,
            prev_block_hash: prev_block_hash,
            hash: BigUint::new(vec![0u32; 8]),
            nonce: 0,
        };
        
        match block.proof_of_work() {
            Ok(_) => return Ok(block),
            Err(_) => return Err(()),
        }        
    }

    fn proof_of_work(&mut self) -> Result<(), ()> {

        let mut hasher = Sha256::new();
        let mut result = hasher.finalize_reset();

        let mut target_big = BigUint::new(Vec::new());
        target_big.set_bit(256 - TARGET_BITS, true);

        while self.nonce < u128::MAX {
            hasher.update(
                [
                    self.timestamp.as_nanos().to_string().as_bytes(),
                    &self.hash_transactions().to_bytes_le()[..],
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
                return Ok(())
            } else {
                self.nonce = self.nonce + 1;
            }
        }
        return Err(())
    }

    fn hash_transactions(&self) -> BigUint {
        let mut hasher = Sha256::new();
        for i in &self.transactions {
            hasher.update(&i.id.to_bytes_le()[..]);
        }
        let result = hasher.finalize();

        BigUint::from_bytes_le(result.as_slice())
    }

    pub fn serialize(&self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }

    pub fn deserialize(block: &Vec<u8>) -> Block {
        let temp: Block = serde_json::from_slice(block).unwrap();
        temp
    }

    pub fn show_block(i: &Block) {
        println!("TimeStamp: {:?}", i.timestamp);
        println!("Prev_hash: {:064x}", i.prev_block_hash);
        println!("Hash     : {:064x}", i.hash);
        println!("Nonce    : {}", i.nonce);
        for temp in &i.transactions {
            println!("- Transaction: ");
            //println!("  id:      {:064x}", temp.id);
            println!("  timestamp: {:?}", temp.timestamp);
            println!("  id:        {:?}", temp.id);
            println!("  vin:       {:?}", temp.vin);
            println!("  vout:      {:?}", temp.vout);
        }
        println!("");
    }      
}

pub struct BlockChainDb {
    tip: Vec<u8>,
    db: nut::DB,
}

impl BlockChainDb {
    pub fn new_blockchain(address: String) -> BlockChainDb {
        let mut tip: Vec<u8> = Vec::new();

        let mut db = DBBuilder::new("test.db").build().unwrap();
        let mut tx = db.begin_rw_tx().unwrap();

        let mut flag: u8 = 0;

        {
            match tx.bucket(b"blocksBucket") {
                Ok(blocks) => {
                    tip = blocks.get(b"l").unwrap().to_vec();
                },
                Err(_)   => {
                    flag = 1;          
                },
            }
        }

        if flag == 1 {
            let genesis_block = Block::new_genesis_block(Transaction::new_coinbase_tx(address)).unwrap();
            let mut blocks = tx.create_bucket(b"blocksBucket").unwrap();
            blocks.put(
                &genesis_block.hash.to_bytes_le(),
                genesis_block.serialize()
            ).unwrap();

            blocks.put(
                b"l",
                genesis_block.hash.to_bytes_le()
            ).unwrap();    
            
            tip = genesis_block.hash.to_bytes_le();
        }

        BlockChainDb {
            tip: tip,
            db: db,
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let last_hash;
        {
            let tx = self.db.begin_tx().unwrap();
            let blocks = tx.bucket(b"blocksBucket").unwrap();
            last_hash = blocks.get(b"l").unwrap().to_vec();
        }

        let new_block = Block::new_block(transactions, BigUint::from_bytes_le(&last_hash[..])).unwrap();

        let mut tx = self.db.begin_rw_tx().unwrap();
        let mut blocks = tx.bucket_mut(b"blocksBucket").unwrap();

        blocks.put(
            &new_block.hash.to_bytes_le(),
            new_block.serialize()
        ).unwrap();

        blocks.put(
            b"l",
            new_block.hash.to_bytes_le()
        ).unwrap();    

        self.tip = new_block.hash.to_bytes_le();
    }

    pub fn show_blockchain(&self) {
        let mut blockchainiterator = BlockchainIterator::new(&self);

        loop {
            if let Some(block) = blockchainiterator.next() {
                //println!("{:?}", block)
                Block::show_block(&block);
            }
            else {
                break;
            }
        }     
    }

    pub fn find_spendable_transaction_output_index(&self, pool: &Pool, address: &str, amount: i64) -> (i64, HashMap<BigUint, Vec<usize>>) {

        let mut sum: i64 = 0;
        let mut spendable_table: HashMap<BigUint, Vec<usize>> = HashMap::new();
        let mut flag: u8 = 0;

        let transactions = self.find_unspent_transactions(pool, address);
        for transaction in transactions {
            for (output_index, output) in transaction.vout.iter().enumerate() {
                if output.can_be_unlocked_with(address) && sum < amount {
                    sum += output.value;
                    spendable_table.entry(transaction.id.clone()).or_insert(Vec::new()).push(output_index);

                    if sum >= amount {
                        flag = 1;
                        break;
                    }
                }
            }
            if flag == 1 {
                flag = 0;
                break;
            }
        }
        (sum, spendable_table)
    }

    pub fn find_unspent_transactions(&self, pool: &Pool, address: &str) -> Vec<Transaction> {

        let mut unspent_transactions: Vec<Transaction> = Vec::new();
        let mut spent_table: HashMap<BigUint, Vec<usize>> = HashMap::new();

        let mut flag: u8 = 0;
        //pool
        for transaction in pool.pool.iter().rev() {
            for (output_index, output) in transaction.vout.iter().enumerate() {
                if let Some(temp) = spent_table.get(&transaction.id) {
                    for &i in temp {
                        if i == output_index {
                            flag = 1;
                            break;
                        }
                    }
                }

                if flag == 1 {
                    flag = 0;
                    break;
                }

                if output.can_be_unlocked_with(address) {
                    unspent_transactions.push(transaction.clone())
                }
            }

            if transaction.is_coinbase() == false {
                for input in &transaction.vin {
                    if input.can_unlock_output_with(address) {
                        spent_table.entry(input.txid.clone()).or_insert(Vec::new()).push(input.vout_index);
                    }
                }
            }
        }



        //blockchain
        flag = 0;

        let mut blockchainiterator = BlockchainIterator::new(&self);

        loop {
            if let Some(block) = blockchainiterator.next() {
                for transaction in block.transactions {
                    for (output_index, output) in transaction.vout.iter().enumerate() {
                        if let Some(temp) = spent_table.get(&transaction.id) {
                            for &i in temp {
                                if i == output_index {
                                    flag = 1;
                                    break;
                                }
                            }
                        }
        
                        if flag == 1 {
                            flag = 0;
                            break;
                        }
        
                        if output.can_be_unlocked_with(address) {
                            unspent_transactions.push(transaction.clone())
                        }
                    }
        
                    if transaction.is_coinbase() == false {
                        for input in &transaction.vin {
                            if input.can_unlock_output_with(address) {
                                spent_table.entry(input.txid.clone()).or_insert(Vec::new()).push(input.vout_index);
                            }
                        }
                    }
                }                
            }
            else {
                break;
            }
        }             



        unspent_transactions
    }

    pub fn get_balance(&self, address: &str) -> i64 {
        let transactions = self.find_unspent_transactions(&Pool::new_pool(), address);
        let mut sum: i64 = 0;

        for transaction in transactions {
            for temp in transaction.vout {
                if temp.can_be_unlocked_with(address) {
                    sum += temp.value;
                }
            }
        }

        sum
    }
}

pub struct BlockchainIterator {
    tip: Vec<u8>,
    db: nut::DB,    
}

impl BlockchainIterator {
    pub fn new(temp: &BlockChainDb) -> BlockchainIterator {
        BlockchainIterator {
            tip: temp.tip.clone(),
            db: temp.db.clone(),
        }
    }
}

impl Iterator for BlockchainIterator {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {

        let tx = self.db.begin_tx().unwrap();

        let blocks = tx.bucket(b"blocksBucket").unwrap();
        if let Some(temp) = blocks.get(&self.tip[..]) {
            let block: Block = Block::deserialize(&temp.to_vec());
            self.tip = block.prev_block_hash.to_bytes_le();

            return Some(block)
        }
        else {
            return None;
        }
    }
}