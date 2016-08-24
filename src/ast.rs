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
    line: u32,
    col: u32,
}

macro_rules! node_structs {
    ($($n:ident { $($m:ident: $t:ty,)* })+) => {
        $(
            #[derive(Debug)]
            pub struct $n {
                $($m: $t,)*
                pos: Position,
            }
            impl $n {
                pub fn new($($m: $t,)* line: u32, col: u32) -> Expr {
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
        )+
        #[derive(Debug)]
        pub enum Node {
            $(
                $n($n),
            )+
        }
    }
}

node_structs! {
    Unit {}

    Bool {
        value: bool,
    }

    Int {
        value: i32,
    }

    Float {
        value: f64,
    }

    Not {
        child: Expr,
    }

    Neg {
        child: Expr,
    }

    Add {
        lhs: Expr,
        rhs: Expr,
    }

    Sub {
        lhs: Expr,
        rhs: Expr,
    }

    FNot {
        child: Expr,
    }

    FNeg {
        child: Expr,
    }

    FAdd {
        lhs: Expr,
        rhs: Expr,
    }

    FSub {
        lhs: Expr,
        rhs: Expr,
    }

    FMul {
        lhs: Expr,
        rhs: Expr,
    }

    FDiv {
        lhs: Expr,
        rhs: Expr,
    }

    Eq {
        lhs: Expr,
        rhs: Expr,
    }

    LessEq {
        lhs: Expr,
        rhs: Expr,
    }

    If {
        cond: Expr,
        then: Expr,
        else_: Expr,
    }

    Let {
        name: String,
        bound: Expr,
        body: Expr,
    }

    Var {
        name: String,
    }

    LetRec {
        name: String,
        params: Vec<String>,
        bound: Expr,
        body: Expr,
    }

    Apply {
        callee: Expr,
        args: Vec<Expr>,
    }

    Tuple {
        elements: Vec<Expr>,
    }

    LetTuple {
        names: Vec<String>,
        bound: Expr,
        body: Expr,
    }

    Array {
        size: Expr,
        elem: Expr,
    }

    Get {
        array: Expr,
        index: Expr,
    }

    Put {
        array: Expr,
        index: Expr,
        new_value: Expr,
    }
}

pub type Expr = Box<Node>;

#[test]
fn test_create_node() {
    match *Bool::new(true, 1, 2) {
        Node::Bool(b) => {
            assert_eq!(b.value, true);
            assert_eq!(b.pos.line, 1);
            assert_eq!(b.pos.col, 2);
        },
        _ => assert!(false),
    }

    match *Add::new(Bool::new(true, 1, 2), Unit::new(1, 2), 1, 2) {
        Node::Add(add) => {
            match *add.lhs {
                Node::Bool(b) => {
                    assert_eq!(b.value, true);
                    assert_eq!(b.pos.line, 1);
                    assert_eq!(b.pos.col, 2);
                },
                _ => assert!(false),
            }
            match *add.rhs {
                Node::Unit(u) => {
                    assert_eq!(u.pos.line, 1);
                    assert_eq!(u.pos.col, 2);
                },
                _ => assert!(false),
            }
            assert_eq!(add.pos.line, 1);
            assert_eq!(add.pos.col, 2);
        },
        _ => assert!(false),
    }
}
