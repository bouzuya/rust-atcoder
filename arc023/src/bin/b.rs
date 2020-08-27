use proconio::input;

fn main() {
    input! {
        r: usize,
        c: usize,
        d: usize,
        a: [[usize; c]; r],
    };
    let mut max_a = 0;
    let o = d % 2 == 0;
    for i in 0..std::cmp::min(d, r) {
        for j in 0..std::cmp::min(d + 1 - i, c) {
            if ((i + j) % 2 == 0) == o {
                max_a = std::cmp::max(max_a, a[i][j]);
            }
        }
    }
    let ans = max_a;
    println!("{}", ans);
}
