use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct Graph(pub Vec<Vec<usize>>);

impl Deref for Graph {
    type Target = Vec<Vec<usize>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Graph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Graph {
    pub fn new(len: usize) -> Self {
        let mut graph = Vec::with_capacity(len);

        for _ in 0..len {
            graph.push(Vec::new());
        }

        Graph(graph)
    }


    pub fn do_bfs(&self, start: usize, end: usize) -> Vec<usize> {
        let graph = self;

        let mut bfs_info: Vec<(Option<i32>, Option<usize>)> = Vec::with_capacity(graph.len());
        for _ in 0..graph.len() {
            bfs_info.push((None, None));
        }


        let mut current_level;
        let mut next_level = Vec::new();

        for node in &graph[start] {
            next_level.push((*node, start));
        }


        let mut visited = Vec::new();


        bfs_info[start] = (Some(0), None);


        let mut level = 1;
        let mut found = false;

        while next_level.len() > 0 && !found {
            current_level = next_level.clone();

            let mut next_neighbor = Vec::new();

            for (node, before) in current_level {
                


                match bfs_info[node] {
                    (None, _) => bfs_info[node] = (Some(level), Some(before)),
                    (Some(count), _) => if count > level {
                        bfs_info[node] = (Some(level), Some(before));
                    }
                }

                visited.push(node);

                for neighbor in &graph[node] {
                    next_neighbor.push((*neighbor, node));
                }

                if node == end {
                    found = true;
                    break;
                }

            }

            level += 1;
            drop(next_level);
            next_level = next_neighbor;
        }
        
        

        let mut res = Vec::new();
        if found {
            res.push(end);
            let mut idx = end;
            loop {
                if let (_, Some(before)) = bfs_info[idx] {
                    res.insert(0,before);
                    idx = before;
                } else {
                    break;
                }
            }
        } else {
            res.push(start);
        }

        res
    }
        
}

#[derive(Debug, Clone)]
pub struct Maze {
    graph: Graph,
    width: usize,
    height: usize,
    start: usize,
    end: usize,
}

impl Maze {
    pub fn new() -> Self {
        let width = 10;
        let height = 10;
        let start = 13;
        let end = start;
        let graph = Graph::new((width + 2) * (height + 2));

        let mut maze = Maze { graph, width, height, start, end };

        maze.init_graph();

        maze
        
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        x + (y * (self.height + 2))
    }

    pub fn get_coord(&self, idx: usize) -> (usize, usize) {
        (idx % (self.width + 2), (idx / (self.height + 2)))
    }

    fn get_neighbor(&self, idx: usize) -> Vec<usize> {
        vec![idx - (self.width + 2), idx - 1, idx + 1, idx + (self.width + 2)]
    }

    fn get_alive_neighbor(&self, idx: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        
        for neigh in vec![idx - (self.width+2), idx - 1, idx + 1, idx + (self.width + 2)] {
            if self.graph[neigh].len() > 0 {
                neighbors.push(neigh);
            }
        }

        neighbors
    }

    fn init_graph(&mut self) {
        for y in 1..(self.height + 1) {
            for x in 1..(self.width + 1) {
                let idx = self.get_index(x, y);
                let neighbors = self.get_neighbor(idx);
                for neigh in neighbors {
                    let (x, y) = self.get_coord(neigh);

                    if x != 0 && y != 0 && x <= self.width && y <= self.height {
                        self.graph[idx].push(neigh);
                    }
                }
            }
        }
    }

    fn kill(&mut self, idx: usize) {
        self.graph[idx].clear();

        for neigh in self.get_neighbor(idx) {
            if let Ok(index) = self.graph[neigh].binary_search(&idx) {
                self.graph[neigh].remove(index);
            }
        }
    }

    fn revive(&mut self, idx: usize) {
        let alive_neighbors = self.get_alive_neighbor(idx);
        
        self.graph[idx] = alive_neighbors.clone();

        for neigh in alive_neighbors {
            self.graph[neigh].push(idx);
        }
    }

    pub fn toggle(&mut self, idx: usize) {
        if self.graph[idx].len() != 0 {
            self.kill(idx);
        } else {
            self.revive(idx);
        }
    }

    pub fn get_node_relation(&self, idx: usize) -> Result<&Vec<usize>, ()> {
        let (x,y) = self.get_coord(idx);

        if x == 0 || x > self.width || y == 0 || y > self.height {
            Err(())
        } else {
            Ok(&self.graph[idx])
        }
    }

    pub fn len(&self) -> usize {
        self.graph.len()
    }

    pub fn get_bfs(&self) -> Vec<usize> {
        self.graph.do_bfs(self.start, self.end)
    }
}



// maze getter setter implementation
impl Maze {
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn set_end(&mut self, idx: usize) {
        self.end = idx;
    }

    pub fn set_start(&mut self, idx: usize) {
        self.start = idx;
    }
}