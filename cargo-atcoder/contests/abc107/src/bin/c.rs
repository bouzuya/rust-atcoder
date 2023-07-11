use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: [i64; n],
    };

    let mut minus = vec![];
    let mut zero = vec![];
    let mut plus = vec![];
    for x_i in x.iter().copied() {
        match x_i.cmp(&0) {
            std::cmp::Ordering::Less => minus.push(x_i),
            std::cmp::Ordering::Equal => zero.push(x_i),
            std::cmp::Ordering::Greater => plus.push(x_i),
        }
    }
    minus.reverse();

    let k = if zero.is_empty() { k } else { k - 1 };

    if k == 0 {
        println!("0");
        return;
    }

    let mut ans = 1 << 60;
    for cm in 0..=k {
        if cm > minus.len() {
            continue;
        }
        let cp = k - cm;
        if cp > plus.len() {
            continue;
        }
        let d_minus = if cm == 0 { 0 } else { minus[cm - 1].abs() };
        let d_plus = if cp == 0 { 0 } else { plus[cp - 1] };
        ans = ans.min(d_minus * 2 + d_plus).min(d_minus + d_plus * 2);
    }

    println!("{}", ans);
}
