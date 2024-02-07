use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    };
    let mut a = vec![vec![0_usize; m]; n];
    for i in 0..n {
        for j in 0..m {
            let mut count = 0_usize;
            for di in -1..=1 {
                for dj in -1..=1 {
                    let (ni, nj) = (i as i64 + di, j as i64 + dj);
                    if !(0..n as i64).contains(&ni) || !(0..m as i64).contains(&nj) {
                        continue;
                    }
                    let (ni, nj) = (ni as usize, nj as usize);
                    if s[ni][nj] == '#' {
                        count += 1;
                    }
                }
            }
            a[i][j] = count;
        }
    }
    for i in 0..n {
        for j in 0..m {
            print!("{}", a[i][j]);
        }
        println!();
    }
}
