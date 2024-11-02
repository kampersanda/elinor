from elinor import elinor


def test_evaluate() -> None:
    true_rels = [
        {"query_id": "q_1", "doc_id": "d_1", "score": 1},
        {"query_id": "q_1", "doc_id": "d_2", "score": 0},
        {"query_id": "q_1", "doc_id": "d_3", "score": 2},
        {"query_id": "q_2", "doc_id": "d_2", "score": 2},
        {"query_id": "q_2", "doc_id": "d_4", "score": 1},
    ]
    pred_rels = [
        {"query_id": "q_1", "doc_id": "d_1", "score": 0.5},
        {"query_id": "q_1", "doc_id": "d_2", "score": 0.4},
        {"query_id": "q_1", "doc_id": "d_3", "score": 0.3},
        {"query_id": "q_2", "doc_id": "d_4", "score": 0.1},
        {"query_id": "q_2", "doc_id": "d_1", "score": 0.2},
        {"query_id": "q_2", "doc_id": "d_3", "score": 0.3},
    ]
    scores = elinor._evaluate(true_rels, pred_rels, "ndcg@3")
    print(scores)
    assert scores["ndcg@3"] == 0.5
