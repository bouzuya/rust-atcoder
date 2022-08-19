use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    };
    let mut b = vec![vec![0; h]; w];
    for i in 0..w {
        for j in 0..h {
            b[i][j] = a[j][i];
        }
    }

    for i in 0..w {
        for j in 0..h {
            print!("{}{}", b[i][j], if j == h - 1 { '\n' } else { ' ' });
        }
    }
}
