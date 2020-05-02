use proconio::input;

fn main() {
    input! {
        x: u128
    };
    let mut ans = 0;
    let mut n = 100_u128;
    for y in 0.. {
        if n >= x {
            ans = y;
            break;
        }
        n = n * 101_u128 / 100_u128;
    }
    println!("{}", ans);
}
