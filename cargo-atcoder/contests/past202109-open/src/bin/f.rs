use proconio::{input, marker::Chars};

fn ok(s: &[char], a: &[usize]) -> bool {
    for (i, c) in s.iter().copied().enumerate() {
        match c {
            '0' => {
                if a[i] == i + 1 {
                    return false;
                }
            }
            '1' => {
                if a[i] != i + 1 {
                    return false;
                }
            }
            _ => unreachable!(),
        }
    }
    true
}

fn main() {
    input! {
        n: usize,
        s: Chars,
    };
    let mut ans = vec![0_usize; n];
    let mut is = vec![];
    for (i, c) in s.iter().copied().enumerate() {
        match c {
            '0' => is.push(i),
            '1' => ans[i] = i + 1,
            _ => unreachable!(),
        }
    }

    if is.len() == 1 {
        println!("-1");
        return;
    }

    let m = is.len();
    for i in 0..m {
        ans[is[i]] = is[(i + 1) % m] + 1;
    }

    assert!(ok(&s, &ans));

    println!(
        "{}",
        ans.into_iter()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
