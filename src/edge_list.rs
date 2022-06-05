use super::graph::Graph;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Edge<W> {
    pub left: usize,
    pub right: usize,
    pub weight: W,
}

/// Edge List representation of a graph.
/// 
/// This implementation simply stores edges in a vector.
/// `EdgeList` can be useful when there is heavily adding edge operation.
/// On the other hand, it might be a good idea to use `AdjacencyMatrix`
/// when you need to run neighbors function many times.
/// Also, of course, you need to choose other graph representations
/// if you treat a graph containing orphan node since `EdgeList` cannot add an node 
/// without adding edge.
///
/// ```
/// use gcollections::edge_list::{EdgeList};
/// use gcollections::Graph;
/// 
/// let edge_list = EdgeList::<i32>::new();
/// ```
/// 
/// For each function, there is a note for the time complexity
/// where n and m denotes # of nodes, # of edges respectively.
#[derive(Debug)]
pub struct EdgeList<W> {
    edges: Vec<Edge<W>>,
}

impl<W> EdgeList<W> {
    pub fn new() -> Self {
        EdgeList { edges: vec![] }
    }

    pub fn len(&self) -> usize {
        self.edges.len()
    }
}

impl<W> Graph<W> for EdgeList<W> {
    type Vertex = usize;
    type VertexValue = usize;
    type Edge = Edge<W>;
    type EdgeNodes = (Self::Vertex, Self::Vertex);

    /// There is no implementation of add_vertex for Edge List graph representation.
    /// 
    /// # Panics
    /// Always panics
    fn add_vertex(&mut self, _vertex: Self::VertexValue) {
        unimplemented!();
    }

    /// Removes a node and all edges incident to it.
    /// 
    /// This operation should compute in *O*(*m*) time.
    /// 
    /// 
    /// # Examples
    /// ```
    /// use gcollections::edge_list::{Edge, EdgeList};
    /// use gcollections::Graph;
    /// 
    /// let mut edge_list = EdgeList::<i32>::new();
    /// 
    /// edge_list.add_edge(Edge { left: 1, right: 2, weight: 10 });
    /// edge_list.add_edge(Edge { left: 1, right: 3, weight: 10 });
    /// edge_list.add_edge(Edge { left: 1, right: 4, weight: 10 });
    /// edge_list.add_edge(Edge { left: 2, right: 5, weight: 10 });
    /// 
    /// edge_list.remove_vertex(4);
    /// assert_eq!(edge_list.len(), 3);
    /// edge_list.remove_vertex(1);
    /// assert_eq!(edge_list.len(), 1);
    /// ```
    fn remove_vertex(&mut self, vertex_id: usize) {
        let result = self.edges.iter()
            .enumerate()
            .filter(|&(_, edge)| edge.left == vertex_id || edge.right == vertex_id)
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();
        
        // Iterate from the latter index to detele all with `swap_remove`
        for index in 0..result.len() {
            self.edges.swap_remove(result.len() - index);
        }
    }

    /// Adds an edge to the graph.
    /// 
    /// This operation should compute in *O*(1) time.
    /// 
    /// Note that  if you try to add edges between the same pair of nodes multiple times, 
    /// it accepts all those edges without checking duplicates.
    /// 
    /// 
    /// # Examples
    /// ```
    /// use gcollections::edge_list::{Edge, EdgeList};
    /// use gcollections::Graph;
    /// 
    /// let mut edge_list = EdgeList::<i32>::new();
    /// 
    /// edge_list.add_edge(Edge { left: 1, right: 2, weight: 10 });
    /// assert_eq!(edge_list.len(), 1);
    /// edge_list.add_edge(Edge { left: 1, right: 3, weight: 10 });
    /// assert_eq!(edge_list.len(), 2);
    /// edge_list.add_edge(Edge { left: 1, right: 4, weight: 10 });
    /// assert_eq!(edge_list.len(), 3);
    /// ```
    fn add_edge(&mut self, edge: Self::Edge) {
        self.edges.push(edge);
    }

    /// Deletes and returns edge from the edge list, 
    /// or None if the specified edge does not exist.
    /// 
    /// This operation should compute in *O*(*m*) time.
    /// 
    /// 
    /// # Examples
    /// ```
    /// use gcollections::edge_list::{Edge, EdgeList};
    /// use gcollections::Graph;
    /// 
    /// let mut edge_list = EdgeList::<i32>::new();
    /// 
    /// let edge1 = Edge { left: 1, right: 2, weight: 10 };
    /// let edge2 = Edge { left: 1, right: 3, weight: 10 };
    /// let edge3 = Edge { left: 1, right: 4, weight: 10 };
    /// edge_list.add_edge(edge1.clone());
    /// edge_list.add_edge(edge2.clone());
    /// edge_list.add_edge(edge3.clone());
    /// 
    /// assert_eq!(edge_list.remove_edge((1, 2)).unwrap(), edge1);
    /// assert_eq!(edge_list.remove_edge((1, 2)), None);
    /// assert_eq!(edge_list.remove_edge((1, 4)).unwrap(), edge3);
    /// assert_eq!(edge_list.len(), 1);
    /// ```
    fn remove_edge(&mut self, target: Self::EdgeNodes) -> Option<Self::Edge> {
        let result = self.edges.iter()
            .enumerate()
            .find(|&(_, edge)| edge.left == target.0 && edge.right == target.1);
        match result {
            Some((index, _)) => {
                let edge = self.edges.swap_remove(index);
                return Some(edge)
            },
            None => return None,
        }
    }

    /// Returns all neighbor nodes.
    /// 
    /// This function assumes two nodes are neighbors 
    /// if there's an edge between them regardless of its direction.
    /// 
    /// This operation should compute in *O*(*m*) time.
    /// 
    /// 
    /// # Examples
    /// ```
    /// use gcollections::edge_list::{Edge, EdgeList};
    /// use gcollections::Graph;
    /// 
    /// let mut edge_list = EdgeList::<i32>::new();
    /// 
    /// let edge1 = Edge { left: 1, right: 2, weight: 10 };
    /// let edge2 = Edge { left: 1, right: 3, weight: 10 };
    /// let edge3 = Edge { left: 1, right: 4, weight: 10 };
    /// edge_list.add_edge(edge1);
    /// edge_list.add_edge(edge2);
    /// edge_list.add_edge(edge3);
    /// 
    /// assert_eq!(edge_list.neighbors(1), vec![(&2, &10), (&3, &10), (&4, &10)]);
    /// assert_eq!(edge_list.neighbors(2), vec![(&1, &10)]);
    /// assert_eq!(edge_list.neighbors(5), Vec::<(&usize, &i32)>::new());
    /// ```
    fn neighbors(&self, vertex_id: usize) -> Vec<(&Self::Vertex, &W)> {
        self.edges.iter()
            .filter(|&edge| edge.left == vertex_id || edge.right == vertex_id)
            .map(|edge| -> (&Self::Vertex, &W) {
                if edge.left == vertex_id {
                    return (&edge.right, &edge.weight)
                } else {
                    return (&edge.left, &edge.weight)
                }
            })
            .collect::<Vec<(&Self::Vertex, &W)>>()
    }

    /// Returns vertex if found, otherwise returns None.
    /// 
    /// This operation should compute in *O*(*m*) time.
    /// 
    /// # Examples
    /// ```
    /// use gcollections::edge_list::{Edge, EdgeList};
    /// use gcollections::Graph;
    /// 
    /// let mut edge_list = EdgeList::<i32>::new();
    /// 
    /// let edge1 = Edge { left: 1, right: 2, weight: 10 };
    /// let edge2 = Edge { left: 1, right: 3, weight: 10 };
    /// let edge3 = Edge { left: 1, right: 4, weight: 10 };
    /// edge_list.add_edge(edge1);
    /// edge_list.add_edge(edge2);
    /// edge_list.add_edge(edge3);
    /// 
    /// assert_eq!(edge_list.find_vertex(1), Some(&1));
    /// assert_eq!(edge_list.find_vertex(5), None);
    /// ```
    fn find_vertex(&self, vertex_id: Self::VertexValue) -> Option<&Self::Vertex> {
        let edge = self.edges.iter()
            .find(|&edge| edge.left == vertex_id || edge.right == vertex_id);
        match edge {
            Some(edge) => {
                if edge.left == vertex_id {
                    return Some(&edge.left)
                } else {
                    return Some(&edge.right)
                }
            },
            None => return None,
        }
    }
}
