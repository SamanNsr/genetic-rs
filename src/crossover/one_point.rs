use rand::{Rng, RngCore};
use crate::chromosome::Chromosome;
use crate::crossover::CrossoverMethod;

pub struct OnePointCrossover{
    chance: f32,
}

impl OnePointCrossover {
    pub fn new(chance: f32) -> Self {
        assert!((0.0..=1.0).contains(&chance));

        Self { chance }
    }
}

impl CrossoverMethod for OnePointCrossover {
    fn crossover(
        &self,
        rng: &mut dyn RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> (Chromosome, Chromosome) {
        assert_eq!(parent_a.len(), parent_b.len());

        let random_number = rng.gen::<f32>();

        if rng.gen_bool(self.chance as _) {
            let crossover_point = rng.gen_range(0..parent_a.len());

            let child1 = parent_a.iter().take(crossover_point)
                .chain(parent_b.iter().skip(crossover_point))
                .cloned()
                .collect();

            let child2 = parent_b.iter().take(crossover_point)
                .chain(parent_a.iter().skip(crossover_point))
                .cloned()
                .collect();

            (child1, child2)
        } else {
            (parent_a.iter().cloned().collect(), parent_b.iter().cloned().collect())
        }

    }
}