use crate::modules::utils::streaming::{increment_slice, parse_number_from_stream_i64};

static SEEDS: &[u8] = "seeds: ".as_bytes();
static SEED_TO_SOIL: &[u8] = "seed-to-soil map:".as_bytes();
static SOIL_TO_FERTILISER: &[u8] = "soil-to-fertilizer map:".as_bytes();
static FERTILISER_TO_WATER: &[u8] = "fertilizer-to-water map:".as_bytes();
static WATER_TO_LIGHT: &[u8] = "water-to-light map:".as_bytes();
static LIGHT_TO_TEMPERATURE: &[u8] = "light-to-temperature map:".as_bytes();
static TEMPERATURE_TO_HUMIDITY: &[u8] = "temperature-to-humidity map:".as_bytes();
static HUMIDITY_TO_LOCATION: &[u8] = "humidity-to-location map:".as_bytes();

struct MappedRange {
    start: i64,
    end: i64,
    offset: i64,
}

struct Map {
    ranges: Vec<MappedRange>,
}

impl Map {
    fn new() -> Self {
        Map { ranges: vec![] }
    }

    fn insert(&mut self, target: &i64, source: &i64, range: &i64) {
        let mapped_range = MappedRange {
            start: source.clone(),
            end: source + range,
            offset: target - source,
        };
        self.ranges.push(mapped_range);
    }

    pub fn map_value(&self, value: &mut i64) {
        for range in self.ranges.iter() {
            if range.start <= *value && *value < range.end {
                *value += range.offset;
                return;
            }
        }
    }
}

/// Map seeds over multiple ranges
pub fn lowest_seed_location(payload: &str) -> i64 {
    let mut slice: &[u8] = payload.as_bytes();
    let mut map_index: u8 = 0;
    let mut target: i64 = 0;
    let mut source: i64 = 0;
    let mut read_seeds: bool = true;
    let mut current_number: i64;

    let mut seeds: Vec<i64> = vec![];
    let mut seed_to_soil = Map::new();
    let mut soil_to_fert = Map::new();
    let mut fert_to_watr = Map::new();
    let mut watr_to_ligh = Map::new();
    let mut ligh_to_temp = Map::new();
    let mut temp_to_humd = Map::new();
    let mut humd_to_loca = Map::new();

    {
        let mut current_map: &mut Map = &mut seed_to_soil;

        while !slice.is_empty() {
            if slice[0].is_ascii_digit() {
                if read_seeds {
                    (current_number, slice) = parse_number_from_stream_i64(slice);
                    seeds.push(current_number);
                } else {
                    map_index += 1;
                    (current_number, slice) = parse_number_from_stream_i64(slice);
                    if map_index == 1 {
                        target = current_number;
                    } else if map_index == 2 {
                        source = current_number;
                    } else {
                        map_index = 0;
                        current_map.insert(&target, &source, &current_number);
                    }
                }
            } else if slice.starts_with(SEEDS) {
                slice = increment_slice(slice, SEEDS.len());
                read_seeds = true;
            } else if slice.starts_with(SEED_TO_SOIL) {
                slice = increment_slice(slice, SEED_TO_SOIL.len());
                read_seeds = false;
                current_map = &mut seed_to_soil;
            } else if slice.starts_with(SOIL_TO_FERTILISER) {
                slice = increment_slice(slice, SOIL_TO_FERTILISER.len());
                read_seeds = false;
                current_map = &mut soil_to_fert;
            } else if slice.starts_with(FERTILISER_TO_WATER) {
                slice = increment_slice(slice, FERTILISER_TO_WATER.len());
                read_seeds = false;
                current_map = &mut fert_to_watr;
            } else if slice.starts_with(WATER_TO_LIGHT) {
                slice = increment_slice(slice, WATER_TO_LIGHT.len());
                read_seeds = false;
                current_map = &mut watr_to_ligh;
            } else if slice.starts_with(LIGHT_TO_TEMPERATURE) {
                slice = increment_slice(slice, LIGHT_TO_TEMPERATURE.len());
                read_seeds = false;
                current_map = &mut ligh_to_temp;
            } else if slice.starts_with(TEMPERATURE_TO_HUMIDITY) {
                slice = increment_slice(slice, TEMPERATURE_TO_HUMIDITY.len());
                read_seeds = false;
                current_map = &mut temp_to_humd;
            } else if slice.starts_with(HUMIDITY_TO_LOCATION) {
                slice = increment_slice(slice, HUMIDITY_TO_LOCATION.len());
                read_seeds = false;
                current_map = &mut humd_to_loca;
            } else {
                slice = increment_slice(slice, 1);
            }
        }
    }

    let mut output: i64 = i64::MAX;
    for seed in seeds.iter_mut() {
        seed_to_soil.map_value(seed);
        soil_to_fert.map_value(seed);
        fert_to_watr.map_value(seed);
        watr_to_ligh.map_value(seed);
        ligh_to_temp.map_value(seed);
        temp_to_humd.map_value(seed);
        humd_to_loca.map_value(seed);
        output = output.min(*seed);
    }

    return output;
}
