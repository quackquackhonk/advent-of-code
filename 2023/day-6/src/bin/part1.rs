use day_6::part1::process;


fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input1");
    let result = process(file)?;

    println!("{:?}", result);
    Ok(())
}
