use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    };
    let mut ans = vec![false; n];
    let mut j = 0;
    for (i, s_i) in s.iter().enumerate() {
        if s_i == &t[j] {
            ans[i] = true;
            j += 1;
        }
    }
    for a in ans {
        println!("{}", if a { "Yes" } else { "No" });
    }
}
