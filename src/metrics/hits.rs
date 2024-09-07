use crate::Relevance;
use crate::RelevanceMap;

/// Computes the number of hits at a given relevance level.
///
/// # Arguments
///
/// * `rels` - Map of relevance levels for each document.
/// * `preds` - Sorted slice of predicted documents with their scores.
/// * `k` - Number of documents to consider.
/// * `rel_lvl` - Relevance level to consider.
pub fn compute_hits(
    rels: &RelevanceMap<i32>,
    preds: &[Relevance<f64>],
    k: usize,
    rel_lvl: i32,
) -> f64 {
    let mut hits = 0;
    for pred in &preds[..k] {
        if let Some(&rel) = rels.get(&pred.id) {
            if rel >= rel_lvl {
                hits += 1;
            }
        }
    }
    hits as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rstest::*;

    #[rstest]
    #[case(0, 0, 0.0)]
    fn test_compute_hits(#[case] k: usize, #[case] rel_lvl: i32, #[case] expected: f64) {
        let rels = {
            let mut rels = RelevanceMap::new();
            rels.insert("doc1".to_string(), 1);
            rels.insert("doc2".to_string(), 2);
            rels.insert("doc3".to_string(), 0);
            rels.insert("doc4".to_string(), 1);
            rels
        };
        let preds = vec![
            Relevance {
                id: "doc1".to_string(),
                score: 0.5,
            },
            Relevance {
                id: "doc2".to_string(),
                score: 0.4,
            },
            Relevance {
                id: "doc3".to_string(),
                score: 0.3,
            },
            Relevance {
                id: "doc4".to_string(),
                score: 0.2,
            },
            Relevance {
                id: "doc5".to_string(),
                score: 0.1,
            },
        ];
        assert_relative_eq!(compute_hits(&rels, &preds, k, rel_lvl), expected);
    }
}
