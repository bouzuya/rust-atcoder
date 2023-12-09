use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };
    let mut sum = 0_usize;
    for bits in 0..1 << n {
        let is = (0..n)
            .filter(|i| ((bits >> i) & 1) == 1)
            .collect::<Vec<usize>>();
        if is.len() != k {
            continue;
        }
        sum += is.into_iter().map(|i| a[i]).sum::<usize>();
    }
    let ans = sum;
    println!("{}", ans);
}
