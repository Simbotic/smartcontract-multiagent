use crate::{Account, AccountError, Basket, Quantity, Rate};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct Ledger<AccountId, A> {
    accounts: HashMap<AccountId, Account<A>>,
}

impl<AccountId, A> Ledger<AccountId, A> {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.accounts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.accounts.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&AccountId, &Account<A>)> {
        self.accounts.iter()
    }
}

impl<AccountId, A> Ledger<AccountId, A>
where
    AccountId: Eq + Hash,
{
    pub fn insert(&mut self, account_id: AccountId, account: Account<A>) -> Option<Account<A>> {
        self.accounts.insert(account_id, account)
    }

    pub fn account(&self, account_id: &AccountId) -> Option<&Account<A>> {
        self.accounts.get(account_id)
    }

    pub fn account_mut(&mut self, account_id: &AccountId) -> Option<&mut Account<A>> {
        self.accounts.get_mut(account_id)
    }
}

impl<AccountId, A> Ledger<AccountId, A>
where
    AccountId: Clone + Eq + Hash,
    A: Clone + Eq + Hash,
{
    pub fn can_exchange(
        &self,
        buyer: &AccountId,
        seller: &AccountId,
        rate: &Rate<A>,
        units: Quantity,
    ) -> Result<(), ExchangeError<AccountId, A>> {
        self.prepare_exchange(buyer, seller, rate, units)
            .map(|_| ())
    }

    pub fn exchange(
        &mut self,
        buyer: &AccountId,
        seller: &AccountId,
        rate: &Rate<A>,
        units: Quantity,
    ) -> Result<Receipt<AccountId, A>, ExchangeError<AccountId, A>> {
        let prepared = self.prepare_exchange(buyer, seller, rate, units)?;

        self.accounts.insert(buyer.clone(), prepared.buyer_account);
        self.accounts
            .insert(seller.clone(), prepared.seller_account);

        Ok(Receipt {
            buyer: buyer.clone(),
            seller: seller.clone(),
            units,
            credit: prepared.credit,
            debit: prepared.debit,
        })
    }

    fn prepare_exchange(
        &self,
        buyer: &AccountId,
        seller: &AccountId,
        rate: &Rate<A>,
        units: Quantity,
    ) -> Result<PreparedExchange<A>, ExchangeError<AccountId, A>> {
        if units.is_zero() {
            return Err(ExchangeError::ZeroUnits);
        }
        if buyer == seller {
            return Err(ExchangeError::SameAccount {
                account: buyer.clone(),
            });
        }

        let mut buyer_account =
            self.accounts
                .get(buyer)
                .cloned()
                .ok_or_else(|| ExchangeError::MissingAccount {
                    account: buyer.clone(),
                })?;
        let mut seller_account =
            self.accounts
                .get(seller)
                .cloned()
                .ok_or_else(|| ExchangeError::MissingAccount {
                    account: seller.clone(),
                })?;

        let credit = rate
            .credit()
            .checked_scale(units)
            .map_err(|asset| ExchangeError::RateOverflow { asset })?;
        let debit = rate
            .debit()
            .checked_scale(units)
            .map_err(|asset| ExchangeError::RateOverflow { asset })?;

        buyer_account
            .withdraw(&debit)
            .map_err(|error| map_account_error(buyer.clone(), error))?;
        seller_account
            .withdraw(&credit)
            .map_err(|error| map_account_error(seller.clone(), error))?;
        buyer_account
            .deposit(&credit)
            .map_err(|error| map_account_error(buyer.clone(), error))?;
        seller_account
            .deposit(&debit)
            .map_err(|error| map_account_error(seller.clone(), error))?;

        Ok(PreparedExchange {
            buyer_account,
            seller_account,
            credit,
            debit,
        })
    }
}

impl<AccountId, A> Default for Ledger<AccountId, A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<AccountId, A> PartialEq for Ledger<AccountId, A>
where
    AccountId: Eq + Hash,
    A: Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.accounts == other.accounts
    }
}

impl<AccountId, A> Eq for Ledger<AccountId, A>
where
    AccountId: Eq + Hash,
    A: Eq + Hash,
{
}

struct PreparedExchange<A> {
    buyer_account: Account<A>,
    seller_account: Account<A>,
    credit: Basket<A>,
    debit: Basket<A>,
}

#[derive(Debug, Clone)]
pub struct Receipt<AccountId, A> {
    buyer: AccountId,
    seller: AccountId,
    units: Quantity,
    credit: Basket<A>,
    debit: Basket<A>,
}

impl<AccountId, A> Receipt<AccountId, A> {
    pub fn buyer(&self) -> &AccountId {
        &self.buyer
    }

    pub fn seller(&self) -> &AccountId {
        &self.seller
    }

    pub fn units(&self) -> Quantity {
        self.units
    }

    pub fn credit(&self) -> &Basket<A> {
        &self.credit
    }

    pub fn debit(&self) -> &Basket<A> {
        &self.debit
    }
}

impl<AccountId, A> PartialEq for Receipt<AccountId, A>
where
    AccountId: PartialEq,
    A: Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.buyer == other.buyer
            && self.seller == other.seller
            && self.units == other.units
            && self.credit == other.credit
            && self.debit == other.debit
    }
}

impl<AccountId, A> Eq for Receipt<AccountId, A>
where
    AccountId: Eq,
    A: Eq + Hash,
{
}

#[derive(Debug, Clone)]
pub enum ExchangeError<AccountId, A> {
    MissingAccount {
        account: AccountId,
    },
    SameAccount {
        account: AccountId,
    },
    ZeroUnits,
    RateOverflow {
        asset: A,
    },
    InsufficientBalance {
        account: AccountId,
        shortfall: Basket<A>,
    },
    BalanceOverflow {
        account: AccountId,
        asset: A,
    },
}

impl<AccountId, A> fmt::Display for ExchangeError<AccountId, A> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingAccount { .. } => formatter.write_str("account does not exist"),
            Self::SameAccount { .. } => {
                formatter.write_str("buyer and seller must be different accounts")
            }
            Self::ZeroUnits => formatter.write_str("exchange units must be greater than zero"),
            Self::RateOverflow { .. } => formatter.write_str("scaled rate overflow"),
            Self::InsufficientBalance { .. } => formatter.write_str("insufficient balance"),
            Self::BalanceOverflow { .. } => formatter.write_str("account balance overflow"),
        }
    }
}

impl<AccountId, A> Error for ExchangeError<AccountId, A>
where
    AccountId: fmt::Debug,
    A: fmt::Debug,
{
}

impl<AccountId, A> PartialEq for ExchangeError<AccountId, A>
where
    AccountId: PartialEq,
    A: Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::MissingAccount { account: left }, Self::MissingAccount { account: right })
            | (Self::SameAccount { account: left }, Self::SameAccount { account: right }) => {
                left == right
            }
            (Self::ZeroUnits, Self::ZeroUnits) => true,
            (Self::RateOverflow { asset: left }, Self::RateOverflow { asset: right }) => {
                left == right
            }
            (
                Self::InsufficientBalance {
                    account: left_account,
                    shortfall: left_shortfall,
                },
                Self::InsufficientBalance {
                    account: right_account,
                    shortfall: right_shortfall,
                },
            ) => left_account == right_account && left_shortfall == right_shortfall,
            (
                Self::BalanceOverflow {
                    account: left_account,
                    asset: left_asset,
                },
                Self::BalanceOverflow {
                    account: right_account,
                    asset: right_asset,
                },
            ) => left_account == right_account && left_asset == right_asset,
            _ => false,
        }
    }
}

impl<AccountId, A> Eq for ExchangeError<AccountId, A>
where
    AccountId: Eq,
    A: Eq + Hash,
{
}

fn map_account_error<AccountId, A>(
    account: AccountId,
    error: AccountError<A>,
) -> ExchangeError<AccountId, A> {
    match error {
        AccountError::InsufficientBalance { shortfall } => {
            ExchangeError::InsufficientBalance { account, shortfall }
        }
        AccountError::Overflow { asset } => ExchangeError::BalanceOverflow { account, asset },
    }
}
