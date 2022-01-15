use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    };
    let mut now = h[0];
    for i in 1..n {
        if h[i] > now {
            now = h[i];
        } else {
            break;
        }
    }
    let ans = now;
    println!("{}", ans);
}
