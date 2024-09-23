use anyhow::Result;
use elinor::statistical_tests::TukeyHsdTest;

fn main() -> Result<()> {
    // From Table 5.1 in Sakai's book, "情報アクセス評価方法論".
    let a = vec![
        0.70, 0.30, 0.20, 0.60, 0.40, 0.40, 0.00, 0.70, 0.10, 0.30, //
        0.50, 0.40, 0.00, 0.60, 0.50, 0.30, 0.10, 0.50, 0.20, 0.10,
    ];
    let b = vec![
        0.50, 0.10, 0.00, 0.20, 0.40, 0.30, 0.00, 0.50, 0.30, 0.30, //
        0.40, 0.40, 0.10, 0.40, 0.20, 0.10, 0.10, 0.60, 0.30, 0.20,
    ];
    let c = vec![
        0.00, 0.00, 0.20, 0.10, 0.30, 0.30, 0.10, 0.20, 0.40, 0.40, //
        0.40, 0.30, 0.30, 0.20, 0.20, 0.20, 0.10, 0.50, 0.40, 0.30,
    ];

    let tupled_samples = a
        .iter()
        .zip(b.iter())
        .zip(c.iter())
        .map(|((&a, &b), &c)| [a, b, c]);
    let result = TukeyHsdTest::from_tupled_samples(tupled_samples, 3)?;

    for (i, mean) in result.system_means().iter().enumerate() {
        println!("mean for system {i}: {mean:.4}");
    }
    let residual_var = result.residual_var();
    println!("residual_var: {residual_var:.4}");
    for (i, j, effect_size) in result.effect_sizes() {
        println!("effect size for {i} and {j}: {effect_size:.4}");
    }
    for (i, j, t_stat) in result.t_stats() {
        println!("t-stat for {i} and {j}: {t_stat:.4}");
    }
    for (i, j, p_value) in result.p_values() {
        println!("p-value for {i} and {j}: {p_value:.4}");
    }

    let moe95 = result.margin_of_error(0.05)?;
    println!("Margin of error at a 95% confidence level: {moe95:.4}");

    let ci95s = result.confidence_intervals(0.05)?;
    for (i, (btm, top)) in ci95s.iter().enumerate() {
        println!(
            "Confidence interval for system {i} at a 95% confidence level: [{btm:.4},{top:.4}]",
        );
    }

    Ok(())
}
