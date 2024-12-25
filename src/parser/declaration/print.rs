
use crate::parser::expression::expr::Expression;
use crate::parser::visitor::visitor::Visitor;
use crate::parser::visitor::rebuild_visitor::RebuildVisitor;
use crate::parser::expression::expression::expression;
use crate::parser::parser::Parser;
use crate::scanner::token::TokenType;
use crate::parser::declaration::declaration::PrintStatement;
use crate::parser::visitor::print_visitor::PrintVisitor;


pub fn print_statement(parser: &mut Parser) {
    expression(parser);
    let ast_mode = true;
    let mut visitor = PrintVisitor;
    let mut rebuild = RebuildVisitor;

    let mut print_statement = PrintStatement{ expression: Expression::Empty };
    // =====================================================
    if ast_mode {
        let expr_ast = parser.ast_stack.pop();

        if let Some(ast_node) = expr_ast { 
            let expr = ast_node.to_expression();
                    
            print_statement.expression = expr.clone();
            parser.comment(&format!("; {};", &rebuild.visit_print(&print_statement)));
            parser.emit_instruction(&visitor.visit_print(&print_statement));

            match expr {
                Expression::StringConstant(_) | Expression::Variable(_) => parser.expr_count += 1,
                _ => ()
            }
        }
    } else {
    }
    parser.consume(TokenType::Semicolon, "Expect semicolon after value");
}

