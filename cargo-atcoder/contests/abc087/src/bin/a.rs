use proconio::input;

fn main() {
    input! {
        x: i64,
        a: i64,
        b: i64,
    };
    let mut y = x;
    y -= a;
    loop {
        if y - b >= 0 {
            y -= b;
        } else {
            break;
        }
    }
    let ans = y;
    println!("{}", ans);
}
