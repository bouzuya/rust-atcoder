use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars
    };
    let mut max_x = 0;
    let mut x = 0;
    for s_i in s.iter() {
        match s_i {
            'I' => x += 1,
            'D' => x -= 1,
            _ => unreachable!(),
        }
        max_x = std::cmp::max(max_x, x);
    }
    let ans = max_x;
    println!("{}", ans);
}
