use serenity::framework::standard::Args;

pub fn parse_int_and_string(mut args: Args) -> Result<(i32, String), String> {
    let amount: i32;
    let name: String;

    if let Ok(a) = args.single::<i32>() {
        amount = a
    } else {
        return Err("Please enter a valid number".to_string())
    }

    if let Ok(s) = args.single::<String>() {
        name = s
    } else {
        return Err("Please enter a valid team name".to_string())
    }

    Ok((amount, name))
}

pub fn parse_string(mut args: Args) -> Result<String, String> {
    let name: String;

    if let Ok(s) = args.single::<String>() {
        name = s
    } else {
        return Err("Please enter a valid name".to_string())
    }

    Ok(name)
}