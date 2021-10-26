use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        _: usize,
        s: [Chars; n]
    };
    let mut e = 0_usize;
    let mut o = 0_usize;
    for s_i in s {
        if s_i.iter().copied().filter(|&s_ij| s_ij == '1').count() % 2 == 0 {
            e += 1;
        } else {
            o += 1;
        }
    }
    let ans = e * o;
    println!("{}", ans);
}
