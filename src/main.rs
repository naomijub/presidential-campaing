mod domain;
mod io;
use domain::gene::{Evolution, Gene};
use io::read_csv;
use std::string::ToString;

const POPULATION_SIZE: usize = 60; // For test.csv 10
const MINIMUM_FITNESS: f64 = 21f64;

fn main() {
    let cities = read_csv();
    let mut best: Gene;

    'outer: loop {
        let mut counter = 0usize;
        let mut best_counter = 0usize;
        best = Gene::default();

        let mut population = (0..POPULATION_SIZE)
            .map(|_| Gene::new(&cities))
            .collect::<Vec<Gene>>();

        'inner: loop {
            population = population.mutate().crossing_over().recalculate_fitness();

            let local_best = population.get_best();

            if local_best.fitness > best.fitness {
                best = local_best;
                best_counter = counter;
                println!(
                    "{}: {:?}, {:?}",
                    counter,
                    best.fitness,
                    domain::gene::DISTANCE_REWARD / best.distance
                );
            }
            counter += 1usize;

            if counter > best_counter + 600 {
                break 'inner;
            }
            if best.fitness > MINIMUM_FITNESS {
                break 'outer;
            }
        }
        println!("LOCAL_BEST {}", best.to_string());
    }

    println!("{}", best.to_string());
}
