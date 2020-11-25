use std::cmp::Ordering;
use std::cell::RefCell;


pub struct Counter<T: Ord> {
    pub val: T,
    pub n_comps: RefCell<usize>,
}

impl<T> Counter<T>
where
    T: Ord
{
    pub fn new(val: T) -> Self {
        Self {
            val,
            n_comps: RefCell::new(0),
        }
    }

    pub fn reset(&mut self) {
        *self.n_comps.borrow_mut() = 0;
    }

    pub fn get_comps(&self) -> usize {
        self.n_comps.borrow().clone()
    }
}

impl<T> Ord for Counter<T>
where
    T: Ord
{
    fn cmp(&self, other: &Self) -> Ordering {
        *self.n_comps.borrow_mut() += 1;
        *other.n_comps.borrow_mut() += 1;
        self.val.cmp(&other.val)
    }
}

impl<T> PartialOrd for Counter<T>
where
    T: Ord
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for Counter<T>
where
    T: Ord
{
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl<T: Ord> Eq for Counter<T> {}
