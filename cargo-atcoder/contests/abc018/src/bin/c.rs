use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        r: usize,
        c: usize,
        k: usize,
        s: [Chars; r],
    };
    let mut ct = vec![vec![0; c]; r];
    for j in 0..c {
        for i in 0..r {
            ct[i][j] = if s[i][j] == 'o' {
                if i < 1 {
                    1
                } else {
                    ct[i - 1][j] + 1
                }
            } else {
                0
            };
        }
    }
    let mut cb = vec![vec![0; c]; r];
    for i in 0..r {
        for j in 0..c {
            cb[r - (i + 1)][j] = if s[r - (i + 1)][j] == 'o' {
                if i < 1 {
                    1
                } else {
                    cb[r - i][j] + 1
                }
            } else {
                0
            };
        }
    }
    let mut count = 0;
    for i in k - 1..r - (k - 1) {
        for j in k - 1..c - (k - 1) {
            if s[i][j] != 'o' {
                continue;
            }
            let mut ok = true;
            for dj in -((k - 1) as i64)..=(k - 1) as i64 {
                let l = (j as i64 + dj) as usize;
                if cb[i][l] < k - dj.abs() as usize || ct[i][l] < k - dj.abs() as usize {
                    ok = false;
                }
            }
            if ok {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
