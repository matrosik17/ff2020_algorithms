mod selection_sort;
mod heap_sort;

pub use selection_sort::selection_sort;
pub use heap_sort::heap_sort;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

}
