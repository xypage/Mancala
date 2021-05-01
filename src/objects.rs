use std::fmt;

#[derive(Debug)]
pub struct Board {
    pub holes: [Hole; 14]
}

// Allows you to print a board without the debugging format, you still can but it looks better this way
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "   {:>2?} {:>2?} {:>2?} {:>2?} {:>2?} {:>2?}\n{:>2?}{:>21?}\n   {:>2?} {:>2?} {:>2?} {:>2?} {:>2?} {:>2?}", 
        self.holes[5].val(), self.holes[4].val(), self.holes[3].val(), self.holes[2].val(), self.holes[1].val(), self.holes[0].val(),
        self.holes[6].val(), self.holes[13].val(),
        self.holes[7].val(), self.holes[8].val(), self.holes[9].val(), self.holes[10].val(), self.holes[11].val(), self.holes[12].val())
    }
}

// Board setting up functions
impl Board {
    pub fn default_holes() -> Board {
        Board::build_holes([4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4, 0])
    }

    pub fn build_holes(values: [u32; 14]) -> Board {
        let holes: [Hole; 14] = [
            Hole::Row(values[0], Side::Player),
            Hole::Row(values[1], Side::Player),
            Hole::Row(values[2], Side::Player),
            Hole::Row(values[3], Side::Player),
            Hole::Row(values[4], Side::Player),
            Hole::Row(values[5], Side::Player),
            Hole::End(values[6], Side::Player),
            Hole::Row(values[7], Side::Opponent),
            Hole::Row(values[8], Side::Opponent),
            Hole::Row(values[9], Side::Opponent),
            Hole::Row(values[10], Side::Opponent),
            Hole::Row(values[11], Side::Opponent),
            Hole::Row(values[12], Side::Opponent),
            Hole::End(values[13], Side::Opponent),
        ];
        Board {holes}
    }
}

#[derive(Debug)]
pub enum Side {
    Player,
    Opponent
}

#[derive(Debug)]
pub enum Hole {
    Row(u32, Side),
    End(u32, Side)
}

impl Hole {
    pub fn val(&self) -> u32 {
        match self {
            Hole::Row(val, _s) => *val,
            Hole::End(val, _s) => *val
        }
    }
}

impl fmt::Display for Hole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Hole::Row(val, s) => write!(f, "Row piece with {} on the {:?}'s side", val, s),
            Hole::End(val, s) => write!(f, "End piece with {} on the {:?}'s side", val, s)
        }
    }
}