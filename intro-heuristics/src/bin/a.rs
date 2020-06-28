use proconio::input;

fn main() {
    input! {
        d: usize,
        _c: [i64; 26],
        _s: [[i64; 26]; d],
    };
    for _ in 0..d {
        println!("{}", 1);
    }
}
