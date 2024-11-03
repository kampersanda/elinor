from __future__ import annotations

from typing import Any

def _evaluate(
    true_rels: list[dict[str, Any]],
    pred_rels: list[dict[str, Any]],
    metric: str,
) -> dict[str, float]: ...

class _StudentTTest:
    def __init__(self, paired_samples: list[tuple[float, float]]) -> None: ...
    @staticmethod
    def from_maps(a: dict[str, float], b: dict[str, float]) -> _StudentTTest: ...
    def n_topics(self) -> int: ...
    def mean(self) -> float: ...
    def variance(self) -> float: ...
    def effect_size(self) -> float: ...
    def t_stat(self) -> float: ...
    def p_value(self) -> float: ...
    def margin_of_error(self, significance_level: float) -> float: ...
    def confidence_interval(self, significance_level: float) -> tuple[float, float]: ...

class _BootstrapTest:
    def __init__(
        self,
        paired_samples: list[tuple[float, float]],
        n_resamples: int = 10000,
        random_state: int | None = None,
    ) -> None: ...
    @staticmethod
    def from_maps(
        a: dict[str, float],
        b: dict[str, float],
        n_resamples: int = 10000,
        random_state: int | None = None,
    ) -> _BootstrapTest: ...
    def n_topics(self) -> int: ...
    def n_resamples(self) -> int: ...
    def random_state(self) -> int: ...
    def p_value(self) -> float: ...

class _TwoWayAnovaWithoutReplication:
    def __init__(
        self,
        tupled_samples: list[list[float]],
        n_systems: int,
    ) -> None: ...
    @staticmethod
    def from_maps(maps: list[dict[str, float]]) -> _TwoWayAnovaWithoutReplication: ...
    def n_systems(self) -> int: ...
    def n_topics(self) -> int: ...
    def system_means(self) -> list[float]: ...
    def topic_means(self) -> list[float]: ...
    def between_system_variation(self) -> float: ...
    def between_topic_variation(self) -> float: ...
    def residual_variation(self) -> float: ...
    def between_system_variance(self) -> float: ...
    def between_topic_variance(self) -> float: ...
    def residual_variance(self) -> float: ...
    def between_system_f_stat(self) -> float: ...
    def between_topic_f_stat(self) -> float: ...
    def between_system_p_value(self) -> float: ...
    def between_topic_p_value(self) -> float: ...
    def margin_of_error(self, significance_level: float) -> float: ...

class _TukeyHsdTest:
    def __init(
        self,
        tupled_samples: list[list[float]],
        n_systems: int,
    ) -> None: ...
    @staticmethod
    def from_maps(maps: list[dict[str, float]]) -> _TukeyHsdTest: ...
    def n_systems(self) -> int: ...
    def n_topics(self) -> int: ...
    def effect_sizes(self) -> list[list[float]]: ...

class _RandomizedTukeyHsdTest:
    def __init(
        self,
        tupled_samples: list[list[float]],
        n_systems: int,
        n_iters: int = 10000,
        random_state: int | None = None,
    ) -> None: ...
    @staticmethod
    def from_maps(
        maps: list[dict[str, float]],
        n_iters: int = 10000,
        random_state: int | None = None,
    ) -> _RandomizedTukeyHsdTest: ...
    def n_systems(self) -> int: ...
    def n_topics(self) -> int: ...
    def n_iters(self) -> int: ...
    def random_state(self) -> int: ...
    def p_values(self) -> list[list[float]]: ...