use proconio::input;

fn main() {
    input! {
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    };
    let mut board = vec![vec![false; 100 + 1]; 100 + 1];
    for (a, b, c, d) in abcd {
        for x in a..b {
            for y in c..d {
                board[x][y] = true;
            }
        }
    }
    let mut count = 0_usize;
    for x in 0..=100 {
        for y in 0..=100 {
            if board[x][y] {
                count += 1;
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
