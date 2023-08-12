use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut users = vec![vec![]; 37];
    let mut a = vec![];
    for i in 0..n {
        input! {
            c_i: usize,
            a_i: [usize; c_i],
        }
        for a_ij in a_i.iter().copied() {
            users[a_ij].push(i);
        }
        a.push(a_i);
    }
    input! {
        x: usize,
    }

    if users[x].is_empty() {
        println!("0");
        return;
    }

    let lens = users[x]
        .iter()
        .copied()
        .map(|i| (i, a[i].len()))
        .collect::<Vec<(usize, usize)>>();
    let min_len = *lens.iter().map(|(_, x)| x).min().unwrap();
    let mut ans = lens
        .iter()
        .filter(|(_, l)| l == &min_len)
        .map(|&(i, _)| i)
        .collect::<Vec<usize>>();
    ans.sort();
    println!("{}", ans.len());
    for i in ans {
        println!("{}", i + 1);
    }
}
