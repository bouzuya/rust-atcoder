use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        t: usize,
    };
    let mut is_a = s == 0;
    for _ in 0..n - 1 {
        is_a = !is_a;
    }
    let ans = if t == 0 {
        if is_a {
            "Alice"
        } else {
            "Bob"
        }
    } else {
        if is_a {
            "Bob"
        } else {
            "Alice"
        }
    };
    println!("{}", ans);
}
