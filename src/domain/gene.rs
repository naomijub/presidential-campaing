use rand::seq::SliceRandom;
use rand::{Rng,thread_rng};
use std::collections::HashSet;
use super::city::{City, Distance};

pub  const DISTANCE_REWARD: f64 = 15000f64; // For test.csv 10000f64
const SF_REWARD: f64 = 10f64;
const SF_PUNISHMENT: f64 = -5f64; // For test.csv -15f64
const UNIQUE_REWARD: f64 = 20f64;
const UNIQUE_PUNISHMENT: f64 = -15f64; // For test.csv -20f64
const CROSSING_OVER_CANDIDATES: usize = 15; // For test.csv 5
const FITNESS_TO_MUTATE: f64 = -20f64;

pub trait MyOrd {
    fn cmp(&self, other: &f64) -> Ordering;
}

pub trait Evolution  {
    fn mutate(self) -> Self;
    fn recalculate_fitness(self) -> Self;
    fn crossing_over(self) -> Self;
    fn get_best(&self) -> Gene;
}

#[derive(PartialEq, Clone, PartialOrd, Debug)]
pub struct Gene {
    pub cities: Vec<City>,
    pub distance: f64,
    pub fitness: f64,
}

impl Gene {
    pub fn new(cities: &Vec<City>) -> Self {
        let mut rng = thread_rng();
        let cities = cities.choose_multiple(&mut rng, cities.len()).cloned().collect::<Vec<City>>();
        let distance = cities.distance();
        Self {
            cities,
            distance,
            fitness: (DISTANCE_REWARD / distance)
        }
    }

    fn distance(&self) -> f64 {
        self.cities.distance()
    }

}

impl Default for Gene {
    fn default() -> Self {
        Self {
            cities: Vec::new(),
            distance: f64::MAX,
            fitness: f64::MIN,
        }
    }
}

impl Evolution for Vec<Gene> {
    fn mutate(mut self) -> Self {
        let mut rng = thread_rng();
        let gene_size = self[0].cities.len();
        let mut rand_value = || rng.gen_range(0, gene_size);

        self.iter_mut()
            .filter(|g| g.fitness > FITNESS_TO_MUTATE)
            .for_each(|gene| {
                gene.cities.swap(rand_value(), rand_value());
                gene.cities.swap(rand_value(), rand_value());
                gene.cities.swap(rand_value(), rand_value());
                gene.cities.swap(rand_value(), rand_value());
            }
        );
        self
    }
    
    fn recalculate_fitness(mut self) -> Self {
        self.iter_mut().for_each(|gene| {
            let sf_first_prize = match gene.cities.first() {
                Some(sf) if &sf.City == "San Francisco" => SF_REWARD,
                _ => SF_PUNISHMENT,
            };

            let sf_last_prize = match gene.cities.last() {
                Some(sf) if &sf.City == "San Francisco" => SF_REWARD,
                _ => SF_PUNISHMENT,
            };

            let unique_cities = gene.cities.clone().drain(0..).map(|c| format!("{:?}-{:?}", c.City, c.State)).collect::<HashSet<String>>().len();
            let all_cities = gene.cities.len();
            let uniqueness_prize = 
                match (unique_cities, all_cities) {
                    (u, a) if u + 1 == a  => UNIQUE_REWARD,
                    _ => UNIQUE_PUNISHMENT
                };

                

            let new_distance = gene.distance();
            gene.distance = new_distance;
            gene.fitness = match new_distance < 0.1f64 && new_distance > -0.1f64 {
                false => (DISTANCE_REWARD / new_distance) + sf_first_prize + sf_last_prize + uniqueness_prize,
                true => f64::MIN
            };
        });
        self 
    }
    
    fn crossing_over(self) -> Self {
        let mut rng = thread_rng();
        let gene_size = self[0].cities.len();
        let middle_gene = gene_size / 2;
        let mut new_population = Vec::new();
        let best = self.get_best();
        
        new_population.push(best);
        let aux_pop = (1..self.len())
            .fold(Vec::new(),|new_pop, _| {
                let p1 = self.choose_multiple(&mut rng, CROSSING_OVER_CANDIDATES).map(|g| g.clone()).max_by(|a,b| a.fitness.cmp(&b.fitness)).unwrap();
                let p2 = self.choose_multiple(&mut rng, CROSSING_OVER_CANDIDATES).map(|g| g.clone()).max_by(|a,b| a.fitness.cmp(&b.fitness)).unwrap();
    
                [new_pop, 
                 vec![Gene {
                    cities: [
                            p1.cities[0..middle_gene].to_vec(),  p2.cities[middle_gene..gene_size].to_vec()
                        ].concat(),
                    fitness: 0f64,
                    distance: 0f64
                 }]].concat()
            });
        [new_population, aux_pop].concat()
    }
    
    fn get_best(&self) -> Gene {
        self.iter().max_by(|a,b| a.fitness.cmp(&b.fitness)).unwrap().clone()
    }
}

use std::cmp::{Ordering};
impl MyOrd for f64 {
    fn cmp(&self, other: &f64) -> Ordering {
        if *self < *other { Ordering::Less }
        else { Ordering::Greater }
    }
}