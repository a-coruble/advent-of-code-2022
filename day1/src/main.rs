use utils::read_lines;

fn main() {
    let mut total_calories_by_elf :Vec<i32> = Vec::new();
    let mut current_total_calories = 0;
    if let Ok(lines) = read_lines("day1/src/input1.txt") {
        for line in lines {
            if let Ok(calories) = line {
                if calories.trim().is_empty() {
                    total_calories_by_elf.push(current_total_calories);
                    current_total_calories = 0;
                } else {
                    current_total_calories += calories.parse::<i32>().unwrap();
                }
            }
        }
    }
    total_calories_by_elf.sort();
    println!("Answer Part 1: {}", total_calories_by_elf.last().unwrap());
    println!("Answer Part 2: {}", total_calories_by_elf.iter().rev().take(3).sum::<i32>());
}