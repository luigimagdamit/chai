use std::fmt;
use crate::parser::visitor::visitor::{Accept, Visitor};
use crate::common::util::convert_bool;
use crate::codegen::expr_ir::{TypeIR, BinaryOpIR, LiteralIR, PrintIR, ExpressionIR};
use crate::codegen::llvm_expr_ir::LlvmExpressionIR;
use crate::codegen::c_expr_ir::CExpressionIR;
use crate::codegen::backend_config::{get_current_backend, IRBackend};
#[allow(unused)]

const DATATYPE_INT_ERROR: &'static str = "Could not retrieve i32 from Datatype";
const DATATYPE_BOOL_ERROR: &'static str = "Could not retrieve i1 from Datatype";

/// Macro to execute IR-specific code based on current backend
macro_rules! with_expr_ir {
    ($method:ident($($args:expr),*)) => {{
        match get_current_backend() {
            IRBackend::LLVM => {
                let ir = LlvmExpressionIR;
                ir.$method($($args),*)
            }
            IRBackend::C => {
                let ir = CExpressionIR;
                ir.$method($($args),*)
            }
        }
    }};
}

// Highest level structure
#[derive(Clone)]
pub enum Expression {
    Literal(DataType),
    Variable(VariableExpression),
    Binary(Binary),
    StringConstant(StringConstant),
    Array(ArrayExpression),
    Empty
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DataType {
    Integer(Option<i32>),
    String(String),
    Boolean(Option<bool>),
    Array(Vec<DataType>, usize) // elements, size
}

#[derive(Clone)]
pub struct Binary {
    left: Box<Expression>,
    right: Box<Expression>,
    pub operator: Operation,
    register: String,
    datatype: DataType

}

#[derive(Clone)]
pub struct VariableExpression {
    pub name: String,
    pub datatype: DataType,
    pub count: usize
}

#[derive(Clone)]
pub struct StringConstant {
    pub name: String,
    pub length: usize,
    pub index: usize,
    pub register: usize,
    pub datatype: DataType
}

#[derive(Clone)]
pub struct ArrayExpression {
    pub name: String,
    pub elements: Vec<Expression>,
    pub element_type: DataType,
    pub size: usize,
    pub register: usize
}

//===========================
// Each inherits from this ExprNode trait
pub trait ExprNode {
    fn get_value(&self) -> String; // get resolved expr value
    fn get_type(&self) -> &str; // get datatype as a str
    fn to_datatype(&self) -> &DataType;
    fn print(&self) -> String;
}
impl ExprNode for DataType {
    fn get_value(&self) -> String {
        match self {
            DataType::Integer(int) => int.expect(DATATYPE_INT_ERROR).to_string(),
            DataType::Boolean(bool) => convert_bool(bool.expect(DATATYPE_BOOL_ERROR)).to_string(),
            DataType::Array(_, size) => format!("array[{}]", size),
            _ => "".to_string()
        }
    }
    fn get_type(&self) -> &str {
        with_expr_ir!(datatype_to_string(self))
    }
    fn to_datatype(&self) -> &DataType {
        self
    }
    fn print(&self) -> String {
        match self {
            DataType::Integer(int) => {
                let value = int.expect(DATATYPE_INT_ERROR);
                with_expr_ir!(int_literal(value))
            },
            DataType::Boolean(bool) => {
                let value = bool.expect(DATATYPE_BOOL_ERROR);
                with_expr_ir!(bool_literal(value))
            },
            DataType::Array(_, size) => format!("; Array of size {}", size),
            _ => "".to_string()
        }
    }
}
impl ExprNode for Binary {
    fn get_value(&self) -> String {
        // should return the register in this case
        format!("%{}", self.register)
    }
    fn get_type(&self) -> &str {
        self.as_datatype().get_type()
    }
    fn to_datatype(&self) -> &DataType {
        self.as_datatype()
    }
    fn print(&self) -> String {
        let register = self.register.clone().parse()
            .expect("Could not parse register name to a string");

        if self.operator.is_boolean_op() {
            with_expr_ir!(print_bool(register))
        } else {
            with_expr_ir!(print_int(register))
        }
    }
}
impl ExprNode for VariableExpression {
    fn get_type(&self) -> &str {
        self.datatype.get_type()
    }
    fn get_value(&self) -> String {
        format!("%{}_{}", self.name, self.count)
    }
    fn to_datatype(&self) -> &DataType {
        &self.datatype
    }
    fn print(&self) -> String {
        let type_str = (&self.datatype).as_str();
        with_expr_ir!(load_variable(&self.name, type_str, self.count))
    }
}
impl ExprNode for StringConstant {
    fn get_type(&self) -> &str {
        with_expr_ir!(string_type())
    }
    fn get_value(&self) -> String {
        format!("%{}", self.register)
    }
    fn to_datatype(&self) -> &DataType {
        &self.datatype
    }
    fn print(&self) -> String {
        with_expr_ir!(print_string(self.register))
    }
}

impl ExprNode for ArrayExpression {
    fn get_type(&self) -> &str {
        self.element_type.get_type()
    }
    fn get_value(&self) -> String {
        format!("%{}", self.register)
    }
    fn to_datatype(&self) -> &DataType {
        // For now, return a reference to the element type since we can't create a temporary DataType::Array
        &self.element_type
    }
    fn print(&self) -> String {
        // For now, arrays can't be printed directly
        format!("; Array {} of size {}", self.name, self.size)
    }
}

// ============================
impl From<i32> for DataType {
    fn from(item: i32) -> DataType {
        DataType::Integer(Some(item))
    } 
}
impl From<String> for DataType {
    fn from(item: String) -> DataType {
        DataType::String(item)
    }
}
impl From<bool> for DataType {
    fn from(item: bool) -> DataType {
        DataType::Boolean(Some(item))
    }
}

impl DataType {
    pub fn as_str(&self) -> &str {
        with_expr_ir!(datatype_to_string(self))
    }
    pub fn empty_bool() -> DataType {
        DataType::Boolean(Some(true))
    }
    pub fn empty_int() -> DataType {
        DataType::Integer(None)
    }
    pub fn empty_array(element_type: DataType, size: usize) -> DataType {
        DataType::Array(vec![element_type; size], size)
    }
    pub fn array_from_elements(elements: Vec<DataType>) -> DataType {
        let size = elements.len();
        DataType::Array(elements, size)
    }
    pub fn _place(&self, register: usize) -> String {
        format!("%{} = {}", register, self.print())
    }
}




impl Binary {
    pub fn new(left: Expression, right: Expression, operator: Operation, register: &str, datatype: DataType) -> Binary {
        Binary {left: Box::new(left), right: Box::new(right), operator, register: register.to_string(), datatype}
    }
    pub fn get_left(&self) -> &Expression { &self.left }
    pub fn get_right(&self) -> &Expression { &self.right }
    pub fn as_datatype(&self) -> &DataType{ &self.datatype }
}



impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{} {} {}>", self.left, self.right, self.operator)
    }
}


pub trait Register {
    fn register(&self) -> String;
}


impl From<VariableExpression> for Expression {
    fn from(value: VariableExpression) -> Self {
        Expression::Variable(value)
    }
}


impl StringConstant {

    pub fn place(&self) -> String {
        with_expr_ir!(string_literal(self.register, self.length, self.index))
    }
}

impl ArrayExpression {
    pub fn new(name: String, elements: Vec<Expression>, element_type: DataType, size: usize, register: usize) -> ArrayExpression {
        ArrayExpression {
            name,
            elements,
            element_type,
            size,
            register,
        }
    }

    pub fn empty(name: String, element_type: DataType, size: usize, register: usize) -> ArrayExpression {
        ArrayExpression {
            name,
            elements: Vec::new(),
            element_type,
            size,
            register,
        }
    }
}



impl Register for Expression {
    fn register(&self) -> String {
        //println!("; [Register Trait] Placing expression in register for declaration use.");
        print!("\t");
        match self {
            Expression::Binary(binary) => Expression::from(binary.clone()).resolve_binary(),
            Expression::Literal(literal) => Expression::from(literal.clone()).resolve_binary(),
            Expression::StringConstant(str_constant) => format!("%{}", str_constant.register),
            Expression::Array(array) => format!("%{}", array.register),
            _ => panic!()

        }
    }
}
impl Accept for Expression {
    fn accept<V: Visitor> (&self, visitor: &mut V) -> String{
        match self {
            Expression::Literal(literal) => visitor.visit_literal(literal),
            Expression::Binary(binary) => visitor.visit_binary(binary),
            Expression::Variable(variable) => visitor.visit_variable_expression(variable),
            Expression::StringConstant(str_constant) => visitor.visit_string(str_constant),
            Expression::Array(array) => visitor.visit_array(array),
            _ => panic!()
        }
    }
}
impl From<Binary> for Expression {
    fn from(binary: Binary) -> Expression {
        Expression::Binary(binary)
    }
}
impl From<&Binary> for Expression {
    fn from(binary: &Binary) -> Expression {
        Expression::Binary(binary.clone())
    }
}
impl From<DataType> for Expression {
    fn from(value: DataType) -> Self {
        Expression::Literal(value)
    }
}
impl From<&DataType> for Expression {
    fn from(value: &DataType) -> Self {
        Expression::Literal(value.clone())
    }
}
impl From<Expression> for Binary {
    fn from(value: Expression) -> Self {
        match value {
            Expression::Binary(binary) => binary,
            _ => panic!("Not a binary expression") 
        }
    }
}
impl From<StringConstant> for Expression {
    fn from(value: StringConstant) -> Self {
        Expression::StringConstant(value)
    }
}

impl From<ArrayExpression> for Expression {
    fn from(value: ArrayExpression) -> Self {
        Expression::Array(value)
    }
}
impl Expression {
    pub fn new_binary(left: Expression, right: Expression, operator: Operation, register: &str, datatype: DataType) -> Expression {
        Expression::Binary(Binary::new(left, right, operator, register, datatype))
    }

    pub fn new_array(name: String, elements: Vec<Expression>, element_type: DataType, size: usize, register: usize) -> Expression {
        Expression::Array(ArrayExpression::new(name, elements, element_type, size, register))
    }

    pub fn empty_array(name: String, element_type: DataType, size: usize, register: usize) -> Expression {
        Expression::Array(ArrayExpression::empty(name, element_type, size, register))
    }
    pub fn as_str_constant(&self) -> StringConstant {
        match self {
            Expression::StringConstant(str_constant) => str_constant.clone(),
            _ => panic!()
        }
    }
    pub fn resolve_binary(&self) -> String {
        match self {
            Expression::Binary(b) => {
                let result_type = b.to_datatype().as_str();
                let left_operand = b.get_left().resolve_operand();
                let right_operand = b.get_right().resolve_operand();

                with_expr_ir!(binary_op(&b.register, &b.operator, &left_operand, &right_operand, result_type))
            },
            _ => panic!()
        }

    }
    // make it so that this eventually calls get_value() for everything
    pub fn resolve_operand(&self) -> String {
        match self {
            Expression::Binary(b) => b.get_value(),
            Expression::Literal(i) => i.get_value(),
            Expression::StringConstant(s) => s.get_value(),
            Expression::Variable(v) => v.get_value(),
            Expression::Array(a) => a.get_value(),
            _ => panic!("resolve_operand should have gotten an object that takes the ExprNode trait")
        }
    }
    pub fn from_literal(literal: DataType) -> Expression{
        Expression::Literal(literal)
    }
    pub fn as_datatype(&self) -> DataType {
        match self {
            Expression::Literal(datatype) => datatype.clone(),
            Expression::Binary(binary) => binary.datatype.clone(),
            Expression::Variable(variable) => variable.datatype.clone(),
            Expression::StringConstant(_) => DataType::String("".to_string()),
            Expression::Array(array) => DataType::Array(
                array.elements.iter().map(|e| e.as_datatype()).collect(),
                array.size
            ),
            _ => panic!()
        }
    }

}
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Binary(b) => write!(f, "\n{b}"),
            Expression::Literal(l) => write!(f, "{l}"),
            Expression::Array(a) => write!(f, "array[{}]:{}", a.size, a.name),
            _ => write!(f, "")
        }
    }
}
pub enum ParseError {
    Generic
}
impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Integer(int) => match int {
                Some(item) => write!(f, "{}", item),
                None => write!(f, "int")
            },
            DataType::Boolean(b) => match b {
                Some(item) => write!(f, "{}", item),
                None => write!(f, "bool")
            },
            DataType::String(str) => write!(f, "str:<{}>", str),
            DataType::Array(elements, size) => {
                write!(f, "array[{}]:<", size)?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", elem)?;
                }
                write!(f, ">")
            }
        }
    }
}
// enum Operator {
//     Add
// }
#[derive(Clone)]
pub struct Expr {
    pub left: String,
    pub right: String,
    pub data_type: DataType
}

#[derive(Clone, PartialEq, Debug)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterEqual,
    LessThan,
    LessEqual
}
impl Operation {
    pub fn is_boolean_op(&self) -> bool{
        match &self {
            Operation::Add | Operation::Div | Operation::Mul | Operation::Sub => false,
            _ => true
        }
    }
}
impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Add => write!(f, "add"),
            Operation::Sub => write!(f, "sub"),
            Operation::Mul => write!(f, "mul"),
            Operation::Div => write!(f, "div"),
            Operation::Equal => write!(f, "icmp eq"),
            Operation::NotEqual => write!(f, "icmp ne"),
            Operation::GreaterEqual => write!(f, "icmp sge"),
            Operation::GreaterThan => write!(f, "icmp sgt"),
            Operation::LessEqual => write!(f, "icmp sle"),
            Operation::LessThan => write!(f, "icmp slt")
        }
    }
}

