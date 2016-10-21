use std::fmt;
use std::rc::Rc;

enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

enum UnOp {
    Negate,
}

pub trait Node: fmt::Display {
    fn value(&self) -> f64;
    fn children(&self) -> Vec<Rc<Node>>;
}

pub struct BinaryOp {
    id: usize,
    op: BinOp,
    lhs: Rc<Node>,
    rhs: Rc<Node>,
}

impl BinaryOp {
    pub fn add(lhs: Rc<Node>, rhs: Rc<Node>) -> Self {
        BinaryOp {
            id: 0,
            op: BinOp::Add,
            lhs: lhs,
            rhs: rhs,
        }
    }

    pub fn subtract(lhs: Rc<Node>, rhs: Rc<Node>) -> Self {
        BinaryOp {
            id: 0,
            op: BinOp::Subtract,
            lhs: lhs,
            rhs: rhs,
        }
    }

    pub fn multiply(lhs: Rc<Node>, rhs: Rc<Node>) -> Self {
        BinaryOp {
            id: 0,
            op: BinOp::Multiply,
            lhs: lhs,
            rhs: rhs,
        }
    }

    pub fn divide(lhs: Rc<Node>, rhs: Rc<Node>) -> Self {
        BinaryOp {
            id: 0,
            op: BinOp::Divide,
            lhs: lhs,
            rhs: rhs,
        }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.op {
            BinOp::Add => write!(f, "+"),
            BinOp::Subtract => write!(f, "-"),
            BinOp::Multiply => write!(f, "*"),
            BinOp::Divide => write!(f, "/"),
        }
    }
}

impl Node for BinaryOp {
    fn value(&self) -> f64 {
        match self.op {
            BinOp::Add => self.lhs.value() + self.rhs.value(),
            BinOp::Subtract => self.lhs.value() - self.rhs.value(),
            BinOp::Multiply => self.lhs.value() * self.rhs.value(),
            BinOp::Divide => self.lhs.value() / self.rhs.value(),
        }
    }

    fn children(&self) -> Vec<Rc<Node>> {
        vec![self.lhs.clone(), self.rhs.clone()]
    }
}

pub struct UnaryOp {
    id: usize,
    op: UnOp,
    operand: Rc<Node>,
}

impl UnaryOp {
    pub fn negate(operand: Rc<Node>) -> Self {
        UnaryOp {
            id: 0,
            op: UnOp::Negate,
            operand: operand,
        }
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.op {
            UnOp::Negate => write!(f, "-"),
        }
    }
}

impl Node for UnaryOp {
    fn value(&self) -> f64 {
        match self.op {
            UnOp::Negate => -self.operand.value(),
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
