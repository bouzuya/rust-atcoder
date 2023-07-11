use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let max = a.iter().copied().max().unwrap();
    if a.iter().copied().filter(|a_i| a_i == &max).count() >= 2 {
        for _ in a {
            println!("{}", max);
        }
        return;
    }
    let mut b = a.clone();
    b.sort();
    b.reverse();
    let max2 = b[1];

    for a_i in a {
        let ans = if a_i != max { max } else { max2 };
        println!("{}", ans);
    }
}
