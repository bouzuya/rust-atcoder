use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    };
    let mut x = vec![0_usize; w];
    for j in 0..w {
        for i in 0..h {
            if c[i][j] == '#' {
                x[j] += 1;
            }
        }
    }
    for a in x {
        println!("{}", a);
    }
}
