use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 50);

    let teams = vec![
        String::from("blue"),
        String::from("yellow"),
    ];

    let initial_scores = vec![10, 100];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("some color");
    let field_value = String::from("red");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 20);
    scores.insert(String::from("white"), 50);

    let team_name = String::from("blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    scores.entry(String::from("black")).or_insert(75);
    println!("{:?}", scores);

    let text = "The silent fox moved swiftly";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}