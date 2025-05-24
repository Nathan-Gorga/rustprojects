pub fn calculate_mean(data: &[f64]) -> Option<f64> {
    if data.is_empty(){
        None
    } else {
        Some(data.iter().sum::<f64>() / data.len() as f64)
    }
}

pub fn calculate_median(data: &mut [f64]) -> Option<f64> {
    let len = data.len();
    if len == 0 {
        return None;
    }

    data.sort_by(|a,b| a.partial_cmp(b).unwrap());

    Some(if len % 2 == 0 {
        (data[len / 2 - 1] + data[len / 2]) / 2.0
    } else {
        data[len / 2]
    })
}