use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

type Edge = (i32, i32);

#[derive(Debug)]
struct Node {
    visited: bool,
}

impl Node {
    fn new() -> Self {
        Node { visited: false }
    }
    fn visite(&mut self) {
        self.visited = true;
    }
    fn is_visited(&self) -> bool {
        self.visited
    }
}

struct Graph {
    vertices: usize,
    name: String,
    matrix: Rc<RefCell<Vec<Rc<RefCell<Vec<Option<Rc<RefCell<Node>>>>>>>>>,
}

impl Graph {
    fn new(name: String, vertices: usize) -> Self {
        let mut _matrix = Rc::new(RefCell::new(Vec::<
            Rc<RefCell<Vec<Option<Rc<RefCell<Node>>>>>>,
        >::new()));
        for _ in 0..vertices {
            let line = Rc::new(RefCell::new(Vec::<Option<Rc<RefCell<Node>>>>::new()));
            for _ in 0..vertices {
                line.borrow_mut().push(None);
            }
            _matrix.borrow_mut().push(Rc::clone(&line));
        }
        Graph {
            vertices: vertices,
            name: name,
            matrix: Rc::clone(&_matrix),
        }
    }
    fn add(&mut self, edge: Edge) {
        if edge.0 >= self.vertices as i32
            || edge.0 < 0
            || edge.1 >= self.vertices as i32
            || edge.1 < 0
        {
            return;
        }
        if self.matrix.as_ref().borrow()[edge.0 as usize].borrow()[edge.1 as usize].is_none() {
            self.matrix.as_ref().borrow()[edge.0 as usize].borrow_mut()[edge.1 as usize] =
                Some(Rc::new(RefCell::new(Node::new())));
        }
    }
    fn dfs(&mut self) {
        println!("{}", self.name);
        for i in 0..self.vertices {
            if self.matrix.as_ref().borrow()[i]
                .borrow()
                .iter()
                .find(|x| x.is_some() && !x.as_ref().unwrap().borrow().is_visited())
                .is_some()
            {
                self._dfs(i, 1);
                println!("");
            }
        }
    }
    fn _dfs(&mut self, value: usize, level: usize) {
        //println!("LINE: {}", value);
        let line_len = self.matrix.borrow()[value].borrow().len();
        for column in 0..line_len {
            if self.matrix.borrow()[value].borrow()[column].is_some() {
                //let level = level * 2;
                if !self.matrix.as_ref().borrow()[value].borrow()[column]
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .is_visited()
                    && value != column
                {
                    println!(
                        "{:spaces$}{}-{} pathR(G,{})",
                        ' ',
                        value,
                        column,
                        column,
                        spaces = level * 2,
                    );
                    // visite all lines setting column value as visited
                    let matrix_len = self.matrix.borrow().len();
                    for l in 0..matrix_len {
                        if self.matrix.as_ref().borrow()[l].borrow()[column].is_some() {
                            self.matrix.as_ref().borrow()[l].borrow()[column]
                                .as_ref()
                                .unwrap()
                                .borrow_mut()
                                .visite();
                        }
                    }
                    if self.matrix.as_ref().borrow()[column].borrow()[value].is_some() {
                        self.matrix.as_ref().borrow()[column].borrow()[value]
                            .as_ref()
                            .unwrap()
                            .borrow_mut()
                            .visite();
                    }
                    self._dfs(column, level + 1);
                } else {
                    println!("{:spaces$}{}-{}", ' ', value, column, spaces = level * 2);
                }
            }
        }
    }
}

fn main() -> () {
    //std::env::set_var("RUST_BACKTRACE", "full");
    let mut graph = Vec::<RefCell<Graph>>::new();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read number of inputs");
    let qtde_inputs = input.trim().parse::<i32>().unwrap_or(0);
    for i in 1..=qtde_inputs {
        let mut input_a = String::new();
        // read the number of vertices and edges
        std::io::stdin()
            .read_line(&mut input_a)
            .expect("failed to read number o vertices and edges");
        // vec with two positions [0] = vertices and [1] = edges
        let ve = input_a
            .trim()
            .parse::<String>()
            .unwrap_or("0".to_string())
            .split(' ')
            .map(|i| i.parse::<usize>().unwrap_or(0))
            .collect::<Vec<usize>>();
        // read line edges number
        if ve.len() == 2 {
            let mut g = Graph::new(format!("Caso {}:", i), ve[0]);
            for _i in 1..=ve[1] {
                let mut input_b = String::new();
                std::io::stdin()
                    .read_line(&mut input_b)
                    .expect("failed to read a graph edge");
                let ve = input_b.trim().parse::<String>().unwrap();
                let mut ve_iter = ve.split(' ');
                g.add((
                    ve_iter
                        .next()
                        .unwrap_or(&"0".to_string())
                        .parse::<i32>()
                        .unwrap_or(0),
                    ve_iter
                        .next()
                        .unwrap_or(&"0".to_string())
                        .parse::<i32>()
                        .unwrap_or(0),
                ));
            }
            graph.push(RefCell::new(g));
        }
    }
    for i in graph.iter() {
        i.borrow_mut().dfs();
    }
    ()
}
