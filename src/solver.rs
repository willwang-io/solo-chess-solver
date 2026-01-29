use std::collections::HashSet;

use crate::{board::Board, piece::PieceType, step::Step};

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

pub fn solo_chess_solver(board: &mut Board) -> Vec<Step> {
    let king_required = board.has_king();
    let mut steps = Vec::new();
    let mut dead = HashSet::<Board>::new();

    fn dfs(
        board: &mut Board,
        steps: &mut Vec<Step>,
        dead: &mut HashSet<Board>,
        king_required: bool,
    ) -> bool {
        let p = board.count_pieces();

        if p <= 1 {
            return if king_required { board.single_is_king() } else { p == 1 };
        }

        if board.sum_move_left() < p - 1 {
            return false;
        }

        if dead.contains(board) {
            return false;
        }

        let mut moves = Vec::new();
        list_capture_pairs_into(board, &mut moves);

        if moves.is_empty() {
            dead.insert(*board);
            return false;
        }

        let mut cnt = [0u8; 64];
        for m in moves.iter() {
            let (fr, fc) = m.from;
            cnt[fr * 8 + fc] = cnt[fr * 8 + fc].saturating_add(1);
        }

        moves.sort_by_key(|m| {
            let (fr, fc) = m.from;
            (cnt[fr * 8 + fc], fr * 8 + fc)
        });

        for step in moves.iter().copied() {
            let (fr, fc) = step.from;
            let (tr, tc) = step.to;

            let from_piece = board.get_cell(fr, fc).unwrap();
            let to_piece = board.get_cell(tr, tc).unwrap();

            board.move_piece(fr, fc, tr, tc);
            steps.push(step);

            if dfs(board, steps, dead, king_required) {
                return true;
            }

            steps.pop();
            board.set_cell(fr, fc, from_piece);
            board.set_cell(tr, tc, to_piece);
        }

        dead.insert(*board);
        false
    }

    if dfs(board, &mut steps, &mut dead, king_required) {
        steps
    } else {
        Vec::new()
    }
}

fn list_capture_pairs_into(board: &Board, out: &mut Vec<Step>) {
    out.clear();

    for (r, c, &p) in board.pieces() {
        if p.move_left() == 0 {
            continue;
        }
        let piece_type = p.piece_type;
        let move_rules = match piece_type {
            PieceType::King | PieceType::Queen => &SLIDER_MOVE,
            PieceType::Bishop => &SLIDER_MOVE[..4],
            PieceType::Rook => &SLIDER_MOVE[4..],
            PieceType::Knight => &KNIGHT_MOVE,
            PieceType::Pawn => &SLIDER_MOVE[..2],
        };
        get_capturable_cells_into(board, r, c, piece_type, move_rules, out);
    }
}

fn get_capturable_cells_into(
    board: &Board,
    r: usize,
    c: usize,
    piece_type: PieceType,
    move_rules: &[(i32, i32)],
    out: &mut Vec<Step>
) {
    let is_king = piece_type == PieceType::King;
    let is_pawn = piece_type == PieceType::Pawn;
    let is_knight = piece_type == PieceType::Knight;

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

            if let Some(cell) = board.get_cell(ur, uc) {
                if cell.is_king() {
                    break;
                }
                out.push(Step {
                    from: (r, c),
                    to: (ur, uc),
                    piece_type,
                });
                break;
            }

            // King, Pawn and Knight can only move one time.
            if is_king || is_pawn || is_knight {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test_solo_chess_solver {
    use super::*;
    use crate::piece::Piece;

    macro_rules! board {
        ( $( ($x:expr, $y:expr, $kind:ident) ),* $(,)? ) => {{
            let mut b = Board::new();
            $(
                b.set_cell($x, $y, Piece::new(PieceType::$kind));
            )*
            b
        }};
    }

    macro_rules! steps {
        ( $( ($x1:expr, $y1:expr, $x2:expr, $y2:expr, $kind:ident) ),* $(,)? ) => {
            vec![
                $(
                    Step {
                        from: ($x1, $y1),
                        to: ($x2, $y2),
                        piece_type: PieceType::$kind,
                    }
                ),*
            ]
        };
    }

    #[test]
    fn random_cases_with_a_solution() {
        // There are ten levels from chess.com. So I randomly picked some, mostly from upper levels.

        // Level 7
        let mut board = board![
            (3, 0, Bishop),
            (1, 1, Knight),
            (2, 3, Queen),
            (3, 2, Knight),
            (5, 5, Rook),
            (3, 5, Knight),
            (4, 2, Knight),
            (5, 5, Bishop),
            (6, 1, Rook),
        ];
        let actual = solo_chess_solver(&mut board);
        let expected = steps![
            (3, 2, 1, 1, Knight),
            (3, 5, 2, 3, Knight),
            (5, 5, 1, 1, Bishop),
            (6, 1, 1, 1, Rook),
            (2, 3, 1, 1, Knight),
            (4, 2, 3, 0, Knight),
            (3, 0, 1, 1, Knight),
        ];
        assert_eq!(expected, actual);
    }
}

#[cfg(test)]
mod test_utilities {
    use super::*;
    use crate::piece::Piece;

    fn step(fr: usize, fc: usize, tr: usize, tc: usize, piece_type: PieceType) -> Step {
        Step {
            from: (fr, fc),
            to: (tr, tc),
            piece_type,
        }
    }

    #[test]
    fn unable_to_capture_piece_if_no_move_left() {
        let mut board = Board::new();

        let queen = Piece::new(PieceType::Queen);
        board.set_cell(4, 4, queen);
        board.set_cell(3, 3, Piece::new(PieceType::Rook));

        board.move_piece(4, 4, 0, 0);
        board.move_piece(0, 0, 4, 4);

        let mut capture_pairs = Vec::new();
        list_capture_pairs_into(&board, &mut capture_pairs);
        assert!(capture_pairs.is_empty());
    }

    #[test]
    fn get_all_capture_pairs_for_knight() {
        let mut board = Board::new();
        let mut capture_pairs = Vec::new();

        // No capturable pieces.
        board.set_cell(4, 4, Piece::new(PieceType::Knight));
        list_capture_pairs_into(&board, &mut capture_pairs);
        assert!(capture_pairs.is_empty());

        // Should not capture pieces that need two Knight moves.
        board.set_cell(0, 6, Piece::new(PieceType::Pawn));
        list_capture_pairs_into(&board, &mut capture_pairs);
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

        list_capture_pairs_into(&board, &mut capture_pairs);

        assert_vec_eq_unordered(
            &vec![
                step(4, 4, 5, 6, PieceType::Knight),
                step(4, 4, 5, 2, PieceType::Knight),
                step(4, 4, 3, 6, PieceType::Knight),
                step(4, 4, 3, 2, PieceType::Knight),
                step(4, 4, 2, 5, PieceType::Knight),
                step(4, 4, 6, 5, PieceType::Knight),
                // These are from the Pawns
                step(4, 4, 2, 3, PieceType::Knight),
                step(6, 5, 5, 6, PieceType::Pawn),
                step(3, 2, 2, 3, PieceType::Pawn),
                step(3, 6, 2, 5, PieceType::Pawn),
                // These are from the Pawns
                step(6, 3, 5, 2, PieceType::King),
            ],
            &capture_pairs,
        );
    }

    #[test]
    fn get_all_capture_paris_for_queen_and_king() {
        let mut board = Board::new();
        let mut capture_pairs = Vec::new();

        // No capturable pieces
        board.set_cell(5, 4, Piece::new(PieceType::Queen));
        list_capture_pairs_into(&board, &mut capture_pairs);
        assert!(capture_pairs.is_empty());

        // The check on that direction should stop if met a King. 
        board.set_cell(5, 0, Piece::new(PieceType::Pawn));
        board.set_cell(5, 2, Piece::new(PieceType::King));
        list_capture_pairs_into(&board, &mut capture_pairs);
        assert!(capture_pairs.is_empty());

        // Two pawns are aligned on one of the queen’s lines of attack. It should capture only the closest one.
        board.set_cell(5, 0, Piece::new(PieceType::Pawn));
        board.set_cell(5, 2, Piece::new(PieceType::Pawn));
        list_capture_pairs_into(&board, &mut capture_pairs);
        assert_eq!(vec![step(5, 4, 5, 2, PieceType::Queen)], capture_pairs);

        /*
        Add more pawns that are attackable in all queen directions. Also include a king, which should be excluded.
        . . . . . . . .
        . . . . . . . .
        . . . . . . . P
        . . P . . . . .
        . . . . P . . .
        P . P . Q . . P
        . . . . . . . .
        . . K . P . P .
        */
        board.set_cell(3, 2, Piece::new(PieceType::Pawn));
        board.set_cell(4, 4, Piece::new(PieceType::Pawn));
        board.set_cell(2, 7, Piece::new(PieceType::Pawn));
        board.set_cell(5, 7, Piece::new(PieceType::Pawn));
        board.set_cell(7, 6, Piece::new(PieceType::Pawn));
        board.set_cell(7, 4, Piece::new(PieceType::Pawn));
        board.set_cell(7, 2, Piece::new(PieceType::King));

        list_capture_pairs_into(&board, &mut capture_pairs);

        assert_vec_eq_unordered(
            &vec![
                step(5, 4, 3, 2, PieceType::Queen),
                step(5, 4, 4, 4, PieceType::Queen),
                step(5, 4, 2, 7, PieceType::Queen),
                step(5, 4, 5, 7, PieceType::Queen),
                step(5, 4, 7, 6, PieceType::Queen),
                step(5, 4, 7, 4, PieceType::Queen),
                step(5, 4, 5, 2, PieceType::Queen),
            ],
            &capture_pairs,
        );

        // Replace the queen with a king. Only adjacent pieces should be capturable.
        board.set_cell(5, 4, Piece::new(PieceType::King));
        list_capture_pairs_into(&board, &mut capture_pairs);
        assert_vec_eq_unordered(&vec![step(5, 4, 4, 4, PieceType::King)], &capture_pairs);
    }

    #[test]
    fn get_all_capture_pairs_for_pawn() {
        let mut board = Board::new();
        let mut capture_pairs = Vec::new();

        board.set_cell(4, 4, Piece::new(PieceType::Pawn));
        list_capture_pairs_into(&board, &mut capture_pairs);
        assert!(capture_pairs.is_empty());

        board.set_cell(3, 3, Piece::new(PieceType::Pawn));
        list_capture_pairs_into(&board, &mut capture_pairs);
        assert_eq!(vec![step(4, 4, 3, 3, PieceType::Pawn)], capture_pairs);

        board.set_cell(3, 5, Piece::new(PieceType::Pawn));
        list_capture_pairs_into(&board, &mut capture_pairs);
        assert_vec_eq_unordered(
            &vec![
                step(4, 4, 3, 3, PieceType::Pawn),
                step(4, 4, 3, 5, PieceType::Pawn),
            ],
            &capture_pairs,
        );
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