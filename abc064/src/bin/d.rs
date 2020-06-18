use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        s: Chars
    };

    let (min, end) = {
        // (((( ... 0, 4
        // )))) ... -4, -4
        // (()) ... 0, 0
        // ((()) ... 0, 1
        // (())) ... -1, -1
        // ))(( ... -2, 0
        let mut cur = 0;
        let mut min = 0;
        for &c in s.iter() {
            match c {
                '(' => cur += 1,
                ')' => {
                    cur -= 1;
                    min = std::cmp::min(min, cur);
                }
                _ => unreachable!(),
            }
        }
        (min, cur)
    };
    let mut t = vec![];
    for _ in 0..-min {
        t.push('(');
    }
    for &c in s.iter() {
        t.push(c);
    }
    for _ in 0..-min + end {
        t.push(')');
    }
    let ans = t.iter().collect::<String>();
    println!("{}", ans);
}
