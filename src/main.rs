mod run_generation;

use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ConwayConfig {
    id: String,
    generationCount: i32,
    size: i32,
    world: Vec<Vec<i32>>
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
struct ConwayPostBody {
    id: String,
    generationCount: i32,
    generations: Vec<Vec<Vec<i32>>>
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://game-of-life-service-ai3nmiz7aa-uc.a.run.app/world")?.text()?;
    let config: ConwayConfig = serde_json::from_str(&resp)?;

    let mut generations: Vec<Vec<Vec<i32>>> = vec![config.world];

    for gen in 0..config.generationCount-1{
        generations.push(run_generation::run(&generations[gen as usize], config.size));
    }

    let client = reqwest::blocking::Client::new();
    let body = ConwayPostBody {
        id: config.id,
        generationCount: config.generationCount,
        generations: generations
    };

    client.post("https://game-of-life-service-ai3nmiz7aa-uc.a.run.app/results").json(&body).send()?;
    Ok(())
}