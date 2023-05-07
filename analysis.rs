use crate::CitationGraph;
use crate::Paper;
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Reverse;


pub fn six_degrees_of_separation(
    start_paper_id: u32,
    end_paper_id: u32,
    citation_graph: &CitationGraph,
    _papers: &HashMap<u32, Paper>,
) -> Option<Vec<u32>> {
    // Create a mapping of paper IDs to their adjacent papers
    let mut adjacency_map = HashMap::new();
    for edge in &citation_graph.edges {
        adjacency_map
            .entry(edge.parent_id)
            .or_insert_with(HashSet::new)
            .insert(edge.child_id);
    }

    // Perform a breadth-first search to find the shortest path
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut prev_paper = HashMap::new();

    visited.insert(start_paper_id);
    queue.push_back(start_paper_id);

    while let Some(paper_id) = queue.pop_front() {
        if paper_id == end_paper_id {
            // Found the end paper, construct and return the path
            let mut path = Vec::new();
            let mut curr_paper_id = end_paper_id;
            while let Some(&prev_id) = prev_paper.get(&curr_paper_id) {
                path.push(curr_paper_id);
                curr_paper_id = prev_id;
            }
            path.push(start_paper_id);
            path.reverse();
            return Some(path);
        }

        if let Some(adjacent_papers) = adjacency_map.get(&paper_id) {
            for &adjacent_paper_id in adjacent_papers {
                if !visited.contains(&adjacent_paper_id) {
                    visited.insert(adjacent_paper_id);
                    prev_paper.insert(adjacent_paper_id, paper_id);
                    queue.push_back(adjacent_paper_id);
                }
            }
        }
    }

    // Did not find a path
   None
}

pub fn find_most_cited_papers(citation_graph: &CitationGraph) -> Vec<u32> {
    let mut paper_counts = HashMap::new();

    // Count the number of citations for each paper
    for edge in &citation_graph.edges {
        *paper_counts.entry(edge.child_id).or_insert(0) += 1;
    }

    // Sort the paper IDs by the number of citations, using a stable sorting algorithm
    let mut sorted_papers: Vec<_> = paper_counts.into_iter().collect();
    sorted_papers.sort_by_cached_key(|&(_, count)| Reverse(count));

    // Take the top 5 papers with the most citations
    let most_cited_papers: Vec<u32> = sorted_papers
        .into_iter()
        .take(5)
        .map(|(paper_id, _)| paper_id)
        .collect();

    most_cited_papers
}
