enum NodeType {
    Program,
    NumericLiteral,
    Identifier,
    BinaryExpr,
}

pub struct Stmt {
    kind: NodeType
}

pub struct Program {
    kind: NodeType,
    body: Vec<Stmt>,
}

impl Default for Program {
    fn default() -> Program {
        Program {
            kind: NodeType.Program,
        }
    }
}

pub struct Expr {}

pub struct BinaryExpr {
    kind: NodeType,
    left: Expr,
    right: Expr,
    operator: String,
}

impl Default for BinaryExpr {
    fn default() -> BinaryExpr {
        BinaryExpr {
            kind: NodeType.BinaryExpr,
        }
    }
}

pub struct Identifier {
    kind: NodeType,
    symbol: String,
}

impl Default for Identifier {
    fn default() -> Identifier {
        Identifier {
            kind: NodeType.Identifier,
        }
    }
}

pub struct NumericLiteral {
    kind: NodeType,
    value: Integer
}

impl Default for NumericLiteral {
    fn default() -> NumericLiteral {
        NumericLiteral {
            kind: NodeType.NumericLiteral,
        }
    }
}