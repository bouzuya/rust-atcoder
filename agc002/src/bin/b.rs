use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(Usize1, Usize1); m],
    };
    let mut count = vec![1; n];
    let mut set_red = std::collections::HashSet::new();
    set_red.insert(0);
    for (x_i, y_i) in xy {
        if set_red.contains(&x_i) {
            set_red.insert(y_i);
        }
        count[x_i] -= 1;
        count[y_i] += 1;
        if count[x_i] == 0 {
            set_red.remove(&x_i);
        }
    }
    let ans = set_red.len();
    println!("{}", ans);
}
