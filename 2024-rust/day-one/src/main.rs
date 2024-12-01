use std::{error::Error, fs::read_to_string, iter::zip};

fn calculate_total_distance(list_one: &mut Vec<u32>, list_two: &mut Vec<u32>) -> u32 {
    list_one.sort();
    list_two.sort();
    let mut total_distance = 0;
    for (&x, &y) in zip(list_one.iter(), list_two.iter()) {
        total_distance += x.abs_diff(y);
    }
    total_distance
}
fn main() -> Result<(), Box<dyn Error>> {
    let file_name = String::from("input.txt");
    let mut first_list: Vec<u32> = Vec::new();
    let mut second_list: Vec<u32> = Vec::new();
    for line in read_to_string(file_name).unwrap().lines() {
        let split_line: Vec<&str> = line.split_whitespace().collect();
        let first_value = split_line
            .get(0)
            .ok_or(format!(
                "First Value Not Present in the line {:?}",
                split_line
            ))?
            .trim()
            .parse::<u32>()?;

        let second_value = split_line
            .get(1)
            .ok_or(format!(
                "Second Value Not Present in the line {:?}",
                split_line
            ))?
            .trim()
            .parse::<u32>()?;

        first_list.push(first_value);
        second_list.push(second_value);
    }

    let result = calculate_total_distance(&mut first_list, &mut second_list);
    println!("Final Result: {}", result);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_input() {
        let mut list_one = vec![3, 4, 2, 1, 3, 3];
        let mut list_two = vec![4, 3, 5, 3, 9, 3];
        let total_distance = calculate_total_distance(&mut list_one, &mut list_two);
        assert_eq!(total_distance, 11);
    }
}
