use board::common;

pub struct LocalBoard {
    board: Vec<i8>
}

impl LocalBoard {
    pub fn new() -> Self {
        LocalBoard {
            board: vec![0; 9]
        }
    }
}

impl common::Board for LocalBoard {
    fn winner(&self) -> Option<common::Player> {
        let values = &self.board;

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
        let index = common::pos_to_index(&pos);

        self.board[index] = match player {
            None => 0,
            Some(p) => common::VALUES[index] * common::player_to_sign(&p)
        }
    }

    fn get_pos(&self, pos: common::Position) -> Option<common::Player> {
        common::sign_to_player(self.board[common::pos_to_index(&pos)])
    }

    fn get_moves<'a>(&'a self) -> Box<Iterator<Item=common::Position> + 'a> {
        let indexes: Vec<_> = (0..9).filter(|&i| self.board[i] == 0).collect();

        Box::new(indexes.into_iter().map(common::index_to_pos))
    }

    fn finished(&self) -> bool {
        self.board.iter().all(|v| *v > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use board::common::*;
    use board::common::test_utils::*;
    use board::common::Board;

    #[test]
    fn test_create() {
        let b = LocalBoard::new();

        assert_eq!(b.board.capacity(), 9);
    }

    #[test]
    fn test_get_set() {
        let mut b = LocalBoard::new();
        let pos = pos!(2, 1);

        assert_eq!(b.get_pos(pos), None);

        b.set_pos(pos, Some(Player::X));

        assert_eq!(b.get_pos(pos), Some(Player::X));

        b.set_pos(pos, None);

        assert_eq!(b.get_pos(pos), None);
    }

    #[test]
    fn test_load_board_from_str() {
        let mut b = LocalBoard::new();

        load_board_from_str("XO OXO  O", &mut b);

        assert_eq!(b.get_pos(pos!(0, 0)), Some(Player::X));
        assert_eq!(b.get_pos(pos!(0, 1)), Some(Player::O));
        assert_eq!(b.get_pos(pos!(0, 2)), None);
        assert_eq!(b.get_pos(pos!(1, 0)), Some(Player::O));
        assert_eq!(b.get_pos(pos!(1, 1)), Some(Player::X));
        assert_eq!(b.get_pos(pos!(1, 2)), Some(Player::O));
        assert_eq!(b.get_pos(pos!(2, 0)), None);
        assert_eq!(b.get_pos(pos!(2, 1)), None);
        assert_eq!(b.get_pos(pos!(2, 2)), Some(Player::O));
    }

    #[test]
    fn test_winner_horizontal() {
        let mut board = LocalBoard::new();

        load_board_from_str("XXX      ", &mut board);
        assert_eq!(board.winner(), Some(Player::X));

        load_board_from_str("   OOO   ", &mut board);
        assert_eq!(board.winner(), Some(Player::O));

        load_board_from_str("      XXX", &mut board);
        assert_eq!(board.winner(), Some(Player::X));
    }

    #[test]
    fn test_winner_vertical() {
        let mut board = LocalBoard::new();

        load_board_from_str("X  X  X  ", &mut board);
        assert_eq!(board.winner(), Some(Player::X));

        load_board_from_str(" O  O  O ", &mut board);
        assert_eq!(board.winner(), Some(Player::O));

        load_board_from_str("  X  X  X", &mut board);
        assert_eq!(board.winner(), Some(Player::X));
    }

    #[test]
    fn test_winner_diagonal() {
        let mut board = LocalBoard::new();

        load_board_from_str("X   X   X", &mut board);
        assert_eq!(board.winner(), Some(Player::X));

        load_board_from_str("  O O O  ", &mut board);
        assert_eq!(board.winner(), Some(Player::O));
    }

    #[test]
    fn test_moves() {
        let mut board = LocalBoard::new();

        let mut hs = generate_pos(3, 3);

        assert_positions(&board.get_moves().collect(), &hs);

        set_and_remove(pos!(1, 1), Some(Player::X), &mut board, &mut hs);

        assert_positions(&board.get_moves().collect(), &hs);

        set_and_remove(pos!(0, 1), Some(Player::O), &mut board, &mut hs);

        assert_positions(&board.get_moves().collect(), &hs);
    }
}
