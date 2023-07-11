use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    };
    let mut ans = a
        .iter()
        .copied()
        .filter(|a_i| b.contains(&a_i))
        .collect::<Vec<usize>>();
    ans.sort();
    println!(
        "{}",
        ans.iter()
            .map(|a_i| a_i.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
