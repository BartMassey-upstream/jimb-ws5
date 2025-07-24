# Word Square Generator
Jim Blandy 2025

A high-performance Rust implementation that generates all
possible 5×5 word squares from a dictionary of 5-letter
words.

## What is a Word Square?

A word square is a square grid of letters where both the
rows and columns form valid words. For example:

```
H E A R T  
E A R T H  
A R I S E  
R T S E S  
T H E S E  
```

In this 5×5 word square:
- **Rows**: HEART, EARTH, ARISE, RTSES, THESE
- **Columns**: HEART, EARTH, ARISE, RTSES, THESE

## Features

- **High Performance**: Optimized trie data structure with bit manipulation
- **Memory Efficient**: Compressed storage and cache-friendly access patterns
- **Fast Pruning**: Bitwise operations to eliminate invalid paths early
- **Complete Search**: Finds all possible word squares in the dictionary

## Algorithm Overview

1. **Dictionary Loading**: Loads 5-letter words from `usa_5.txt`
2. **Trie Construction**: Builds an optimized trie with bitmask-based character tracking
3. **Backtracking Search**: Fills the grid position by position using constraint propagation
4. **Constraint Checking**: Uses bitwise AND to find letters that satisfy both row and column constraints
5. **Early Pruning**: Eliminates invalid paths as soon as constraints cannot be satisfied

## Usage

```bash
# Build and run
cargo run

# Run with optimizations
cargo run --release
```

## Performance Characteristics

- **Time Complexity**: Exponential in worst case, but heavily pruned by constraints
- **Space Complexity**: O(dictionary size) for trie storage
- **Optimizations**:
  - Bit manipulation for fast character set operations
  - Compressed trie storage
  - Linear memory access patterns
  - Early constraint violation detection

## Implementation Details

### Trie Structure

The trie implementation (`src/trie.rs`) uses several optimizations:

- **Bitmask Members**: Each node stores a 32-bit mask indicating which letters can follow
- **Compressed Children**: Child nodes stored in insertion order rather than full 26-element arrays  
- **Bit Counting**: Uses `count_ones()` for fast index calculation

### Search Algorithm

The backtracking search (`src/main.rs`) operates by:

1. Filling positions left-to-right, top-to-bottom
2. At each position, computing the intersection of valid letters from row and column constraints
3. Recursively trying each valid letter and backtracking when no valid extensions exist

## Files

- `src/main.rs` - Main application and search algorithm
- `src/trie.rs` - Optimized trie data structure  
- `usa_5.txt` - Dictionary of 5-letter words
- `Cargo.toml` - Rust project configuration

## Dependencies

- `anyhow` - Error handling

## License

This work is made available under the "Apache 2.0 or MIT
License". See the file `LICENSE.txt` in this distribution for
license terms.

---
This README and other comments and documentation authored by
Claude Code Sonnet with minimal human intervention.
