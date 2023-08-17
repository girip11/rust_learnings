// Given a list of integers, use a vector and return the median (when sorted,
// the value in the middle position) and mode (the value that occurs most often;
// a hash map will be helpful here) of the list.

fn find_median(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort();
    let count = numbers.len();
    let median_index = if count % 2 != 0 {
        count / 2
    } else {
        (count + 1) / 2
    };
    numbers[median_index]
}

// Convert strings to pig latin. The first consonant of each word is moved to
// the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words
// that start with a vowel have “hay” added to the end instead (“apple” becomes
// “apple-hay”). Keep in mind the details about UTF-8 encoding!

fn convert_pig_latin(given: &str) -> String {
    if given.len() == 0 {
        return String::new();
    }

    match given.chars().nth(0).unwrap() {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}-hay", given),
        _ => {
            format!(
                "{}-{}ay",
                &given.chars().skip(1).collect::<String>(),
                //&given.chars().take(1).last().unwrap(),
                &given.chars().next().unwrap(),
            )
        }
    }
}

// Using a hash map and vectors, create a text interface to allow a user to add
// employee names to a department in a company. For example, “Add Sally to
// Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all
// people in a department or all people in the company by department, sorted
// alphabetically.

fn main() {
    let mut numbers = vec![-5, 1, -4, 2, 5];
    println!("{}", find_median(&mut numbers));

    println!("{}", convert_pig_latin("first"));
}
