use std::collections::HashMap;

use crate::part2::Index;

#[derive(Debug, PartialEq)]
pub struct Gear(Vec<usize>);

impl Gear {
    pub fn valid(&self) -> bool {
        self.0.len() == 2
    }
    
    pub fn gear_ratio(&self) -> usize {
        self.0.iter().fold(1, |acc, gear| gear * acc)
    }

    pub fn from_center_and_nums(center: &Index, nums: &HashMap<Index, usize>) -> Self {
        let mut gears: Vec<usize> = [checked_get(nums, *center, (-1, -1)),
             checked_get(nums, *center, (-1, 0)),
             checked_get(nums, *center, (-1, 1)),
             checked_get(nums, *center, (0, -1)),
             checked_get(nums, *center, (0, 1)),
             checked_get(nums, *center, (1, -1)),
             checked_get(nums, *center, (1, 0)),
             checked_get(nums, *center, (1, 1))].iter().flat_map(|x| *x).collect();
        gears.sort();
        gears.dedup();
        Gear(gears)
    }
}

fn checked_get(map: &HashMap<Index, usize>, center: Index, offset: (isize, isize)) -> Option<usize> {
    let line_idx = center.0.checked_add_signed(offset.0)?;
    let char_idx = center.1.checked_add_signed(offset.1)?;
    map.get(&(line_idx, char_idx)).copied()
}
