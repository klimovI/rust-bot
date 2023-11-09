use async_openai::types::{ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs};
use async_openai::Client;
use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::CommandDataOptionValue;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub const NAME: &str = "ask";

pub async fn run(options: &[CommandDataOption]) -> String {
  let option = options
    .get(0)
    .expect("Expected user query")
    .resolved
    .as_ref()
    .expect("Expected string");

  let client = Client::new();

  let request = CreateChatCompletionRequestArgs::default()
    .model("gpt-3.5-turbo")
    .messages([ChatCompletionRequestUserMessageArgs::default()
      .content("Hi")
      .build()
      .expect("qwe")
      .into()])
    .build()
    .expect("qwe");

  println!("{}", serde_json::to_string(&request).unwrap());

  let response = client.chat().create(request).await.expect("asd");

  println!("\nResponse:\n");
  for choice in response.choices {
    println!(
      "{}: Role: {}  Content: {:?}",
      choice.index, choice.message.role, choice.message.content
    );
  }

  if let CommandDataOptionValue::String(query) = option {
    query.to_string()
  } else {
    "provide query".to_string()
  }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
  command
    .name(NAME)
    .description("Ask ChatGPT")
    .create_option(|option| {
      option
        .name("query")
        .description("ChatGPT query")
        .kind(CommandOptionType::String)
        .required(true)
    })
}
