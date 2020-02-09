use proconio::input;

fn main() {
    input! {
        r1: usize,
        c1: usize,
        r2: usize,
        c2: usize
    };
    let modp = 1_000_000_007;
    let mut tbl = vec![vec![0usize; c2 + 1]; r2 + 1];
    tbl[0][0] = 0;
    for r in 0..r2 + 1 {
        tbl[r][0] = 1;
    }
    for c in 0..c2 + 1 {
        tbl[0][c] = 1;
    }
    for r in 1..r2 + 1 {
        for c in 1..c2 + 1 {
            tbl[r][c] = ((tbl[r][c - 1] + tbl[r - 1][c]) % modp) % modp;
        }
    }
    let mut sum = 0;
    for r in r1..r2 + 1 {
        for c in c1..c2 + 1 {
            sum = (sum + tbl[r][c]) % modp;
        }
    }
    let ans = sum;
    println!("{}", ans);
}
