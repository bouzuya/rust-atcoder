use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    };
    let rows = a
        .iter()
        .map(|r| r.iter().sum::<usize>())
        .collect::<Vec<usize>>();
    let cols = (0..w)
        .map(|j| (0..h).map(|i| a[i][j]).sum::<usize>())
        .collect::<Vec<usize>>();
    for i in 0..h {
        for j in 0..w {
            let ans = rows[i] + cols[j] - a[i][j];
            print!("{}{}", ans, if j == w - 1 { "\n" } else { " " });
        }
    }
}
