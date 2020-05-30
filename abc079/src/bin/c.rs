use proconio::input;
use proconio::marker::Chars;

fn dfs(d: &Vec<i64>, o: &mut Vec<bool>) -> Option<Vec<bool>> {
    if o.len() == d.len() - 1 {
        let mut r = d[0];
        for (&o_i, &d_i) in o.iter().zip(d.iter().skip(1)) {
            if o_i {
                r += d_i;
            } else {
                r -= d_i;
            }
        }
        return if r == 7 { Some(o.to_vec()) } else { None };
    }

    o.push(true);
    if let Some(o) = dfs(d, o) {
        return Some(o);
    }
    o.pop();
    o.push(false);
    if let Some(o) = dfs(d, o) {
        return Some(o);
    }
    o.pop();
    None
}

fn main() {
    input! {
        abcd: Chars
    };
    let d = abcd
        .iter()
        .map(|&c| c.to_digit(10).unwrap() as i64)
        .collect::<Vec<i64>>();
    match dfs(&d, &mut vec![]) {
        None => unreachable!(),
        Some(o) => {
            print!("{}", d[0]);
            for (&o_i, &d_i) in o.iter().zip(d.iter().skip(1)) {
                print!("{}{}", if o_i { "+" } else { "-" }, d_i);
            }
            println!("=7");
        }
    }
}
