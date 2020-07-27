use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
    };
    let mut l = vec![];
    l.push((1_usize, 1_usize));
    for i in 1..=n {
        l.push((l[i - 1].0 * 2 + 3, l[i - 1].1 * 2 + 1));
    }

    let mut count = 0;
    let mut level = n;
    let mut offset = 0;
    loop {
        let m = l[level].0 / 2 + 1;
        if x == 1 + offset {
            count += if level == 0 { 1 } else { 0 };
            break;
        } else if x == l[level].0 + offset {
            count += l[level].1;
            break;
        } else if x == m + offset {
            count += l[level - 1].1 + 1;
            break;
        } else if m + offset > x {
            offset += 1;
            level -= 1;
        } else if x > m + offset {
            count += l[level - 1].1 + 1;
            offset += m;
            level -= 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
