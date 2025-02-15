// Static trie to help search for a word in a string
use std::collections::HashMap;
use once_cell::sync::Lazy;


fn map_word_to_value(word: &str) -> char {
    if word == "one" {
        return '1';
    } else if word == "two" {
        return '2';
    } else if word == "three" {
        return '3';
    } else if word == "four" {
        return '4';
    } else if word == "five" {
        return '5';
    } else if word == "six" {
        return '6';
    } else if word == "seven" {
        return '7';
    } else if word == "eight" {
        return '8';
    } else if word == "nine" {
        return '9';
    } else {
        return '0';
    }
}


#[derive(Debug, Default)]
pub struct CharacterTrieNode {
    pub children: HashMap<char, CharacterTrieNode>,
    pub value: char,
}

impl CharacterTrieNode {
    fn new() -> Self {
        CharacterTrieNode {
            children: HashMap::new(),
            value: '0',
        }
    }

    fn insert(&mut self, word: &str) {
        let mut current = self;
        for ch in word.chars() {
            current = current.children.entry(ch).or_insert_with(CharacterTrieNode::new);
        }
        current.value = map_word_to_value(word);
    }

    pub fn update_char(&self, ch: &char) -> Option<&CharacterTrieNode> {
        match self.children.get(&ch) {
            Some(child) => return Some(child),
            None => return Option::None,
        }
    }
}

// Use a trie, popularised by Edward Fredkin
// Using just the first two chracters would have been a more C like solution
// Use lazy static intialization to create immutable static at runtime
pub static TRIE: Lazy<CharacterTrieNode> = Lazy::new(|| {
    const WORDS: [&str; 9] = [
        "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine"
    ];

    let mut trie = CharacterTrieNode::new();
    for word in WORDS.iter() {
        trie.insert(word);
    }
    return trie;
});