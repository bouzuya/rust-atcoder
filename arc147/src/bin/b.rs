use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut p: [Usize1; n],
    };

    let count_a = p
        .iter()
        .copied()
        .enumerate()
        .filter(|(i, p_i)| i % 2 != p_i % 2)
        .count();

    let mut ans = vec![];

    for i in 0..n {
        if p[i] % 2 == i % 2 {
            continue;
        }

        let mut r = i;
        while r >= 2 && (p[r - 2] % 2 == (r - 2) % 2) {
            let i = r - 2;
            p.swap(i, r);
            ans.push(('B', i + 1));
            r = i;
        }
    }

    for i in (0..n).step_by(2) {
        if p[i] % 2 == i % 2 {
            break;
        }

        p.swap(i, i + 1);
        ans.push(('A', i + 1));
    }

    for i in 0..n {
        if p[i] == i {
            continue;
        }
        let mut j = i + 1 + p[i + 1..].iter().copied().position(|p_j| p_j == i).unwrap();
        while i < j {
            let nj = j - 2;
            p.swap(nj, j);
            ans.push(('B', nj + 1));
            j = nj;
        }
    }

    assert_eq!(ans.iter().filter(|(t, _)| t == &'A').count(), count_a / 2);

    println!("{}", ans.len());
    for (t, i) in ans {
        println!("{} {}", t, i);
    }
}
