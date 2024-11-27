pub enum Precedence {
    PrecNone,
    PrecAssignment,
    PrecAnd,
    PrecEquality,
    PrecComparison,
    PrecTerm,
    PrecFactor,
    PrecUnary,
    PrecCall,
    PrecPrimary
}
impl Precedence {
    // Convert a number to a Precedence enum variant
    pub fn from_u32(value: u32) -> Precedence {
        match value {
            0 => Precedence::PrecNone,
            1 => Precedence::PrecAssignment,
            2 => Precedence::PrecAnd,
            3 => Precedence::PrecEquality,
            4 => Precedence::PrecComparison,
            5 => Precedence::PrecTerm,
            6 => Precedence::PrecFactor,
            7 => Precedence::PrecUnary,
            8 => Precedence::PrecCall,
            9 => Precedence::PrecPrimary,
            _ => Precedence::PrecNone

        }
    }
    pub fn to_u32(&self) -> u32 {
        match self {
            Precedence::PrecNone => 0,
            Precedence::PrecAssignment => 1,
            Precedence::PrecAnd => 2,
            Precedence::PrecEquality => 3,
            Precedence::PrecComparison => 4,
            Precedence::PrecTerm => 5,
            Precedence::PrecFactor => 6,
            Precedence::PrecUnary => 7,
            Precedence::PrecCall => 8,
            Precedence::PrecPrimary => 9,
        }
    }
}