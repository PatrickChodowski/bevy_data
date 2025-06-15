use polars::prelude::*;

pub fn group_by_sum(
    df: &DataFrame, 
    dim: &str,
    metric: &str
) -> DataFrame {
    return df.clone().lazy().group_by([col(dim)]).agg([col(metric).sum()]).collect().ok().unwrap();
}