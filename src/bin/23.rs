use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines().filter(|l| !l.is_empty()) {
        let (a, b) = line.split_once("-").unwrap();
        connections.entry(a).or_default().push(b);
        connections.entry(b).or_default().push(a);
    }
    connections
}

fn solve(input: &str) -> Option<usize> {
    let connections = parse(input);

    // 1. Adjacency Set for O(1) lookups
    let adj_set: HashMap<&str, HashSet<&str>> = connections
        .into_iter()
        .map(|(node, neighbors)| (node, neighbors.into_iter().collect()))
        .collect();

    let mut count = 0;

    // 2. Iterate through edges
    for (&u, neighbors_u) in &adj_set {
        for &v in neighbors_u {
            if u < v {
                // Avoid duplicate counting
                // 3. Find common neighbors (intersection of adjacency sets)
                for &w in neighbors_u {
                    if v < w
                        && adj_set
                            .get(v)
                            .map_or(false, |neighbors_v| neighbors_v.contains(w))
                    {
                        // Triangle found (u, v, w)
                        if u.starts_with('t') || v.starts_with('t') || w.starts_with('t') {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    Some(count)
}

fn max_clique<'a>(edges: &HashMap<&'a str, Vec<&'a str>>) -> Option<Vec<&'a str>> {
    // 1. Build the graph and calculate vertex degrees
    let mut graph: HashMap<&'a str, HashSet<&'a str>> = HashMap::new();
    let mut degrees: HashMap<&'a str, usize> = HashMap::new();
    for (&u, neighbors) in edges {
        for &v in neighbors {
            graph.entry(u).or_default().insert(v);
            graph.entry(v).or_default().insert(u);
            *degrees.entry(u).or_default() += 1;
            *degrees.entry(v).or_default() += 1;
        }
    }

    // 2. Order vertices by degree (descending)
    let mut vertices: Vec<&'a str> = graph.keys().cloned().collect();
    vertices.sort_by_key(|v| std::cmp::Reverse(degrees.get(v).unwrap_or(&0)));

    // 3. Bron-Kerbosch with Pivot (recursive function)
    fn bron_kerbosch<'a>(
        graph: &HashMap<&'a str, HashSet<&'a str>>,
        r: &mut HashSet<&'a str>,
        mut p: HashSet<&'a str>,
        mut x: HashSet<&'a str>,
        max_clique: &mut HashSet<&'a str>,
    ) {
        if p.is_empty() && x.is_empty() {
            if r.len() > max_clique.len() {
                *max_clique = r.clone();
            }
            return;
        }

        // Pivot selection (vertex in P with most neighbors in P)
        let pivot = p
            .iter()
            .max_by_key(|&u| {
                p.iter()
                    .filter(|&v| {
                        graph
                            .get(u)
                            .map_or(false, |neighbors| neighbors.contains(v))
                    })
                    .count()
            })
            .cloned()
            .unwrap_or("");

        let p_without_pivot_neighbors: HashSet<_> = p
            .iter()
            .filter(|&v| {
                !graph
                    .get(pivot)
                    .map_or(false, |neighbors| neighbors.contains(v))
            })
            .cloned()
            .collect();

        for &v in &p_without_pivot_neighbors {
            let neighbors = graph.get(v).cloned().unwrap_or_default();
            r.insert(v);
            let p_intersect_neighbors: HashSet<_> = p
                .iter()
                .filter(|&u| neighbors.contains(u))
                .cloned()
                .collect();
            let x_intersect_neighbors: HashSet<_> = x
                .iter()
                .filter(|&u| neighbors.contains(u))
                .cloned()
                .collect();

            bron_kerbosch(
                graph,
                r,
                p_intersect_neighbors,
                x_intersect_neighbors,
                max_clique,
            );

            r.remove(&v);
            p.remove(&v);
            x.insert(v);
        }
    }

    // 4. Initialize and call Bron-Kerbosch
    let mut max_clique_set: HashSet<&'a str> = HashSet::new();
    let mut r: HashSet<&'a str> = HashSet::new();
    let p: HashSet<&'a str> = vertices.iter().cloned().collect();
    let x: HashSet<&'a str> = HashSet::new();

    bron_kerbosch(&graph, &mut r, p, x, &mut max_clique_set);

    // 5. Convert to sorted vector and return
    if max_clique_set.is_empty() {
        None
    } else {
        let mut max_clique_vec: Vec<&'a str> = max_clique_set.into_iter().collect();
        max_clique_vec.sort();
        Some(max_clique_vec)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let result = solve(input);
    result
}

pub fn part_two(input: &str) -> Option<String> {
    if let Some(mut clique) = max_clique(&parse(input)) {
        clique.sort();
        let result = clique.join(",");
        Some(result)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
