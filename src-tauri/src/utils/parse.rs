pub fn parse_position(args: &[String]) -> (Option<f64>, Option<f64>) {
    let mut x_pos = None;
    let mut y_pos = None;
    for i in 0..args.len() {
        if args[i] == "-x" && i + 1 < args.len() {
            if let Ok(x) = args[i + 1].parse::<f64>() {
                x_pos = Some(x);
            }
        } else if args[i] == "-y" && i + 1 < args.len() {
            if let Ok(y) = args[i + 1].parse::<f64>() {
                y_pos = Some(y);
            }
        }
    }
    (x_pos, y_pos)
}

pub fn parse_close_delay(args: &[String]) -> Option<u64> {
    for i in 0..args.len() {
        if args[i] == "-t" && i + 1 < args.len() {
            if let Ok(t) = args[i + 1].parse::<u64>() {
                return Some(t);
            }
        }
    }
    None
}
