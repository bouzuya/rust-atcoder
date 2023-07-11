use proconio::input;

fn main() {
    input! {
        _: usize,
        d: usize,
        k: usize,
        lr: [(i64, i64); d],
        st: [(i64, i64); k],
    };
    let mut cd = vec![];
    for &(s_i, _) in st.iter() {
        cd.push((s_i, None));
    }
    for (i, &(l_i, r_i)) in lr.iter().enumerate() {
        let d_i = i + 1;
        for j in 0..st.len() {
            let (s_j, t_j) = st[j];
            let (c_j, d_j) = cd[j];
            match d_j {
                Some(_) => {
                    // do nothing
                }
                None => {
                    if (l_i..=r_i).contains(&c_j) {
                        if s_j < t_j {
                            cd[j].0 = std::cmp::max(c_j, r_i);
                            cd[j].1 = if cd[j].0 >= t_j { Some(d_i) } else { None };
                        } else if s_j > t_j {
                            cd[j].0 = std::cmp::min(c_j, l_i);
                            cd[j].1 = if cd[j].0 <= t_j { Some(d_i) } else { None };
                        } else {
                            unreachable!();
                        }
                    }
                }
            }
        }
    }
    for &(_, d_i) in cd.iter() {
        match d_i {
            None => unreachable!(),
            Some(d_i) => println!("{}", d_i),
        }
    }
}
