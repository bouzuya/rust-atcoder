use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut max_count = 0_usize;
    let mut max_k = 0_usize;
    let max_a = a.iter().copied().max().unwrap();
    for k in 2..=max_a {
        let count = a.iter().copied().filter(|a_i| a_i % k == 0).count();
        if count >= max_count {
            max_count = count;
            max_k = k;
        }
    }
    let ans = max_k;
    println!("{}", ans);
}
