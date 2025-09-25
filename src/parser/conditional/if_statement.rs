use std::fmt::format;

use crate::parser::{
    parser::Parser,
    expression::expr::DataType,
    expression::expression::expression,
    parse_fn::declaration,
    parse_fn::statement
};
use crate::parser::expression::expr::Expr;
use crate::scanner::token::TokenType;

pub enum LlvmConditional {
    If(u32) // represents the depth.
}
impl LlvmConditional {
    pub fn create_branch(&self, bool_reg: u32) -> String {
        match self {
            Self::If(depth) => format!("br i1 %{}, label %then{}, label %else{}", bool_reg - 2, depth, depth)
        }
    }
    pub fn then_branch(&self) -> String {
        match self {
            Self::If(depth) => format!("then{}:", depth)
        }
    }
    pub fn else_branch(&self) -> String {
        match self {
            Self::If(depth) => format!("\nelse{}:", depth)
        }
    }
    pub fn end_branch(&self) -> String {
        match self {
            Self::If(depth) => format!("\nend{}:", depth)
        }
    }
    pub fn to_end(&self) -> String {
        match self {
            Self::If(depth) => format!("br label %end{}", depth)
        }
    }

    pub fn while_cond(&self, bool_reg: u32) -> String {
        match self {
            Self::If(depth) => format!("br i1 %{}, label %body{}, label %exit{}", bool_reg, depth, depth)
        }
    }
    pub fn while_start(&self) -> String {
        match self {
            Self::If(depth) => format!("\ncond{}:", depth)
        }
    }
    pub fn while_body(&self) -> String {
        match self {
            Self::If(depth) => format!("\nbody{}:", depth)
        }
    }
    pub fn while_exit(&self) -> String {
        match self {
            Self::If(depth) => format!("\nexit{}:", depth)
        }
    }
    pub fn while_check_cond(&self, bool_reg: u32) -> String {
        match self {
            Self::If(depth) => format!("br label %cond{}", depth)
        }
    }
    
}
struct IfStatement {
    expr: Option<Expr>,
    branch_struct: Option<LlvmConditional>,
    count: u32,
    depth: u32,
}
impl IfStatement {
    fn new() -> IfStatement {
        IfStatement { expr: None, branch_struct: None, count: 0, depth: 0 }
    }
    fn init(&mut self, parser: &mut Parser) -> &mut IfStatement {
        parser.expr_count += 3;
        self.depth = parser.depth;
        expression(parser);
        let (expr, count) = parser.expr_pop();
        (self.expr, self.count) = (Some(expr), count);
        self.branch_struct = Some(LlvmConditional::If(self.depth));
        parser.comment(&format!("depth: {}", self.depth).to_string());
        parser.depth += 1;
        self
    }
    fn then_branch(&mut self, parser: &mut Parser) -> &mut IfStatement {
        if let Some(branch) = &self.branch_struct {
            parser.emit_instruction(&branch.create_branch(self.count+1));
            parser.consume(TokenType::LeftBrace, "message");
            parser.emit_instruction(&branch.then_branch());
            // parse block
            while !parser.match_current(TokenType::RightBrace) {
                declaration(parser);
            }
            parser.emit_instruction(&&branch.to_end());
        }
        self
    }
    fn else_branch(&mut self, parser: &mut Parser) -> &mut IfStatement {
        if let Some(branch) = &self.branch_struct {
            // Emit the else branch instruction
            parser.emit_instruction(&branch.else_branch());

            // Handle an explicit `else { ... }` block
            if parser.match_current(TokenType::Else) {
                parser.consume(TokenType::LeftBrace, "Expected '{' after 'else'");
                while !parser.match_current(TokenType::RightBrace) {
                    declaration(parser);
                }
            }
            parser.emit_instruction(&branch.to_end());
            
        }
        self
    }
    fn end_branch(&mut self, parser: &mut Parser) -> &mut IfStatement {
        if let Some(branch) = &self.branch_struct {
            parser.emit_instruction(&branch.end_branch());
            parser.depth -= 1;
        }
       self
    }
}
pub fn if_statement(parser: &mut Parser) {
    IfStatement::new()
        .init(parser).
        then_branch(parser)
        .else_branch(parser)
        .end_branch(parser);
}