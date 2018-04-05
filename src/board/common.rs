use std::ops::{Add, Sub, Div, Mul};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Player {
    O,
    X,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Position {
    pub(super) row: usize,
    pub(super) col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
        }
    }
}

impl Mul<usize> for Position {
    type Output = Position;

    fn mul(self, rhs: usize) -> Self::Output {
        Position {
            row: self.row * rhs,
            col: self.col * rhs,
        }
    }
}

impl Div<usize> for Position {
    type Output = Position;

    fn div(self, rhs: usize) -> Self::Output {
        Position {
            row: self.row / rhs,
            col: self.col / rhs,
        }
    }
}

impl From<(usize, usize)> for Position {
    fn from(t: (usize, usize)) -> Self {
        Position::new(t.0, t.1)
    }
}

#[macro_export]
macro_rules! pos {
    ($i:expr, $j:expr) => { Position::new($i, $j) };
}

pub trait Board {
    fn winner(&self) -> Option<Player>;

    fn set_pos(&mut self, pos: Position, player: Option<Player>);

    fn get_pos(&self, pos: Position) -> Option<Player>;

    fn get_moves<'a>(&'a self) -> Box<Iterator<Item=Position> + 'a>;

    fn finished(&self) -> bool;
}

pub(super) const VALUES: [i8; 9] = [4, 9, 2, 3, 5, 7, 8, 1, 6];

pub(super) fn pos_to_index(pos: &Position) -> usize {
    pos.row * 3 + pos.col
}

pub(super) fn index_to_pos(index: usize) -> Position {
    pos!(index / 3, index % 3)
}

pub(super) fn player_to_sign(player: &Player) -> i8 {
    match *player {
        Player::O => 1,
        Player::X => -1
    }
}

pub(super) fn sign_to_player(sign: i8) -> Option<Player> {
    match sign {
        x if x > 0 => Some(Player::O),
        x if x < 0 => Some(Player::X),
        _ => None
    }
}

pub(super) fn top_left_pos(pos: &Position) -> Position {
    pos!(pos.row - pos.row % 3, pos.col - pos.col % 3)
}

pub(super) fn get_winning_player_at(values: &[i8], indexes: &[usize]) -> Option<Player> {
    let sum: i8 = indexes
        .iter()
        .map(|i| values[*i])
        .sum();

    let expected: i8 = VALUES.iter().take(3).sum();

    let v = if sum.abs() == expected { sum.signum() } else { 0 };

    sign_to_player(v)
}

#[cfg(test)]
pub(super) mod test_utils {
    use super::*;
    use std::collections::HashSet;
    use board::local_board::LocalBoard;

    pub(in board) fn assert_positions(moves: &HashSet<Position>, expected: &HashSet<Position>) {
        assert_eq!(moves, expected);
    }

    pub(in board) fn generate_pos(max_i: usize, max_j: usize) -> HashSet<Position> {
        iproduct!(0..max_i, 0..max_j).map(From::from).collect()
    }

    pub(in board) fn load_board_from_str(s: &str, board: &mut LocalBoard) {
        for (i, c) in s.chars().enumerate() {
            let pos = index_to_pos(i);
            match c {
                'X' => board.set_pos(pos, Some(Player::X)),
                'O' => board.set_pos(pos, Some(Player::O)),
                _ => board.set_pos(pos, None),
            }
        }
    }

    pub(in board) fn set_and_remove<B>(p: Position, pl: Option<Player>, board: &mut B, pos: &mut HashSet<Position>) where B : Board {
        pos.remove(&p);
        board.set_pos(p, pl);
    }

    #[test]
    fn test_generate_pos() {
        let v = generate_pos(4, 5);

        for i in 0..4 {
            for j in 0..5 {
                assert!(v.contains(&pos!(i, j)));
            }
        }
    }

    #[test]
    fn test_sign_to_player() {
        for i in -10..10 {
            match i {
                x if x > 0 => assert_eq!(sign_to_player(x), Some(Player::O)),
                x if x < 0 => assert_eq!(sign_to_player(x), Some(Player::X)),
                _          => assert_eq!(sign_to_player(i), None),
            }
        }
    }
}