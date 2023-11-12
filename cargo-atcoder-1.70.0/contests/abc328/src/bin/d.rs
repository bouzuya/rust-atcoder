use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut t = vec![];
    for s_i in s {
        match s_i {
            'A' => t.push('A'),
            'B' => t.push('B'),
            'C' => {
                match (t.pop(), t.pop()) {
                    (None, None) => t.push('C'),
                    (None, Some(_)) => unreachable!(),
                    (Some(p1), None) => {
                        t.push(p1);
                        t.push('C');
                    }
                    (Some('B'), Some('A')) => {
                        // do nothing
                    }
                    (Some(p1), Some(p2)) => {
                        t.push(p2);
                        t.push(p1);
                        t.push('C');
                    }
                }
            }
            _ => unreachable!(),
        }
    }
    let ans = t.iter().collect::<String>();
    println!("{}", ans);
}
