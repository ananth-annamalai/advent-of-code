use std::{collections::HashMap, error::Error, fs::read_to_string, iter::zip};

fn calculate_total_distance(list_one: &Vec<u32>, list_two: &Vec<u32>) -> u32 {
    let mut sorted_first_list = list_one.clone();
    let mut sorted_second_list = list_two.clone();
    sorted_first_list.sort();
    sorted_second_list.sort();
    let mut total_distance = 0;
    for (&x, &y) in zip(sorted_first_list.iter(), sorted_second_list.iter()) {
        total_distance += x.abs_diff(y);
    }
    total_distance
}

fn calculate_similarity_score(list_one: &Vec<u32>, list_two: &Vec<u32>) -> u32 {
    let mut second_list_counter = HashMap::new();
    let mut similarity_score = 0;

    for &x in list_two {
        *second_list_counter.entry(x).or_insert(0) += 1;
    }

    for &x in list_one {
        if let Some(count) = second_list_counter.get(&x) {
            similarity_score += count * x;
        }
    }

    similarity_score
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

    let part_one_result = calculate_total_distance(&first_list, &second_list);
    println!("Part One Result: {}", part_one_result);

    let part_two_result = calculate_similarity_score(&first_list, &second_list);
    println!("Part Two Result: {}", part_two_result);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_input_part_one() {
        let mut list_one = vec![3, 4, 2, 1, 3, 3];
        let mut list_two = vec![4, 3, 5, 3, 9, 3];
        let total_distance = calculate_total_distance(&mut list_one, &mut list_two);
        assert_eq!(total_distance, 11);
    }

    #[test]
    fn test_basic_input_part_two() {
        let mut list_one = vec![3, 4, 2, 1, 3, 3];
        let mut list_two = vec![4, 3, 5, 3, 9, 3];
        let total_distance = calculate_similarity_score(&mut list_one, &mut list_two);
        assert_eq!(total_distance, 31);
    }
}
