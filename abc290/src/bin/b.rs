use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    };
    let mut count = 0_usize;
    let mut ans = vec!['x'; n];
    for (i, c) in s.into_iter().enumerate() {
        match c {
            'o' => {
                count += 1;
                if count <= k {
                    ans[i] = 'o';
                }
            }
            'x' => {}
            _ => unreachable!(),
        }
    }
    println!("{}", ans.iter().collect::<String>());
}
