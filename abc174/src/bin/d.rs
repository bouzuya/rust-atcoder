use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        c: Chars
    };
    let mut c_r = 0;
    let mut c_w = 0;
    for &c_i in c.iter() {
        match c_i {
            'R' => c_r += 1,
            'W' => c_w += 1,
            _ => unreachable!(),
        }
    }
    let mut ans = 0;
    for (i, &c_i) in c.iter().enumerate() {
        if i < c_r && c_i == 'W' {
            ans += 1;
        }
    }

    println!("{}", ans);
}
