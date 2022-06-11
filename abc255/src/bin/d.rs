use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        x: [usize; q]
    };
    a.sort();

    let c = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<usize>>();

    // let mut p = a[0];
    // let mut d = vec![];
    // for a_i in a.iter().copied() {
    //     d.push(a_i - p);
    //     p = a_i;
    // }

    // let mut c = d.clone();
    // for i in 1..n {
    //     c[i] += c[i - 1];
    // }

    for x_i in x {
        let c_l = a.lower_bound(&x_i);
        let c_r = n - a.upper_bound(&x_i);
        let s_l = c[c_l];
        let s_r = c[n] - c[n - c_r];
        let ans = ((x_i * c_l) - s_l) + (s_r - (x_i * c_r));
        println!("{}", ans);
    }
}
