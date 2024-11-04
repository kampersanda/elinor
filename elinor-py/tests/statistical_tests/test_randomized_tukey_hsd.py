from elinor.statistical_tests import RandomizedTukeyHsdTest


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
    result = RandomizedTukeyHsdTest(tupled_samples, 3)
    assert result.n_systems() == 3
    assert result.n_topics() == 20
    p_values = result.p_values()
    assert len(p_values) == 3
    assert all(len(values) == 3 for values in p_values)
    assert all(0.0 <= value <= 1.0 for values in p_values for value in values)
