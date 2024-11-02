from elinor import PredRecord, TrueRecord, evaluate


def test_evaluate() -> None:
    true_records = [
        TrueRecord(query_id="q_1", doc_id="d_1", score=1),
        TrueRecord(query_id="q_1", doc_id="d_2", score=0),
        TrueRecord(query_id="q_1", doc_id="d_3", score=2),
        TrueRecord(query_id="q_2", doc_id="d_2", score=2),
        TrueRecord(query_id="q_2", doc_id="d_4", score=1),
    ]
    pred_records = [
        PredRecord(query_id="q_1", doc_id="d_1", score=0.5),
        PredRecord(query_id="q_1", doc_id="d_2", score=0.4),
        PredRecord(query_id="q_1", doc_id="d_3", score=0.3),
        PredRecord(query_id="q_2", doc_id="d_4", score=0.1),
        PredRecord(query_id="q_2", doc_id="d_1", score=0.2),
        PredRecord(query_id="q_2", doc_id="d_3", score=0.3),
    ]
    result = evaluate(true_records, pred_records, "ndcg@3")
    assert result.metric == "ndcg@3"
