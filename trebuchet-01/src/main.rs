use std::fs;

fn main() {
    // --snip--
    let file_path = "input/input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines = contents.lines();

    let mut sum = 0;
    for line in lines {
        let numbers_vec: Vec<char> = line
            .chars()
            .filter(|c| c.is_numeric())
            // .map(|c| c.to_digit(10).expect("Error at map to digit"))
            .collect();

        if numbers_vec.len() == 1 {
            let first_digit = numbers_vec.first().expect("Error at get first digit");
            let value: i32 = format!("{}{}", first_digit, first_digit)
                .parse()
                .expect("Error at parse value to i32");
            sum += value;

            continue;
        }

        let first_digit = numbers_vec.first().expect("Error at get first digit");
        let last_digit = numbers_vec.last().expect("Error at get last digit");
        let value: i32 = format!("{}{}", first_digit, last_digit)
            .parse()
            .expect("Error at parse value to i32");

        println!("Value: {}", value);
        sum += value;
    }

    println!("Sum: {}", sum)
}
