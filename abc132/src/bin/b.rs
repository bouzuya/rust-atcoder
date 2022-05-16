use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    };
    let mut count = 0_usize;
    for i in 1..n - 1 {
        let mut u = vec![p[i - 1], p[i], p[i + 1]];
        u.sort();
        if u[1] == p[i] {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
