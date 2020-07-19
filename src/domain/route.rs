use super::city::{City, Distance};
use crate::domain::city::Unique;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use rayon::prelude::*;

pub const DISTANCE_REWARD: f64 = 20000f64; // For test.csv 10000f64
const SF_REWARD: f64 = 5f64;
const SF_PUNISHMENT: f64 = -2.5f64; // For test.csv -15f64
const UNIQUE_REWARD: f64 = 7f64; // For test.csv 10f64
const UNIQUE_PUNISHMENT: f64 = -15f64; // For test.csv -20f64
const CROSSING_OVER_CANDIDATES: usize = 15; // For test.csv 5
const FITNESS_TO_MUTATE: f64 = -3f64; // For test.csv it can be aroung -10f64
const NEXT_GEN_PROBABILITY: f64 = 0.3f64; // Not used for test.csv

pub trait MyOrd {
    fn cmp(&self, other: &f64) -> Ordering;
}

pub trait Evolution {
    fn mutate(self) -> Self;
    fn recalculate_fitness(self) -> Self;
    fn crossing_over(self) -> Self;
    fn get_best(&self) -> Route;
    fn choose_parent(&self) -> Route;
}

#[derive(PartialEq, Clone, PartialOrd, Debug)]
pub struct Route {
    pub cities: Vec<City>,
    pub distance: f64,
    pub fitness: f64,
}

impl Route {
    pub fn new(cities: &Vec<City>) -> Self {
        let mut rng = thread_rng();
        let cities = cities
            .choose_multiple(&mut rng, cities.len())
            .cloned()
            .collect::<Vec<City>>();
        let distance = cities.distance();
        Self {
            cities,
            distance,
            fitness: (DISTANCE_REWARD / distance),
        }
    }

    fn distance(&self) -> f64 {
        self.cities.distance()
    }
}

impl Default for Route {
    fn default() -> Self {
        Self {
            cities: Vec::new(),
            distance: f64::MAX,
            fitness: f64::MIN,
        }
    }
}

impl Evolution for Vec<Route> {
    fn mutate(mut self) -> Self {
        let mut rng = thread_rng();
        let route_size = self[0].cities.len();
        let mut rand_value = || rng.gen_range(0, route_size);

        self.iter_mut()
            .filter(|g| g.fitness > FITNESS_TO_MUTATE)
            .for_each(|route| {
                route.cities.swap(rand_value(), rand_value());
                route.cities.swap(rand_value(), rand_value());
                route.cities.swap(rand_value(), rand_value());
            });
        self
    }

    fn recalculate_fitness(mut self) -> Self {
        self.iter_mut().for_each(|route| {
            let sf_first_prize = match route.cities.first() {
                Some(sf) if &sf.City == "San Francisco" => SF_REWARD,
                _ => SF_PUNISHMENT,
            };

            let sf_last_prize = match route.cities.last() {
                Some(sf) if &sf.City == "San Francisco" => SF_REWARD,
                _ => SF_PUNISHMENT,
            };

            let unique_cities = route.cities.clone().uniqueness_count();
            let all_cities = route.cities.len();
            let uniqueness_prize = match (unique_cities, all_cities) {
                (u, a) if u + 1 == a => UNIQUE_REWARD,
                (u, a) if u + 5 == a => UNIQUE_REWARD / 5f64,
                _ => UNIQUE_PUNISHMENT,
            };

            let new_distance = route.distance();
            route.distance = new_distance;
            route.fitness = match new_distance < 100f64 {
                false => calculate_fitness(
                    new_distance,
                    sf_first_prize,
                    sf_last_prize,
                    uniqueness_prize,
                ),
                true => f64::MIN,
            };
        });
        self
    }

    fn crossing_over(self) -> Self {
        let mut rng = thread_rng();
        let route_size = self[0].cities.len();
        let middle_route = route_size / 2;
        let mut new_population = Vec::new();
        let best = self.get_best();

        new_population.push(best);
        let aux_pop = (1..self.len()).fold(Vec::new(), |new_pop, _| {
            let p1 = self.choose_parent();

            if rng.gen_range(0.0, 1.0) > NEXT_GEN_PROBABILITY {
                let p2 = self.choose_parent();

                [
                    new_pop,
                    vec![Route {
                        cities: [
                            p1.cities[0..middle_route].to_vec(),
                            p2.cities[middle_route..route_size].to_vec(),
                        ]
                        .concat(),
                        fitness: 0f64,
                        distance: 0f64,
                    }],
                ]
                .concat()
            } else {
                [new_pop, vec![p1]].concat()
            }
        });
        [new_population, aux_pop].concat()
    }

    fn choose_parent(&self) -> Route {
        let mut rng = thread_rng();

        self.choose_multiple(&mut rng, CROSSING_OVER_CANDIDATES)
            .map(|g| g.clone())
            .max_by(|a, b| a.fitness.cmp(&b.fitness))
            .unwrap()
    }

    fn get_best(&self) -> Route {
        self.par_iter()
            .max_by(|a, b| a.fitness.cmp(&b.fitness))
            .unwrap()
            .clone()
    }
}

fn calculate_fitness(
    new_distance: f64,
    sf_first_prize: f64,
    sf_last_prize: f64,
    uniqueness_prize: f64,
) -> f64 {
    (DISTANCE_REWARD / new_distance) + sf_first_prize + sf_last_prize + uniqueness_prize
}

impl std::string::ToString for Route {
    fn to_string(&self) -> String {
        format!(
            "Cities: {}\nDistance: {}",
            self.cities
                .iter()
                .map(|c| format!("{},{}", c.City, c.State))
                .collect::<Vec<String>>()
                .join("\n"),
            self.distance
        )
    }
}

use std::cmp::Ordering;
impl MyOrd for f64 {
    fn cmp(&self, other: &f64) -> Ordering {
        if *self < *other {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}
