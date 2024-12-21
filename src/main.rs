use sha2::{Digest, Sha256};
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};
use std::thread;
use std::time::Duration;
use chrono::{DateTime, NaiveDateTime, Utc};

// Define the structure of a block in the blockchain
const DIFFICULTY: usize = 2;
// Struct and impl

struct Block {
    index: usize,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
    nonce: u64,
}

impl Block {
    fn new(index: usize, previous_hash: String, data: String) -> Block {
        let timestamp = SystemTime::now().duration_since
        (UNIX_EPOCH).expect("Time went backwards").as_secs();
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        }

    }

    fn calculate_hash(&self) -> String {
        let data: String = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp,
            self.data,
            self.previous_hash,
            self.nonce,
        );


        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();

        let hash_str: String = format!("{:x}", result);
        hash_str
    }

    fn mine_block_with_visual_effects(&mut self){
        let mut iterations: i32 = 0;
        loop {
            self.hash = self.calculate_hash();
            iterations += 1;
            if !self.hash.is_empty() && &self.hash[..DIFFICULTY] == "00".
                repeat(DIFFICULTY) {
                println!("⛏️ Block mined! Hash: {}", self.index);
                break;
            }
            if iterations > 100 {
                println!("⏳ Mining in progress... ");
                thread::sleep(Duration::from_millis(3000));
                println!("Calculated hash: {}", self.hash);
                break;
            }
            self.nonce +=1;
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime: NaiveDateTime = chrono::NaiveDateTime::from_timestamp
            (self.timestamp as i64, 0);
        write!(
            f,
            "Block #{}: {} at {}",
            self.index, self.data, datetime.format("%Y-%m-%d %H:%M:%S")
        )
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        let genesis_block = Block::new(0, String::new(), String::from("Genesis Block"));
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    fn add_block(&mut self, mut new_block: Block) {
        let previous_hash: String = self.chain.last().unwrap().hash.clone();
        new_block.previous_hash = previous_hash;
        new_block.mine_block_with_visual_effects();
        self.chain.push(new_block);
    }

    fn get_total_blocks(&self) -> usize {
        self.chain.len()
    }
}


fn main() {
    println!("🚀 Welcome to BeeCoin Mining Simulator! 🚀");

    println!("👷🏾 Enter your name miner: ");

    let mut miner_name: String = String::new();

    std::io::stdin().read_line(&mut miner_name).expect("Failed to read input");
    miner_name = miner_name.trim().to_string();

    let trader_names: Vec<&str> = vec!["Bee", "Jon", "Jurex", "Henx", "Nix", "Yon", "Jun", "Kim",
                                       "Cole", "Paul", "Mica", "Jup", "Jenny", "Amira", "Nica",
                                       "Jonas", "Novo", "Tobe", "Abba", "Ibrahim", "Yemi", "Dave",
                                       "Sol"];

    let mut beecoin: Blockchain = Blockchain::new();

    println!("\n⛏️ Let's start mining and simulating transactions!\n");

    let mut sender: String = miner_name.clone();
    for i in 0..trader_names.len() {
        println!("⛓️ Mining block {}...⛏️", i+1);
        let recipient: String = if i < trader_names.len() -1 {
           trader_names[i + 1].to_string()
        } else {
            miner_name.clone()
        };

        let transaction: String = format!("{} sent to {}", sender, recipient);

        let new_block = Block::new((i+1), String::new(), transaction.clone());
        beecoin.add_block(new_block);
        println!("📧 Transaction: {}", transaction);
        sender = recipient;
        println!();
    }

    let total_blocks = beecoin.get_total_blocks();
    println!("✅ Total blocks added to the blockchain: {}", total_blocks);
    let beecoin_per_block: usize = 137;
    let beecoin_traded = total_blocks * beecoin_per_block;
    println!("💰 Total Beecoin traded: {} Beecoin", beecoin_traded);

    let end_timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect(
        "Time went backwards").as_secs();
    let end_datetime= chrono::NaiveDateTime::from_timestamp(end_timestamp as i64,
    0);

    println!("🕒 Simualtion ended at: {}", end_datetime);
    println!("🎉 Congratulations! Mining operation completed successfully!");




}
