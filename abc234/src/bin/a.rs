use proconio::input;

fn f(x: usize) -> usize {
    x * x + 2 * x + 3
}

fn main() {
    input! {
        t: usize
    };
    let ans = f(f(f(t) + t) + f(f(t)));
    println!("{}", ans);
}
