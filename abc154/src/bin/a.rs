use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
        a: usize,
        b: usize,
        u: String,
    };
    let (a, b) = if s == u {
        (a - 1, b)
    } else if t == u {
        (a, b - 1)
    } else {
        unreachable!()
    };
    println!("{} {}", a, b);
}
