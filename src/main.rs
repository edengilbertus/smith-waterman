const MATCH_SCORE: i32 = 2;
const MISMATCH_PENALTY: i32 = -1;
const GAP_PENALTY: i32 = -2;

fn smith_waterman(seq1: &str, seq2: &str) -> (i32, String, String) {
    let s1: Vec<char> = seq1.chars().collect();
    let s2: Vec<char> = seq2.chars().collect();
    let m = s1.len();
    let n = s2.len();

    // Build scoring matrix (m+1) x (n+1), initialized to 0
    let mut matrix = vec![vec![0i32; n + 1]; m + 1];
    let mut max_score = 0;
    let mut max_pos = (0, 0);

    for i in 1..=m {
        for j in 1..=n {
            let score = if s1[i - 1] == s2[j - 1] { MATCH_SCORE } else { MISMATCH_PENALTY };
            let diag = matrix[i - 1][j - 1] + score;
            let up   = matrix[i - 1][j] + GAP_PENALTY;
            let left = matrix[i][j - 1] + GAP_PENALTY;
            matrix[i][j] = *[0, diag, up, left].iter().max().unwrap();
            if matrix[i][j] > max_score {
                max_score = matrix[i][j];
                max_pos = (i, j);
            }
        }
    }

    // Traceback from max_score position
    let mut aligned1 = String::new();
    let mut aligned2 = String::new();
    let (mut i, mut j) = max_pos;

    while i > 0 && j > 0 && matrix[i][j] > 0 {
        let score = if s1[i - 1] == s2[j - 1] { MATCH_SCORE } else { MISMATCH_PENALTY };
        if matrix[i][j] == matrix[i - 1][j - 1] + score {
            aligned1.push(s1[i - 1]);
            aligned2.push(s2[j - 1]);
            i -= 1;
            j -= 1;
        } else if matrix[i][j] == matrix[i - 1][j] + GAP_PENALTY {
            aligned1.push(s1[i - 1]);
            aligned2.push('-');
            i -= 1;
        } else {
            aligned1.push('-');
            aligned2.push(s2[j - 1]);
            j -= 1;
        }
    }

    // Traceback builds strings in reverse
    let aligned1: String = aligned1.chars().rev().collect();
    let aligned2: String = aligned2.chars().rev().collect();

    (max_score, aligned1, aligned2)
}

fn main() {
    let seq1 = "ACGTACGT";
    let seq2 = "TACGTACC";

    let (score, aligned1, aligned2) = smith_waterman(seq1, seq2);

    println!("Sequence 1: {}", seq1);
    println!("Sequence 2: {}", seq2);
    println!("Best local alignment score: {}", score);
    println!("Aligned seq1: {}", aligned1);
    println!("Aligned seq2: {}", aligned2);
}
