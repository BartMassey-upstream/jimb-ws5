use std::path::PathBuf;

use anyhow::Error;

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

#[derive(Default)]
struct Node {
    members: u32,
    leaves: u32,
    children: Vec<Node>,
}

impl Node {
    fn insert(&mut self, s: &str) {
        let mut node = self;
        for ch in s.chars() {
            node.leaves += 1;
            let (bit, index) = node.bit_and_index(ch);
            if node.members & bit == 0 {
                node.children.insert(index, Self::default());
                node.members |= bit;
            }
            node = &mut node.children[index];
        }
    }

    fn follow(&self, ch: char) -> Option<&Node> {
        let (bit, index) = self.bit_and_index(ch);
        if self.members & bit == 0 {
            return None;
        }
        Some(&self.children[index])
    }

    fn bit_and_index(&self, ch: char) -> (u32, usize) {
        let bit = 1 << letter_index(ch);
        let index = (self.members & (bit - 1)).count_ones() as usize;
        (bit, index)
    }
}

fn letter_index(ch: char) -> u32 {
    assert!(ch.is_ascii_alphabetic());
    ch.to_ascii_uppercase() as u32 - b'A' as u32
}

fn letters_in(mut mask: u32) -> impl Iterator<Item = char> {
    std::iter::from_fn(move || {
        if mask == 0 {
            return None;
        }
        let index = (mask & !(mask - 1)).trailing_zeros();
        mask = mask & (mask - 1);
        Some(char::from_u32(b'A' as u32 + index).unwrap())
    })
}

#[test]
fn insertions() {
    let mut root = Node::default();
    root.insert("foo");
    root.insert("bar");
    root.insert("baz");

    assert_eq!(
        root.members,
        (1 << letter_index('f')) | (1 << letter_index('b'))
    );
    assert_eq!(root.leaves, 3);
    assert_eq!(root.children[0].members, (1 << letter_index('a')));
    assert_eq!(root.children[0].leaves, 2);
    assert_eq!(root.children[1].members, (1 << letter_index('o')));
    assert_eq!(root.children[1].leaves, 1);
}
