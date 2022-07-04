use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    };
    println!(
        "{}",
        a.into_iter()
            .filter(|a_i| a_i != &x)
            .map(|a_i| a_i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
