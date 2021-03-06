# Genetic algorithm for travel salesperson with Rust

To avoid convergence problems the inner loop terminates when the last best counter is greater than the current counter plus 600. This took between 3 and 15 loops to find a solution, mostly between 3 and 7.

```rust
fn main() {
    let cities = read_csv();
    let mut best: Route;

    'outer: loop {
        let mut counter = 0usize;
        let mut best_counter = 0usize;
        best = Route::default();

        let mut population = (0..POPULATION_SIZE)
            .map(|_| Route::new(&cities))
            .collect::<Vec<Route>>();

        'inner: loop {
            population = population.crossing_over().recalculate_fitness().mutate().recalculate_fitness();

            let local_best = population.get_best();

            if local_best.fitness > best.fitness {
                best = local_best;
            }

            if counter > best_counter + 300 {
                break 'inner;
            }
            if best.fitness > MINIMUM_FITNESS {
                break 'outer;
            }
        }
        // Prints
    }

    // Prints
}
```

Whenever the `'inner` loop breaks, the `local_best` is show, that mean, the best `Route` for that loop. When the `'outer` loop breaks, the best `Route` is printed.
```rust
if counter > best_counter + 600 {
    break 'inner;
}
```

### The implementation
The basic key concept of this algorithm is the population that in this case is a `Vec<Route>` and implement the trait `Evolution` for `Vec<Route>`. The trait `Evolution` is responsable for the main caculation in this code base which is `population.crossing_over().recalculate_fitness().mutate().recalculate_fitness().get_best`. The function `mutate` gently mutates the reasonable best `Route`s by swapping two element of its "gene streams". `crossing_over` is responsable for selecting the current best, inserting it in the next population and then for each other element in the vector it chooses the best fitted element between 15 elements of the gene stream probably (70%) mix with another one in the next generation. `recalculate_fitness` recalculates the fitness for each `Route`.

The Route implementation has the fields `cities: Vec<City>, distance: f64, fitness: f64`, where `cities` is the `Route` itself, `distance` is total distance between each consecutive node of `City` in `cities`, in latitude/longitude (did not convert to km). `fitness` is the value to be compared when looking for the best.

`City` is basically a wrapper of the CSV line content, this is why `#[allow(non_snake_case)]` was used, so I could easily match the CSV headers with my struct. The total `distance` implementation is a trait `Distance` implemented for `Vec<City>`, it sums the distance of each pair of cities:

```rust
impl Distance for Vec<City> {
    fn distance(&self) -> f64 {
        self.par_windows(POINTS_TO_DISTANCE)
            .map(|c| c[0].octagonal_distance(&c[1]))
            .sum()
    }
}
```

* `Route` is the chromossom and `City` is the gene.
* I really hate that `main` is so mutable.

### Crates used
* `rand`: Generates a near safe approximation of a randomly seeded sequence. Important for better variability in the code. As well as retrieving random slices.
* `csv`: Parses csv files. 
* `serde`: Helps parsing the CSV line into the struct `City`.
* `rayon`: Rayon was a test on parallelism that I didn't remove, it had a very small time improvement, but the heavies calculation in not thread safe and it is found in `crossing_over` function.

### Possible Improvements
* One possible improvement would be some normalization algorithm for the `fitness` value.
* Test, as it is a common algorithm which hardest part is not the logic itself, but the tunning of the algorithm, I used my time trying to tune it better than test it. However, I elaborated a test guide for this problem:
    * `io::read_csv` could read at an integration level `test.csv`.
    * `domain::city::uniqueness_count` could receive a vector that we know how many unique elements there are in it.
    * `domain::city::octagonal_distance` is just simples math, but we could test it indirectly via `domain::city::distance` for more than 2 elements in the vector.
    * `domain::route::mutate` is possible to test with a small vector of `Route` that have a known fitness. Only one of them would have a fitness that could mutate so we could check that the mutated element is different than the previous one, as well as testing that the others remained the same.
    * The problem of testing `domain::route::recalculate_fitness` is that it has many variable, and some of them are expensive to test, like `uniqueness_prize`. However, its negative value is known to us, so we could test `San Francisco` prize in each case as well as the fitness calculation for specific `distances`.
    * `domain::route::crossing_over` is the hardest to test as it has random values associated with it. Usually when we have random values and need to unit test something, we pass it as argument, but in this case we would need to pass too many random values to make this feasable. The simplest test would be to assure the previous population and the new population are different.
    * `domain::route::choose_parent` could be tested by fixing a population of size `CROSSING_OVER_CANDIDATES`, then we would know the best fitness and select it. Maybe `CROSSING_OVER_CANDIDATES` could be annotated with `#[cfg(test)]` and have size 3 to help us have a smaller population to choose from.
    * `domain::route::get_best` is mostly giving a know population, with known fitnesses, and selecting the best, that we already know.
    * `domain::route::gcalculate_fitness` is just mathematics like `octagonal_distance`.
    * `MyOrd::cmp` would be interesting to test, but it only needs two cases, 1. larger float tested againt smaller float, 2. smaller float tested against larger float.


## The distance function
The best way to calculate the exact distance in an Euclidean Space is to use the Euclidean Distance Algorithm. However, this approach can be overengineer as it takes longer to calculate and won't stay linear over distance increase due to power of 2 and square root. So my approach was to use a more empiracal calculation that would reduce calculation time by half using the data I had. It was inspired by the octagonal approximation of points in the N-Space. The error compared to Eucledean Algorithm is around +/-1.5% that is sufficient to calculate distance on a national scale and assume it is perfectly correct (road, detours, fueling, etc).

```rust
fn octagonal_distance(a: City, b: City) -> f64 {
    let dx = (b.latitute - a.latitute).abs();
    let dy = (b.longitude - a.longitude).abs();

    0.4 * (dx + dy)  + 0.55 * (dx.max(dy))
}
```

## Generating Combinations
I chose to use `rand::seq::SliceRandom` as it allows a more immutable solution to shuffle vectors than `rand::shuffle`, also it doesn't need cloning for each new case.

```rust
let cities = read_csv;
let cities_rand = cities.choose_multiple(&mut rng, cities.len()).cloned().collect::<Vec<City>>();
```

## Comparing Floats

Rust doesn't have a std solution to compare float, so in most cases the crate `ordered_float` can be used. However, as I didn't want to use too many crates, I came with a simple solution that is comparing without equity by implementing a trait called `MyOrd` which has and `Ordering` as result. This helps us retrieve the best `Route` in a population by comparing `fitness`.

```rust
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
```

## Running the example
To run the `cities_all.csv` you should execute `cargo run --release < cities_all.csv` and to runt he `test.csv` you should run `cargo run --release < test.csv`.

* As `test.csv` is way smaller, its coefficients are different and the tunning was way easier. Not fixing the coefficient for `test.csv` will result in a case where the best solution is to almost never leave `San Francisco`.
* If you want a more optimal solution `const MINIMUM_FITNESS: f64 = 21f64` could be changed to `23f64`, but this can take around 20 interactions

## Example output:
To keep track of the time taken the program prints the new best interation counter, the current best fitness and the normalized distance.
```
...
300: 21.368710719086252, 4.368710719086254
Cities: San Francisco,California
Southaven,Mississippi
Edmond,Oklahoma
Franklin,Tennessee
Shakopee,Minnesota
Poway,California
Providence,Rhode Island
Plymouth,Minnesota
Decatur,Illinois
Springfield,Illinois
Cape Girardeau,Missouri
Norfolk,Virginia
Tucson,Arizona
Camden,New Jersey
Maplewood,Minnesota
Kansas City,Kansas
Las Cruces,New Mexico
Oklahoma City,Oklahoma
Schaumburg,Illinois
Glendale,California
El Paso,Texas
El Monte,California
Colton,California
Everett,Washington
Youngstown,Ohio
Toledo,Ohio
Lincoln Park,Michigan
Bozeman,Montana
Miramar,Florida
Moreno Valley,California
East Lansing,Michigan
Whittier,California
Irvine,California
Delray Beach,Florida
North Little Rock,Arkansas
Arcadia,California
Merced,California
Denver,Colorado
Montebello,California
Calexico,California
Palm Desert,California
Surprise,Arizona
Fort Myers,Florida
Union City,New Jersey
Hagerstown,Maryland
Newport Beach,California
Wausau,Wisconsin
Draper,Utah
Pinellas Park,Florida
Wheeling,Illinois
Eagan,Minnesota
Kettering,Ohio
Beaumont,Texas
Florissant,Missouri
Erie,Pennsylvania
North Miami Beach,Florida
Goose Creek,South Carolina
Bonita Springs,Florida
Lake Havasu City,Arizona
Hanover Park,Illinois
Fontana,California
Laguna Niguel,California
Livonia,Michigan
Newton,Massachusetts
Washington,District of Columbia
Aliso Viejo,California
Cleveland,Ohio
Pearland,Texas
Redondo Beach,California
Auburn,Alabama
Davis,California
Eugene,Oregon
Longview,Texas
Warren,Michigan
Reno,Nevada
Pleasanton,California
Vista,California
Aurora,Colorado
Roswell,New Mexico
Redlands,California
Bell Gardens,California
Casper,Wyoming
Clovis,California
Napa,California
Burnsville,Minnesota
Charlottesville,Virginia
Coeur d'Alene,Idaho
Perris,California
Milwaukee,Wisconsin
Peabody,Massachusetts
New York,New York
Carpentersville,Illinois
St. Peters,Missouri
New Brunswick,New Jersey
Annapolis,Maryland
Macon,Georgia
Dayton,Ohio
Allentown,Pennsylvania
Rosemead,California
Bowie,Maryland
Chino Hills,California
Madison,Alabama
Nampa,Idaho
Edinburg,Texas
Roseville,California
Weston,Florida
Worcester,Massachusetts
Norman,Oklahoma
Brentwood,California
San Ramon,California
Round Rock,Texas
Rapid City,South Dakota
Rancho Cucamonga,California
Castle Rock,Colorado
Little Rock,Arkansas
Chico,California
Campbell,California
Yakima,Washington
Gilbert,Arizona
Chino,California
Lynchburg,Virginia
Huber Heights,Ohio
Woonsocket,Rhode Island
San Jose,California
Orange,California
Bullhead City,Arizona
Santa Ana,California
Commerce City,Colorado
Fort Collins,Colorado
Bend,Oregon
Kirkland,Washington
Chandler,Arizona
West Covina,California
La Mirada,California
Santee,California
Raleigh,North Carolina
Yuba City,California
Orlando,Florida
Carson,California
Valdosta,Georgia
Long Beach,California
Keizer,Oregon
Stockton,California
Shelton,Connecticut
Hattiesburg,Mississippi
Vallejo,California
Loveland,Colorado
San Leandro,California
Riverton,Utah
Waterbury,Connecticut
Homestead,Florida
Nashua,New Hampshire
Minnetonka,Minnesota
Bellevue,Washington
Yorba Linda,California
Simi Valley,California
Sanford,Florida
Fitchburg,Massachusetts
Norwalk,Connecticut
Hempstead,New York
New Rochelle,New York
Columbus,Georgia
Bloomington,Illinois
Topeka,Kansas
Royal Oak,Michigan
Altoona,Pennsylvania
South Bend,Indiana
Waltham,Massachusetts
Greenville,North Carolina
Cambridge,Massachusetts
Folsom,California
Houston,Texas
Gulfport,Mississippi
Frederick,Maryland
Grand Island,Nebraska
Frisco,Texas
Oro Valley,Arizona
Montgomery,Alabama
Schenectady,New York
Buffalo,New York
Akron,Ohio
Titusville,Florida
Enid,Oklahoma
Gaithersburg,Maryland
Roanoke,Virginia
Bedford,Texas
Woburn,Massachusetts
Atlantic City,New Jersey
Jacksonville,Florida
Missoula,Montana
New Bedford,Massachusetts
Cedar Falls,Iowa
Danbury,Connecticut
Noblesville,Indiana
Troy,Michigan
Rowlett,Texas
Midland,Texas
Duluth,Minnesota
Tracy,California
Malden,Massachusetts
Hialeah,Florida
Murfreesboro,Tennessee
White Plains,New York
Cincinnati,Ohio
Ames,Iowa
Germantown,Tennessee
Calumet City,Illinois
Wellington,Florida
Dothan,Alabama
Columbia,Missouri
Maple Grove,Minnesota
Idaho Falls,Idaho
Auburn,Washington
Corpus Christi,Texas
Collierville,Tennessee
Lawrence,Indiana
Smyrna,Tennessee
Springfield,Massachusetts
Mount Pleasant,South Carolina
Jefferson City,Missouri
Allen,Texas
Fort Worth,Texas
Redmond,Washington
St. George,Utah
Spartanburg,South Carolina
Friendswood,Texas
Menifee,California
Scranton,Pennsylvania
Haverhill,Massachusetts
Yonkers,New York
Hilton Head Island,South Carolina
St. Joseph,Missouri
Wilmington,North Carolina
Coon Rapids,Minnesota
Spokane Valley,Washington
Temecula,California
Rancho Palos Verdes,California
Apple Valley,California
Des Moines,Iowa
Coral Gables,Florida
Ontario,California
Fountain Valley,California
East Orange,New Jersey
Logan,Utah
South San Francisco,California
San Mateo,California
Medford,Oregon
Columbia,South Carolina
Ann Arbor,Michigan
Mobile,Alabama
Perth Amboy,New Jersey
Mesa,Arizona
Chapel Hill,North Carolina
Prescott Valley,Arizona
Abilene,Texas
Marysville,Washington
San Francisco,California 
Distance: 4578.009688905
```
