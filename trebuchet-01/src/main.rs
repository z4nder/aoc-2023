use std::{collections::HashMap, fs};

fn main() {
    let file_content = read_file("input/input.txt");
    let result = sum_numbers(file_content);

    println!("Sum: {}", result)
}

fn create_map() -> HashMap<&'static str, char> {
    let mut terms = HashMap::new();

    terms.insert("one", '1');
    terms.insert("two", '2');
    terms.insert("three", '3');
    terms.insert("four", '4');
    terms.insert("five", '5');
    terms.insert("six", '6');
    terms.insert("seven", '7');
    terms.insert("eight", '8');
    terms.insert("nine", '9');
    terms.insert("1", '1');
    terms.insert("2", '2');
    terms.insert("3", '3');
    terms.insert("4", '4');
    terms.insert("5", '5');
    terms.insert("6", '6');
    terms.insert("7", '7');
    terms.insert("8", '8');
    terms.insert("9", '9');

    terms
}

fn find_first_number(string: &str) -> Option<char> {
    let terms = create_map();

    let mut search_term = String::new();

    for character in string.chars() {
        search_term.push(character);

        let value = find_term(terms.clone(), &search_term);

        match value {
            Some(value) => return Some(value),
            None => continue,
        }
    }

    None
}

fn find_last_number(string: &str) -> Option<char> {
    let terms = create_map();

    let mut search_term = String::new();

    for character in string.chars().rev() {
        search_term = format!("{}{}", character, search_term);

        let value = find_term(terms.clone(), &search_term);

        match value {
            Some(value) => return Some(value),
            None => continue,
        }
    }

    None
}

fn find_term(terms: HashMap<&str, char>, search: &String) -> Option<char> {
    for (key, value) in terms {
        if search.contains(key) {
            return Some(value);
        }
    }

    None
}

fn sum_numbers(file_content: String) -> i32 {
    let lines = file_content.lines();

    let mut sum = 0;

    for line in lines {
        let first_number = find_first_number(line).expect("Should have found a first number");
        let last_number = find_last_number(line).expect("Should have found a last number");

        let result: i32 = format!("{}{}", first_number, last_number)
            .parse()
            .expect("Error at parse number");

        sum += result;
    }

    sum
}

fn read_file(file_path: &str) -> String {
    return fs::read_to_string(file_path).expect("Should have been able to read the file");
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{find_first_number, find_last_number};

    #[test]
    fn test_find_first_number() {
        let mut hash = HashMap::new();

        hash.insert("two1nine", '2');
        hash.insert("eightwothree", '8');
        hash.insert("abcone2threexyz", '1');
        hash.insert("xtwone3four", '2');
        hash.insert("4nineeightseven2", '4');
        hash.insert("zoneight234", '1');
        hash.insert("7pqrstsixteen", '7');

        for (key, value) in hash {
            let result = find_first_number(key);
            assert_eq!(result.unwrap(), value);
        }
    }

    #[test]
    fn test_find_last_number() {
        let mut hash = HashMap::new();

        hash.insert("two1nine", '9');
        hash.insert("eightwothree", '3');
        hash.insert("abcone2threexyz", '3');
        hash.insert("xtwone3four", '4');
        hash.insert("4nineeightseven2", '2');
        hash.insert("zoneight234", '4');
        hash.insert("7pqrstsixteen", '6');

        for (key, value) in hash {
            let result = find_last_number(key);

            assert_eq!(result.unwrap(), value);
        }
    }
}
