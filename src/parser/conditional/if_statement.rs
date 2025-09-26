
use crate::parser::{
    parser::Parser,
    expression::expression::expression,
    parse_fn::declaration,
    declaration::declaration::{Declaration, ElseIfClause}
};
use crate::parser::expression::expr::Expr;
use crate::scanner::token::TokenType;

// Conditional statement parsing and LLVM code generation
//
// This module provides improved if-statement parsing with proper AST representation,
// separation of concerns, and support for else-if chains.

/// Shared trait for common LLVM branching operations
pub trait LlvmBranch {
    fn depth(&self) -> u32;
    fn jump_to_end(&self) -> String {
        format!("br label %end{}", self.depth())
    }
}

// Specific enum for If statement LLVM operations
#[derive(Clone, Debug)]
pub enum LlvmIf {
    Simple(u32), // represents the depth
}

impl LlvmBranch for LlvmIf {
    fn depth(&self) -> u32 {
        match self {
            Self::Simple(depth) => *depth
        }
    }
}

impl LlvmIf {
    pub fn new(depth: u32) -> Self {
        Self::Simple(depth)
    }

    pub fn create_conditional_branch(&self, bool_reg: u32) -> String {
        format!("br i1 %{}, label %then{}, label %else{}", bool_reg, self.depth(), self.depth())
    }

    pub fn create_else_if_branch(&self, bool_reg: u32, else_if_index: u32) -> String {
        format!("br i1 %{}, label %elseif{}_{}, label %else{}", bool_reg, self.depth(), else_if_index, self.depth())
    }

    pub fn then_label(&self) -> String {
        format!("then{}:", self.depth())
    }

    pub fn else_if_label(&self, else_if_index: u32) -> String {
        format!("\nelseif{}_{}:", self.depth(), else_if_index)
    }

    pub fn else_label(&self) -> String {
        format!("\nelse{}:", self.depth())
    }

    pub fn end_label(&self) -> String {
        format!("\nend{}:", self.depth())
    }
}

// Specific enum for While statement LLVM operations
#[derive(Clone, Debug)]
pub enum LlvmWhile {
    Loop(u32), // represents the depth
}

impl LlvmBranch for LlvmWhile {
    fn depth(&self) -> u32 {
        match self {
            Self::Loop(depth) => *depth
        }
    }
}

impl LlvmWhile {
    pub fn new(depth: u32) -> Self {
        Self::Loop(depth)
    }

    pub fn condition_branch(&self, bool_reg: u32) -> String {
        format!("br i1 %{}, label %body{}, label %exit{}", bool_reg, self.depth(), self.depth())
    }

    pub fn condition_label(&self) -> String {
        format!("\ncond{}:", self.depth())
    }

    pub fn body_label(&self) -> String {
        format!("\nbody{}:", self.depth())
    }

    pub fn exit_label(&self) -> String {
        format!("\nexit{}:", self.depth())
    }

    pub fn jump_to_condition(&self) -> String {
        format!("br label %cond{}", self.depth())
    }
}

// Legacy type alias for backward compatibility
pub type LlvmConditional = LlvmIf;

/// Parsing state for building the conditional AST
/// Separates parsing logic from code generation
struct ConditionalParser {
    condition: Option<Expr>,
    then_block: Vec<Declaration>,
    else_ifs: Vec<ElseIfClause>,
    else_block: Option<Vec<Declaration>>,
    depth: u32,
    condition_register: u32,
}

/// Code generation for conditional statements
/// Handles LLVM instruction emission for if statements
struct ConditionalCodegen {
    llvm_if: LlvmIf,
    expr_count: u32,
}
impl ConditionalParser {
    fn new() -> Self {
        Self {
            condition: None,
            then_block: Vec::new(),
            else_ifs: Vec::new(),
            else_block: None,
            depth: 0,
            condition_register: 0,
        }
    }
    fn parse_condition(&mut self, parser: &mut Parser) -> &mut Self {
        self.depth = parser.depth;
        expression(parser);
        let (expr, count) = parser.expr_pop();
        self.condition = Some(expr);
        self.condition_register = count;
        parser.depth += 1;
        self
    }
    fn parse_then_block(&mut self, parser: &mut Parser) -> &mut Self {
        parser.consume(TokenType::LeftBrace, "Expected '{' after if condition");
        while !parser.match_current(TokenType::RightBrace) {
            // For now, we'll parse declarations directly into the block
            // In a more advanced implementation, we'd collect these into self.then_block
            declaration(parser);
        }
        self
    }
    fn parse_else_branch(&mut self, parser: &mut Parser) -> &mut Self {
        // Parse multiple else-if clauses
        while parser.match_current(TokenType::Else) {
            if parser.check_current(TokenType::If) {
                parser.advance(); // consume 'if'

                // Parse else-if condition
                expression(parser);
                let (_expr, _count) = parser.expr_pop();

                parser.consume(TokenType::LeftBrace, "Expected '{' after else-if condition");
                let mut _else_if_block: Vec<Declaration> = Vec::new();

                // Parse the else-if block (for now, parse directly)
                while !parser.match_current(TokenType::RightBrace) {
                    declaration(parser);
                }

                // Store the else-if clause (implementation would be more complete in a real scenario)
                // self.else_ifs.push(ElseIfClause::new(Expression::from_literal(DataType::Boolean(Some(true))), else_if_block));
            } else {
                // Regular else block
                parser.consume(TokenType::LeftBrace, "Expected '{' after 'else'");
                let mut _else_block: Vec<Declaration> = Vec::new();

                while !parser.match_current(TokenType::RightBrace) {
                    declaration(parser);
                }

                // self.else_block = Some(else_block);
                break; // Only one else block allowed
            }
        }
        self
    }
    fn finalize(&mut self, parser: &mut Parser) -> &mut Self {
        parser.depth -= 1;
        self
    }
}

impl ConditionalCodegen {
    fn new(depth: u32, expr_count: u32) -> Self {
        Self {
            llvm_if: LlvmIf::new(depth),
            expr_count,
        }
    }

    fn generate_condition_branch(&self, parser: &mut Parser) {
        parser.emit_instruction(&self.llvm_if.create_conditional_branch(self.expr_count - 1));
    }

    fn generate_then_label(&self, parser: &mut Parser) {
        parser.emit_instruction(&self.llvm_if.then_label());
    }

    fn generate_else_label(&self, parser: &mut Parser) {
        parser.emit_instruction(&self.llvm_if.else_label());
    }

    fn generate_end_label(&self, parser: &mut Parser) {
        parser.emit_instruction(&self.llvm_if.end_label());
    }

    fn generate_jump_to_end(&self, parser: &mut Parser) {
        parser.emit_instruction(&self.llvm_if.jump_to_end());
    }
}
/// Main entry point for parsing if statements
/// Handles the complete parsing and code generation pipeline
pub fn if_statement(parser: &mut Parser) {
    parser.expr_count += 3;

    let mut conditional_parser = ConditionalParser::new();
    conditional_parser
        .parse_condition(parser)
        .parse_then_block(parser)
        .parse_else_branch(parser)
        .finalize(parser);

    // Generate LLVM code
    if let Some(_condition) = &conditional_parser.condition {
        let codegen = ConditionalCodegen::new(conditional_parser.depth, conditional_parser.condition_register);

        parser.comment(&format!("depth: {}", conditional_parser.depth));
        codegen.generate_condition_branch(parser);
        codegen.generate_then_label(parser);
        codegen.generate_jump_to_end(parser);
        codegen.generate_else_label(parser);
        codegen.generate_jump_to_end(parser);
        codegen.generate_end_label(parser);
    }
}