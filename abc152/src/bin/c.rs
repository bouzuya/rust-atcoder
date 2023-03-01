use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    };
    let mut count = 0_usize;
    let mut min = p[0];
    for p_i in p {
        if p_i <= min {
            count += 1;
            min = p_i;
        }
    }
    let ans = count;
    println!("{}", ans);
}
