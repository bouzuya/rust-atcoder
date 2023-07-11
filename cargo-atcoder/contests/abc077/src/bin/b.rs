use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut max = 0_usize;
    for x in 1.. {
        if x * x > n {
            break;
        }
        max = x * x;
    }
    let ans = max;
    println!("{}", ans);
}
