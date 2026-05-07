// https://leetcode.com/problems/implement-trie-prefix-tree

#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end_of_word: bool,
}

pub struct Trie {
    root: Box<TrieNode>,
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: Box::default(),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut current = self.root.as_mut();
        for c in word.bytes() {
            let idx = (c - b'a') as usize;
            let child = current.children[idx].get_or_insert_with(Box::default);
            current = child.as_mut();
        }

        current.is_end_of_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut current = self.root.as_ref();
        for c in word.bytes() {
            let idx = (c - b'a') as usize;
            if let Some(child) = current.children[idx].as_ref() {
                current = child;
            } else {
                return false;
            }
        }

        current.is_end_of_word
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut current = self.root.as_ref();
        for c in prefix.bytes() {
            let idx = (c - b'a') as usize;
            if let Some(child) = current.children[idx].as_ref() {
                current = child;
            } else {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn example_one() {
        let mut trie = Trie::new();
        trie.insert("apple".to_owned());

        assert!(trie.search("apple".to_owned()));
        assert!(!trie.search("app".to_owned()));
        assert!(trie.starts_with("app".to_owned()));

        trie.insert("app".to_owned());
        assert!(trie.search("app".to_owned()));
    }
}
