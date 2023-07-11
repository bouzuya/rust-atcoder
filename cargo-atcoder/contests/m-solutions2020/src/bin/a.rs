use proconio::input;

fn main() {
    input! {
        x: i64
    };
    let ans = if x < 600 {
        8
    } else if x < 800 {
        7
    } else if x < 1000 {
        6
    } else if x < 1200 {
        5
    } else if x < 1400 {
        4
    } else if x < 1600 {
        3
    } else if x < 1800 {
        2
    } else {
        1
    };
    println!("{}", ans);
}
