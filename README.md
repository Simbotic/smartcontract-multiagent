# Simulation engine for a provably correct multiagent economy 
An experimental economy simulation engine designed to model multiagent systems. A category theory inspired rules engine, where objects are assets and morphisms are rates, creating a mathematical formalization for an exchange (assets, accounts, rates and transactions).

The end goal is to optimize for provably fair behavioral economics design.

## Requirements:
- Install Rust [https://rustup.rs](https://rustup.rs)
```
$ curl https://sh.rustup.rs -sSf | sh
```

## Run tests:
```
$ cargo test -- --nocapture
```

If everything worked you should see test results like this:
```
running 1 test
RIP! Agent was alive for 2 hours, 55 minutes and 1 seconds.
test agent_lifetime_until_death ... ok
```
