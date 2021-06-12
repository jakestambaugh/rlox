#[derive(Debug, Clone)]
pub enum Literal {
    LoxNumber(f64),
    LoxString(String),
    LoxIdentifier(String),
}
