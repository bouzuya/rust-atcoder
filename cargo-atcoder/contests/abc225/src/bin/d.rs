use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut prev = vec![n; n];
    let mut next = vec![n; n];
    for _ in 0..q {
        input! {
            t: usize,
        };
        match t {
            1 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                };
                prev[y] = x;
                next[x] = y;
            }
            2 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                };
                prev[y] = n;
                next[x] = n;
            }
            3 => {
                input! {
                    x: Usize1,
                };
                let mut start = x;
                while prev[start] != n {
                    start = prev[start];
                }
                let mut ans = vec![];
                while next[start] != n {
                    ans.push(start + 1);
                    start = next[start];
                }
                ans.push(start + 1);

                print!("{} ", ans.len());
                for (i, a) in ans.iter().copied().enumerate() {
                    print!("{}{}", a, if i == ans.len() - 1 { "\n" } else { " " });
                }
            }
            _ => unreachable!(),
        }
    }
}
