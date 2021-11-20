use proconio::input;

fn main() {
    input! {
        s: usize,
        t: usize,
        x: usize,
    };

    let ans = if s < t {
        (s..t).contains(&x)
    } else {
        (s..=24).contains(&x) || (0..t).contains(&x)
    };
    println!("{}", if ans { "Yes" } else { "No" });
}
