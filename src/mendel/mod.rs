mod laws;
mod ncr;
mod npr;
mod permute;
mod prob;

pub(super) use laws::{execute_iprb, expected_offspring, second_law};
pub(super) use npr::npr;
pub(super) use permute::{factorial, permutation_list, permute};
pub(super) use prob::dna_prob;
