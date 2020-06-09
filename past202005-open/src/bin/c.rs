use proconio::input;

fn main() {
    input! {
        a: u64,
        r: u64,
        n: u64,
    };
    let mut ans = a;
    for _ in 1..n {
        ans *= r;
        if ans > 1_000_000_000_u64 {
            println!("large");
            return;
        }
    }
    println!("{}", ans);
}
