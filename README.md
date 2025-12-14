# AMM Contract

A Solana automated market maker (AMM - CONSTANT PRODUCT)  smart contract built using the Anchor framework.

This program allows users to:
- Initialize a liquidity pool
- Provide liquidity to the pool
- Swap between two tokens
- Withdraw liquidity using LP tokens

---

## Features

- Initialize an AMM liquidity pool
- Provide liquidity to the pool
- Swap tokens with slippage protection
- Withdraw liquidity using LP tokens
- Configurable swap fee

---

## Instructions

The program exposes the following instructions:

- **initialize_pool**  
  Initializes a new AMM pool with a swap fee and initial LP supply.

- **provide_liquidity**  
  Deposits token X and token Y into the pool and mints LP tokens.

- **swap**  
  Swaps one token for another using the pool’s pricing logic.

- **withdraw**  
  Burns LP tokens and withdraws liquidity from the pool.

---

## Local Setup

### Prerequisites

- Rust
- Solana CLI
- Anchor

---

### Clone the Repository

```bash
git clone https://github.com/ajeeshRS/amm-cp
cd amm-cp
```

### Build the Program

```
anchor build
```

### Deploy the Program

```
anchor deploy
```

## Security Notes

> ⚠️ **Note:** Tests are **not implemented yet**.
- This contract is experimental
- Not audited
- Do not deploy to mainnet without proper testing and auditing

## License

MIT
