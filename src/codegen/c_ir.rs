use super::ir_traits::{BranchIR, ConditionalIR, LoopIR, IRFactory};

/// C language IR implementation for conditional statements (if/else)
#[derive(Clone, Debug)]
pub struct CConditional {
    depth: u32,
}

impl CConditional {
    pub fn new(depth: u32) -> Self {
        Self { depth }
    }
}

impl BranchIR for CConditional {
    fn depth(&self) -> u32 {
        self.depth
    }

    fn jump_to_end(&self) -> String {
        format!("goto end{};", self.depth)
    }
}

impl ConditionalIR for CConditional {
    fn conditional_branch(&self, bool_reg: u32) -> String {
        format!("if (reg{}) goto then{}; else goto else{};", bool_reg, self.depth, self.depth)
    }

    fn then_label(&self) -> String {
        format!("then{}:", self.depth)
    }

    fn else_label(&self) -> String {
        format!("else{}:", self.depth)
    }

    fn end_label(&self) -> String {
        format!("end{}:", self.depth)
    }
}

/// C language IR implementation for loop statements (while/for)
#[derive(Clone, Debug)]
pub struct CLoop {
    depth: u32,
}

impl CLoop {
    pub fn new(depth: u32) -> Self {
        Self { depth }
    }
}

impl BranchIR for CLoop {
    fn depth(&self) -> u32 {
        self.depth
    }

    fn jump_to_end(&self) -> String {
        format!("goto exit{};", self.depth)
    }
}

impl LoopIR for CLoop {
    fn jump_to_condition(&self) -> String {
        format!("goto cond{};", self.depth)
    }

    fn condition_label(&self) -> String {
        format!("cond{}:", self.depth)
    }

    fn condition_branch(&self, bool_reg: u32) -> String {
        format!("if (reg{}) goto body{}; else goto exit{};", bool_reg, self.depth, self.depth)
    }

    fn body_label(&self) -> String {
        format!("body{}:", self.depth)
    }

    fn exit_label(&self) -> String {
        format!("exit{}:", self.depth)
    }
}

/// Factory for creating C IR implementations
pub struct CIRFactory;

impl IRFactory for CIRFactory {
    type ConditionalIR = CConditional;
    type LoopIR = CLoop;

    fn create_conditional(&self, depth: u32) -> Self::ConditionalIR {
        CConditional::new(depth)
    }

    fn create_loop(&self, depth: u32) -> Self::LoopIR {
        CLoop::new(depth)
    }
}