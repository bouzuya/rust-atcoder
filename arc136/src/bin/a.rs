use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };
    let mut t = String::new();
    let mut count = (0, 0);
    for s_i in s {
        match s_i {
            'A' => count.0 += 1,
            'B' => count.1 += 1,
            'C' => {
                t.push_str(&"A".repeat(count.0 + count.1 / 2));
                if count.1 % 2 != 0 {
                    t.push_str("B");
                }
                t.push_str("C");
                count = (0, 0);
            }
            _ => unreachable!(),
        }
    }
    t.push_str(&mut "A".repeat(count.0 + count.1 / 2));
    if count.1 % 2 != 0 {
        t.push_str("B");
    }
    let ans = t;
    println!("{}", ans);
}
