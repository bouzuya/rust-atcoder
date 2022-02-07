use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let a = match 2_usize.checked_pow(n as u32) {
        Some(a) => a,
        None => {
            println!("Yes");
            return;
        }
    };
    let b = n * n;
    let ans = a > b;
    println!("{}", if ans { "Yes" } else { "No" });
}
