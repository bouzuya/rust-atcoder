use proconio::{input, marker::Usize1};

fn main() {
    input! {
        abcde: [Usize1; 5],
    };
    let mut count = vec![0; 13];
    for x in abcde {
        count[x] += 1;
    }
    let ans = count.contains(&3) && count.contains(&2);
    println!("{}", if ans { "Yes" } else { "No" });
}
