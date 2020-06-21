use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut c = 0;
    let mut s = 0;
    for &a_i in a.iter() {
        if a_i > 0 {
            c += 1;
            s += a_i;
        }
    }
    let ans = (s + c - 1) / c;
    println!("{}", ans);
}
