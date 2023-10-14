pub use self::roulette_wheel::*;

mod roulette_wheel;

use rand::RngCore;
use crate::individual::Individual;

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
        where
            I: Individual;
}