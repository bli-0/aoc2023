use std::collections::HashMap;

fn main() {
    let inputs = include_str!("inputs/05");
    let (init_seeds, mappings) = parse(inputs);

    // part1;
    let dest_locations: Vec<i64> = init_seeds
        .iter()
        .map(|seed| {
            let seed_to_soil = mappings.get(&(Thing::Seed, Thing::Soil)).unwrap();
            let mut soil = *seed;
            for map in seed_to_soil {
                if let Some(x) = map.try_map_num(*seed) {
                    soil = x;
                }
            }

            let soil_to_fert = mappings.get(&(Thing::Soil, Thing::Fertilizer)).unwrap();
            let mut fert = soil;
            for map in soil_to_fert {
                if let Some(x) = map.try_map_num(soil) {
                    fert = x;
                }
            }

            let fert_to_water = mappings.get(&(Thing::Fertilizer, Thing::Water)).unwrap();
            let mut water = fert;
            for map in fert_to_water {
                if let Some(x) = map.try_map_num(fert) {
                    water = x;
                }
            }

            let water_to_light = mappings.get(&(Thing::Water, Thing::Light)).unwrap();
            let mut light: i64 = water;
            for map in water_to_light {
                if let Some(x) = map.try_map_num(water) {
                    light = x;
                }
            }

            let light_to_temp = mappings.get(&(Thing::Light, Thing::Temperature)).unwrap();
            let mut temp: i64 = light;
            for map in light_to_temp {
                if let Some(x) = map.try_map_num(light) {
                    temp = x;
                }
            }

            let temp_to_humidity = mappings
                .get(&(Thing::Temperature, Thing::Humidity))
                .unwrap();
            let mut humidity: i64 = temp;
            for map in temp_to_humidity {
                if let Some(x) = map.try_map_num(temp) {
                    humidity = x;
                }
            }

            let humidity_to_location = mappings.get(&(Thing::Humidity, Thing::Location)).unwrap();
            let mut location: i64 = humidity;
            for map in humidity_to_location {
                if let Some(x) = map.try_map_num(humidity) {
                    location = x;
                }
            }

            location
        })
        .collect();

    let part1 = dest_locations
        .iter()
        .reduce(|a, b| if a < b { a } else { b })
        .unwrap();

    println!("part1: {}", part1);

    // part2
    let mut actual_seeds: Vec<i64> = vec![];
    let mut seed_iter = init_seeds.iter();
    while let Some(first) = seed_iter.next() {
        let range: &i64 = seed_iter.next().unwrap();
        for seed in *first..=*first + range {
            actual_seeds.push(seed);
        }
    }

    let dest_locations2: Vec<i64> = actual_seeds
        .iter()
        .map(|seed| {
            let seed_to_soil = mappings.get(&(Thing::Seed, Thing::Soil)).unwrap();
            let mut soil = *seed;
            for map in seed_to_soil {
                if let Some(x) = map.try_map_num(*seed) {
                    soil = x;
                }
            }

            let soil_to_fert = mappings.get(&(Thing::Soil, Thing::Fertilizer)).unwrap();
            let mut fert = soil;
            for map in soil_to_fert {
                if let Some(x) = map.try_map_num(soil) {
                    fert = x;
                }
            }

            let fert_to_water = mappings.get(&(Thing::Fertilizer, Thing::Water)).unwrap();
            let mut water = fert;
            for map in fert_to_water {
                if let Some(x) = map.try_map_num(fert) {
                    water = x;
                }
            }

            let water_to_light = mappings.get(&(Thing::Water, Thing::Light)).unwrap();
            let mut light: i64 = water;
            for map in water_to_light {
                if let Some(x) = map.try_map_num(water) {
                    light = x;
                }
            }

            let light_to_temp = mappings.get(&(Thing::Light, Thing::Temperature)).unwrap();
            let mut temp: i64 = light;
            for map in light_to_temp {
                if let Some(x) = map.try_map_num(light) {
                    temp = x;
                }
            }

            let temp_to_humidity = mappings
                .get(&(Thing::Temperature, Thing::Humidity))
                .unwrap();
            let mut humidity: i64 = temp;
            for map in temp_to_humidity {
                if let Some(x) = map.try_map_num(temp) {
                    humidity = x;
                }
            }

            let humidity_to_location = mappings.get(&(Thing::Humidity, Thing::Location)).unwrap();
            let mut location: i64 = humidity;
            for map in humidity_to_location {
                if let Some(x) = map.try_map_num(humidity) {
                    location = x;
                }
            }

            location
        })
        .collect();

    println!("{:?}", dest_locations2);

    let part2 = dest_locations2
        .iter()
        .reduce(|a, b| if a < b { a } else { b })
        .unwrap();

    println!("part2: {}", part2);
}

fn parse(inputs: &str) -> (Vec<i64>, HashMap<(Thing, Thing), Vec<Map>>) {
    // Much faster to just hard code the line nos.
    let lines: Vec<&str> = inputs.split('\n').collect();
    let seeds = lines[0]
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let mut mappings = HashMap::<(Thing, Thing), Vec<Map>>::new();

    let seed_to_soil_start = lines
        .iter()
        .position(|s| *s == "seed-to-soil map:")
        .unwrap();
    let seed_to_soil_end = lines
        .iter()
        .skip(seed_to_soil_start)
        .position(|l| l.is_empty())
        .unwrap();
    let seed_to_soil_mappings = lines
        .iter()
        .skip(seed_to_soil_start + 1)
        .take(seed_to_soil_end - 1)
        .map(|line| {
            let nums: Vec<&str> = line.split(' ').collect();
            Map {
                dest_start: nums[0].parse::<i64>().unwrap(),
                orig_start: nums[1].parse::<i64>().unwrap(),
                range: nums[2].parse::<i64>().unwrap(),
            }
        })
        .collect();

    mappings.insert((Thing::Seed, Thing::Soil), seed_to_soil_mappings);

    let soil_to_fertilizer_start = lines
        .iter()
        .position(|s| *s == "soil-to-fertilizer map:")
        .unwrap();
    let soil_to_fertilizer_end = lines
        .iter()
        .skip(soil_to_fertilizer_start)
        .position(|l| l.is_empty())
        .unwrap();
    let soil_to_fertilizer_mappings = lines
        .iter()
        .skip(soil_to_fertilizer_start + 1)
        .take(soil_to_fertilizer_end - 1)
        .map(|line| {
            let nums: Vec<&str> = line.split(' ').collect();
            Map {
                dest_start: nums[0].parse::<i64>().unwrap(),
                orig_start: nums[1].parse::<i64>().unwrap(),
                range: nums[2].parse::<i64>().unwrap(),
            }
        })
        .collect();
    mappings.insert(
        (Thing::Soil, Thing::Fertilizer),
        soil_to_fertilizer_mappings,
    );

    let fertilizer_to_water_start = lines
        .iter()
        .position(|s| *s == "fertilizer-to-water map:")
        .unwrap();
    let fertilizer_to_water_end = lines
        .iter()
        .skip(fertilizer_to_water_start)
        .position(|l| l.is_empty())
        .unwrap();
    let fertilizer_to_water_mappings = lines
        .iter()
        .skip(fertilizer_to_water_start + 1)
        .take(fertilizer_to_water_end - 1)
        .map(|line| {
            let nums: Vec<&str> = line.split(' ').collect();
            Map {
                dest_start: nums[0].parse::<i64>().unwrap(),
                orig_start: nums[1].parse::<i64>().unwrap(),
                range: nums[2].parse::<i64>().unwrap(),
            }
        })
        .collect();
    mappings.insert(
        (Thing::Fertilizer, Thing::Water),
        fertilizer_to_water_mappings,
    );

    let water_to_light_start = lines
        .iter()
        .position(|s| *s == "water-to-light map:")
        .unwrap();
    let water_to_light_end = lines
        .iter()
        .skip(water_to_light_start)
        .position(|l| l.is_empty())
        .unwrap();
    let water_to_light_mappings = lines
        .iter()
        .skip(water_to_light_start + 1)
        .take(water_to_light_end - 1)
        .map(|line| {
            let nums: Vec<&str> = line.split(' ').collect();
            Map {
                dest_start: nums[0].parse::<i64>().unwrap(),
                orig_start: nums[1].parse::<i64>().unwrap(),
                range: nums[2].parse::<i64>().unwrap(),
            }
        })
        .collect();
    mappings.insert((Thing::Water, Thing::Light), water_to_light_mappings);

    let light_to_temperature_start = lines
        .iter()
        .position(|s| *s == "light-to-temperature map:")
        .unwrap();
    let light_to_temperature_end = lines
        .iter()
        .skip(light_to_temperature_start)
        .position(|l| l.is_empty())
        .unwrap();
    let light_to_temperature_mappings = lines
        .iter()
        .skip(light_to_temperature_start + 1)
        .take(light_to_temperature_end - 1)
        .map(|line| {
            let nums: Vec<&str> = line.split(' ').collect();
            Map {
                dest_start: nums[0].parse::<i64>().unwrap(),
                orig_start: nums[1].parse::<i64>().unwrap(),
                range: nums[2].parse::<i64>().unwrap(),
            }
        })
        .collect();
    mappings.insert(
        (Thing::Light, Thing::Temperature),
        light_to_temperature_mappings,
    );

    let temperature_to_humidity_start = lines
        .iter()
        .position(|s| *s == "temperature-to-humidity map:")
        .unwrap();
    let temperature_to_humidity_end = lines
        .iter()
        .skip(temperature_to_humidity_start)
        .position(|l| l.is_empty())
        .unwrap();
    let temperature_to_humidity_mappings = lines
        .iter()
        .skip(temperature_to_humidity_start + 1)
        .take(temperature_to_humidity_end - 1)
        .map(|line| {
            let nums: Vec<&str> = line.split(' ').collect();
            Map {
                dest_start: nums[0].parse::<i64>().unwrap(),
                orig_start: nums[1].parse::<i64>().unwrap(),
                range: nums[2].parse::<i64>().unwrap(),
            }
        })
        .collect();
    mappings.insert(
        (Thing::Temperature, Thing::Humidity),
        temperature_to_humidity_mappings,
    );

    let humidity_to_location_start = lines
        .iter()
        .position(|s| *s == "humidity-to-location map:")
        .unwrap();
    let humidity_to_location_mappings = lines
        .iter()
        .skip(humidity_to_location_start + 1)
        .map(|line| {
            let nums: Vec<&str> = line.split(' ').collect();
            Map {
                dest_start: nums[0].parse::<i64>().unwrap(),
                orig_start: nums[1].parse::<i64>().unwrap(),
                range: nums[2].parse::<i64>().unwrap(),
            }
        })
        .collect();
    mappings.insert(
        (Thing::Humidity, Thing::Location),
        humidity_to_location_mappings,
    );

    (seeds, mappings)
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum Thing {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug)]
struct Map {
    dest_start: i64,
    orig_start: i64,
    range: i64,
}

impl Map {
    fn try_map_num(&self, num: i64) -> Option<i64> {
        if num >= self.orig_start && num <= self.orig_start + self.range {
            let diff = self.orig_start + self.range - num;
            Some(self.dest_start + self.range - diff)
        } else {
            None
        }
    }
}
