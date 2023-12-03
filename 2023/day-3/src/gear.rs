use std::collections::HashMap;

use crate::part2::Index;

#[derive(Debug)]
pub struct Gear {
    first: Option<usize>,
    second: Option<usize>,
}

impl Gear {
    pub fn new(first: Option<usize>, second: Option<usize>) -> Self {
        Gear { first, second }
    }

    pub fn valid(&self) -> bool {
        self.first.is_some() && self.second.is_some()
    }

    pub fn gear_ratio(&self) -> usize {
        let Some(f) = self.first else {
            panic!("This gear should be validated before calling gear_ratio!");
        };
        let Some(s) = self.second else {
            panic!("This gear should be validated before calling gear_ratio!");
        };
        s * f
    }

    pub fn check(self, nums: HashMap<Index, usize>, center: Index, offset: (isize, isize)) -> Self {
        let (Some(line_idx), Some(char_idx)) = (
            center.0.checked_add_signed(offset.0),
            center.1.checked_add_signed(offset.1),
        ) else {
            return Gear::new(self.first, self.second)
        };

        let Some(num_at_offset) = nums.get(&(line_idx, char_idx)) else {
            return Gear::new(self.first, self.second)
        };

        if self.first.is_none() {
            Gear::new(Some(num_at_offset), self.second)
        }
        else if self.second.is_none() {
            Gear::new(self.first, Some(num_at_offset))
        } else {
            // both are already filled
        }

        Gear::new(None, None)
    }

    pub fn from_center_and_nums(center: &Index, nums: &HashMap<Index, usize>) -> Self {
        let x = [
            checked_get(nums, *center, (-1, -1)),
            checked_get(nums, *center, (0, -1)),
            checked_get(nums, *center, (1, -1)),
            checked_get(nums, *center, (-1, 0)),
            checked_get(nums, *center, (1, 0)),
            checked_get(nums, *center, (-1, 1)),
            checked_get(nums, *center, (0, 1)),
            checked_get(nums, *center, (1, 1)),
        ];

        Gear::new(None, None)
    }
}

fn checked_get(
    map: &HashMap<Index, usize>,
    center: Index,
    offset: (isize, isize),
) -> Option<usize> {
    let line_idx = center.0.checked_add_signed(offset.0)?;
    let char_idx = center.1.checked_add_signed(offset.1)?;
    map.get(&(line_idx, char_idx)).copied()
}
