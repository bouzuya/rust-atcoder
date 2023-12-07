use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
        r: i64,
        n: i64,
    };

    let mut s = vec![vec!['.'; (2 * n + 1) as usize]; (2 * n + 1) as usize];
    for i in -n..=n {
        for j in -n..=n {
            if (i - x).pow(2) + (j - y).pow(2) <= r.pow(2) {
                s[(i + n) as usize][(j + n) as usize] = '#';
            }
        }
    }
    for i in 0..2 * n as usize + 1 {
        for j in 0..2 * n as usize + 1 {
            print!(
                "{}{}",
                s[i][j],
                if j == 2 * n as usize { '\n' } else { ' ' }
            );
        }
    }
}
