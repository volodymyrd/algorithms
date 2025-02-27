use std::collections::{BTreeMap};

#[derive(PartialEq, Debug)]
struct V {
    label: char,
}

fn bfs(edges: &[(char, char)]) -> Vec<V> {
    // let mut vertices: HashMap<char, V> = HashMap::new();
    // let mut adj = HashMap::<char, Vec<&V>>::new();
    // for &(src, dest) in edges {
    //     let ref_src = vertices.entry(src).or_insert_with(|| V { label: src });
    //     let ref_dest = vertices.entry(dest).or_insert_with(|| V { label: dest });
    //     adj.entry(src).or_default().push(ref_dest);
    //     adj.entry(src).or_default().push(ref_src);
    // }
    edges
        .iter()
        .fold(BTreeMap::new(), |mut vertices, &(src, dest)| {
            vertices.entry(src).or_insert_with(|| V { label: src });
            vertices.entry(dest).or_insert_with(|| V { label: dest });
            vertices
        })
        .into_values()
        .collect()

    //vertices.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
        let vertices = bfs(&vec![
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
        assert_eq!(vertices.len(), 9);
        assert_eq!(vertices[0], V { label: 'r' });
    }
}
