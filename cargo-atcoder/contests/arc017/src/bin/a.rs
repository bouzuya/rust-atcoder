use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut ans = true;
    for i in 2..n {
        if n % i == 0 {
            ans = false;
            break;
        }
    }
    println!("{}", if ans { "YES" } else { "NO" });
}
