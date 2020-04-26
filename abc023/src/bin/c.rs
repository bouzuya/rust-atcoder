use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        r: usize,
        c: usize,
        k: usize,
        n: usize,
        rcs: [(Usize1, Usize1); n],
    };
    let mut tbl = vec![vec![0_usize; c]; r];
    let mut rc = vec![0_usize; r];
    let mut cc = vec![0_usize; c];
    for &(r_i, c_i) in rcs.iter() {
        rc[r_i] += 1;
        cc[c_i] += 1;
        tbl[r_i][c_i] += 1;
    }
    let mut count = 0;
    for y in 0..r {
        for x in 0..c {
            if rc[y] + cc[x] - tbl[y][x] == k {
                count += 1;
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
