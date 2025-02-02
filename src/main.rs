use std::env;
use std::process;


fn convert_to_format_one_unit(total_duration: u64, unit: u64, unit_name: &str, string_to_complete_is_empty: bool) -> (String, u64)
{
    let mut duration_formated: String = String::new();
    let mut remaining_duration = total_duration;

    if total_duration >= unit
    {
        let unit_value = total_duration / unit;
        remaining_duration = total_duration % unit;
        let mut extra_mark= "";

        if string_to_complete_is_empty
        {
            // Do Nothing
        }
        else if remaining_duration == 0
        {
            extra_mark = " and ";
        }
        else
        {
            extra_mark = ", ";
        }

        duration_formated = format!("{}{} {}{}", extra_mark, unit_value, unit_name, if unit_value > 1 {"s"} else {""});
    }


    return (duration_formated, remaining_duration);
}

fn format_duration(total_duration : u64) -> String {
    let mut duration_formated: String = String::new();

    let mut total_duration_remaining = total_duration;

    let duration_map: Vec<(u64, &str)> = Vec::from([
        (31536000, "year"),
        (2592000, "month"),
        (86400, "day"),
        (3600, "hour"),
        (60, "minute"),
        (1, "second")
    ]);

    if total_duration == 0
    {
        return "now".to_string();
    }

    for (key, value) in duration_map.into_iter() {
        let single_duration_formated: String;

        (single_duration_formated, total_duration_remaining) = convert_to_format_one_unit(total_duration_remaining, key, value, duration_formated.is_empty());
        duration_formated.push_str(&single_duration_formated)
    }

    return duration_formated
}

fn main() -> Result<(), std::io::Error>{
    let args: Vec<String> = env::args().collect();

    let num = args[1].parse::<u64>();
    match num {
        Ok(val) => {
            println!("Duration: {}", format_duration(val));
        }
        ,
        Err(why) => {
            println!("Doesn't look like a unsigned number ({})", why);
            process::exit(1);
        },
    }

    Ok(())
}
