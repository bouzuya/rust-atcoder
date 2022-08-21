use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        g: [Chars; h],
    };

    let mut used = vec![vec![false; w]; h];
    let mut pos = (0, 0);
    loop {
        let (i, j) = pos;
        if used[i][j] {
            println!("-1");
            return;
        }
        used[i][j] = true;

        match g[i][j] {
            'U' => {
                if i == 0 {
                    println!("{} {}", i + 1, j + 1);
                    return;
                }
                pos = (i - 1, j);
            }
            'D' => {
                if i == h - 1 {
                    println!("{} {}", i + 1, j + 1);
                    return;
                }
                pos = (i + 1, j);
            }
            'L' => {
                if j == 0 {
                    println!("{} {}", i + 1, j + 1);
                    return;
                }
                pos = (i, j - 1);
            }
            'R' => {
                if j == w - 1 {
                    println!("{} {}", i + 1, j + 1);
                    return;
                }
                pos = (i, j + 1);
            }
            _ => unreachable!(),
        }
    }
}
