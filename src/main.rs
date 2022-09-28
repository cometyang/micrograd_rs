use std::fmt;
use std::fmt::Debug;
use std::ops;

trait ValuePrint: fmt::Display {
    fn value_print(&self) {
        let output = self.to_string();
        println!("{}", output)
    }
}

type ValueRef = Option<Box<Value>>;

#[derive(Clone, Debug)]
struct Value {
    data: f32,
    //grad: f32,
    _prev: (ValueRef, ValueRef),
    label: Option<String>,
    // grad: f32,
    // _backward: f32,
    _op: Option<String>,
}

impl Value {
    pub fn new(data: f32, label: &str) -> Self {
        Value {
            data: data,
            _prev: (None, None),
            label: Some(label.to_string()),
            _op: None,
        }
    }
}

impl ops::Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let left_op = Box::new(self.clone());
        let right_op = Box::new(other.clone());
        Self {
            data: self.data + other.data,
            _prev: (Some(left_op), Some(right_op)),
            _op: Some("+".to_string()),
            label: None,
        }
    }
}

impl ops::Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let left_op = Box::new(self.clone());
        let right_op = Box::new(other.clone());
        Self {
            data: self.data * other.data,
            _prev: (Some(left_op), Some(right_op)),
            _op: Some('*'.to_string()),
            label: None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.label {
            Some(ref node_label) => write!(f, "{{ {}| data: {:.4} }}", node_label, self.data), // data node
            None => write!(f, "{}", self._op.as_ref().unwrap()),
        }
    }
}

trait Trace {
    fn trace(&self) -> Graph<String, String>;
}

use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;

impl Trace for Value {
    fn trace(&self) -> Graph<String, String> {
        let mut graph = Graph::<String, String>::new();

        let origin = graph.add_node(self.to_string());
        fn build<'a>(graph: &mut Graph<String, String>, node: &'a Value, index: NodeIndex) {
            match &node._op {
                Some(op) => {
                    let op_index = graph.add_node(format!("{}", op.to_string()));
                    graph.add_edge(op_index, index, "".to_string());

                    match &node._prev.0 {
                        Some(v) => {
                            let destination_1 = graph.add_node(v.to_string());
                            graph.add_edge(destination_1, op_index, "".to_string());
                            build(graph, v, destination_1);
                        }
                        None => (),
                    }

                    match &node._prev.1 {
                        Some(v) => {
                            let destination_1 = graph.add_node(v.to_string());
                            graph.add_edge(destination_1, op_index, "".to_string());
                            build(graph, v, destination_1);
                        }
                        None => (),
                    }
                }
                None => (),
            }
        }

        build(&mut graph, &self, origin);
        graph
    }
}

use petgraph::dot::{Config, Dot};
fn main() {
    let a = Value::new(2.0, "a");
    let b = Value::new(-3.0, "b");
    let c = Value::new(10.0, "c");
    let mut e = a * b;
    e.label = Some('e'.to_string());
    let mut d = e + c;
    d.label = Some('d'.to_string());
    let f = Value::new(-2.0, "f");
    let mut l = d * f;
    l.label = Some('L'.to_string());

    let graph = l.trace();

    println!(
        "{}",
        Dot::with_attr_getters(
            &graph,
            &[Config::EdgeNoLabel],
            &|_, _| format!(""),
            &|_, nr| {
                let (_, weight) = nr;
                match weight.as_str() {
                    "*" => format!(""),
                    "+" => format!(""),
                    _ => format!("shape=record"),
                }
            },
        )
    );
}

#[cfg(test)]

mod test {
    use super::*;
    #[test]
    fn add() {
        let a = Value::new(2.0, "a");
        let b = Value::new(-3.0, "b");
        let c = a + b;
        // let d = a*b;
        assert_eq!(c.data, -1.0);
    }

    #[test]
    fn mul() {
        let a = Value::new(2.0, "a");
        let b = Value::new(-3.0, "b");
        let c = Value::new(10.0, "c");
        let d = a * b + c;
        assert_eq!(d.data, 4.0);
    }

    #[test]
    fn test_graph() {
        let a = Value::new(2.0, "a");
        let b = Value::new(-3.0, "b");
        let c = Value::new(10.0, "c");
        let mut e = a * b;
        e.label = Some('e'.to_string());
        let mut d = e + c;
        d.label = Some('d'.to_string());
        let f = Value::new(-2.0, "f");
        let mut l = d * f;
        l.label = Some('L'.to_string());

        let graph = l.trace();

        let dot_graph = Dot::with_attr_getters(
            &graph,
            &[Config::EdgeNoLabel],
            &|_, _| format!(""),
            &|_, nr| {
                let (_, weight) = nr;
                match weight.as_str() {
                    "*" => format!(""),
                    "+" => format!(""),
                    _ => format!("shape=record"),
                }
            },
        );
        assert_eq!(dot_graph.to_string(), String::from("digraph {\n    \
        0 [ label = \"{ L| data: -8.0000 }\" shape=record]\n    \
        1 [ label = \"*\" ]\n    \
        2 [ label = \"{ d| data: 4.0000 }\" shape=record]\n    \
        3 [ label = \"+\" ]\n    \
        4 [ label = \"{ e| data: -6.0000 }\" shape=record]\n    \
        5 [ label = \"*\" ]\n    \
        6 [ label = \"{ a| data: 2.0000 }\" shape=record]\n    \
        7 [ label = \"{ b| data: -3.0000 }\" shape=record]\n    \
        8 [ label = \"{ c| data: 10.0000 }\" shape=record]\n    \
        9 [ label = \"{ f| data: -2.0000 }\" shape=record]\n    \
        1 -> 0 [ ]\n    \
        2 -> 1 [ ]\n    \
        3 -> 2 [ ]\n    \
        4 -> 3 [ ]\n    \
        5 -> 4 [ ]\n    \
        6 -> 5 [ ]\n    \
        7 -> 5 [ ]\n    \
        8 -> 3 [ ]\n    \
        9 -> 1 [ ]\n}\n"));
    }
}
