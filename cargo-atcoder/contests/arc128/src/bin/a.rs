use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut ans = vec![0_usize; n];
    for i in 0..n - 1 {
        if a[i] > a[i + 1] {
            ans[i] ^= 1;
            ans[i + 1] ^= 1;
        }
    }
    for a in ans {
        println!("{}", a);
    }
}
