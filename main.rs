mod analysis;
mod citation_graph;
mod paper;
use std::env;
use std::process;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use analysis::{find_most_cited_papers, six_degrees_of_separation};
use citation_graph::CitationGraph;
use paper::{Paper, read_papers};


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <papers-file>", args[0]);
        process::exit(1);
    }

    let papers = read_papers(&args[1]);
    let graph = CitationGraph::from_papers(&papers);
    
    let most_cited_papers = find_most_cited_papers(&graph);
    println!("Top 5 most cited papers:");
    for paper in &most_cited_papers {
        println!("{}", paper);
    }
    let top_cited_papers = find_most_cited_papers(&graph);
let random_paper = papers.values().choose(&mut thread_rng()).unwrap();


println!("Random paper: {}", random_paper.id);

for paper_id in top_cited_papers {
    match six_degrees_of_separation(paper_id, random_paper.id, &graph, &papers) {
        Some(path) => {
            println!("Path from paper {} to paper {}:", paper_id, random_paper.id);
            for paper_id in path.iter().rev() {
                let paper = papers.get(paper_id).unwrap();
                println!("{}", paper.id);
            }
        }
        None => {
            println!("No path found from paper {} to paper {}.", paper_id, random_paper.id);
        }
    } 
}
}


