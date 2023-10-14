use crate::chromosome::Chromosome;

pub trait Individual {
    fn create(chromosome: Chromosome) -> Self;
    fn chromosome(&self) -> &Chromosome;
    fn fitness(&self) -> f32;
}