use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Reads lines from a file and returns them as a vector of strings.
///
/// # Arguments
///
/// * `file_path` - The path of the input file.
///
/// # Returns
///
/// A `Result` containing a vector of strings representing the lines read from the file, or an `std::io::Error` if there was an error reading the file.
fn read_lines_from_file(file_path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Read the lines from the file
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    Ok(lines)
}

/// Groups lines into subgroups of a given count.
///
/// # Arguments
///
/// * `lines` - The vector of lines to be grouped.
/// * `group_count` - The number of lines in each group.
///
/// # Returns
///
/// A vector of vectors, where each inner vector represents a group of lines.
fn group_lines_by_count(lines: Vec<String>, group_count: usize) -> Vec<Vec<String>> {
    let mut lines_grouped: Vec<Vec<String>> = Vec::new();
    let mut current_group: Vec<String> = Vec::new();

    for line in lines {
        current_group.push(line);

        if current_group.len() == group_count {
            lines_grouped.push(current_group);
            current_group = Vec::new();
        }
    }

    lines_grouped
}

/// Finds the unique character that is present in all the lines of the group.
///
/// # Arguments
///
/// * `group` - A slice of strings representing the group of lines.
///
/// # Returns
///
/// A vector containing the unique character that is present in all the lines of the group, or an empty vector if there is no common character.
fn find_common_chars_in_group(group: &[String]) -> Vec<char> {
    let mut char_counts: HashMap<char, usize> = HashMap::new();

    for line in group {
        let line_chars: HashSet<char> = line.chars().collect();

        for ch in line_chars {
            *char_counts.entry(ch).or_insert(0) += 1;
        }
    }

    let mut common_chars: Vec<char> = char_counts
        .iter()
        .filter(|(_, count)| **count == group.len())
        .map(|(&ch, _)| ch)
        .collect();

    common_chars.sort();

    common_chars
}

fn main() -> Result<(), std::io::Error> {
    let file_path = "input.txt";
    let lines = read_lines_from_file(file_path)?;

    let group_count = 3;
    let lines_grouped = group_lines_by_count(lines, group_count);

    let mut global_sum = 0;

    // Iterate over each group of lines
    for group in lines_grouped.iter() {
        // Find the common characters in the group
        let common_chars = find_common_chars_in_group(group);

        // Ensure there is only one character in common_chars
        if common_chars.len() == 1 {
            let char_value = match common_chars[0] {
                'a'..='z' => common_chars[0] as u32 - 'a' as u32 + 1,
                'A'..='Z' => common_chars[0] as u32 - 'A' as u32 + 27,
                _ => 0, // Assign 0 if the character is not a valid lowercase or uppercase letter
            };
            global_sum += char_value;
        }
    }

    println!("Global Sum: {}", global_sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_common_chars_in_group() {
        let group = vec![
            String::from("abc"),
            String::from("def"),
            String::from("ghi"),
        ];
        let common_chars = find_common_chars_in_group(&group);
        assert_eq!(common_chars, vec![]);

        let group = vec![
            String::from("abd"),
            String::from("abe"),
            String::from("abe"),
        ];
        let common_chars = find_common_chars_in_group(&group);
        assert_eq!(common_chars, vec!['a', 'b']);

        let group = vec![
            String::from("abc"),
            String::from("dbf"),
            String::from("gbi"),
        ];
        let common_chars = find_common_chars_in_group(&group);
        assert_eq!(common_chars, vec!['b']);
    }

    #[test]
    fn test_group_lines_by_count() {
        let lines = vec![
            String::from("abc"),
            String::from("def"),
            String::from("ghi"),
            String::from("jkl"),
            String::from("mno"),
            String::from("pqr"),
        ];
        let grouped_lines = group_lines_by_count(lines, 3);
        assert_eq!(
            grouped_lines,
            vec![
                vec![
                    String::from("abc"),
                    String::from("def"),
                    String::from("ghi")
                ],
                vec![
                    String::from("jkl"),
                    String::from("mno"),
                    String::from("pqr")
                ]
            ]
        );
    }

    #[test]
    fn test_read_lines_from_file() {
        let result = read_lines_from_file("input.test.txt");
        assert!(result.is_ok());

        let lines = result.unwrap();
        assert_eq!(lines.len(), 6);
        assert_eq!(lines[0], "abc");
        assert_eq!(lines[5], "pqr");
    }
}
