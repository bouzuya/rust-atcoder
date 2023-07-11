use proconio::input;

fn main() {
    input! {
        a: char,
        b: char,
    };
    let ans = match a {
        'H' => b,
        'D' => match b {
            'H' => 'D',
            'D' => 'H',
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    println!("{}", ans);
}
