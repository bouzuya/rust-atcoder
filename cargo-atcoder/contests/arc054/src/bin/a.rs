use proconio::input;

fn main() {
    input! {
        l: usize,
        x: usize,
        y: usize,
        s: usize,
        d: usize,
    };
    if d == s {
        println!("0");
        return;
    }
    let mut ans = l as f64 / x as f64;
    if x < y {
        // 反時計回り
        let da = y - x;
        let la = if d < s { s - d } else { l + s - d };
        let ta = la as f64 / da as f64;
        ans = if ta < ans { ta } else { ans };
    }
    // 時計回り
    let db = x + y;
    let lb = if d < s { l - s + d } else { d - s };
    let tb = lb as f64 / db as f64;
    ans = if tb < ans { tb } else { ans };
    println!("{}", ans);
}
