use std::collections::HashMap;

const ANSWER_RAW: &str = include_str!("../data/answers.txt");
const EXTRA_GUESSES_RAW: &str = include_str!("../data/valid_guesses_extra.txt");

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum LetterResult {
    Green,
    Yellow,
    Gray,
}

fn get_feedback(guess: &str, answer: &str) -> [LetterResult; 5] {
    let guess: Vec<char> = guess.chars().collect();
    let answer: Vec<char> = answer.chars().collect();
    let mut result = [LetterResult::Gray; 5];
    let mut answer_used = [false; 5];

    for i in 0..5 {
        if guess[i] == answer[i] {
            result[i] = LetterResult::Green;
            answer_used[i] = true;
        }
    }

    for i in 0..5 {
        if result[i] == LetterResult::Green {
            continue;
        }
        for j in 0..5 {
            if !answer_used[j] && guess[i] == answer[j] {
                result[i] = LetterResult::Yellow;
                answer_used[j] = true;
                break;
            }
        }
    }

    result
}

fn calculate_entropy(guess: &str, candidates: &[&str]) -> f64 {
    let mut pattern_counts: HashMap<[LetterResult; 5], usize> = HashMap::new();

    for &answer in candidates {
        let pattern = get_feedback(guess, answer);
        *pattern_counts.entry(pattern).or_insert(0) += 1;
    }

    let total = candidates.len() as f64;
    let mut entropy = 0.0;

    for &count in pattern_counts.values() {
        let p = count as f64 / total;
        entropy -= p * p.log2();
    }

    entropy
}

fn main() {
    let answers: Vec<&str> = ANSWER_RAW.lines().collect();
    let extra_guesses: Vec<&str> = EXTRA_GUESSES_RAW.lines().collect();

    let mut all_guesses: Vec<&str> = Vec::new();
    all_guesses.extend(&answers);
    all_guesses.extend(&extra_guesses);

    println!("Answers: {}", answers.len());
    println!("Total Guesses: {}", all_guesses.len());
}
