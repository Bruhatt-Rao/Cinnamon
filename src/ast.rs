#[derive(Debug)]
enum NodeType {
    Program,
    NumericLiteral,
    Identifier,
    BinaryExpr,
}

#[derive(Debug)]
pub struct Stmt {
    kind: NodeType
}

#[derive(Debug)]
pub struct Program {
    pub kind: NodeType,
    pub body: Vec<Stmt>,
}

impl Program {
    pub fn new() -> Program {
        let kind = NodeType::Program;
        let body = Vec::<Stmt>::new();
        Program { kind, body }
    }
}

pub struct Expr {}

pub struct BinaryExpr {
    kind: NodeType,
    left: Expr,
    right: Expr,
    operator: String,
}

impl BinaryExpr {
    pub fn new(left: Expr, right: Expr, operator: String) -> BinaryExpr {
        let kind = NodeType::BinaryExpr;
        BinaryExpr { kind, left, right, operator }
    }
}

pub struct Identifier {
    kind: NodeType, 
    symbol: String,
}

impl Identifier {
    pub fn new(symbol: String) -> Identifier {
        let kind = NodeType::Identifier;
        Identifier { kind, symbol }
    }
}

pub struct NumericLiteral {
    kind: NodeType,
    value: i32
}

impl NumericLiteral {
    pub fn new(value: i32) -> NumericLiteral {
        let kind = NodeType::NumericLiteral;
        NumericLiteral { kind, value }
    }
}