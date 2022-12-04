use utils::read_lines;

fn part1() {
    // Rock = A, X / Score = 1
    // Paper = B, Y / Score = 2
    // Scissors = C, Z Score = 3

    // Loss = 0
    // Draw = 3
    // Win = 6

    // A - X = 3 (Draw) + 1 (score of Rock)
    // A - Y = 6 (Win) + 2 (score of Paper)
    // A - Z = 0 (Loss) + 3 (Score of Scissors)

    // B - X = 0 (Loss) + 1 (score of Rock)
    // B - Y = 3 (Draw) + 2 (score of Paper)
    // B - Z = 6 (Win) + 3 (Score of Scissors)

    // C - X = 6 (Win) + 1 (score of Rock)
    // C - Y = 0 (Loss) + 2 (score of Paper)
    // C - Z = 3 (Draw) + 3 (Score of Scissors)
    let mut score_per_tour: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("day2/src/input1.txt") {
        for line in lines {
            if let Ok(round) = line {
                match round.as_str() {
                    "A X" => score_per_tour.push(4),
                    "A Y" => score_per_tour.push(8),
                    "A Z" => score_per_tour.push(3),
                    "B X" => score_per_tour.push(1),
                    "B Y" => score_per_tour.push(5),
                    "B Z" => score_per_tour.push(9),
                    "C X" => score_per_tour.push(7),
                    "C Y" => score_per_tour.push(2),
                    "C Z" => score_per_tour.push(6),
                    _ => println!("Unknown")
                }
            }
        }
    }
    println!("Answer part 1: {}", score_per_tour.iter().sum::<i32>())
}

fn part2() {
    // Rock = A / Score = 1
    // Paper = B / Score = 2
    // Scissors = C / Score = 3

    // Loss = X / Score = 0
    // Draw = Y / Score = 3
    // Win = Z / Score = 6

    // A - X = C => 0 (Loss) + 3 (score of Scissors)
    // A - Y = A => 3 (Draw) + 1 (score of Rock)
    // A - Z = B => 6 (Win) + 2 (Score of Paper)

    // B - X = A => 0 (Loss) + 1 (score of Rock)
    // B - Y = B => 3 (Draw) + 2 (score of Paper)
    // B - Z = C => 6 (Win) + 3 (Score of Scissors)

    // C - X = B => 0 (Loss) + 2 (score of Paper)
    // C - Y = C => 3 (Draw) + 3 (score of Scissors)
    // C - Z = A => 6 (Win) + 1 (Score of Paper)
    let mut score_per_tour: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("day2/src/input1.txt") {
        for line in lines {
            if let Ok(round) = line {
                match round.as_str() {
                    "A X" => score_per_tour.push(3),
                    "A Y" => score_per_tour.push(4),
                    "A Z" => score_per_tour.push(8),
                    "B X" => score_per_tour.push(1),
                    "B Y" => score_per_tour.push(5),
                    "B Z" => score_per_tour.push(9),
                    "C X" => score_per_tour.push(2),
                    "C Y" => score_per_tour.push(6),
                    "C Z" => score_per_tour.push(7),
                    _ => println!("Unknown")
                }
            }
        }
    }
    println!("Answer part 2: {}", score_per_tour.iter().sum::<i32>())
}

fn main() {
    part1();
    part2();
}
