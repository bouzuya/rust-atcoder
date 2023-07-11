use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    };
    let mut count1 = 0;
    let mut count2 = 0;
    for (i, a_i) in a.iter().copied().enumerate() {
        for (j, b_j) in b.iter().copied().enumerate() {
            if a_i == b_j && i == j {
                count1 += 1;
            }
            if a_i == b_j && i != j {
                count2 += 1;
            }
        }
    }
    println!("{}", count1);
    println!("{}", count2);
}
