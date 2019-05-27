extern crate multiagent;

use multiagent::*;
use account::*;
use asset::*;

#[test]
fn accounts_equal_exisiting_assets() {
    let lhs = Account(hashmap![
        Asset::Reward(Reward::Score) => Quantity(500),
        Asset::Reward(Reward::Token) => Quantity(10000),
        Asset::Reward(Reward::Prediction) => Quantity(800),
        Asset::Reward(Reward::Policy) => Quantity(100),
    ]);
    let rhs = Account(hashmap![
        Asset::Reward(Reward::Score) => Quantity(500),
        Asset::Reward(Reward::Prediction) => Quantity(800),
        Asset::Reward(Reward::Policy) => Quantity(100),
        Asset::Reward(Reward::Token) => Quantity(10000),
    ]);
    assert_eq!(lhs, rhs);
}

#[test]
fn accounts_equal_missing_assets() {
    let lhs = Account(hashmap![
        Asset::Reward(Reward::Score) => Quantity(0),
        Asset::Reward(Reward::Token) => Quantity(10000),
        Asset::Reward(Reward::Policy) => Quantity(100),
    ]);
    let rhs = Account(hashmap![
        Asset::Reward(Reward::Prediction) => Quantity(0),
        Asset::Reward(Reward::Policy) => Quantity(100),
        Asset::Reward(Reward::Token) => Quantity(10000),
    ]);
    assert_eq!(lhs, rhs);
}

#[test]
fn accounts_not_equal_existing() {
    let lhs = Account(hashmap![
        Asset::Reward(Reward::Score) => Quantity(500),
        Asset::Reward(Reward::Token) => Quantity(10000),
        Asset::Reward(Reward::Prediction) => Quantity(800),
        Asset::Reward(Reward::Policy) => Quantity(100),
    ]);
    let rhs = Account(hashmap![
        Asset::Reward(Reward::Score) => Quantity(500),
        Asset::Reward(Reward::Token) => Quantity(10000),
        Asset::Reward(Reward::Prediction) => Quantity(800),
        Asset::Reward(Reward::Policy) => Quantity(10),
    ]);
    assert!(lhs != rhs);
}

#[test]
fn accounts_sub_existing_assets() {
    let lhs = Account(hashmap![
        Asset::Reward(Reward::Score) => Quantity(500),
        Asset::Reward(Reward::Token) => Quantity(10000),
        Asset::Reward(Reward::Prediction) => Quantity(800),
        Asset::Reward(Reward::Policy) => Quantity(100),
    ]);
    let rhs = Account(hashmap![
        Asset::Reward(Reward::Token) => Quantity(10000),
        Asset::Reward(Reward::Score) => Quantity(250),
        Asset::Reward(Reward::Policy) => Quantity(200),
        Asset::Reward(Reward::Prediction) => Quantity(700),
    ]);
    let res = Account(hashmap![
        Asset::Reward(Reward::Prediction) => Quantity(100),
        Asset::Reward(Reward::Score) => Quantity(250),
        Asset::Reward(Reward::Token) => Quantity(0),
        Asset::Reward(Reward::Policy) => Quantity(-100),
    ]);
    assert_eq!(&lhs - &rhs, res);
}

#[test]
fn accounts_sub_missing_assets() {
    let lhs = Account(hashmap![
        Asset::Reward(Reward::Score) => Quantity(500),
        Asset::Reward(Reward::Prediction) => Quantity(800),
        Asset::Reward(Reward::Policy) => Quantity(100),
    ]);
    let rhs = Account(hashmap![
        Asset::Reward(Reward::Score) => Quantity(250),
        Asset::Reward(Reward::Token) => Quantity(10000),
        Asset::Reward(Reward::Prediction) => Quantity(700),
    ]);
    let res = Account(hashmap![
        Asset::Reward(Reward::Score) => Quantity(250),
        Asset::Reward(Reward::Token) => Quantity(-10000),
        Asset::Reward(Reward::Prediction) => Quantity(100),
        Asset::Reward(Reward::Policy) => Quantity(100),
    ]);
    assert_eq!(&lhs - &rhs, res);
}

#[test]
fn accounts_add_existing_assets() {
    let lhs = Account(hashmap![
        Asset::Reward(Reward::Score) => Quantity(250),
        Asset::Reward(Reward::Token) => Quantity(100),
        Asset::Reward(Reward::Prediction) => Quantity(800),
        Asset::Reward(Reward::Policy) => Quantity(400),
    ]);
    let rhs = Account(hashmap![
        Asset::Reward(Reward::Score) => Quantity(250),
        Asset::Reward(Reward::Token) => Quantity(200),
        Asset::Reward(Reward::Policy) => Quantity(200),
        Asset::Reward(Reward::Prediction) => Quantity(700),
    ]);
    let res = Account(hashmap![
        Asset::Reward(Reward::Prediction) => Quantity(1500),
        Asset::Reward(Reward::Score) => Quantity(500),
        Asset::Reward(Reward::Token) => Quantity(300),
        Asset::Reward(Reward::Policy) => Quantity(600),
    ]);
    assert_eq!(&lhs + &rhs, res);
}

#[test]
fn accounts_add_missing_assets() {
    let lhs = Account(hashmap![
        Asset::Reward(Reward::Score) => Quantity(500),
        Asset::Reward(Reward::Prediction) => Quantity(800),
        Asset::Reward(Reward::Policy) => Quantity(100),
    ]);
    let rhs = Account(hashmap![
        Asset::Reward(Reward::Score) => Quantity(250),
        Asset::Reward(Reward::Token) => Quantity(10000),
        Asset::Reward(Reward::Prediction) => Quantity(700),
    ]);
    let res = Account(hashmap![
        Asset::Reward(Reward::Score) => Quantity(750),
        Asset::Reward(Reward::Token) => Quantity(10000),
        Asset::Reward(Reward::Prediction) => Quantity(1500),
        Asset::Reward(Reward::Policy) => Quantity(100),
    ]);
    assert_eq!(&lhs + &rhs, res);
}

