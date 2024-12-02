use std::collections::HashMap;
use crate::scanner::tokens::TokenType;

pub trait Trie {
    fn insert(&mut self, key: &str, value: TokenType);
    fn query(&self, word: &str) -> TokenType;
}

pub struct TrieNode {
    ch: HashMap<char, TrieNode>,
    leaf: bool,
    token_type: TokenType,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            ch: HashMap::new(),
            leaf: false,
            token_type: TokenType::None,
        }
    }

    pub fn add_node(&mut self, key: char) {
        self.ch.insert(key, TrieNode::new());
    }

    pub fn set_leaf(&mut self) {
        self.leaf = true;
    }

    pub fn set_token_type(&mut self, token: TokenType) {
        self.token_type = token;
    }
}

pub struct KeywordTrie {
    root: TrieNode,
}

pub struct KeywordTrieChecker<'a> {
    trie: &'a KeywordTrie,
    curr: Option<&'a TrieNode>,
    check: bool,
}

impl Trie for KeywordTrie {
    fn insert(&mut self, key: &str, value: TokenType) {
        let mut cur = &mut self.root;
        for c in key.chars() {
            if !cur.ch.contains_key(&c) {
                cur.add_node(c);
            }
            cur = cur.ch.get_mut(&c).unwrap();
        }
        cur.set_leaf();
        cur.set_token_type(value);
    }

    fn query(&self, word: &str) -> TokenType {
        let mut cur = &self.root;
        for c in word.chars() {
            match cur.ch.get(&c) {
                Some(node) => cur = node,
                None => return TokenType::None,
            }
        }
        if cur.leaf {
            cur.token_type
        } else {
            TokenType::None
        }
    }
}

impl KeywordTrie {
    pub fn new() -> Self {
        let mut keyword_trie = KeywordTrie {
            root: TrieNode::new(),
        };

        keyword_trie.insert("break", TokenType::Break);
        keyword_trie.insert("continue", TokenType::Continue);
        keyword_trie.insert("if", TokenType::If);
        keyword_trie.insert("else", TokenType::Else);
        keyword_trie.insert("for", TokenType::For);
        keyword_trie.insert("while", TokenType::While);
        keyword_trie.insert("return", TokenType::Return);

        keyword_trie
    }

    pub fn into_checker(&self) -> KeywordTrieChecker {
        KeywordTrieChecker {
            trie: self,
            curr: Some(&self.root),
            check: false,
        }
    }
}

impl<'a> KeywordTrieChecker<'a> {

    pub fn query(&self, word: &str) -> TokenType {
        self.trie.query(word)
    }

    pub fn update(&mut self, c: char) {
        self.check = true;
        if let Some(cur) = self.curr {
            self.curr = cur.ch.get(&c);
        }
    }

    pub fn check(&mut self) -> TokenType {
        let mut return_value = TokenType::None;
        if let Some(cur) = self.curr {
            if cur.leaf {
                return_value = cur.token_type;
            }
        }
        self.curr = Some(&self.trie.root);
        self.check = false;
        return_value
    }

    pub fn checkable(&self) -> bool {
        self.check
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let mut trie = KeywordTrie::new();

        trie.insert("break", TokenType::Break);
        trie.insert("continue", TokenType::Continue);
        trie.insert("if", TokenType::If);
        trie.insert("else", TokenType::Else);
        trie.insert("for", TokenType::For);
        trie.insert("while", TokenType::While);
        trie.insert("return", TokenType::Return);

        assert_eq!(trie.query("break"), TokenType::Break);
        assert_eq!(trie.query("continue"), TokenType::Continue);
        assert_eq!(trie.query("if"), TokenType::If);
        assert_eq!(trie.query("else"), TokenType::Else);
        assert_eq!(trie.query("for"), TokenType::For);
        assert_eq!(trie.query("while"), TokenType::While);
        assert_eq!(trie.query("return"), TokenType::Return);

        assert_eq!(trie.query("i"), TokenType::None);
        assert_eq!(trie.query("whilea"), TokenType::None);
        assert_eq!(trie.query("esle"), TokenType::None);
        assert_eq!(trie.query("retun"), TokenType::None);
        assert_eq!(trie.query("contin"), TokenType::None);
    }

    #[test]
    fn test_checker_normal_checks() {
        let mut trie = KeywordTrie::new();

        trie.insert("break", TokenType::Break);
        trie.insert("continue", TokenType::Continue);
        trie.insert("if", TokenType::If);
        trie.insert("else", TokenType::Else);
        trie.insert("for", TokenType::For);
        trie.insert("while", TokenType::While);
        trie.insert("return", TokenType::Return);

        let checker = trie.into_checker();

        assert_eq!(checker.query("break"), TokenType::Break);
        assert_eq!(checker.query("continue"), TokenType::Continue);
        assert_eq!(checker.query("if"), TokenType::If);
        assert_eq!(checker.query("else"), TokenType::Else);
        assert_eq!(checker.query("for"), TokenType::For);
        assert_eq!(checker.query("while"), TokenType::While);
        assert_eq!(checker.query("return"), TokenType::Return);

        assert_eq!(checker.query("i"), TokenType::None);
        assert_eq!(checker.query("whilea"), TokenType::None);
        assert_eq!(checker.query("esle"), TokenType::None);
        assert_eq!(checker.query("retun"), TokenType::None);
        assert_eq!(checker.query("contin"), TokenType::None);
    }

    #[test]
    fn test_check_on_the_fly() {
        let mut trie = KeywordTrie::new();

        trie.insert("break", TokenType::Break);
        trie.insert("continue", TokenType::Continue);
        trie.insert("if", TokenType::If);
        trie.insert("else", TokenType::Else);
        trie.insert("for", TokenType::For);
        trie.insert("while", TokenType::While);
        trie.insert("return", TokenType::Return);

        let mut checker = trie.into_checker();

        checker.update('b');
        assert_eq!(checker.check(), TokenType::None);

        checker.update('b');
        checker.update('r');
        checker.update('e');
        checker.update('a');
        checker.update('k');
        assert_eq!(checker.check(), TokenType::Break);
        assert_eq!(checker.check(), TokenType::None);

        checker.update('c');
        checker.update('o');
        checker.update('n');
        checker.update('t');
        checker.update('t');
        assert_eq!(checker.check(), TokenType::None);
    }
}
