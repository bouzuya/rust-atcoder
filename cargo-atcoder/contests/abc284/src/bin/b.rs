use proconio::input;

fn main() {
    input! {
        t: usize,
    };
    for _ in 0..t {
        input! {
            n: usize,
            a: [usize; n]
        }

        let ans = a.into_iter().filter(|a_i| a_i % 2 != 0).count();
        println!("{}", ans);
    }
}
