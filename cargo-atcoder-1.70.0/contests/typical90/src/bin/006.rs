use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    };
    let mut ans = vec![];
    let mut l = 0_usize;
    for j in (1..=k).rev() {
        let r = n + 1 - j;
        let min_i = s[l..r]
            .iter()
            .copied()
            .enumerate()
            .min_by_key(|&(_, c)| c)
            .map(|(i, _)| l + i)
            .unwrap();
        ans.push(s[min_i]);
        l = min_i + 1;
    }

    println!("{}", ans.into_iter().collect::<String>());
}
