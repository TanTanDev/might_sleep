/// defines what duration setting for CpuLimiter to use
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Usage {
    /// almost no processing needs be done, high sleeping time
    Low,
    /// normal use, minimal sleeping time
    Normal,
}
