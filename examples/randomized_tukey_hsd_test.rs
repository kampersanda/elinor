use anyhow::Result;
use elinor::statistical_tests::RandomizedTukeyHsdTest;

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

    println!("Comparing two systems (equivalent to Fisher's randomization test).");
    let tupled_samples = a.iter().zip(b.iter()).map(|(&a, &b)| [a, b]);
    let result = RandomizedTukeyHsdTest::from_tupled_samples(tupled_samples, 2)?;
    let p_values = result.p_values();
    println!("p-value for 0 and 1: {:.4}", p_values[0][1]);

    println!("Comparing three systems.");
    let tupled_samples = a
        .iter()
        .zip(b.iter())
        .zip(c.iter())
        .map(|((&a, &b), &c)| [a, b, c]);
    let result = RandomizedTukeyHsdTest::from_tupled_samples(tupled_samples, 3)?;
    let p_values = result.p_values();
    println!("p-value for 0 and 1: {:.4}", p_values[0][1]);
    println!("p-value for 0 and 2: {:.4}", p_values[0][2]);
    println!("p-value for 1 and 2: {:.4}", p_values[1][2]);

    let effect_sizes = result.effect_sizes();
    println!("Effect size for 0 and 1: {:.4}", effect_sizes[0][1]);
    println!("Effect size for 0 and 2: {:.4}", effect_sizes[0][2]);
    println!("Effect size for 1 and 2: {:.4}", effect_sizes[1][2]);

    Ok(())
}
