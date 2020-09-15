use proconio::input;

fn main() {
    input! {
        m: i64,
    };
    let vv = if m < 100 {
        0
    } else if 100 <= m && m <= 5_000 {
        m * 10 / 1_000
    } else if 6_000 <= m && m <= 30_000 {
        m / 1_000 + 50
    } else if 35_000 <= m && m <= 70_000 {
        (m / 1_000 - 30) / 5 + 80
    } else if 70_000 < m {
        89
    } else {
        unreachable!()
    };
    let ans = vv;
    println!("{:02}", ans);
}
