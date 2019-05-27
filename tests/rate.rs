extern crate multiagent;

use account::*;
use asset::*;
use multiagent::*;
use rate::*;
use std::collections::HashMap;

fn mission_default() -> Account {
    Account(hashmap![
        Asset::MissionTime => Quantity(1000000),
    ])
}

fn agent_default() -> Account {
    Account(hashmap![
        Asset::Reward(Reward::Score) => Quantity(10000),
        Asset::Reward(Reward::Token) => Quantity(10000),
        Asset::Reward(Reward::Prediction) => Quantity(10000),
        Asset::Reward(Reward::Policy) => Quantity(10000),
    ])
}

fn rates_default() -> HashMap<Exchange, Rate> {
    hashmap![
        Exchange::MissionTimeWithResource =>
        Rate {
            credit: hashmap![Asset::MissionTime => Quantity(1)],
            debit: hashmap![
                Asset::Reward(Reward::Prediction) => Quantity(9),
                Asset::Reward(Reward::Token) => Quantity(3),
                Asset::Reward(Reward::Policy) => Quantity(1),
            ],
        },
    ]
}

#[test]
fn rate_buy_lifetime() {
    let mission = mission_default();
    let agent = agent_default();
    let rates = rates_default();
    let rate = rates.get(&Exchange::MissionTimeWithResource).unwrap();

    let res_seller = Account(hashmap![
        Asset::MissionTime => Quantity(999999),
        Asset::Reward(Reward::Token) => Quantity(3),
        Asset::Reward(Reward::Prediction) => Quantity(9),
        Asset::Reward(Reward::Policy) => Quantity(1),
    ]);

    let res_buyer = Account(hashmap![
        Asset::MissionTime => Quantity(1),
        Asset::Reward(Reward::Score) => Quantity(10000),
        Asset::Reward(Reward::Token) => Quantity(9997),
        Asset::Reward(Reward::Prediction) => Quantity(9991),
        Asset::Reward(Reward::Policy) => Quantity(9999),
    ]);

    match Account::exchange(rate, Quantity(1), &agent, &mission) {
        Tranx::Approved(buyer, seller) => {
            assert_eq!(res_seller, seller);
            assert_eq!(res_buyer, buyer);
        }
        _ => assert!(false),
    }
}

#[test]
fn rate_buy_lifetime_quantity() {
    let mission = mission_default();
    let agent = agent_default();
    let rates = rates_default();
    let rate = rates.get(&Exchange::MissionTimeWithResource).unwrap();

    let res_seller = Account(hashmap![
        Asset::MissionTime => Quantity(999998),
        Asset::Reward(Reward::Token) => Quantity(6),
        Asset::Reward(Reward::Prediction) => Quantity(18),
        Asset::Reward(Reward::Policy) => Quantity(2),
    ]);

    let res_buyer = Account(hashmap![
        Asset::MissionTime => Quantity(2),
        Asset::Reward(Reward::Score) => Quantity(10000),
        Asset::Reward(Reward::Token) => Quantity(9994),
        Asset::Reward(Reward::Prediction) => Quantity(9982),
        Asset::Reward(Reward::Policy) => Quantity(9998),
    ]);

    match Account::exchange(rate, Quantity(2), &agent, &mission) {
        Tranx::Approved(buyer, seller) => {
            assert_eq!(res_seller, seller);
            assert_eq!(res_buyer, buyer);
        }
        _ => assert!(false),
    }
}
