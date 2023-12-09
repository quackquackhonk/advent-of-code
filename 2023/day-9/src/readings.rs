use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub struct Reading(pub Vec<isize>);

impl Reading {
    pub fn extrapolate(&self) -> isize {
        let prev_deriv = self.make_diffs()
            .into_iter()
            .rev()
            // .tuple_windows()
            .fold(0, |acc, diff| match diff.last() {
                Some(d) => acc + *d,
                None => unreachable!(),
            });

        match self.0.last() {
            Some(l) => l + prev_deriv,
            None => unreachable!(),
        }
    }

    pub fn make_diffs(&self) -> Vec<Vec<isize>> {
        let mut curr = self.0.clone();
        let mut differences = vec![];

        loop {
            let diff = curr
                .iter()
                .tuple_windows()
                .map(|(fst, snd)| snd - fst)
                .collect_vec();

            curr = diff;

            if curr.iter().all(|n| *n == 0) {
                break;
            }

            differences.push(curr.clone());
        }

        differences
    }
}
