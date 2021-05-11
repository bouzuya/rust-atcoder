use proconio::input;

fn main() {
    input! {
        m: usize,
        k: usize,
    };
    let ans = match m {
        0 => {
            if k == 0 {
                Some(vec![0, 0])
            } else {
                None
            }
        }
        1 => {
            if k == 0 {
                Some(vec![0, 0, 1, 1])
            } else if k == 1 {
                None
            } else {
                None
            }
        }
        _ => {
            if k < (1 << m) {
                let mut ans = vec![];
                for i in 0..1 << m {
                    if i == k {
                        continue;
                    }
                    ans.push(i);
                }
                ans.push(k);
                for i in (0..1 << m).rev() {
                    if i == k {
                        continue;
                    }
                    ans.push(i);
                }
                ans.push(k);
                Some(ans)
            } else {
                None
            }
        }
    };
    match ans {
        None => println!("-1"),
        Some(ans) => {
            println!(
                "{}",
                ans.iter()
                    .map(|i| format!("{}", i))
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }
    }
}
