use std::path::PathBuf;

use anyhow::Result;
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
        let mut stats = vec![];
        for df in df_metrics.iter() {
            let values_1 = df.column("system_1")?.f64()?;
            let values_2 = df.column("system_2")?.f64()?;
            let paired_scores = values_1
                .into_iter()
                .zip(values_2.into_iter())
                .map(|(x, y)| (x.unwrap(), y.unwrap()));
            stats.push(StudentTTest::from_paired_samples(paired_scores)?);
        }
        let columns = vec![
            Series::new(
                "Metric".into(),
                metrics.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            ),
            Series::new(
                "Mean".into(),
                stats.iter().map(|stat| stat.mean()).collect::<Vec<_>>(),
            ),
            Series::new(
                "Variance".into(),
                stats.iter().map(|stat| stat.var()).collect::<Vec<_>>(),
            ),
            Series::new(
                "Effect Size".into(),
                stats
                    .iter()
                    .map(|stat| stat.effect_size())
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "T Stat".into(),
                stats.iter().map(|stat| stat.t_stat()).collect::<Vec<_>>(),
            ),
            Series::new(
                "P Value".into(),
                stats.iter().map(|stat| stat.p_value()).collect::<Vec<_>>(),
            ),
            Series::new(
                "95% MOE".into(),
                stats
                    .iter()
                    .map(|stat| stat.margin_of_error(0.05).unwrap())
                    .collect::<Vec<_>>(),
            ),
        ];
        println!("{:?}", DataFrame::new(columns)?);
    }

    println!("Bootstrap test");
    {
        let mut stats = vec![];
        for df in df_metrics.iter() {
            let values_1 = df.column("system_1")?.f64()?;
            let values_2 = df.column("system_2")?.f64()?;
            let paired_scores = values_1
                .into_iter()
                .zip(values_2.into_iter())
                .map(|(x, y)| (x.unwrap(), y.unwrap()));
            stats.push(BootstrapTest::from_paired_samples(paired_scores)?);
        }
        let columns = vec![
            Series::new(
                "Metric".into(),
                metrics.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            ),
            Series::new(
                "P Value".into(),
                stats.iter().map(|stat| stat.p_value()).collect::<Vec<_>>(),
            ),
        ];
        println!("{:?}", DataFrame::new(columns)?);
    }

    println!("Fisher's randomized test");
    {
        let mut stats = vec![];
        for df in df_metrics.iter() {
            let values_1 = df.column("system_1")?.f64()?;
            let values_2 = df.column("system_2")?.f64()?;
            let paired_scores = values_1
                .into_iter()
                .zip(values_2.into_iter())
                .map(|(x, y)| [x.unwrap(), y.unwrap()]);
            stats.push(RandomizedTukeyHsdTest::from_tupled_samples(
                paired_scores,
                2,
            )?);
        }
        let columns = vec![
            Series::new(
                "Metric".into(),
                metrics.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            ),
            Series::new(
                "P Value".into(),
                stats
                    .iter()
                    .map(|stat| stat.p_values()[0][1])
                    .collect::<Vec<_>>(),
            ),
        ];
        println!("{:?}", DataFrame::new(columns)?);
    }

    Ok(())
}
