use proconio::input;

fn swap(a: &mut Vec<usize>, s: &mut std::collections::BTreeSet<usize>, i: usize) {
    a.swap(i, i + 1);
    if a[i] > a[i + 1] {
        s.insert(i);
    } else {
        s.remove(&i);
    }
    if 0 <= (i as i64 - 1) {
        if a[i - 1] > a[i] {
            s.insert(i - 1);
        } else {
            s.remove(&(i - 1));
        }
    }
    if i + 2 < a.len() {
        if a[i + 1] > a[i + 2] {
            s.insert(i + 1);
        } else {
            s.remove(&(i + 1));
        }
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        txy: [(u8, usize, usize); q],
    };
    let mut a = (1..=n).collect::<Vec<_>>();
    let mut s = std::collections::BTreeSet::new();
    for &(t_i, x_i, y_i) in txy.iter() {
        match t_i {
            1 => {
                swap(&mut a, &mut s, x_i - 1);
            }
            2 => {
                while let Some(i) = s.range(x_i - 1..y_i - 1).take(1).map(|&x| x).next() {
                    swap(&mut a, &mut s, i);
                }
            }
            _ => unreachable!(),
        }
    }
    for (i, a_i) in a.iter().enumerate() {
        print!("{}{}", a_i, if i < n - 1 { " " } else { "\n" });
    }
}
