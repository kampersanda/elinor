use std::{fmt::format, path::PathBuf};

use anyhow::Result;
use big_s::S;
use clap::Parser;
use elinor::statistical_tests::{BootstrapTest, RandomizedTukeyHsdTest, StudentTTest};
use polars::prelude::*;
use polars_lazy::prelude::*;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long, help = "Path to the input CSV files")]
    input_csvs: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut dfs = vec![];
    for input_csv in args.input_csvs {
        let df = CsvReadOptions::default()
            .try_into_reader_with_file_path(Some(input_csv))?
            .finish()?;
        dfs.push(df);
    }

    let metrics = extract_metrics(&dfs[0]);
    let mut columns = vec![Series::new(
        "Metric".into(),
        metrics.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
    )];
    for (i, df) in dfs.iter().enumerate() {
        let means = df
            .clone()
            .lazy()
            .select([col("*").exclude(["query_id"]).mean()])
            .collect()?;
        let values = metrics
            .iter()
            .map(|metric| {
                means
                    .column(metric)
                    .unwrap()
                    .f64()
                    .unwrap()
                    .first()
                    .unwrap()
            })
            .collect::<Vec<_>>();
        columns.push(Series::new(format!("System_{}", i + 1).into(), values));
    }
    println!("Means");
    println!("{:?}", DataFrame::new(columns)?);

    if dfs.len() == 2 {
        compare_two_systems(&dfs[0], &dfs[1])?;
    }

    Ok(())
}

fn extract_metrics(df: &DataFrame) -> Vec<String> {
    df.get_columns()
        .iter()
        .skip(1) // The first column is the query_id
        .map(|column| column.name().to_string())
        .collect()
}

fn compare_two_systems(df_1: &DataFrame, df_2: &DataFrame) -> Result<()> {
    let metrics = extract_metrics(df_1);

    let mut df_metrics = vec![];
    for metric in &metrics {
        let metric = metric.as_str();
        let system_1 = df_1
            .clone()
            .lazy()
            .select([col("query_id"), col(metric).alias("system_1")])
            .collect()?;
        let system_2 = df_2
            .clone()
            .lazy()
            .select([col("query_id"), col(metric).alias("system_2")])
            .collect()?;
        let joined = system_1
            .clone()
            .lazy()
            .join(
                system_2.clone().lazy(),
                [col("query_id")],
                [col("query_id")],
                JoinArgs::new(JoinType::Left),
            )
            .collect()?;
        df_metrics.push(joined);
    }

    println!("Paired Student's t-test");
    {
        let mut rows: Vec<Vec<String>> = Vec::new();
        rows.push(vec![
            S("Metric"),
            S("Mean"),
            S("Variance"),
            S("Effect Size"),
            S("T Stat"),
            S("P Value"),
            S("95% CI"),
        ]);
        for (metric, df) in metrics.iter().zip(df_metrics.iter()) {
            let values_1 = df.column("system_1")?.f64()?;
            let values_2 = df.column("system_2")?.f64()?;
            let paired_scores = values_1
                .into_iter()
                .zip(values_2.into_iter())
                .map(|(x, y)| (x.unwrap(), y.unwrap()));
            let stat = StudentTTest::from_paired_samples(paired_scores)?;
            let (ci95_btm, ci95_top) = stat.confidence_interval(0.05).unwrap();
            rows.push(vec![
                format!("{}", metric.as_str()),
                format!("{:.4}", stat.mean()),
                format!("{:.4}", stat.var()),
                format!("{:.4}", stat.effect_size()),
                format!("{:.4}", stat.t_stat()),
                format!("{:.4}", stat.p_value()),
                format!("[{:.4}, {:.4}]", ci95_btm, ci95_top),
            ]);
        }
        elinor_commands::to_prettytable(rows).printstd();
    }

    println!("Bootstrap test");
    {
        let mut rows: Vec<Vec<String>> = Vec::new();
        rows.push(vec![S("Metric"), S("P Value")]);
        for (metric, df) in metrics.iter().zip(df_metrics.iter()) {
            let values_1 = df.column("system_1")?.f64()?;
            let values_2 = df.column("system_2")?.f64()?;
            let paired_scores = values_1
                .into_iter()
                .zip(values_2.into_iter())
                .map(|(x, y)| (x.unwrap(), y.unwrap()));
            let stat = BootstrapTest::from_paired_samples(paired_scores).unwrap();
            rows.push(vec![format!("{metric}"), format!("{:.4}", stat.p_value())]);
        }
        elinor_commands::to_prettytable(rows).printstd();
    }

    println!("Fisher's randomized test");
    {
        let mut rows: Vec<Vec<String>> = Vec::new();
        rows.push(vec![S("Metric"), S("P Value")]);
        for (metric, df) in metrics.iter().zip(df_metrics.iter()) {
            let values_1 = df.column("system_1")?.f64()?;
            let values_2 = df.column("system_2")?.f64()?;
            let paired_scores = values_1
                .into_iter()
                .zip(values_2.into_iter())
                .map(|(x, y)| [x.unwrap(), y.unwrap()]);
            let stat = RandomizedTukeyHsdTest::from_tupled_samples(paired_scores, 2).unwrap();
            let p_values = stat.p_values();
            rows.push(vec![format!("{metric}"), format!("{:.4}", p_values[0][1])]);
        }
        elinor_commands::to_prettytable(rows).printstd();
    }

    Ok(())
}
