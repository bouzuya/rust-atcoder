use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        mut s: [Chars; n],
    };

    let mut ans = 0;
    for i in 0..n {
        match s[i].iter().rposition(|&s_ij| s_ij == '.') {
            None => {}
            Some(i_r) => {
                for j in 0..=i_r {
                    s[i][j] = 'o';
                }
                if i + 1 < n {
                    for j in i_r..n {
                        s[i + 1][j] = 'o';
                    }
                }
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
