use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BinaryHeap;
use std::cmp::{Ordering, Reverse};
use rayon::prelude::*;
use std::cell::RefCell;

thread_local! {
    static TLS_CTX: RefCell<Option<SearchContext>> = RefCell::new(None);
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
struct Edge {
    target: u32,
    weight: u32,
}

struct Graph {
    forward: Vec<Vec<Edge>>,
    backward: Vec<Vec<Edge>>,
}

struct SearchContext {
    dist: Vec<u32>,
    gen: Vec<u32>,
    current_gen: u32,
    pq: BinaryHeap<State>,
}

impl SearchContext {
    fn new(num_nodes: usize) -> Self {
        Self {
            dist: vec![u32::MAX; num_nodes + 1],
            gen: vec![0; num_nodes + 1],
            current_gen: 0,
            pq: BinaryHeap::new(),
        }
    }

    fn reset(&mut self) {
        self.current_gen += 1;
        if self.current_gen == 0 {
            self.gen.fill(0);
            self.current_gen = 1;
        }
        self.pq.clear();
    }

    fn set_dist(&mut self, node: u32, d: u32) {
        self.gen[node as usize] = self.current_gen;
        self.dist[node as usize] = d;
    }

    fn get_dist(&self, node: u32) -> u32 {
        if self.gen[node as usize] == self.current_gen {
            self.dist[node as usize]
        } else {
            u32::MAX
        }
    }
}

fn add_edge(adj: &mut Vec<Vec<Edge>>, u: u32, v: u32, weight: u32) {
    for edge in &mut adj[u as usize] {
        if edge.target == v {
            if weight < edge.weight {
                edge.weight = weight;
            }
            return;
        }
    }
    adj[u as usize].push(Edge { target: v, weight });
}

struct WitnessLimits {
    max_settled: usize,
}

fn exact_edge_difference(
    v: u32,
    graph: &Graph,
    contracted: &[bool],
    limits: &WitnessLimits,
    ctx: &mut SearchContext,
) -> (i32, Vec<(u32, u32, u32)>) {
    let in_edges = &graph.backward[v as usize];
    let out_edges = &graph.forward[v as usize];

    let mut uncontracted_in = Vec::new();
    for e in in_edges {
        if !contracted[e.target as usize] {
            uncontracted_in.push(e.clone());
        }
    }

    let mut uncontracted_out = Vec::new();
    for e in out_edges {
        if !contracted[e.target as usize] {
            uncontracted_out.push(e.clone());
        }
    }

    let mut shortcuts = Vec::new();

    for in_e in &uncontracted_in {
        let u = in_e.target;
        let dist_uv = in_e.weight;

        let mut max_cost = 0;
        for out_e in &uncontracted_out {
            if dist_uv + out_e.weight > max_cost {
                max_cost = dist_uv + out_e.weight;
            }
        }

        if max_cost == 0 {
            continue;
        }

        ctx.reset();
        ctx.set_dist(u, 0);
        ctx.pq.push(State { cost: 0, position: u });

        let mut settled = 0;

        while let Some(State { cost, position }) = ctx.pq.pop() {
            if cost > max_cost || settled >= limits.max_settled {
                break;
            }
            if cost > ctx.get_dist(position) {
                continue;
            }
            settled += 1;

            for e in &graph.forward[position as usize] {
                let w = e.target;
                if contracted[w as usize] || w == v {
                    continue;
                }
                let next_cost = cost + e.weight;
                if next_cost < ctx.get_dist(w) && next_cost <= max_cost {
                    ctx.set_dist(w, next_cost);
                    ctx.pq.push(State { cost: next_cost, position: w });
                }
            }
        }

        for out_e in &uncontracted_out {
            let w = out_e.target;
            let p_cost = dist_uv + out_e.weight;
            if ctx.get_dist(w) > p_cost {
                shortcuts.push((u, w, p_cost));
            }
        }
    }

    let ed = shortcuts.len() as i32 - (uncontracted_in.len() as i32 + uncontracted_out.len() as i32);
    (ed, shortcuts)
}

fn query(
    s: u32,
    t: u32,
    graph: &Graph,
    node_order: &[u32],
    contracted: &[bool],
    num_nodes: u32,
) -> Option<u32> {
    if s == t {
        return Some(0);
    }

    let mut dist_f = vec![u32::MAX; (num_nodes + 1) as usize];
    let mut dist_b = vec![u32::MAX; (num_nodes + 1) as usize];

    let mut pq_f = BinaryHeap::new();
    let mut pq_b = BinaryHeap::new();

    let mut best_dist = u32::MAX;

    dist_f[s as usize] = 0;
    pq_f.push(State { cost: 0, position: s });

    dist_b[t as usize] = 0;
    pq_b.push(State { cost: 0, position: t });

    let mut visited_f = vec![false; (num_nodes + 1) as usize];
    let mut visited_b = vec![false; (num_nodes + 1) as usize];

    while !pq_f.is_empty() || !pq_b.is_empty() {
        let min_f = pq_f.peek().map(|s| s.cost).unwrap_or(u32::MAX);
        let min_b = pq_b.peek().map(|s| s.cost).unwrap_or(u32::MAX);

        if min_f >= best_dist && min_b >= best_dist {
            break;
        }

        if min_f <= min_b {
            let State { cost, position } = pq_f.pop().unwrap();
            if visited_f[position as usize] {
                continue;
            }
            visited_f[position as usize] = true;

            if cost > best_dist {
                continue;
            }

            if dist_b[position as usize] != u32::MAX {
                best_dist = best_dist.min(cost + dist_b[position as usize]);
            }

            for e in &graph.forward[position as usize] {
                let w = e.target;
                let valid = if contracted[position as usize] {
                    !contracted[w as usize] || node_order[w as usize] > node_order[position as usize]
                } else {
                    !contracted[w as usize]
                };
                if valid {
                    let next_cost = cost + e.weight;
                    if next_cost < dist_f[w as usize] {
                        dist_f[w as usize] = next_cost;
                        pq_f.push(State { cost: next_cost, position: w });
                    }
                }
            }
        } else {
            let State { cost, position } = pq_b.pop().unwrap();
            if visited_b[position as usize] {
                continue;
            }
            visited_b[position as usize] = true;

            if cost > best_dist {
                continue;
            }

            if dist_f[position as usize] != u32::MAX {
                best_dist = best_dist.min(cost + dist_f[position as usize]);
            }

            for e in &graph.backward[position as usize] {
                let w = e.target;
                let valid = if contracted[position as usize] {
                    !contracted[w as usize] || node_order[w as usize] > node_order[position as usize]
                } else {
                    !contracted[w as usize]
                };
                if valid {
                    let next_cost = cost + e.weight;
                    if next_cost < dist_b[w as usize] {
                        dist_b[w as usize] = next_cost;
                        pq_b.push(State { cost: next_cost, position: w });
                    }
                }
            }
        }
    }

    if best_dist == u32::MAX {
        None
    } else {
        Some(best_dist)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: solve <input_file> [num_threads]");
        std::process::exit(1);
    }

    if args.len() > 2 {
        if let Ok(threads) = args[2].parse::<usize>() {
            let _ = rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build_global();
        }
    }

    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut num_nodes = 0;
    let mut _num_edges = 0;

    let mut forward = Vec::new();
    let mut backward = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() || line.starts_with('c') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        if parts[0] == "p" {
            num_nodes = parts[2].parse::<u32>().unwrap();
            _num_edges = parts[3].parse::<u32>().unwrap();
            forward = vec![Vec::new(); (num_nodes + 1) as usize];
            backward = vec![Vec::new(); (num_nodes + 1) as usize];
        } else if parts[0] == "a" {
            let u: u32 = parts[1].parse().unwrap();
            let v: u32 = parts[2].parse().unwrap();
            let w: u32 = parts[3].parse().unwrap();
            forward[u as usize].push(Edge { target: v, weight: w });
            backward[v as usize].push(Edge { target: u, weight: w });
        }
    }

    if num_nodes == 0 {
        eprintln!("Error: number of nodes not found");
        std::process::exit(1);
    }

    let mut graph = Graph {
        forward,
        backward,
    };

    let mut contracted = vec![false; (num_nodes + 1) as usize];
    let limits = WitnessLimits { max_settled: 5 }; // Very low to be fast

    let initial_importances: Vec<_> = (1..=num_nodes).into_par_iter().map(|v| {
        let mut exact_ed = 0;
        TLS_CTX.with(|ctx_cell| {
            let mut opt_ctx = ctx_cell.borrow_mut();
            if opt_ctx.is_none() {
                *opt_ctx = Some(SearchContext::new(num_nodes as usize));
            }
            let ctx = opt_ctx.as_mut().unwrap();
            let (ed, _) = exact_edge_difference(v, &graph, &contracted, &limits, ctx);
            exact_ed = ed;
        });
        (exact_ed, v)
    }).collect();

    let mut pq = BinaryHeap::new();
    for (ed, v) in initial_importances {
        pq.push(Reverse((ed, v, 0))); // importance, node, eval_count
    }

    let mut node_order = vec![0; (num_nodes + 1) as usize];
    let mut order = 0;

    let mut deleted_neighbors = vec![0; (num_nodes + 1) as usize];
    // ctx is no longer needed in the main loop since we'll use TLS_CTX


    let mut _num_contracted = 0;
    
    let batch_size = 256;
    let mut in_batch = vec![false; (num_nodes + 1) as usize];
    
    loop {
        let mut batch = Vec::new();
        let mut deferred = Vec::new();

        while let Some(Reverse((imp, v, eval_count))) = pq.pop() {
            if contracted[v as usize] { continue; }
            
            let un_in = graph.backward[v as usize].iter().filter(|e| !contracted[e.target as usize]).count();
            let un_out = graph.forward[v as usize].iter().filter(|e| !contracted[e.target as usize]).count();

            // STRICT core condition: stop if the best node has degree > 5
            if un_in > 5 || un_out > 5 {
                deferred.push(Reverse((imp, v, eval_count)));
                break;
            }

            if eval_count == 0 {
                let approx_ed = (un_in * un_out) as i32 - un_in as i32 - un_out as i32;
                let approx_imp = approx_ed + deleted_neighbors[v as usize];
                
                let next_imp = pq.peek().map_or(u32::MAX as i32, |Reverse((i, _, _))| *i);
                
                if approx_imp > next_imp {
                    deferred.push(Reverse((approx_imp, v, 1)));
                    continue;
                }
            }

            // Check independence
            let mut is_independent = true;
            for e in &graph.forward[v as usize] {
                if in_batch[e.target as usize] {
                    is_independent = false;
                    break;
                }
            }
            if is_independent {
                for e in &graph.backward[v as usize] {
                    if in_batch[e.target as usize] {
                        is_independent = false;
                        break;
                    }
                }
            }

            if is_independent {
                batch.push((v, eval_count, imp));
                in_batch[v as usize] = true;
                if batch.len() >= batch_size {
                    break;
                }
            } else {
                deferred.push(Reverse((imp, v, eval_count)));
            }
        }

        // Put deferred back
        for item in deferred {
            pq.push(item);
        }

        if batch.is_empty() {
            break;
        }

        // Parallel evaluate
        let mut evaluated: Vec<_> = batch.into_par_iter().map(|(v, eval_count, _old_imp)| {
            let mut exact_ed = 0;
            let mut shortcuts = Vec::new();
            TLS_CTX.with(|ctx_cell| {
                let mut opt_ctx = ctx_cell.borrow_mut();
                if opt_ctx.is_none() {
                    *opt_ctx = Some(SearchContext::new(num_nodes as usize));
                }
                let ctx = opt_ctx.as_mut().unwrap();
                let res = exact_edge_difference(v, &graph, &contracted, &limits, ctx);
                exact_ed = res.0;
                shortcuts = res.1;
            });
            let exact_imp = exact_ed + deleted_neighbors[v as usize];
            (v, eval_count, exact_imp, shortcuts)
        }).collect();

        // Sort by exact_imp to contract the best ones first
        evaluated.sort_by_key(|&(_, _, exact_imp, ref _shortcuts)| exact_imp);

        for (v, eval_count, exact_imp, shortcuts) in evaluated {
            // Unmark in_batch
            in_batch[v as usize] = false;

            let next_imp = pq.peek().map_or(u32::MAX as i32, |Reverse((i, _, _))| *i);
            
            if exact_imp > next_imp && eval_count < 2 {
                pq.push(Reverse((exact_imp, v, 2)));
            } else {
                contracted[v as usize] = true;
                node_order[v as usize] = order;
                order += 1;
                _num_contracted += 1;

                for (u, w, weight) in shortcuts {
                    add_edge(&mut graph.forward, u, w, weight);
                    add_edge(&mut graph.backward, w, u, weight);
                }

                for e in &graph.forward[v as usize] {
                    if !contracted[e.target as usize] {
                        deleted_neighbors[e.target as usize] += 1;
                    }
                }
                for e in &graph.backward[v as usize] {
                    if !contracted[e.target as usize] {
                        deleted_neighbors[e.target as usize] += 1;
                    }
                }
            }
        }
    }

    let s = 1;
    let t = num_nodes;

    if let Some(dist) = query(s, t, &graph, &node_order, &contracted, num_nodes) {
        println!("{}", dist);
    } else {
        println!("Unreachable");
    }
}
