pub const PARSE_FN_OUTPUT: bool = true;

pub const PARSE_TOKEN_OUTPUT: bool = false;

// pub const LLVM_DEBUG_OUTPUT: bool = false;


// LLVM Codegen Flags===========================================================================================
// GENERAL
pub const PARSE_CONSTANT_FOLD: bool = false; // toggles constant folding in math operations such as 1 * 2 - 3 etc.

// EXPRESSION MODE
pub const PARSE_DECLARATION_MODE: bool = true; // make compiler expect a function main body and not include a mainless helper
pub const PARSE_EXPRESSION_MODE: bool = false && !PARSE_DECLARATION_MODE; // will make it so that top level expressions are printed to output as functions like in LLVM tutorial
pub const PARSE_SUPRESS_PREDEFINES: bool = true && PARSE_EXPRESSION_MODE; // turn off predefined things, only see top level expression codegen

pub const EXPR_ONLY: bool = true; // only generates the expression function without a main body