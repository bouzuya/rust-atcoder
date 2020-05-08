use proconio::input;

fn main() {
    input! {
        n: usize,
        xu: [(f64, String); n],
    };
    let mut ans = 0_f64;
    for (x, u) in xu {
        ans += match u.as_str() {
            "JPY" => x,
            "BTC" => x * 380000_f64,
            _ => unreachable!(),
        }
    }
    println!("{}", ans);
}
