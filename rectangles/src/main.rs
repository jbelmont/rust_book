fn main() {
    let width = 30;
    let height = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );

    let rect = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect)
    );

    let rect = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect)
    );

    println!("{:#?}", rect);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}