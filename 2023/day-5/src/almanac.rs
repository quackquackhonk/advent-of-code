use std::ops::Range;

use itertools::Itertools;

#[derive(Debug)]
pub struct Alamanac {
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
pub enum AlamanacType {
    Seed(usize),
    Soil(usize),
    Fertilizer(usize),
    Water(usize),
    Light(usize),
    Temperature(usize),
    Humidity(usize),
    Location(usize),
}

impl AlamanacType {
    pub fn value(&self) -> usize {
        match self {
            AlamanacType::Seed(x) => *x,
            AlamanacType::Soil(x) => *x,
            AlamanacType::Fertilizer(x) => *x,
            AlamanacType::Water(x) => *x,
            AlamanacType::Light(x) => *x,
            AlamanacType::Temperature(x) => *x,
            AlamanacType::Humidity(x) => *x,
            AlamanacType::Location(x) => *x,
        }
    }
}

impl Alamanac {
    pub fn lookup(&self, key: &AlamanacType) -> AlamanacType {
        match key {
            AlamanacType::Seed(_) => self.seed_to_soil.lookup(key),
            AlamanacType::Soil(_) => self.soil_to_fertilizer.lookup(key),
            AlamanacType::Fertilizer(_) => self.fertilizer_to_water.lookup(key),
            AlamanacType::Water(_) => self.water_to_light.lookup(key),
            AlamanacType::Light(_) => self.light_to_temp.lookup(key),
            AlamanacType::Temperature(_) => self.temp_to_humidity.lookup(key),
            AlamanacType::Humidity(_) => self.humidity_to_location.lookup(key),
            AlamanacType::Location(_) => *key,
        }
    }

    pub fn all_locations(&self) -> Vec<usize> {
        self.seeds
            .iter()
            .map(|seed| {
                let mut key = AlamanacType::Seed(*seed);
                loop {
                    key = match key {
                        AlamanacType::Location(x) => break x,
                        _ => self.lookup(&key),
                    };
                }
            })
            .collect_vec()
    }
}

#[derive(Debug)]
pub struct ConversionMap {
    pub ranges: Vec<AlamanacRange>,
}

impl ConversionMap {
    pub fn lookup(&self, key: &AlamanacType) -> AlamanacType {
        match key {
            AlamanacType::Seed(x) => AlamanacType::Soil(self.generic_lookup(x)),
            AlamanacType::Soil(x) => AlamanacType::Fertilizer(self.generic_lookup(x)),
            AlamanacType::Fertilizer(x) => AlamanacType::Water(self.generic_lookup(x)),
            AlamanacType::Water(x) => AlamanacType::Light(self.generic_lookup(x)),
            AlamanacType::Light(x) => AlamanacType::Temperature(self.generic_lookup(x)),
            AlamanacType::Temperature(x) => AlamanacType::Humidity(self.generic_lookup(x)),
            AlamanacType::Humidity(x) => AlamanacType::Location(self.generic_lookup(x)),
            AlamanacType::Location(x) => AlamanacType::Location(*x),
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

    pub fn new(ranges: Vec<AlamanacRange>) -> ConversionMap {
        ConversionMap { ranges }
    }
}

#[derive(Debug)]
pub struct AlamanacRange {
    pub source_start: usize,
    pub destination_start: usize,
    pub range_length: usize,
}

impl AlamanacRange {
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
        AlamanacRange,
        AlamanacType::{self, *},
        ConversionMap,
    };

    #[rstest]
    #[case((ConversionMap::new(vec![AlamanacRange::new(50, 98, 2), AlamanacRange::new(52, 50, 48)]), Seed(79)), Soil(81))]
    #[case((ConversionMap::new(vec![AlamanacRange::new(50, 98, 2), AlamanacRange::new(52, 50, 48)]), Seed(14)), Soil(14))]
    #[case((ConversionMap::new(vec![AlamanacRange::new(50, 98, 2), AlamanacRange::new(52, 50, 48)]), Seed(55)), Soil(57))]
    #[case((ConversionMap::new(vec![AlamanacRange::new(50, 98, 2), AlamanacRange::new(52, 50, 48)]), Seed(13)), Soil(13))]
    pub fn test_conversion_map_lookup(
        #[case] input: (ConversionMap, AlamanacType),
        #[case] expected: AlamanacType,
    ) -> anyhow::Result<()> {
        let (cv, seed) = input;
        assert_eq!(expected, cv.lookup(&seed));
        Ok(())
    }

    #[rstest]
    #[case((AlamanacRange::new(52, 50, 48), 79), Some(81))]
    pub fn test_almanac_range_lookup(#[case] input: (AlamanacRange, usize), #[case] expected: Option<usize>) {
        let (r, v) = input;
        assert_eq!(expected, r.lookup(&v));
    }
}
