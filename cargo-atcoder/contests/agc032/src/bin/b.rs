use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let mut e = vec![];
    let groups = if n % 2 == 0 {
        // {1,n}, {2,n-1}, ...
        let mut groups = vec![vec![]; n / 2];
        for (i, g_i) in groups.iter_mut().enumerate() {
            g_i.push(i);
            g_i.push(n - 1 - i);
        }
        groups
    } else {
        // {1,n-1}, {2,n-2}, ... {n}
        let mut groups = vec![vec![]; n / 2 + 1];
        for (i, g_i) in groups.iter_mut().enumerate() {
            if i == n / 2 {
                g_i.push(n - 1);
            } else {
                g_i.push(i);
                g_i.push(n - 2 - i);
            }
        }
        groups
    };

    for (i, g_i) in groups.iter().enumerate() {
        for (j, g_j) in groups.iter().enumerate() {
            if i == j {
                continue;
            }
            for &g_ik in g_i.iter() {
                for &g_jl in g_j.iter() {
                    if g_ik >= g_jl {
                        continue;
                    }
                    e.push((g_ik, g_jl));
                }
            }
        }
    }
    e.sort();

    println!("{}", e.len());
    for (u_i, v_i) in e {
        println!("{} {}", u_i + 1, v_i + 1);
    }
}
