use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; 2 * n],
    };
    let mut count = vec![0; 2 * n];
    let mut rank = (0..2 * n).collect::<Vec<usize>>();
    for i in 0..m {
        for k in 1..=n {
            let l = rank[2 * k - 1 - 1];
            let r = rank[2 * k - 1];
            let a_l = a[l][i];
            let a_r = a[r][i];
            match (a_l, a_r) {
                ('G', 'G') => {}
                ('G', 'C') => count[l] += 1,
                ('G', 'P') => count[r] += 1,
                ('C', 'G') => count[r] += 1,
                ('C', 'C') => {}
                ('C', 'P') => count[l] += 1,
                ('P', 'G') => count[l] += 1,
                ('P', 'C') => count[r] += 1,
                ('P', 'P') => {}
                (_, _) => unreachable!(),
            }
        }
        let mut next = count
            .iter()
            .copied()
            .enumerate()
            .collect::<Vec<(usize, i64)>>();
        next.sort_by_key(|&(index, win_count)| (-win_count, index));
        rank = next
            .iter()
            .copied()
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();
    }

    for a in rank {
        println!("{}", a + 1);
    }
}
