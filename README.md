# Simulation for a provably correct multiagent economy 
An experimental economy simulation designed to model multiagent systems. A category theory inspired smart contract, where objects are assets and morphisms are rates, creating a mathematical formalization for an exchange (assets, accounts, rates and transactions).

The end goal is to optimize for provably fair behavioral economics design and provide dense rewards for reinforcement learning.

## Reinforcement Learning
Encoding mechanics and behaviors into economy primitives, effectively creating an economy algebra of the problem domain, has the added benefit of improving sample efficiency of reinforcement learning models by providing a fully dense reward state space. Reward shaping would be more dynamic as it would emerge from traditional economic activity and game theory.

## Test this first iteration:
- In every tick agent should be able to purchase 1 MissionTime.
- First it tries to purchase MissionTime with its Resource through Exchange::MissionTimeWithResource.
- If this fails, it will try to purchase through Exchange::MissionTimeWithTrust.
- If agent cannot purchase any more MissionTime it dies.

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
