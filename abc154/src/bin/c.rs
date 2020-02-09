use proconio::input;

fn main() {
    input! {
        n: usize,
        mut av: [usize; n]
    };
    av.sort();
    let mut same = false;
    for i in 1..n {
        if av[i - 1] == av[i] {
            same = true;
            break;
        }
    }
    let ans = if !same { "YES" } else { "NO" };
    println!("{}", ans);
}
