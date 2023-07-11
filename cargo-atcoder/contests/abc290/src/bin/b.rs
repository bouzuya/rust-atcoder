use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    };
    let mut t = vec!['x'; n];
    let mut count = 0_usize;
    for (i, s_i) in s.into_iter().enumerate() {
        if count < k && s_i == 'o' {
            count += 1;
            t[i] = 'o';
        }
    }
    println!("{}", t.into_iter().collect::<String>());
}
