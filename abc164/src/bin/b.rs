use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    };
    let mut hpt = a;
    let mut hpa = c;
    for i in 0.. {
        if hpt <= 0 || hpa <= 0 {
            break;
        }
        if i % 2 == 0 {
            hpa -= b;
        } else {
            hpt -= d;
        }
    }
    println!("{}", if hpt > 0 { "Yes" } else { "No" });
}
