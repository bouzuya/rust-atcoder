use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let mut ans = false;
    let mut deque = s
        .iter()
        .map(|&x| x)
        .collect::<std::collections::VecDeque<_>>();
    for _ in 0..s.len() {
        if deque.iter().zip(t.iter()).all(|(s_i, t_i)| s_i == t_i) {
            ans = true;
        }
        deque.rotate_left(1);
    }
    println!("{}", if ans { "Yes" } else { "No" });
}
