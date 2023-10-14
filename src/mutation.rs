pub use self::bit_flip::*;
mod bit_flip;

use rand::RngCore;
use crate::chromosome::Chromosome;

pub trait MutationMethod {
    fn mutate(
        &self,
        rng: &mut dyn RngCore,
        child: &mut Chromosome
    );
}