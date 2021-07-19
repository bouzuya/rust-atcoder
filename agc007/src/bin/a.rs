use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [Chars; h]
    };

    let mut cur = (0, 0);
    a[0][0] = '.';
    while cur != (h - 1, w - 1) {
        let (r, c) = cur;
        let ok_b = if r + 1 < h { a[r + 1][c] == '#' } else { false };
        let ok_r = if c + 1 < w { a[r][c + 1] == '#' } else { false };
        match (ok_b, ok_r) {
            (false, false) | (true, true) => {
                println!("Impossible");
                return;
            }
            (true, false) => {
                cur = (r + 1, c);
                a[r + 1][c] = '.';
            }
            (false, true) => {
                cur = (r, c + 1);
                a[r][c + 1] = '.';
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                println!("Impossible");
                return;
            }
        }
    }
    println!("Possible");
}
