use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut a: [usize; 10],
        c: [Chars; n],
    };
    let mut xs = vec![0; 7];
    let mut cs = vec![0; 7];
    for c_i in c.iter() {
        for (j, c_ij) in c_i.iter().copied().enumerate() {
            match c_ij {
                'X' => {
                    cs[j] += 1;
                    xs[j] = xs[j].max(cs[j]);
                }
                '-' => {
                    cs[j] = 0;
                }
                _ => unreachable!(),
            }
        }
    }
    xs.sort();
    a.sort();

    let ans = xs.iter().zip(a.iter().skip(3)).all(|(x_i, a_i)| x_i <= a_i);
    println!("{}", if ans { "YES" } else { "NO" });
}
