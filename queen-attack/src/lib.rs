pub struct ChessPosition {
    x: isize,
    y: isize,
}

impl ChessPosition {
    pub fn new(x: isize, y: isize) -> Result<ChessPosition, String> {
        if x < 0 || x > 7 || y < 0 || y > 7 {
            return Err("Illegal position".to_string());
        }
        Ok(ChessPosition { x, y })
    }
}

pub struct Queen {
    position: ChessPosition,
}

impl Queen {
    pub fn new(pos: ChessPosition) -> Queen {
        Queen { position: pos }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let x_delta = self.position.x - other.position.x;
        let y_delta = self.position.y - other.position.y;
        
        self.position.x == other.position.x
            || self.position.y == other.position.y
            || x_delta.abs() == y_delta.abs()
    }
}
