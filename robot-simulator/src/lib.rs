#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: isize,
    y: isize,
    d: Direction,
}

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot { x, y, d }
    }

    pub fn turn_right(self) -> Self {
        Robot {
            x: self.x,
            y: self.y,
            d: match self.d {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
        }
    }

    pub fn turn_left(self) -> Self {
        Robot {
            x: self.x,
            y: self.y,
            d: match self.d {
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
            },
        }
    }

    pub fn advance(self) -> Self {
        Robot {
            x: match self.d {
                Direction::East => self.x + 1,
                Direction::West => self.x - 1,
                _ => self.x,
            },
            y: match self.d {
                Direction::North => self.y + 1,
                Direction::South => self.y - 1,
                _ => self.y,
            },
            d: self.d,
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;

        for command in instructions.chars() {
            match command {
                'L' => robot = robot.turn_left(),
                'R' => robot = robot.turn_right(),
                'A' => robot = robot.advance(),
                _ => panic!("Illegal command"),
            }
        }
        robot
    }

    pub fn position(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
