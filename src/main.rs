//! Word Square Generator
//!
//! This program generates all possible 5x5 word squares from a dictionary of 5-letter words.
//! A word square is a square grid of letters where both the rows and columns form valid words.
//!
//! The algorithm uses a high-performance trie data structure with bit manipulation to efficiently
//! search for valid letter combinations. It employs backtracking to explore all possibilities
//! while pruning invalid paths early using bitwise intersection of possible characters.
//!
//! # Algorithm Overview
//!
//! 1. Load dictionary into a specialized trie structure
//! 2. Use backtracking to fill the 5x5 grid position by position
//! 3. At each position, find letters that can extend both the current row and column prefixes
//! 4. Use bitwise AND operations to quickly compute valid letter intersections
//! 5. Recursively try each valid letter and backtrack when no more valid letters exist
//!
//! # Performance Characteristics
//!
//! - **Memory efficient**: Compressed trie storage with bit manipulation
//! - **Fast pruning**: Bitwise operations to eliminate invalid paths early
//! - **Cache friendly**: Linear memory access patterns in the search

use std::{path::PathBuf, env};

use anyhow::Error;

mod trie;
use trie::{Node, letters_in};

/// Main entry point for the word square generator.
/// 
/// Loads a dictionary of 5-letter words, builds a trie, and then searches for all
/// possible 5x5 word squares where both rows and columns form valid dictionary words.
fn main() -> Result<(), Error> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let json_output = args.contains(&"--json".to_string());

    // Load dictionary from the usa_5.txt file in the project directory
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("usa_5.txt");
    let all_words = std::fs::read_to_string(&path)?;
    let words: Vec<&str> = all_words.lines().collect();

    // Build trie from dictionary words
    let mut dict = Node::default();
    for word in &words {
        dict.insert(word);
    }

    if !json_output {
        println!("dictionary contains {} words", dict.leaves);
    }

    // Initialize empty 5x5 grid and start search
    let mut square = vec![Cell::dummy(&dict); 25];
    let mut squares = Vec::new();
    let mut count = 0;
    
    search_from(&mut square, 0, &dict, &mut count, if json_output { Some(&mut squares) } else { None });
    
    if json_output {
        println!("{}", serde_json::to_string(&squares)?);
    } else {
        println!("Found {count} squares in total");
    }

    Ok(())
}

/// Recursive backtracking search to fill the word square.
/// 
/// Fills positions left-to-right, top-to-bottom (reading order). At each position,
/// finds letters that can extend both the current row prefix (from left) and the
/// current column prefix (from above) into valid dictionary words.
/// 
/// # Arguments
/// 
/// * `square` - Mutable array of 25 cells representing the 5x5 grid
/// * `next` - Index of the next position to fill (0-24)
/// * `root` - Root of the dictionary trie (used for starting new words)
/// * `count` - Mutable counter of complete word squares found
/// 
/// # Algorithm Details
/// 
/// The key insight is that each cell must satisfy constraints from both directions:
/// - **Row constraint**: Must extend the prefix formed by cells to the left
/// - **Column constraint**: Must extend the prefix formed by cells above
/// 
/// We use bitwise AND of the two trie nodes' member sets to find valid letters
/// that satisfy both constraints simultaneously.
fn search_from<'s, 'd: 's>(
    square: &'s mut [Cell<'d>],
    next: usize,
    root: &'d Node,
    count: &mut usize,
    mut squares: Option<&mut Vec<Vec<String>>>,
) {
    // Base case: filled entire 5x5 grid
    if next == 25 {
        *count += 1;
        
        if let Some(squares_vec) = squares {
            // JSON output: collect the square as a vector of strings
            let mut square_words = Vec::new();
            for row in 0..5 {
                let mut word = String::new();
                for col in 0..5 {
                    word.push(square[row * 5 + col].letter.to_ascii_lowercase());
                }
                square_words.push(word);
            }
            squares_vec.push(square_words);
        } else {
            // Text output: print the completed word square
            for row in 0..5 {
                for col in 0..5 {
                    print!("{}", square[row * 5 + col].letter.to_ascii_lowercase());
                }
                println!()
            }
            println!();
        }
        return;
    }

    // Get trie node for column constraint (prefix from above)
    let up = if next >= 5 {
        // Use the trie node from the cell directly above
        square[next - 5].down
    } else {
        // Top row: start from root (no prefix constraint)
        root
    };
    
    // Get trie node for row constraint (prefix from left)
    let left = if next % 5 > 0 {
        // Use the trie node from the cell to the left
        square[next - 1].right
    } else {
        // Leftmost column: start from root (no prefix constraint)
        root
    };
    
    // Find letters that satisfy both row and column constraints
    // Bitwise AND gives us the intersection of possible characters
    let possible = up.members & left.members;
    
    // Try each valid letter
    for letter in letters_in(possible) {
        square[next] = Cell {
            letter,
            // Follow the letter down each trie path
            down: up.follow(letter).unwrap(),
            right: left.follow(letter).unwrap(),
        };
        // Recurse to fill the next position
        search_from(square, next + 1, root, count, squares.as_deref_mut());
    }
}

/// Represents a single cell in the 5x5 word square grid.
/// 
/// Each cell tracks not only its letter, but also the current state in the trie
/// for both the row word (extending rightward) and column word (extending downward).
/// This allows efficient constraint checking during backtracking search.
#[derive(Clone)]
struct Cell<'d> {
    /// The letter placed in this cell
    letter: char,
    /// Trie node representing the current state for the column word (going down)
    down: &'d Node,
    /// Trie node representing the current state for the row word (going right)  
    right: &'d Node,
}

impl<'d> Cell<'d> {
    /// Create a dummy cell used for initialization.
    /// 
    /// Uses a space character as a placeholder and points both trie references
    /// to the root, indicating no prefix constraints yet.
    fn dummy(root: &'d Node) -> Self {
        Self {
            letter: ' ',
            down: root,
            right: root,
        }
    }
}

