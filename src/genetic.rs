use std::cmp::Ordering;
use rand::{RngCore};
use crate::chromosome::Chromosome;
use crate::crossover::CrossoverMethod;
use crate::individual::Individual;
use crate::mutation::MutationMethod;
use crate::selection::SelectionMethod;
use crate::statistics::Statistics;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S>
    where
        S: SelectionMethod,
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
        }
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> (Vec<I>, I, Statistics)
        where
            I: Individual + Clone,
    {
        assert!(!population.is_empty());

        let new_population = (0..population.len()/2)
            .flat_map(|_| {
                let parent_a = self.selection_method.select(rng, &population).chromosome();
                let parent_b = self.selection_method.select(rng, &population).chromosome();

                let (child1, child2) = self.crossover_method.crossover(rng, parent_a, parent_b);
                let mut child1 = child1;
                let mut child2 = child2;

                self.mutation_method.mutate(rng, &mut child1);
                self.mutation_method.mutate(rng, &mut child2);

                vec![I::create(child1), I::create(child2)]
            })
            .collect();

        let best_individual = population.iter()
            .max_by(|a, b| a.fitness().partial_cmp(&b.fitness()).unwrap_or(Ordering::Equal))
            .map(|individual| individual.clone())
            .unwrap();


        (new_population, best_individual, Statistics::new(population))
    }
}