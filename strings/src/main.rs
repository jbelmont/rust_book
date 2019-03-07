fn main() {
    let mut s1 = String::from("one");
    let s2 = "two";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let strs = vec![
        String::from("one"),
        String::from("two"),
        String::from("three"),
    ];

    let s = format!("{}-{}-{}", strs[0], strs[1], strs[2]);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}