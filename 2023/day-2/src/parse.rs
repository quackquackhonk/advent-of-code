use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, space0},
    multi::{separated_list0, separated_list1},
    sequence::separated_pair,
    IResult,
};

use crate::game::{BlockDraw, Draw, Game};

pub fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id_str) = digit1(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, draws) = separated_list0(tag(";"), parse_draws)(input)?;

    let id = id_str
        .parse()
        .expect("This should always be a positive number");
    Ok((input, Game::new(id, draws)))
}

pub fn parse_draws(input: &str) -> IResult<&str, Draw> {
    let input = input.trim();
    let (input, block_draws) = separated_list1(tag(","), parse_block_draw)(input)?;
    Ok((input, block_draws))
}

pub fn parse_block_draw(input: &str) -> IResult<&str, BlockDraw> {
    let input = input.trim();
    let (input, (count_str, color)) = separated_pair(digit1, space0, alpha1)(input)?;

    let count: u64 = count_str
        .parse()
        .expect("This should always be a positiVe number");
    let draw = match color {
        "red" => BlockDraw::RedDraw(count),
        "green" => BlockDraw::GreenDraw(count),
        "blue" => BlockDraw::BlueDraw(count),
        _ => unreachable!(),
    };

    Ok((input, draw))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::game::{BlockDraw, Game};
    use crate::parse::parse_game;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", Game::new(
        1,
        vec![
            vec![BlockDraw::BlueDraw(3), BlockDraw::RedDraw(4)],
            vec![
                BlockDraw::RedDraw(1),
                BlockDraw::GreenDraw(2),
                BlockDraw::BlueDraw(6),
            ],
            vec![BlockDraw::GreenDraw(2)],
        ],
    ))]
    pub fn parse_game_test(#[case] input: &str, #[case] expected: Game) {
        let (_, game) = parse_game(input).unwrap();
        assert_eq!(expected, game);
    }
}
