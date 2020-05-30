use proconio::input;

fn main() {
    input! {
        n: usize
    };
    let mut l = vec![0_usize; n + 1];
    l[0] = 2;
    l[1] = 1;
    for i in 2..=n {
        l[i] = l[i - 1] + l[i - 2];
    }
    let ans = l[n];
    println!("{}", ans);
}
