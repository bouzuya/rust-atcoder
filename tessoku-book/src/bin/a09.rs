use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    };
    let mut count = vec![vec![0_i64; w + 2]; h + 2];
    for (a, b, c, d) in abcd {
        count[a][b] += 1;
        count[c + 1][b] -= 1;
        count[a][d + 1] -= 1;
        count[c + 1][d + 1] += 1;
    }

    for i in 0..h {
        for j in 0..w {
            count[i + 1][j + 1] += count[i + 1][j];
        }
    }
    for j in 0..=w {
        for i in 0..h {
            count[i + 1][j] += count[i][j];
        }
    }

    for i in 1..=h {
        for j in 1..=w {
            print!("{}{}", count[i][j], if j == w { '\n' } else { ' ' });
        }
    }
}
