use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    };
    let mut sum = 0_usize;
    for (i, a_i) in a.iter().copied().enumerate() {
        if ((x >> i) & 1) == 1 {
            sum += a_i;
        }
    }
    let ans = sum;
    println!("{}", ans);
}
