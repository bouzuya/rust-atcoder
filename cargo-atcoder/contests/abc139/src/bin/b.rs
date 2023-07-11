use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let mut x = 0;
    let mut c = 1;
    while c < b {
        c -= 1;
        c += a;
        x += 1;
    }
    let ans = x;
    println!("{}", ans);
}
