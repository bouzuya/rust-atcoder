use proconio::input;

fn main() {
    input! {
        n: i64,
    };
    let p = 998_244_353_i64;
    let n = ((n % p) + p) % p;
    println!("{}", n);
}
