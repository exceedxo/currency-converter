use tokio::task::spawn;
use user_input::command_loop;

mod api;
mod user_input;
mod commands;
mod config;

#[tokio::main]
async fn main() {
    println!("Welcome to the Currency Converter!");
    println!("This program uses www.exchangerate-api.com to get the latest exchange rates.");
    println!("Type help for a list of commands.");
    spawn(command_loop()).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::api;

    #[tokio::test]
    async fn test_get_all_exchange_rates_correct() {
        let response = api::get_all_exchange_rates("USD").await;
        match response {
            Ok(response) => {
                assert_eq!(response.base_code, "USD");
            }
            Err(e) => {
                panic!("Error getting exchange rates: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_all_exchange_rates_incorrect() {
        match api::get_all_exchange_rates("UST").await {
            Ok(_) => {
                panic!("Expected an error, but got a response.");
            }
            Err(e) => {
                assert_eq!(e.to_string(), "Unsupported currency.");
            }
        }
    }

    #[tokio::test]
    async fn test_get_rate_both_correct() {
        match api::get_exchange_rate("USD", "EUR").await {
            Ok(response) => {
                assert_eq!(response.base_code, "USD");
                assert_eq!(response.target_code, "EUR");
                assert_eq!(response.conversion_rate.is_nan(), false);
            }
            Err(e) => {
                panic!("Error getting exchange rate: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_rate_left_wrong() {
        match api::get_exchange_rate("UST", "EUR").await {
            Ok(_) => {
                panic!("Expected an error, but got a response.");
            }
            Err(e) => {
                assert_eq!(e.to_string(), "Unsupported currency.");
            }
        }
    }

    #[tokio::test]
    async fn test_get_rate_right_wrong() {
        match api::get_exchange_rate("USD", "EUX").await {
            Ok(_) => {
                panic!("Expected an error, but got a response.");
            }
            Err(e) => {
                assert_eq!(e.to_string(), "Unsupported currency.");
            }
        }
    }

    #[tokio::test]
    async fn test_get_rate_both_wrong() {
        match api::get_exchange_rate("UST", "EUX").await {
            Ok(_) => {
                panic!("Expected an error, but got a response.");
            }
            Err(e) => {
                assert_eq!(e.to_string(), "Unsupported currency.");
            }
        }
    }

    #[tokio::test]
    async fn test_convert_all_correct() {
        match api::convert("USD", "EUR", 100.into()).await {
            Ok(response) => {
                assert_eq!(response.base_code, "USD");
                assert_eq!(response.target_code, "EUR");
                assert_eq!(response.conversion_result.is_nan(), false);
            }
            Err(e) => {
                panic!("Error converting currency: {}", e);
            }
        }

        match api::convert("USD", "EUR", 100.0.into()).await {
            Ok(response) => {
                assert_eq!(response.base_code, "USD");
                assert_eq!(response.target_code, "EUR");
                assert_eq!(response.conversion_result.is_nan(), false);
            }
            Err(e) => {
                panic!("Error converting currency: {}", e);
            }
        }

        match api::convert("USD", "EUR", 4231.1296.into()).await {
            Ok(response) => {
                assert_eq!(response.base_code, "USD");
                assert_eq!(response.target_code, "EUR");
                assert_eq!(response.conversion_result.is_nan(), false);
            }
            Err(e) => {
                panic!("Error converting currency: {}", e);
            }
        }
    }

    
    #[tokio::test]
    async fn test_convert_wrong_left_currency() {
        match api::convert("UST", "EUR", 100.into()).await {
            Ok(_) => {
                panic!("Expected an error, but got a response.");
            }
            Err(e) => {
                assert_eq!(e.to_string(), "Unsupported currency.");
            }
        }
    }

    #[tokio::test]
    async fn test_convert_wrong_right_currency() {
        match api::convert("USD", "EUX", 100.into()).await {
            Ok(_) => {
                panic!("Expected an error, but got a response.");
            }
            Err(e) => {
                assert_eq!(e.to_string(), "Unsupported currency.");
            }
        }
    }

    #[tokio::test]
    async fn test_convert_small_amount() {
        match api::convert("USD", "EUR", 0.00025.into()).await {
            Ok(response) => {
                assert_eq!(response.base_code, "USD");
                assert_eq!(response.target_code, "EUR");
                assert_eq!(response.conversion_result.is_nan(), false);
            }
            Err(e) => {
                panic!("Error converting currency: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_convert_big_amount() {
        match api::convert("USD", "EUR", 326235234543.32452362323.into()).await {
            Ok(response) => {
                assert_eq!(response.base_code, "USD");
                assert_eq!(response.target_code, "EUR");
                assert_eq!(response.conversion_result.is_nan(), false);
            }
            Err(e) => {
                panic!("Error converting currency: {}", e);
            }
        }
    }
}