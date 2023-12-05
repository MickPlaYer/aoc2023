use std::ops::RangeInclusive;

#[derive(Debug)]
struct MapLine {
    dest: usize,
    source: usize,
    range: usize,
}

impl MapLine {
    fn get(&self, key: usize) -> Option<usize> {
        let source_range = self.get_source_range();
        if source_range.contains(&key) {
            let value = self.dest + (key - self.source);
            Some(value)
        } else {
            None
        }
    }

    fn get_source_range(&self) -> RangeInclusive<usize> {
        self.source..=(self.source + self.range - 1)
    }

    fn get_range(&self, range: &RangeInclusive<usize>) -> RangeInclusive<usize> {
        let start = self.dest + (range.start() - self.source);
        let end = self.dest + (range.end() - self.source);
        start..=end
    }
}

#[derive(Debug)]
pub struct Map {
    lines: Vec<MapLine>,
}

impl Map {
    pub fn new(data: Vec<(usize, usize, usize)>) -> Self {
        let mut data = data.clone();
        data.sort_by_key(|(_, source, _)| *source);
        Self {
            lines: data
                .into_iter()
                .map(|(dest, source, range)| MapLine {
                    dest,
                    source,
                    range,
                })
                .collect(),
        }
    }

    fn get(&self, key: usize) -> usize {
        let mut ranges = self.lines.iter().map(|range| range.get(key));
        while let Some(value) = ranges.next() {
            if let Some(value) = value {
                return value;
            }
        }
        key
    }

    fn get_ranges(&self, ranges: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
        let mut result = Vec::new();
        ranges.into_iter().for_each(|range| {
            let mut has_last_remain = Some(range.clone());
            let mut skips = Vec::new();
            let mut dest_ranges = Vec::new();
            self.lines.iter().for_each(|source_and_dest| {
                if has_last_remain.is_none() {
                    return;
                }
                let last_remain = has_last_remain.clone().unwrap();
                let source = source_and_dest.get_source_range();
                let intersection = range_intersection(&source, &last_remain);
                if let Some(intersection) = intersection {
                    let (skip, remain) = cut_range(&last_remain, &intersection);
                    if let Some(skip) = skip {
                        skips.push(skip);
                    }
                    has_last_remain = remain;
                    dest_ranges.push(source_and_dest.get_range(&intersection));
                }
            });
            if let Some(last_remain) = has_last_remain {
                result.push(last_remain);
            }
            result.append(&mut skips);
            result.append(&mut dest_ranges);
        });
        result
    }
}

fn range_intersection(
    a: &RangeInclusive<usize>,
    b: &RangeInclusive<usize>,
) -> Option<RangeInclusive<usize>> {
    if a.contains(b.start()) && a.contains(b.end()) {
        Some(b.clone())
    } else if a.contains(b.start()) {
        Some(*b.start()..=*a.end())
    } else if a.contains(b.end()) {
        Some(*a.start()..=*b.end())
    } else {
        None
    }
}

fn cut_range(
    base: &RangeInclusive<usize>,
    cut: &RangeInclusive<usize>,
) -> (Option<RangeInclusive<usize>>, Option<RangeInclusive<usize>>) {
    let skip = if base.start() == cut.start() {
        None
    } else {
        let range = *base.start()..=(*cut.start() - 1);
        if range.is_empty() {
            None
        } else {
            Some(range)
        }
    };
    let remain = if base.end() == cut.end() {
        None
    } else {
        let range = (*cut.end() + 1)..=*base.end();
        if range.is_empty() {
            None
        } else {
            Some(range)
        }
    };
    (skip, remain)
}

#[derive(Debug)]
pub struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl Almanac {
    pub fn new(
        seeds: Vec<usize>,
        seed_to_soil: Map,
        soil_to_fertilizer: Map,
        fertilizer_to_water: Map,
        water_to_light: Map,
        light_to_temperature: Map,
        temperature_to_humidity: Map,
        humidity_to_location: Map,
    ) -> Self {
        Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        }
    }

    pub fn get_locations(&self) -> Vec<usize> {
        self.seeds
            .iter()
            .map(|seed| self.get_seed_to_location(*seed))
            .collect()
    }

    pub fn get_location_ranges(&self) -> Vec<RangeInclusive<usize>> {
        let seed_ranges = self
            .seeds
            .chunks(2)
            .map(|pair| pair[0]..=(pair[0] + pair[1] - 1))
            .collect();
        self.get_seed_range_to_location_ranges(seed_ranges)
    }

    fn get_seed_range_to_location_ranges(
        &self,
        seed: Vec<RangeInclusive<usize>>,
    ) -> Vec<RangeInclusive<usize>> {
        let soil = self.seed_to_soil.get_ranges(seed);
        let fertilizer = self.soil_to_fertilizer.get_ranges(soil);
        let water = self.fertilizer_to_water.get_ranges(fertilizer);
        let light = self.water_to_light.get_ranges(water);
        let temperature = self.light_to_temperature.get_ranges(light);
        let humidity = self.temperature_to_humidity.get_ranges(temperature);
        self.humidity_to_location.get_ranges(humidity)
    }

    fn get_seed_to_location(&self, seed: usize) -> usize {
        let soil = self.seed_to_soil.get(seed);
        let fertilizer = self.soil_to_fertilizer.get(soil);
        let water = self.fertilizer_to_water.get(fertilizer);
        let light = self.water_to_light.get(water);
        let temperature = self.light_to_temperature.get(light);
        let humidity = self.temperature_to_humidity.get(temperature);
        self.humidity_to_location.get(humidity)
    }
}
