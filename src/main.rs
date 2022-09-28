use std::ops;
use std::rc::Rc;
use std::fmt;

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
    label: String,
    // grad: f32,
    // _backward: f32,
    // _op: Ops
}

impl Value {
    pub fn new(data: f32, label: &str) -> Self {
       Value{ data:data,
        _prev: (None, None),
        label: label.to_string()}
    }
}

impl ops::Add for Value {
    type Output = Self;

    fn add(self, other: Self) -> Self {

        let left_op = Box::new(self.clone());
        let right_op = Box::new(other.clone());
        Self{ data: self.data+other.data, _prev: (Some(left_op), Some(right_op)), label: "+".to_string() }
    }
}

impl ops::Mul for Value {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let left_op = Box::new(self.clone());
        let right_op = Box::new(other.clone());
        Self{ data: self.data*other.data, _prev: (Some(left_op), Some(right_op)), label: '*'.to_string()}
    }

}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {}| data: {:.4} }}", self.label, self.data)
    }
}

trait Trace {
    fn trace(&self) -> Graph<&Value, &str>;
}

use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;


impl Trace for Value {
    fn trace(&self) -> Graph<&Value, &str> {
        let mut graph = Graph::<&Value, &str>::new();
        let origin = graph.add_node(&self);
        fn build<'a>(graph: &mut Graph<&'a Value, &str>, node: &'a Value, index: NodeIndex) {

            match &node._prev.0 {
                Some(v) => {
                    let destination_1 = graph.add_node(&v);
                    let cost_1 = graph.add_edge(index, destination_1,"");
                    build(graph, v, destination_1);
                },
                None => ()
            }
            
            match &node._prev.1 {
                Some(v) => {
                    let destination_1 = graph.add_node(&v);
                    let cost_1 = graph.add_edge(index, destination_1,"");
                    build(graph,v, destination_1);
                },
                None => ()
            }

        }


        build(&mut graph, &self, origin);
        graph
    }

}

use petgraph::dot::{Dot, Config};
fn main() {

    let a = Value::new(2.0, "a"); 
    let b = Value::new(-3.0, "b");
    let c = Value::new(10.0,"c");
    let mut e =a*b;
    e.label='e'.to_string();
    let mut d = e+c;
    d.label = 'd'.to_string();
    let f = Value::new(-2.0, "f");
    let mut L =d*f;
    L.label = 'L'.to_string();

    //println!("{:?},{:?},{:?},{:?},{:?}", a,b,c,d,e);
    //println!("{}",L);
    let graph = L.trace();
    //println!("{}", Dot::new(&graph));
    //Dot::with_attr_getters(&graph, config, get_edge_attributes, get_node_attributes);
}

#[cfg(test)]

mod test {
    use super::*;
    #[test]
    fn add(){
        let a = Value::new(2.0,"a"); // Value{data:2.0, _prev: (None, None), label:'a'.to_string()};
        let b = Value::new(-3.0, "b"); //Value{data:-3.0, _prev: (None, None), label: 'b'.to_string()};
        let c =a+b;
       // let d = a*b;
        assert_eq!(c.data, -1.0);
        
    }

    #[test]
    fn mul(){
        let a = Value::new(2.0, "a"); //Value{data:2.0, _prev: (None, None), label: 'a'.to_string()};
        let b = Value::new(-3.0, "b"); //Value{data:-3.0, _prev: (None, None), label: 'b'.to_string()};
        let c = Value::new(10.0, "c");//Value{data:10.0, _prev: (None, None), label: 'c'.to_string()};
        let d =a*b+c;
        assert_eq!(d.data, 4.0);

    }


}
