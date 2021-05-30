use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let ans = if a == b {
        c
    } else if a == c {
        b
    } else if b == c {
        a
    } else {
        0
    };
    println!("{}", ans);
}
