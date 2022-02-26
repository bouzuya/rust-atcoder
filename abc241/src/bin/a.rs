use proconio::input;

fn main() {
    input! {
        a: [usize; 10],
    };
    let mut c = 0;
    for _ in 0..3 {
        c = a[c];
    }
    let ans = c;
    println!("{}", ans);
}
