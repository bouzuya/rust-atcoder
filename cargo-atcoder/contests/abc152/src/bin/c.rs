use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    };
    let mut min = p[0];
    let mut count = 0_usize;
    for p_i in p {
        if p_i <= min {
            min = min.min(p_i);
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
