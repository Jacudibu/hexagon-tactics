use bevy::utils::HashMap;
use hexx::Hex;
use std::collections::BinaryHeap;

struct Node {
    coord: Hex,
    cost: u8,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Node {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        rhs.cost.cmp(&self.cost)
    }
}

/// Computes a field of movement around `start` given a `budget`, returning the `Vec<Hex>` of reachable tiles.
/// The `cost` function indicates the costs for traversing from one coordinate to its neighbor,
/// and should return `None` if the edge to neighbor or the neighbor itself isn't traversable.
///
/// This here is based off of `hexx::algorithms::pathfinding`.
/// `hexx::algorithms::field_of_movement` doesn't allow testing if it's possible to traverse
/// individual borders between hexagons (e.g. if there are walls between tiles, but the tiles
/// themselves would be traversable when coming from a different direction).
pub fn field_of_movement_with_edge_detection(
    start: Hex,
    budget: u8,
    cost: impl Fn(Hex, Hex) -> Option<u8>,
) -> Vec<Hex> {
    let start_node = Node {
        coord: start,
        cost: 0,
    };

    let mut open = BinaryHeap::new();
    open.push(start_node);
    let mut costs = HashMap::new();
    costs.insert(start, 0);

    while let Some(node) = open.pop() {
        let current_cost = costs[&node.coord];
        for neighbor in node.coord.all_neighbors() {
            let Some(cost) = cost(node.coord, neighbor) else {
                continue;
            };

            let neighbor_cost = current_cost + cost;
            if neighbor_cost <= budget
                && (!costs.contains_key(&neighbor) || costs[&neighbor] > neighbor_cost)
            {
                costs.insert(neighbor, neighbor_cost);
                open.push(Node {
                    coord: neighbor,
                    cost: neighbor_cost,
                })
            }
        }
    }

    costs.remove(&start);
    costs.into_keys().collect()
}
