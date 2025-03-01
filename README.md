### Week 0:

#### Task 1. Prerequisite Typescript Task

1. Generate keygen
2. Airdrop devnet Solana
3. Base58 Conversion
   1. Write a typescript function to convert a Base58 string to wallet address.
   2. Write a typescript function to convert a wallet address to Base58 string.
4. Enroll in Turbine3 using IDL

#### Task 2. Prerequisite Rust Task

1. Generate Keygen
2. Airdrop devnet Solana
3. Base58 Conversion
   a. Write a rust function to convert a Base58 string to wallet address.
   b. Write a rust function to convert a wallet address to Base58 string.
4. Enroll in Turbine3 using IDL
   1. Use the provided IDL with the updated program address to enroll in the Turbine3 program.

### Week 1:

#### Day 1: Core concepts

1. Accounts
2. Account Flags
3. Account structure
4. Programs
5. Program Flags
6. Rent
7. Rent exception
8. Transaction
9. Transaction Structure
10. Compute on Solana
11. IDL (Interface Definition Language)
12. SPL Token
13. SPL Token functions
14. PDA (Program Derived Address)

#### Day 2: Metaplex and Token

1. Metaplex - Introduction to Metaplex, metadata, master edition and collections
2. Metaplex Token standard - NFT, Semi-fungible assets and programmable NFTs.
3. Understanding URI and UMI framework which is a modular framework for creating javascript clients for Solana programs.
4. Adding metadata to SPL tokens
5. Transferring SPL token

#### Day 3: Rug Day!

1. Capstone project discussions
   1. Overview on the projects/topics and timelines
   2. Emphasis on application of anchor/solana concepts to build a functional, market ready environment
2. Created image URI and metadata URI for the project
3. Minted unique rug NFTs using metaplex

### Week 2:

#### Day 1: Vault program

1. Vault program initialization
2. Vault deposit function : using CPI context to transfer funds from signer to vault
3. Vault withdraw function: using CPI context to transfer funds back from vault to signer
4. Vault close function: close the vault and transfer all funds to signer
5. Used CPI context

#### Day 2 & 3: Escrow program

1. Discussion on User stories
2. Maker Function: Implemented a maker function to send Token A (from mint_a) to the vault.
3. Taker Function: Developed a taker function to transfer Token B (from mint_b) to mint_a and retrieve Token A from the vault.
4. Refund Function: Created a refund mechanism to return Token A from the vault in case of cancellation.
5. Token Transfers: Facilitated secure transfers of Token A to the vault and Token B to complete the escrow exchange process.

### Week 3:

#### Day 1 & 2: Marketplace Program

1. Initialize Marketplace: Set up the marketplace program to handle NFT and token listings.
2. Listing Function: Implemented a function to list NFTs and tokens for sale.
3. Delisting Function: Developed a function to remove listed items from the marketplace.
4. Purchase Function: Enabled buyers to purchase listed assets securely through the program.

#### Day 3: Automatic Market Maker (AMM) Program

1. Constant Product Curve: Implemented the constant product formula (x \* y = k) to maintain liquidity balance.
2. Learnt about Liquidity Pools: Create liquidity pools to facilitate decentralized token swaps.

### Week 4:

#### Day 1: Automatic Market Maker (AMM) Program

1. Initialization

- Set up the AMM program with a provided seed, an optional authority, and a fee parameter.
- Uses Anchor's context and bump seed management to ensure secure account initialization.

2. Deposit Function

- Allows users to deposit tokens into the liquidity pool, with constraints on maximum values (max_x and max_y).
- Utilizes a CPI (Cross-Program Invocation) to handle token transfers securely.

3. Withdraw Function

- Enables users to withdraw tokens from the liquidity pool by enforcing minimum constraints (min_x and min_y).
- Incorporates signer seeds for secure withdrawal operations.

#### Day 2: NFT Staking Program

1. Configuration Initialization

- Initializes staking configuration with parameters such as points per stake, maximum state, and a freeze period.
- Ensures that the staking mechanism is properly configured from the start.

2. User Registration

- Provides a mechanism to register users so they can participate in the staking process.

3. Stake Function

- Allows users to stake their NFTs, thereby participating in the staking rewards program.
- Utilizes Anchor's context for secure state management and staking logic.

#### Day 3: Dice game Program

1. Initialization

- Sets up the dice game environment by initializing the game state with a starting amount.

2. Place Bet Function

- Enables users to place bets by specifying an amount, a unique seed, and a roll prediction.
- Handles both the creation of the bet record and the deposit of the wagered amount.

3. Resolve Bet Function

- Verifies the bet outcome using an ed25519 signature to ensure integrity and authenticity.
- Processes the bet resolution based on the verified signature and game logic.

4. Refund Bet Function

- Provides a mechanism to refund the bet and close the bet account if necessary (e.g., in case of cancellation or error).
