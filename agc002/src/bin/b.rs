use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(Usize1, Usize1); m],
    };
    let mut count = vec![1; n];
    let mut red = vec![false; n];
    red[0] = true;
    for (x, y) in xy {
        count[x] -= 1;
        count[y] += 1;
        red[y] |= red[x];
        if count[x] == 0 {
            red[x] = false;
        }
    }
    let ans = red.into_iter().filter(|&b| b).count();
    println!("{}", ans);
}
