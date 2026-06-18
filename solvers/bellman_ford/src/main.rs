use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Edge {
    u: usize,
    v: usize,
    w: i64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: solve <input_file>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    let mut num_nodes = 0;
    let mut edges = Vec::new();
    let mut line = String::new();

    while reader.read_line(&mut line).unwrap() > 0 {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('c') {
            line.clear();
            continue;
        }

        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if parts.is_empty() {
            line.clear();
            continue;
        }

        if parts[0] == "p" {
            num_nodes = parts[2].parse::<usize>().unwrap();
        } else if parts[0] == "a" {
            let u: usize = parts[1].parse().unwrap();
            let v: usize = parts[2].parse().unwrap();
            let w: i64 = parts[3].parse().unwrap();
            edges.push(Edge { u, v, w });
        }
        line.clear();
    }

    if num_nodes == 0 {
        eprintln!("Error: number of nodes not found");
        std::process::exit(1);
    }

    let mut dist = vec![i64::MAX; num_nodes + 1];
    dist[1] = 0;

    for _ in 0..num_nodes - 1 {
        let mut updated = false;
        for edge in &edges {
            if dist[edge.u] != i64::MAX {
                let new_dist = dist[edge.u] + edge.w;
                if new_dist < dist[edge.v] {
                    dist[edge.v] = new_dist;
                    updated = true;
                }
            }
        }
        if !updated {
            break;
        }
    }

    let ans = dist[num_nodes];
    if ans == i64::MAX {
        println!("Unreachable");
    } else {
        println!("{}", ans);
    }
}
