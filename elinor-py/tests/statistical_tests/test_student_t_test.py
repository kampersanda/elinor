import pytest

from elinor.statistical_tests import StudentTTest


def test_student_t_test() -> None:
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
    result = StudentTTest(paired_samples)
    assert result.n_topics() == 20
    assert result.mean() == pytest.approx(0.0750, abs=1e-4)
    assert result.variance() == pytest.approx(0.0251, abs=1e-4)
    assert result.effect_size() == pytest.approx(0.473, abs=1e-3)
    assert result.t_stat() == pytest.approx(2.116, abs=1e-3)
    assert result.p_value() == pytest.approx(0.048, abs=1e-3)

    moe95 = result.margin_of_error(0.05)
    assert moe95 == pytest.approx(0.0742, abs=1e-4)

    mean = result.mean()
    ci95 = result.confidence_interval(0.05)
    assert ci95 == pytest.approx((mean - moe95, mean + moe95), abs=1e-4)
