use pathfinding::prelude::Matrix;

#[aoc(day25, part1)]
fn part1(input: &str) -> usize {
    let mut board: Matrix<u8> = input.lines().map(|l| l.bytes()).collect();
    let mut unchanged = true;
    for step in 0.. {
        unchanged |= step % 2 == 0;
        let mut nb = board.clone();
        let (s, r) = [(b'>', 3), (b'v', 1)][step % 2];
        for i @ (r, c) in board.indices() {
            if board[i] == s && board[(r, (c + 1) % board.columns)] == b'.' {
                nb.swap(board.idx(i), board.idx((r, (c + 1) % board.columns)));
                unchanged = false;
            }
        }
        board = nb.rotated_cw(r);
        if unchanged && step % 2 == 1 {
            return step / 2 + 1;
        }
    }
    unreachable!();
}
