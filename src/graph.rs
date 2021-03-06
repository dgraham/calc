use std::fmt;
use std::fmt::Write;
use std::rc::Rc;

use iter::Iter;
use node::Node;

pub struct Graph;

impl Graph {
    pub fn dot(node: Rc<Node>) -> String {
        let iter = Iter::new(node);
        let pieces: Vec<String> = iter.map(|node| Graph::stmt(node).unwrap()).collect();
        format!("strict graph {{\n{}\n}}", pieces.join("\n"))
    }

    fn stmt(node: Rc<Node>) -> Result<String, fmt::Error> {
        let mut buffer = String::new();

        // Add node statement.
        write!(buffer, "  {} [ label = \"{}\" ]", node.id(), node)?;

        // Add edge statements.
        if !node.children().is_empty() {
            write!(buffer, "\n  {} -- {{", node.id())?;
            for child in node.children() {
                write!(buffer, " {}", child.id())?;
            }
            write!(buffer, " }}")?;
        }

        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::Graph;
    use node::{BinaryOp, Constant};
    use std::rc::Rc;

    #[test]
    fn it_converts_to_dot_syntax() {
        let lhs = Rc::new(Constant::new(1, 1));
        let rhs = Rc::new(Constant::new(2, 2));
        let op = BinaryOp::add(0, lhs, rhs);
        let dot = Graph::dot(Rc::new(op));
        assert_eq!(
            "strict graph {\n  0 [ label = \"+\" ]\n  0 -- { 1 2 }\n  1 [ label = \"1\" \
             ]\n  2 [ label = \"2\" ]\n}",
            dot
        );
    }
}
