from typing import Any

def evaluate(
    true_rels: list[dict[str, Any]],
    pred_rels: list[dict[str, Any]],
    metric: str,
) -> dict[str, float]:
    """Evaluate the ranking performance.

    :param true_rels: The true relevance scores.
    :param pred_rels: The predicted relevance scores.
    :param metric: The evaluation metric to use.
    :return: The evaluation scores.
    """
    pass
