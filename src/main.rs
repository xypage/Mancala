fn main() {
    let mut b = Board {
        holes: Board::default_holes()
    };

    
}

struct Board {
    holes: [Hole; 14]
}

// Board setting up functions
impl Board {
    fn default_holes() -> [Hole; 14] {
        Board::build_holes([4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4, 0])
    }

    fn build_holes(values: [u32; 14]) -> [Hole; 14] {
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
        holes
    }
}

enum Side {
    Player,
    Opponent
}

enum Hole {
    Row(u32, Side),
    End(u32, Side)
}