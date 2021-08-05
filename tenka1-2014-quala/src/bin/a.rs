fn main() {
    let mut a = (1..=1000).map(|i| i.to_string()).collect::<Vec<String>>();
    a.sort();
    for a_i in a {
        println!("{}", a_i);
    }
}
