use proconio::input;

fn main() {
    input! {
        a: [usize; 3],
    };
    let ans = a.iter().sum::<usize>();
    println!("{}", ans);
}
