use multiagent::{Account, Basket, ExchangeError, Ledger, Quantity, Rate};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Asset {
    Coin,
    Fuel,
    Time,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum AccountId {
    Buyer,
    Seller,
    Missing,
}

fn basket<const N: usize>(entries: [(Asset, u64); N]) -> Basket<Asset> {
    entries
        .map(|(asset, quantity)| (asset, Quantity::new(quantity)))
        .into()
}

fn ledger(buyer: Basket<Asset>, seller: Basket<Asset>) -> Ledger<AccountId, Asset> {
    let mut ledger = Ledger::new();
    ledger.insert(AccountId::Buyer, Account::new(buyer));
    ledger.insert(AccountId::Seller, Account::new(seller));
    ledger
}

fn total(ledger: &Ledger<AccountId, Asset>, asset: Asset) -> u128 {
    ledger
        .iter()
        .map(|(_, account)| u128::from(account.balance(&asset).get()))
        .sum()
}

#[test]
fn exchange_updates_both_accounts_and_returns_scaled_receipt() {
    let mut ledger = ledger(
        basket([(Asset::Coin, 10), (Asset::Fuel, 7)]),
        basket([(Asset::Time, 10)]),
    );
    let rate = Rate::new(basket([(Asset::Time, 2)]), basket([(Asset::Coin, 3)]));
    let totals_before = [
        total(&ledger, Asset::Coin),
        total(&ledger, Asset::Fuel),
        total(&ledger, Asset::Time),
    ];

    let receipt = ledger
        .exchange(
            &AccountId::Buyer,
            &AccountId::Seller,
            &rate,
            Quantity::new(2),
        )
        .unwrap();

    let buyer = ledger.account(&AccountId::Buyer).unwrap();
    assert_eq!(buyer.balance(&Asset::Coin), Quantity::new(4));
    assert_eq!(buyer.balance(&Asset::Fuel), Quantity::new(7));
    assert_eq!(buyer.balance(&Asset::Time), Quantity::new(4));

    let seller = ledger.account(&AccountId::Seller).unwrap();
    assert_eq!(seller.balance(&Asset::Coin), Quantity::new(6));
    assert_eq!(seller.balance(&Asset::Time), Quantity::new(6));

    assert_eq!(receipt.buyer(), &AccountId::Buyer);
    assert_eq!(receipt.seller(), &AccountId::Seller);
    assert_eq!(receipt.units(), Quantity::new(2));
    assert_eq!(receipt.credit(), &basket([(Asset::Time, 4)]));
    assert_eq!(receipt.debit(), &basket([(Asset::Coin, 6)]));
    assert_eq!(
        totals_before,
        [
            total(&ledger, Asset::Coin),
            total(&ledger, Asset::Fuel),
            total(&ledger, Asset::Time),
        ]
    );
}

#[test]
fn buyer_shortfall_rejects_the_whole_exchange() {
    let mut ledger = ledger(basket([(Asset::Coin, 5)]), basket([(Asset::Time, 10)]));
    let original = ledger.clone();
    let rate = Rate::new(basket([(Asset::Time, 1)]), basket([(Asset::Coin, 6)]));

    let error = ledger
        .exchange(
            &AccountId::Buyer,
            &AccountId::Seller,
            &rate,
            Quantity::new(1),
        )
        .unwrap_err();

    assert_eq!(
        error,
        ExchangeError::InsufficientBalance {
            account: AccountId::Buyer,
            shortfall: basket([(Asset::Coin, 1)]),
        }
    );
    assert_eq!(ledger, original);
}

#[test]
fn seller_must_own_the_credited_assets() {
    let mut ledger = ledger(basket([(Asset::Coin, 10)]), basket([(Asset::Time, 1)]));
    let original = ledger.clone();
    let rate = Rate::new(basket([(Asset::Time, 2)]), basket([(Asset::Coin, 3)]));

    let error = ledger
        .exchange(
            &AccountId::Buyer,
            &AccountId::Seller,
            &rate,
            Quantity::new(1),
        )
        .unwrap_err();

    assert_eq!(
        error,
        ExchangeError::InsufficientBalance {
            account: AccountId::Seller,
            shortfall: basket([(Asset::Time, 1)]),
        }
    );
    assert_eq!(ledger, original);
}

#[test]
fn arithmetic_errors_leave_the_ledger_unchanged() {
    let mut rate_overflow_ledger = ledger(Basket::new(), basket([(Asset::Time, u64::MAX)]));
    let original = rate_overflow_ledger.clone();
    let overflowing_rate = Rate::new(basket([(Asset::Time, u64::MAX)]), Basket::new());

    assert_eq!(
        rate_overflow_ledger
            .exchange(
                &AccountId::Buyer,
                &AccountId::Seller,
                &overflowing_rate,
                Quantity::new(2),
            )
            .unwrap_err(),
        ExchangeError::RateOverflow { asset: Asset::Time }
    );
    assert_eq!(rate_overflow_ledger, original);

    let mut balance_overflow_ledger = ledger(
        basket([(Asset::Time, u64::MAX)]),
        basket([(Asset::Time, 1)]),
    );
    let original = balance_overflow_ledger.clone();
    let overflowing_balance_rate = Rate::new(basket([(Asset::Time, 1)]), Basket::new());

    assert_eq!(
        balance_overflow_ledger
            .exchange(
                &AccountId::Buyer,
                &AccountId::Seller,
                &overflowing_balance_rate,
                Quantity::new(1),
            )
            .unwrap_err(),
        ExchangeError::BalanceOverflow {
            account: AccountId::Buyer,
            asset: Asset::Time,
        }
    );
    assert_eq!(balance_overflow_ledger, original);
}

#[test]
fn invalid_account_requests_are_structured_errors() {
    let mut ledger = ledger(Basket::new(), Basket::new());
    let rate = Rate::new(Basket::new(), Basket::new());

    assert_eq!(
        ledger
            .exchange(
                &AccountId::Buyer,
                &AccountId::Buyer,
                &rate,
                Quantity::new(1),
            )
            .unwrap_err(),
        ExchangeError::SameAccount {
            account: AccountId::Buyer,
        }
    );
    assert_eq!(
        ledger
            .exchange(
                &AccountId::Missing,
                &AccountId::Seller,
                &rate,
                Quantity::new(1),
            )
            .unwrap_err(),
        ExchangeError::MissingAccount {
            account: AccountId::Missing,
        }
    );
    assert_eq!(
        ledger
            .exchange(&AccountId::Buyer, &AccountId::Seller, &rate, Quantity::ZERO,)
            .unwrap_err(),
        ExchangeError::ZeroUnits
    );
}

#[test]
fn exchange_feasibility_check_does_not_mutate_state() {
    let ledger = ledger(basket([(Asset::Coin, 5)]), basket([(Asset::Time, 5)]));
    let original = ledger.clone();
    let rate = Rate::new(basket([(Asset::Time, 1)]), basket([(Asset::Coin, 1)]));

    ledger
        .can_exchange(
            &AccountId::Buyer,
            &AccountId::Seller,
            &rate,
            Quantity::new(2),
        )
        .unwrap();

    assert_eq!(ledger, original);
}

#[test]
fn users_can_supply_non_enum_asset_and_account_types() {
    let mut ledger = Ledger::<u32, String>::new();
    ledger.insert(
        1,
        Account::new(
            [(String::from("coin"), Quantity::new(3))]
                .into_iter()
                .collect(),
        ),
    );
    ledger.insert(
        2,
        Account::new(
            [(String::from("time"), Quantity::new(1))]
                .into_iter()
                .collect(),
        ),
    );
    let rate = Rate::new(
        [(String::from("time"), Quantity::new(1))]
            .into_iter()
            .collect(),
        [(String::from("coin"), Quantity::new(2))]
            .into_iter()
            .collect(),
    );

    ledger.exchange(&1, &2, &rate, Quantity::new(1)).unwrap();

    assert_eq!(
        ledger.account(&1).unwrap().balance(&String::from("time")),
        Quantity::new(1)
    );
}
