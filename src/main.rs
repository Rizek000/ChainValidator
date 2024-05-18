use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
struct Block {
    index: u32,
    timestamp: u128,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u32, timestamp: u128, data: String, previous_hash: String, hash: String) -> Self {
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    fn calculate_hash(index: u32, timestamp: u128, data: &str, previous_hash: &str) -> String {
        format!("{:x}", md5::compute(format!("{}{}{}{}", index, timestamp, data, previous_hash)))
    }

    fn is_valid(&self) -> bool {
        self.hash == Block::calculate_hash(self.index, self.timestamp, &self.data, &self.previous_hash)
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Block {{ index: {}, timestamp: {}, data: {}, previous_hash: {}, hash: {} }}",
            self.index, self.timestamp, self.data, self.previous_hash, self.hash
        )
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        Blockchain { chain: Vec::new() }
    }

    fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if !current_block.is_valid() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}

impl Display for Blockchain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for block in &self.chain {
            writeln!(f, "{}", block)?;
        }
        Ok(())
    }
}

fn create_mockup_file(filename: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).create(true).open(filename)?;

    let blocks = vec![
        Block::new(0, current_timestamp(), "Genesis Block".to_string(), "0".to_string(), Block::calculate_hash(0, current_timestamp(), "Genesis Block", "0")),
        Block::new(1, current_timestamp(), "First Block".to_string(), Block::calculate_hash(0, current_timestamp(), "Genesis Block", "0"), Block::calculate_hash(1, current_timestamp(), "First Block after Genesis", &Block::calculate_hash(0, current_timestamp(), "Genesis Block", "0"))),
        Block::new(2, current_timestamp(), "Second Block".to_string(), Block::calculate_hash(1, current_timestamp(), "First Block after Genesis", &Block::calculate_hash(0, current_timestamp(), "Genesis Block", "0")), Block::calculate_hash(2, current_timestamp(), "Second Block after Genesis", &Block::calculate_hash(1, current_timestamp(), "First Block after Genesis", &Block::calculate_hash(0, current_timestamp(), "Genesis Block", "0"))))
    ];

    for block in blocks {
        writeln!(file, "{},{},{},{},{},{}", block.index, block.timestamp, block.data, block.previous_hash, block.hash, "true")?;
    }

    Ok(())
}

fn read_blocks_from_file(filename: &str) -> io::Result<Vec<Block>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut blocks = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 5 {
            let index = parts[0].parse::<u32>().unwrap_or(0);
            let timestamp = parts[1].parse::<u128>().unwrap_or(0);
            let data = parts[2].to_string();
            let previous_hash = parts[3].to_string();
            let hash = parts[4].to_string();

            let block = Block::new(index, timestamp, data, previous_hash, hash);
            blocks.push(block);
        }
    }

    Ok(blocks)
}

fn write_blocks_to_file(filename: &str, blocks: &[Block]) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).truncate(true).open(filename)?;

    for block in blocks {
        writeln!(file, "{},{},{},{},{},{}", block.index, block.timestamp, block.data, block.previous_hash, block.hash, block.is_valid())?;
    }

    Ok(())
}

fn current_timestamp() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}

fn main() -> io::Result<()> {
    let filename = "hash.txt";

    if !Path::new(filename).exists() {
        println!("File does not exist. Creating a mockup file...");
        create_mockup_file(filename)?;
    }

    let blocks = read_blocks_from_file(filename)?;
    let mut blockchain = Blockchain::new();

    for block in blocks {
        blockchain.add_block(block);
    }

    println!("Is valid? {}", blockchain.is_valid());
    println!("{}", blockchain);

    write_blocks_to_file(filename, &blockchain.chain)?;

    Ok(())
}
