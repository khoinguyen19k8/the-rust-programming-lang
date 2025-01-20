use std::collections::HashMap;

fn main() {
    // Vector
    println!("Vector");

    // let v_empty: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3, 4, 5];
    let mut v_mutable: Vec<i32> = Vec::new();
    v_mutable.push(5);
    v_mutable.push(6);
    v_mutable.push(7);
    v_mutable.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {third}");
    let third_with_option: Option<&i32> = v.get(2);
    match third_with_option {
        Some(third_with_option) => println!("The third element is {third_with_option}"),
        None => println!("There is no third element."),
    }

    // This won't work due to borrowing rule.
    // A vector is a contiguous block of memory. Pushing an element may involve deallocating and
    // allocating a new block of memory. first may keep pointing to an already deallocated memory block.
    // let mut v1 = vec![1,2,3,4,5];
    // let first = &v1[0];
    // v1.push(6);
    // println!("The first element is {first}");

    // Iterate over immutable references
    println!("----------");
    println!("Iterate over immutable references");
    for i in &v {
        println!("{i}");
    }
    // Iterate over mutable references
    for i in &mut v_mutable {
        *i += 50;
        println!("{i}");
    }

    // Using an Enum to Store Multiple Types
    println!("----------");
    println!("Using Enum to store multiple types");
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


    // Strings
    println!("----------");
    println!("String");
    let mut s = String::from("initial contents!");
    s.push_str("foo");
    println!("s is {s}");
    let mut s2 = String::from("foo");
    let bar = "bar";
    s2.push_str(bar);
    println!("s2 is {s2}");

    // String concatenation
    println!("----------");
    println!("String concatenation");
    let hello = String::from("Hello, ");
    let world = String::from("World!");
    let s3 = hello + &world;
    // println!("{hello}"); // This is invalid as s3 now owns the string
    println!("world string: {world}");
    println!("s3 string: {s3}");

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let tic_tac_toe = format!("{tic}-{tac}-{toe}");
    println!("{tic_tac_toe}");

    // Slicing string
    println!("----------");
    println!("Slicing string");
    let weird_s= "Здравствуйте";

    let s_slice = &weird_s[0..4];
    println!("{s_slice}");

    // Iterate over strings
    println!("----------");
    println!("Iterate over strings");
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }


    // Hash Map
    println!("----------");
    println!("Hash Map");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("score = {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Hash Maps and Ownership
    println!("----------");
    println!("Hash Maps and Ownership");

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // let field_name_value = field_name + &field_value;


    // Updating a Hash Map
    println!("----------");
    println!("Overwriting a value");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("Value for Blue key has been overwritten from 10 to 25 {scores:?}");

    println!("----------");
    println!("Adding a Key and Value Only If a Key Isn’t Present");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");

    println!("----------");
    println!("Updating a Value Based on the Old Value");
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count: &mut i32 = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
}
