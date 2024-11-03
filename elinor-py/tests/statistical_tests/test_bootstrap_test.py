from elinor.statistical_tests import BootstrapTest


def test_bootstrap_test() -> None:
    # From Table 5.1 in Sakai's book, "情報アクセス評価方法論".
    paired_samples = [
        (0.70, 0.50),
        (0.30, 0.10),
        (0.20, 0.00),
        (0.60, 0.20),
        (0.40, 0.40),
        (0.40, 0.30),
        (0.00, 0.00),
        (0.70, 0.50),
        (0.10, 0.30),
        (0.30, 0.30),
        (0.50, 0.40),
        (0.40, 0.40),
        (0.00, 0.10),
        (0.60, 0.40),
        (0.50, 0.20),
        (0.30, 0.10),
        (0.10, 0.10),
        (0.50, 0.60),
        (0.20, 0.30),
        (0.10, 0.20),
    ]
    result = BootstrapTest(paired_samples)
    assert result.n_topics() == 20
    assert result.n_resamples() == 10000
    assert 0.0 <= result.p_value() <= 1.0
