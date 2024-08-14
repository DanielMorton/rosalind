mod laws;
mod ncr;
mod npr;
mod permute;
mod prob;

pub(super) use laws::{expected_offspring, first_law, second_law};
pub(super) use npr::npr;
pub(super) use permute::{factorial, permute};
pub(super) use prob::dna_prob;
