use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        q: usize,
    };
    // f[a][b] : a が b をフォローしているとき true
    let mut following = vec![vec![false; n]; n];
    for _ in 0..q {
        input! { t: usize };
        match t {
            1 => {
                input! { a: Usize1, b: Usize1 };
                following[a][b] = true;
            }
            2 => {
                input! { a: Usize1 };
                let followers = (0..n).filter(|&i| following[i][a]).collect::<Vec<usize>>();
                for i in followers {
                    following[a][i] = true;
                }
            }
            3 => {
                input! { a: Usize1 };
                let f = following[a]
                    .iter()
                    .enumerate()
                    .filter(|(_, &b)| b)
                    .map(|(i, _)| i)
                    .collect::<Vec<usize>>();
                for i in f {
                    let ff = following[i]
                        .iter()
                        .enumerate()
                        .filter(|(_, &b)| b)
                        .map(|(j, _)| j)
                        .collect::<Vec<usize>>();
                    for j in ff {
                        if j == a {
                            continue;
                        }
                        following[a][j] = true;
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    for i in 0..n {
        for j in 0..n {
            print!("{}", if following[i][j] { "Y" } else { "N" });
        }
        println!();
    }
}
