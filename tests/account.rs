use multiagent::{Account, AccountError, Basket, Quantity};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Asset {
    Credits,
    Tokens,
}

fn basket<const N: usize>(entries: [(Asset, u64); N]) -> Basket<Asset> {
    entries
        .map(|(asset, quantity)| (asset, Quantity::new(quantity)))
        .into()
}

#[test]
fn missing_assets_have_zero_balance() {
    let account = Account::new(basket([(Asset::Credits, 10)]));

    assert_eq!(account.balance(&Asset::Tokens), Quantity::ZERO);
}

#[test]
fn zero_quantities_are_not_stored() {
    let mut balances = basket([(Asset::Credits, 10)]);

    balances.insert(Asset::Credits, Quantity::ZERO);

    assert!(balances.is_empty());
}

#[test]
fn withdrawal_reports_exact_shortfall_and_is_atomic() {
    let mut account = Account::new(basket([(Asset::Credits, 5), (Asset::Tokens, 2)]));
    let original = account.clone();

    let error = account
        .withdraw(&basket([(Asset::Credits, 8), (Asset::Tokens, 2)]))
        .unwrap_err();

    assert_eq!(
        error,
        AccountError::InsufficientBalance {
            shortfall: basket([(Asset::Credits, 3)]),
        }
    );
    assert_eq!(account, original);
}

#[test]
fn deposit_overflow_is_atomic() {
    let mut account = Account::new(basket([(Asset::Credits, 10), (Asset::Tokens, u64::MAX)]));
    let original = account.clone();

    let error = account
        .deposit(&basket([(Asset::Credits, 5), (Asset::Tokens, 1)]))
        .unwrap_err();

    assert_eq!(
        error,
        AccountError::Overflow {
            asset: Asset::Tokens,
        }
    );
    assert_eq!(account, original);
}

#[test]
fn baskets_scale_with_checked_arithmetic() {
    let balances = basket([(Asset::Credits, 4), (Asset::Tokens, 3)]);

    let scaled = balances.checked_scale(Quantity::new(2)).unwrap();

    assert_eq!(scaled, basket([(Asset::Credits, 8), (Asset::Tokens, 6)]));
    assert_eq!(
        basket([(Asset::Credits, u64::MAX)])
            .checked_scale(Quantity::new(2))
            .unwrap_err(),
        Asset::Credits
    );
}
