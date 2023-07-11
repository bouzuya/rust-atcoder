use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    for s_i in s {
        let c = match s_i {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => s_i,
            'O' => '0',
            'D' => '0',
            'I' => '1',
            'Z' => '2',
            'S' => '5',
            'B' => '8',
            _ => unreachable!(),
        };
        print!("{}", c);
    }
    println!();
}
