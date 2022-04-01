use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    };
    let mut sum = 0_usize;
    for (a, b) in ab {
        let m = b - a + 1;
        sum += (2 * a + (m - 1)) * m / 2;
    }
    let ans = sum;
    println!("{}", ans);
}
