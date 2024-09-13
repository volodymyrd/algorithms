#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CryptoHash(pub [u8; 4]);

impl CryptoHash {
    pub fn new() -> Self {
        Self([0, 0, 0, 0])
    }

    pub fn hash(value: i32) -> Self {
        Self(value.to_le_bytes())
    }

    pub fn combine_hash(hash1: &CryptoHash, hash2: &CryptoHash) -> Self {
        Self(
            (hash1.0.iter().map(|e| *e as i32).sum::<i32>()
                + hash2.0.iter().map(|e| *e as i32).sum::<i32>())
            .to_le_bytes(),
        )
    }
}

impl Default for CryptoHash {
    fn default() -> Self {
        CryptoHash::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MerklePathItem {
    pub hash: CryptoHash,
    pub direction: Direction,
}

pub type MerklePath = Vec<MerklePathItem>;

pub fn merklize(arr: &[i32]) -> (CryptoHash, Vec<MerklePath>) {
    if arr.is_empty() {
        return (CryptoHash::default(), vec![]);
    }
    let mut len = arr.len().next_power_of_two();
    let mut hashes = arr
        .iter()
        .map(|value: &i32| CryptoHash::hash(*value))
        .collect::<Vec<_>>();

    // degenerate case
    if len == 1 {
        return (hashes[0], vec![vec![]]);
    }
    let mut arr_len = arr.len();
    let mut paths: Vec<MerklePath> = (0..arr_len)
        .map(|i| {
            if i % 2 == 0 {
                if i + 1 < arr_len {
                    vec![MerklePathItem {
                        hash: hashes[i + 1],
                        direction: Direction::Right,
                    }]
                } else {
                    vec![]
                }
            } else {
                vec![MerklePathItem {
                    hash: hashes[i - 1],
                    direction: Direction::Left,
                }]
            }
        })
        .collect();

    let mut counter = 1;
    while len > 1 {
        len /= 2;
        counter *= 2;
        for i in 0..len {
            let hash = if 2 * i >= arr_len {
                continue;
            } else if 2 * i + 1 >= arr_len {
                hashes[2 * i]
            } else {
                CryptoHash::combine_hash(&hashes[2 * i], &hashes[2 * i + 1])
            };
            hashes[i] = hash;
            if len > 1 {
                if i % 2 == 0 {
                    for j in 0..counter {
                        let index = (i + 1) * counter + j;
                        if index < arr.len() {
                            paths[index].push(MerklePathItem {
                                hash,
                                direction: Direction::Left,
                            });
                        }
                    }
                } else {
                    for j in 0..counter {
                        let index = (i - 1) * counter + j;
                        if index < arr.len() {
                            paths[index].push(MerklePathItem {
                                hash,
                                direction: Direction::Right,
                            });
                        }
                    }
                }
            }
        }
        arr_len = (arr_len + 1) / 2;
    }
    (hashes[0], paths)
}

pub fn verify_path(root: CryptoHash, path: &MerklePath, item: i32) -> bool {
    verify_hash(root, path, CryptoHash::hash(item))
}

pub fn verify_hash(root: CryptoHash, path: &MerklePath, item_hash: CryptoHash) -> bool {
    compute_root_from_path(path, item_hash) == root
}

pub fn compute_root_from_path(path: &MerklePath, item_hash: CryptoHash) -> CryptoHash {
    let mut res = item_hash;
    for item in path {
        match item.direction {
            Direction::Left => {
                res = CryptoHash::combine_hash(&item.hash, &res);
            }
            Direction::Right => {
                res = CryptoHash::combine_hash(&res, &item.hash);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(CryptoHash::hash(12), CryptoHash::hash(12));
    }

    #[test]
    fn test_hash_1() {
        assert_eq!(CryptoHash::hash(1), CryptoHash([1, 0, 0, 0]));
    }

    #[test]
    fn test_hash_2() {
        assert_eq!(CryptoHash::hash(2), CryptoHash([2, 0, 0, 0]));
    }

    #[test]
    fn test_hash_3() {
        assert_eq!(CryptoHash::hash(3), CryptoHash([3, 0, 0, 0]));
    }

    #[test]
    fn test_hash_4() {
        assert_eq!(CryptoHash::hash(4), CryptoHash([4, 0, 0, 0]));
    }

    #[test]
    fn test_build_merkle_path_with_empty_arr() {
        assert_eq!(merklize(&vec![]), (CryptoHash::default(), vec![]))
    }

    #[test]
    fn test_build_merkle_path_with_one_element_arr() {
        assert_eq!(merklize(&vec![1]), (CryptoHash([1, 0, 0, 0]), vec![vec![]]))
    }

    #[test]
    fn test_build_merkle_path_with_two_elements_arr() {
        assert_eq!(
            merklize(&vec![1, 2]),
            (
                CryptoHash([3, 0, 0, 0]),
                vec![
                    vec![MerklePathItem {
                        hash: CryptoHash([2, 0, 0, 0]),
                        direction: Direction::Right
                    }],
                    vec![MerklePathItem {
                        hash: CryptoHash([1, 0, 0, 0]),
                        direction: Direction::Left
                    }]
                ]
            )
        )
    }

    #[test]
    fn test_build_merkle_path_with_odd_arr_len() {
        assert_eq!(
            merklize(&vec![5, 15, 25]),
            (
                CryptoHash([45, 0, 0, 0]),
                vec![
                    vec![
                        MerklePathItem {
                            hash: CryptoHash([15, 0, 0, 0]),
                            direction: Direction::Right
                        },
                        MerklePathItem {
                            hash: CryptoHash([25, 0, 0, 0]),
                            direction: Direction::Right
                        }
                    ],
                    vec![
                        MerklePathItem {
                            hash: CryptoHash([5, 0, 0, 0]),
                            direction: Direction::Left
                        },
                        MerklePathItem {
                            hash: CryptoHash([25, 0, 0, 0]),
                            direction: Direction::Right
                        }
                    ],
                    vec![MerklePathItem {
                        hash: CryptoHash([20, 0, 0, 0]),
                        direction: Direction::Left
                    },]
                ]
            )
        )
    }

    #[test]
    fn test_verify_path() {
        assert!(verify_path(
            CryptoHash([45, 0, 0, 0]),
            &vec![
                MerklePathItem {
                    hash: CryptoHash([5, 0, 0, 0]),
                    direction: Direction::Left
                },
                MerklePathItem {
                    hash: CryptoHash([25, 0, 0, 0]),
                    direction: Direction::Right
                }
            ],
            15
        ));
    }
    #[test]
    fn test_build_merkle_path_with_even_arr_len() {
        assert_eq!(
            merklize(&vec![1, 2, 3, 4]),
            (
                CryptoHash([10, 0, 0, 0]),
                vec![
                    vec![
                        MerklePathItem {
                            hash: CryptoHash([2, 0, 0, 0]),
                            direction: Direction::Right
                        },
                        MerklePathItem {
                            hash: CryptoHash([7, 0, 0, 0]),
                            direction: Direction::Right
                        }
                    ],
                    vec![
                        MerklePathItem {
                            hash: CryptoHash([1, 0, 0, 0]),
                            direction: Direction::Left
                        },
                        MerklePathItem {
                            hash: CryptoHash([7, 0, 0, 0]),
                            direction: Direction::Right
                        }
                    ],
                    vec![
                        MerklePathItem {
                            hash: CryptoHash([4, 0, 0, 0]),
                            direction: Direction::Right
                        },
                        MerklePathItem {
                            hash: CryptoHash([3, 0, 0, 0]),
                            direction: Direction::Left
                        }
                    ],
                    vec![
                        MerklePathItem {
                            hash: CryptoHash([3, 0, 0, 0]),
                            direction: Direction::Left
                        },
                        MerklePathItem {
                            hash: CryptoHash([3, 0, 0, 0]),
                            direction: Direction::Left
                        }
                    ]
                ]
            )
        )
    }
}
