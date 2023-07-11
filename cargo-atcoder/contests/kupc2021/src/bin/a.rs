use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: [usize; n],
        t: usize,
    };
    s.sort();
    let mut count = 0_usize;
    let mut next = 0;
    for s_i in s {
        if s_i > next {
            count += 1;
            next = (s_i + t - 1) / t * t;
        }
    }
    let ans = count;
    println!("{}", ans);
}
