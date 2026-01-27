use crate::{board::Board, piece::PieceType};

// 0..2 are Pawn attack directions (Remainder: every move must be a capture under solo-chess rule);
// 0..4 are Bishop move directions;
// 4.. are Rook move directions.
// Combine together they cover Queen and King directions.
const SLIDER_MOVE: &[(i32, i32)] = &[
    (-1, 1),
    (-1, -1),
    (1, -1),
    (1, 1),
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
];

const KNIGHT_MOVE: &[(i32, i32)] = &[
    (2, 1),
    (-2, 1),
    (2, -1),
    (-2, -1),
    (1, 2),
    (-1, 2),
    (1, -2),
    (-1, -2),
];

pub fn solo_chess_solver(board: &mut Board) -> bool {
    fn dfs(board: &mut Board) {

    }
    // dfs(&mut board);
    true
}

fn list_capture_pairs(board: &Board) -> Vec<(usize, usize, usize, usize)> {
    let all_pieces = board.pieces();
    let mut capture_pairs = vec![];

    for (r, c, &p) in all_pieces {
        if p.move_left() == 0 {
            continue;
        }
        let move_rules = match p.piece_type {
            PieceType::King | PieceType::Queen => &SLIDER_MOVE,
            PieceType::Bishop => &SLIDER_MOVE[..4],
            PieceType::Rook => &SLIDER_MOVE[4..],
            PieceType::Knight => &KNIGHT_MOVE,
            PieceType::Pawn => &SLIDER_MOVE[..2],
        };
        let tmp = get_capturable_cells(&board, r, c, move_rules);
        capture_pairs.extend_from_slice(&tmp);
    }

    capture_pairs
}

// Note: Base on the game rule, if King present, it must be the last piece,
// hence, no other piece can capture it at all time.
fn is_non_king_occupied(board: &Board, r: usize, c: usize) -> bool {
    !board.is_empty(r, c) && board.get_cell(r, c).map(|p| p.piece_type) != Some(PieceType::King)
}

fn get_capturable_cells(
    board: &Board,
    r: usize,
    c: usize,
    move_rules: &[(i32, i32)],
) -> Vec<(usize, usize, usize, usize)> {
    let mut can_capture_cell = vec![];
    let is_king = board.get_cell(r, c).map(|p| p.piece_type) == Some(PieceType::King);
    let is_pawn = board.get_cell(r, c).map(|p| p.piece_type) == Some(PieceType::Pawn);

    for (dr, dc) in move_rules {
        let mut cr = r as i32;
        let mut cc = c as i32;
        loop {
            cr += dr;
            cc += dc;
            if cr < 0 || cr >= 8 || cc < 0 || cc >= 8 {
                break;
            }
            let ur = cr as usize;
            let uc = cc as usize;
            if is_non_king_occupied(board, ur, uc) {
                can_capture_cell.push((r, c, ur, uc));
                break;
            }
            // King and Pawn can only move one space.
            if is_king || is_pawn {
                break;
            }
        }
    }
    can_capture_cell
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::piece::Piece;

    #[test]
    fn unable_to_capture_piece_if_no_move_left() {
        let mut board = Board::new();

        let queen = Piece::new(PieceType::Queen);
        board.set_cell(4, 4, queen);
        board.set_cell(3, 3, Piece::new(PieceType::Rook));

        board.move_piece(4, 4, 0, 0);
        board.move_piece(0, 0, 4, 4);

        let capture_pairs = list_capture_pairs(&board);
        assert!(capture_pairs.is_empty());
    }

    #[test]
    fn get_all_capture_pairs_for_knight() {
        let mut board = Board::new();
        // No capturable pieces
        board.set_cell(4, 4, Piece::new(PieceType::Knight));
        let capture_pairs = list_capture_pairs(&board);
        assert!(capture_pairs.is_empty());

        // Pieces can be captured in all direction of the Knight, but one of them is the King.
        board.set_cell(5, 6, Piece::new(PieceType::Pawn));
        board.set_cell(5, 2, Piece::new(PieceType::Pawn));
        board.set_cell(3, 6, Piece::new(PieceType::Pawn));
        board.set_cell(3, 2, Piece::new(PieceType::Pawn));
        board.set_cell(2, 5, Piece::new(PieceType::Pawn));
        board.set_cell(6, 5, Piece::new(PieceType::Pawn));
        board.set_cell(2, 3, Piece::new(PieceType::Pawn));
        board.set_cell(6, 3, Piece::new(PieceType::King));
        let capture_pairs = list_capture_pairs(&board);
        assert_vec_eq_unordered(
            &vec![
                (4, 4, 5, 6),
                (4, 4, 5, 2),
                (4, 4, 3, 6),
                (4, 4, 3, 2),
                (4, 4, 2, 5),
                (4, 4, 6, 5),
                (4, 4, 2, 3),
                // These are from the Pawns
                (6, 5, 5, 6),
                (3, 2, 2, 3),
                (3, 6, 2, 5),
                // This is from the King
                (6, 3, 5, 2),
            ],
            &capture_pairs,
        );
    }

    #[test]
    fn get_all_capture_paris_for_queen_and_king() {
        let mut board = Board::new();

        // No capturable pieces
        board.set_cell(5, 3, Piece::new(PieceType::Queen));
        let capture_pairs = list_capture_pairs(&board);
        assert!(capture_pairs.is_empty());

        // Two pawns are aligned on one of the queen’s lines of attack. It should capture only the closest one.
        board.set_cell(5, 0, Piece::new(PieceType::Pawn));
        board.set_cell(5, 1, Piece::new(PieceType::Pawn));
        let capture_pairs = list_capture_pairs(&board);
        assert_eq!(vec![(5, 3, 5, 1)], capture_pairs);

        /*
        Add more pawns that are attackable in all queen directions.
        Also include a king, which should be excluded, and a few pieces that are not attackable from the queen’s position.
        . P . P . . . .
        . . . . . . . P
        P . . . P . . .
        . . . . . . . .
        . . . . . . . .
        P P . Q . . . P
        . . . P . . . .
        . P . . . K . .
         */
        board.set_cell(0, 3, Piece::new(PieceType::Pawn));
        board.set_cell(1, 7, Piece::new(PieceType::Pawn));
        board.set_cell(2, 0, Piece::new(PieceType::Pawn));
        board.set_cell(5, 7, Piece::new(PieceType::Pawn));
        board.set_cell(6, 3, Piece::new(PieceType::Pawn));
        board.set_cell(7, 1, Piece::new(PieceType::Pawn));
        board.set_cell(7, 5, Piece::new(PieceType::King));
        // Not attackable from the queen’s position.
        board.set_cell(2, 4, Piece::new(PieceType::Pawn));
        board.set_cell(0, 1, Piece::new(PieceType::Pawn));

        let capture_pairs = list_capture_pairs(&board);
        assert_vec_eq_unordered(
            &vec![
                (5, 3, 0, 3),
                (5, 3, 1, 7),
                (5, 3, 2, 0),
                (5, 3, 5, 7),
                (5, 3, 6, 3),
                (5, 3, 7, 1),
                (5, 3, 5, 1),
            ],
            &capture_pairs,
        );

        // Replace the queen with a king. Only adjacent pieces should be capturable.
        board.set_cell(5, 3, Piece::new(PieceType::King));
        let capture_pairs = list_capture_pairs(&board);
        assert_vec_eq_unordered(&vec![(5, 3, 6, 3)], &capture_pairs);
    }

    #[test]
    fn get_all_capture_pairs_for_pawn() {
        let mut board = Board::new();

        // No capturable pieces
        board.set_cell(4, 4, Piece::new(PieceType::Pawn));
        let capture_pairs = list_capture_pairs(&board);
        assert!(capture_pairs.is_empty());

        // Piece on the left diagonal
        board.set_cell(3, 3, Piece::new(PieceType::Pawn));
        let capture_pairs = list_capture_pairs(&board);
        assert_eq!(vec![(4, 4, 3, 3)], capture_pairs);

        // Another piece on the right diagonal
        board.set_cell(3, 5, Piece::new(PieceType::Pawn));
        let capture_pairs = list_capture_pairs(&board);
        assert_vec_eq_unordered(&vec![(4, 4, 3, 3), (4, 4, 3, 5)], &capture_pairs);
    }

    #[test]
    fn test_is_non_king_occupied() {
        let mut board = Board::new();
        board.set_cell(4, 4, Piece::new(PieceType::King));

        assert!(!is_non_king_occupied(&board, 4, 4));
    }

    fn assert_vec_eq_unordered<T>(a: &[T], b: &[T])
    where
        T: Eq + std::hash::Hash + std::fmt::Debug,
    {
        use std::collections::HashMap;
        let mut ca: HashMap<&T, usize> = HashMap::new();
        let mut cb: HashMap<&T, usize> = HashMap::new();
        for x in a {
            *ca.entry(x).or_insert(0) += 1;
        }
        for x in b {
            *cb.entry(x).or_insert(0) += 1;
        }
        assert_eq!(ca, cb);
    }
}
