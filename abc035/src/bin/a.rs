use proconio::input;

fn main() {
    input! {
        w: i64,
        h: i64,
    };
    let ans = if w * 3 == h * 4 {
        "4:3"
    } else if w * 9 == h * 16 {
        "16:9"
    } else {
        unreachable!()
    };
    println!("{}", ans);
}
