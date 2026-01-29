# Solo Chess Solver

A lightweight, browser-based Solo Chess puzzle solver built with Rust and Dioxus.

## Features
- Place pieces on an 8x8 board and clear them with right-click.
- Generate a capture sequence that leaves one piece.
- Review solutions step-by-step with visual cues.

## How to use
1. Select a piece from the palette.
2. Click squares to place pieces; right-click to remove them.
3. Review the generated capture sequence below the board.

## Development

### Prerequisites
- Rust toolchain (stable).
- Optional: Dioxus CLI for web development.

### Run locally (web)
```bash
dx serve
```

### Build for production
```bash
dx build --release
```

## Constraints
Solo Chess with at most two captures per piece is NP-complete[^1]. With memoization and other minor pruning strategies, solve time becomes noticeably longer with more than 15 pieces, but that is enough to solve all the puzzles from chess.com.

[^1]: https://scale.iti.kit.edu/_media/resources/theses/ma_kolja_kuehn.pdf
