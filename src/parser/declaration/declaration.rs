use std::fmt;
use crate::parser::expression::expr::{DataType, Expression, Operation};
use crate::parser::visitor::visitor::{Accept, Visitor};
use crate::codegen::llvm_codegen::LlvmPrint;
use crate::codegen::codegen_print::CodegenPrint;
use crate::parser::expression::expr::ExprNode;

#[derive(Clone)]
pub struct PrintStatement {
    pub expression: Expression
}

#[derive(Clone)]
pub struct ConditionalStatement {
    pub condition: Expression,
    pub then_block: Vec<Declaration>,
    pub else_ifs: Vec<ElseIfClause>,
    pub else_block: Option<Vec<Declaration>>,
}

#[derive(Clone)]
pub struct ElseIfClause {
    pub condition: Expression,
    pub block: Vec<Declaration>,
}
impl From<Expression> for PrintStatement {
    fn from(expression: Expression) -> Self {
        PrintStatement {
            expression
        }
    }
}
#[derive(Clone)]
pub enum Statement {
    PrintStatement(PrintStatement),
    Conditional(ConditionalStatement)
}
impl From<PrintStatement> for Statement {
    fn from(print_statement: PrintStatement) -> Statement {
        Statement::PrintStatement(print_statement)
    }
}

impl From<ConditionalStatement> for Statement {
    fn from(conditional: ConditionalStatement) -> Statement {
        Statement::Conditional(conditional)
    }
}

impl ConditionalStatement {
    pub fn new(
        condition: Expression,
        then_block: Vec<Declaration>,
    ) -> Self {
        Self {
            condition,
            then_block,
            else_ifs: Vec::new(),
            else_block: None,
        }
    }

    pub fn with_else(mut self, else_block: Vec<Declaration>) -> Self {
        self.else_block = Some(else_block);
        self
    }

    pub fn with_else_if(mut self, condition: Expression, block: Vec<Declaration>) -> Self {
        self.else_ifs.push(ElseIfClause { condition, block });
        self
    }
}

impl ElseIfClause {
    pub fn new(condition: Expression, block: Vec<Declaration>) -> Self {
        Self { condition, block }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::PrintStatement(expr) => write!(f, "\nPrint Statement => \n  | {}", expr.expression),
            Statement::Conditional(cond) => write!(f, "\nIf Statement => Condition: {}", cond.condition)
        }
    }
}
#[derive(Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub variable_type: DataType,
    pub expression: Option<Expression> // sometimes will have no value right?

}
impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(expr) = &self.expression {
            write!(f, "Variable Name: {}, Type: {},  Expresion: {}", self.name, "TODO", expr)
        } else {
            write!(f, "Variable Name: {}", self.name)
        }
    }
}
impl VariableDeclaration {

    pub fn as_datatype(&self) -> DataType {
        if let Some(expr) = &self.expression {
            expr.as_datatype()
        } else {
            self.variable_type.clone()
        }
    }
    pub fn print(&self) -> String {
        LlvmPrint::new_variable(&self) + &"\n\t" + &LlvmPrint::store_variable(&self)
    }
}

#[derive(Clone)]
pub enum Declaration {
    Statement(Statement),
    Variable(VariableDeclaration)
}
impl Accept for Declaration {
    fn accept<V: Visitor> (&self, visitor: &mut V) -> String{
        match self {
            Declaration::Statement(statement) => {
                match statement {
                    Statement::PrintStatement(print_statement) => visitor.visit_print(print_statement),
                    Statement::Conditional(_conditional) => "TODO: implement conditional visitor".to_string()
                }
            },
            Declaration::Variable(var_declaration) => visitor.visit_variable_declaration(var_declaration)
        }
    }
}
impl From<PrintStatement> for Declaration {
    fn from(value: PrintStatement) -> Self {
        Declaration::Statement(Statement::PrintStatement(value))
    }
}

impl From<ConditionalStatement> for Declaration {
    fn from(value: ConditionalStatement) -> Self {
        Declaration::Statement(Statement::Conditional(value))
    }
}
impl Declaration {
    pub fn new_variable(name: &str, expression: Option<Expression>, variable_type: DataType) -> Declaration {
        Declaration::Variable(VariableDeclaration {
            name: name.to_string(),
            variable_type,
            expression
        })
    }
    pub fn as_variable(&self) -> &VariableDeclaration {
        match self {
            Declaration::Variable(inner) => {
                inner
            },
            _ => panic!()
        }
    }
}

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Declaration::Variable(var_struct) => {
                write!(f, "{var_struct}")
            },
            _ => panic!()
        }
    }
}

impl PrintStatement {
    pub fn print(&self) -> String {
        
        match &self.expression {
            Expression::Binary(binary) => {
                match binary.operator {
                    Operation::Equal | Operation::GreaterEqual | Operation::GreaterThan |Operation::LessEqual |Operation::LessThan | Operation::NotEqual => {
                        LlvmPrint::print_i1(&Expression::from(binary))
                    },
                    _ => LlvmPrint::print_i32(&Expression::from(binary))
                }
            },
            Expression::Literal(literal) => {
                match literal {
                    DataType::Integer(_) => LlvmPrint::print_i32(&Expression::from(literal)),
                    DataType::Boolean(_) => LlvmPrint::print_i1(&Expression::from(literal)), 
                    _ => panic!()
                }
            },
            Expression::StringConstant(str_constant) => {
                str_constant.print()
            }
            Expression::Variable(variable) => {
                match variable.datatype {
                    DataType::Integer(_) => LlvmPrint::print_i32(&Expression::from(variable.clone())),
                    DataType::Boolean(_) => LlvmPrint::print_i1(&Expression::from(variable.clone())),
                    DataType::String(_) => LlvmPrint::print_str_constant(&Expression::from(variable.clone())),
                }
            },
            _ => panic!("Unrecognized print statement expression input")
        }
    }
}