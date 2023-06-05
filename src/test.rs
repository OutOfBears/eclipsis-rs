use crate::{api::EclipseApi, config::ConfigurationBuilder};

#[tokio::test]
async fn test_api() {
    let api_key = std::env::var("API_KEY").expect("API_KEY not set");

    let config = ConfigurationBuilder::new().api_key(api_key).build();
    let client = EclipseApi::new(config);

    let match_id = String::from("1114650753156518018");
    let user_match = client
        .get_match_data(match_id.clone())
        .await
        .expect("Failed to get match data");

    assert_eq!(user_match.id.unwrap(), match_id);
    assert_eq!(user_match.teams.unwrap()[0].team_id.unwrap(), 2066592);
}
