use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    };

    let mut rows = vec![true; h];
    for i in 0..h {
        let mut used = false;
        for j in 0..w {
            if a[i][j] != '.' {
                used = true;
            }
        }
        rows[i] = used;
    }

    let mut cols = vec![true; w];
    for j in 0..w {
        let mut used = false;
        for i in 0..h {
            if a[i][j] != '.' {
                used = true;
            }
        }
        cols[j] = used;
    }

    for i in 0..h {
        for j in 0..w {
            if rows[i] && cols[j] {
                print!("{}", a[i][j]);
            }
        }
        println!();
    }
}
