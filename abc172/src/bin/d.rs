use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut c = vec![1_usize; n + 1];
    for i in 2..=n {
        for j in (i..=n).step_by(i) {
            c[j] += 1;
        }
    }
    let ans = (1..=n).fold(0_usize, |acc, i| acc + i * c[i]);
    println!("{}", ans);
}
