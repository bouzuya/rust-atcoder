use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    };
    s.reverse();

    let mut ans = 0_usize;
    let mut count = 0_usize;
    for s_i in s {
        match s_i {
            'B' => {
                ans += count;
            }
            'W' => {
                count += 1;
            }
            _ => unreachable!(),
        }
    }
    println!("{}", ans);
}
