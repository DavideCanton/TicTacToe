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

        let offset_index = common::pos_to_index(&offset);

        match board.winner() {
            None => self.next_board = Some(offset_index),
            Some(player) => {
                self.values[index] = common::VALUES[index] * common::player_to_sign(&player);
                self.next_board = None;
            }
        }
    }

    fn get_pos(&self, pos: common::Position) -> Option<common::Player> {
        let corner = common::top_left_pos(&pos);
        let offset = pos - corner;

        let board_pos = corner / 3;
        let index = common::pos_to_index(&board_pos);

        let board = &self.boards[index];
        let value = self.values[index];

        match common::sign_to_player(value) {
            s @ Some(_) => s,
            None => board.get_pos(offset)
        }
    }

    fn get_moves<'a>(&'a self) -> Box<Iterator<Item=common::Position> + 'a> {
        match self.next_board {
            Some(index) => self.boards[index].get_moves(),
            None => {
                Box::new(self.boards
                    .iter()
                    .zip(&self.values)
                    .filter(|&b| common::sign_to_player(*b.1).is_none())
                    .flat_map(|b| b.0.get_moves()))
            }
        }
    }

    fn finished(&self) -> bool {
        self.values.iter().all(|&v| v > 0)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use board::common::*;
    use board::common::test_utils::*;

    #[test]
    fn test_create() {
        let b = GlobalBoard::new();

        assert_eq!(b.boards.len(), 9);
        assert_eq!(b.values.len(), 9);
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
            board.set_pos(Position::new(0, i), Some(Player::X));
        }

        for i in 0..9 {
            assert_eq!(board.get_pos(Position::new(0, i)), Some(Player::X));
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
            board.set_pos(Position::new(i, 1), Some(Player::X));
        }

        for i in 0..9 {
            assert_eq!(board.get_pos(Position::new(i, 1)), Some(Player::X));
        }

        assert_eq!(board.values[0], VALUES[0] * player_to_sign(&Player::X));
        assert_eq!(board.values[3], VALUES[3] * player_to_sign(&Player::X));
        assert_eq!(board.values[6], VALUES[6] * player_to_sign(&Player::X));

        assert_eq!(board.boards[0].winner(), Some(Player::X));
        assert_eq!(board.boards[3].winner(), Some(Player::X));
        assert_eq!(board.boards[6].winner(), Some(Player::X));

        assert_eq!(board.winner(), Some(Player::X));
    }

    #[test]
    fn test_winner_diagonal_p() {
        let mut board = GlobalBoard::new();

        for i in 0..9 {
            board.set_pos(Position::new(i, i), Some(Player::X));
        }

        for i in 0..9 {
            assert_eq!(board.get_pos(Position::new(i, i)), Some(Player::X));
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
            board.set_pos(Position::new(i, 8 - i), Some(Player::X));
        }

        for i in 0..9 {
            assert_eq!(board.get_pos(Position::new(i, 8 - i)), Some(Player::X));
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

        let mut pos = convert_vec(generate_pos(9, 9));

        assert_positions(&board.get_moves().collect(), &pos);

        board.set_pos(Position::new(1, 1), Some(Player::X));
        pos.remove(&Position::new(1, 1));

        assert_positions(&board.get_moves().collect(), &pos);

        board.set_pos(Position::new(0, 1), Some(Player::O));
        pos.remove(&Position::new(0, 1));

        assert_positions(&board.get_moves().collect(), &pos);
    }
}
