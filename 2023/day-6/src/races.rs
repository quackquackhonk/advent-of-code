use itertools::Itertools;

pub type Races = Vec<Race>;

pub struct Race {
    pub time: usize,
    pub dist: usize,
}

impl Race {
    pub fn new(time: usize, dist: usize) -> Self {
        Self { time, dist }
    }

    /// Returns the number of ways you can win this race.
    pub fn ways_to_win(&self) -> usize {
        (0..=self.time)
            .filter_map(|dur| {
                let dist = self.dist_after_push(dur);
                if dist > self.dist {
                    Some(dist)
                } else {
                    None
                }
            })
            .collect_vec()
            .len()
    }

    /// Given the duration that the boat button was pushed, returns the distance traveled.
    pub fn dist_after_push(&self, duration: usize) -> usize {
        let vel = duration;
        let time_left = self.time - duration;
        vel * time_left
    }
}
