use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    if (0..s.len() / 2).all(|i| s[i] == s[s.len() - i - 1]) {
        println!("0");
        return;
    }
    let nx = s
        .iter()
        .filter(|&&c| c != 'x')
        .map(|&c| c)
        .collect::<Vec<char>>();
    if !(0..nx.len() / 2).all(|i| nx[i] == nx[nx.len() - i - 1]) {
        println!("-1");
        return;
    }
    let mut ans = 0;
    let mut i_l = 0;
    let mut i_r = s.len() - 1;
    while i_l < i_r {
        let s_l = s[i_l];
        let s_r = s[i_r];
        if s_l == s_r {
            i_l += 1;
            i_r -= 1;
        } else if s_l == 'x' {
            i_l += 1;
            ans += 1;
        } else if s_r == 'x' {
            i_r -= 1;
            ans += 1;
        } else {
            unreachable!();
        }
    }
    println!("{}", ans);
}
