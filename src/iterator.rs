pub fn which_end_with_m() {
    let lst = vec!["lorem", "ipsum", "dolor"];
    lst.iter()
        .filter(|item| item.ends_with('m'))
        .take(5)
        .collect()
}
