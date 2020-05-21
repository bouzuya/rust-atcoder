use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut is_back = true;
    let mut d = std::collections::VecDeque::new();
    for &a_i in a.iter() {
        if is_back {
            d.push_back(a_i);
        } else {
            d.push_front(a_i);
        }
        is_back = !is_back;
    }
    let b = if is_back {
        d.iter().collect::<Vec<_>>()
    } else {
        d.iter().rev().collect::<Vec<_>>()
    };
    for (i, &b_i) in b.iter().enumerate() {
        print!("{}{}", b_i, if i == b.len() - 1 { "\n" } else { " " });
    }
}
