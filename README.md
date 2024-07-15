# Xen DAO Voting

Xen DAO Voting is a decentralized autonomous organization (DAO) voting system built on the Solana blockchain using the Anchor framework.

## Features

- Create and manage DAOs
- Create proposals within a DAO
- Vote on proposals
- Close proposals
- Track voting history and prevent double voting

## Prerequisites

- Rust
- Solana CLI
- Anchor Framework
- Node.js and npm

## Installation

1. Clone the repository:
```
git clone https://github.com/yourusername/xen-dao-voting.git
```

```
cd xen-dao-voting
```

2. Install dependencies:
```
npm install
```

3. Build the program:
```
anchor build
```

5. Start a local Solana cluster:
```
solana-test-validator
```

7. Deploy the program:
```
anchor deploy
```

9. Run the tests:
```
anchor test
```

## Structure

- `programs/xen-dao-voting/src/lib.rs`: Contains the main program logic
- `tests/xen-dao-voting.ts`: Contains the test suite for the program



