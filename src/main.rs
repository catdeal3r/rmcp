pub mod consts;
pub mod ai;

fn main() {
    //println!("{}", consts::generate_full_prompt("What are the animal cousins of the rabbit?", vec!["User: \"what are the animal cousins of the dingo?\"", "AI: \"The New Guinea Singing Dog and East Asian dogs are close cousins to the dingo\""]));

    let response = ai::get_ai_response(&"Who are the cousins to the dingo?".to_string(), &"gemma4:e2b".to_string());

    println!("Response: {}", response);
}
