use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut count = 0_usize;
    for i in 0..30 {
        if n & (1 << i) != 0 {
            break;
        }
        count += 1;
    }
    let ans = count;
    println!("{}", ans);
}
