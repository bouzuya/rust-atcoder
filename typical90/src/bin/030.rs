use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let mut count = vec![0; n + 1];
    for i in 2..=n {
        if count[i] > 0 {
            continue;
        }
        for j in (i..=n).step_by(i) {
            count[j] += 1;
        }
    }
    let ans = count.iter().filter(|&&c| c >= k).count();
    println!("{}", ans);
}
