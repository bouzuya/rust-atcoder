use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };
    let mut count = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] != '.' {
                continue;
            }
            for di in -1..=1 {
                for dj in -1..=1 {
                    let (ni, nj) = (i as i64 + di, j as i64 + dj);
                    if !(0..h as i64).contains(&ni) || !(0..w as i64).contains(&nj) {
                        continue;
                    }
                    let (ni, nj) = (ni as usize, nj as usize);
                    if s[ni][nj] == '#' {
                        count[i][j] += 1;
                    }
                }
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                print!("#");
            } else {
                print!("{}", count[i][j]);
            }
        }
        println!();
    }
}
