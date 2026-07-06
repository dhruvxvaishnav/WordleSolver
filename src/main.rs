use std::collections::HashMap;

const ANSWER_RAW: &str = include_str!("../data/answers.txt");
const EXTRA_GUESSES_RAW: &str = include_str!("../data/valid_guesses_extra.txt");

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum LetterResult {
    Green,
    Yellow,
    Gray,
}

fn get_feedback(guess: &[u8], answer: &[u8]) -> [LetterResult; 5] {
    let mut result = [LetterResult::Gray; 5];
    let mut answer_used = [false; 5];

    for i in 0..5 {
        if guess[i] == answer[i] {
            result[i] = LetterResult::Green;
            answer_used[i] = true;
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

fn to_bytes(word: &str) -> [u8; 5] {
    let b = word.as_bytes();
    [b[0], b[1], b[2], b[3], b[4]]
}

fn calculate_entropy(guess: &[u8; 5], candidates: &[[u8; 5]]) -> f64 {
    let mut pattern_counts: HashMap<[LetterResult; 5], usize> = HashMap::new();

    for answer in candidates {
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

fn find_best_guess(guesses: &[[u8; 5]], candidates: &[[u8; 5]]) -> ([u8; 5], f64) {
    let mut best_word = [0u8; 5];
    let mut best_entropy = -1.0;

    for &guess in guesses {
        let entropy = calculate_entropy(&guess, candidates);
        if entropy > best_entropy {
            best_entropy = entropy;
            best_word = guess;
        }
    }

    (best_word, best_entropy)
}

fn main() {
    let answers: Vec<[u8; 5]> = ANSWER_RAW.lines().map(to_bytes).collect();
    let extra_guesses: Vec<[u8; 5]> = EXTRA_GUESSES_RAW.lines().map(to_bytes).collect();

    let mut all_guesses: Vec<[u8; 5]> = Vec::new();
    all_guesses.extend(&answers);
    all_guesses.extend(&extra_guesses);

    let (best_word, best_entropy) = find_best_guess(&all_guesses, &answers);

    let display_word = std::str::from_utf8(&best_word).unwrap_or("?????");
    println!("Best first guess: {display_word} (entropy: {best_entropy})");
}

