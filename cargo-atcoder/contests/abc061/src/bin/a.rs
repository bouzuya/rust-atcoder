use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    };
    let ans = (a..=b).contains(&c);
    println!("{}", if ans { "Yes" } else { "No" });
}
