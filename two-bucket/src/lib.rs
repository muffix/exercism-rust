use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(capacity_1: u8, capacity_2: u8, goal: u8, start_bucket: &Bucket) -> BucketStats {
    let mut seen = HashSet::new();

    let (mut one, mut two) = (0, 0);

    if *start_bucket == Bucket::One {
        seen.insert((0, capacity_2));
        one = capacity_1
    } else {
        seen.insert((capacity_1, 0));
        two = capacity_2
    }

    let mut queue = VecDeque::new();
    queue.push_back((1, (one, two)));

    while let Some((step, levels)) = queue.pop_front() {
        if seen.contains(&levels) {
            continue;
        }

        seen.insert(levels);

        if levels.0 == goal {
            return BucketStats {
                moves: step,
                goal_bucket: Bucket::One,
                other_bucket: levels.1,
            };
        }
        if levels.1 == goal {
            return BucketStats {
                moves: step,
                goal_bucket: Bucket::Two,
                other_bucket: levels.0,
            };
        }

        queue.push_back((step + 1, (levels.0, capacity_2)));
        queue.push_back((step + 1, (capacity_1, levels.1)));
        queue.push_back((step + 1, (levels.0, 0)));
        queue.push_back((step + 1, (0, levels.1)));

        if levels.0 + levels.1 < capacity_1 {
            queue.push_back((step + 1, (levels.0 + levels.1, 0)))
        } else {
            queue.push_back((step + 1, (capacity_1, levels.0 + levels.1 - capacity_1)))
        }

        if levels.0 + levels.1 < capacity_2 {
            queue.push_back((step + 1, (0, levels.0 + levels.1)))
        } else {
            queue.push_back((step + 1, (levels.0 + levels.1 - capacity_2, capacity_2)))
        }
    }

    panic!("No solution");
}
