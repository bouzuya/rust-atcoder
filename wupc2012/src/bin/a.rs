use proconio::input;
use proconio::marker::Usize1;

fn cumsum(a: &Vec<usize>) -> Vec<usize> {
    std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect()
}

fn main() {
    input! {
        m_a: Usize1,
        d_a: usize,
        m_b: Usize1,
        d_b: usize,
    };
    let d = vec![31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let c = cumsum(&d);
    let ans = (c[m_b] - c[m_a]) + (d_b - d_a);
    println!("{}", ans);
}
