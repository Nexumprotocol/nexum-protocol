# NEXUM Protocol — Architecture

## Stack

```
User
 │
 ▼
Web Interface  (React + Tailwind + Wallet Adapter)
 │
 ▼
NEXUM SDK      (TypeScript — wraps all contract calls)
 │
 ▼
Solana Program (Anchor/Rust — escrow, reputation, NXM)
```

## Smart Contract Instructions

### Escrow
- `create_task` — Lock SOL/NXM into PDA escrow
- `accept_task` — Contributor claims task
- `submit_task` — Mark work done
- `approve_task` — Release funds to contributor
- `dispute_task` — Open NXM staker vote

### Reputation
- `mint_reputation_sbt` — Non-transferable SBT on completion
- `get_reputation_score` — Query score on-chain

### NXM Token
- SPL Token with governance extensions
- Staking vault for reputation boost
- Fee collection and weekly distribution
