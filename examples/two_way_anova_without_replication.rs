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
    let result = TwoWayAnovaWithoutReplication::from_tupled_samples(tupled_samples, 3)?;
    println!("n_systems: {}", result.n_systems());
    println!("n_topics: {}", result.n_topics());
    println!(
        "between_system_sum_of_squares: {:.4}",
        result.between_system_sum_of_squares()
    );
    println!(
        "between_topic_sum_of_squares: {:.4}",
        result.between_topic_sum_of_squares()
    );
    println!(
        "residual_sum_of_squares: {:.4}",
        result.residual_sum_of_squares()
    );
    println!(
        "between_system_mean_square: {:.4}",
        result.between_system_mean_square()
    );
    println!(
        "between_topic_mean_square: {:.4}",
        result.between_topic_mean_square()
    );
    println!("residual_mean_square: {:.4}", result.residual_mean_square());
    println!(
        "between_system_f_stat: {:.4}",
        result.between_system_f_stat()
    );
    println!("between_topic_f_stat: {:.4}", result.between_topic_f_stat());
    println!(
        "between_system_p_value: {:.4}",
        result.between_system_p_value()
    );
    println!(
        "between_topic_p_value: {:.4}",
        result.between_topic_p_value()
    );

    Ok(())
}
