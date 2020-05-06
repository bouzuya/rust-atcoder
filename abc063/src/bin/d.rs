use proconio::input;

fn main() {
    input! {
        n: usize,
        a: i64,
        b: i64,
        h : [i64; n],
    };

    let mut ng = 0_i64;
    let mut ok = 1_000_000_000_i64;
    while ok - ng > 1 {
        let x = ng + (ok - ng) / 2;
        let x_a = h
            .iter()
            .map(|&h_i| (std::cmp::max(h_i - x * b, 0) + (a - b - 1)) / (a - b))
            .sum::<i64>();
        let is_ok = x_a <= x;
        if is_ok {
            ok = x;
        } else {
            ng = x;
        }
    }
    let ans = ok;
    println!("{}", ans);
}
