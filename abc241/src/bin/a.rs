use proconio::input;

fn main() {
    input! {
        a: [usize; 10],
    };
    let mut cur = 0;
    for _ in 0..3 {
        cur = a[cur];
    }
    let ans = cur;
    println!("{}", ans);
}
