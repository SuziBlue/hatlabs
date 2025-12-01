use core::marker::PhantomData;

use crate::geometry::{Arithmetic, Region};

use super::Component;



pub struct Node<T: Arithmetic, Reg: Region<T>, NodeValue> {
    pub region: Reg,
    pub value: NodeValue,
    _marker: PhantomData<T>,
}

pub struct Edge<'a, T: Arithmetic, Reg: Region<T>, NodeValue, EdgeValue> {
    pub from: &'a Node<T, Reg, NodeValue>,
    pub to: &'a Node<T, Reg, NodeValue>,
    pub value: EdgeValue,
}

pub struct Graph<'a, T, Reg, N, NodeValue, E, EdgeValue> 
where 
    T: Arithmetic + 'a,
    Reg: Region<T> + 'a,
    N: AsRef<[Node<T, Reg, NodeValue>]>,
    NodeValue: 'a,
    E: AsRef<[Edge<'a, T, Reg, NodeValue, EdgeValue>]>,
{
    nodes: N,
    edges: E,
    _edge_value: PhantomData<EdgeValue>,
    _node_value: PhantomData<NodeValue>,
    _reg: PhantomData<Reg>,
    _t : &'a PhantomData<T>,
}

impl<'a, T, Reg, N, NodeValue, E, EdgeValue> Component for Graph<'a, T, Reg, N, NodeValue, E, EdgeValue>
where 
    T: Arithmetic + 'a,
    Reg: Region<T> + 'a,
    N: AsRef<[Node<T, Reg, NodeValue>]> + Clone + Default,
    NodeValue: 'a,
    E: AsRef<[Edge<'a, T, Reg, NodeValue, EdgeValue>]> + Clone + Default,
{
    type State = (N, E);

    fn state(&self) -> Self::State {
        (self.nodes.clone(), self.edges.clone())
    }

    fn update(&mut self, new_state: Self::State) {
        (self.nodes, self.edges) = new_state;
    }
}





