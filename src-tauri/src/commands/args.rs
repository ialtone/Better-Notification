use std::env;

#[tauri::command]
pub fn get_str_arg() -> String {
    let args: Vec<String> = env::args().collect();
    parse_str_arg(&args)
}

#[tauri::command]
pub fn get_route_arg() -> String {
    let args: Vec<String> = env::args().collect();
    parse_route_arg(&args)
}

fn parse_str_arg(args: &[String]) -> String {
    let mut str_arg = "欢迎使用Better Notification!!".to_string();
    for i in 0..args.len() {
        if args[i] == "-str" && i + 1 < args.len() {
            str_arg = args[i + 1].clone();
            break;
        }
    }
    str_arg.replace("\\n", "\n")
}

fn parse_route_arg(args: &[String]) -> String {
    let mut route_arg = "/".to_string();
    for i in 0..args.len() {
        if args[i] == "-route" && i + 1 < args.len() {
            route_arg = args[i + 1].clone();
            break;
        }
    }
    route_arg
}
