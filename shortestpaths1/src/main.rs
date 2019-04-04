use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self};
use std::usize;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as an `usize`, for a shorter implementation.
struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

fn run_test_case(num_nodes: usize, num_edges: usize, num_queries: usize, source_idx: usize) {
    let mut adj_list: Vec<Vec<Edge>> = vec![];
    for _ in 0..num_nodes {
        adj_list.push(vec![]);
    }
    let mut input = String::new();
    for _ in 0..num_edges {
        if let Ok(_) = io::stdin().read_line(&mut input) {
            let edge_data: Vec<usize> = input
                .trim()
                .split(' ')
                .map(|x| x.parse().unwrap())
                .collect();
            adj_list[edge_data[0]].push(Edge {
                node: edge_data[1],
                cost: edge_data[2],
            });
        }
        input.clear();
    }
    for _ in 0..num_queries {
        if let Ok(_) = io::stdin().read_line(&mut input) {
            let target: usize = input.trim().parse().unwrap();
            match shortest_path(&adj_list, source_idx, target) {
                Some(v) => println!("{}", v),
                None => println!("Impossible"),
            }
        }
        input.clear();
    }
}

fn main() {
    let mut input = String::new();
    let mut first = true;
    loop {
        if let Ok(_) = io::stdin().read_line(&mut input) {
            let configs: Vec<usize> = input
                .trim()
                .split(' ')
                .map(|x| x.parse().unwrap())
                .collect();

            if configs == vec![0, 0, 0, 0] {
                return;
            } else {
                if !first {
                    println!();
                } else {
                    first = false;
                }
                run_test_case(configs[0], configs[1], configs[2], configs[3]);
            }
        }
        input.clear();
    }
}
