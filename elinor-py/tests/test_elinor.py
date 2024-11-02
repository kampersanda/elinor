import pytest
from elinor import PredRecord, TrueRecord, evaluate
from elinor.statistical_tests import StudentTTest


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


def test_student_t_test() -> None:
    stat = StudentTTest.from_maps(
        {"a": 0.60, "b": 0.10, "c": 0.20},
        {"a": 0.50, "b": 0.10, "c": 0.00},
    )
    assert stat.n_samples() == 3
    assert stat.mean() == pytest.approx((0.10 + 0.00 + 0.20) / 3.0)
    assert stat.variance() == pytest.approx(
        (
            (0.10 - stat.mean()) ** 2
            + (0.00 - stat.mean()) ** 2
            + (0.20 - stat.mean()) ** 2
        )
        / 2.0
    )
    assert stat.effect_size() == pytest.approx(stat.mean() / stat.variance() ** 0.5)
    assert stat.t_stat() == pytest.approx(stat.mean() / (stat.variance() / 3.0) ** 0.5)
    assert 0.0 <= stat.p_value() <= 1.0
    moe95 = stat.margin_of_error(0.05)
    assert moe95 > 0.0
    ci95_btm, ci95_top = stat.confidence_interval(0.05)
    assert ci95_btm == pytest.approx(stat.mean() - moe95)
    assert ci95_top == pytest.approx(stat.mean() + moe95)
