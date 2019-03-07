fn main() {
    let numbers = vec![
        1, 2, 3, 4, 5
    ];

    let third: &i32 = &numbers[2];
    println!("The third element is {}", third);

    match numbers.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];

    let avg = average(vec![1, 2, 3, 4, 5]);
    println!("{}", avg);
}

fn average(numbers: Vec<u32>) -> f32 {
    let mut sum = 0;
    for num in numbers.iter() {
        sum += num;
    }
    sum as f32 / numbers.len() as f32
}

enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}