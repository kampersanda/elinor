use anyhow::Result;
use elinor::statistical_tests::BootstrapTest;

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

    let paired_samples = a.into_iter().zip(b.into_iter()).map(|(x, y)| (x, y));
    let result = BootstrapTest::from_paired_samples(paired_samples)?;

    println!("Mean: {:.4}", result.mean());
    println!("Variance: {:.4}", result.var());
    println!("Effect size: {:.4}", result.effect_size());
    println!("p-value: {:.4}", result.p_value());

    Ok(())
}
