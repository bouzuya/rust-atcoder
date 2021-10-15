use proconio::input;

fn main() {
    input! {
        a: [i64; 3],
    };
    let ans = if a[1] * 2 == a[0] + a[2] {
        0
    } else if a[1] * 2 > a[0] + a[2] {
        a[1] * 2 - a[0] - a[2]
    } else {
        (a[0] + a[2] - a[1] * 2) / 2
            + if (a[0] + a[2] - a[1] * 2) % 2 == 0 {
                0
            } else {
                2
            }
    };
    println!("{}", ans);
}
