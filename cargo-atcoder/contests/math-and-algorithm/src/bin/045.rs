use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };
    let mut count = vec![0; n];
    for (a, b) in ab {
        match a.cmp(&b) {
            std::cmp::Ordering::Less => count[b] += 1,
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => count[a] += 1,
        }
    }
    let ans = count.into_iter().filter(|c| c == &1).count();
    println!("{}", ans);
}
