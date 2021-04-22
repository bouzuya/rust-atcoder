use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
    };
    let ans = if x < y { "Better" } else { "Worse" };
    println!("{}", ans);
}
