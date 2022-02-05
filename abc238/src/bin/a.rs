use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let b = n * n;
    let mut a = 1_usize;
    for _ in 0..n {
        a = match a.checked_mul(2) {
            Some(a_next) => a_next,
            None => {
                println!("Yes");
                return;
            }
        }
    }
    let ans = a > b;
    println!("{}", if ans { "Yes" } else { "No" });
}
