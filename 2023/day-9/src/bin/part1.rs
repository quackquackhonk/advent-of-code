use day_9::part1::process;
use anyhow::Result;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input1");
    let result = process(file)?;

    println!("{:?}", result);
    Ok(())
}
