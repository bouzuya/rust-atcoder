use proconio::{input, marker::Usize1};

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let print = |e: &[i64]| {
        for (i, e_i) in e.iter().copied().enumerate() {
            print!("{}{}", e_i, if i == e.len() - 1 { "\n" } else { " " });
        }
    };
    let mut e = vec![];
    match a.cmp(&b) {
        std::cmp::Ordering::Equal => {
            for i in 0..a as i64 {
                let v = i + 1;
                e.push(v);
                e.push(-v);
            }
            e.sort();
            print(&e);
        }
        std::cmp::Ordering::Less => {
            let mut sum_b = 0_i64;
            for i in 0..b as i64 {
                let v = i + 1;
                sum_b += v;
                e.push(-v);
            }
            let mut sum_a = 0_i64;
            for i in 0..(a - 1) as i64 {
                let v = i + 1;
                sum_a += v;
                e.push(v);
            }
            e.push(sum_b - sum_a);
            e.sort();
            print(&e);
        }
        std::cmp::Ordering::Greater => {
            let mut sum_a = 0_i64;
            for i in 0..a as i64 {
                let v = i + 1;
                sum_a += v;
                e.push(v);
            }
            let mut sum_b = 0_i64;
            for i in 0..(b - 1) as i64 {
                let v = i + 1;
                sum_b += v;
                e.push(-v);
            }
            e.push(-(sum_a - sum_b));
            e.sort();
            print(&e);
        }
    }
}
