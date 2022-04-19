use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    };
    let mut sum = 0_usize;
    let mut prev = 0_usize;
    for h_i in h {
        if prev < h_i {
            sum += h_i - prev;
        }
        prev = h_i;
    }
    let ans = sum;
    println!("{}", ans);
}
