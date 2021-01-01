use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        t: Chars,
    };
    for t_i in t {
        let c = match t_i {
            'P' => 'P',
            'D' => 'D',
            '?' => 'D',
            _ => unreachable!(),
        };
        print!("{}", c);
    }
    println!();
}
