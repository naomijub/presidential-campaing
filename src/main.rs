mod domain;
mod io;

use domain::gene::{Gene, Evolution};
use io::read_csv;

const POPULATION_SIZE: usize = 30; // For test.csv 10
const MINIMUM_FITNESS: f64 = 45f64;

fn main() {
    let cities = read_csv();

    let mut best = Gene::default();

    let mut population = (0..POPULATION_SIZE)
        .map(|_| 
            Gene::new(&cities)
        )
        .collect::<Vec<Gene>>();

    loop {
        population = population
            .mutate()
            .crossing_over()
            .recalculate_fitness();

        let local_best = population.get_best();

        if local_best.fitness > best.fitness {
            best = local_best;
            println!("{:?}, {:?}", best.fitness, domain::gene::DISTANCE_REWARD / best.distance);
        }

        if best.fitness > MINIMUM_FITNESS { break; }
    }

    println!("{:#?}", best);
    println!("{:?}", best.cities.clone().drain(0..).map(|c| format!("{:?}-{:?}", c.City, c.State)).collect::<std::collections::HashSet<String>>().len());
}
