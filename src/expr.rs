use std::fmt;
#[derive(Clone)]
pub enum DataType {
    Integer(u32)
}
impl<'a> fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataType::Integer(int) => write!(f, "int:{}", int)
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

impl Expr {
    pub fn print_leaf(&self) {
        println!("<leaf> <left: {}> <right: {}> <data_type: {}>", self.left, self.right, self.data_type);
    }
}
// struct Root {
//     left: Expr,
//     right: Expr,
//     operator: Operator
// }