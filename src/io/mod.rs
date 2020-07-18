use crate::domain::city::City;

pub fn read_csv() -> Vec<City> {
    let header = csv::StringRecord::from(vec!["City","State","Latitude","Longitude"]);
    let mut rdr = csv::Reader::from_reader(std::io::stdin());

    let mut cities = rdr
        .records()
        .map(|r| {
            let record: City = r
                .expect("Failed to extract")
                .deserialize(Some(&header))
                .expect("failed to deserialize record");
            record
        })
        .collect::<Vec<City>>();

    cities.push(cities[0].clone());
    cities
}