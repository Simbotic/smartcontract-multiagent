use asset::*;
use account::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Rate {
    pub credit: HashMap<Asset, Quantity>,
    pub debit: HashMap<Asset, Quantity>,
}

impl Rate {
}
