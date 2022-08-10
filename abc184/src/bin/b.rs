use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        x: usize,
        s: Chars,
    };
    let mut ans = x;
    for s_i in s {
        match s_i {
            'o' => ans += 1,
            'x' => ans = ans.saturating_sub(1),
            _ => unreachable!(),
        }
    }
    println!("{}", ans);
}
