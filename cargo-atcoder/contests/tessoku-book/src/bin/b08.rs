use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    };
    let mut count = vec![vec![0_usize; 1500 + 1]; 1500 + 1];
    for (x, y) in xy {
        count[y][x] += 1;
    }
    for i in 0..1500 {
        for j in 0..1500 {
            count[i + 1][j + 1] += count[i + 1][j];
        }
    }
    for j in 1..=1500 {
        for i in 0..1500 {
            count[i + 1][j] += count[i][j];
        }
    }

    for (a, b, c, d) in abcd {
        let ans = count[d][c] + count[b - 1][a - 1] - count[d][a - 1] - count[b - 1][c];
        println!("{}", ans);
    }
}
