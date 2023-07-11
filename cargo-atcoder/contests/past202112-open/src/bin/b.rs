use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    };
    let coins = vec![500, 100, 50, 10, 5, 1];
    let mut count = vec![0; coins.len()];
    for (a, b) in ab {
        let mut x = b - a;
        for (i, c) in coins.iter().copied().enumerate() {
            count[i] += x / c;
            x %= c;
        }
    }
    let ans = count[2] + count[4];
    println!("{}", ans);
}
