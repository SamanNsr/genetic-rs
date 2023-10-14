use rand::{Rng, RngCore};
use crate::chromosome::Chromosome;
use crate::mutation::MutationMethod;

pub struct BitFlipMutation {
    chance: f32,
}

impl BitFlipMutation {
    pub fn new(chance: f32) -> Self {
        assert!((0.0..=1.0).contains(&chance));

        Self { chance }
    }
}

impl MutationMethod for BitFlipMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        for gene in child.iter_mut() {
            if rng.gen_bool(self.chance as _) {
                // Flip the bit at the selected index.
                *gene = 1.0 - *gene;
            }
        }
    }
}
