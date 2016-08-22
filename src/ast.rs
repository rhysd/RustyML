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

pub type Expr = Box<Expr>;
pub enum Node {
    Unit,
    Bool(bool),
    Int(i32),
    Float(f64),
    Not(Expr),
    Neg(Expr),
    Add(Expr, Expr),
    Sub(Expr, Expr),
    FNot(Expr),
    FNeg(Expr),
    FAdd(Expr, Expr),
    FSub(Expr, Expr),
    FMul(Expr, Expr),
    FDiv(Expr, Expr),
    Eq(Expr, Expr),
    LessEq(Expr, Expr),
    If(Expr, Expr, Expr),
    Let(String, Expr, Expr), // let name = expr in expr
    Var(String),
    LetRec(String, Vec<String>, Expr, Expr), // let rec name x y z = expr in expr
    Apply(Expr, Vec<Expr>),
    Tuple(Vec<Expr>),
    LetTuple(Vec<String>, Expr, Expr), // let (x, y, z) = expr in expr
    Array(Expr, Expr),
    Get(Expr, Expr),
    Put(Expr, Expr, Expr),
}
