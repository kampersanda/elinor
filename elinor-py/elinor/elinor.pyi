from __future__ import annotations

from typing import Any

def _evaluate(
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

class _StudentTTest:
    def __init__(self, paired_samples: list[tuple[float, float]]) -> None:
        """Initialize the Student's t-test.

        :param paired_samples: The paired samples to test.
        """
        pass

    @staticmethod
    def from_maps(a: dict[str, float], b: dict[str, float]) -> _StudentTTest:
        """Create a Student's t-test from two maps.

        :param a: The first map.
        :param b: The second map.
        :return: The Student's t-test.
        """
        pass

    def n_samples(self) -> int:
        """Return the number of samples.

        :return: The number of samples.
        """
        pass

    def mean(self) -> float:
        """Return the mean.

        :return: The mean.
        """
        pass

    def variance(self) -> float:
        """Return the variance.

        :return: The variance.
        """
        pass

    def effect_size(self) -> float:
        """Return the effect size.

        :return: The effect size.
        """
        pass

    def t_stat(self) -> float:
        """Return the t-statistic.

        :return: The t-statistic.
        """
        pass

    def p_value(self) -> float:
        """Return the p-value.

        :return: The p-value.
        """
        pass

    def margin_of_error(self, significance_level: float) -> float:
        """Return the margin of error.

        :param significance_level: The significance level.
        :return: The margin of error.
        """
        pass

    def confidence_interval(self, significance_level: float) -> tuple[float, float]:
        """Return the confidence interval.

        :param significance_level: The significance level.
        :return: The confidence interval.
        """
        pass
