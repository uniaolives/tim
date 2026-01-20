use thiserror::Error;
use crate::ast::OntoType;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Unsupported type: {0:?}")]
    UnsupportedType(OntoType),
    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),
    #[error("Other error: {0}")]
    Other(String),
}

pub type CompilerResult<T> = Result<T, CompilerError>;

// --- Estruturas de retorno ---
#[derive(Debug, Clone)]
pub struct CompiledContract {
    pub target_language: String,
    pub source_code: String,
    pub bytecode: Option<Vec<u8>>,
    pub abi: Option<serde_json::Value>,
    pub stats: CompilationStats,
}

#[derive(Debug, Clone)]
pub struct CompilationStats {
    pub functions_compiled: usize,
    pub contracts_deployed: usize,
    pub transmutations_applied: usize,
    pub diplomatic_constraints: usize,
    pub paradigm_guards_injected: usize,
    pub gas_estimate: u64,
}
