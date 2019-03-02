fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 3
        }
    };
    assert_eq!(result, 30);

    let numbers = [10, 20, 30, 40, 50];
    let mut sum = 0;
    let mut index = 0;

    while index < 5 {
        sum += numbers[index];
        println!("{}", sum);
        index += 1;
    }

    sum = 0;
    for elem in numbers.iter() {
        sum += elem;
    }
    let average = sum / numbers.len();
    println!("{}", average);
}
