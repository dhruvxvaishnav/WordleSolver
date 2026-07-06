#[derive(Debug, PartialEq, Clone, Copy)]
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

fn main() {
    let result = get_feedback("crane", "react");
    println!("{:?}", result)
}
