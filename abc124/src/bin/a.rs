use proconio::input;

fn main() {
    input! {
       mut a: usize,
       mut b: usize,
    };
    let mut ans = 0_usize;
    if a > b {
        ans += a;
        a -= 1;
    } else {
        ans += b;
        b -= 1;
    };
    if a > b {
        ans += a;
        a -= 1;
    } else {
        ans += b;
        b -= 1;
    };
    println!("{}", ans);
}
