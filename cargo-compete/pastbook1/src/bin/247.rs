use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut count_w = vec![0; n];
    for i in 0..n {
        count_w[i] = if i > 0 { count_w[i - 1] } else { 0 } + if s[i] == 'W' { 1 } else { 0 };
    }
    let mut count_e = vec![0; n];
    for i in 0..n {
        count_e[i] =
            if i > 0 { count_e[i - 1] } else { 0 } + if s[n - 1 - i] == 'E' { 1 } else { 0 };
    }
    count_e.reverse();

    let mut min = n;
    for i in 0..n {
        min = min.min(count_e[i] + count_w[i] - 1);
    }
    println!("{}", min);
}
