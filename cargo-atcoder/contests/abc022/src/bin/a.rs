use proconio::input;

fn main() {
    input! {
        n: usize,
        s: i64,
        t: i64,
        w: i64,
        a: [i64; n - 1],
    };
    let mut c = w;
    let mut count = if (s..=t).contains(&c) { 1 } else { 0 };
    for a_i in a {
        c += a_i;
        if (s..=t).contains(&c) {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
