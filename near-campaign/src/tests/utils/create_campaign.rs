use crate::*;

pub fn create_campaign() -> Campaign {
  Campaign::new(U128::from(1_000_000_000_000_000_000_000_000), None, None, None)
}

pub fn create_campaign_with_social_network(social_network: SocialNetwork) -> Campaign {
  Campaign::new(U128::from(1_000_000_000_000_000_000_000_000), None, None, Some(social_network))
}
