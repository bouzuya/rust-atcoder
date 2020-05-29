use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: [Chars; 3],
    };

    let mut i_c = 0;
    let mut i = vec![0; 3];
    loop {
        let i_x = i[i_c];
        let s_x = &s[i_c];
        if i_x >= s_x.len() {
            break;
        }
        let next = match s_x[i_x] {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            _ => unreachable!(),
        };
        i[i_c] += 1;
        i_c = next;
    }
    let ans = match i_c {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        _ => unreachable!(),
    };
    println!("{}", ans);
}
