use crate::{Basket, Quantity};
use std::error::Error;
use std::fmt;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct Account<A> {
    balances: Basket<A>,
}

impl<A> Account<A> {
    pub fn new(balances: Basket<A>) -> Self {
        Self { balances }
    }

    pub fn balances(&self) -> &Basket<A> {
        &self.balances
    }

    pub fn into_balances(self) -> Basket<A> {
        self.balances
    }
}

impl<A> Account<A>
where
    A: Eq + Hash,
{
    pub fn balance(&self, asset: &A) -> Quantity {
        self.balances.quantity(asset)
    }
}

impl<A> Account<A>
where
    A: Clone + Eq + Hash,
{
    pub fn deposit(&mut self, assets: &Basket<A>) -> Result<(), AccountError<A>> {
        let mut updated = self.balances.clone();

        for (asset, amount) in assets.iter() {
            let balance = updated.quantity(asset);
            let Some(balance) = balance.checked_add(amount) else {
                return Err(AccountError::Overflow {
                    asset: asset.clone(),
                });
            };
            updated.insert(asset.clone(), balance);
        }

        self.balances = updated;
        Ok(())
    }

    pub fn withdraw(&mut self, assets: &Basket<A>) -> Result<(), AccountError<A>> {
        let shortfall = self.balances.shortfall(assets);
        if !shortfall.is_empty() {
            return Err(AccountError::InsufficientBalance { shortfall });
        }

        let mut updated = self.balances.clone();
        for (asset, amount) in assets.iter() {
            let balance = updated
                .quantity(asset)
                .checked_sub(amount)
                .expect("shortfall was checked before withdrawal");
            updated.insert(asset.clone(), balance);
        }

        self.balances = updated;
        Ok(())
    }
}

impl<A> Default for Account<A> {
    fn default() -> Self {
        Self::new(Basket::new())
    }
}

impl<A> PartialEq for Account<A>
where
    A: Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.balances == other.balances
    }
}

impl<A> Eq for Account<A> where A: Eq + Hash {}

impl<A> From<Basket<A>> for Account<A> {
    fn from(balances: Basket<A>) -> Self {
        Self::new(balances)
    }
}

#[derive(Debug, Clone)]
pub enum AccountError<A> {
    InsufficientBalance { shortfall: Basket<A> },
    Overflow { asset: A },
}

impl<A> fmt::Display for AccountError<A> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InsufficientBalance { .. } => formatter.write_str("insufficient balance"),
            Self::Overflow { .. } => formatter.write_str("account balance overflow"),
        }
    }
}

impl<A> Error for AccountError<A> where A: fmt::Debug {}

impl<A> PartialEq for AccountError<A>
where
    A: Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::InsufficientBalance { shortfall: left },
                Self::InsufficientBalance { shortfall: right },
            ) => left == right,
            (Self::Overflow { asset: left }, Self::Overflow { asset: right }) => left == right,
            _ => false,
        }
    }
}

impl<A> Eq for AccountError<A> where A: Eq + Hash {}
