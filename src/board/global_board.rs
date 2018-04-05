use board::local_board;
use board::common;

pub struct GlobalBoard {
    values: Vec<i8>,
    boards: Vec<local_board::LocalBoard>,
    next_board: Option<usize>,
}

impl GlobalBoard {
    pub fn new() -> Self {
        GlobalBoard {
            values: vec![0; 9],
            boards: (0..9).map(|_| local_board::LocalBoard::new()).collect(),
            next_board: None,
        }
    }
}

impl common::Board for GlobalBoard {
    fn winner(&self) -> Option<common::Player> {
        let values = &self.values;

        common::get_winning_player_at(values, &[0, 1, 2])
            .or_else(|| common::get_winning_player_at(values, &[3, 4, 5]))
            .or_else(|| common::get_winning_player_at(values, &[6, 7, 8]))
            .or_else(|| common::get_winning_player_at(values, &[0, 3, 6]))
            .or_else(|| common::get_winning_player_at(values, &[1, 4, 7]))
            .or_else(|| common::get_winning_player_at(values, &[2, 5, 8]))
            .or_else(|| common::get_winning_player_at(values, &[0, 4, 8]))
            .or_else(|| common::get_winning_player_at(values, &[2, 4, 6]))
    }

    fn set_pos(&mut self, pos: common::Position, player: Option<common::Player>) {
        let corner = common::top_left_pos(&pos);
        let offset = pos - corner;
        let board_pos = corner / 3;
        let index = common::pos_to_index(&board_pos);
        let board = &mut self.boards[index];

        board.set_pos(offset, player);

        match board.winner() {
            None => self.next_board = Some(index),
            Some(player) => {
                self.values[index] = common::VALUES[index] * common::player_to_sign(&player);
                self.next_board = None;
            }
        }
    }

    fn get_pos(&self, pos: common::Position) -> Option<common::Player> {
        let corner = common::top_left_pos(&pos);

        let board_pos = corner / 3;
        let index = common::pos_to_index(&board_pos);

        let value = self.values[index];

        common::sign_to_player(value).or_else(|| {
            let board = &self.boards[index];
            let offset = pos - corner;
            board.get_pos(offset)
        })
    }

    fn get_moves<'a>(&'a self) -> Box<Iterator<Item=common::Position> + 'a> {
        match self.next_board {
            Some(index) => normalize_moves(self.boards[index].get_moves(), index),
            None => Box::new(self.boards
                                 .iter()
                                 .zip(&self.values)
                                 .enumerate()
                                 .filter(|&(_, (_, &j))| common::sign_to_player(j).is_none())
                                 .flat_map(|(i, (b, _))| normalize_moves(b.get_moves(), i)))
        }
    }

    fn finished(&self) -> bool {
        self.values.iter().all(|&v| v > 0)
    }
}

fn normalize_moves<'a>(moves: Box<Iterator<Item=common::Position> + 'a>, i: usize) -> Box<Iterator<Item=common::Position> + 'a> {
    let pos = common::index_to_pos(i) * 3;
    Box::new(moves.map(move |m| m + pos))
}

#[cfg(test)]
mod tests {
    use super::*;
    use board::common::*;
    use board::common::test_utils::*;
    use std::collections::HashSet;

    #[test]
    fn test_create() {
        let b = GlobalBoard::new();

        assert_eq!(b.boards.len(), 9);
        assert_eq!(b.values.len(), 9);

        assert!(b.values.iter().all(|&v| sign_to_player(v).is_none()));
        assert!(b.boards.iter().all(|b| b.winner().is_none()));

        assert_eq!(b.next_board, None);
    }

    #[test]
    fn test_get_set() {
        let mut b = GlobalBoard::new();
        let pos = Position::new(2, 1);

        assert_eq!(b.get_pos(pos), None);

        b.set_pos(pos, Some(Player::X));

        assert_eq!(b.get_pos(pos), Some(Player::X));

        b.set_pos(pos, None);

        assert_eq!(b.get_pos(pos), None);
    }

    #[test]
    fn test_winner_horizontal() {
        let mut board = GlobalBoard::new();

        for i in 0..9 {
            board.set_pos(pos!(0, i), Some(Player::X));
        }

        for i in 0..9 {
            assert_eq!(board.get_pos(pos!(0, i)), Some(Player::X));
        }

        assert_eq!(board.values[0], VALUES[0] * player_to_sign(&Player::X));
        assert_eq!(board.values[1], VALUES[1] * player_to_sign(&Player::X));
        assert_eq!(board.values[2], VALUES[2] * player_to_sign(&Player::X));

        assert_eq!(board.boards[0].winner(), Some(Player::X));
        assert_eq!(board.boards[1].winner(), Some(Player::X));
        assert_eq!(board.boards[2].winner(), Some(Player::X));

        assert_eq!(board.winner(), Some(Player::X));
    }

    #[test]
    fn test_winner_vertical() {
        let mut board = GlobalBoard::new();

        for i in 0..9 {
            board.set_pos(pos!(i, 1), Some(Player::X));
        }

        for i in 0..9 {
            assert_eq!(board.get_pos(pos!(i, 1)), Some(Player::X));
        }

        for i in vec![0, 3, 6].into_iter() {
            assert_eq!(board.values[i], VALUES[i] * player_to_sign(&Player::X));
            assert_eq!(board.boards[i].winner(), Some(Player::X));
        }

        assert_eq!(board.winner(), Some(Player::X));
    }

    #[test]
    fn test_winner_diagonal_p() {
        let mut board = GlobalBoard::new();

        for i in 0..9 {
            board.set_pos(pos!(i, i), Some(Player::X));
        }

        for i in 0..9 {
            assert_eq!(board.get_pos(pos!(i, i)), Some(Player::X));
        }

        assert_eq!(board.values[0], VALUES[0] * player_to_sign(&Player::X));
        assert_eq!(board.values[4], VALUES[4] * player_to_sign(&Player::X));
        assert_eq!(board.values[8], VALUES[8] * player_to_sign(&Player::X));

        assert_eq!(board.boards[0].winner(), Some(Player::X));
        assert_eq!(board.boards[4].winner(), Some(Player::X));
        assert_eq!(board.boards[8].winner(), Some(Player::X));

        assert_eq!(board.winner(), Some(Player::X));
    }

    #[test]
    fn test_winner_diagonal_s() {
        let mut board = GlobalBoard::new();

        for i in 0..9 {
            board.set_pos(pos!(i, 8 - i), Some(Player::X));
        }

        for i in 0..9 {
            assert_eq!(board.get_pos(pos!(i, 8 - i)), Some(Player::X));
        }

        assert_eq!(board.values[2], VALUES[2] * player_to_sign(&Player::X));
        assert_eq!(board.values[4], VALUES[4] * player_to_sign(&Player::X));
        assert_eq!(board.values[6], VALUES[6] * player_to_sign(&Player::X));

        assert_eq!(board.boards[2].winner(), Some(Player::X));
        assert_eq!(board.boards[4].winner(), Some(Player::X));
        assert_eq!(board.boards[6].winner(), Some(Player::X));

        assert_eq!(board.winner(), Some(Player::X));
    }

    #[test]
    fn test_moves() {
        let mut board = GlobalBoard::new();
        assert_eq!(board.next_board, None);

        let mut pos = generate_pos(9, 9);

        assert_positions(&board.get_moves().collect(), &pos);

        board.set_pos(pos!(1, 1), Some(Player::X));
        pos = vec![pos!(0,0), pos!(0,1), pos!(0,2),
                   pos!(1,0), pos!(1,2),
                   pos!(2,0), pos!(2,1), pos!(2,2)].into_iter().collect();

        assert_eq!(board.next_board, Some(0));
        assert_positions(&board.get_moves().collect(), &pos);

        board.set_pos(pos!(4, 4), Some(Player::O));
        pos = vec![pos!(3,3), pos!(3,4), pos!(3,5),
                   pos!(4,3), pos!(4,5),
                   pos!(5,3), pos!(5,4), pos!(5,5)].into_iter().collect();

        assert_eq!(board.next_board, Some(4));
        assert_positions(&board.get_moves().collect(), &pos);
    }

    #[test]
    fn test_normalize_0() {
        let i = 0;
        let moves: HashSet<Position> = vec![pos!(0,0), pos!(1,2), pos!(2,1)].into_iter().collect();
        let expected: HashSet<Position> = vec![pos!(0,0), pos!(1,2), pos!(2,1)].into_iter().collect();

        assert_positions(&normalize_moves(Box::new(moves.into_iter()), i).collect(), &expected);
    }

    #[test]
    fn test_normalize_2() {
        let i = 2;
        let moves: HashSet<Position> = vec![pos!(0,0), pos!(1,2), pos!(2,1)].into_iter().collect();
        let expected: HashSet<Position> = vec![pos!(0,6), pos!(1,8), pos!(2,7)].into_iter().collect();

        assert_positions(&normalize_moves(Box::new(moves.into_iter()), i).collect(), &expected);
    }

    #[test]
    fn test_normalize_4() {
        let i = 4;
        let moves: HashSet<Position> = vec![pos!(0,0), pos!(1,2), pos!(2,1)].into_iter().collect();
        let expected: HashSet<Position> = vec![pos!(3,3), pos!(4,5), pos!(5,4)].into_iter().collect();

        assert_positions(&normalize_moves(Box::new(moves.into_iter()), i).collect(), &expected);
    }
}
