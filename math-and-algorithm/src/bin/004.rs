use proconio::input;

fn main() {
    input! {
        a: [usize; 3],
    };
    let ans = a.iter().product::<usize>();
    println!("{}", ans);
}
