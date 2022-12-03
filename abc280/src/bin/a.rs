use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };
    let mut count = 0_usize;
    for r in 0..h {
        for c in 0..w {
            if s[r][c] == '#' {
                count += 1;
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
