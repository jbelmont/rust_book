struct Context<'a>(&'a str);

struct Parser<'c, 's: 'c> {
    context: &'c Context<'s>,
}

trait Red {}

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> {}

fn main() {
    let num = 5;
    
    let obj = Box::new(Ball {
        diameter: &num
    }) as Box<dyn Red>;
}