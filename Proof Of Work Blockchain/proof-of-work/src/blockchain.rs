
use block::Block;
use mining_error::MiningError;

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    /// Initializes a new blockchain with a genesis block
    pub fn new() -> Result<Self, MiningError> {
        let genesis_block = Block::genesis()?;
        println!("Genesis block mined after {} tries", genesis_block.nonce());
        Ok(Self { blocks: vec![genesis_block] })
    }

    /// Add a newly-mined block to the blockchain
    pub fn add_block(&mut self, data: &str) -> Result<u64, MiningError> {
        let block: Block;

        match self.blocks.last() {
            Some(prev) => {
                block = Block::new(data, prev.hash())?;
            },
            // Adding a block to an empty blockchain is an error,
            // a genesis block must be created first
            None => {
                return Err(MiningError::NoParent);
            },
        }

        let nonce: u64 = block.nonce();

        self.blocks.push(block);

        Ok(nonce)
    }

    /// A method that iterates over the Blockchain's blocks and prints out information for each
    pub fn traverse(&self) {
        for (i, block) in self.blocks.iter().enumerate() {
            println!("block: {}", i);
            println!("prev: {:?}", block.prev_hash());
            println!("hash: {:?}", block.hash());
            println!("data: {:?}", block.pretty_data());
            println!();
        }
    }
}
