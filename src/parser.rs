use crate::ast;
use crate::lexer;

pub struct Parser {
    tokens: Vec<lexer::Token>,
    left: Option<Box<ast::Expr>>,
    right: Option<Box<ast::Expr>>,
    operator: String,
}

impl Parser {
	pub fn new(left: ast::Expr, right: ast::Expr, operator: String) -> Parser {
        let tokens = Vec::<lexer::Token>::new();
        Parser { tokens, left, right, operator }
    }

	pub fn produceAST (&mut self, source: String) -> ast::Program {
		self.tokens = lexer::tokenize(source);
    	let mut program: ast::Program = ast::Program::new();
    	while self.not_eof() {
    		program.body.push(self.parse_stmt());
    	}
    	program
    }

    pub fn not_eof(&mut self) -> bool {
        return self.tokens[0].ty != lexer::TokenType::Eof;
    }

    pub fn parse_stmt(&mut self) -> ast::Stmt {
        // self.parse_expr()
        unimplemented!();
    }

    pub fn parse_expr(&mut self) -> ast::Expr {
        unimplemented!();
    }
}
