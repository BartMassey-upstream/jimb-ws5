//! High-performance trie implementation for ASCII alphabetic strings.
//!
//! This module provides a specialized trie data structure optimized for storing
//! and querying 5-letter words containing only ASCII alphabetic characters.
//! The implementation uses bit manipulation techniques for maximum efficiency:
//!
//! - **Compressed storage**: Child nodes are stored in a compact array indexed by
//!   the count of set bits, rather than a full 26-element array.
//! - **Bit-parallel operations**: Character membership is tracked using a 32-bit
//!   bitmask, enabling fast intersection operations for word square generation.
//! - **Cache-friendly**: Minimal memory overhead and good locality of reference.
//!
//! The trie is particularly well-suited for applications like word square generation
//! where you need to efficiently find all possible characters that can extend both
//! row and column prefixes simultaneously.

/// A trie node optimized for storing 5-letter words with ASCII alphabetic characters.
/// 
/// Uses bit manipulation for efficient storage and lookup. Each node stores:
/// - A bitmask indicating which letters can follow this prefix
/// - A count of word endings that pass through this node
/// - Child nodes stored in a compressed array
#[derive(Default)]
pub struct Node {
    /// Bitmask where bit i represents whether letter (A + i) is present as a child.
    /// Uses positions 0-25 for A-Z respectively.
    pub members: u32,
    /// Count of complete words that pass through this node during insertion.
    /// Used for statistics about dictionary size.
    pub leaves: u32,
    /// Child nodes stored in insertion order, indexed by compressed member positions.
    pub children: Vec<Node>,
}

impl Node {
    /// Insert a word into the trie.
    /// 
    /// Each character in the word creates or traverses to a child node.
    /// The `leaves` counter is incremented at each node to track how many
    /// words pass through that node.
    pub fn insert(&mut self, s: &str) {
        let mut node = self;
        for ch in s.chars() {
            node.leaves += 1;
            let (bit, index) = node.bit_and_index(ch);
            // Create new child if this letter hasn't been seen before
            if node.members & bit == 0 {
                node.children.insert(index, Self::default());
                node.members |= bit;
            }
            node = &mut node.children[index];
        }
    }

    /// Follow a character to the corresponding child node.
    /// 
    /// Returns None if no child exists for this character.
    /// Used during word square generation to check if a prefix can be extended.
    pub fn follow(&self, ch: char) -> Option<&Node> {
        let (bit, index) = self.bit_and_index(ch);
        if self.members & bit == 0 {
            return None;
        }
        Some(&self.children[index])
    }

    /// Convert a character to its bit position and compressed array index.
    /// 
    /// Returns:
    /// - `bit`: The bitmask with only the character's bit set (1 << letter_index)
    /// - `index`: Position in the compressed children array
    /// 
    /// The index is calculated by counting how many bits are set before
    /// this character's bit position, which gives us the insertion order.
    fn bit_and_index(&self, ch: char) -> (u32, usize) {
        let bit = 1 << letter_index(ch);
        let index = (self.members & (bit - 1)).count_ones() as usize;
        (bit, index)
    }
}

/// Convert an ASCII alphabetic character to its index position (A=0, B=1, ..., Z=25).
/// 
/// Normalizes to uppercase before conversion. Panics if the character is not ASCII alphabetic.
pub fn letter_index(ch: char) -> u32 {
    assert!(ch.is_ascii_alphabetic());
    ch.to_ascii_uppercase() as u32 - b'A' as u32
}

/// Iterate over characters represented by set bits in a bitmask.
/// 
/// Given a bitmask where bit i represents letter (A + i), yields the corresponding
/// characters in order. Uses bit manipulation to efficiently find and clear the
/// lowest set bit on each iteration.
/// 
/// # Example
/// ```
/// let mask = (1 << 0) | (1 << 2); // Bits for 'A' and 'C'
/// let chars: Vec<char> = letters_in(mask).collect();
/// assert_eq!(chars, vec!['A', 'C']);
/// ```
pub fn letters_in(mut mask: u32) -> impl Iterator<Item = char> {
    std::iter::from_fn(move || {
        if mask == 0 {
            return None;
        }
        // Find the position of the lowest set bit
        let index = (mask & !(mask - 1)).trailing_zeros();
        // Clear the lowest set bit
        mask = mask & (mask - 1);
        // Convert bit position back to character
        Some(char::from_u32(b'A' as u32 + index).unwrap())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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
}