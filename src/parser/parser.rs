use crate::common::errors::error::CompilerErrorKind;
use crate::common::errors::error::CompilerErrorKind::CompilerError;
use crate::scanner::tokens::{Token, TokenType};
use crate::common::Result;
use crate::parser::ast::block_node::BlockNode;
use crate::parser::ast::core::AstNode;
use crate::parser::ast::print_node::PrintAstNode;
use crate::parser::precedence::Precedence;
use crate::scanner::scanner::Scanner;

pub struct Parser<'a> {

    scanner: &'a mut Scanner<'a>,


    left_hand_stack: Vec<Box<dyn AstNode>>,

    /**
     <p> This stack is for token breaker.  e.g. right arrow '>>' into two brackets '>''>' </p>

    vector<pair<int, int>> is recognized as "vector" "<" "pair" "<" "int" "," "int" ">>"
    which should be "vector" "<" "pair" "<" "int" "," "int" ">" ">"
     */
    temp_token_stack: Vec<Token>,
    /**
    On token breaking cases, the column shift should be adjusted
    */
    column_shift: usize,

    previous: Token,
    current: Token,

}

impl<'a> Parser<'a> {

    pub fn new(scanner: &'a mut Scanner<'a>) -> Parser<'a> {
        let mut parser = Self {
            scanner,

            left_hand_stack: Vec::new(),

            temp_token_stack: Vec::new(),
            column_shift: 0,

            previous: Token::None,
            current: Token::None,
        };
        parser.current = parser.scanner.scan().unwrap();
        parser
    }

    pub fn parse(&mut self) -> Result<Box<dyn AstNode>> {
        self.block()
    }

    pub fn get_previous(&self) -> &Token {
        &self.previous
    }

    pub fn get_current(&self) -> &Token {
        self.fetch_cur()
    }

    pub fn push_ast(&mut self, ast: Box<dyn AstNode>) {
        self.left_hand_stack.push(ast);
    }

    pub fn pop_ast(&mut self) -> Box<dyn AstNode> {
        self.left_hand_stack.pop().unwrap()
    }

    fn push_broken_tokens(&mut self, token: Token) {
        self.temp_token_stack.push(token);
        self.column_shift += 1;
    }

    fn pop_broken_tokens(&mut self) -> Token {
        let token = self.temp_token_stack.pop().unwrap();
        self.column_shift -= 1;
        token
    }

    fn get_position(&self) -> (usize, usize) {
        let (line, column) = self.scanner.get_position();
        (line, column + self.column_shift)
    }

    fn error(&self, message: &'static str) -> CompilerErrorKind {
        let (line, column) = self.get_position();
        CompilerError(line, column, String::from(message))
    }

    fn advance(&mut self) -> Result<()> {
        self.previous = self.current.clone();
        if self.temp_token_stack.len() > 0 {
            self.pop_broken_tokens();
        } else {
            self.previous = self.current.clone();
            self.current = self.scanner.scan()?;
        }
        Ok(())
    }

    fn fetch_cur(&self) -> &Token {
        if self.temp_token_stack.len() > 0 {
            self.temp_token_stack.last().unwrap()
        } else {
            &self.current
        }
    }

    fn check(&self, token_type: TokenType) -> bool {
        self.fetch_cur().get_token_type() == token_type
    }

    pub fn consume(&mut self, token_type: TokenType, msg: &'static str) -> Result<()> {
        if self.check(token_type) {
            self.advance()?;
            return Ok(());
        }
        Err(self.error(msg))
    }

    fn t_match(&mut self, token_type: TokenType) -> Result<bool> {
        if !self.check(token_type) {
            return Ok(false);
        }
        self.advance()?;
        Ok(true)
    }

    /**********************************************************************************/

    pub fn parse_precedence(&mut self, precedence: Precedence) -> Result<Box<dyn AstNode>> {
        self.advance()?;
        let can_assign = precedence.value() <= Precedence::PrecAssignment.value();

        let Some(prefix_rule) = self.get_previous().get_token_type().get_rule().get_prefix() else {
            return Err(self.error("Expected expression."))
        };

        let prefix = prefix_rule(self, can_assign)?;
        self.push_ast(prefix);

        while precedence.value() <= self.get_current().get_token_type().get_rule().precedence.value() {
            self.advance()?;
            let infix_rule = self.get_previous().get_token_type().get_rule().get_infix().unwrap();
            let infix = infix_rule(self, can_assign)?;
            self.push_ast(infix);
        }

        if can_assign && self.t_match(TokenType::Equal)? {
            return Err(self.error("Invalid assignment target."))
        }

        Ok(self.pop_ast())
    }

    pub fn expression(&mut self) -> Result<Box<dyn AstNode>> {
        self.parse_precedence(Precedence::PrecAssignment)
    }

    pub fn statement(&mut self) -> Result<Box<dyn AstNode>> {
        if self.t_match(TokenType::Print)? {
            self.print_statement()
            // TODO: add int declaration
        // } else if self.t_match(TokenType::KeyInt)? {
        } else {
            self.expression_statement()
        }
    }

    pub fn block(&mut self) -> Result<Box<dyn AstNode>> {

        // temporary: consume a left brace {

        self.t_match(TokenType::LeftBrace)?;

        let mut asts: Vec<Box<dyn AstNode>> = vec!();
        while !self.check(TokenType::RightBrace) && !self.t_match(TokenType::Eof)? {
            asts.push(self.statement()?)
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block")?;

        Ok(Box::new(BlockNode::new(asts)))
    }

    /**********************************************************************************/

    fn print_statement(&mut self) -> Result<Box<dyn AstNode>> {
        let ast = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after expression.")?;
        Ok(Box::new(PrintAstNode::new(ast)))
    }

    fn expression_statement(&mut self) -> Result<Box<dyn AstNode>> {
        let ast = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after expression.")?;
        Ok(ast)
    }

    // fn variable_declaration(&mut self, token_type: &TokenType) -> Result<Box<dyn AstNode>> {
    //
    // }

}