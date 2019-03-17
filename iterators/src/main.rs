struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 13, style: String::from("sandal") },
            Shoe { size: 10, style: String::from("boot") },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe { size: 10, style: String::from("sneaker") },
                Shoe { size: 10, style: String::from("boot") },
            ]
        );
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                        .map(|(a, b)| a * b)
                        .filter(|x| x % 3 == 0)
                        .sum();

        assert_eq!(18, sum);
    }
}

fn main() {
    let buffer: &mut [i32];
    let coefficients: [i64; 12];
    let qlp_shift: i16;

    for i in 12..buffer.len() {
        let prediction = coefficients.iter()
            .zip(&buffer[i - 12..i])
            .map(|(&c, &s)| c * s as i64)
            .sum::<i64>() >> qlp_shift;
        
        let delta = buffer[i];
        buffer[i] = prediction as i32 + delta;
    }
}