mod constraint_parser;
mod infix_parser;
mod parser;
mod prefix_parser;

pub use parser::Parser;

use librasac_ast::{Expression, FunctionDefinition, Statement};
use librasac_lexer::token::Token;

#[derive(Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub functions: Vec<FunctionDefinition>,
    pub directives: Vec<FunctionDefinition>,
    pub custom_directives: Vec<(String, FunctionDefinition)>,
    pub main : Option<FunctionDefinition>,
}

const MAIN_FUNCTION : &str = "main";

impl Program {
    pub fn new() -> Self {
        Program::with_capacity(0)
    }
    pub fn with_capacity(cap: usize) -> Self {
        Program {
            statements: Vec::with_capacity(cap),
            custom_directives: Vec::with_capacity(cap),
            directives: Vec::with_capacity(cap),
            functions: Vec::with_capacity(cap),
            main: None,
        }
    }
    pub fn push_statement(&mut self, stmt: Statement) {
        self.statements.push(stmt)
    }
    pub fn push_constraint_function(&mut self, func: FunctionDefinition) {
        if func.name == MAIN_FUNCTION.to_string().into() {
            self.main = Some(func)
        } else {
            self.functions.push(func);
        }
    }
    pub fn push_directive_function(&mut self, name : Option<String>, func: FunctionDefinition) {
            // If it has no name, it is a regular directive, which will be used to do field operations without constraints
            // If it has a name, it is a custom directive, which will map to a custom gate
        match name {
            Some(name) => self.custom_directives.push((name, func)),
            None => self.directives.push(func)
        }
    }
    /// Returns the program abi which is only present for executables and not libraries
    pub fn abi(&self) -> Option<Vec<String>>{
        match &self.main {
            Some(main_func) => {
                let abi = main_func.func.parameters.iter().map(|(ident, _)| ident.0.clone()).collect();
                Some(abi)
            },
            None => None
        }
    }
}

trait PrefixParser {
    fn parse(parser: &mut Parser) -> Expression;
}
trait InfixParser {
    fn parse(parser: &mut Parser, lhs: Expression) -> Expression;
}

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
    Index,
}
impl Precedence {
    // Higher the number, the higher(more priority) the precedence
    fn token_precedence(tok: &Token) -> Precedence {
        match tok {
            Token::Equal => Precedence::Equals,
            Token::NotEqual => Precedence::Equals,
            Token::Less => Precedence::LessGreater,
            Token::Greater => Precedence::LessGreater,
            Token::Plus => Precedence::Sum,
            Token::Minus => Precedence::Sum,
            Token::Slash => Precedence::Product,
            Token::Star => Precedence::Product,
            Token::LeftParen => Precedence::Call,
            Token::LeftBracket => Precedence::Index,
            _ => Precedence::Lowest,
        }
    }
}
impl From<&Token> for Precedence {
    fn from(t: &Token) -> Precedence {
        Precedence::token_precedence(t)
    }
}
