use std::{collections::{HashMap, HashSet}, hash::Hash, str::FromStr};

use itertools::Itertools;
use uuid::Uuid;


static INPUT: &str = include_str!("../input/dayeight.txt");

#[derive(Clone, PartialEq, Eq, Hash)]
struct JunctionBox{ x:usize, y:usize, z:usize }

impl JunctionBox {
    fn euclidean_distance(box_1: JunctionBox, box_2: JunctionBox) -> f64 {
        let dx = (box_2.x as f64) - (box_1.x as f64);
        let dy = (box_2.y as f64) - (box_1.y as f64);
        let dz = (box_2.z as f64) - (box_1.z as f64);
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl FromStr for JunctionBox {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(",").collect();
        let x: usize = parts[0].parse().map_err(|_| "Failed to parse x")?;
        let y: usize = parts[1].parse().map_err(|_| "Failed to parse y")?;
        let z: usize = parts[2].parse().map_err(|_| "Failed to parse z")?;
        Ok(JunctionBox { x, y, z })
    }
}

struct Graph {
    edges: Vec<(JunctionBox, JunctionBox, f64)>
}
fn part_one(input: Vec<&str>) -> Result<usize, String> {
    let junction_boxes: Vec<JunctionBox> = input.iter().map(|i| JunctionBox::from_str(i)).collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
    let edges: Vec<(JunctionBox, JunctionBox, f64)> = junction_boxes.into_iter().combinations(2).map(|pair| {
        let box_1 = pair[0].clone();
        let box_2 = pair[1].clone();
        (box_1.clone(), box_2.clone(), JunctionBox::euclidean_distance(box_1, box_2))
    }).collect();
    let mut graph = Graph { edges };
    graph.edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    let mut circuits: HashMap<Uuid, Vec<JunctionBox>> = HashMap::new();
    let mut count: usize = 0;

    graph.edges.iter().take_while(|(box_1, box_2, _weight)| {
        if count >= 1000 {
            return false;
        }
        count += 1;
        
        if !circuits.iter().any(|(_, v)| v.contains(box_1) && v.contains(box_2)) {
            let matching_circuits: Vec<(Uuid, Vec<JunctionBox>)> = circuits.iter().filter(|(_k, v)| {
                v.contains(box_1) || v.contains(box_2)
            }).map(|(k, v)| (*k, v.clone())).collect();

            match matching_circuits.len() {
                0 => {
                    circuits.insert(Uuid::new_v4(), vec![box_1.clone(), box_2.clone()]);
                }
                1 => {
                    let (uuid, boxes) = matching_circuits.first().unwrap();
                    let mut updated_boxes = boxes.clone();
                    updated_boxes.extend(vec![box_1.clone(), box_2.clone()]);
                    circuits.insert(*uuid, updated_boxes.into_iter().unique().collect());
                }
                _ => {
                    let (uuid1, boxes1) = matching_circuits.first().unwrap();
                    let (uuid2, boxes2) = matching_circuits.last().unwrap();
                    circuits.remove(uuid1);
                    circuits.remove(uuid2);
                    let mut merged = boxes1.clone();
                    merged.extend(boxes2.clone());
                    circuits.insert(Uuid::new_v4(), merged.into_iter().unique().collect());
                }
            }
        }
        true
    }).for_each(drop);

    let circuit_product: usize = circuits.values().map(|v| v.len()).k_largest(3).product();

    Ok(circuit_product)
}

fn part_two(input: Vec<&str>) -> Result<usize, String> {
    let junction_boxes: Vec<JunctionBox> = input.iter().map(|i| JunctionBox::from_str(i)).collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
    let num_boxes = junction_boxes.len();
    let edges: Vec<(JunctionBox, JunctionBox, f64)> = junction_boxes.into_iter().combinations(2).map(|pair| {
        let box_1 = pair[0].clone();
        let box_2 = pair[1].clone();
        (box_1.clone(), box_2.clone(), JunctionBox::euclidean_distance(box_1, box_2))
    }).collect();
    let graph = Graph { edges };
    let starting_node:JunctionBox = graph.edges.first().unwrap().0.clone();
    let mut visited_nodes = HashSet::with_capacity(num_boxes);
    let mut last_added_edge = graph.edges.first().unwrap().clone();
    visited_nodes.insert(starting_node);
    
    while visited_nodes.len() < num_boxes {
        let edge_to_add = graph.edges.iter().filter(|(x, y, _)| {
            (visited_nodes.contains(x) || visited_nodes.contains(y)) && !(visited_nodes.contains(x) && visited_nodes.contains(y))
        }).min_by(|(_, _, weight1), (_, _, weight2)| weight1.partial_cmp(weight2).unwrap()).unwrap();

        let node_to_add = if visited_nodes.contains(&edge_to_add.0) {
            &edge_to_add.1
        } else {
            &edge_to_add.0
        };

        visited_nodes.insert(node_to_add.clone());
        last_added_edge = edge_to_add.clone();
        println!("Visited {} nodes", visited_nodes.len())
    }
    Ok(last_added_edge.0.x * last_added_edge.1.x)
}

fn main() {
    let result = part_one(INPUT.lines().collect::<Vec<&str>>());
    println!("Part one: {:?}", result);
    let result = part_two(INPUT.lines().collect::<Vec<&str>>());
    println!("Part two: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // Given
        let input = vec![
            "162,817,812",
            "57,618,57",           
            "906,360,560",
            "592,479,940",
            "352,342,300",
            "466,668,158",
            "542,29,236",            
            "431,825,988",
            "739,650,466",
            "52,470,668",            
            "216,146,977",
            "819,987,18",            
            "117,168,530",
            "805,96,715",            
            "346,949,466",
            "970,615,88",            
            "941,993,340",
            "862,61,35",           
            "984,92,344",            
            "425,690,689",
        ];
        
        // When
        let result = part_one(input).unwrap();
        
        // Then
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part_two() {
        // Given
        let input = vec![
            "162,817,812",
            "57,618,57",           
            "906,360,560",
            "592,479,940",
            "352,342,300",
            "466,668,158",
            "542,29,236",            
            "431,825,988",
            "739,650,466",
            "52,470,668",            
            "216,146,977",
            "819,987,18",            
            "117,168,530",
            "805,96,715",            
            "346,949,466",
            "970,615,88",            
            "941,993,340",
            "862,61,35",           
            "984,92,344",            
            "425,690,689",
        ];
        
        // When
        let result = part_two(input).unwrap();
        
        // Then
        assert_eq!(result, 25272);
    }
}