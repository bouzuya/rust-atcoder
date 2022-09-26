use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
        capital_k: usize,
        a: [usize; x],
        b: [usize; y],
        c: [usize; z],
    };
    let mut ab = vec![];
    for a_i in a {
        for b_i in b.iter().copied() {
            ab.push(a_i + b_i);
        }
    }
    ab.sort();
    ab.reverse();

    let mut abc = vec![];
    let ab = ab[0..capital_k.min(ab.len())].to_vec();
    for ab_i in ab {
        for c_i in c.iter().copied() {
            abc.push(ab_i + c_i);
        }
    }
    abc.sort();
    abc.reverse();
    for a in abc.into_iter().take(capital_k) {
        println!("{}", a);
    }
}
