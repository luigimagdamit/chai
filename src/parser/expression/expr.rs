use std::fmt;
use crate::parser::visitor::visitor::{Accept, Visitor};
use crate::common::util::convert_bool;
use crate::llvm::llvm_print::llvm_call_print_local;
use crate::llvm::llvm_string::*;
#[allow(unused)]

const DATATYPE_INT_ERROR: &'static str = "Could not retrieve i32 from Datatype";
const DATATYPE_BOOL_ERROR: &'static str = "Could not retrieve i1 from Datatype";

pub trait ExprNode {
    fn get_value(&self) -> String; // get resolved expr value
    fn get_type(&self) -> &str; // get datatype as a str
    fn to_datatype(&self) -> &DataType;
    fn print(&self) -> String;
}

// DataType is a literal
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DataType {
    Integer(Option<i32>),
    String(String),
    Boolean(Option<bool>)
}

impl ExprNode for DataType {
    fn get_value(&self) -> String {
        match self {
            DataType::Integer(int) => int.expect(DATATYPE_INT_ERROR).to_string(),
            DataType::Boolean(bool) => convert_bool(bool.expect(DATATYPE_BOOL_ERROR)).to_string(),
            _ => "".to_string()
        }
    }
    fn get_type(&self) -> &str {
        match self {
            DataType::Integer(_) => "i32",
            DataType::Boolean(_) => "i1",
            _ => ""
        }
    }
    fn to_datatype(&self) -> &DataType {
        self
    }
    fn print(&self) -> String {
        match self {
            DataType::Integer(int) => format!("add i32 {}, 0; a", int.expect(DATATYPE_INT_ERROR)),
            DataType::Boolean(bool) => {
                let bool_val = if bool.expect(DATATYPE_BOOL_ERROR) {1} else {0};
                format!("add i1 {bool_val}, 0")
            }
            _ => "".to_string()
        }
    }
}

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
        match self {
            DataType::Integer(_) => "i32",
            DataType::Boolean(_) => "i1",
            DataType::String(_) => "i8*"
        }
    }
    pub fn empty_bool() -> DataType {
        DataType::Boolean(Some(true))
    }
    pub fn empty_int() -> DataType {
        DataType::Integer(None)
    }
    pub fn _place(&self, register: usize) -> String {
        format!("%{} = {}", register, self.print())
    }
}


#[derive(Clone)]
pub struct Binary {
    left: Box<Expression>,
    right: Box<Expression>,
    pub operator: Operation,
    register: String,
    datatype: DataType

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
        if self.operator.is_boolean_op() {
            format!("{}", llvm_call_print_local(
                self.register.clone()
                    .parse()
                    .expect("Could not convert the register name to a string")
                , "i1"))
        } else {
            format!("{}", llvm_call_print_local(self.register
                .clone()
                .parse()
                .expect("Could not parse register name to a string")
            , "i32"))
        }
        
    }
}
#[allow(unused)]
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

// Expressions
// These are all the default actions I want to use with Expression Types

pub trait Register {
    fn register(&self) -> String;
}
#[derive(Clone)]
pub struct VariableExpression {
    pub name: String,
    pub datatype: DataType,
    pub count: usize
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
        format!("%{}_{} = load {type_str}, {type_str}* %{} ; loading existing variable", self.name, self.count, self.name)
    }
}
impl From<VariableExpression> for Expression {
    fn from(value: VariableExpression) -> Self {
        Expression::Variable(value)
    }
}
#[derive(Clone)]
pub struct StringConstant {
    pub name: String,
    pub length: usize,
    pub index: usize,
    pub register: usize,
    pub datatype: DataType
}

impl ExprNode for StringConstant {
    fn get_type(&self) -> &str { "i8" }
    fn get_value(&self) -> String {
        format!("%{}", self.register)
    }
    fn to_datatype(&self) -> &DataType {
        &self.datatype
    }
    fn print(&self) -> String {
        format!("call i32 (i8*, ...) @printf(i8* %{})", self.register)
    }
}
impl StringConstant {
    
    pub fn place(&self) -> String {
        format!("%{} = {} ; place() in impl StringConstant", self.register, &llvm_retrieve_static_string(self.length, self.index))
    }
}

#[derive(Clone)]
pub enum Expression {
    Literal(DataType),
    Variable(VariableExpression),
    Binary(Binary),
    StringConstant(StringConstant),
    Empty
}


impl Register for Expression {
    fn register(&self) -> String {
        //println!("; [Register Trait] Placing expression in register for declaration use.");
        print!("\t");
        match self {
            Expression::Binary(binary) => Expression::from(binary.clone()).resolve_binary(),
            Expression::Literal(literal) => Expression::from(literal.clone()).resolve_binary(),
            Expression::StringConstant(str_constant) => format!("%{}", str_constant.register),
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
impl Expression {
    pub fn new_binary(left: Expression, right: Expression, operator: Operation, register: &str, datatype: DataType) -> Expression {
        Expression::Binary(Binary::new(left, right, operator, register, datatype))
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
                let tag = b.to_datatype().as_str();
                let mut codegen = format!("{} {tag} ", b.operator);
                codegen += &b.get_left().resolve_operand();
                codegen += &", ".to_string();
                codegen += &b.get_right().resolve_operand();
                format!("%{} = ", b.register) + &codegen
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
            _ => panic!()
        }
    }

}
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Binary(b) => write!(f, "\n{b}"),
            Expression::Literal(l) => write!(f, "{l}"),
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
            DataType::String(str) => write!(f, "str:<{}>", str)
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

