// use core::fmt;
use sha2::{Digest, Sha256};
// use std::fmt::format;
use std::fmt;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

// define difficulty of mining
const DIFFICULTY: usize = 2;

//define the structure of a block in the blockchain
struct Block {
    index: u32,
    previous_hash: String,
    timestamp: u64,
    data: String,
    hash: String,
    nonce: u64,
}

impl Block {
    fn new(index: u32, previous_hash: String, data: String) -> Block {
        let timestamp: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backward")
            .as_secs();
        Block {
            index,
            previous_hash,
            timestamp,
            data,
            nonce: 0,
            hash: String::new(),
        }
    }

    fn calculate_hash(&mut self) -> String {
        let data: String = format!(
            "{}{}{}{}{}",
            self.index, &self.previous_hash, &self.timestamp, &self.data, self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();

        let hash_str: String = format!("{:x}", result);
        hash_str
    }

    fn mine_block_with_visual_effects(&mut self) {
        let mut iterations: i32 = 0;
        loop {
            self.hash = self.calculate_hash();
            iterations += 1;
            if !self.hash.is_empty() && &self.hash[..DIFFICULTY] == "00".repeat(DIFFICULTY) {
                println!("Block mined: {}", self.hash);
                break;
            }
            // if iterations
            if iterations > 100 {
                print!("Mining in progress...");
                thread::sleep(Duration::from_millis(3000));
                println!("Calculated hash: {}", self.hash);
                break;
            }
            self.nonce += 1;
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime = chrono::DateTime::from_timestamp(self.timestamp as i64, 0);
        write!(f, "Block {}: {} at {:?}", self.index, self.data, datetime)
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        let genesis_block: Block = Block::new(0, String::new(), String::from("Genesis Block"));
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    fn add_block(&mut self, mut new_block: Block) {
        let previous_hash: String = self.chain.last().unwrap().hash.clone();
        new_block.previous_hash = previous_hash;
        new_block.mine_block_with_visual_effects();
        self.chain.push(new_block)
    }

    fn get_total_blocks(self) -> usize {
        self.chain.len()
    }
}

fn main() {
    println!(" 🚀 Welcome to bekcoin Mining Simulator! 🚀");
    println!(" 🧍🏻‍♂️ Enter your name");

    let mut miner_name: String = String::new();

    std::io::stdin()
        .read_line(&mut miner_name)
        .expect("Failed to read input");

    miner_name = miner_name.trim().to_string();

    let traders_names: Vec<&str> = vec![
        "Bob", "Linda", "John", "Omar", "Eve", "Svetlana", "Grace", "Jiro",
    ];

    let mut bekcoin: Blockchain = Blockchain::new();

    println!("\n ⛏ Let's start mining and simulating transactions!\n");

    let mut sender: String = miner_name.clone();

    for i in 0..traders_names.len() {
        println!("Mining block {}... ⛏", i + 1);
        let recipient: String = if i < traders_names.len() - 1 {
            traders_names[i + 1].to_string()
        } else {
            miner_name.clone()
        };
        let transactions = format!("{} sent to {}", sender, recipient);

        let new_block: Block = Block::new((i + 1) as u32, String::new(), transactions.clone());

        bekcoin.add_block(new_block);

        println!("Transaction: {}", transactions);

        sender = recipient;

        println!()
    }
    let total_blocks: usize = bekcoin.get_total_blocks();

    println!("✅ Total blocks added to the blockchain: {}", total_blocks);

    let bekcoin_per_block: usize = 137;
    let beckcoin_traded: usize = total_blocks * bekcoin_per_block;

    println!("💰 Total Bekcoin traded: {} Bekcoin", beckcoin_traded);

    let end_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwords")
        .as_secs();

    let end_datetime = chrono::DateTime::from_timestamp(end_timestamp as i64, 0);

    println!(" ⏱  Simulation ended at: {:?}", end_datetime);

    println!(" 🎉 Congrats Mining operation completed successfully ");

    let n = 99;
    let mut r = n;
    r += 1;
    println!("{n}");

    let mut i_like = "cats";
    {
        i_like = "dogs";
    }
    println!("{}", i_like)
}
