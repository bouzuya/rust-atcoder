use proconio::input;

fn main() {
    input! {
        a: [usize; 3],
    };
    let bust = a.iter().sum::<usize>() >= 22;
    println!("{}", if bust { "bust" } else { "win" });
}
