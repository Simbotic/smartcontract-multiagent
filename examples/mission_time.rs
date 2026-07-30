use multiagent::{
    Account, Basket, ExchangeError, Ledger, Policy, Quantity, Rate, TerminationCondition,
};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Resource {
    Battery,
    RgbSensor,
    ThermalSensor,
    PoseEstimation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Asset {
    Resource(Resource),
    MissionTime,
    Trust,
    EnlistCertificate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AccountId {
    Agent,
    Mission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RateId {
    MissionTimeWithResource,
    MissionTimeWithTrust,
}

struct AgentState {
    is_alive: bool,
    ledger: Ledger<AccountId, Asset>,
}

struct AgentView {
    is_alive: bool,
}

struct ResourceThenTrust;

impl Policy<AgentView, [RateId; 2]> for ResourceThenTrust {
    fn decide(&mut self, view: &AgentView) -> Option<[RateId; 2]> {
        view.is_alive.then_some([
            RateId::MissionTimeWithResource,
            RateId::MissionTimeWithTrust,
        ])
    }
}

struct AgentIsDead;

impl TerminationCondition<AgentState> for AgentIsDead {
    fn is_terminal(&self, state: &AgentState) -> bool {
        !state.is_alive
    }
}

fn basket<const N: usize>(entries: [(Asset, u64); N]) -> Basket<Asset> {
    entries
        .map(|(asset, quantity)| (asset, Quantity::new(quantity)))
        .into()
}

fn rates() -> HashMap<RateId, Rate<Asset>> {
    HashMap::from([
        (
            RateId::MissionTimeWithResource,
            Rate::new(
                basket([(Asset::MissionTime, 1)]),
                basket([
                    (Asset::Resource(Resource::Battery), 20),
                    (Asset::Resource(Resource::ThermalSensor), 9),
                    (Asset::Resource(Resource::RgbSensor), 3),
                    (Asset::Resource(Resource::PoseEstimation), 1),
                ]),
            ),
        ),
        (
            RateId::MissionTimeWithTrust,
            Rate::new(
                basket([(Asset::MissionTime, 1)]),
                basket([(Asset::Trust, 1)]),
            ),
        ),
    ])
}

fn ledger() -> Ledger<AccountId, Asset> {
    let mut ledger = Ledger::new();
    ledger.insert(
        AccountId::Mission,
        Account::new(basket([(Asset::MissionTime, 1_000_000)])),
    );
    ledger.insert(
        AccountId::Agent,
        Account::new(basket([
            (Asset::MissionTime, 1),
            (Asset::Trust, 10_000),
            (Asset::EnlistCertificate, 1),
            (Asset::Resource(Resource::Battery), 10_000),
            (Asset::Resource(Resource::RgbSensor), 10_000),
            (Asset::Resource(Resource::ThermalSensor), 10_000),
            (Asset::Resource(Resource::PoseEstimation), 10_000),
        ])),
    );
    ledger
}

fn main() -> Result<(), Box<dyn Error>> {
    let rates = rates();
    let mut state = AgentState {
        is_alive: true,
        ledger: ledger(),
    };
    let mut policy = ResourceThenTrust;
    let termination = AgentIsDead;

    while !termination.is_terminal(&state) {
        let view = AgentView {
            is_alive: state.is_alive,
        };
        let mut purchased_mission_time = false;

        if let Some(rate_ids) = policy.decide(&view) {
            for rate_id in rate_ids {
                let rate = rates
                    .get(&rate_id)
                    .expect("the policy returned an unknown rate");

                match state.ledger.exchange(
                    &AccountId::Agent,
                    &AccountId::Mission,
                    rate,
                    Quantity::new(1),
                ) {
                    Ok(_) => {
                        purchased_mission_time = true;
                        break;
                    }
                    Err(ExchangeError::InsufficientBalance {
                        account: AccountId::Agent,
                        ..
                    }) => {}
                    Err(error) => return Err(Box::new(error)),
                }
            }
        }

        state.is_alive = purchased_mission_time;
    }

    let lifetime = state
        .ledger
        .account(&AccountId::Agent)
        .expect("agent account exists")
        .balance(&Asset::MissionTime)
        .get();
    let hours = lifetime / 3_600;
    let minutes = (lifetime % 3_600) / 60;
    let seconds = lifetime % 60;

    println!("RIP! Agent was alive for {hours} hours, {minutes} minutes and {seconds} seconds.");

    Ok(())
}
