use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    // \log_2 a < b \log_2 c
    // \log_2 a < \log_2 c^b
    // a < c^b
    let ans = if c == 1 {
        false
    } else {
        match c.checked_pow(b.min(60) as u32) {
            None => true,
            Some(cb) => a < cb,
        }
    };
    println!("{}", if ans { "Yes" } else { "No" });
}
