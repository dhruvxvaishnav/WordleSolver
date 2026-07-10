use rayon::prelude::*;
use std::time::Instant;

const ANSWER_RAW: &str = include_str!("../data/answers.txt");
const EXTRA_GUESSES_RAW: &str = include_str!("../data/valid_guesses_extra.txt");

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
enum LetterResult {
    Green,
    Yellow,
    Gray,
}

// Converts a 5-byte word array into a single packed u64
fn word_to_u64(word: &[u8; 5]) -> u64 {
    ((word[0] as u64) << 32)
        | ((word[1] as u64) << 24)
        | ((word[2] as u64) << 16)
        | ((word[3] as u64) << 8)
        | (word[4] as u64)
}

// Helper to print the u64 back as a readable string
fn u64_to_string(packed: u64) -> String {
    let bytes = [
        ((packed >> 32) & 0xFF) as u8,
        ((packed >> 24) & 0xFF) as u8,
        ((packed >> 16) & 0xFF) as u8,
        ((packed >> 8) & 0xFF) as u8,
        (packed & 0xFF) as u8,
    ];
    String::from_utf8(bytes.to_vec()).unwrap()
}

fn get_feedback(guess_packed: u64, answer_packed: u64) -> [LetterResult; 5] {
    // Unpack bytes on the fly via quick bitwise operations
    let guess = [
        ((guess_packed >> 32) & 0xFF) as u8,
        ((guess_packed >> 24) & 0xFF) as u8,
        ((guess_packed >> 16) & 0xFF) as u8,
        ((guess_packed >> 8) & 0xFF) as u8,
        (guess_packed & 0xFF) as u8,
    ];

    let answer = [
        ((answer_packed >> 32) & 0xFF) as u8,
        ((answer_packed >> 24) & 0xFF) as u8,
        ((answer_packed >> 16) & 0xFF) as u8,
        ((answer_packed >> 8) & 0xFF) as u8,
        (answer_packed & 0xFF) as u8,
    ];

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

fn to_u64_from_str(word: &str) -> u64 {
    let b = word.as_bytes();
    word_to_u64(&[b[0], b[1], b[2], b[3], b[4]])
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

fn calculate_entropy(guess: u64, candidates: &[u64]) -> f64 {
    let mut pattern_counts = [0usize; 243];

    for &answer in candidates {
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

fn find_best_guess(guesses: &[u64], candidates: &[u64]) -> (u64, f64) {
    guesses
        .par_iter()
        .map(|&guess| (guess, calculate_entropy(guess, candidates)))
        .reduce(|| (0u64, -1.0), |a, b| if a.1 > b.1 { a } else { b })
}

fn filter_candidates(
    candidates: &[u64],
    guess: u64,
    observed_pattern: &[LetterResult; 5],
) -> Vec<u64> {
    candidates
        .iter()
        .filter(|&&answer| get_feedback(guess, answer) == *observed_pattern)
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

fn is_valid_hard_mode_guess(
    guess_packed: u64,
    prev_guess_packed: u64,
    prev_pattern: &[LetterResult; 5],
) -> bool {
    let guess = [
        ((guess_packed >> 32) & 0xFF) as u8,
        ((guess_packed >> 24) & 0xFF) as u8,
        ((guess_packed >> 16) & 0xFF) as u8,
        ((guess_packed >> 8) & 0xFF) as u8,
        (guess_packed & 0xFF) as u8,
    ];
    let prev_guess = [
        ((prev_guess_packed >> 32) & 0xFF) as u8,
        ((prev_guess_packed >> 24) & 0xFF) as u8,
        ((prev_guess_packed >> 16) & 0xFF) as u8,
        ((prev_guess_packed >> 8) & 0xFF) as u8,
        (prev_guess_packed & 0xFF) as u8,
    ];

    for i in 0..5 {
        match prev_pattern[i] {
            LetterResult::Green => {
                if guess[i] != prev_guess[i] {
                    return false;
                }
            }
            LetterResult::Yellow => {
                if !guess.contains(&prev_guess[i]) {
                    return false;
                }
            }
            LetterResult::Gray => {}
        }
    }
    true
}

fn run_solver(all_guesses: &[u64], initial_answers: &[u64], hard_mode: bool) {
    let mut candidates: Vec<u64> = initial_answers.to_vec();
    let mut valid_guesses: Vec<u64> = all_guesses.to_vec();
    let mut prev_guess: Option<u64> = None;
    let mut prev_pattern: Option<[LetterResult; 5]> = None;

    loop {
        if candidates.len() == 1 {
            println!("Answer Found: {}", u64_to_string(candidates[0]));
            break;
        }

        if hard_mode {
            if let (Some(pg), Some(pp)) = (prev_guess, prev_pattern) {
                valid_guesses.retain(|&g| is_valid_hard_mode_guess(g, pg, &pp));
            }
        }

        let guess_pool = if hard_mode {
            &valid_guesses
        } else {
            all_guesses
        };

        // Track time taken to compute entropy for this turn
        let start_time = Instant::now();
        let (best_word, best_entropy) = find_best_guess(guess_pool, &candidates);
        let duration = start_time.elapsed();

        println!(
            "Suggested Guess: {} (entropy: {best_entropy:.4}, candidates left: {}, calculated in: {:?}",
            u64_to_string(best_word),
            candidates.len(),
            duration
        );

        println!("Enter Feedback (G/Y/B x5):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let pattern = parse_pattern(input.trim());

        candidates = filter_candidates(&candidates, best_word, &pattern);
        prev_guess = Some(best_word);
        prev_pattern = Some(pattern);

        if candidates.is_empty() {
            println!("No Candidates Left - Check Your Feedback Input");
            break;
        }
    }
}

fn main() {
    let answers: Vec<u64> = ANSWER_RAW.lines().map(to_u64_from_str).collect();
    let extra_guesses: Vec<u64> = EXTRA_GUESSES_RAW.lines().map(to_u64_from_str).collect();

    let mut all_guesses: Vec<u64> = Vec::new();
    all_guesses.extend(&answers);
    all_guesses.extend(&extra_guesses);

    println!("Hard mode? (y/n):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let hard_mode = input.trim().eq_ignore_ascii_case("y");
    run_solver(&all_guesses, &answers, hard_mode);
}
