use llm_chain::{prompt, Parameters};
use llm_chain_openai::chatgpt::Executor;

// Declare an async main function
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new ChatGPT executor
    let exec = Executor::default();
    // Create our prompt...
    let res = prompt!(
        "You are a robot assistant for making personalized greetings",
        "Make a personalized greeting for Joe"
    )
    .run(&Parameters::new(), &exec) // ...and run it
    .await?;
    println!("{}", res);
    Ok(())
}
