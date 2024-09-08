use std::collections::HashMap;

use emir::Metric;
use emir::Qrels;
use emir::Run;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let qrels_map = HashMap::from([(
        "q_1".to_string(),
        HashMap::from([
            ("d_1".to_string(), 1),
            ("d_2".to_string(), 0),
            ("d_3".to_string(), 2),
        ]),
    )]);

    let run_map = HashMap::from([(
        "q_1".to_string(),
        HashMap::from([
            ("d_1".to_string(), 0.5.into()),
            ("d_2".to_string(), 0.4.into()),
            ("d_3".to_string(), 0.3.into()),
            ("d_4".to_string(), 0.2.into()),
        ]),
    )]);

    let qrels = Qrels::from_map(qrels_map);
    let run = Run::from_map(run_map);
    let metrics = vec![Metric::Precision(1, 1)];

    let evaluated = emir::evaluate(&qrels, &run, metrics)?;
    for (metric, score) in evaluated.mean_scores.iter() {
        println!("{}:\t{:.4}", metric, score);
    }

    Ok(())
}
