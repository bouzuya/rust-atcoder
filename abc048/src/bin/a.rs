use proconio::input;

fn main() {
    input! {
        _atcoder: String,
        s: String,
        _contest: String,
    };
    println!("A{}C", s.chars().next().unwrap());
}
