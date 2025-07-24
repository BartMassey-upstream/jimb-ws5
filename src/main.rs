use std::path::PathBuf;

use anyhow::Error;

mod trie;
use trie::{Node, letters_in};

fn main() -> Result<(), Error> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("usa_5.txt");
    let all_words = std::fs::read_to_string(&path)?;
    let words: Vec<&str> = all_words.lines().collect();

    let mut dict = Node::default();
    for word in &words {
        dict.insert(word);
    }

    println!("dictionary contains {} words", dict.leaves);

    let mut square = vec![Cell::dummy(&dict); 25];
    let mut count = 0;
    search_from(&mut square, 0, &dict, &mut count);
    println!("Found {count} squares in total");

    Ok(())
}

fn search_from<'s, 'd: 's>(
    square: &'s mut [Cell<'d>],
    next: usize,
    root: &'d Node,
    count: &mut usize,
) {
    if next == 25 {
        *count += 1;
        for row in 0..5 {
            for col in 0..5 {
                print!("{} ", square[row * 5 + col].letter);
            }
            println!()
        }
        println!();
        return;
    }

    let up = if next >= 5 {
        square[next - 5].down
    } else {
        root
    };
    let left = if next % 5 > 0 {
        square[next - 1].right
    } else {
        root
    };
    let possible = up.members & left.members;
    for letter in letters_in(possible) {
        square[next] = Cell {
            letter,
            down: up.follow(letter).unwrap(),
            right: left.follow(letter).unwrap(),
        };
        search_from(square, next + 1, root, count);
    }
}

#[derive(Clone)]
struct Cell<'d> {
    letter: char,
    down: &'d Node,
    right: &'d Node,
}

impl<'d> Cell<'d> {
    fn dummy(root: &'d Node) -> Self {
        Self {
            letter: ' ',
            down: root,
            right: root,
        }
    }
}

