use crate::Quantity;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;

#[derive(Debug, Clone)]
pub struct Basket<A> {
    quantities: HashMap<A, Quantity>,
}

impl<A> Basket<A> {
    pub fn new() -> Self {
        Self {
            quantities: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.quantities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.quantities.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&A, Quantity)> {
        self.quantities
            .iter()
            .map(|(asset, quantity)| (asset, *quantity))
    }
}

impl<A> Basket<A>
where
    A: Eq + Hash,
{
    pub fn quantity(&self, asset: &A) -> Quantity {
        self.quantities
            .get(asset)
            .copied()
            .unwrap_or(Quantity::ZERO)
    }

    pub fn insert(&mut self, asset: A, quantity: Quantity) -> Option<Quantity> {
        if quantity.is_zero() {
            self.quantities.remove(&asset)
        } else {
            self.quantities.insert(asset, quantity)
        }
    }
}

impl<A> Basket<A>
where
    A: Clone + Eq + Hash,
{
    pub fn checked_scale(&self, units: Quantity) -> Result<Self, A> {
        let mut scaled = Self::new();

        for (asset, quantity) in self.iter() {
            let Some(quantity) = quantity.checked_mul(units) else {
                return Err(asset.clone());
            };
            scaled.insert(asset.clone(), quantity);
        }

        Ok(scaled)
    }

    pub fn shortfall(&self, required: &Self) -> Self {
        let mut shortfall = Self::new();

        for (asset, required_quantity) in required.iter() {
            let available = self.quantity(asset);
            if available < required_quantity {
                let missing = required_quantity
                    .checked_sub(available)
                    .expect("available quantity is smaller than required quantity");
                shortfall.insert(asset.clone(), missing);
            }
        }

        shortfall
    }
}

impl<A> Default for Basket<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A> PartialEq for Basket<A>
where
    A: Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.quantities == other.quantities
    }
}

impl<A> Eq for Basket<A> where A: Eq + Hash {}

impl<A> FromIterator<(A, Quantity)> for Basket<A>
where
    A: Eq + Hash,
{
    fn from_iter<T>(entries: T) -> Self
    where
        T: IntoIterator<Item = (A, Quantity)>,
    {
        let mut basket = Self::new();
        for (asset, quantity) in entries {
            basket.insert(asset, quantity);
        }
        basket
    }
}

impl<A, const N: usize> From<[(A, Quantity); N]> for Basket<A>
where
    A: Eq + Hash,
{
    fn from(entries: [(A, Quantity); N]) -> Self {
        entries.into_iter().collect()
    }
}
