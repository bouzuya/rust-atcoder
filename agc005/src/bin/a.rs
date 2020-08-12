use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: Chars,
    };
    let mut c = 0;
    let mut s = 0;
    for &x_i in x.iter() {
        match x_i {
            'S' => {
                s += 1;
            }
            'T' => {
                if s > 0 {
                    s -= 1;
                    c += 2;
                }
            }
            _ => unreachable!(),
        }
    }
    let ans = x.len() - c;
    println!("{}", ans);
}
