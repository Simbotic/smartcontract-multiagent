use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct Rate<A> {
    credit: crate::Basket<A>,
    debit: crate::Basket<A>,
}

impl<A> Rate<A> {
    pub fn new(credit: crate::Basket<A>, debit: crate::Basket<A>) -> Self {
        Self { credit, debit }
    }

    pub fn credit(&self) -> &crate::Basket<A> {
        &self.credit
    }

    pub fn debit(&self) -> &crate::Basket<A> {
        &self.debit
    }
}

impl<A> PartialEq for Rate<A>
where
    A: Eq + Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.credit == other.credit && self.debit == other.debit
    }
}

impl<A> Eq for Rate<A> where A: Eq + Hash {}
