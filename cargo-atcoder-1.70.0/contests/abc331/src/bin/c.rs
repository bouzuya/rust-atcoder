use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut b = a.clone();
    b.sort();
    let c = std::iter::once(0)
        .chain(b.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<usize>>();
    for a_i in a.iter().copied() {
        let index = b.lower_bound(&(a_i + 1));
        println!("{}", c[n] - c[index]);
    }
}
