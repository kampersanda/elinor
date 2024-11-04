import pytest

from elinor.statistical_tests import TwoWayAnovaWithoutReplication


def test_two_way_anova_without_replication() -> None:
    # From Table 5.1 in Sakai's book, "情報アクセス評価方法論".
    tupled_samples = [
        [0.70, 0.50, 0.00],
        [0.30, 0.10, 0.00],
        [0.20, 0.00, 0.20],
        [0.60, 0.20, 0.10],
        [0.40, 0.40, 0.30],
        [0.40, 0.30, 0.30],
        [0.00, 0.00, 0.10],
        [0.70, 0.50, 0.20],
        [0.10, 0.30, 0.40],
        [0.30, 0.30, 0.40],
        [0.50, 0.40, 0.40],
        [0.40, 0.40, 0.30],
        [0.00, 0.10, 0.30],
        [0.60, 0.40, 0.20],
        [0.50, 0.20, 0.20],
        [0.30, 0.10, 0.20],
        [0.10, 0.10, 0.10],
        [0.50, 0.60, 0.50],
        [0.20, 0.30, 0.40],
        [0.10, 0.20, 0.30],
    ]
    result = TwoWayAnovaWithoutReplication(tupled_samples, 3)
    assert result.n_systems() == 3
    assert result.n_topics() == 20
    assert result.between_system_variation() == pytest.approx(0.1083, abs=1e-4)
    assert result.between_topic_variation() == pytest.approx(1.0293, abs=1e-4)
    assert result.residual_variation() == pytest.approx(0.8317, abs=1e-4)
    assert result.between_system_variance() == pytest.approx(0.0542, abs=1e-4)
    assert result.between_topic_variance() == pytest.approx(0.0542, abs=1e-4)
    assert result.residual_variance() == pytest.approx(0.0219, abs=1e-4)
    assert result.between_system_f_stat() == pytest.approx(2.475, abs=1e-3)
    assert result.between_topic_f_stat() == pytest.approx(2.475, abs=1e-3)
    assert result.between_system_p_value() == pytest.approx(0.098, abs=1e-3)
    assert result.between_topic_p_value() == pytest.approx(0.009, abs=1e-3)
    assert result.margin_of_error(0.05) == pytest.approx(0.0670, abs=1e-4)
