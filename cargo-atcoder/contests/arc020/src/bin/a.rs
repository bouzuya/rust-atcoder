use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let ans = if a.abs() == b.abs() {
        "Draw"
    } else if a.abs() < b.abs() {
        "Ant"
    } else if a.abs() > b.abs() {
        "Bug"
    } else {
        unreachable!()
    };
    println!("{}", ans);
}
