# The algorithm

As a basic travel salesperson algorithm, I decided to go with the best easiest solution that is to use a Genetic algorithm to solve this problem. Faster and less precise algorithms would be Greedy and Local Reasearch, as well as a simples simulated annealing.

To avoid convergence problems the inner loop terminates when the last best counter is greater than the current counter plus 600. This took an average of 7 loops to find the solution. 

```rust
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
            }

            if counter > best_counter + 600 {
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

Whenever the `'inner` loop breaks, the `local_best` is show, that mean, the best `Gene` for that loop. When the `'outer` loop breaks, the best `Gene` is printed.
```rust
if counter > best_counter + 600 {
    break 'inner;
}
```

### The implementation
The basic key concept of this algorithm is the population that in this case is a `Vec<Gene>` and implement the trait `Evolution` for `Vec<Gene>`. The trait `Evolution` is responsable for the main caculation in this code base which is `population.mutate().crossing_over().recalculate_fitness().get_best`. The function `mutate` gently mutates the reasonable best `Gene`s by swapping two element of its "gene streams". `crossing_over` is responsable for selecting the current best, inserting it in the next population and then for each other element in the vector it chooses the best fitted element between 15 elements of the gene stream. `recalculate_fitness` recalculates the fitness for each `Gene`.

The Gene implementation has the fields `cities: Vec<City>, distance: f64, fitness: f64`, where `cities` is the `Gene` itself, `distance` is total distance between each consecutive node of `City` in `cities`, in latitude/longitude (did not convert to km). `fitness` is the value to be compared when looking for the best.

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

### Crates used
* `rand`: Generates a near safe approximation of a randomly seeded sequence. Important for better variability in the code. As well as retrieving random slices.
* `csv`: Parses csv files. 
* `serde`: Helps parsing the CSV line into the struct `City`.
* `rayon`: Rayon was a test on parallelism that I didn't remove, it had a very small time improvement, but the heavies calculation in not thread safe and it is found in `crossing_over` function.

### Possible Improvements
* Test, as it is a common algorithm which hardest part is not the logic itself, but the tunning of the algorithm, I used my time trying to tune it better (Did not work quite well).
* As I said before, tune better.
* One possible improvement would be some normalization algorithm for the `fitness` value.
  
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

Rust doesn't have a std solution to compare float, so in most cases the crate `ordered_float` can be used. However, as I didn't want to use too many crates, I came with a simple solution that is comparing without equity by implementing a trait called `MyOrd` which has and `Ordering` as result. This helps us retrieve the best `Gene` in a population by comparing `fitness`.

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