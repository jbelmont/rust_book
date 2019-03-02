fn main() {
    let months = ["January", "February", "March", "April", "May", "June", "July",
                "August", "September", "October", "November", "December"];
    for month in months.iter() {
        println!("month is: {}", month);
    }

    println!("{}", multiplier(5, 6));
}

fn multiplier(number: u32, multiplicand: u32) -> u32 {
    number * multiplicand
}