use super::ir_traits::{BranchIR, ConditionalIR, LoopIR, IRFactory};

/// LLVM-specific implementation for conditional statements (if/else)
#[derive(Clone, Debug)]
pub struct LlvmConditional {
    depth: u32,
}

impl LlvmConditional {
    pub fn new(depth: u32) -> Self {
        Self { depth }
    }
}

impl BranchIR for LlvmConditional {
    fn depth(&self) -> u32 {
        self.depth
    }

    fn jump_to_end(&self) -> String {
        format!("br label %end{}", self.depth)
    }
}

impl ConditionalIR for LlvmConditional {
    fn conditional_branch(&self, bool_reg: u32) -> String {
        format!("br i1 %{}, label %then{}, label %else{}", bool_reg, self.depth, self.depth)
    }

    fn then_label(&self) -> String {
        format!("then{}:", self.depth)
    }

    fn else_label(&self) -> String {
        format!("\nelse{}:", self.depth)
    }

    fn end_label(&self) -> String {
        format!("\nend{}:", self.depth)
    }

    fn else_if_branch(&self, bool_reg: u32, else_if_index: u32) -> String {
        format!("br i1 %{}, label %elseif{}_{}, label %else{}", bool_reg, self.depth, else_if_index, self.depth)
    }

    fn else_if_label(&self, else_if_index: u32) -> String {
        format!("\nelseif{}_{}:", self.depth, else_if_index)
    }
}

/// LLVM-specific implementation for loop statements (while/for)
#[derive(Clone, Debug)]
pub struct LlvmLoop {
    depth: u32,
}

impl LlvmLoop {
    pub fn new(depth: u32) -> Self {
        Self { depth }
    }
}

impl BranchIR for LlvmLoop {
    fn depth(&self) -> u32 {
        self.depth
    }

    fn jump_to_end(&self) -> String {
        format!("br label %exit{}", self.depth)
    }
}

impl LoopIR for LlvmLoop {
    fn jump_to_condition(&self) -> String {
        format!("br label %cond{}", self.depth)
    }

    fn condition_label(&self) -> String {
        format!("\ncond{}:", self.depth)
    }

    fn condition_branch(&self, bool_reg: u32) -> String {
        format!("br i1 %{}, label %body{}, label %exit{}", bool_reg, self.depth, self.depth)
    }

    fn body_label(&self) -> String {
        format!("\nbody{}:", self.depth)
    }

    fn exit_label(&self) -> String {
        format!("\nexit{}:", self.depth)
    }
}

/// Factory for creating LLVM IR implementations
pub struct LlvmIRFactory;

impl IRFactory for LlvmIRFactory {
    type ConditionalIR = LlvmConditional;
    type LoopIR = LlvmLoop;

    fn create_conditional(&self, depth: u32) -> Self::ConditionalIR {
        LlvmConditional::new(depth)
    }

    fn create_loop(&self, depth: u32) -> Self::LoopIR {
        LlvmLoop::new(depth)
    }
}