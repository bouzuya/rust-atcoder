use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };
    let ans = s.iter().filter(|&s_i| s_i == "For").count() > n / 2;
    println!("{}", if ans { "Yes" } else { "No" });
}
