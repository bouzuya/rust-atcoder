use proconio::input;
use std::collections::{BTreeMap, VecDeque};

fn main() {
    input! {
        n: usize,
        lms: [(String, i64, String); n],
    }

    let mut lms2: Vec<(usize, i64, usize)> = vec![];
    let mut key_map: BTreeMap<String, usize> = BTreeMap::new();
    for (l_i, m_i, s_i) in lms {
        let len = key_map.len();
        let l = *key_map.entry(l_i).or_insert(len);
        let len = key_map.len();
        let s = *key_map.entry(s_i).or_insert(len);
        lms2.push((l, m_i, s));
    }
    let mut keys = vec![""; key_map.len()];
    for (s, &i) in key_map.iter() {
        keys[i] = s;
    }

    let mut e = vec![vec![]; keys.len()];
    for &(l_i, m_i, s_i) in lms2.iter() {
        e[l_i].push((true, m_i, s_i));
        e[s_i].push((false, m_i, l_i));
    }

    let mut values: Vec<Option<f64>> = vec![None; keys.len()];
    let mut q = VecDeque::new();
    values[0] = Some(1_f64);
    q.push_back(0);
    while let Some(u) = q.pop_front() {
        for &(dir, m_i, v) in e[u].iter() {
            if values[v].is_some() {
                continue;
            }
            let value_u = values[u].unwrap();
            let value_v = if dir {
                value_u / m_i as f64
            } else {
                value_u * m_i as f64
            };
            values[v] = Some(value_v);
            q.push_back(v);
        }
    }

    let mut max_position = 0;
    let mut max_value = values[0].unwrap();
    let mut min_position = 0;
    let mut min_value = values[0].unwrap();
    for (i, &v) in values.iter().enumerate() {
        if let Some(v) = v {
            if v > max_value {
                max_value = v;
                max_position = i;
            }
            if v < min_value {
                min_value = v;
                min_position = i;
            }
        }
    }

    println!(
        "1{}={}{}",
        keys[max_position],
        (max_value / min_value).round(),
        keys[min_position]
    );
}
