/// Generic IR abstraction traits for control flow operations
/// This allows swapping between different IR backends (LLVM, custom IR, etc.)

pub trait BranchIR {
    /// Get the depth/nesting level of this branch
    fn depth(&self) -> u32;

    /// Generate an unconditional jump to the end of this control structure
    fn jump_to_end(&self) -> String;
}

pub trait ConditionalIR: BranchIR {
    /// Generate a conditional branch instruction
    /// bool_reg: register containing the boolean condition
    fn conditional_branch(&self, bool_reg: u32) -> String;

    /// Generate the "then" block label
    fn then_label(&self) -> String;

    /// Generate the "else" block label
    fn else_label(&self) -> String;

    /// Generate the end label for the entire if statement
    fn end_label(&self) -> String;

    /// Generate else-if branch (optional, for advanced if statements)
    fn else_if_branch(&self, bool_reg: u32, else_if_index: u32) -> String {
        // Default implementation - can be overridden
        self.conditional_branch(bool_reg)
    }

    /// Generate else-if label (optional)
    fn else_if_label(&self, else_if_index: u32) -> String {
        // Default implementation - can be overridden
        format!("elseif{}_{}", self.depth(), else_if_index)
    }
}

pub trait LoopIR: BranchIR {
    /// Generate an unconditional jump to the condition check
    fn jump_to_condition(&self) -> String;

    /// Generate the condition check label
    fn condition_label(&self) -> String;

    /// Generate a conditional branch for the loop
    /// bool_reg: register containing the loop condition
    fn condition_branch(&self, bool_reg: u32) -> String;

    /// Generate the loop body label
    fn body_label(&self) -> String;

    /// Generate the loop exit label
    fn exit_label(&self) -> String;
}

/// Factory trait for creating IR implementations
pub trait IRFactory {
    type ConditionalIR: ConditionalIR;
    type LoopIR: LoopIR;

    fn create_conditional(&self, depth: u32) -> Self::ConditionalIR;
    fn create_loop(&self, depth: u32) -> Self::LoopIR;
}