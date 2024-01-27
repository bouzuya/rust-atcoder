use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n]
    };
    let mut p = vec![(2, 0); 2 * n];
    for (i, (a, b)) in ab.into_iter().enumerate() {
        p[a.min(b)] = (0, i);
        p[a.max(b)] = (1, i);
    }
    let mut stack = vec![];
    for i in 0..2 * n {
        match p[i].0 {
            0 => {
                stack.push(p[i].1);
            }
            1 => {
                let item = stack.pop().unwrap();
                if item != p[i].1 {
                    println!("Yes");
                    return;
                }
            }
            _ => {}
        }
    }

    println!("No");
}
