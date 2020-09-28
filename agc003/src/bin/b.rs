use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut s = 0;
    let mut c = 0;
    for &a_i in a.iter() {
        if a_i == 0 {
            s += c / 2;
            c = 0;
        } else {
            c += a_i;
        }
    }
    if c > 0 {
        s += c / 2;
    }
    let ans = s;
    println!("{}", ans);
}
