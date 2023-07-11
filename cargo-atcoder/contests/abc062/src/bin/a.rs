use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let groups = vec![vec![1, 3, 5, 7, 8, 10, 12], vec![4, 6, 9, 11], vec![2]];
    for group in groups {
        if group.contains(&a) && group.contains(&b) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
