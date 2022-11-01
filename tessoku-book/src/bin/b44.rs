use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        q: usize,
        query: [(usize, Usize1, Usize1); q],
    };
    let mut is = (0..n).collect::<Vec<usize>>();
    for (t, x, y) in query {
        match t {
            1 => {
                is.swap(x, y);
            }
            2 => {
                println!("{}", a[is[x]][y]);
            }
            _ => unreachable!(),
        }
    }
}
