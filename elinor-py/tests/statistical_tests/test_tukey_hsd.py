import pytest

from elinor.statistical_tests import TukeyHsdTest


def test_tukey_hsd() -> None:
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
    result = TukeyHsdTest(tupled_samples, 3)
    assert result.n_systems() == 3
    assert result.n_topics() == 20
    effect_sizes = result.effect_sizes()
    assert len(effect_sizes) == 3
    assert effect_sizes[0] == pytest.approx([0.000, 0.5070, 0.6760], abs=1e-4)
    assert effect_sizes[1] == pytest.approx([-0.5070, 0.0000, 0.1690], abs=1e-4)
    assert effect_sizes[2] == pytest.approx([-0.6760, -0.1690, 0.000], abs=1e-4)
