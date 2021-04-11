use proconio::input;

fn main() {
    input! {
        r: i64,
        x: i64,
        y: i64,
    };
    let d = ((x.pow(2) + y.pow(2)) as f64).sqrt();
    let r = r as f64;
    let mut c = 0;
    let mut x = d;
    while x > r {
        c += 1;
        x -= r;
    }
    let ans = c + if x == r {
        1
    } else {
        assert!(x < r);
        if c == 0 {
            2
        } else if c == 1 {
            1
        } else {
            1
        }
    };
    println!("{}", ans);
}
