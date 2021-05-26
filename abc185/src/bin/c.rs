use proconio::input;

fn main() {
    input! {
        l: usize,
    };
    let c = |n: usize, k: usize| -> usize {
        let mut x = 1_usize;
        for i in 0..k {
            x *= n - i;
            x /= i + 1;
        }
        x
    };
    let ans = c(l - 12 + 11, 11);
    println!("{}", ans);
}
