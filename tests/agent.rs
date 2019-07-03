extern crate multiagent;

use account::*;
use asset::*;
use agent::*;
use multiagent::*;
use rate::*;
use std::collections::HashMap;
use std::time::Instant;

fn mission_default() -> Account {
    Account(hashmap![
        Asset::MissionTime => Quantity(1000000),
    ])
}

fn agent_default() -> Account {
    Account(hashmap![
        Asset::MissionTime => Quantity(1),
        Asset::Trust => Quantity(10000),
        Asset::EnlistCertificate(Instant::now()) => Quantity(1),
        Asset::Resource(Resource::Battery) => Quantity(10000),
        Asset::Resource(Resource::RgbSensor) => Quantity(10000),
        Asset::Resource(Resource::ThermalSensor) => Quantity(10000),
        Asset::Resource(Resource::PoseEstimation) => Quantity(10000),
    ])
}

fn rates_default() -> HashMap<Exchange, Rate> {
    hashmap![
        Exchange::MissionTimeWithResource =>
        Rate {
            credit: hashmap![Asset::MissionTime => Quantity(1)],
            debit: hashmap![
                Asset::Resource(Resource::Battery) => Quantity(20),
                Asset::Resource(Resource::ThermalSensor) => Quantity(9),
                Asset::Resource(Resource::RgbSensor) => Quantity(3),
                Asset::Resource(Resource::PoseEstimation) => Quantity(1),
            ],
        },
        Exchange::MissionTimeWithTrust =>
        Rate {
            credit: hashmap![Asset::MissionTime => Quantity(1)],
            debit: hashmap![Asset::Trust => Quantity(1)],
        },
    ]
}

#[test]
fn agent_lifetime_until_death() {
    let mission = mission_default();
    let rates = rates_default();

    let mut agent = Agent {
        account: agent_default(),
        is_alive: true,
    };

    while agent.is_alive {
        agent.simulate(&rates, &mission);
    }

    let Quantity(total_secs) = agent.account.quantity(&Asset::MissionTime);

    let hours = (total_secs / 60) / 60;
    let mins = (total_secs - hours * 60 * 60) / 60;
    let secs = total_secs - (hours * 60 * 60 + mins * 60);

    println!("{:#?}", agent.account);
    println!(
        "RIP! Agent was alive for {} hours, {} minutes and {} seconds.",
        hours, mins, secs
    );
}
