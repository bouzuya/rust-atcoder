use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars
    };
    let mut c = vec![0; 4];
    for s_i in s.iter() {
        c[match s_i {
            'N' => 0,
            'W' => 1,
            'S' => 2,
            'E' => 3,
            _ => unreachable!(),
        }] += 1;
    }
    let ans = !((c[0] > 0 && c[2] == 0)
        || (c[0] == 0 && c[2] > 0)
        || (c[1] > 0 && c[3] == 0)
        || (c[1] == 0 && c[3] > 0));
    println!("{}", if ans { "Yes" } else { "No" });
}
