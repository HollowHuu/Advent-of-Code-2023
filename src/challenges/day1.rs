use std::collections::HashMap;


pub fn part1() -> i32 {
    let input = include_str!("input/day1.txt");
    let mut sum = 0;
 
    for line in input.lines() {
        let mut numbers: Vec<String> = vec![];

        for char in line.chars() {
            if char.is_numeric() {
                numbers.push(char.to_string());
            }
        }

        let string_num = numbers[0].to_string() + numbers[numbers.len() - 1].as_str();
        let num: i32 = string_num.parse().unwrap();
        sum += num;
    }

    sum
}

// Not working yet
pub fn part2() -> i32 {
    let number_map: HashMap<String, i32> = HashMap::from([
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9)
    ]);

    let input = include_str!("input/day1.txt");
    let mut sum = 0;

    for line in input.lines() {
        let mut numbers: Vec<String> = vec![];
        let mut chars: Vec<String> = vec![];

        for char in line.chars() {
            if char.is_numeric() {
                numbers.push(char.to_string());
                chars.clear();
            } 
            else if char.is_alphabetic() {
                chars.push(char.to_string());
                if number_map.contains_key(&chars.join("")) {
                    numbers.push(number_map.get(&chars.join("")).unwrap().to_string());
                    chars.clear();
                }
            }
        }

        let string_num = numbers[0].to_string() + numbers[numbers.len() - 1].as_str();
        let num: i32 = string_num.parse().unwrap();
        sum += num;
    }

    

    return sum;
}