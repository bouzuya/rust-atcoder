use proconio::input;

fn main() {
    input! {
        a: i64,
        m: i64,
        l: i64,
        r: i64,
    };

    // a = 0 に補正
    let l = l - a;
    let r = r - a;

    // l >= 0, r >= 0 に補正
    let (l, r) = if l >= 0 {
        (l, r)
    } else {
        let d = ((l.abs() + m - 1) / m) * m;
        (l + d, r + d)
    };
    assert!(l >= 0 && r >= 0);

    if l == r {
        println!("{}", if l % m == 0 { 1 } else { 0 });
        return;
    }

    let count_r = r / m;
    let count_l = if l == 0 { 0 } else { (l - 1) / m };
    let ans = count_r - count_l + if l == 0 { 1 } else { 0 };
    println!("{}", ans);
}
