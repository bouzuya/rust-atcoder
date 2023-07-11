use proconio::input;

fn main() {
    input! {
        n: usize,
        xu: [(f64, String); n],
    };
    let mut y = 0_f64;
    for (x, u) in xu {
        y += match u.as_str() {
            "JPY" => x,
            "BTC" => x * 380_000_f64,
            _ => unreachable!(),
        };
    }
    let ans = y;
    println!("{}", ans);
}
