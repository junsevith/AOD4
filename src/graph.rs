use std::fmt::{Debug, Display, Formatter};

#[derive(Clone)]
pub struct Graph {
    pub vertices: Vec<Vertex>,
    pub edge_count: usize,
}

#[derive(Debug, Clone)]
pub struct Vertex {
    pub number: usize,
    pub edges: Vec<Edge>,
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub destination: usize,
    pub weight: usize,
}

impl Graph {
    pub fn with_vertices(count: usize) -> Graph {
        let vertices = (0..count).map(Vertex::new).collect();
        Graph { vertices, edge_count: 0 }
    }

    pub fn custom(vertex_count: usize, edges_per_vertex:usize) -> Graph {
        let vertices = (0..vertex_count).map(|i| Vertex { number: i, edges: Vec::with_capacity(edges_per_vertex) }).collect();
        Graph { vertices, edge_count: 0 }
    }

    pub fn add_edge(&mut self, source: usize, destination: usize, weight: usize) {
        self.vertices[source]
            .edges
            .push(Edge::new(destination, weight));
        self.edge_count += 1;
    }

    pub fn add_edge_undirected(&mut self, source: usize, destination: usize, weight: usize) {
        self.add_edge(source, destination, weight);
        self.add_edge(destination, source, weight);
    }

    pub fn get_edge(&self, source: usize, destination: usize) -> Option<&Edge> {
        // edges are sorted by destination due to the specific generation of the hypercube
        // self.vertices[source].edges.binary_search_by_key(&destination, |edge| edge.destination).ok().map(|index| &self.vertices[source].edges[index])
        self.vertices[source].edge_to(destination)
    }

    pub fn get_edge_mut(&mut self, source: usize, destination: usize) -> Option<&mut Edge> {
        // edges are sorted by destination due to the specific generation of the hypercube
        // self.vertices[source].edges.binary_search_by_key(&destination, |edge| edge.destination).ok().map(move |index| &mut self.vertices[source].edges[index])
        self.vertices[source].edge_to_mut(destination)
    }

    pub fn remove_edge(&mut self, source: usize, destination: usize) {
        if let Some(index) = self.vertices[source].edges.iter().position(|edge| edge.destination == destination) {
            self.vertices[source].edges.swap_remove(index);
            self.edge_count -= 1;
        }
    }
}

impl Debug for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        write!(f, "{}", self)?;

        if self.vertices.len() < 20 {
            for vertex in &self.vertices {
                s.push_str(&format!("Vertex {}\n", vertex.number));
                for edge in &vertex.edges {
                    s.push_str(&format!(
                        "  -> {} (weight: {})\n",
                        edge.destination, edge.weight
                    ));
                }
            }
        }

        write!(f, "{}", s)
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Graph with {} vertices and {} edges\n", self.vertices.len(), self.edge_count)
    }
}

impl Vertex {
    fn new(number: usize) -> Vertex {
        Vertex {
            number,
            edges: Vec::new(),
        }
    }

    pub fn edge_to(&self, destination: usize) -> Option<&Edge> {
        self.edges.iter().find(|edge| edge.destination == destination)
    }

    pub fn edge_to_mut(&mut self, destination: usize) -> Option<&mut Edge> {
        self.edges.iter_mut().find(|edge| edge.destination == destination)
    }
}

impl Edge {
    pub fn new(destination: usize, weight: usize) -> Edge {
        Edge {
            destination,
            weight,
        }
    }
}
