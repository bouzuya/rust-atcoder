use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        c: usize,
        a: [Usize1; n],
    };
    let mut pos = vec![n; c];
    let mut map = vec![0; c];
    for (i, a_i) in a.iter().copied().enumerate() {
        let l = if pos[a_i] == n { i } else { i - 1 - pos[a_i] };
        map[a_i] += l * (l + 1) / 2;
        pos[a_i] = i;
    }
    for i in 0..c {
        if pos[i] != n {
            let l = n - 1 - pos[i];
            map[i] += l * (l + 1) / 2;
        }
    }
    for i in 0..c {
        let ans = n * (n + 1) / 2 - map[i];
        println!("{}", ans);
    }
}
