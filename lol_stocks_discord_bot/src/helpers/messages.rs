use std::fmt::Display;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;
use crate::helpers::portfolio_view::PlayersHoldings;

pub async fn send_message<T: Display, S: Display + Into<String>>(
    ctx: &Context,
    msg: &Message,
    title: T,
    description: Option<T>,
    fields: Option<Vec<(S, S, bool)>>
) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.colour(0x4287f5);
            e.title(title);

            if let Some(desc) = description {
                e.description(desc);
            }

            if let Some(f) = fields {
                e.fields(f);
            }

            e
        })
    }).await?;
    Ok(())
}


pub async fn send_error_message<T: Display>(ctx: &Context, msg: &Message, error: T) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e
                .colour(0xff5733)
                .title(format!("❌ An error occurred: {}", error))
        })
    }).await?;
    Ok(())
}

pub async fn send_image_message(ctx: &Context, msg: &Message, image_location: String) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.add_file(&image_location[..])
    }).await?;
    Ok(())
}

pub async fn send_image_as_attachment<T: Display, S: Display + Into<String>>(
    ctx: &Context,
    msg: &Message,
    title: T,
    description: Option<T>,
    fields: Option<Vec<(S, S, bool)>>,
    attachment: &str,
) -> CommandResult {
    let file_location = format!("./images/{}", &attachment);

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.colour(0x4287f5);
            e.title(title);

            if let Some(desc) = description {
                e.description(desc);
            }

            if let Some(f) = fields {
                e.fields(f);
            }

            e.thumbnail(format!("attachment://{}", attachment))
        });

        m.add_file(&file_location[..]);
        m
    }).await?;
    Ok(())
}

pub async fn send_portfolio(
    ctx: &Context,
    msg: &Message,
    holdings: PlayersHoldings,
) -> CommandResult {
    let mut response = "".to_string();
    response.push_str(&format!("**Balance:** {}\n", holdings.balance));

    response.push_str("──────────\n");

    for holding in holdings.holdings {
        let mut body: String = "".to_string();
        body.push_str(&format!("**{}:** {} ({})\n", holding.team.name, holding.amount, holding.value));
        response.push_str(&body);
    };

    response.push_str("──────────\n");
    response.push_str(&format!("**Total:** {}", holdings.total_value));

    send_message::<String, String>(
    ctx,
    msg,
    format!("{}'s Portfolio:", holdings.user),
    Some(response),
    None
    ).await?;
    Ok(())
}