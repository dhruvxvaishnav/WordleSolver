use rayon::prelude::*;

const ANSWER_RAW: &str = include_str!("../data/answers.txt");
const EXTRA_GUESSES_RAW: &str = include_str!("../data/valid_guesses_extra.txt");

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum LetterResult {
    Green,
    Yellow,
    Gray,
}

fn get_feedback(guess: &[u8; 5], answer: &[u8]) -> [LetterResult; 5] {
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

fn to_bytes(word: &str) -> [u8; 5] {
    let b = word.as_bytes();
    [b[0], b[1], b[2], b[3], b[4]]
}

fn pattern_to_index(pattern: &[LetterResult; 5]) -> usize {
    let mut index = 0;
    for &r in pattern.iter() {
        let digit = match r {
            LetterResult::Gray => 0,
            LetterResult::Yellow => 1,
            LetterResult::Green => 2,
        };
        index = index * 3 + digit;
    }
    index
}

fn calculate_entropy(guess: &[u8; 5], candidates: &[[u8; 5]]) -> f64 {
    let mut pattern_counts = [0usize; 243];

    for answer in candidates {
        let pattern = get_feedback(guess, answer);
        let index = pattern_to_index(&pattern);
        pattern_counts[index] += 1;
    }

    let total = candidates.len() as f64;
    let mut entropy = 0.0;

    for &count in pattern_counts.iter() {
        if count == 0 {
            continue;
        }
        let p = count as f64 / total;
        entropy -= p * p.log2();
    }

    entropy
}

fn find_best_guess(guesses: &[[u8; 5]], candidates: &[[u8; 5]]) -> ([u8; 5], f64) {
    guesses
        .par_iter()
        .map(|&guess| (guess, calculate_entropy(&guess, candidates)))
        .reduce(|| ([0u8; 5], -1.0), |a, b| if a.1 > b.1 { a } else { b })
}

fn filter_candidates(
    candidates: &[[u8; 5]],
    guess: &[u8; 5],
    observed_pattern: &[LetterResult; 5],
) -> Vec<[u8; 5]> {
    candidates
        .iter()
        .filter(|&&answer| get_feedback(guess, &answer) == *observed_pattern)
        .copied()
        .collect()
}

fn parse_pattern(input: &str) -> [LetterResult; 5] {
    let chars: Vec<char> = input.chars().collect();
    let mut pattern = [LetterResult::Gray; 5];

    for i in 0..5 {
        pattern[i] = match chars[i] {
            'G' | 'g' => LetterResult::Green,
            'Y' | 'y' => LetterResult::Yellow,
            'B' | 'b' => LetterResult::Gray,
            _ => panic!("Invalid Character in pattern"),
        }
    }

    pattern
}

fn run_solver(all_guesses: &[[u8; 5]], initial_answers: &[[u8; 5]]) {
    let mut candidates: Vec<[u8; 5]> = initial_answers.to_vec();

    loop {
        if candidates.len() == 1 {
            println!(
                "Answer Found: {}",
                std::str::from_utf8(&candidates[0]).unwrap()
            );
            break;
        }

        let (best_word, best_entropy) = find_best_guess(all_guesses, &candidates);
        println!(
            "Suggested guess: {} (entropy: {best_entropy}, candidates left: {})",
            std::str::from_utf8(&best_word).unwrap(),
            candidates.len()
        );

        println!("Enter Feedback (G/Y/B x5):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let pattern = parse_pattern(input.trim());

        candidates = filter_candidates(&candidates, &best_word, &pattern);

        if candidates.is_empty() {
            println!("No candidates left - Check your feedback input.");
            break;
        }
    }
}

fn main() {
    let answers: Vec<[u8; 5]> = ANSWER_RAW.lines().map(to_bytes).collect();
    let extra_guesses: Vec<[u8; 5]> = EXTRA_GUESSES_RAW.lines().map(to_bytes).collect();

    let mut all_guesses: Vec<[u8; 5]> = Vec::new();
    all_guesses.extend(&answers);
    all_guesses.extend(&extra_guesses);

    run_solver(&all_guesses, &answers);
}
