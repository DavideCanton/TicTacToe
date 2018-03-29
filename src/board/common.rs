use std::ops::Sub;
use std::ops::Div;

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

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position {
            row: self.row - rhs.row,
            col: self.col - rhs.col,
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
    Position {
        row: index / 3,
        col: index % 3,
    }
}

pub(super) fn player_to_sign(player: &Player) -> i8 {
    match *player {
        Player::O => 1,
        Player::X => -1
    }
}

pub(super) fn sign_to_player(sign: i8) -> Option<Player> {
    match sign {
        0 => None,
        x if x > 0 => Some(Player::O),
        x if x < 0 => Some(Player::X),
        _ => panic!()
    }
}

pub(super) fn top_left_pos(pos: &Position) -> Position {
    Position {
        row: pos.row - pos.row % 3,
        col: pos.col - pos.col % 3,
    }
}

pub(super) fn get_winning_player_at(values: &[i8], indexes: &[usize]) -> Option<Player> {
    let sum = indexes
        .iter()
        .map(|i| values[*i])
        .sum();

    match sum {
        15 => Some(Player::O),
        -15 => Some(Player::X),
        _ => None
    }
}

#[cfg(test)]
pub(super) mod test_utils {
    use super::*;
    use std::collections::HashSet;
    use board::local_board::LocalBoard;

    pub(in board) fn convert_vec(v: Vec<(usize, usize)>) -> HashSet<Position> {
        v.into_iter().map(From::from).collect()
    }

    pub(in board) fn assert_positions(moves: &HashSet<Position>, expected: &HashSet<Position>) {
        assert_eq!(moves, expected);
    }

    pub(in board) fn generate_pos(max_i: usize, max_j: usize) -> Vec<(usize, usize)> {
        let mut vec = Vec::with_capacity(max_i * max_j);

        for i in 0..max_i {
            for j in 0..max_j {
                vec.push((i, j));
            }
        }

        vec
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
}