use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub struct Vertex {
    label: char,
    visited: bool,
    dist: i32,
    pi: Option<Rc<RefCell<Vertex>>>,
}

impl Vertex {
    pub fn new(label: char) -> Self {
        Self {
            label,
            visited: false,
            dist: i32::MAX,
            pi: None,
        }
    }
    pub fn with_all_params(
        label: char,
        visited: bool,
        dist: i32,
        pi: Option<Rc<RefCell<Vertex>>>,
    ) -> Self {
        Self {
            label,
            visited,
            dist,
            pi,
        }
    }
    pub fn to_string(&self) -> String {
        let pi_label = match &self.pi {
            Some(pi) => pi.borrow().label,
            None => '-',
        };
        format!(
            "Vertex{{label={}, visited={}, dist={}, pi={}}}",
            self.label, self.visited, self.dist, pi_label
        )
    }
}

pub struct Graph {
    vertices: HashMap<char, Rc<RefCell<Vertex>>>,
    adj: HashMap<char, Vec<Rc<RefCell<Vertex>>>>,
}

impl Graph {
    pub fn new(edges: Vec<(char, char)>) -> Self {
        let (vertices, adj) = edges.into_iter().fold(
            (
                HashMap::new(),
                HashMap::<char, Vec<Rc<RefCell<Vertex>>>>::new(),
            ),
            |(mut vertices, mut adj), (src, dest)| {
                {
                    let src_vertex = vertices
                        .entry(src)
                        .or_insert_with(|| Rc::new(RefCell::new(Vertex::new(src))));
                    adj.entry(dest).or_default().push(Rc::clone(src_vertex));
                }

                {
                    let dest_vertex = vertices
                        .entry(dest)
                        .or_insert_with(|| Rc::new(RefCell::new(Vertex::new(dest))));
                    adj.entry(src).or_default().push(Rc::clone(dest_vertex));
                }

                (vertices, adj)
            },
        );

        Self { vertices, adj }
    }

    pub fn bfs(&self, source: char, target: Option<char>) {
        let src_vertex = self.vertices.get(&source).expect("Source vertex not found");
        src_vertex.borrow_mut().visited = true;
        src_vertex.borrow_mut().dist = 0;

        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(src_vertex));

        while let Some(u) = queue.pop_front() {
            let u_vertex = u.borrow();

            if let Some(neighbors) = self.adj.get(&u_vertex.label) {
                for v in neighbors {
                    let mut v_vertex = v.borrow_mut();
                    if !v_vertex.visited {
                        v_vertex.visited = true;
                        v_vertex.dist = u_vertex.dist + 1;
                        v_vertex.pi = Some(Rc::clone(&u));

                        if target == Some(v_vertex.label) {
                            break;
                        }
                        queue.push_back(Rc::clone(v));
                    }
                }
            }
        }
    }

    pub fn vertices(&self) -> Vec<Rc<RefCell<Vertex>>> {
        let mut v: Vec<Rc<RefCell<Vertex>>> = self.vertices.clone().into_values().collect();
        v.sort_by(|a, b| a.borrow().label.cmp(&b.borrow().label));
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let graph = Graph::new(vec![
            ('r', 's'),
            ('r', 't'),
            ('r', 'w'),
            ('s', 'u'),
            ('s', 'v'),
            ('t', 'u'),
            ('u', 'y'),
            ('v', 'w'),
            ('v', 'y'),
            ('w', 'x'),
            ('w', 'z'),
            ('x', 'y'),
            ('x', 'z'),
        ]);

        graph.bfs('s', None);
        let vertices = graph.vertices();

        let expected = vec![
            Rc::new(RefCell::new(Vertex::with_all_params(
                'r',
                true,
                1,
                Some(Rc::new(RefCell::new(Vertex::with_all_params(
                    's', true, 0, None,
                )))),
            ))),
            Rc::new(RefCell::new(Vertex::with_all_params('s', true, 0, None))),
            Rc::new(RefCell::new(Vertex::with_all_params(
                't',
                true,
                2,
                Some(Rc::new(RefCell::new(Vertex::with_all_params(
                    'r',
                    true,
                    1,
                    Some(Rc::new(RefCell::new(Vertex::with_all_params(
                        's', true, 0, None,
                    )))),
                )))),
            ))),
            Rc::new(RefCell::new(Vertex::with_all_params(
                'u',
                true,
                1,
                Some(Rc::new(RefCell::new(Vertex::with_all_params(
                    's', true, 0, None,
                )))),
            ))),
            Rc::new(RefCell::new(Vertex::with_all_params(
                'v',
                true,
                1,
                Some(Rc::new(RefCell::new(Vertex::with_all_params(
                    's', true, 0, None,
                )))),
            ))),
            Rc::new(RefCell::new(Vertex::with_all_params(
                'w',
                true,
                2,
                Some(Rc::new(RefCell::new(Vertex::with_all_params(
                    'r',
                    true,
                    1,
                    Some(Rc::new(RefCell::new(Vertex::with_all_params(
                        's', true, 0, None,
                    )))),
                )))),
            ))),
            Rc::new(RefCell::new(Vertex::with_all_params(
                'x',
                true,
                3,
                Some(Rc::new(RefCell::new(Vertex::with_all_params(
                    'w',
                    true,
                    2,
                    Some(Rc::new(RefCell::new(Vertex::with_all_params(
                        'r',
                        true,
                        1,
                        Some(Rc::new(RefCell::new(Vertex::with_all_params(
                            's', true, 0, None,
                        )))),
                    )))),
                )))),
            ))),
            Rc::new(RefCell::new(Vertex::with_all_params(
                'y',
                true,
                2,
                Some(Rc::new(RefCell::new(Vertex::with_all_params(
                    'u',
                    true,
                    1,
                    Some(Rc::new(RefCell::new(Vertex::with_all_params(
                        's', true, 0, None,
                    )))),
                )))),
            ))),
            Rc::new(RefCell::new(Vertex::with_all_params(
                'z',
                true,
                3,
                Some(Rc::new(RefCell::new(Vertex::with_all_params(
                    'w',
                    true,
                    2,
                    Some(Rc::new(RefCell::new(Vertex::with_all_params(
                        'r',
                        true,
                        1,
                        Some(Rc::new(RefCell::new(Vertex::with_all_params(
                            's', true, 0, None,
                        )))),
                    )))),
                )))),
            ))),
        ];

        assert_eq!(vertices, expected);
    }
}
