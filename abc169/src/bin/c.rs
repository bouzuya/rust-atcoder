use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        mut a: u128,
        b: Chars,
    };

    let mut d = vec![];
    for &c_i in b.iter() {
        if c_i.is_digit(10) {
            d.push(c_i.to_digit(10).unwrap() as u128);
        }
    }
    let mut ans = 0;
    for &d_i in d.iter() {
        ans *= 10;
        ans += a * d_i;
    }
    ans *= 10;
    ans /= 10_u128.pow(d.len() as u32);
    println!("{}", ans);
}
