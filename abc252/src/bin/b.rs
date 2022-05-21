use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [Usize1; k],
    };
    let max = *a.iter().max().unwrap();
    for (i, a_i) in a.iter().copied().enumerate() {
        if a_i == max && b.contains(&i) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
