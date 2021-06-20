use proconio::input;

fn main() {
    input! {
        _: usize,
        a: usize,
        b: usize,
    };
    let ans = if (b - a - 1) % 2 == 0 {
        "Borys"
    } else {
        "Alice"
    };
    println!("{}", ans);
}
