use std::collections::HashSet;
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

/// Finds the unique character that is present in all the lines of the group, based on the provided scale.
///
/// # Arguments
///
/// * `group` - A slice of strings representing the group of lines.
///
/// # Returns
///
/// A vector containing the unique character that is present in all the lines of the group, or an empty vector if there is no common character.
fn find_common_chars_in_group(group: &[String]) -> Vec<char> {
    let mut common_chars: HashSet<char> = HashSet::new();
    let mut unique_chars: HashSet<char> = HashSet::new();

    // Iterate over each line in the group
    for line in group {
        let line_chars: HashSet<char> = line.chars().collect();

        // Add the characters from the first line to the unique_chars set
        // The unique_chars set will be used as the initial reference set
        if unique_chars.is_empty() {
            unique_chars.extend(line_chars.iter().cloned());
        } else {
            // Remove characters from the unique_chars set that are not present in the current line
            unique_chars.retain(|ch| line_chars.contains(ch));
        }

        // Add all the characters from the line to the common_chars set
        common_chars.extend(line_chars.iter().cloned());
    }

    // Filter the common_chars set by removing characters that are not in the unique_chars set
    common_chars.retain(|ch| unique_chars.contains(ch));

    // Convert the common_chars set into a sorted vector and return it
    let mut result: Vec<char> = common_chars.into_iter().collect();
    result.sort();

    result
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
        assert_eq!(common_chars, vec!['a', 'b', 'c']);

        let group = vec![
            String::from("abc"),
            String::from("def"),
            String::from("xyz"),
        ];
        let common_chars = find_common_chars_in_group(&group);
        assert_eq!(common_chars, vec![]);

        let group = vec![
            String::from("aabb"),
            String::from("bbcc"),
            String::from("ccdd"),
        ];
        let common_chars = find_common_chars_in_group(&group);
        assert_eq!(common_chars, vec!['b', 'c']);
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
        let result = read_lines_from_file("input.txt");
        assert!(result.is_ok());

        let lines = result.unwrap();
        assert_eq!(lines.len(), 6);
        assert_eq!(lines[0], "abc");
        assert_eq!(lines[5], "pqr");
    }
}
