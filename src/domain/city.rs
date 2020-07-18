use rayon::prelude::*;
use serde::Deserialize;

const POINTS_TO_DISTANCE: usize = 2;
pub trait Distance {
    fn distance(&self) -> f64;
}

#[derive(Clone, PartialEq, Deserialize, PartialOrd, Debug)]
#[allow(non_snake_case)]
pub struct City {
    pub City: String,
    pub State: String,
    Latitude: f64,
    Longitude: f64,
}

impl City {
    fn octagonal_distance(&self, b: &City) -> f64 {
        let dx = (b.Latitude - self.Latitude).abs();
        let dy = (b.Longitude - self.Longitude).abs();

        0.4 * (dx + dy) + 0.55 * (dx.max(dy))
    }
}

impl Distance for Vec<City> {
    fn distance(&self) -> f64 {
        self.par_windows(POINTS_TO_DISTANCE)
            .map(|c| c[0].octagonal_distance(&c[1]))
            .sum()
    }
}
