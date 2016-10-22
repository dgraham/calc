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
    pub fn add(lhs: Rc<Node>, rhs: Rc<Node>) -> Self {
        BinaryOp {
            id: 0,
            op: Binary::Add,
            lhs: lhs,
            rhs: rhs,
        }
    }

    pub fn subtract(lhs: Rc<Node>, rhs: Rc<Node>) -> Self {
        BinaryOp {
            id: 0,
            op: Binary::Subtract,
            lhs: lhs,
            rhs: rhs,
        }
    }

    pub fn multiply(lhs: Rc<Node>, rhs: Rc<Node>) -> Self {
        BinaryOp {
            id: 0,
            op: Binary::Multiply,
            lhs: lhs,
            rhs: rhs,
        }
    }

    pub fn divide(lhs: Rc<Node>, rhs: Rc<Node>) -> Self {
        BinaryOp {
            id: 0,
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
    pub fn negate(operand: Rc<Node>) -> Self {
        UnaryOp {
            id: 0,
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
    pub fn new(value: u64) -> Self {
        Constant {
            id: 0,
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
    fn value(&self) -> f64 {
        self.value as f64
    }

    fn children(&self) -> Vec<Rc<Node>> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use super::{BinaryOp, Constant, Node, UnaryOp};

    #[test]
    fn it_adds() {
        let lhs = Rc::new(Constant::new(1));
        let rhs = Rc::new(Constant::new(2));
        assert_eq!(3.0, BinaryOp::add(lhs, rhs).value());
    }

    #[test]
    fn it_subtracts() {
        let lhs = Rc::new(Constant::new(1));
        let rhs = Rc::new(Constant::new(2));
        assert_eq!(-1.0, BinaryOp::subtract(lhs, rhs).value());
    }

    #[test]
    fn it_multiplies() {
        let lhs = Rc::new(Constant::new(2));
        let rhs = Rc::new(Constant::new(3));
        assert_eq!(6.0, BinaryOp::multiply(lhs, rhs).value());
    }

    #[test]
    fn it_divides() {
        let lhs = Rc::new(Constant::new(6));
        let rhs = Rc::new(Constant::new(2));
        assert_eq!(3.0, BinaryOp::divide(lhs, rhs).value());
    }

    #[test]
    fn it_negates() {
        let rhs = Rc::new(Constant::new(2));
        assert_eq!(-2.0, UnaryOp::negate(rhs).value());
    }
}
