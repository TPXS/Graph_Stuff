struct DisjointSets {
    parent: Vec<usize>,
}

impl DisjointSets {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
        }
    }

    fn find(&mut self, element: usize) -> usize {
        let parent_of_element = self.parent[element];
        if parent_of_element != element {
            self.parent[element] = self.find(parent_of_element);
        }
        
        self.parent[element]
    }

    fn merge(&mut self, (u, v): (usize, usize)) -> bool {
        let (parent_of_u, parent_of_v) = (self.find(u), self.find(v));
        self.parent[parent_of_u] = parent_of_v;
        parent_of_u != parent_of_v
    }

}

struct AdjacencyListIterator<'a> {
    graph: &'a Graph,
    next_edge: Option<usize>,
}

impl<'a> Iterator for AdjacencyListIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.next_edge.map(|edge| {
            let vertex = self.graph.edge_to_vertex[edge];
            self.next_edge = self.graph.edge_to_edge[edge];
            (edge, vertex)
        })
    }
}

struct Graph {
    vertex_to_edge: Vec<Option<usize>>,
    edge_to_edge: Vec<Option<usize>>,
    edge_to_vertex: Vec<usize>,
}

impl Graph {
    fn new(max_vertices: usize, max_edges_est: usize) -> Self {
        Self {
            vertex_to_edge: vec!(None; max_vertices),
            edge_to_edge: Vec::with_capacity(max_edges_est),
            edge_to_vertex: Vec::with_capacity(max_edges_est),
    
        }
    }

    fn get_num_vertices(&self) -> usize {
        self.vertex_to_edge.len()
    }

    fn get_num_edges(&self) -> usize {
        self.edge_to_vertex.len()
    }

    fn add_directed_edge(&mut self, (u, v): (usize, usize)) {
        self.edge_to_edge.push(self.vertex_to_edge[u]);
        self.vertex_to_edge[u] = Some(self.get_num_edges());
        self.edge_to_vertex.push(v);
    }

    fn add_undirected_edge(&mut self, (u, v): (usize, usize)) {
        self.add_directed_edge((u,v));
        self.add_directed_edge((v,u));
    }

    fn boolean_satistfiability(&mut self, u: usize, v: usize) {
        self.add_directed_edge((u ^ 1, v));
        self.add_directed_edge((v ^ 1, u));
    }

    fn get_adjacency_list(&self, u: usize) -> AdjacencyListIterator {
        AdjacencyListIterator {
            graph: self,
            next_edge: self.vertex_to_edge[u],
        }
    }
}


fn main() {

    //TODO: Turn into a test
    let edges: Vec<(usize, usize)> = vec![(0, 1), (0, 2), (1, 3), (4, 8), (5, 6), (5, 7)];
    let mut graph = Graph::new(10, 15);
    for i in 0..edges.len() {
        if i % 2 == 0 {
            graph. add_directed_edge(edges[i]);
        } else {
            graph.add_undirected_edge(edges[i]);
        }
    }

    let adjacency = graph.get_adjacency_list(0).collect::<Vec<_>>();
    assert_eq!(adjacency, vec![(1, 2), (0, 1)]);
    
    for (edge, vertex) in adjacency {
        assert_eq!(vertex, graph.edge_to_vertex[edge])
    }

    println!("passing!")



}


