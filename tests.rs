#[cfg(test)]
use super::*;
mod tests {
    
    use std::collections::HashMap;

    use crate::{citation_graph::{CitationEdge, CitationGraph}, analysis::{six_degrees_of_separation, find_most_cited_papers}, paper::Paper};

    #[test]
    fn test_six_degrees_of_separation() {
        // Set up a citation graph with 5 papers connected in a chain
        let edges = vec![
            CitationEdge { parent_id: 1, child_id: 2 },
            CitationEdge { parent_id: 2, child_id: 3 },
            CitationEdge { parent_id: 3, child_id: 4 },
            CitationEdge { parent_id: 4, child_id: 5 },
        ];
        let citation_graph = CitationGraph::from_edges(edges);

        // Set up a hashmap of papers for testing purposes
        let mut papers = HashMap::new();
        for i in 1..=5 {
            let paper = Paper {
                id: i,
            };
            papers.insert(i, paper);
        }

        // Test that the function returns the correct path for two papers in the chain
        let path = six_degrees_of_separation(1, 5, &citation_graph, &papers).unwrap();
        assert_eq!(path, vec![1, 2, 3, 4, 5]);

        // Test that the function returns None when there is no path between two papers
        let path = six_degrees_of_separation(1, 10, &citation_graph, &papers);
        assert_eq!(path, None);
    }

    #[test]
    fn test_find_most_cited_papers() {
        // Set up a citation graph with 5 papers and varying numbers of citations
        let edges = vec![
            CitationEdge { parent_id: 1, child_id: 2 },
            CitationEdge { parent_id: 1, child_id: 3 },
            CitationEdge { parent_id: 2, child_id: 4 },
            CitationEdge { parent_id: 3, child_id: 4 },
            CitationEdge { parent_id: 4, child_id: 5 },
            CitationEdge { parent_id: 4, child_id: 5 },
            CitationEdge { parent_id: 4, child_id: 5 },
        ];
        let citation_graph = CitationGraph::from_edges(edges);

        // Test that the function returns the correct top 5 most cited papers
        let most_cited_papers = find_most_cited_papers(&citation_graph);
        assert_eq!(most_cited_papers, vec![5, 4, 2, 3, 1]);
    }
}
