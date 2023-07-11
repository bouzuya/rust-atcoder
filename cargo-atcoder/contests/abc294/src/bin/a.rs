use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let ans = a
        .into_iter()
        .filter(|a_i| a_i % 2 == 0)
        .collect::<Vec<usize>>();
    for a_i in ans {
        println!("{}", a_i);
    }
}
