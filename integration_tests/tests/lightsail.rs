#![cfg(feature = "lightsail")]

extern crate rusoto_core;
extern crate rusoto_lightsail;

use rusoto_core::Region;
use rusoto_lightsail::{GetDomainsRequest, Lightsail, LightsailClient};

#[test]
fn should_list_domains() {
    let client = LightsailClient::new(Region::UsEast1);
    let request = GetDomainsRequest::default();

    let result = client.get_domains(request).sync().unwrap();
    println!("{:#?}", result);
}
