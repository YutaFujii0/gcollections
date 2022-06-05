use std::cmp::PartialEq;

use super::graph::Graph;

#[derive(Debug, PartialEq, Eq)]
pub struct Vertex<T: PartialEq> {
    pub value: T,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Weight<W>
where W: Clone + PartialEq
{
    Real(W),
    NonExist,
}

/// Adjacency Matrix representation of a graph.
/// 
/// This implementation stores vertices in a vector and hold their relationship in a 2D matrix.
/// The value at (i, j) means the weight of edge i -> j or implies that vertex i and j are not connected
/// if it is `Weight::NonExist`.
/// 
/// `AdjacencyMatrix` is suitable when you need to see neighbors frequently.
/// On the other hand, adding edge operation is relatively expensive.
///
/// ```
/// use gcollections::adjacency_matrix::{AdjacencyMatrix};
/// use gcollections::Graph;
/// 
/// let adj_matrix = AdjacencyMatrix::<String, i32>::new();
/// ```
/// 
/// For each function, there is a note for the time complexity
/// where n and m denotes # of nodes, # of edges respectively.
#[derive(Debug)]
pub struct AdjacencyMatrix<T, W>
where
    T: PartialEq,
    W: Clone + PartialEq,
{
    matrix: Vec<Vec<Weight<W>>>,
    pub vertices: Vec<Vertex<T>>,
}

impl<T, W> AdjacencyMatrix<T, W>
where
    T: PartialEq,
    W: Clone + PartialEq,
{
    pub fn new() -> Self {
        AdjacencyMatrix { matrix: vec![], vertices: vec![] }
    }
}

impl<T, W> Graph<W> for AdjacencyMatrix<T, W>
where
    T: PartialEq,
    W: Clone + PartialEq,
{
    type Vertex = Vertex<T>;
    type VertexValue = T;
    type Edge = (usize, usize, W);
    type EdgeNodes = (usize, usize);

    /// Add a vertex to the graph.
    /// 
    /// This operation should compute in *O*(*n*) time.
    /// 
    /// # Examples
    /// ```
    /// use gcollections::adjacency_matrix::{AdjacencyMatrix};
    /// use gcollections::Graph;
    /// 
    /// let mut adj_matrix = AdjacencyMatrix::<String, i32>::new();
    /// 
    /// adj_matrix.add_vertex(String::from("Seatle"));
    /// adj_matrix.add_vertex(String::from("Boston"));
    /// adj_matrix.add_vertex(String::from("New York"));
    /// 
    /// assert_eq!(adj_matrix.vertices.len(), 3);
    /// ```
    fn add_vertex(&mut self, value: T) {
        let n = self.vertices.len();
        self.vertices.push(Vertex { value });
        for row in &mut self.matrix {
            row.push(Weight::NonExist);
        }
        
        self.matrix.push(vec![Weight::NonExist; n + 1]);
    }

    /// Remove a vertex from the graph.
    /// 
    /// This operation should compute in *O*(*n*) time.
    /// 
    /// # Panic
    /// 
    /// Panics if vertex_index is out of range.
    /// 
    /// # Examples
    /// ```
    /// use gcollections::adjacency_matrix::{AdjacencyMatrix};
    /// use gcollections::Graph;
    /// 
    /// let mut adj_matrix = AdjacencyMatrix::<String, i32>::new();
    /// 
    /// adj_matrix.add_vertex(String::from("Seatle"));
    /// adj_matrix.add_vertex(String::from("Boston"));
    /// adj_matrix.add_vertex(String::from("New York"));
    /// 
    /// adj_matrix.remove_vertex(0);
    /// assert_eq!(adj_matrix.vertices.len(), 2);
    /// 
    /// adj_matrix.remove_vertex(0);
    /// assert_eq!(adj_matrix.vertices.len(), 1);
    /// ```
    fn remove_vertex(&mut self, vertex_index: usize) {
        self.vertices.remove(vertex_index);
        // Deliberately not using swap_remove because it causes us to swap 
        // corresponding columns of all other rows for consistency.
        self.matrix.remove(vertex_index);
        for row in &mut self.matrix {
            row.remove(vertex_index);
        }
    }

    /// Adds an edge to the graph.
    /// 
    /// This operation should compute in *O*(1) time.
    /// 
    /// # Examples
    /// ```
    /// use gcollections::adjacency_matrix::{AdjacencyMatrix};
    /// use gcollections::Graph;
    /// 
    /// let mut adj_matrix = AdjacencyMatrix::<String, i32>::new();
    /// 
    /// adj_matrix.add_vertex(String::from("Seatle"));
    /// adj_matrix.add_vertex(String::from("Boston"));
    /// adj_matrix.add_vertex(String::from("New York"));
    /// 
    /// adj_matrix.add_edge((0, 1, 1000));
    /// adj_matrix.add_edge((1, 2, 10));
    /// ```
    fn add_edge(&mut self, edge: Self::Edge) {
        self.matrix[edge.0][edge.1] = Weight::Real(edge.2);
    }

    /// Deletes and returns edge from the edge list, 
    /// or None if the specified edge does not exist.
    /// 
    /// This operation should compute in *O*(1) time.
    /// 
    /// # Examples
    /// ```
    /// use gcollections::adjacency_matrix::{AdjacencyMatrix};
    /// use gcollections::Graph;
    /// 
    /// let mut adj_matrix = AdjacencyMatrix::<String, i32>::new();
    /// 
    /// adj_matrix.add_vertex(String::from("Seatle"));
    /// adj_matrix.add_vertex(String::from("Boston"));
    /// adj_matrix.add_vertex(String::from("New York"));
    /// 
    /// adj_matrix.add_edge((0, 1, 1000));
    /// adj_matrix.add_edge((1, 2, 10));
    /// 
    /// assert_eq!(adj_matrix.remove_edge((0, 1)).unwrap(), (0, 1, 1000));
    /// assert_eq!(adj_matrix.remove_edge((0, 2)), None);
    /// assert_eq!(adj_matrix.remove_edge((0, 1)), None);
    /// ```
    fn remove_edge(&mut self, edge: Self::EdgeNodes) -> Option<Self::Edge> {
        match &mut self.matrix[edge.0][edge.1] {
            Weight::Real(weight) => {
                let weight = weight.clone();
                self.matrix[edge.0][edge.1] = Weight::NonExist;
                return Some((edge.0, edge.1, weight))
            },
            Weight::NonExist => return None,
        }
    }

    /// only neighbor to which there exists an edge
    /// Returns all neighbor nodes.
    /// 
    /// This function assumes that a vertex j is a neighbor of i
    /// if there is a edge from i to j.
    /// The opposite case (edge j -> i) does not apply.
    /// 
    /// This operation should compute in *O*(*n*) time.
    /// 
    /// # Examples
    /// ```
    /// use gcollections::adjacency_matrix::{AdjacencyMatrix};
    /// use gcollections::Graph;
    /// 
    /// let mut adj_matrix = AdjacencyMatrix::<String, i32>::new();
    /// 
    /// adj_matrix.add_vertex(String::from("Seatle"));
    /// adj_matrix.add_vertex(String::from("Boston"));
    /// adj_matrix.add_vertex(String::from("New York"));
    /// 
    /// adj_matrix.add_edge((0, 1, 1000));
    /// adj_matrix.add_edge((0, 2, 1200));
    /// adj_matrix.add_edge((1, 2, 10));
    /// 
    /// assert_eq!(adj_matrix.neighbors(0).len(), 2);
    /// assert_eq!(adj_matrix.neighbors(1).len(), 1);
    /// assert_eq!(adj_matrix.neighbors(2).len(), 0);
    /// ```
    fn neighbors(&self, vertex_index: usize) -> Vec<(&Self::Vertex, &W)> {
        let indices = self
            .matrix[vertex_index]
            .iter()
            .enumerate()
            .filter(|&(_, v)| v != &Weight::NonExist)
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();
        indices.iter()
            .map(|&index| -> (&Self::Vertex, &W) {
                let weight = &self.matrix[vertex_index][index];
                match weight {
                    Weight::Real(weight) => return (&self.vertices[index], weight),
                    _ => panic!("Something unexpected")
                }
            })
            .collect::<Vec<(&Self::Vertex, &W)>>()
    }

    /// Returns vertex if found, otherwise returns None.
    /// 
    /// This operation should compute in *O*(*n*) time.
    /// 
    /// # Examples
    /// ```
    /// use gcollections::adjacency_matrix::{Vertex, AdjacencyMatrix};
    /// use gcollections::Graph;
    /// 
    /// let mut adj_matrix = AdjacencyMatrix::<String, i32>::new();
    /// 
    /// adj_matrix.add_vertex(String::from("Seatle"));
    /// adj_matrix.add_vertex(String::from("Boston"));
    /// adj_matrix.add_vertex(String::from("New York"));
    /// 
    /// assert_eq!(adj_matrix.find_vertex(String::from("Seatle")).unwrap().value, String::from("Seatle"));
    /// assert_eq!(adj_matrix.find_vertex(String::from("San Francisco")), None);
    /// ```
    fn find_vertex(&self, vertex_value: Self::VertexValue) -> Option<&Self::Vertex> {
        self.vertices.iter().find(|&vertex| vertex.value == vertex_value)
    }
}
