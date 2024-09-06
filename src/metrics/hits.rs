use crate::Predicted;
use crate::RelevanceMap;

/// Computes the number of hits at a given relevance level.
///
/// # Arguments
///
/// * `rels` - Map of relevance levels for each document.
/// * `preds` - Slice of predicted documents with their scores.
/// * `k` - Number of documents to consider.
/// * `rel_lvl` - Relevance level to consider.
pub fn compute_hits(rels: &RelevanceMap, preds: &[Predicted], k: usize, rel_lvl: usize) -> f64 {
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

    #[test]
    fn test_compute_hits() {
        let preds = vec![
            Predicted {
                id: "1".to_string(),
                score: 0.1,
            },
            Predicted {
                id: "2".to_string(),
                score: 0.2,
            },
            Predicted {
                id: "3".to_string(),
                score: 0.3,
            },
            Predicted {
                id: "4".to_string(),
                score: 0.4,
            },
            Predicted {
                id: "5".to_string(),
                score: 0.5,
            },
        ];
        let mut rels = RelevanceMap::new();
        rels.insert("1".to_string(), 0);
        rels.insert("2".to_string(), 1);
        rels.insert("3".to_string(), 2);
        rels.insert("4".to_string(), 3);
        rels.insert("5".to_string(), 4);
        assert_eq!(compute_hits(&rels, &preds, 0, 0), 0.0);
        assert_eq!(compute_hits(&rels, &preds, 1, 0), 1.0);
        assert_eq!(compute_hits(&rels, &preds, 2, 0), 1.0);
        assert_eq!(compute_hits(&rels, &preds, 3, 0), 1.0);
        assert_eq!(compute_hits(&rels, &preds, 4, 0), 1.0);
        assert_eq!(compute_hits(&rels, &preds, 5, 0), 1.0);
        assert_eq!(compute_hits(&rels, &preds, 0, 1), 0.0);
        assert_eq!(compute_hits(&rels, &preds, 1, 1), 0.0);
        assert_eq!(compute_hits(&rels, &preds, 2, 1), 1.0);
        assert_eq!(compute_hits(&rels, &preds, 3, 1), 1.0);
        assert_eq!(compute_hits(&rels, &preds, 4, 1), 1.0);
        assert_eq!(compute_hits(&rels, &preds, 5, 1), 1.0);
        assert_eq!(compute_hits(&rels, &preds, 0, 2), 0.0);
    }
}
