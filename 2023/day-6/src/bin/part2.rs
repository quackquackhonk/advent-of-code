use day_6::part2::process;
use anyhow::Result;

fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input2");
    let result = process(file)?;

    println!("{:?}", result);
    Ok(())
}
