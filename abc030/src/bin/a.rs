use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    };
    let ans = if b * c == d * a {
        "DRAW"
    } else if b * c > d * a {
        "TAKAHASHI"
    } else if b * c < d * a {
        "AOKI"
    } else {
        unreachable!()
    };
    println!("{}", ans);
}
