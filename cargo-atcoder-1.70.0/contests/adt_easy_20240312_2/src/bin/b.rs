use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let a = n
        .to_string()
        .chars()
        .map(|c| (c as u8 - b'0') as usize)
        .collect::<Vec<usize>>();
    let mut p = a[0];
    for a_i in a.into_iter().skip(1) {
        if a_i >= p {
            println!("No");
            return;
        }
        p = a_i;
    }
    println!("Yes");
}
