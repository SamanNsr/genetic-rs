use rand::{Rng, RngCore};
use rand::seq::SliceRandom;
use crate::individual::*;
use crate::selection::*;

pub struct RouletteWheelSelection;

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
        where
            I: Individual,
    {
        assert!(!population.is_empty());

        let total_fitness: f32 = population
            .iter()
            .map(|individual| individual.fitness())
            .sum();

        // This is a naïve approach for demonstration purposes; a more
        // efficient implementation could invoke `rng` just once
        loop {
            let indiv = population
                .choose(rng)
                .expect("got an empty population");

            let indiv_share = indiv.fitness() / total_fitness;

            if rng.gen_bool(indiv_share as f64) {
                return indiv;
            }
        }
    }
}