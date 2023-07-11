use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };
    let mut p = vec![];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == 'o' {
                p.push((i, j));
            }
        }
    }
    let (r1, c1) = p[0];
    let (r2, c2) = p[1];
    let dr = if r1 > r2 { r1 - r2 } else { r2 - r1 };
    let dc = if c1 > c2 { c1 - c2 } else { c2 - c1 };
    let ans = dr + dc;
    println!("{}", ans);
}
