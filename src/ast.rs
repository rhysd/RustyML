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
    start_byte_index: usize,
    end_byte_index: usize,
    line: usize,
    column: usize,
    offset: usize,
}

pub trait ChildrenGettable {
    fn children<'a>(&'a self) -> Vec<&'a Node>;
}

macro_rules! generate_push_to_children {
    (Expr, $m:ident) => {
        children.push(self.$m);
    };
    (Vec<Expr>, $m:ident) => {{
        for child in self.$m.iter() {
            children.push(child);
        }
    }};
    (Option<Expr>, $m:ident) => {{
        if self.$m.is_some() {
            children.push(self.$m.unwrap());
        }
    }};
    ($t:ty, $m:ident) => ();
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
                pub fn new($($m: $t,)* s: usize, e: usize) -> Expr {
                    Expr::new(
                        Node::$n(
                            $n {
                                $($m: $m,)*
                                pos: Position {
                                    start_byte_index: s,
                                    end_byte_index: e,
                                    line: 0,
                                    column: 0,
                                    offset: e - s,
                                }
                            }
                        )
                    )
                }
            }
            impl ChildrenGettable for $n {
                fn children<'a>(&'a self) -> Vec<&'a Node> {
                    let children = vec![];
                    $(
                        generate_push_to_children!($t, $m);
                    )*
                    children
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

trace_macros!(true);
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
trace_macros!(false);

pub type Expr = Box<Node>;

#[test]
fn test_create_node() {
    match *Bool::new(true, 1, 2) {
        Node::Bool(b) => {
            assert_eq!(b.value, true);
            assert_eq!(b.pos.start_byte_index, 1);
            assert_eq!(b.pos.end_byte_index, 2);
            assert_eq!(b.pos.offset, 1);
        },
        _ => assert!(false),
    }

    match *Add::new(Bool::new(true, 1, 2), Unit::new(1, 2), 1, 2) {
        Node::Add(add) => {
            match *add.lhs {
                Node::Bool(b) => {
                    assert_eq!(b.value, true);
                    assert_eq!(b.pos.start_byte_index, 1);
                    assert_eq!(b.pos.end_byte_index, 2);
                    assert_eq!(b.pos.offset, 1);
                },
                _ => assert!(false),
            }
            match *add.rhs {
                Node::Unit(u) => {
                    assert_eq!(u.pos.start_byte_index, 1);
                    assert_eq!(u.pos.end_byte_index, 2);
                    assert_eq!(u.pos.offset, 1);
                },
                _ => assert!(false),
            }
            assert_eq!(add.pos.start_byte_index, 1);
            assert_eq!(add.pos.end_byte_index, 2);
            assert_eq!(add.pos.offset, 1);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_children() {
    match *Bool::new(true, 1, 2) {
        Node::Bool(b) => {
            assert_eq!(b.children().len(), 0);
        },
        _ => unreachable!(),
    }
    match *Add::new(Bool::new(true, 1, 2), Unit::new(1, 2), 1, 2) {
        Node::Add(add) => {
            assert_eq!(add.children().len(), 2);
            match *add.children().first().unwrap() {
                &Node::Bool(ref b) => {
                    assert!(b.value);
                },
                _ => unreachable!(),
            }
        },
        _ => unreachable!(),
    }
}
