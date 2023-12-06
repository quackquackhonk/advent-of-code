use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{digit1, newline};
use nom::multi::separated_list1;
use nom::IResult;

use crate::almanac::{Alamanac, AlamanacRange, ConversionMap};

pub fn parse_almanac(input: &str) -> Alamanac {
    let (_, al) = parse_almanac_inner(input).expect("Parsing the almanac failed!");
    al
}

pub fn parse_almanac_inner(input: &str) -> IResult<&str, Alamanac> {
    let mut chunks = input.split("\n\n");

    // parsing seeds
    let Some(seeds_chunk) = chunks.next() else {
        panic!("blew up trying to grab seed chunk");
    };
    let (seeds_chunk, _) = tag("seeds: ")(seeds_chunk)?;
    let (_, seeds) = separated_list1(tag(" "), digit1)(seeds_chunk.trim_start())?;
    let seeds: Vec<usize> = seeds
        .iter()
        .map(|s| s.parse().expect("This is gonna be a number"))
        .collect();

    // parsing seed to soil
    let (_, seed_to_soil) = parse_conversion_map(chunks.next().expect("We should have this map"))?;
    let (_, soil_to_fertilizer) =
        parse_conversion_map(chunks.next().expect("We should have this map"))?;
    let (_, fertilizer_to_water) =
        parse_conversion_map(chunks.next().expect("We should have this map"))?;
    let (_, water_to_light) =
        parse_conversion_map(chunks.next().expect("We should have this map"))?;
    let (_, light_to_temp) = parse_conversion_map(chunks.next().expect("We should have this map"))?;
    let (_, temp_to_humidity) =
        parse_conversion_map(chunks.next().expect("We should have this map"))?;
    let (_, humidity_to_location) =
        parse_conversion_map(chunks.next().expect("We should have this map"))?;

    let al = Alamanac {
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temp,
        temp_to_humidity,
        humidity_to_location,
    };

    Ok(("", al))
}

fn parse_conversion_map(input: &str) -> IResult<&str, ConversionMap> {
    // first remove the title line
    let (input, _) = take_until("\n")(input)?;
    let (input, _) = tag("\n")(input)?;

    let (input, ranges) = separated_list1(newline, parse_almanac_range)(input)?;
    Ok((input, ConversionMap::new(ranges)))
}

fn parse_almanac_range(input: &str) -> IResult<&str, AlamanacRange> {
    // parse three numbers
    let (input, destination_start) = digit1(input.trim_start())?;
    let (input, source_start) = digit1(input.trim_start())?;
    let (input, range_length) = digit1(input.trim_start())?;

    let destination = destination_start.parse().expect("NUMBER!");
    let source = source_start.parse().expect("NUMBER!");
    let range = range_length.parse().expect("NUMBERRR");
    Ok((input, AlamanacRange::new(destination, source, range)))
}
