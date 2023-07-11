use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut sum = 0_usize;
    let mut prev = 0_usize;
    for a_i in a {
        if prev > a_i {
            sum += prev - a_i;
        } else {
            prev = a_i;
        }
    }
    let ans = sum;
    println!("{}", ans);
}
