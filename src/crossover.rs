pub use self::one_point::*;
mod one_point;

use rand::RngCore;
use crate::chromosome::Chromosome;


pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> (Chromosome, Chromosome);
}