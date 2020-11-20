use std::cmp::Ordering;
use std::cell::RefCell;

pub struct Counter<T: Ord> {
    pub val: T,
    pub n_comps: RefCell<usize>,
    pub n_swaps: RefCell<usize>,
}

impl<T: Ord> Counter<T> {
    pub fn reset(&mut self) {
        *self.n_comps.borrow_mut() = 0;
        *self.n_swaps.borrow_mut() = 0;
    }
}

impl<T: Ord> Ord for Counter<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        *self.n_comps.borrow_mut() += 1;
        *other.n_comps.borrow_mut() += 1;
        self.val.cmp(&other.val)
    }
}

impl<T: Ord> PartialOrd for Counter<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord> PartialEq for Counter<T> {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl<T: Ord> Eq for Counter<T> {}