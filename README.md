# Solana Escrow Program

A trustless, on-chain escrow built with Anchor on Solana. Enables two parties
to swap SPL tokens without trusting each other — the program holds the funds
and enforces the exchange atomically.

## How It Works

**Alice** wants to trade Token A for Token B.  
**Bob** has Token B and wants Token A.  
Neither trusts the other. The program is the middleman.


## Instructions

### `make` — Initialize Escrow
Alice locks her Token A into a vault PDA and specifies:
- How much Token A she's depositing
- How much Token B she wants in return

```ts
await program.methods
  .make(new BN(depositAmount), new BN(receiveAmount))
  .accounts({ maker, mintA, mintB, makerAtaA, vault, escrow, ... })
  .rpc();
```

### `take` — Execute the Swap
Bob sends Token B and receives Token A. The escrow closes.

```ts
await program.methods
  .take()
  .accounts({ taker, maker, mintA, mintB, takerAtaA, takerAtaB, escrow, vault, ... })
  .rpc();
```

### `cancel` — Refund the Maker
If no taker, Alice can cancel and get her Token A back.

```ts
await program.methods
  .cancel()
  .accounts({ maker, mintA, makerAtaA, vault, escrow, ... })
  .rpc();
```


