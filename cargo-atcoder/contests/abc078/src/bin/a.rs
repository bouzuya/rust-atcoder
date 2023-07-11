use proconio::input;

fn main() {
    input! {
        x: char,
        y: char,
    };
    let ans = if x == y {
        "="
    } else if x < y {
        "<"
    } else if x > y {
        ">"
    } else {
        unreachable!()
    };
    println!("{}", ans);
}
