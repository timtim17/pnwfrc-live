#[cfg(feature = "lambda")] use lambda_web::{launch_rocket_on_lambda, LambdaError};
#[cfg(not(feature = "lambda"))] use rocket::{Build, Rocket};

use pnwfrc_live::build_rocket;

#[cfg(not(feature = "lambda"))]
#[rocket::launch]
fn rocket() -> Rocket<Build> {
    build_rocket()
}

#[cfg(feature = "lambda")]
#[rocket::main]
async fn main() -> Result<(), LambdaError> {
    launch_rocket_on_lambda(build_rocket()).await?;
    Ok(())
}
