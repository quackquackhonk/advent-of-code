use std::ops::Range;

use itertools::Itertools;

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<usize>,
    pub seed_to_soil: ConversionMap,
    pub soil_to_fertilizer: ConversionMap,
    pub fertilizer_to_water: ConversionMap,
    pub water_to_light: ConversionMap,
    pub light_to_temp: ConversionMap,
    pub temp_to_humidity: ConversionMap,
    pub humidity_to_location: ConversionMap,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AlmanacType {
    Seed(usize),
    Soil(usize),
    Fertilizer(usize),
    Water(usize),
    Light(usize),
    Temperature(usize),
    Humidity(usize),
    Location(usize),
}

impl AlmanacType {
    pub fn value(&self) -> usize {
        match self {
            AlmanacType::Seed(x) => *x,
            AlmanacType::Soil(x) => *x,
            AlmanacType::Fertilizer(x) => *x,
            AlmanacType::Water(x) => *x,
            AlmanacType::Light(x) => *x,
            AlmanacType::Temperature(x) => *x,
            AlmanacType::Humidity(x) => *x,
            AlmanacType::Location(x) => *x,
        }
    }
}

impl Almanac {
    pub fn expand_seeds(self) -> Almanac {
        let seeds: Vec<usize> = self
            .seeds
            .chunks(2)
            .flat_map(|s| s[0]..(s[0] + s[1]))
            .collect();

        Almanac { seeds, ..self }
    }

    pub fn lookup(&self, key: &AlmanacType) -> AlmanacType {
        match key {
            AlmanacType::Seed(_) => self.seed_to_soil.lookup(key),
            AlmanacType::Soil(_) => self.soil_to_fertilizer.lookup(key),
            AlmanacType::Fertilizer(_) => self.fertilizer_to_water.lookup(key),
            AlmanacType::Water(_) => self.water_to_light.lookup(key),
            AlmanacType::Light(_) => self.light_to_temp.lookup(key),
            AlmanacType::Temperature(_) => self.temp_to_humidity.lookup(key),
            AlmanacType::Humidity(_) => self.humidity_to_location.lookup(key),
            AlmanacType::Location(_) => *key,
        }
    }

    pub fn all_locations(&self) -> Vec<usize> {
        self.seeds
            .iter()
            .map(|seed| {
                let mut key = dbg!(AlmanacType::Seed(*seed));
                loop {
                    key = match key {
                        AlmanacType::Location(x) => break x,
                        _ => self.lookup(&key),
                    };
                }
            })
            .collect_vec()
    }
}

#[derive(Debug)]
pub struct ConversionMap {
    pub ranges: Vec<AlmanacRange>,
}

impl ConversionMap {
    pub fn lookup(&self, key: &AlmanacType) -> AlmanacType {
        match key {
            AlmanacType::Seed(x) => AlmanacType::Soil(self.generic_lookup(x)),
            AlmanacType::Soil(x) => AlmanacType::Fertilizer(self.generic_lookup(x)),
            AlmanacType::Fertilizer(x) => AlmanacType::Water(self.generic_lookup(x)),
            AlmanacType::Water(x) => AlmanacType::Light(self.generic_lookup(x)),
            AlmanacType::Light(x) => AlmanacType::Temperature(self.generic_lookup(x)),
            AlmanacType::Temperature(x) => AlmanacType::Humidity(self.generic_lookup(x)),
            AlmanacType::Humidity(x) => AlmanacType::Location(self.generic_lookup(x)),
            AlmanacType::Location(x) => AlmanacType::Location(*x),
        }
    }

    fn generic_lookup(&self, key: &usize) -> usize {
        // loop through ranges, trying to find one where key is in the source range
        for ar in &self.ranges {
            if ar.in_source_range(key) {
                if let Some(v) = ar.lookup(key) {
                    return v;
                }
            }
        }

        *key
    }

    pub fn new(ranges: Vec<AlmanacRange>) -> ConversionMap {
        ConversionMap { ranges }
    }
}

#[derive(Debug)]
pub struct AlmanacRange {
    pub source_start: usize,
    pub destination_start: usize,
    pub range_length: usize,
}

impl AlmanacRange {
    pub fn new(destination_start: usize, source_start: usize, range_length: usize) -> Self {
        Self {
            source_start,
            destination_start,
            range_length,
        }
    }

    pub fn in_source_range(&self, v: &usize) -> bool {
        &self.source_start <= v && v <= &(self.source_start + self.range_length)
    }

    pub fn source_range(&self) -> Range<usize> {
        self.source_start..(self.source_start + self.range_length)
    }

    pub fn destination_range(&self) -> Range<usize> {
        self.destination_start..(self.destination_start + self.range_length)
    }

    pub fn lookup(&self, key: &usize) -> Option<usize> {
        if !self.in_source_range(key) {
            None
        } else {
            let offset = key - self.source_start;
            Some(self.destination_start + offset)
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{
        AlmanacRange,
        AlmanacType::{self, *},
        ConversionMap,
    };

    #[rstest]
    #[case((ConversionMap::new(vec![AlmanacRange::new(50, 98, 2), AlmanacRange::new(52, 50, 48)]), Seed(79)), Soil(81))]
    #[case((ConversionMap::new(vec![AlmanacRange::new(50, 98, 2), AlmanacRange::new(52, 50, 48)]), Seed(14)), Soil(14))]
    #[case((ConversionMap::new(vec![AlmanacRange::new(50, 98, 2), AlmanacRange::new(52, 50, 48)]), Seed(55)), Soil(57))]
    #[case((ConversionMap::new(vec![AlmanacRange::new(50, 98, 2), AlmanacRange::new(52, 50, 48)]), Seed(13)), Soil(13))]
    pub fn test_conversion_map_lookup(
        #[case] input: (ConversionMap, AlmanacType),
        #[case] expected: AlmanacType,
    ) -> anyhow::Result<()> {
        let (cv, seed) = input;
        assert_eq!(expected, cv.lookup(&seed));
        Ok(())
    }

    #[rstest]
    #[case((AlmanacRange::new(52, 50, 48), 79), Some(81))]
    pub fn test_almanac_range_lookup(
        #[case] input: (AlmanacRange, usize),
        #[case] expected: Option<usize>,
    ) {
        let (r, v) = input;
        assert_eq!(expected, r.lookup(&v));
    }
}
