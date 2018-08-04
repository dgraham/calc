use std::fmt;
use std::rc::Rc;

enum Binary {
    Add,
    Subtract,
    Multiply,
    Divide,
}

enum Unary {
    Negate,
}

pub trait Node: fmt::Display {
    fn id(&self) -> usize;

    fn value(&self) -> f64;

    fn children(&self) -> Vec<Rc<Node>>;
}

pub struct BinaryOp {
    id: usize,
    op: Binary,
    lhs: Rc<Node>,
    rhs: Rc<Node>,
}

impl BinaryOp {
    pub fn add(id: usize, lhs: Rc<Node>, rhs: Rc<Node>) -> Self {
        BinaryOp {
            id: id,
            op: Binary::Add,
            lhs: lhs,
            rhs: rhs,
        }
    }

    pub fn subtract(id: usize, lhs: Rc<Node>, rhs: Rc<Node>) -> Self {
        BinaryOp {
            id: id,
            op: Binary::Subtract,
            lhs: lhs,
            rhs: rhs,
        }
    }

    pub fn multiply(id: usize, lhs: Rc<Node>, rhs: Rc<Node>) -> Self {
        BinaryOp {
            id: id,
            op: Binary::Multiply,
            lhs: lhs,
            rhs: rhs,
        }
    }

    pub fn divide(id: usize, lhs: Rc<Node>, rhs: Rc<Node>) -> Self {
        BinaryOp {
            id: id,
            op: Binary::Divide,
            lhs: lhs,
            rhs: rhs,
        }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.op {
            Binary::Add => write!(f, "+"),
            Binary::Subtract => write!(f, "-"),
            Binary::Multiply => write!(f, "*"),
            Binary::Divide => write!(f, "/"),
        }
    }
}

impl Node for BinaryOp {
    fn id(&self) -> usize {
        self.id
    }

    fn value(&self) -> f64 {
        match self.op {
            Binary::Add => self.lhs.value() + self.rhs.value(),
            Binary::Subtract => self.lhs.value() - self.rhs.value(),
            Binary::Multiply => self.lhs.value() * self.rhs.value(),
            Binary::Divide => self.lhs.value() / self.rhs.value(),
        }
    }

    fn children(&self) -> Vec<Rc<Node>> {
        vec![self.lhs.clone(), self.rhs.clone()]
    }
}

pub struct UnaryOp {
    id: usize,
    op: Unary,
    operand: Rc<Node>,
}

impl UnaryOp {
    pub fn negate(id: usize, operand: Rc<Node>) -> Self {
        UnaryOp {
            id: id,
            op: Unary::Negate,
            operand: operand,
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.op {
            Unary::Negate => write!(f, "-"),
        }
    }
}

impl Node for UnaryOp {
    fn id(&self) -> usize {
        self.id
    }

    fn value(&self) -> f64 {
        match self.op {
            Unary::Negate => -self.operand.value(),
        }
    }

    fn children(&self) -> Vec<Rc<Node>> {
        vec![self.operand.clone()]
    }
}

pub struct Constant {
    id: usize,
    value: u64,
}

impl Constant {
    pub fn new(id: usize, value: u64) -> Self {
        Constant {
            id: id,
            value: value,
        }
    }
}

impl fmt::Display for Constant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value.to_string())
    }
}

impl Node for Constant {
    fn id(&self) -> usize {
        self.id
    }

    fn value(&self) -> f64 {
        self.value as f64
    }

    fn children(&self) -> Vec<Rc<Node>> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::{BinaryOp, Constant, Node, UnaryOp};
    use std::rc::Rc;

    #[test]
    fn it_adds() {
        let lhs = Rc::new(Constant::new(0, 1));
        let rhs = Rc::new(Constant::new(0, 2));
        assert_eq!(3.0, BinaryOp::add(0, lhs, rhs).value());
    }

    #[test]
    fn it_subtracts() {
        let lhs = Rc::new(Constant::new(0, 1));
        let rhs = Rc::new(Constant::new(0, 2));
        assert_eq!(-1.0, BinaryOp::subtract(0, lhs, rhs).value());
    }

    #[test]
    fn it_multiplies() {
        let lhs = Rc::new(Constant::new(0, 2));
        let rhs = Rc::new(Constant::new(0, 3));
        assert_eq!(6.0, BinaryOp::multiply(0, lhs, rhs).value());
    }

    #[test]
    fn it_divides() {
        let lhs = Rc::new(Constant::new(0, 6));
        let rhs = Rc::new(Constant::new(0, 2));
        assert_eq!(3.0, BinaryOp::divide(0, lhs, rhs).value());
    }

    #[test]
    fn it_negates() {
        let rhs = Rc::new(Constant::new(0, 2));
        assert_eq!(-2.0, UnaryOp::negate(0, rhs).value());
    }
}
