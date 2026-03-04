fn main() {
    let mut v = vec![2, 4, 6, 8];
    v = v.iter()
        .map(|x| x * 3)
        .filter(|x| *x > 10)
        .collect();
    println!("{:?}", v);
}
