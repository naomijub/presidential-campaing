# The algorithm

As a basic travel salesperson algorithm, I decided to go with the best easiest solution that is to use a Genetic algorithm to solve this problem. Faster and less precise algorithms would be Greedy and Local Reasearch, as well as a simples simulated annealing.

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
let cities = vec![a, b, c, d, e, f, g];
let cities_rand = cities.choose_multiple(&mut rng, cities.len()).cloned().collect::<Vec<City>>();
```