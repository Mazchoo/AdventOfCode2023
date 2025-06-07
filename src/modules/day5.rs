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
    value: i64,
}

impl MappedRange {
    pub fn offset(&mut self, amount: &i64) {
        self.start += amount;
        self.end += amount;
    }

    pub fn overlap(&self, other: &Self) -> Option<MappedRange> {
        let overlap_start = self.start.max(other.start);
        let overlap_end = self.end.min(other.end);
        if overlap_start < overlap_end {
            return Some(MappedRange {
                start: overlap_start,
                end: overlap_end,
                value: self.value,
            });
        } else {
            return None;
        }
    }

    pub fn split_off_range(&self, other: &MappedRange) -> Vec<MappedRange> {
        let mut split_ranges: Vec<MappedRange> = vec![];
        if other.start > self.start {
            split_ranges.push(MappedRange {
                start: self.start,
                end: other.start,
                value: self.value,
            });
        }
        if other.end < self.end {
            split_ranges.push(MappedRange {
                start: other.end,
                end: self.end,
                value: self.value,
            });
        }
        return split_ranges;
    }
}

struct Map {
    ranges: Vec<MappedRange>,
}

impl Map {
    fn new() -> Self {
        Map { ranges: vec![] }
    }

    pub fn insert(&mut self, target: &i64, source: &i64, range: &i64) {
        let mapped_range = MappedRange {
            start: source.clone(),
            end: source + range,
            value: target - source,
        };
        self.ranges.push(mapped_range);
    }

    pub fn map_value(&self, value: &mut i64) {
        for range in self.ranges.iter() {
            if range.start <= *value && *value < range.end {
                *value += range.value;
                return;
            }
        }
    }

    pub fn map_ranges(&self, input_ranges: &mut Vec<MappedRange>) {
        let mut output_ranges: Vec<MappedRange> = vec![];

        for range in self.ranges.iter() {
            if input_ranges.len() == 0 {
                break;
            }

            let mut new_ranges: Vec<MappedRange> = vec![];

            let mut i: usize = 0;
            while i < input_ranges.len() {
                let input_range = &mut input_ranges[i];
                if let Some(mut overlap) = input_range.overlap(range) {
                    new_ranges.extend(input_range.split_off_range(&overlap));
                    overlap.offset(&range.value);
                    output_ranges.push(overlap);
                    input_ranges.remove(i);
                } else {
                    i += 1;
                }
            }

            input_ranges.extend(new_ranges);
        }

        input_ranges.extend(output_ranges);
    }
}

struct SeedToLocMaps {
    seeds: Vec<i64>,
    seed_to_soil: Map,
    soil_to_fert: Map,
    fert_to_watr: Map,
    watr_to_ligh: Map,
    ligh_to_temp: Map,
    temp_to_humd: Map,
    humd_to_loca: Map,
}

impl SeedToLocMaps {
    fn new(payload: &str) -> Self {
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

        return SeedToLocMaps {
            seeds: seeds,
            seed_to_soil: seed_to_soil,
            soil_to_fert: soil_to_fert,
            fert_to_watr: fert_to_watr,
            watr_to_ligh: watr_to_ligh,
            ligh_to_temp: ligh_to_temp,
            temp_to_humd: temp_to_humd,
            humd_to_loca: humd_to_loca,
        };
    }

    pub fn get_min_mapped_seed(&self) -> i64 {
        let mut output: i64 = i64::MAX;
        for seed in self.seeds.iter() {
            let mut mapped_seed = seed.clone();
            self.seed_to_soil.map_value(&mut mapped_seed);
            self.soil_to_fert.map_value(&mut mapped_seed);
            self.fert_to_watr.map_value(&mut mapped_seed);
            self.watr_to_ligh.map_value(&mut mapped_seed);
            self.ligh_to_temp.map_value(&mut mapped_seed);
            self.temp_to_humd.map_value(&mut mapped_seed);
            self.humd_to_loca.map_value(&mut mapped_seed);
            output = output.min(mapped_seed);
        }
        return output;
    }

    pub fn get_seeds_as_ranges(&self) -> Vec<MappedRange> {
        let mut output: Vec<MappedRange> = vec![];

        let nr_ranges = self.seeds.len() / 2;
        for i in 0..nr_ranges {
            output.push(MappedRange {
                start: self.seeds[i * 2],
                end: self.seeds[i * 2] + self.seeds[i * 2 + 1],
                value: 0,
            });
        }

        return output;
    }

    // Could fuse ranges if number of ranges gets too big
    // Number output ranges O(3^nr_maps) which isn't too bad for this
    pub fn get_min_mapped_seed_range(&self) -> i64 {
        let mut seed_ranges = self.get_seeds_as_ranges();

        self.seed_to_soil.map_ranges(&mut seed_ranges);
        self.soil_to_fert.map_ranges(&mut seed_ranges);
        self.fert_to_watr.map_ranges(&mut seed_ranges);
        self.watr_to_ligh.map_ranges(&mut seed_ranges);
        self.ligh_to_temp.map_ranges(&mut seed_ranges);
        self.temp_to_humd.map_ranges(&mut seed_ranges);
        self.humd_to_loca.map_ranges(&mut seed_ranges);

        let mut output = i64::MAX;
        for range in seed_ranges {
            output = output.min(range.start);
        }
        return output;
    }
}

/// Map seeds over multiple ranges
pub fn lowest_seed_location(payload: &str) -> i64 {
    let seed_maps = SeedToLocMaps::new(payload);
    return seed_maps.get_min_mapped_seed();
}

/// Map ranges over multiple ranges
pub fn lowest_seed_range_location(payload: &str) -> i64 {
    let seed_maps = SeedToLocMaps::new(payload);
    return seed_maps.get_min_mapped_seed_range();
}
