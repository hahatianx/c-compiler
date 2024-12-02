use std::collections::HashMap;
use crate::scanner::tokens::TokenType;
use crate::common::Result;

pub trait Trie {
    fn insert(&mut self, key: &str, value: TokenType);
    fn query(&self, word: &str) -> Option<TokenType>;
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
    cache: String,
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

    fn query(&self, word: &str) -> Option<TokenType> {
        let mut cur = &self.root;
        for c in word.chars() {
            match cur.ch.get(&c) {
                Some(node) => cur = node,
                None => return None,
            }
        }
        if cur.leaf {
            Some(cur.token_type)
        } else {
            None
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

        keyword_trie.insert("int", TokenType::KeyInt);
        keyword_trie.insert("long", TokenType::KeyLong);
        keyword_trie.insert("float", TokenType::KeyFloat);
        keyword_trie.insert("double", TokenType::KeyDouble);
        keyword_trie.insert("string", TokenType::KeyString);

        keyword_trie
    }

    pub fn into_checker(&self) -> KeywordTrieChecker {
        KeywordTrieChecker {
            trie: self,
            curr: Some(&self.root),
            cache: String::new(),
        }
    }
}

impl<'a> KeywordTrieChecker<'a> {

    pub fn query(&self, word: &str) -> Option<TokenType> {
        self.trie.query(word)
    }

    pub fn update(&mut self, c: char) {
        if let Some(cur) = self.curr {
            self.curr = cur.ch.get(&c);
        }
        self.cache.push(c);
    }

    pub fn check(&mut self) -> Option<TokenType> {
        let mut return_value = None;
        if let Some(cur) = self.curr {
            if cur.leaf {
                return_value = Some(cur.token_type);
            }
        }
        self.curr = Some(&self.trie.root);
        return_value
    }

    pub fn get_str(&mut self) -> Result<String> {
        // validate
        let return_value = Ok(self.cache.clone());
        self.cache.clear();
        return_value
    }

    pub fn can_consume(&self, c: &char) -> bool {
        c.is_ascii_alphanumeric() || *c == '_'
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

        assert_eq!(trie.query("break"), Some(TokenType::Break));
        assert_eq!(trie.query("continue"), Some(TokenType::Continue));
        assert_eq!(trie.query("if"), Some(TokenType::If));
        assert_eq!(trie.query("else"), Some(TokenType::Else));
        assert_eq!(trie.query("for"), Some(TokenType::For));
        assert_eq!(trie.query("while"), Some(TokenType::While));
        assert_eq!(trie.query("return"), Some(TokenType::Return));

        assert_eq!(trie.query("i"), None);
        assert_eq!(trie.query("whilea"), None);
        assert_eq!(trie.query("esle"), None);
        assert_eq!(trie.query("retun"), None);
        assert_eq!(trie.query("contin"), None);
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

        assert_eq!(checker.query("break"), Some(TokenType::Break));
        assert_eq!(checker.query("continue"), Some(TokenType::Continue));
        assert_eq!(checker.query("if"), Some(TokenType::If));
        assert_eq!(checker.query("else"), Some(TokenType::Else));
        assert_eq!(checker.query("for"), Some(TokenType::For));
        assert_eq!(checker.query("while"), Some(TokenType::While));
        assert_eq!(checker.query("return"), Some(TokenType::Return));

        assert_eq!(checker.query("i"), None);
        assert_eq!(checker.query("whilea"), None);
        assert_eq!(checker.query("esle"), None);
        assert_eq!(checker.query("retun"), None);
        assert_eq!(checker.query("contin"), None);
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
        assert_eq!(checker.check(), None);

        checker.update('b');
        checker.update('r');
        checker.update('e');
        checker.update('a');
        checker.update('k');
        assert_eq!(checker.check(), Some(TokenType::Break));
        assert_eq!(checker.check(), None);

        checker.update('c');
        checker.update('o');
        checker.update('n');
        checker.update('t');
        checker.update('t');
        assert_eq!(checker.check(), None);
    }
}
