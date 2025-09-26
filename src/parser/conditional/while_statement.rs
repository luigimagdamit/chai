
use crate::parser::{
    parser::Parser,
    expression::expression::expression,
    parse_fn::declaration
};
use crate::scanner::token::TokenType;
use crate::codegen::ir_traits::{BranchIR, LoopIR};
use crate::codegen::llvm_ir::LlvmLoop;

/// Generic loop parser that works with any IR backend
struct LoopParser {
    depth: u32,
    condition_register: u32,
}

impl LoopParser {
    fn new() -> Self {
        Self {
            depth: 0,
            condition_register: 0,
        }
    }

    fn parse_condition(&mut self, parser: &mut Parser) -> &mut Self {
        self.depth = parser.expr_count;
        expression(parser);
        let expr = parser.expr_pop();
        self.condition_register = expr.1;
        parser.depth += 1;
        self
    }

    fn parse_body(&mut self, parser: &mut Parser) -> &mut Self {
        parser.consume(TokenType::LeftBrace, "Expected '{' after while condition");
        while !parser.match_current(TokenType::RightBrace) {
            declaration(parser);
        }
        self
    }
}

/// Generic loop codegen that works with any LoopIR implementation
struct LoopCodegen<IR: LoopIR> {
    ir: IR,
    condition_register: u32,
}

impl<IR: LoopIR> LoopCodegen<IR> {
    fn new(ir: IR, condition_register: u32) -> Self {
        Self { ir, condition_register }
    }

    fn generate_setup(&self, parser: &mut Parser) {
        parser.emit_instruction(&self.ir.jump_to_condition());
        parser.emit_instruction(&self.ir.condition_label());
    }

    fn generate_condition_branch(&self, parser: &mut Parser) {
        parser.emit_instruction(&self.ir.condition_branch(self.condition_register - 1));
    }

    fn generate_body_label(&self, parser: &mut Parser) {
        parser.emit_instruction(&self.ir.body_label());
    }

    fn generate_loop_back(&self, parser: &mut Parser) {
        parser.emit_instruction(&self.ir.jump_to_condition());
        parser.emit_instruction(&self.ir.exit_label());
    }
}

pub fn while_statement(parser: &mut Parser) {
    // Parse the while loop structure
    let mut loop_parser = LoopParser::new();

    // Setup and parse condition
    let ir = LlvmLoop::new(parser.expr_count);
    let codegen = LoopCodegen::new(ir, 0); // Will be updated after condition parsing

    codegen.generate_setup(parser);
    loop_parser.parse_condition(parser);

    parser.comment(&format!("depth: {}", loop_parser.depth));

    // Create new codegen with correct condition register
    let ir = LlvmLoop::new(loop_parser.depth);
    let codegen = LoopCodegen::new(ir, loop_parser.condition_register);

    codegen.generate_condition_branch(parser);
    codegen.generate_body_label(parser);

    loop_parser.parse_body(parser);

    codegen.generate_loop_back(parser);
}