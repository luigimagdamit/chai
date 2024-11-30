use super::parser::{Parser, StringEntry};
use super::expr::{DataType, Expr};

pub fn parse_string(parser: &mut Parser) {
    let value = parser.previous.unwrap().start;
    let length = value.len();

    let codegen = format!("@str{} = private unnamed_addr constant [{} x i8] c\"{}\\0A\\00\", align 1", parser.expr_count, length , &value[1..length - 1]);
    match parser.string_table.get(value) {
        Some(str) => {
            let string_expr = Expr {
                left: String::from(format!("getelementptr inbounds [{} x i8], [{} x i8]* @str{}, i32 0, i32 0", str.index , length, str.index)),
                right: String::from(format!("getelementptr inbounds [{} x i8], [{} x i8]* @str{}, i32 0, i32 0", str.index , length, str.index)),
                data_type: DataType::String(String::from(value))
            };
            parser.constant_stack.push(Some(string_expr));
            parser.expr_count += 1;
        },
        None => {
            parser.string_table.insert(String::from(value), StringEntry {
                codegen: codegen,
                length: length - 1,
                index: parser.expr_count
            }); 
            let string_expr = Expr {
                left: String::from(format!("getelementptr inbounds [{} x i8], [{} x i8]* @str{}, i32 0, i32 0", length , length, parser.expr_count)),
                right: String::from(format!("getelementptr inbounds [{} x i8], [{} x i8]* @str{}, i32 0, i32 0", length , length, parser.expr_count)),
                data_type: DataType::String(String::from(value))
            };
            // println!("{}", string_expr.left);
            parser.constant_stack.push(Some(string_expr));
            parser.expr_count += 1;
        }
    }
    

    
}