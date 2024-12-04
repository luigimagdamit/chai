use std::fmt;
#[derive(Clone, PartialEq, Eq)]
#[allow(unused)]
pub enum DataType {
    Integer(i32),
    String(String),
    Boolean(bool)
}
impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Integer(int) => write!(f, "int: <{}>", int),
            DataType::Boolean(bool) => write!(f, "bool <{}>", bool),
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
#[allow(unused)]
impl Expr {
    pub fn print_leaf(&self) {
        println!("<leaf> <left: {}> <right: {}> <data_type: {}>", self.left, self.right, self.data_type);
    }
}