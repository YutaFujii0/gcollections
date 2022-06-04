/// Graph trait
/// 
/// A common behavior that a graph have.

pub trait Graph {
    type Vertex;
    type VertexValue;
    type Edge;
    type EdgeNodes;

    fn add_vertex(&mut self, vertex: Self::VertexValue);

    fn remove_vertex(&mut self, vertex_id: usize);

    fn add_edge(&mut self, edge: Self::Edge);

    fn remove_edge(&mut self, edge: Self::EdgeNodes) -> Option<Self::Edge>;

    fn neighbors(&self, vertex_id: usize) -> Vec<&Self::Vertex>;

    fn find_vertex(&self, vertex: Self::VertexValue) -> Option<&Self::Vertex>;
}
