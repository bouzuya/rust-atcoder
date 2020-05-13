use proconio::input;

// F(0, n)
fn f(n: u64) -> u64 {
    if n % 2 == 0 {
        if (n / 2) % 2 == 0 {
            n ^ 0
        } else {
            n ^ 1
        }
    } else {
        f(n - 1) ^ n
    }
}

fn main() {
    input! {
        a: u64,
        b: u64,
    };
    let ans = if a == 0 { b } else { f(a - 1) ^ f(b) };
    println!("{}", ans);
}
