use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut following = vec![vec![false; n]; n];
    for _ in 0..q {
        input! { t: usize };
        match t {
            1 => {
                // a が b をフォロー
                input! {
                    a: Usize1,
                    b: Usize1,
                };
                following[a][b] = true;
            }
            2 => {
                // a が a のフォロワーをフォロー
                input! { a: Usize1 };
                for b in 0..n {
                    if following[b][a] {
                        following[a][b] = true;
                    }
                }
            }
            3 => {
                // a がフォローしているユーザー x のフォローしているユーザーをフォロー
                input! { a: Usize1 };
                let mut new_b = vec![];
                for x in 0..n {
                    if following[a][x] {
                        for b in 0..n {
                            if following[x][b] {
                                new_b.push(b);
                            }
                        }
                    }
                }
                for &b in new_b.iter() {
                    following[a][b] = true;
                }
            }
            _ => unreachable!(),
        }
    }

    for a in 0..n {
        println!(
            "{}",
            (0..n)
                .map(|b| if a != b && following[a][b] { 'Y' } else { 'N' })
                .collect::<String>()
        );
    }
}
