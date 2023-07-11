// 解説 AC
use proconio::input;

fn main() {
    input! {
        n: i64,
        m: i64,
    };
    assert!(1 <= n && n <= 200_000);
    assert!(-n <= m && m <= n);
    if m < 0 {
        println!("-1");
        return;
    }
    if m == 0 {
        for l in (1..=1 + 2 * n - 2).step_by(2) {
            println!("{} {}", l, l + 1);
        }
        return;
    }
    assert!(1 <= m && m <= n);
    if m == 1 && n == 1 {
        println!("-1");
        return;
    }
    assert!(2 <= n);
    if m <= n - 2 {
        //
        println!("{} {}", 1, 1 + 2 * (m + 1) + 1);
        // 123456
        // |----|
        // .||||.
        let o = 2;
        for l in (o..=o + 2 * (m + 1) - 2).step_by(2) {
            println!("{} {}", l, l + 1);
        }
        // m+2
        let o = 1 + 2 * (m + 1) + 1 + 1;
        for l in (o..=o + 2 * (n - (m + 2)) - 2).step_by(2) {
            println!("{} {}", l, l + 1);
        }
        return;
    }
    if m == n - 1 {
        println!("-1");
        return;
    }
    if m == n {
        println!("-1");
        return;
    }
}
