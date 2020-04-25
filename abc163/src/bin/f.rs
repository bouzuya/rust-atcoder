// https://atcoder.jp/contests/abc163/submissions/12204917
use proconio::input;
use proconio::marker::Usize1;

// 頂点数 n のときのパスの和を返す (u -> v と v -> u を重複と見なす)
fn f(n: usize) -> usize {
    n * (n + 1) / 2
}

// 部分木のサイズを countv に設定する
fn g(countv: &mut Vec<usize>, evv: &Vec<Vec<usize>>, v: usize, p: usize) {
    let mut count = 1;
    for &u in evv[v].iter() {
        if u == p {
            continue;
        }
        g(countv, evv, u, v);
        count += countv[u];
    }
    countv[v] = count;
}

fn h(
    av: &mut Vec<usize>,
    ccv: &mut Vec<usize>,
    cpv: &mut Vec<usize>,
    stackv: &mut Vec<Vec<usize>>,
    countv: &Vec<usize>,
    cv: &Vec<usize>,
    evv: &Vec<Vec<usize>>,
    v: usize,
    p: usize,
) {
    let c_v = cv[v];
    stackv[c_v].push(v);
    for &u in evv[v].iter() {
        if u == p {
            continue;
        }
        ccv[v] = countv[u];
        h(av, ccv, cpv, stackv, countv, cv, evv, u, v);
        av[c_v] -= f(ccv[v]);
    }
    stackv[c_v].pop();
    if let Some(&i_pc) = stackv[c_v].last() {
        ccv[i_pc] -= countv[v];
    } else {
        cpv[c_v] -= countv[v];
    }
}

fn main() {
    input! {
        n: usize,
        cv: [Usize1; n],
        abv: [(Usize1, Usize1); n - 1],
    };
    let mut evv = vec![vec![]; n];
    for (a, b) in abv {
        evv[a].push(b);
        evv[b].push(a);
    }

    let mut countv = vec![0_usize; n];
    g(&mut countv, &evv, 0, 0);

    let mut av = vec![f(n); n];
    let mut ccv = vec![0_usize; n];
    let mut cpv = vec![n; n];
    let mut stackv = vec![vec![]; n];
    h(
        &mut av,
        &mut ccv,
        &mut cpv,
        &mut stackv,
        &countv,
        &cv,
        &evv,
        0,
        0,
    );

    for (i, &a) in av.iter().enumerate() {
        println!("{}", a - f(cpv[i]));
    }
}
