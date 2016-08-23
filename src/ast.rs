// Type t = (* MinCamlの構文を表現するデータ型 (caml2html: syntax_t) *)
//   | Unit
//   | Bool of bool
//   | Int of int
//   | Float of float
//   | Not of t
//   | Neg of t
//   | Add of t * t
//   | Sub of t * t
//   | FNeg of t
//   | FAdd of t * t
//   | FSub of t * t
//   | FMul of t * t
//   | FDiv of t * t
//   | Eq of t * t
//   | LE of t * t
//   | If of t * t * t
//   | Let of (Id.t * Type.t) * t * t
//   | Var of Id.t
//   | LetRec of fundef * t
//   | App of t * t list
//   | Tuple of t list
//   | LetTuple of (Id.t * Type.t) list * t * t
//   | Array of t * t
//   | Get of t * t
//   | Put of t * t * t
// and fundef = { name : Id.t * Type.t; args : (Id.t * Type.t) list; body : t }

#[derive(Debug)]
pub struct Position {
    line: i32,
    col: i32,
}

// Note:
// node_struct!(Foo {...}) defines 'struct Foo {...}' with Position member and
// Foo::new() factory function.
macro_rules! node_struct {
    ($n:ident { $($m:ident: $t:ty,)* }) => {
        #[derive(Debug)]
        pub struct $n {
            $($m: $t,)*
            pos: Position,
        }
        impl $n {
            pub fn new($($m: $t,)* line: i32, col: i32) -> Expr {
                Expr::new(
                    Node::$n(
                        $n {
                            $($m: $m,)*
                            pos: Position {line: line, col: col}
                        }
                    )
                )
            }
        }
    }
}

node_struct!(Unit {});

node_struct!(Bool {
    value: bool,
});

node_struct!(Int {
    value: i32,
});

node_struct!(Float {
    value: f64,
});

node_struct!(Not {
    child: Expr,
});

node_struct!(Neg {
    child: Expr,
});

node_struct!(Add {
    lhs: Expr,
    rhs: Expr,
});

node_struct!(Sub {
    lhs: Expr,
    rhs: Expr,
});

node_struct!(FNot {
    child: Expr,
});

node_struct!(FNeg {
    child: Expr,
});

node_struct!(FAdd {
    lhs: Expr,
    rhs: Expr,
});

node_struct!(FSub {
    lhs: Expr,
    rhs: Expr,
});

node_struct!(FMul {
    lhs: Expr,
    rhs: Expr,
});

node_struct!(FDiv {
    lhs: Expr,
    rhs: Expr,
});

node_struct!(Eq {
    lhs: Expr,
    rhs: Expr,
});

node_struct!(LessEq {
    lhs: Expr,
    rhs: Expr,
});

node_struct!(If {
    cond: Expr,
    then: Expr,
    else_: Expr,
});

node_struct!(Let {
    name: String,
    bound: Expr,
    body: Expr,
});

node_struct!(Var {
    name: String,
});

node_struct!(LetRec {
    name: String,
    params: Vec<String>,
    bound: Expr,
    body: Expr,
});

node_struct!(Apply {
    callee: Expr,
    args: Vec<Expr>,
});

node_struct!(Tuple {
    elements: Vec<Expr>,
});

node_struct!(LetTuple {
    names: Vec<String>,
    bound: Expr,
    body: Expr,
});

node_struct!(Array {
    size: Expr,
    elem: Expr,
});

node_struct!(Get {
    array: Expr,
    index: Expr,
});

node_struct!(Put {
    array: Expr,
    index: Expr,
    new_value: Expr,
});

pub type Expr = Box<Node>;

#[derive(Debug)]
pub enum Node {
    Unit(Unit),
    Bool(Bool),
    Int(Int),
    Float(Float),
    Not(Not),
    Neg(Neg),
    Add(Add),
    Sub(Sub),
    FNot(FNot),
    FNeg(FNeg),
    FAdd(FAdd),
    FSub(FSub),
    FMul(FMul),
    FDiv(FDiv),
    Eq(Eq),
    LessEq(LessEq),
    If(If),
    Let(Let),
    Var(Var),
    LetRec(LetRec),
    Apply(Apply),
    Tuple(Tuple),
    LetTuple(LetTuple),
    Array(Array),
    Get(Get),
    Put(Put),
}
