use proconio::input;

fn next(board: &Vec<Vec<char>>, h: usize, w: usize, i: usize, j: usize) -> (usize, usize) {
    let mut start = false;
    for r in 0..h {
        for c in 0..w {
            if start && board[r][c] == '.' {
                return (r, c);
            }
            if r == i && c == j {
                start = true;
            }
        }
    }
    (h, w)
}

fn dfs(
    count: &mut usize,
    board: &mut Vec<Vec<char>>,
    h: usize,
    w: usize,
    a: usize,
    b: usize,
    i: usize,
    j: usize,
) {
    if i == h && j == w {
        *count += 1;
        return;
    }

    if a > 0 && i + 1 < h && board[i + 1][j] == '.' {
        board[i][j] = 'a';
        board[i + 1][j] = 'a';
        let (ni, nj) = next(board, h, w, i, j);
        dfs(count, board, h, w, a - 1, b, ni, nj);
        board[i][j] = '.';
        board[i + 1][j] = '.';
    }

    if a > 0 && j + 1 < w && board[i][j + 1] == '.' {
        board[i][j] = 'b';
        board[i][j + 1] = 'b';
        let (ni, nj) = next(board, h, w, i, j);
        dfs(count, board, h, w, a - 1, b, ni, nj);
        board[i][j] = '.';
        board[i][j + 1] = '.';
    }

    if b > 0 {
        board[i][j] = 'c';
        let (ni, nj) = next(board, h, w, i, j);
        dfs(count, board, h, w, a, b - 1, ni, nj);
        board[i][j] = '.';
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        a: usize,
        b: usize,
    };
    let mut board = vec![vec!['.'; w]; h];
    let mut count = 0;
    dfs(&mut count, &mut board, h, w, a, b, 0, 0);
    let ans = count;
    println!("{}", ans);
}
