# WordleSolver

A fast command-line Wordle solver written in Rust. It suggests guesses by computing the information gain (entropy) of each candidate guess against the current candidate answer set and prunes the candidate list based on the feedback you enter. It supports an optional "hard mode" that enforces previously revealed constraints on subsequent guesses.

## Stack
- Language: Rust (edition = "2024")
- Notable library: rayon (parallel entropy calculation)

## What it does
WordleSolver loads Wordle answer and guess lists from the repository's `data/` directory, then interactively suggests the next best guess. You provide the feedback for the suggested guess using the letters G (green), Y (yellow) and B (gray/black), and the solver filters candidates until the answer is found.

## Files of interest
- `src/main.rs` — solver implementation and CLI loop
- `data/answers.txt` — canonical Wordle answers (used as the target set)
- `data/valid_guesses_extra.txt` and `data/allowed_guesses.txt` — additional guesses allowed by the solver
- `Cargo.toml` — build manifest (depends on `rayon`)

> Note: This repository currently does not include a LICENSE file. Add one if you intend to publish or relicense the code.

## How it works (short)
- Words are packed into a single u64 for compact and fast bitwise operations.
- For each candidate guess the solver computes the distribution of feedback patterns (3^5 = 243 possible patterns) against the current candidate answers.
- It computes the Shannon entropy for that distribution and selects the guess with the highest entropy as the recommendation.
- Entropy calculations run in parallel using rayon to take advantage of multiple CPU cores.
- In hard mode the solver filters valid guesses to those that respect previously revealed green/yellow constraints.

## Build and run
Prerequisites: Rust toolchain (nightly/stable compatible with `edition = "2024"`) and Cargo.

Build (recommended release build for speed):

```bash
# from repository root
cargo build --release
```

Run directly (interactive):

```bash
# run the compiled binary
cargo run --release
```

The program will prompt:

1. `Hard mode? (y/n):` — choose `y` to enforce Wordle hard-mode rules on subsequent guesses, or `n` to allow any guess from the list.
2. The solver prints a suggested guess and computed entropy, then asks for feedback.
3. Enter feedback as five characters using `G`/`g` for green, `Y`/`y` for yellow, and `B`/`b` for gray/black (e.g. `GbYBB` or `GYBBB`).

Example session:

```text
Hard mode? (y/n): n
Suggested Guess: SLATE (entropy: 4.1234, candidates left: 230, calculated in: 12.3ms)
Enter Feedback (G/Y/B x5):
GYBBB
Suggested Guess: ROUGH ...
```

If feedback is entered incorrectly the solver may print `No Candidates Left - Check Your Feedback Input`.

## Data
The solver uses the files in `data/`:
- `answers.txt` should contain one 5-letter answer per line.
- `valid_guesses_extra.txt` and `allowed_guesses.txt` contain additional valid guesses.

To use a different word list, replace these files (preserve one 5-letter word per line) or supply your own files with the same names.

## Implementation notes
- Key functions in `src/main.rs`:
  - `get_feedback(guess, answer)` — computes green/yellow/gray feedback for a guess against an answer.
  - `calculate_entropy(guess, candidates)` — computes the Shannon entropy of feedback patterns for `guess` given `candidates`.
  - `find_best_guess(guesses, candidates)` — parallel scan over guesses using rayon to pick the highest-entropy guess.
  - `filter_candidates(candidates, guess, observed_pattern)` — retains only answers consistent with the observed feedback.
  - `is_valid_hard_mode_guess(...)` — enforces hard-mode constraints when enabled.

- The solver packs 5 ASCII bytes into a u64 for compact comparisons and efficient bit shifts.

## Performance
- Parallel entropy computation (rayon) speeds up the recommendation step; for best responsiveness build with `--release`.
- The repository sets the `release` profile with `debug = true` in `Cargo.toml`.

## Contributing
- Bug reports, suggestions, and PRs are welcome.
- When opening an issue or PR, include: Rust toolchain version, OS, and a short reproduction (word lists used, input sequence).

## License
No LICENSE file is present in this repository. If you'd like this project to be open source, add a `LICENSE` file (e.g., MIT, Apache-2.0) to make the terms explicit.

---

If you'd like, I can:
- Add example session transcripts with real words from the `data/` files,
- Add a non-interactive mode that accepts a series of guesses+patterns as command-line arguments for scripting,
- Add a CONTRIBUTING.md and a chosen LICENSE file and commit them for you.
