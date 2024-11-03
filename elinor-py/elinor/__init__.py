import statistics

from pydantic import BaseModel

from elinor import _lowlevel


class TrueRecord(BaseModel, frozen=True):
    query_id: str
    doc_id: str
    score: int


class PredRecord(BaseModel, frozen=True):
    query_id: str
    doc_id: str
    score: float


class Evaluation(BaseModel, frozen=True):
    metric: str
    scores: dict[str, float]

    def mean(self) -> float:
        return statistics.mean(self.scores.values())


def evaluate(
    true_records: list[TrueRecord],
    pred_records: list[PredRecord],
    metric: str,
) -> Evaluation:
    """Evaluate the ranking performance.

    :param true_records: The true relevance scores.
    :param pred_records: The predicted relevance scores.
    :param metric: The evaluation metric to use.
    :return: The evaluation scores.
    """
    true_rels = [record.model_dump() for record in true_records]
    pred_rels = [record.model_dump() for record in pred_records]
    scores = _lowlevel._evaluate(true_rels, pred_rels, metric)
    return Evaluation(metric=metric, scores=scores)
