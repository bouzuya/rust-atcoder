use proconio::{input, marker::Usize1};

fn f(a: &[Vec<usize>], set: &[bool]) -> Vec<usize> {
    let mut count = vec![0; a[0].len()];
    for a_i in a.iter() {
        for a_ij in a_i.iter().copied() {
            if set[a_ij] {
                count[a_ij] += 1;
                break;
            }
        }
    }
    count
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[Usize1; m]; n],
    };
    let mut set = vec![true; m];
    let mut ans = n;
    for _ in 0..m {
        let count = f(&a, &set);
        let max = count.iter().copied().max().unwrap();
        ans = ans.min(max);
        set[count.iter().copied().position(|c| c == max).unwrap()] = false;
    }
    println!("{}", ans);
}
