use proconio::input;

fn main() {
    input! {
        b: char
    };
    let ans = match b {
        'A' => 'T',
        'C' => 'G',
        'G' => 'C',
        'T' => 'A',
        _ => unreachable!(),
    };
    println!("{}", ans);
}
