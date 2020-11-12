use proconio::input;

fn main() {
    input! {
        x: char,
        y: char,
    };
    let a = x as u8;
    let b = y as u8;
    let ans = if a > b {
        ">"
    } else if a < b {
        "<"
    } else if a == b {
        "="
    } else {
        unreachable!()
    };
    println!("{}", ans);
}
