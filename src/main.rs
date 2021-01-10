mod maxwealth;

fn main() {
    let mut accounts: Vec<Vec<i32>> = Vec::new();
    accounts.push([1,2,3].to_vec());
    accounts.push([4,5,6].to_vec());
    accounts.push([7,8,9].to_vec());

    println!("\n\nMAXWEALTH");
    println!("Accounts:\n{:?}\n", accounts);
    println!("Max Wealth:\n{:?}\n", maxwealth::max_wealth(accounts));
}

