use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        x: Usize1,
        a: [Usize1; n],
    };
    let mut visited = vec![false; n];
    let mut i = x;
    while !visited[i] {
        visited[i] = true;
        i = a[i];
    }
    let ans = visited.into_iter().filter(|b| *b).count();
    println!("{}", ans);
}
