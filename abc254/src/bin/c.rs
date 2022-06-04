use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mut b = a.clone();
    b.sort();

    let mut cs = vec![vec![]; k];
    for (i, a_i) in a.iter().copied().enumerate() {
        cs[i % k].push(a_i);
    }

    for i in 0..cs.len() {
        cs[i].sort();
    }

    let mut is = vec![0; k];
    for (i, b_i) in b.iter().copied().enumerate() {
        if cs[i % k][is[i % k]] != b_i {
            println!("No");
            return;
        }
        is[i % k] += 1;
    }
    println!("Yes");
}
