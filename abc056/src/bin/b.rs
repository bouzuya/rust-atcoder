use proconio::input;

fn main() {
    input! {
        w: i64,
        a: i64,
        b: i64,
    };
    let (b_l, b_r) = (b, b + w);
    let (a_l, a_r) = (a, a + w);
    let ans = if b_r < a_l {
        a_l - b_r
    } else if a_r < b_l {
        b_l - a_r
    } else {
        0
    };
    println!("{}", ans);
}
