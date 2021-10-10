use yajlish::ndjson_handler::Selector;

#[derive(Debug, PartialEq)]
pub enum Num {
    Float(f64),
    PosInt(u64),
    Int(i64),
}

#[derive(Debug, PartialEq)]
pub enum Term {
    Num(Num),
    Value(Vec<Selector>),
    ProbGenerator(ProbGenerator),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Term(Term),
    Op(Box<Expr>, BinaryOp, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Abs,
    Sqrt,
    Ceiling,
    Floor,
    Trunc,
    Cos,
    Sin,
    Tan,
    ACos,
    ASin,
    ATan,
    Cosh,
    Sinh,
    Tanh,
    ACosh,
    ASinh,
    ATanh,
    Log,
    Log10,
    Exp,
    ToUpper,
    ToLower,
    ToString,
    IsDouble,
    IsFloatNan,
    IsNull,
    ToDouble,
    ToInt,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Div,
    Mul,
    Pow,
}

#[derive(Debug, PartialEq)]
pub enum ProbGenerator {
    RBern { seed: u64, prob: f64 },
    RBeta { seed: u64, shape1: u64, shape2: u64 },
    RBinom { seed: u64, size: u64, prob: f64 },
    RCauchey { seed: u64, scale: u64 },
    RChiSq { seed: u64, df: u64 },
    RF { seed: u64, df1: u64, df2: u64 },
    RGamma { seed: u64, shape: u64 },
    RGeom { seed: u64, prob: f64 },
    RHyper { seed: u64, m: u64, n: u64, k: u64 },
    RLNorm { seed: u64 },
    RLogis { seed: u64 },
    RNBinom { seed: u64, size: u64, prob: f64 },
    RNorm { seed: u64 },
    RPois { seed: u64, lambda: u64 },
}
