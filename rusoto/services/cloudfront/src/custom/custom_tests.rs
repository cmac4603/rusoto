extern crate rusoto_mock;

use crate::generated::{CloudFront, CloudFrontClient, ListDistributionsRequest};

use rusoto_core::Region;
use self::rusoto_mock::*;

#[test]
fn should_list_distributions() {
    let body = MockResponseReader::read_response("test_resources/generated/valid", "cloudfront-list-distributions.xml");
    let mock = MockRequestDispatcher::with_status(200).with_body(&body);

    let request = ListDistributionsRequest::default();

    let client = CloudFrontClient::new_with(mock, MockCredentialsProvider, Region::UsEast1);
    let result = client.list_distributions(request).sync().unwrap();
    assert!(&result.distribution_list.is_some());
    let parsed_result = result.distribution_list.unwrap();
    assert!(&parsed_result.items.is_some());
    let items = parsed_result.items.unwrap();
    assert_eq!(items.len(), 1);
    let first_item = &items[0];
    assert_eq!(first_item.id, "EDFDVBD6EXAMPLE");
    assert_eq!(first_item.status, "Deployed");
    assert_eq!(first_item.domain_name, "d111111abcdef8.cloudfront.net");
    assert_eq!(first_item.origins.quantity, 2);
    match first_item.origins.items {
        Some(ref i) => assert_eq!(i[1].domain_name, "example.com"),
        None => panic!("Should have items"),
    }
}
