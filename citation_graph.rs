use std::collections::HashMap;
use crate::Paper;
#[derive(Debug)]
pub struct CitationEdge {
    pub parent_id: u32,
    pub child_id: u32,
}

pub struct CitationGraph {
    pub edges: Vec<CitationEdge>,
}

impl CitationGraph {
    pub fn from_edges(edges: Vec<CitationEdge>) -> CitationGraph {
        CitationGraph { edges }
    }

    pub fn from_papers(papers: &HashMap<u32, Paper>) -> CitationGraph {
        let mut edges = Vec::new();
        let paper_ids: Vec<_> = papers.keys().cloned().collect();
        for i in 0..paper_ids.len() {
            let parent_id = paper_ids[i];
            for j in (i + 1)..paper_ids.len() {
                let child_id = paper_ids[j];
                edges.push(CitationEdge { parent_id, child_id });
            }
        }
        CitationGraph::from_edges(edges)
    }
    
    }

