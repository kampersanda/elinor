use anyhow::Result;
use elinor::statistical_tests::RandomizedTukeyHsdTest;

fn main() -> Result<()> {
    // From Table 5.1 in Sakai's book, "情報アクセス評価方法論".
    let x = vec![
        0.70, 0.30, 0.20, 0.60, 0.40, 0.40, 0.00, 0.70, 0.10, 0.30, //
        0.50, 0.40, 0.00, 0.60, 0.50, 0.30, 0.10, 0.50, 0.20, 0.10,
    ];
    let y = vec![
        0.50, 0.10, 0.00, 0.20, 0.40, 0.30, 0.00, 0.50, 0.30, 0.30, //
        0.40, 0.40, 0.10, 0.40, 0.20, 0.10, 0.10, 0.60, 0.30, 0.20,
    ];
    let z = vec![
        0.00, 0.00, 0.20, 0.10, 0.30, 0.30, 0.10, 0.20, 0.40, 0.40, //
        0.40, 0.30, 0.30, 0.20, 0.20, 0.20, 0.10, 0.50, 0.40, 0.30,
    ];

    println!("Comparing two systems.");
    let tupled_samples = x.iter().zip(y.iter()).map(|(&x, &y)| [x, y]);
    let result = RandomizedTukeyHsdTest::from_tupled_samples(tupled_samples, 2)?;
    for (i, j, p_value) in result.p_values() {
        println!("p-value for {} and {}: {:.4}", i, j, p_value);
    }

    println!("Comparing three systems.");
    let tupled_samples = x
        .iter()
        .zip(y.iter())
        .zip(z.iter())
        .map(|((&x, &y), &z)| [x, y, z]);
    let result = RandomizedTukeyHsdTest::from_tupled_samples(tupled_samples, 3)?;
    for (i, j, p_value) in result.p_values() {
        println!("p-value for {} and {}: {:.4}", i, j, p_value);
    }

    Ok(())
}
