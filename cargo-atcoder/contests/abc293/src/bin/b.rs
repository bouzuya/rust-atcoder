use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };
    let mut called = vec![false; n];
    for (i, a_i) in a.iter().copied().enumerate() {
        if called[i] {
            continue;
        }
        called[a_i] = true;
    }

    let mut uncalled = vec![];
    for (i, called) in called.iter().copied().enumerate() {
        if called {
            continue;
        }
        uncalled.push(i);
    }
    println!("{}", uncalled.len());
    for a in uncalled {
        println!("{}", a + 1);
    }
}
