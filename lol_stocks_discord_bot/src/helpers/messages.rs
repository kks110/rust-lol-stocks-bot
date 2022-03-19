use std::fmt::Display;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

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

            if description.is_some() {
                e.description(description.unwrap());
            }

            if fields.is_some() {
                e.fields(fields.unwrap());
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
                .title(format!("❌ An error as occurred: {}", error))
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

            if description.is_some() {
                e.description(description.unwrap());
            }

            if fields.is_some() {
                e.fields(fields.unwrap());
            }
            e.thumbnail(format!("attachment://{}", attachment))
        });

        m.add_file(&file_location[..]);
        m
    }).await?;
    Ok(())
}
