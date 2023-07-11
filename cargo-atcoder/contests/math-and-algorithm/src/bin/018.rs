use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut count = vec![0_usize; 5];
    for a_i in a {
        count[a_i / 100] += 1;
    }

    let ans = (count[1] * count[4]) + (count[2] * count[3]);
    println!("{}", ans);
}
