use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };
    let mut map = std::collections::HashMap::new();
    for s_i in s.iter() {
        *map.entry(s_i).or_insert(0) += 1;
    }

    let mut names = map
        .iter()
        .map(|(&name, &count)| (count, name))
        .collect::<Vec<(usize, &String)>>();
    names.sort();
    names.reverse();
    println!("{}", names.iter().next().unwrap().1);
}
