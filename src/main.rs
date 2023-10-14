use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::RngCore;
use crate::chromosome::Chromosome;
use crate::crossover::*;
use crate::genetic::*;
use crate::individual::Individual;
use crate::mutation::*;
use crate::selection::*;

mod genetic;
mod chromosome;
mod selection;
mod mutation;
mod individual;
mod crossover;
mod statistics;

fn main() {

    let mut rng = ChaCha8Rng::from_seed(Default::default());

    let ga = GeneticAlgorithm::new(
        RouletteWheelSelection,
        OnePointCrossover::new(0.25),
        BitFlipMutation::new(0.01),
    );

    let mut population = vec![
        OPIndividual::create(random_vector().iter().cloned().collect()),
        OPIndividual::create(random_vector().iter().cloned().collect()),
        OPIndividual::create(random_vector().iter().cloned().collect()),
        OPIndividual::create(random_vector().iter().cloned().collect()),
        OPIndividual::create(random_vector().iter().cloned().collect()),
        OPIndividual::create(random_vector().iter().cloned().collect()),
        OPIndividual::create(random_vector().iter().cloned().collect()),
        OPIndividual::create(random_vector().iter().cloned().collect()),
        OPIndividual::create(random_vector().iter().cloned().collect()),
        OPIndividual::create(random_vector().iter().cloned().collect()),
    ];
    let mut best_individual =  OPIndividual::create(random_vector().iter().cloned().collect());
    let mut best_index = 0;

    for i in 0..1500 {
        let (new_population, new_best_individual, stat) = ga.evolve(&mut rng, &population);

        if new_best_individual.fitness() > best_individual.fitness() {
            best_individual = new_best_individual;
            best_index = i + 1;
        }

        println!("max {}, min {}, average {}", stat.max_fitness(), stat.min_fitness(), stat.avg_fitness());


        population = new_population;
    }

    let (best_x1, best_x2) = best_individual.decode();

    println!("best: x1= {}, x2= {}, objective= {}, generation={}", best_x1, best_x2, best_individual.fitness(), best_index)
}

#[derive(Clone)]
enum OPIndividual {
    WithChromosome { chromosome: Chromosome },
    WithFitness { fitness: f32 },
}

impl OPIndividual {
    fn new(fitness: f32) -> Self {
        Self::WithFitness { fitness }
    }

/*    fn encode(&self) -> &Chromosome {

    }*/

    fn decode(&self) -> (f32, f32) {
        let first_part: Vec<f32> = self.chromosome().iter().cloned().take(18).collect();
        let second_part: Vec<f32> = self.chromosome().iter().cloned().skip(18).collect();


        let x1 = -3.0 + ( binary_vector_to_int(first_part) as f32 * ( (12.1 - (-3.0)) / (f32::powf(2.0,18.0) - 1.0) ));
        let x2 = 4.1 + ( binary_vector_to_int(second_part) as f32 * ( (5.8 - (4.1)) / (f32::powf(2.0,15.0) - 1.0) ));

        (x1, x2)
    }
}

impl Individual for OPIndividual {
    fn create(chromosome: Chromosome) -> Self {
        Self::WithChromosome { chromosome }
    }

    fn chromosome(&self) -> &Chromosome {
        match self {
            Self::WithChromosome { chromosome } => chromosome,
            Self::WithFitness { .. } => panic!("not supported for TestIndividual::WithFitness"),
        }
    }

    fn fitness(&self) -> f32 {
        match self {
            Self::WithChromosome { chromosome } => {
                let (x1, x2) = self.decode();
                21.5 + x1 * f32::sin(4.0 * 3.14 * x1) + x2 * f32::sin(20.0 * 3.14 * x2)
            },
            Self::WithFitness { fitness } => *fitness,
        }
    }
}

fn binary_vector_to_int(binary_vec: Vec<f32>) -> i32 {
    let mut result = 0;

    for &bit in &binary_vec {
        let bit_as_int = if bit >= 0.5 { 1 } else { 0 }; // Convert f32 to i32
        result = (result << 1) | bit_as_int;
    }
    result
}

fn random_vector() -> Vec<f32> {
    let mut rng = ChaCha8Rng::from_seed(Default::default());

    let mut random_vector: Vec<f32> = Vec::new();

    for _ in 0..33 {
        let random_value = if rng.gen::<bool>() { 1.0 } else { 0.0 };
        random_vector.push(random_value);
    }
    random_vector

}