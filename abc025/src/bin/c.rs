use proconio::input;

fn score(b: &[Vec<i64>], c: &[Vec<i64>], board: &[Vec<Option<bool>>]) -> i64 {
    let mut score = 0;
    for i in 0..2 {
        for j in 0..3 {
            if board[i][j] == board[i + 1][j] {
                score += b[i][j];
            }
        }
    }
    for i in 0..3 {
        for j in 0..2 {
            if board[i][j] == board[i][j + 1] {
                score += c[i][j];
            }
        }
    }
    score
}

fn f(sum: i64, b: &[Vec<i64>], c: &[Vec<i64>], board: &mut [Vec<Option<bool>>], t: usize) -> i64 {
    if t == 9 {
        return score(b, c, board);
    }

    let mut max = -sum;
    let mut min = sum;
    for i in 0..3 {
        for j in 0..3 {
            if board[i][j].is_none() {
                board[i][j] = Some(t % 2 == 0);
                let s = f(sum, b, c, board, t + 1);
                max = max.max(s);
                min = min.min(s);
                board[i][j] = None;
            }
        }
    }

    if t % 2 == 0 {
        max
    } else {
        min
    }
}

fn main() {
    input! {
        b: [[i64; 3]; 2],
        c: [[i64; 2]; 3],
    };

    let sum = b.iter().map(|b_i| b_i.iter().sum::<i64>()).sum::<i64>()
        + c.iter().map(|c_i| c_i.iter().sum::<i64>()).sum::<i64>();
    let mut board = vec![vec![None; 3]; 3];
    let score_t = f(sum, &b, &c, &mut board, 0);
    let score_a = sum - score_t;
    println!("{}", score_t);
    println!("{}", score_a);
}
