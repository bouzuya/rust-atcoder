use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        mut c: [usize; n],
        q: usize,
        x: [usize; q],
    };

    c.sort();
    let cs = std::iter::once(0)
        .chain(c.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<usize>>();

    for x_j in x {
        let count = cs.lower_bound(&(x_j + 1)).saturating_sub(1);
        println!("{}", count);
    }
}
