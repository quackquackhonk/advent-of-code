#[derive(Debug, PartialEq)]
pub struct ScratchCard {
    pub id: usize,
    pub winning: Vec<usize>,
    pub revealed: Vec<usize>,
}

impl ScratchCard {
    pub fn new(id: usize, winning: Vec<usize>, revealed: Vec<usize>) -> Self {
        Self {
            id,
            winning,
            revealed,
        }
    }

    pub fn calculate_points(&self) -> usize {
        let num_matches = self.num_matches();

        if num_matches > 0 {
            // LOL
            2_usize.pow(
                num_matches
                    .checked_sub(1_usize)
                    .unwrap()
                    .try_into()
                    .unwrap(),
            )
        } else {
            0
        }
    }

    pub fn num_matches(&self) -> usize {
        self.revealed.iter().fold(0, |acc, num| {
            if self.winning.contains(num) {
                acc + 1
            } else {
                acc
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::ScratchCard;

    #[rstest]
    #[case(ScratchCard{id: 1, winning: vec![41, 48, 83, 86, 17], revealed: vec![83, 86, 6, 31, 17, 9, 48, 53]}, 8)]
    #[case(ScratchCard{id: 2, winning: vec![13, 32, 20, 16, 61], revealed: vec![61, 30, 68, 82, 17, 32, 24, 19]}, 2)]
    #[case(ScratchCard{id: 3, winning: vec![1, 21, 53, 59, 44], revealed: vec![69, 82, 63, 72, 16, 21, 14, 1]}, 2)]
    #[case(ScratchCard{id: 4, winning: vec![41, 92, 73, 84, 69], revealed: vec![59, 84, 76, 51, 58, 5, 54, 83]}, 1)]
    #[case(ScratchCard{id: 5, winning: vec![31, 18, 13, 56, 72], revealed: vec![74, 77, 10, 23, 35, 67, 36, 11]}, 0)]
    pub fn test_calculate_points(#[case] input: ScratchCard, #[case] expected: usize) {
        assert_eq!(input.calculate_points(), expected)
    }
}
