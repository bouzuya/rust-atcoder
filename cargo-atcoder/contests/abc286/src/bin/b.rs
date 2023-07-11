use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };
    let mut is_n = false;
    let mut t = vec![];
    for s_i in s {
        if is_n {
            match s_i {
                'a' => {
                    t.push('y');
                    is_n = false;
                }
                'n' => {
                    is_n = true;
                }
                _ => {
                    is_n = false;
                }
            }
        } else {
            is_n = s_i == 'n';
        }
        t.push(s_i);
    }
    let ans = t.into_iter().collect::<String>();
    println!("{}", ans);
}
