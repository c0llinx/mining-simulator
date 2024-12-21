# BeeCoin Mining Simulator

BeeCoin Mining Simulator is a simple blockchain mining simulation program written in Rust. It demonstrates the basic concepts of blockchain technology, including blocks, mining, and transactions, while adding a fun and interactive experience for users.

---

## Features

- **Blockchain Simulation**: Implements a basic blockchain with blocks containing index, timestamp, data, previous hash, hash, and nonce.
- **Proof of Work Mining**: Mines blocks by calculating hashes until the desired difficulty level is reached.
- **Visual Effects**: Displays progress during mining.
- **Transactions**: Simulates a series of transactions between a miner and various traders.
- **Custom Miner Name**: Allows the user to enter their name as the miner.

---

## How It Works

1. The program initializes a blockchain with a genesis block.
2. The user enters their name to become the miner.
3. Blocks are mined sequentially, with each block containing a transaction between traders or the miner.
4. The hash of each block is calculated to satisfy the Proof of Work difficulty.
5. The total number of blocks mined and BeeCoin traded is displayed at the end of the simulation.

---

## Requirements

- Rust (latest stable version)

---

## Installation

1. Clone this repository:

   ```bash
   git clone https://github.com/your-username/beecoin-mining-simulator.git
   ```

2. Navigate to the project directory:

   ```bash
   cd beecoin-mining-simulator
   ```

3. Build the project:

   ```bash
   cargo build
   ```

4. Run the program:

   ```bash
   cargo run
   ```

---

## Usage

1. Launch the program.
2. Enter your name as the miner.
3. Watch as blocks are mined and transactions are simulated.
4. Review the total number of blocks mined and BeeCoin traded at the end.

---

## Example Output

```
🚀 Welcome to BeeCoin Mining Simulator! 🚀
👷🏾 Enter your name miner: 

⛏️ Let's start mining and simulating transactions!

⛓️ Mining block 1...⛏️
📧 Transaction: MinerName sent to Bee

⛓️ Mining block 2...⛏️
📧 Transaction: Bee sent to Jon

... (continued for all blocks) ...

✅ Total blocks added to the blockchain: 24
💰 Total BeeCoin traded: 3288 BeeCoin
🕒 Simulation ended at: 2024-12-21 15:30:00
🎉 Congratulations! Mining operation completed successfully!
```

---

## Customization

- **Difficulty**: Adjust the `DIFFICULTY` constant to change the mining difficulty.
- **Trader Names**: Update the `trader_names` vector to include your own set of names.
- **BeeCoin Per Block**: Modify the `beecoin_per_block` variable to change the reward per block.

---

## Concepts Demonstrated

- Blockchain structure and mechanics
- Proof of Work (PoW)
- Hashing using SHA-256
- Timestamps and block linking

---

## Dependencies

The program uses the following Rust crates:

- [sha2](https://crates.io/crates/sha2): For SHA-256 hashing.
- [chrono](https://crates.io/crates/chrono): For working with dates and times.

To install these dependencies, run:

```bash
cargo build
```
