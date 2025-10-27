use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //fn main() -> Result<(), Box<dyn Error>> {
    let now = std::time::SystemTime::now();
    let a1 = env::args().nth(1).unwrap_or("?".to_string());
    match a1.as_str() {
        "3W" => {
            bbsg_a05::stg3::stage_03()?;
            bbsg_a05::p09::web1().await?;
        }
        "23W" => {
            bbsg_a05::stg2::stage_02()?;
            bbsg_a05::stg3::stage_03()?;
            bbsg_a05::p09::web1().await?;
        }
        "stage_13" => {
            bbsg_a05::stg1::stage_01()?;
            bbsg_a05::stg2::stage_02()?;
            bbsg_a05::stg3::stage_03()?;
        }
        "stage_23" => {
            bbsg_a05::stg2::stage_02()?;
            bbsg_a05::stg3::stage_03()?;
        }
        "stage_03" => bbsg_a05::stg3::stage_03()?,
        "stage_02" => bbsg_a05::stg2::stage_02()?,
        "stage_01" => bbsg_a05::stg1::stage_01()?,
        "web1" => bbsg_a05::p09::web1().await?,
        n => {
            println!("'{}' NG command", n);
        }
    }
    let se = now.elapsed().unwrap().as_secs();
    let mi = se / 60;
    println!("time {se} sec = {mi} min");
    Ok(())
}
