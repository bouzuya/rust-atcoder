use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut p = 1;
    let mut c = 1;
    for _ in 1..n {
        let n = c + p;
        p = c;
        c = n;
    }
    let ans = c;
    println!("{}", ans);
}
