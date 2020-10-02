use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        m: usize,
    };

    let mut set = std::collections::BTreeSet::new();
    let mut a = vec![x];
    set.insert(x);
    for i in 0.. {
        let next = (a[i] * a[i]) % m;
        if set.insert(next) {
            a.push(next);
        } else {
            break;
        }
    }

    if n <= m {
        let ans = a[0..n].iter().sum::<usize>();
        println!("{}", ans);
        return;
    }
    let cycle_first_value = (a[a.len() - 1] * a[a.len() - 1]) % m;
    let cycle_start_index = a.iter().position(|&a_i| a_i == cycle_first_value).unwrap();
    let prefix = &a[0..cycle_start_index];
    let cycle = &a[cycle_start_index..];
    let prefix_len = prefix.len();
    let prefix_sum = prefix.iter().sum::<usize>();
    let cycle_len = cycle.len();
    let cycle_sum = a[cycle_start_index..].iter().sum::<usize>();
    let cycle_count = (n - prefix_len) / cycle_len;
    let suffix_len = n - prefix_len - cycle_count * cycle_len;
    let suffix_sum = cycle.iter().take(suffix_len).sum::<usize>();
    let ans = prefix_sum + cycle_sum * cycle_count + suffix_sum;
    println!("{}", ans);
}
