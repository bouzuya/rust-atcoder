use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };

    let mut count_ieqai = 0_usize;
    for (i, a_i) in a.iter().copied().enumerate() {
        if i == a_i {
            count_ieqai += 1;
        }
    }

    let mut count = 0;
    for (i, a_i) in a.iter().copied().enumerate().rev() {
        let j = a_i;
        let a_j = a[j];
        if i == a_j && i < j {
            count += 1;
        }
    }
    let ans = count
        + if count_ieqai > 0 {
            count_ieqai * (count_ieqai - 1) / 2
        } else {
            0
        };
    println!("{}", ans);
}
