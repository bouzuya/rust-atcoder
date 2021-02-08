use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
    };
    let ans = x < y;
    println!("{}", if ans { "Better" } else { "Worse" });
}
