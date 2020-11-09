use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut max_c = 0;
    let mut p = (0, 0);
    let mut c = 0;
    for a_i in a {
        p = if a_i > 0 {
            (std::cmp::max(p.0, p.1 + a_i), p.1 + a_i)
        } else {
            (p.0, p.1 + a_i)
        };
        if c + p.0 > max_c {
            max_c = c + p.0;
        }
        c += p.1;
    }
    let ans = max_c;
    println!("{}", ans);
}
