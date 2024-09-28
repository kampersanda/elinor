use anyhow::Result;
use elinor::statistical_tests::TwoWayAnovaWithoutReplication;

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
    let stat = TwoWayAnovaWithoutReplication::from_tupled_samples(tupled_samples, 3)?;
    println!("n_systems: {}", stat.n_systems());
    println!("n_topics: {}", stat.n_topics());
    println!(
        "between_system_variation: {:.4}",
        stat.between_system_variation()
    );
    println!(
        "between_topic_variation: {:.4}",
        stat.between_topic_variation()
    );
    println!("residual_variation: {:.4}", stat.residual_variation());
    println!(
        "between_system_variance: {:.4}",
        stat.between_system_variance()
    );
    println!(
        "between_topic_variance: {:.4}",
        stat.between_topic_variance()
    );
    println!("residual_variance: {:.4}", stat.residual_variance());
    println!("between_system_f_stat: {:.4}", stat.between_system_f_stat());
    println!("between_topic_f_stat: {:.4}", stat.between_topic_f_stat());
    println!(
        "between_system_p_value: {:.4}",
        stat.between_system_p_value()
    );
    println!("between_topic_p_value: {:.4}", stat.between_topic_p_value());

    let moe95 = stat.margin_of_error(0.05)?;
    let system_means = stat.system_means();
    for (i, mean) in system_means.iter().enumerate() {
        let ci95_btm = mean - moe95;
        let ci95_top = mean + moe95;
        println!("Mean and 95% CI of system {i}: {mean:.4} [{ci95_btm:.4}, {ci95_top:.4}]");
    }

    let effect_sizes = stat.between_system_effect_sizes();
    for i in 0..stat.n_systems() {
        for j in (i + 1)..stat.n_systems() {
            let effect_size = effect_sizes[i][j];
            println!("Effect size between system {i} and {j}: {effect_size:.4}");
        }
    }

    Ok(())
}
