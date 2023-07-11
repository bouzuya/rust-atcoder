use proconio::input;

fn main() {
    input! {
        n: i64,
    };
    let ans = (-(2_i64.pow(31))..2_i64.pow(31)).contains(&n);
    println!("{}", if ans { "Yes" } else { "No" });
}
