#[cfg(test)]
mod tests;

mod algorithms;

mod proof_algorithms;

mod performance_algorithms;

pub use algorithms::{my_merge_sort, my_merge_sort_optmized};

pub use performance_algorithms::{my_merge_sort_it, my_merge_sort_optmized_it};


