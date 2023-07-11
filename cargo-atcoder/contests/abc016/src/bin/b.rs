use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    };
    let p = a + b == c;
    let m = a - b == c;
    let ans = if p && m {
        "?"
    } else if p && !m {
        "+"
    } else if !p && m {
        "-"
    } else if !p && !m {
        "!"
    } else {
        unreachable!()
    };
    println!("{}", ans);
}
