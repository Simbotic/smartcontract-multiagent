# Multiagent closed economic state machine

`multiagent` is building a closed computational system in which every
semantically meaningful part of a problem is represented through assets,
accounts, rates, and exchanges.

The fundamental rule is:

> Nothing semantically authoritative may live outside assets, accounts, rates,
> and exchanges.

Position, graph connectivity, time, energy, goals, constraints, lifecycle,
memory, uncertainty, planner statistics, and rewards must all be encoded
through the same ruleset. The ledger is not one component of an external world
state. The closed economic state is the world.

See [PDD.md](PDD.md) for the complete product philosophy, current technical
contract, known gaps, and roadmap.

## The model

```text
State       = Account × Asset → Quantity
Problem     = Initial state + Available rates
Computation = A sequence of valid exchanges
Goal        = A desired asset configuration
Solution    = A replayable exchange trace reaching that configuration
```

The four primitives have deliberately broad meanings:

| Primitive | Meaning |
| --- | --- |
| Asset | A resource, fact, proposition, capability, condition, memory item, or state token |
| Account | An owner, actor, location, scope, branch, or namespace |
| Rate | A law specifying which asset configuration may become another |
| Exchange | A concrete firing of a rate and therefore a state transition |

Users define the vocabulary of their problem, but they do not maintain a
parallel authoritative world.

## Solvers over one source of truth

BFS, Dijkstra, A*, constraint and OR solvers, Monte Carlo simulation, MCTS,
reinforcement learning, game-theoretic policies, and LLM planners can all
operate over the same closed system.

They may:

- Inspect core-derived economic views.
- Enumerate, prioritize, or simulate exchanges.
- Fork states for search.
- Compile bounded problems into specialized solver representations.
- Propose an exchange sequence.

They may not bypass core semantics or install hidden state. A proposed solution
is accepted only after it is translated into exchanges and replayed through the
core.

> Solvers may propose. Only the asset/account/rate/exchange system defines and
> validates reality.

## Current implementation

Version `0.1.0` is a correctness-oriented bilateral foundation for the larger
model:

- `Quantity` is a non-negative integer amount with checked arithmetic.
- `Basket<A>` is a sparse collection of user-defined asset quantities.
- `Account<A>` owns balances.
- `Rate<A>` currently describes what a buyer receives and pays.
- `Ledger<AccountId, A>` owns accounts and executes atomic bilateral
  exchanges.
- `Receipt` records a successful exchange.
- Structured errors report exact shortfalls and arithmetic failures.

Assets and account identifiers are ordinary user-defined Rust types. They need
only standard traits such as `Eq`, `Hash`, and `Clone`.

The current bilateral API is not yet the complete closed machine. General
problem spaces require rates that can atomically consume, produce, and preserve
assets across multiple account roles. The current policy, termination, mutable
account, and mission-example APIs also contain known closure escape hatches
documented in the PDD.

## Current API example

```rust
use multiagent::{Account, Basket, Ledger, Quantity, Rate};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Asset {
    Coin,
    MissionTime,
}

let mut ledger = Ledger::new();
ledger.insert(
    "agent",
    Account::new(Basket::from([
        (Asset::Coin, Quantity::new(10)),
    ])),
);
ledger.insert(
    "mission",
    Account::new(Basket::from([
        (Asset::MissionTime, Quantity::new(10)),
    ])),
);

let rate = Rate::new(
    Basket::from([(Asset::MissionTime, Quantity::new(1))]),
    Basket::from([(Asset::Coin, Quantity::new(2))]),
);

let receipt = ledger.exchange(
    &"agent",
    &"mission",
    &rate,
    Quantity::new(3),
)?;

assert_eq!(receipt.units(), Quantity::new(3));
# Ok::<(), multiagent::ExchangeError<&str, Asset>>(())
```

The ledger validates both sides and every arithmetic operation before
committing. A failed exchange leaves the complete stored ledger unchanged.

## First proof problem: pathfinding

The next architectural milestone is pathfinding encoded entirely inside the
four primitives:

- Position is an asset.
- Nodes are accounts or structured assets.
- Edges are assets or rates.
- Movement is an exchange.
- Time, energy, cost, frontier state, and parent relationships are assets.
- The goal is an asset configuration producing `Solved`.
- The answer is a replayable exchange trace.

BFS and A* must solve exactly the same closed encoding without external
position, graph, cost, or goal state. If this representation is elegant—not
merely possible—it validates the model beyond traditional economic exchange.

## Mission-time example

The original resource-first, trust-second scenario remains available:

```console
cargo run --example mission_time
```

It demonstrates the current generic bilateral API and still produces:

```text
RIP! Agent was alive for 2 hours, 55 minutes and 1 seconds.
```

It is intentionally documented as transitional: agent life is currently an
external Boolean. The closed model will replace it with `Alive` and `Dead`
assets and a rate-driven lifecycle transition.

## Development

The crate uses Rust Edition 2024 and supports Rust 1.85 or newer. It currently
has no third-party dependencies.

```console
cargo test --all-targets
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
```
