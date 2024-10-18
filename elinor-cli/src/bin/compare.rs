use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use elinor::statistical_tests::bootstrap_test::BootstrapTester;
use elinor::statistical_tests::randomized_tukey_hsd_test::RandomizedTukeyHsdTester;
use elinor::statistical_tests::{StudentTTest, TwoWayAnovaWithoutReplication};
use polars::prelude::*;
use polars_lazy::prelude::*;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long, num_args = 1.., help = "Path to the input CSV files")]
    input_csvs: Vec<PathBuf>,

    #[arg(
        short,
        long,
        default_value = "query_id",
        help = "Header name of the topic identifier column"
    )]
    topic_header: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.input_csvs.is_empty() {
        return Err(anyhow::anyhow!("Specify at least one input CSV file."));
    }

    let mut dfs = vec![];
    for input_csv in &args.input_csvs {
        let df = CsvReadOptions::default()
            .try_into_reader_with_file_path(Some(input_csv.clone()))?
            .finish()?;
        dfs.push(df);
    }

    // If there is only one input CSV file, just print the means.
    if args.input_csvs.len() == 1 {
        println!("# Means");
        {
            let metrics = extract_metrics(&dfs[0]);
            let values = get_means(&dfs[0], &metrics, &args.topic_header);
            let columns = vec![
                Series::new("Metric".into(), metrics),
                Series::new("Score".into(), values),
            ];
            let df = DataFrame::new(columns)?;
            df_to_prettytable(&df).printstd();
        }
        return Ok(());
    }

    println!("# Alias");
    {
        let columns = vec![
            Series::new(
                "Alias".into(),
                (1..=dfs.len())
                    .map(|i| format!("System_{}", i))
                    .collect::<Vec<_>>(),
            ),
            Series::new(
                "Path".into(),
                args.input_csvs
                    .iter()
                    .map(|p| p.to_string_lossy())
                    .collect::<Vec<_>>(),
            ),
        ];
        let df = DataFrame::new(columns)?;
        df_to_prettytable(&df).printstd();
    }

    if dfs.len() == 2 {
        compare_two_systems(&dfs[0], &dfs[1], &args.topic_header)?;
    }
    if dfs.len() > 2 {
        compare_multiple_systems(&dfs, &args.topic_header)?;
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

fn extract_common_metrics<'a, I>(dfs: I) -> Vec<String>
where
    I: IntoIterator<Item = &'a DataFrame>,
{
    let mut dfs = dfs.into_iter();
    let mut common_metrics = extract_metrics(dfs.next().unwrap());
    for df in dfs {
        let metrics = extract_metrics(df);
        common_metrics.retain(|metric| metrics.contains(metric));
    }
    common_metrics
}

fn get_means(df: &DataFrame, metrics: &[String], topic_header: &str) -> Vec<f64> {
    let means = df
        .clone()
        .lazy()
        .select([col("*").exclude([topic_header]).mean()])
        .collect()
        .unwrap();
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
    values
}

fn compare_two_systems(df_1: &DataFrame, df_2: &DataFrame, topic_header: &str) -> Result<()> {
    let metrics = extract_common_metrics([df_1, df_2]);
    if metrics.is_empty() {
        return Err(anyhow::anyhow!("No common metrics found."));
    }

    println!("\n# Means");
    {
        let mut columns = vec![Series::new(
            "Metric".into(),
            metrics.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
        )];
        for (i, df) in [df_1, df_2].into_iter().enumerate() {
            let values = get_means(df, &metrics, topic_header);
            columns.push(Series::new(format!("System_{}", i + 1).into(), values));
        }
        let df = DataFrame::new(columns)?;
        df_to_prettytable(&df).printstd();
    }

    let mut df_metrics = vec![];
    for metric in &metrics {
        let metric = metric.as_str();
        let system_1 = df_1
            .clone()
            .lazy()
            .select([col(topic_header), col(metric).alias("system_1")])
            .collect()?;
        let system_2 = df_2
            .clone()
            .lazy()
            .select([col(topic_header), col(metric).alias("system_2")])
            .collect()?;
        let joined = system_1
            .clone()
            .lazy()
            .join(
                system_2.clone().lazy(),
                [col(topic_header)],
                [col(topic_header)],
                JoinArgs::new(JoinType::Left),
            )
            .collect()?;
        df_metrics.push(joined);
    }

    println!("\n# Two-sided paired Student's t-test for (System_1 - System_2)");
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
                "Var".into(),
                stats.iter().map(|stat| stat.var()).collect::<Vec<_>>(),
            ),
            Series::new(
                "ES".into(),
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
        let df = DataFrame::new(columns)?;
        df_to_prettytable(&df).printstd();
    }

    let n_resamples = 10000;
    println!("\n# Two-sided paired Bootstrap test (n_resamples = {n_resamples})");
    {
        let mut stats = vec![];
        let tester = BootstrapTester::new().with_n_resamples(n_resamples);
        for df in df_metrics.iter() {
            let values_1 = df.column("system_1")?.f64()?;
            let values_2 = df.column("system_2")?.f64()?;
            let paired_scores = values_1
                .into_iter()
                .zip(values_2.into_iter())
                .map(|(x, y)| (x.unwrap(), y.unwrap()));
            stats.push(tester.test_for_paired_samples(paired_scores)?);
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
        let df = DataFrame::new(columns)?;
        df_to_prettytable(&df).printstd();
    }

    let n_iters = 10000;
    println!("\n# Fisher's randomized test (n_iters = {n_iters})");
    {
        let mut stats = vec![];
        let tester = RandomizedTukeyHsdTester::new(2).with_n_iters(n_iters);
        for df in df_metrics.iter() {
            let values_1 = df.column("system_1")?.f64()?;
            let values_2 = df.column("system_2")?.f64()?;
            let paired_scores = values_1
                .into_iter()
                .zip(values_2.into_iter())
                .map(|(x, y)| [x.unwrap(), y.unwrap()]);
            stats.push(tester.test(paired_scores)?);
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
        let df = DataFrame::new(columns)?;
        df_to_prettytable(&df).printstd();
    }

    Ok(())
}

fn compare_multiple_systems(dfs: &[DataFrame], topic_header: &str) -> Result<()> {
    let metrics = extract_common_metrics(dfs);
    if metrics.is_empty() {
        return Err(anyhow::anyhow!("No common metrics found."));
    }

    let mut df_metrics = vec![];
    for metric in &metrics {
        let metric = metric.as_str();
        let mut df_systems = vec![];
        for (i, df) in dfs.iter().enumerate() {
            let df_system = df
                .clone()
                .lazy()
                .select([
                    col(topic_header),
                    col(metric).alias(format!("system_{}", i + 1)),
                ])
                .collect()?;
            df_systems.push(df_system);
        }
        let joined = df_systems
            .iter()
            .skip(1)
            .fold(df_systems[0].clone(), |acc, df| {
                acc.lazy()
                    .join(
                        df.clone().lazy(),
                        [col(topic_header)],
                        [col(topic_header)],
                        JoinArgs::new(JoinType::Left),
                    )
                    .collect()
                    .unwrap()
            });
        df_metrics.push(joined);
    }

    let n_iters = 10000;
    let hsd_tester = RandomizedTukeyHsdTester::new(dfs.len()).with_n_iters(n_iters);

    for (metric, df_metric) in metrics.iter().zip(df_metrics.iter()) {
        println!("\n# {metric:#}");

        let mut data = vec![];
        for i in 0..dfs.len() {
            let values = df_metric
                .column(format!("system_{}", i + 1).as_str())?
                .f64()?;
            data.push(values);
        }
        let mut tupled_scores = vec![];
        for i in 0..data[0].len() {
            let mut scores = vec![];
            for values in &data {
                scores.push(values.get(i).unwrap());
            }
            tupled_scores.push(scores);
        }

        println!("## Statistics for system means");
        let anove_stat =
            TwoWayAnovaWithoutReplication::from_tupled_samples(tupled_scores.iter(), dfs.len())?;
        let system_means = anove_stat.system_means();
        let moe95 = anove_stat.margin_of_error(0.05)?;
        let columns = vec![
            Series::new(
                "System".into(),
                (1..=dfs.len())
                    .map(|i| format!("System_{i}"))
                    .collect::<Vec<_>>(),
            ),
            Series::new("Mean".into(), system_means.to_vec()),
            Series::new("95% MOE".into(), vec![moe95; dfs.len()]),
        ];
        let df = DataFrame::new(columns)?;
        df_to_prettytable(&df).printstd();

        println!("## Two-way ANOVA without replication");
        let columns = vec![
            Series::new(
                "Factor".into(),
                vec!["Between-systems", "Between-topics", "Residual"],
            ),
            Series::new(
                "Variation ".into(),
                vec![
                    anove_stat.between_system_variation(),
                    anove_stat.between_topic_variation(),
                    anove_stat.residual_variation(),
                ],
            ),
            Series::new(
                "DF".into(),
                vec![
                    anove_stat.n_systems() as u64 - 1,
                    anove_stat.n_topics() as u64 - 1,
                    (anove_stat.n_systems() as u64 - 1) * (anove_stat.n_topics() as u64 - 1),
                ],
            ),
            Series::new(
                "Variance".into(),
                vec![
                    anove_stat.between_system_variance(),
                    anove_stat.between_topic_variance(),
                    anove_stat.residual_variance(),
                ],
            ),
            Series::new(
                "F Stat".into(),
                vec![
                    anove_stat.between_system_f_stat(),
                    anove_stat.between_topic_f_stat(),
                    f64::NAN,
                ],
            ),
            Series::new(
                "P Value".into(),
                vec![
                    anove_stat.between_system_p_value(),
                    anove_stat.between_topic_p_value(),
                    f64::NAN,
                ],
            ),
        ];
        let df = DataFrame::new(columns)?;
        df_to_prettytable(&df).printstd();

        println!("## Between-system effect sizes for randomized Tukey HSD test");
        let hsd_stat = hsd_tester.test(tupled_scores.iter())?;
        let effect_sizes = hsd_stat.effect_sizes();
        let mut columns = vec![Series::new(
            "ES".into(),
            (1..=dfs.len())
                .map(|i| format!("System_{}", i))
                .collect::<Vec<_>>(),
        )];
        for i in 1..=dfs.len() {
            let values = (1..=dfs.len())
                .map(|j| effect_sizes[j - 1][i - 1])
                .collect::<Vec<_>>();
            columns.push(Series::new(format!("System_{}", i).into(), values));
        }
        let df = DataFrame::new(columns)?;
        df_to_prettytable(&df).printstd();

        println!("## Between-system P values for randomized Tukey HSD test (n_iters = {n_iters})");
        let p_values = hsd_stat.p_values();
        let mut columns = vec![Series::new(
            "P Value".into(),
            (1..=dfs.len())
                .map(|i| format!("System_{}", i))
                .collect::<Vec<_>>(),
        )];
        for i in 1..=dfs.len() {
            let values = (1..=dfs.len())
                .map(|j| p_values[j - 1][i - 1])
                .collect::<Vec<_>>();
            columns.push(Series::new(format!("System_{}", i).into(), values));
        }
        let df = DataFrame::new(columns)?;
        df_to_prettytable(&df).printstd();
    }

    Ok(())
}

fn df_to_prettytable(df: &DataFrame) -> prettytable::Table {
    let columns = df.get_columns();
    let mut table = prettytable::Table::new();
    table.set_titles(prettytable::Row::new(
        columns
            .iter()
            .map(|s| s.name().as_str())
            .map(prettytable::Cell::new)
            .collect(),
    ));
    for i in 0..df.height() {
        let mut row = vec![];
        for column in columns.iter() {
            let value = column.get(i).unwrap();
            match value {
                AnyValue::String(value) => {
                    row.push(prettytable::Cell::new(value));
                }
                AnyValue::Float64(value) => {
                    if value.is_nan() {
                        row.push(prettytable::Cell::new(""));
                    } else {
                        row.push(prettytable::Cell::new(&format!("{value:.4}")));
                    }
                }
                AnyValue::UInt64(value) => {
                    row.push(prettytable::Cell::new(&format!("{value}")));
                }
                _ => {
                    row.push(prettytable::Cell::new("N/A"));
                }
            }
        }
        table.add_row(prettytable::Row::new(row));
    }
    table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table
}
