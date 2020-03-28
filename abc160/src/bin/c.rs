use proconio::input;

fn main() {
    input! {
        k: usize,
        n: usize,
        av: [usize; n],
    };
    let mut d = k - av[n - 1] + av[0];
    for i in 0..av.len() - 1 {
        d = std::cmp::max(d, av[i + 1] - av[i]);
    }
    let ans = k - d;
    println!("{}", ans);
}
