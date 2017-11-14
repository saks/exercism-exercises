pub struct ChessPosition {
    x: i8,
    y: i8,
}

impl ChessPosition {
    pub fn new(x: i8, y: i8) -> Result<Self, String> {
        if x < 0 || y < 0 || x > 7 || y > 7 {
            return Err(String::from("x and y must be >= 0"));
        }

        Ok(Self { x, y })
    }
}

pub struct Queen {
    position: ChessPosition,
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(self, queen: &Queen) -> bool {
        let x1 = self.position.x;
        let y1 = self.position.y;

        let x2 = queen.position.x;
        let y2 = queen.position.y;

        x1 == x2 || y1 == y2 || (x1 - x2).abs() == (y1 - y2).abs()
    }
}
