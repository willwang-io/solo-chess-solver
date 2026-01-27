use crate::{board::Board, piece::Piece};

// 0..4 are Bishop directions; 4.. are Rook directions. Together combine they cover queen and king directions.
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

pub fn solo_chess_solver(board: &Board) -> bool {
    // fn dfs(board: &mut Board) -> bool {
    //     if board.count_pieces() > 1 {
    //         let capture_pairs = get_all_capture_pairs(board);
    //         for (fr, fc, tr, tc) in capture_pairs {
    //             let from_piece = board.get_cell(fr, fc);
    //             let to_piece = board.get_cell(tr, tc);
    //             board.move_piece(fr, fc, tr, tc);
    //             if dfs(board) {
    //                 return true;
    //             }
    //             board.set_cell(fr, fc, from_piece);
    //             board.set_cell(tr, tc, to_piece);
    //         }
    //         false
    //     } else {
    //         true
    //     }
    // }
    true
}

fn get_all_capture_pairs(board: &Board) -> Vec<(usize, usize, usize, usize)> {
    let all_pieces = board.pieces();

    let mut capture_pairs = vec![];

    for (r, c, &p) in all_pieces {
        // TODO: if p's available move is zero, skip it.
        // let mut cr = r;
        // let mut cc = c;
        match p {
            Piece::King | Piece::Queen => {
                let tmp = get_can_capture_cell(&board, r, c, &SLIDER_MOVE);
                capture_pairs.extend_from_slice(&tmp);
            }
            Piece::Bishop => {
                let tmp = get_can_capture_cell(&board, r, c, &SLIDER_MOVE[..4]);
                capture_pairs.extend_from_slice(&tmp);
            }
            Piece::Rook => {
                let tmp = get_can_capture_cell(&board, r, c, &SLIDER_MOVE[4..]);
                capture_pairs.extend_from_slice(&tmp);
            }
            Piece::Knight => {
                let tmp = get_can_capture_cell(&board, r, c, &KNIGHT_MOVE);
                capture_pairs.extend_from_slice(&tmp);
            }
            Piece::Pawn => {
                if r >= 1 && c >= 1 && is_non_king_occupied(board, r - 1, c - 1) {
                    capture_pairs.push((r, c, r - 1, c - 1));
                }
                if r >= 1 && c + 1 < 8 && is_non_king_occupied(board, r - 1, c + 1) {
                    capture_pairs.push((r, c, r - 1, c + 1));
                }
            }
        };
    }
    capture_pairs
}

// Note: Base on the game rule, if King present, it must be the last piece,
// hence, no other piece can capture it at all time.
fn is_non_king_occupied(board: &Board, r: usize, c: usize) -> bool {
    !board.is_empty(r, c) && board.get_cell(r, c) != Some(Piece::King)
}

fn get_can_capture_cell(
    board: &Board,
    r: usize,
    c: usize,
    move_rules: &[(i32, i32)],
) -> Vec<(usize, usize, usize, usize)> {
    let mut can_capture_cell = vec![];
    let is_king = board.get_cell(r, c) == Some(Piece::King);

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
            // King can only move one space.
            if is_king {
                break;
            }
        }
    }
    can_capture_cell
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solo_chess_solver() {

    }

    #[test]
    fn get_all_capture_pairs_for_knight() {
        let mut board = Board::new();
        // No capturable pieces
        board.set_cell(4, 4, Piece::Knight);
        let capture_pairs = get_all_capture_pairs(&board);
        assert!(capture_pairs.is_empty());

        // Pieces can be captured in all direction of the Knight, but one of them is the King.
        board.set_cell(5, 6, Piece::Pawn);
        board.set_cell(5, 2, Piece::Pawn);
        board.set_cell(3, 6, Piece::Pawn);
        board.set_cell(3, 2, Piece::Pawn);
        board.set_cell(2, 5, Piece::Pawn);
        board.set_cell(6, 5, Piece::Pawn);
        board.set_cell(2, 3, Piece::Pawn);
        board.set_cell(6, 3, Piece::King);
        let capture_pairs = get_all_capture_pairs(&board);
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
        board.set_cell(5, 3, Piece::Queen);
        let capture_pairs = get_all_capture_pairs(&board);
        assert!(capture_pairs.is_empty());

        // Two pawns are aligned on one of the queen’s lines of attack. It should capture only the closest one.
        board.set_cell(5, 0, Piece::Pawn);
        board.set_cell(5, 1, Piece::Pawn);
        let capture_pairs = get_all_capture_pairs(&board);
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
        board.set_cell(0, 3, Piece::Pawn);
        board.set_cell(1, 7, Piece::Pawn);
        board.set_cell(2, 0, Piece::Pawn);
        board.set_cell(5, 7, Piece::Pawn);
        board.set_cell(6, 3, Piece::Pawn);
        board.set_cell(7, 1, Piece::Pawn);
        board.set_cell(7, 5, Piece::King);
        // Not attackable from the queen’s position.
        board.set_cell(2, 4, Piece::Pawn);
        board.set_cell(0, 1, Piece::Pawn);

        let capture_pairs = get_all_capture_pairs(&board);
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
        board.set_cell(5, 3, Piece::King);
        let capture_pairs = get_all_capture_pairs(&board);
        assert_vec_eq_unordered(&vec![(5, 3, 6, 3)], &capture_pairs);
    }

    #[test]
    fn get_all_capture_pairs_for_pawn() {
        let mut board = Board::new();

        // No capturable pieces
        board.set_cell(4, 4, Piece::Pawn);
        let capture_pairs = get_all_capture_pairs(&board);
        assert!(capture_pairs.is_empty());

        // Piece on the left diagonal
        board.set_cell(3, 3, Piece::Pawn);
        let capture_pairs = get_all_capture_pairs(&board);
        assert_eq!(vec![(4, 4, 3, 3)], capture_pairs);

        // Another piece on the right diagonal
        board.set_cell(3, 5, Piece::Pawn);
        let capture_pairs = get_all_capture_pairs(&board);
        assert_vec_eq_unordered(&vec![(4, 4, 3, 3), (4, 4, 3, 5)], &capture_pairs);
    }

    #[test]
    fn test_is_non_king_occupied() {
        let mut board = Board::new();
        board.set_cell(4, 4, Piece::King);

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
