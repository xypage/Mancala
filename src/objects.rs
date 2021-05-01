use std::fmt;

#[derive(Debug)]
pub struct Board {
    holes: [Hole; 14],
    current_player: Side,
    other_player: Side
}

// Allows you to print a board without the debugging format, you still can but it looks better this way
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}'s turn\n   {:>2?} {:>2?} {:>2?} {:>2?} {:>2?} {:>2?}\n{:>2?}{:>21?}\n   {:>2?} {:>2?} {:>2?} {:>2?} {:>2?} {:>2?}",
        self.current_player,
        self.holes[5].val(), self.holes[4].val(), self.holes[3].val(), self.holes[2].val(), self.holes[1].val(), self.holes[0].val(),
        self.holes[6].val(), self.holes[13].val(),
        self.holes[7].val(), self.holes[8].val(), self.holes[9].val(), self.holes[10].val(), self.holes[11].val(), self.holes[12].val())
    }
}

// Board setting up functions
impl Board {
    pub fn default_holes() -> Board {
        Board::build_board([4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4, 0], Side::Player, Side::Opponent)
    }

    pub fn build_board(values: [u32; 14], current_player: Side, other_player: Side) -> Board {
        let mut holes: [Hole; 14] = [
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
        Board {holes, current_player, other_player}
    }

    pub fn copy(&self) -> Board {
        let mut copy_vals: [u32; 14] = [0;14];
        for i in 0..14 {
            match &self.holes[i] {
                Hole::Row(val, _s) => copy_vals[i] = *val,
                Hole::End(val, _s) => copy_vals[i] = *val
            }
        }

        let current_player = match &self.current_player {
            Side::Player => Side::Player,
            Side::Opponent => Side::Opponent,
            Side::Named(name) => Side::Named(name.clone())
        };

        let other_player = match &self.other_player {
            Side::Player => Side::Player,
            Side::Opponent => Side::Opponent,
            Side::Named(name) => Side::Named(name.clone())
        };

        Board::build_board(copy_vals, current_player, other_player)
    }
}

// Board manipulation functions
impl Board {
    // pub fn play(mut &self, index: u32) -> Board {
        
    // }

    pub fn change_current(&mut self, current_player: Side) {
        self.current_player = current_player;
    }

    pub fn change_other(&mut self, other_player: Side) {
        self.other_player = other_player;
    }

    pub fn end_turn(&mut self) {
        std::mem::swap(&mut self.current_player, &mut self.other_player);
    }
}

#[derive(Debug)]
pub enum Side {
    Player,
    Opponent,
    Named(String)
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Side::Player => write!(f, "Player"),
            Side::Opponent => write!(f, "Opponent"),
            Side::Named(name) => write!(f, "{}", name)
        }
    }
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