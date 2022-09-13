use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct Node<T>
where
    T: Ord,
{
    visited: bool,
    value: Option<T>,
    children: Rc<RefCell<Vec<Rc<RefCell<Node<T>>>>>>,
}

/*impl<T: Ord + Debug> std::fmt::Debug for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();
        if self.value.is_some() {
            res.push_str(&format!("\nvalue: \n{:?}", self.value.as_ref().unwrap()));
            res.push_str("\nChildren:");
            for i in self.clone().children.borrow().iter() {
                res.push_str(&format!("\n  -> {:?}", i.borrow().value.as_ref().unwrap()));
            }
        }
        write!(f, "{}", res)
    }
}*/

impl<T: std::cmp::Ord> Ord for Node<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value
            .as_ref()
            .unwrap()
            .cmp(&other.value.as_ref().unwrap())
    }
}
impl<T: PartialEq + Ord> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: std::cmp::PartialEq + Ord> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value.as_ref().unwrap() == other.value.as_ref().unwrap()
    }
}

impl<T: PartialEq + Ord> Eq for Node<T> {}

impl<T: Ord> Node<T> {
    fn new(value: Option<T>) -> Self {
        Node {
            visited: false,
            value: value,
            children: Rc::new(RefCell::new(Vec::<Rc<RefCell<Node<T>>>>::new())),
        }
    }
    fn is_empty(&self) -> bool {
        (self.children).borrow().is_empty()
    }
    fn is_visited(&self) -> bool {
        self.visited
    }
    fn visite(&mut self) {
        self.visited = true;
    }
    fn add_child(&self, value: T) {
        (self.children)
            .borrow_mut()
            .push(Rc::new(RefCell::new(Node::<T>::new(Some(value)))));
    }
}

type Edges = (i32, i32);

#[derive(Clone, Debug)]
struct Graph<T: Ord> {
    name: String,
    node: Rc<RefCell<Node<T>>>,
    aux_step_two: Vec<(T, T)>,
}

impl<T: Ord> Graph<T>
where
    T: From<i32> + std::fmt::Debug + std::cmp::PartialEq + Clone,
{
    fn new(name: String) -> Self {
        Graph {
            name: name,
            node: Rc::new(RefCell::new(Node::<T>::new(None))),
            aux_step_two: Vec::<(T, T)>::new(),
        }
    }
    fn add_vec(&mut self, mut nodes: Vec<(T, T)>) {
        // try insert the nodes into graph
        nodes.sort();
        nodes.dedup();
        for i in nodes.iter() {
            self.add(i.0.clone(), i.1.clone());
        }
        // try insert the nodes too
        loop {
            if self.aux_step_two.is_empty() {
                break;
            }
            let values = Some(self.aux_step_two.remove(0));

            match self.find(Rc::clone(&self.node), values.clone().unwrap().0) {
                Some(node) => {
                    node.as_ref()
                        .borrow()
                        .children
                        .borrow_mut()
                        .push(Rc::clone(&Rc::new(RefCell::new(Node::<T>::new(Some(
                            values.unwrap().1,
                        ))))));
                }
                None => {
                    // add first node in graph root
                    let new_node = Rc::new(RefCell::new(Node::<T>::new(Some(
                        values.clone().unwrap().0,
                    ))));
                    // add the child to node
                    new_node.borrow().add_child(values.unwrap().1);
                    // add the node to graph
                    self.node
                        .as_ref()
                        .borrow()
                        .children
                        .borrow_mut()
                        .push(Rc::clone(&new_node));
                }
            }
        }
    }
    fn add(&mut self, node_value: T, new_node_value: T) {
        // graph is empty
        if self.node.as_ref().borrow().is_empty() {
            // create a new node
            let new_node = Rc::new(RefCell::new(Node::<T>::new(Some(node_value.clone()))));
            // add the child to node
            new_node.borrow().add_child(new_node_value);
            // add the node to graph
            self.node
                .as_ref()
                .borrow()
                .children
                .borrow_mut()
                .push(Rc::clone(&new_node));
        } else {
            match self.find(Rc::clone(&self.node), node_value.clone()) {
                Some(node_origin) => {
                    // find node with value of destination node
                    match self.find(Rc::clone(&self.node), new_node_value.clone()) {
                        Some(node_destination) => {
                            node_origin
                                .as_ref()
                                .borrow()
                                .children
                                .borrow_mut()
                                .push(Rc::clone(&node_destination));
                        }
                        None => {
                            // add the node to graph
                            node_origin
                                .as_ref()
                                .borrow()
                                .children
                                .borrow_mut()
                                .push(Rc::new(RefCell::new(Node::<T>::new(Some(
                                    new_node_value.clone(),
                                )))));
                        }
                    }
                }
                None => {
                    // not found node to add child
                    self.aux_step_two.push((node_value, new_node_value));
                }
            }
        }
    }
    fn find(&self, node: Rc<RefCell<Node<T>>>, value: T) -> Option<Rc<RefCell<Node<T>>>> {
        let mut visited = Vec::<T>::new();
        visited.push(T::from(99_i32));
        self.inner_find(node, value, Some(Rc::new(RefCell::new(visited))))
    }
    fn inner_find(
        &self,
        node: Rc<RefCell<Node<T>>>,
        value: T,
        visited: Option<Rc<RefCell<Vec<T>>>>,
    ) -> Option<Rc<RefCell<Node<T>>>> {
        let v = Rc::clone(&visited.unwrap());
        if node.as_ref().borrow().value.as_ref().is_some() {
            if node.as_ref().borrow().value.as_ref().unwrap() == &value {
                return Some(Rc::clone(&node));
            }
            v.as_ref()
                .borrow_mut()
                .push(node.as_ref().borrow().value.clone().unwrap());
        }
        let children_len = node.as_ref().borrow().children.borrow().len();
        for i in 0..children_len {
            if node.as_ref().borrow().children.borrow()[i]
                .borrow()
                .value
                .is_some()
                && v.as_ref()
                    .borrow()
                    .iter()
                    .find(|&n| {
                        *n == node.as_ref().borrow().children.borrow()[i]
                            .borrow()
                            .value
                            .clone()
                            .unwrap()
                    })
                    .is_none()
            {
                match self.inner_find(
                    Rc::clone(&node.as_ref().borrow().children.borrow()[i]),
                    value.clone(),
                    Some(Rc::clone(&v)),
                ) {
                    Some(result) => {
                        return Some(result);
                    }
                    None => {}
                }
            }
        }
        None
    }
    fn find_cycle(&self, value: T, next_value: T) -> bool {
        let finded = self.find(Rc::clone(&self.node), next_value);
        if finded.is_some() {
            if finded.clone().unwrap().borrow().is_visited()
                && self.find(Rc::clone(&finded.unwrap()), value).is_some()
            {
                return true;
            }
        }
        false
    }
    fn dfs(&mut self) {
        // has nodes?
        if self.node.as_ref().borrow().is_empty() {
            return;
        }
        println!("{}", self.name);
        self.inner_dfs(Rc::clone(&self.node), None, 0);
    }

    fn inner_dfs(
        &mut self,
        node: Rc<RefCell<Node<T>>>,
        prior_node: Option<Rc<RefCell<Node<T>>>>,
        level: u8,
    ) {
        let mut tabs = String::new();
        for _ in 1..level {
            tabs.push_str("  ");
        }
        if prior_node.is_some() {
            let prior_clone = Rc::clone(&prior_node.unwrap());
            if prior_clone.as_ref().borrow().value.is_some() {
                if !node.as_ref().borrow().is_visited()
                    && node.as_ref().borrow().value != prior_clone.as_ref().borrow().value
                    && !self.find_cycle(
                        prior_clone.as_ref().borrow().value.clone().unwrap(),
                        node.as_ref().borrow().value.clone().unwrap(),
                    )
                {
                    println!(
                        "{}{:?}-{:?} pathR(G,{:?})",
                        tabs,
                        prior_clone.as_ref().borrow().value.clone().unwrap(),
                        node.as_ref().borrow().value.clone().unwrap(),
                        node.as_ref().borrow().value.clone().unwrap()
                    );
                } else {
                    println!(
                        "{}{:?}-{:?}",
                        tabs,
                        prior_clone.clone().as_ref().borrow().value.clone().unwrap(),
                        node.clone().as_ref().borrow().value.clone().unwrap()
                    );
                }
            }
        }
        node.as_ref().borrow_mut().visite();
        node.as_ref().borrow().children.borrow_mut().sort();
        let children_len = node.as_ref().borrow().children.borrow().len();
        let mut node_value: Option<T> = None;

        for i in 0..children_len {
            if !node.as_ref().borrow().children.borrow()[i]
                .borrow()
                .is_visited()
            {
                self.inner_dfs(
                    Rc::clone(&node.as_ref().borrow().children.borrow()[i]),
                    Some(Rc::clone(&node)),
                    level + 1,
                );
                if level == 0
                    && node_value != node.as_ref().borrow().children.borrow()[i].borrow().value
                {
                    println!("");
                }
                node_value = node.as_ref().borrow().children.borrow()[i]
                    .borrow()
                    .value
                    .clone();
            } else {
                println!(
                    "  {}{:?}-{:?}",
                    tabs,
                    node.clone().as_ref().borrow().value.clone().unwrap(),
                    node.as_ref().borrow().children.borrow()[i]
                        .borrow()
                        .value
                        .clone()
                        .unwrap(),
                );
            }
        }
    }
}

fn main() -> () {
    //std::env::set_var("RUST_BACKTRACE", "full");
    let mut graph = Vec::<RefCell<Graph<i32>>>::new();
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read number of inputs");
    let qtde_inputs = input.trim().parse::<i32>().unwrap_or(0);
    for i in 1..=qtde_inputs {
        let mut input_a = String::new();
        // le o numero de vertices e arestas
        std::io::stdin()
            .read_line(&mut input_a)
            .expect("failed to read number o vertices and edges");
        // vec with two positions [0] = vertices and [1] = edges
        let ve = input_a
            .trim()
            .parse::<String>()
            .unwrap_or("0".to_string())
            .split(' ')
            .map(|i| i.parse::<i32>().unwrap_or(0))
            .collect::<Vec<i32>>();
        //input.clear();
        let mut g = Graph::<i32>::new(format!("Caso {}:", i));
        let mut vetor = Vec::<Edges>::new();
        // read line edges number
        if ve.len() == 2 {
            for _i in 1..=ve[1] {
                let mut input_b = String::new();
                std::io::stdin()
                    .read_line(&mut input_b)
                    .expect("failed to read a graph edge");
                let ve = input_b.trim().parse::<String>().unwrap();
                let mut ve_iter = ve.split(' ');
                vetor.push((
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
            g.add_vec(vetor.clone());
            graph.push(RefCell::new(g));
        }
    }
    for i in graph.iter() {
        i.borrow_mut().dfs();
    }
    ()
}
