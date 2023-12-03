use day_3::part2::process;


fn main() -> anyhow::Result<()> {
    let file = include_str!("../../input2");
    let result = process(file)?;

    println!("{}", result);
    Ok(())
}
