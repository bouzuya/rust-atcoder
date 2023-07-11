use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        _: usize,
        s: [Chars; h],
    };
    let mut i = 1;
    let mut d = vec![vec![0; w]; h];
    let mut done = vec![false; h];
    for r in 0..h {
        let mut first = true;
        for c in 0..w {
            if s[r][c] == '#' {
                if first {
                    first = false;
                } else {
                    i += 1;
                }
            }
            d[r][c] = i;
        }
        if !first {
            i += 1;
            done[r] = true;
        }
    }
    let mut first = true;
    let mut skip = 0;
    let mut o = 0;
    for r in 0..h {
        if first {
            if done[r] {
                first = false;
                o = r;
            } else {
                skip += 1;
            }
        } else {
            if done[r] {
                o = r;
            } else {
                for c in 0..w {
                    d[r][c] = d[o][c];
                }
                done[r] = true;
            }
        }
    }
    for r in (0..skip).rev() {
        for c in 0..w {
            d[r][c] = d[skip][c];
        }
    }
    for r in 0..h {
        for c in 0..w {
            print!("{}{}", d[r][c], if c == w - 1 { "\n" } else { " " });
        }
    }
}
