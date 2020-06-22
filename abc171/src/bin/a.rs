use proconio::input;

fn main() {
    input! {
        a: char,
    };
    let ans = if a.is_uppercase() { "A" } else { "a" };
    println!("{}", ans);
}
