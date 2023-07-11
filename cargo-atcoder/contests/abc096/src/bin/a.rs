use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = if a <= b { a } else { a - 1 };
    println!("{}", ans);
}
