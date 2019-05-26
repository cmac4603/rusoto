// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
/// <p>A collection of accounts and regions.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountAggregationSource {
    /// <p>The 12-digit account ID of the account being aggregated. </p>
    #[serde(rename = "AccountIds")]
    pub account_ids: Vec<String>,
    /// <p>If true, aggregate existing AWS Config regions and future regions.</p>
    #[serde(rename = "AllAwsRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_aws_regions: Option<bool>,
    /// <p>The source regions being aggregated.</p>
    #[serde(rename = "AwsRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_regions: Option<Vec<String>>,
}

/// <p>Indicates whether an AWS Config rule is compliant based on account ID, region, compliance, and rule name.</p> <p>A rule is compliant if all of the resources that the rule evaluated comply with it. It is noncompliant if any of these resources do not comply.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AggregateComplianceByConfigRule {
    /// <p>The 12-digit account ID of the source account.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The source region from where the data is aggregated.</p>
    #[serde(rename = "AwsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    /// <p>Indicates whether an AWS resource or AWS Config rule is compliant and provides the number of contributors that affect the compliance.</p>
    #[serde(rename = "Compliance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<Compliance>,
    /// <p>The name of the AWS Config rule.</p>
    #[serde(rename = "ConfigRuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
}

/// <p>Returns the number of compliant and noncompliant rules for one or more accounts and regions in an aggregator.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AggregateComplianceCount {
    /// <p>The number of compliant and noncompliant AWS Config rules.</p>
    #[serde(rename = "ComplianceSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summary: Option<ComplianceSummary>,
    /// <p>The 12-digit account ID or region based on the GroupByKey value.</p>
    #[serde(rename = "GroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

/// <p>The details of an AWS Config evaluation for an account ID and region in an aggregator. Provides the AWS resource that was evaluated, the compliance of the resource, related time stamps, and supplementary information. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AggregateEvaluationResult {
    /// <p>The 12-digit account ID of the source account.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>Supplementary information about how the agrregate evaluation determined the compliance.</p>
    #[serde(rename = "Annotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<String>,
    /// <p>The source region from where the data is aggregated.</p>
    #[serde(rename = "AwsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    /// <p>The resource compliance status.</p> <p>For the <code>AggregationEvaluationResult</code> data type, AWS Config supports only the <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>. AWS Config does not support the <code>NOT_APPLICABLE</code> and <code>INSUFFICIENT_DATA</code> value.</p>
    #[serde(rename = "ComplianceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    /// <p>The time when the AWS Config rule evaluated the AWS resource.</p>
    #[serde(rename = "ConfigRuleInvokedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_invoked_time: Option<f64>,
    /// <p>Uniquely identifies the evaluation result.</p>
    #[serde(rename = "EvaluationResultIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_result_identifier: Option<EvaluationResultIdentifier>,
    /// <p>The time when AWS Config recorded the aggregate evaluation result.</p>
    #[serde(rename = "ResultRecordedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_recorded_time: Option<f64>,
}

/// <p>The details that identify a resource that is collected by AWS Config aggregator, including the resource type, ID, (if available) the custom resource name, the source account, and source region.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AggregateResourceIdentifier {
    /// <p>The ID of the AWS resource.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The name of the AWS resource.</p>
    #[serde(rename = "ResourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// <p>The type of the AWS resource.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
    /// <p>The 12-digit account ID of the source account.</p>
    #[serde(rename = "SourceAccountId")]
    pub source_account_id: String,
    /// <p>The source region where data is aggregated.</p>
    #[serde(rename = "SourceRegion")]
    pub source_region: String,
}

/// <p>The current sync status between the source and the aggregator account.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AggregatedSourceStatus {
    /// <p>The region authorized to collect aggregated data.</p>
    #[serde(rename = "AwsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    /// <p>The error code that AWS Config returned when the source account aggregation last failed.</p>
    #[serde(rename = "LastErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<String>,
    /// <p>The message indicating that the source account aggregation failed due to an error.</p>
    #[serde(rename = "LastErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    /// <p><p>Filters the last updated status type.</p> <ul> <li> <p>Valid value FAILED indicates errors while moving data.</p> </li> <li> <p>Valid value SUCCEEDED indicates the data was successfully moved.</p> </li> <li> <p>Valid value OUTDATED indicates the data is not the most recent.</p> </li> </ul></p>
    #[serde(rename = "LastUpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_status: Option<String>,
    /// <p>The time of the last update.</p>
    #[serde(rename = "LastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The source account ID or an organization.</p>
    #[serde(rename = "SourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// <p>The source account or an organization.</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

/// <p>An object that represents the authorizations granted to aggregator accounts and regions.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AggregationAuthorization {
    /// <p>The Amazon Resource Name (ARN) of the aggregation object.</p>
    #[serde(rename = "AggregationAuthorizationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_authorization_arn: Option<String>,
    /// <p>The 12-digit account ID of the account authorized to aggregate data.</p>
    #[serde(rename = "AuthorizedAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_account_id: Option<String>,
    /// <p>The region authorized to collect aggregated data.</p>
    #[serde(rename = "AuthorizedAwsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_aws_region: Option<String>,
    /// <p>The time stamp when the aggregation authorization was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
}

/// <p>The detailed configuration of a specified resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BaseConfigurationItem {
    /// <p>The 12-digit AWS account ID associated with the resource.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The Availability Zone associated with the resource.</p>
    #[serde(rename = "availabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The region where the resource resides.</p>
    #[serde(rename = "awsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    /// <p>The description of the resource configuration.</p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    /// <p>The time when the configuration recording was initiated.</p>
    #[serde(rename = "configurationItemCaptureTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_item_capture_time: Option<f64>,
    /// <p>The configuration item status.</p>
    #[serde(rename = "configurationItemStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_item_status: Option<String>,
    /// <p>An identifier that indicates the ordering of the configuration items of a resource.</p>
    #[serde(rename = "configurationStateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_state_id: Option<String>,
    /// <p>The time stamp when the resource was created.</p>
    #[serde(rename = "resourceCreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_creation_time: Option<f64>,
    /// <p>The ID of the resource (for example., sg-xxxxxx).</p>
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The custom name of the resource, if available.</p>
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// <p>The type of AWS resource.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>Configuration attributes that AWS Config returns for certain resource types to supplement the information returned for the configuration parameter.</p>
    #[serde(rename = "supplementaryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplementary_configuration: Option<::std::collections::HashMap<String, String>>,
    /// <p>The version number of the resource configuration.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetAggregateResourceConfigRequest {
    /// <p>The name of the configuration aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: String,
    /// <p>A list of aggregate ResourceIdentifiers objects. </p>
    #[serde(rename = "ResourceIdentifiers")]
    pub resource_identifiers: Vec<AggregateResourceIdentifier>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BatchGetAggregateResourceConfigResponse {
    /// <p>A list that contains the current configuration of one or more resources.</p>
    #[serde(rename = "BaseConfigurationItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_configuration_items: Option<Vec<BaseConfigurationItem>>,
    /// <p>A list of resource identifiers that were not processed with current scope. The list is empty if all the resources are processed.</p>
    #[serde(rename = "UnprocessedResourceIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_resource_identifiers: Option<Vec<AggregateResourceIdentifier>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchGetResourceConfigRequest {
    /// <p>A list of resource keys to be processed with the current request. Each element in the list consists of the resource type and resource ID.</p>
    #[serde(rename = "resourceKeys")]
    pub resource_keys: Vec<ResourceKey>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BatchGetResourceConfigResponse {
    /// <p>A list that contains the current configuration of one or more resources.</p>
    #[serde(rename = "baseConfigurationItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_configuration_items: Option<Vec<BaseConfigurationItem>>,
    /// <p>A list of resource keys that were not processed with the current response. The unprocessesResourceKeys value is in the same form as ResourceKeys, so the value can be directly provided to a subsequent BatchGetResourceConfig operation. If there are no unprocessed resource keys, the response contains an empty unprocessedResourceKeys list. </p>
    #[serde(rename = "unprocessedResourceKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unprocessed_resource_keys: Option<Vec<ResourceKey>>,
}

/// <p>Indicates whether an AWS resource or AWS Config rule is compliant and provides the number of contributors that affect the compliance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Compliance {
    /// <p>The number of AWS resources or AWS Config rules that cause a result of <code>NON_COMPLIANT</code>, up to a maximum number.</p>
    #[serde(rename = "ComplianceContributorCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_contributor_count: Option<ComplianceContributorCount>,
    /// <p>Indicates whether an AWS resource or AWS Config rule is compliant.</p> <p>A resource is compliant if it complies with all of the AWS Config rules that evaluate it. A resource is noncompliant if it does not comply with one or more of these rules.</p> <p>A rule is compliant if all of the resources that the rule evaluates comply with it. A rule is noncompliant if any of these resources do not comply.</p> <p>AWS Config returns the <code>INSUFFICIENT_DATA</code> value when no evaluation results are available for the AWS resource or AWS Config rule.</p> <p>For the <code>Compliance</code> data type, AWS Config supports only <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>INSUFFICIENT_DATA</code> values. AWS Config does not support the <code>NOT_APPLICABLE</code> value for the <code>Compliance</code> data type.</p>
    #[serde(rename = "ComplianceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
}

/// <p>Indicates whether an AWS Config rule is compliant. A rule is compliant if all of the resources that the rule evaluated comply with it. A rule is noncompliant if any of these resources do not comply.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ComplianceByConfigRule {
    /// <p>Indicates whether the AWS Config rule is compliant.</p>
    #[serde(rename = "Compliance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<Compliance>,
    /// <p>The name of the AWS Config rule.</p>
    #[serde(rename = "ConfigRuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
}

/// <p>Indicates whether an AWS resource that is evaluated according to one or more AWS Config rules is compliant. A resource is compliant if it complies with all of the rules that evaluate it. A resource is noncompliant if it does not comply with one or more of these rules.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ComplianceByResource {
    /// <p>Indicates whether the AWS resource complies with all of the AWS Config rules that evaluated it.</p>
    #[serde(rename = "Compliance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<Compliance>,
    /// <p>The ID of the AWS resource that was evaluated.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The type of the AWS resource that was evaluated.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>The number of AWS resources or AWS Config rules responsible for the current compliance of the item, up to a maximum number.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ComplianceContributorCount {
    /// <p>Indicates whether the maximum count is reached.</p>
    #[serde(rename = "CapExceeded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cap_exceeded: Option<bool>,
    /// <p>The number of AWS resources or AWS Config rules responsible for the current compliance of the item.</p>
    #[serde(rename = "CappedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capped_count: Option<i64>,
}

/// <p>The number of AWS Config rules or AWS resources that are compliant and noncompliant.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ComplianceSummary {
    /// <p>The time that AWS Config created the compliance summary.</p>
    #[serde(rename = "ComplianceSummaryTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summary_timestamp: Option<f64>,
    /// <p>The number of AWS Config rules or AWS resources that are compliant, up to a maximum of 25 for rules and 100 for resources.</p>
    #[serde(rename = "CompliantResourceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliant_resource_count: Option<ComplianceContributorCount>,
    /// <p>The number of AWS Config rules or AWS resources that are noncompliant, up to a maximum of 25 for rules and 100 for resources.</p>
    #[serde(rename = "NonCompliantResourceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub non_compliant_resource_count: Option<ComplianceContributorCount>,
}

/// <p>The number of AWS resources of a specific type that are compliant or noncompliant, up to a maximum of 100 for each.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ComplianceSummaryByResourceType {
    /// <p>The number of AWS resources that are compliant or noncompliant, up to a maximum of 100 for each.</p>
    #[serde(rename = "ComplianceSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summary: Option<ComplianceSummary>,
    /// <p>The type of AWS resource.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Provides status of the delivery of the snapshot or the configuration history to the specified Amazon S3 bucket. Also provides the status of notifications about the Amazon S3 delivery to the specified Amazon SNS topic.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ConfigExportDeliveryInfo {
    /// <p>The time of the last attempted delivery.</p>
    #[serde(rename = "lastAttemptTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_attempt_time: Option<f64>,
    /// <p>The error code from the last attempted delivery.</p>
    #[serde(rename = "lastErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<String>,
    /// <p>The error message from the last attempted delivery.</p>
    #[serde(rename = "lastErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    /// <p>Status of the last attempted delivery.</p>
    #[serde(rename = "lastStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    /// <p>The time of the last successful delivery.</p>
    #[serde(rename = "lastSuccessfulTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_time: Option<f64>,
    /// <p>The time that the next delivery occurs.</p>
    #[serde(rename = "nextDeliveryTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_delivery_time: Option<f64>,
}

/// <p>An AWS Config rule represents an AWS Lambda function that you create for a custom rule or a predefined function for an AWS managed rule. The function evaluates configuration items to assess whether your AWS resources comply with your desired configurations. This function can run when AWS Config detects a configuration change to an AWS resource and at a periodic frequency that you choose (for example, every 24 hours).</p> <note> <p>You can use the AWS CLI and AWS SDKs if you want to create a rule that triggers evaluations for your resources when AWS Config delivers the configuration snapshot. For more information, see <a>ConfigSnapshotDeliveryProperties</a>.</p> </note> <p>For more information about developing and using AWS Config rules, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config.html">Evaluating AWS Resource Configurations with AWS Config</a> in the <i>AWS Config Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigRule {
    /// <p>The Amazon Resource Name (ARN) of the AWS Config rule.</p>
    #[serde(rename = "ConfigRuleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_arn: Option<String>,
    /// <p>The ID of the AWS Config rule.</p>
    #[serde(rename = "ConfigRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_id: Option<String>,
    /// <p>The name that you assign to the AWS Config rule. The name is required if you are adding a new rule.</p>
    #[serde(rename = "ConfigRuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
    /// <p>Indicates whether the AWS Config rule is active or is currently being deleted by AWS Config. It can also indicate the evaluation status for the AWS Config rule.</p> <p>AWS Config sets the state of the rule to <code>EVALUATING</code> temporarily after you use the <code>StartConfigRulesEvaluation</code> request to evaluate your resources against the AWS Config rule.</p> <p>AWS Config sets the state of the rule to <code>DELETING_RESULTS</code> temporarily after you use the <code>DeleteEvaluationResults</code> request to delete the current evaluation results for the AWS Config rule.</p> <p>AWS Config temporarily sets the state of a rule to <code>DELETING</code> after you use the <code>DeleteConfigRule</code> request to delete the rule. After AWS Config deletes the rule, the rule and all of its evaluations are erased and are no longer available.</p>
    #[serde(rename = "ConfigRuleState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_state: Option<String>,
    /// <p><p>Service principal name of the service that created the rule.</p> <note> <p>The field is populated only if the service linked rule is created by a service. The field is empty if you create your own rule.</p> </note></p>
    #[serde(rename = "CreatedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// <p>The description that you provide for the AWS Config rule.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A string, in JSON format, that is passed to the AWS Config rule Lambda function.</p>
    #[serde(rename = "InputParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_parameters: Option<String>,
    /// <p><p>The maximum frequency with which AWS Config runs evaluations for a rule. You can specify a value for <code>MaximumExecutionFrequency</code> when:</p> <ul> <li> <p>You are using an AWS managed rule that is triggered at a periodic frequency.</p> </li> <li> <p>Your custom rule is triggered when AWS Config delivers the configuration snapshot. For more information, see <a>ConfigSnapshotDeliveryProperties</a>.</p> </li> </ul> <note> <p>By default, rules with a periodic trigger are evaluated every 24 hours. To change the frequency, specify a valid value for the <code>MaximumExecutionFrequency</code> parameter.</p> </note></p>
    #[serde(rename = "MaximumExecutionFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_execution_frequency: Option<String>,
    /// <p>Defines which resources can trigger an evaluation for the rule. The scope can include one or more resource types, a combination of one resource type and one resource ID, or a combination of a tag key and value. Specify a scope to constrain the resources that can trigger an evaluation for the rule. If you do not specify a scope, evaluations are triggered when any resource in the recording group changes.</p>
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    /// <p>Provides the rule owner (AWS or customer), the rule identifier, and the notifications that cause the function to evaluate your AWS resources.</p>
    #[serde(rename = "Source")]
    pub source: Source,
}

/// <p>Filters the compliance results based on account ID, region, compliance type, and rule name.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ConfigRuleComplianceFilters {
    /// <p>The 12-digit account ID of the source account. </p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The source region where the data is aggregated. </p>
    #[serde(rename = "AwsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    /// <p>The rule compliance status.</p> <p>For the <code>ConfigRuleComplianceFilters</code> data type, AWS Config supports only <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>. AWS Config does not support the <code>NOT_APPLICABLE</code> and the <code>INSUFFICIENT_DATA</code> values.</p>
    #[serde(rename = "ComplianceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    /// <p>The name of the AWS Config rule.</p>
    #[serde(rename = "ConfigRuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
}

/// <p>Filters the results based on the account IDs and regions.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ConfigRuleComplianceSummaryFilters {
    /// <p>The 12-digit account ID of the source account.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The source region where the data is aggregated.</p>
    #[serde(rename = "AwsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
}

/// <p>Status information for your AWS managed Config rules. The status includes information such as the last time the rule ran, the last time it failed, and the related error for the last failure.</p> <p>This action does not return status information about custom AWS Config rules.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ConfigRuleEvaluationStatus {
    /// <p>The Amazon Resource Name (ARN) of the AWS Config rule.</p>
    #[serde(rename = "ConfigRuleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_arn: Option<String>,
    /// <p>The ID of the AWS Config rule.</p>
    #[serde(rename = "ConfigRuleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_id: Option<String>,
    /// <p>The name of the AWS Config rule.</p>
    #[serde(rename = "ConfigRuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
    /// <p>The time that you first activated the AWS Config rule.</p>
    #[serde(rename = "FirstActivatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_activated_time: Option<f64>,
    /// <p><p>Indicates whether AWS Config has evaluated your resources against the rule at least once.</p> <ul> <li> <p> <code>true</code> - AWS Config has evaluated your AWS resources against the rule at least once.</p> </li> <li> <p> <code>false</code> - AWS Config has not once finished evaluating your AWS resources against the rule.</p> </li> </ul></p>
    #[serde(rename = "FirstEvaluationStarted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_evaluation_started: Option<bool>,
    /// <p>The error code that AWS Config returned when the rule last failed.</p>
    #[serde(rename = "LastErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<String>,
    /// <p>The error message that AWS Config returned when the rule last failed.</p>
    #[serde(rename = "LastErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    /// <p>The time that AWS Config last failed to evaluate your AWS resources against the rule.</p>
    #[serde(rename = "LastFailedEvaluationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failed_evaluation_time: Option<f64>,
    /// <p>The time that AWS Config last failed to invoke the AWS Config rule to evaluate your AWS resources.</p>
    #[serde(rename = "LastFailedInvocationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failed_invocation_time: Option<f64>,
    /// <p>The time that AWS Config last successfully evaluated your AWS resources against the rule.</p>
    #[serde(rename = "LastSuccessfulEvaluationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_evaluation_time: Option<f64>,
    /// <p>The time that AWS Config last successfully invoked the AWS Config rule to evaluate your AWS resources.</p>
    #[serde(rename = "LastSuccessfulInvocationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_invocation_time: Option<f64>,
}

/// <p>Provides options for how often AWS Config delivers configuration snapshots to the Amazon S3 bucket in your delivery channel.</p> <p>The frequency for a rule that triggers evaluations for your resources when AWS Config delivers the configuration snapshot is set by one of two values, depending on which is less frequent:</p> <ul> <li> <p>The value for the <code>deliveryFrequency</code> parameter within the delivery channel configuration, which sets how often AWS Config delivers configuration snapshots. This value also sets how often AWS Config invokes evaluations for AWS Config rules.</p> </li> <li> <p>The value for the <code>MaximumExecutionFrequency</code> parameter, which sets the maximum frequency with which AWS Config invokes evaluations for the rule. For more information, see <a>ConfigRule</a>.</p> </li> </ul> <p>If the <code>deliveryFrequency</code> value is less frequent than the <code>MaximumExecutionFrequency</code> value for a rule, AWS Config invokes the rule only as often as the <code>deliveryFrequency</code> value.</p> <ol> <li> <p>For example, you want your rule to run evaluations when AWS Config delivers the configuration snapshot.</p> </li> <li> <p>You specify the <code>MaximumExecutionFrequency</code> value for <code>Six_Hours</code>. </p> </li> <li> <p>You then specify the delivery channel <code>deliveryFrequency</code> value for <code>TwentyFour_Hours</code>.</p> </li> <li> <p>Because the value for <code>deliveryFrequency</code> is less frequent than <code>MaximumExecutionFrequency</code>, AWS Config invokes evaluations for the rule every 24 hours. </p> </li> </ol> <p>You should set the <code>MaximumExecutionFrequency</code> value to be at least as frequent as the <code>deliveryFrequency</code> value. You can view the <code>deliveryFrequency</code> value by using the <code>DescribeDeliveryChannnels</code> action.</p> <p>To update the <code>deliveryFrequency</code> with which AWS Config delivers your configuration snapshots, use the <code>PutDeliveryChannel</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigSnapshotDeliveryProperties {
    /// <p>The frequency with which AWS Config delivers configuration snapshots.</p>
    #[serde(rename = "deliveryFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_frequency: Option<String>,
}

/// <p>A list that contains the status of the delivery of the configuration stream notification to the Amazon SNS topic.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ConfigStreamDeliveryInfo {
    /// <p>The error code from the last attempted delivery.</p>
    #[serde(rename = "lastErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<String>,
    /// <p>The error message from the last attempted delivery.</p>
    #[serde(rename = "lastErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    /// <p>Status of the last attempted delivery.</p> <p> <b>Note</b> Providing an SNS topic on a <a href="https://docs.aws.amazon.com/config/latest/APIReference/API_DeliveryChannel.html">DeliveryChannel</a> for AWS Config is optional. If the SNS delivery is turned off, the last status will be <b>Not_Applicable</b>.</p>
    #[serde(rename = "lastStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    /// <p>The time from the last status change.</p>
    #[serde(rename = "lastStatusChangeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_change_time: Option<f64>,
}

/// <p>The details about the configuration aggregator, including information about source accounts, regions, and metadata of the aggregator. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ConfigurationAggregator {
    /// <p>Provides a list of source accounts and regions to be aggregated.</p>
    #[serde(rename = "AccountAggregationSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_aggregation_sources: Option<Vec<AccountAggregationSource>>,
    /// <p>The Amazon Resource Name (ARN) of the aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aggregator_arn: Option<String>,
    /// <p>The name of the aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aggregator_name: Option<String>,
    /// <p>The time stamp when the configuration aggregator was created.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The time of the last update.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    /// <p>Provides an organization and list of regions to be aggregated.</p>
    #[serde(rename = "OrganizationAggregationSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_aggregation_source: Option<OrganizationAggregationSource>,
}

/// <p>A list that contains detailed configurations of a specified resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ConfigurationItem {
    /// <p>The 12-digit AWS account ID associated with the resource.</p>
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>accoun</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The Availability Zone associated with the resource.</p>
    #[serde(rename = "availabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The region where the resource resides.</p>
    #[serde(rename = "awsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    /// <p>The description of the resource configuration.</p>
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    /// <p>The time when the configuration recording was initiated.</p>
    #[serde(rename = "configurationItemCaptureTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_item_capture_time: Option<f64>,
    /// <p>Unique MD5 hash that represents the configuration item's state.</p> <p>You can use MD5 hash to compare the states of two or more configuration items that are associated with the same resource.</p>
    #[serde(rename = "configurationItemMD5Hash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_item_md5_hash: Option<String>,
    /// <p>The configuration item status.</p>
    #[serde(rename = "configurationItemStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_item_status: Option<String>,
    /// <p>An identifier that indicates the ordering of the configuration items of a resource.</p>
    #[serde(rename = "configurationStateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_state_id: Option<String>,
    /// <p>A list of CloudTrail event IDs.</p> <p>A populated field indicates that the current configuration was initiated by the events recorded in the CloudTrail log. For more information about CloudTrail, see <a href="https://docs.aws.amazon.com/awscloudtrail/latest/userguide/what_is_cloud_trail_top_level.html">What Is AWS CloudTrail</a>.</p> <p>An empty field indicates that the current configuration was not initiated by any event.</p>
    #[serde(rename = "relatedEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_events: Option<Vec<String>>,
    /// <p>A list of related AWS resources.</p>
    #[serde(rename = "relationships")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationships: Option<Vec<Relationship>>,
    /// <p>The time stamp when the resource was created.</p>
    #[serde(rename = "resourceCreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_creation_time: Option<f64>,
    /// <p>The ID of the resource (for example, <code>sg-xxxxxx</code>).</p>
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The custom name of the resource, if available.</p>
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// <p>The type of AWS resource.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>Configuration attributes that AWS Config returns for certain resource types to supplement the information returned for the <code>configuration</code> parameter.</p>
    #[serde(rename = "supplementaryConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplementary_configuration: Option<::std::collections::HashMap<String, String>>,
    /// <p>A mapping of key value tags associated with the resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The version number of the resource configuration.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>An object that represents the recording of configuration changes of an AWS resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConfigurationRecorder {
    /// <p>The name of the recorder. By default, AWS Config automatically assigns the name "default" when creating the configuration recorder. You cannot change the assigned name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Specifies the types of AWS resources for which AWS Config records configuration changes.</p>
    #[serde(rename = "recordingGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording_group: Option<RecordingGroup>,
    /// <p>Amazon Resource Name (ARN) of the IAM role used to describe the AWS resources associated with the account.</p>
    #[serde(rename = "roleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>The current status of the configuration recorder.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ConfigurationRecorderStatus {
    /// <p>The error code indicating that the recording failed.</p>
    #[serde(rename = "lastErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_code: Option<String>,
    /// <p>The message indicating that the recording failed due to an error.</p>
    #[serde(rename = "lastErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    /// <p>The time the recorder was last started.</p>
    #[serde(rename = "lastStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_start_time: Option<f64>,
    /// <p>The last (previous) status of the recorder.</p>
    #[serde(rename = "lastStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    /// <p>The time when the status was last changed.</p>
    #[serde(rename = "lastStatusChangeTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status_change_time: Option<f64>,
    /// <p>The time the recorder was last stopped.</p>
    #[serde(rename = "lastStopTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_stop_time: Option<f64>,
    /// <p>The name of the configuration recorder.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Specifies whether or not the recorder is currently recording.</p>
    #[serde(rename = "recording")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recording: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAggregationAuthorizationRequest {
    /// <p>The 12-digit account ID of the account authorized to aggregate data.</p>
    #[serde(rename = "AuthorizedAccountId")]
    pub authorized_account_id: String,
    /// <p>The region authorized to collect aggregated data.</p>
    #[serde(rename = "AuthorizedAwsRegion")]
    pub authorized_aws_region: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteConfigRuleRequest {
    /// <p>The name of the AWS Config rule that you want to delete.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteConfigurationAggregatorRequest {
    /// <p>The name of the configuration aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: String,
}

/// <p>The request object for the <code>DeleteConfigurationRecorder</code> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteConfigurationRecorderRequest {
    /// <p>The name of the configuration recorder to be deleted. You can retrieve the name of your configuration recorder by using the <code>DescribeConfigurationRecorders</code> action.</p>
    #[serde(rename = "ConfigurationRecorderName")]
    pub configuration_recorder_name: String,
}

/// <p>The input for the <a>DeleteDeliveryChannel</a> action. The action accepts the following data, in JSON format. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDeliveryChannelRequest {
    /// <p>The name of the delivery channel to delete.</p>
    #[serde(rename = "DeliveryChannelName")]
    pub delivery_channel_name: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteEvaluationResultsRequest {
    /// <p>The name of the AWS Config rule for which you want to delete the evaluation results.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
}

/// <p>The output when you delete the evaluation results for the specified AWS Config rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteEvaluationResultsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePendingAggregationRequestRequest {
    /// <p>The 12-digit account ID of the account requesting to aggregate data.</p>
    #[serde(rename = "RequesterAccountId")]
    pub requester_account_id: String,
    /// <p>The region requesting to aggregate data.</p>
    #[serde(rename = "RequesterAwsRegion")]
    pub requester_aws_region: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRemediationConfigurationRequest {
    /// <p>The name of the AWS Config rule for which you want to delete remediation configuration.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
    /// <p>The type of a resource.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteRemediationConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRetentionConfigurationRequest {
    /// <p>The name of the retention configuration to delete.</p>
    #[serde(rename = "RetentionConfigurationName")]
    pub retention_configuration_name: String,
}

/// <p>The input for the <a>DeliverConfigSnapshot</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeliverConfigSnapshotRequest {
    /// <p>The name of the delivery channel through which the snapshot is delivered.</p>
    #[serde(rename = "deliveryChannelName")]
    pub delivery_channel_name: String,
}

/// <p>The output for the <a>DeliverConfigSnapshot</a> action, in JSON format.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeliverConfigSnapshotResponse {
    /// <p>The ID of the snapshot that is being created.</p>
    #[serde(rename = "configSnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_snapshot_id: Option<String>,
}

/// <p>The channel through which AWS Config delivers notifications and updated configuration states.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeliveryChannel {
    /// <p>The options for how often AWS Config delivers configuration snapshots to the Amazon S3 bucket.</p>
    #[serde(rename = "configSnapshotDeliveryProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_snapshot_delivery_properties: Option<ConfigSnapshotDeliveryProperties>,
    /// <p>The name of the delivery channel. By default, AWS Config assigns the name "default" when creating the delivery channel. To change the delivery channel name, you must use the DeleteDeliveryChannel action to delete your current delivery channel, and then you must use the PutDeliveryChannel command to create a delivery channel that has the desired name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The name of the Amazon S3 bucket to which AWS Config delivers configuration snapshots and configuration history files.</p> <p>If you specify a bucket that belongs to another AWS account, that bucket must have policies that grant access permissions to AWS Config. For more information, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/s3-bucket-policy.html">Permissions for the Amazon S3 Bucket</a> in the AWS Config Developer Guide.</p>
    #[serde(rename = "s3BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_bucket_name: Option<String>,
    /// <p>The prefix for the specified Amazon S3 bucket.</p>
    #[serde(rename = "s3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_key_prefix: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon SNS topic to which AWS Config sends notifications about configuration changes.</p> <p>If you choose a topic from another account, the topic must have policies that grant access permissions to AWS Config. For more information, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/sns-topic-policy.html">Permissions for the Amazon SNS Topic</a> in the AWS Config Developer Guide.</p>
    #[serde(rename = "snsTopicARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
}

/// <p>The status of a specified delivery channel.</p> <p>Valid values: <code>Success</code> | <code>Failure</code> </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeliveryChannelStatus {
    /// <p>A list that contains the status of the delivery of the configuration history to the specified Amazon S3 bucket.</p>
    #[serde(rename = "configHistoryDeliveryInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_history_delivery_info: Option<ConfigExportDeliveryInfo>,
    /// <p>A list containing the status of the delivery of the snapshot to the specified Amazon S3 bucket.</p>
    #[serde(rename = "configSnapshotDeliveryInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_snapshot_delivery_info: Option<ConfigExportDeliveryInfo>,
    /// <p>A list containing the status of the delivery of the configuration stream notification to the specified Amazon SNS topic.</p>
    #[serde(rename = "configStreamDeliveryInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_stream_delivery_info: Option<ConfigStreamDeliveryInfo>,
    /// <p>The name of the delivery channel.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAggregateComplianceByConfigRulesRequest {
    /// <p>The name of the configuration aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: String,
    /// <p>Filters the results by ConfigRuleComplianceFilters object. </p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ConfigRuleComplianceFilters>,
    /// <p>The maximum number of evaluation results returned on each page. The default is maximum. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeAggregateComplianceByConfigRulesResponse {
    /// <p>Returns a list of AggregateComplianceByConfigRule object.</p>
    #[serde(rename = "AggregateComplianceByConfigRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_compliance_by_config_rules: Option<Vec<AggregateComplianceByConfigRule>>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAggregationAuthorizationsRequest {
    /// <p>The maximum number of AggregationAuthorizations returned on each page. The default is maximum. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeAggregationAuthorizationsResponse {
    /// <p>Returns a list of authorizations granted to various aggregator accounts and regions.</p>
    #[serde(rename = "AggregationAuthorizations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_authorizations: Option<Vec<AggregationAuthorization>>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeComplianceByConfigRuleRequest {
    /// <p>Filters the results by compliance.</p> <p>The allowed values are <code>COMPLIANT</code> and <code>NON_COMPLIANT</code>.</p>
    #[serde(rename = "ComplianceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_types: Option<Vec<String>>,
    /// <p>Specify one or more AWS Config rule names to filter the results by rule.</p>
    #[serde(rename = "ConfigRuleNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_names: Option<Vec<String>>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeComplianceByConfigRuleResponse {
    /// <p>Indicates whether each of the specified AWS Config rules is compliant.</p>
    #[serde(rename = "ComplianceByConfigRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_by_config_rules: Option<Vec<ComplianceByConfigRule>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeComplianceByResourceRequest {
    /// <p>Filters the results by compliance.</p> <p>The allowed values are <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>INSUFFICIENT_DATA</code>.</p>
    #[serde(rename = "ComplianceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_types: Option<Vec<String>>,
    /// <p>The maximum number of evaluation results returned on each page. The default is 10. You cannot specify a number greater than 100. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the AWS resource for which you want compliance information. You can specify only one resource ID. If you specify a resource ID, you must also specify a type for <code>ResourceType</code>.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The types of AWS resources for which you want compliance information (for example, <code>AWS::EC2::Instance</code>). For this action, you can specify that the resource type is an AWS account by specifying <code>AWS::::Account</code>.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeComplianceByResourceResponse {
    /// <p>Indicates whether the specified AWS resource complies with all of the AWS Config rules that evaluate it.</p>
    #[serde(rename = "ComplianceByResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_by_resources: Option<Vec<ComplianceByResource>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConfigRuleEvaluationStatusRequest {
    /// <p>The name of the AWS managed Config rules for which you want status information. If you do not specify any names, AWS Config returns status information for all AWS managed Config rules that you use.</p>
    #[serde(rename = "ConfigRuleNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_names: Option<Vec<String>>,
    /// <p>The number of rule evaluation results that you want returned.</p> <p>This parameter is required if the rule limit for your account is more than the default of 150 rules.</p> <p>For information about requesting a rule limit increase, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_config">AWS Config Limits</a> in the <i>AWS General Reference Guide</i>.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeConfigRuleEvaluationStatusResponse {
    /// <p>Status information about your AWS managed Config rules.</p>
    #[serde(rename = "ConfigRulesEvaluationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rules_evaluation_status: Option<Vec<ConfigRuleEvaluationStatus>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConfigRulesRequest {
    /// <p>The names of the AWS Config rules for which you want details. If you do not specify any names, AWS Config returns details for all your rules.</p>
    #[serde(rename = "ConfigRuleNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_names: Option<Vec<String>>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeConfigRulesResponse {
    /// <p>The details about your AWS Config rules.</p>
    #[serde(rename = "ConfigRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rules: Option<Vec<ConfigRule>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConfigurationAggregatorSourcesStatusRequest {
    /// <p>The name of the configuration aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: String,
    /// <p>The maximum number of AggregatorSourceStatus returned on each page. The default is maximum. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>Filters the status type.</p> <ul> <li> <p>Valid value FAILED indicates errors while moving data.</p> </li> <li> <p>Valid value SUCCEEDED indicates the data was successfully moved.</p> </li> <li> <p>Valid value OUTDATED indicates the data is not the most recent.</p> </li> </ul></p>
    #[serde(rename = "UpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeConfigurationAggregatorSourcesStatusResponse {
    /// <p>Returns an AggregatedSourceStatus object. </p>
    #[serde(rename = "AggregatedSourceStatusList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregated_source_status_list: Option<Vec<AggregatedSourceStatus>>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConfigurationAggregatorsRequest {
    /// <p>The name of the configuration aggregators.</p>
    #[serde(rename = "ConfigurationAggregatorNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aggregator_names: Option<Vec<String>>,
    /// <p>The maximum number of configuration aggregators returned on each page. The default is maximum. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeConfigurationAggregatorsResponse {
    /// <p>Returns a ConfigurationAggregators object.</p>
    #[serde(rename = "ConfigurationAggregators")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aggregators: Option<Vec<ConfigurationAggregator>>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The input for the <a>DescribeConfigurationRecorderStatus</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConfigurationRecorderStatusRequest {
    /// <p>The name(s) of the configuration recorder. If the name is not specified, the action returns the current status of all the configuration recorders associated with the account.</p>
    #[serde(rename = "ConfigurationRecorderNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_recorder_names: Option<Vec<String>>,
}

/// <p>The output for the <a>DescribeConfigurationRecorderStatus</a> action, in JSON format.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeConfigurationRecorderStatusResponse {
    /// <p>A list that contains status of the specified recorders.</p>
    #[serde(rename = "ConfigurationRecordersStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_recorders_status: Option<Vec<ConfigurationRecorderStatus>>,
}

/// <p>The input for the <a>DescribeConfigurationRecorders</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConfigurationRecordersRequest {
    /// <p>A list of configuration recorder names.</p>
    #[serde(rename = "ConfigurationRecorderNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_recorder_names: Option<Vec<String>>,
}

/// <p>The output for the <a>DescribeConfigurationRecorders</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeConfigurationRecordersResponse {
    /// <p>A list that contains the descriptions of the specified configuration recorders.</p>
    #[serde(rename = "ConfigurationRecorders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_recorders: Option<Vec<ConfigurationRecorder>>,
}

/// <p>The input for the <a>DeliveryChannelStatus</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDeliveryChannelStatusRequest {
    /// <p>A list of delivery channel names.</p>
    #[serde(rename = "DeliveryChannelNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_channel_names: Option<Vec<String>>,
}

/// <p>The output for the <a>DescribeDeliveryChannelStatus</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeDeliveryChannelStatusResponse {
    /// <p>A list that contains the status of a specified delivery channel.</p>
    #[serde(rename = "DeliveryChannelsStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_channels_status: Option<Vec<DeliveryChannelStatus>>,
}

/// <p>The input for the <a>DescribeDeliveryChannels</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDeliveryChannelsRequest {
    /// <p>A list of delivery channel names.</p>
    #[serde(rename = "DeliveryChannelNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_channel_names: Option<Vec<String>>,
}

/// <p>The output for the <a>DescribeDeliveryChannels</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeDeliveryChannelsResponse {
    /// <p>A list that contains the descriptions of the specified delivery channel.</p>
    #[serde(rename = "DeliveryChannels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_channels: Option<Vec<DeliveryChannel>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribePendingAggregationRequestsRequest {
    /// <p>The maximum number of evaluation results returned on each page. The default is maximum. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribePendingAggregationRequestsResponse {
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns a PendingAggregationRequests object.</p>
    #[serde(rename = "PendingAggregationRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_aggregation_requests: Option<Vec<PendingAggregationRequest>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeRemediationConfigurationsRequest {
    /// <p>A list of AWS Config rule names of remediation configurations for which you want details. </p>
    #[serde(rename = "ConfigRuleNames")]
    pub config_rule_names: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeRemediationConfigurationsResponse {
    /// <p>Returns a remediation configuration object.</p>
    #[serde(rename = "RemediationConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_configurations: Option<Vec<RemediationConfiguration>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeRemediationExecutionStatusRequest {
    /// <p>A list of AWS Config rule names.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
    /// <p>The maximum number of RemediationExecutionStatuses returned on each page. The default is maximum. If you specify 0, AWS Config uses the default. </p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of resource keys to be processed with the current request. Each element in the list consists of the resource type and resource ID. </p>
    #[serde(rename = "ResourceKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_keys: Option<Vec<ResourceKey>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeRemediationExecutionStatusResponse {
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns a list of remediation execution statuses objects.</p>
    #[serde(rename = "RemediationExecutionStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remediation_execution_statuses: Option<Vec<RemediationExecutionStatus>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeRetentionConfigurationsRequest {
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>A list of names of retention configurations for which you want details. If you do not specify a name, AWS Config returns details for all the retention configurations for that account.</p> <note> <p>Currently, AWS Config supports only one retention configuration per region in your account.</p> </note></p>
    #[serde(rename = "RetentionConfigurationNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_configuration_names: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeRetentionConfigurationsResponse {
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns a retention configuration object.</p>
    #[serde(rename = "RetentionConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_configurations: Option<Vec<RetentionConfiguration>>,
}

/// <p>Identifies an AWS resource and indicates whether it complies with the AWS Config rule that it was evaluated against.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Evaluation {
    /// <p>Supplementary information about how the evaluation determined the compliance.</p>
    #[serde(rename = "Annotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<String>,
    /// <p>The ID of the AWS resource that was evaluated.</p>
    #[serde(rename = "ComplianceResourceId")]
    pub compliance_resource_id: String,
    /// <p>The type of AWS resource that was evaluated.</p>
    #[serde(rename = "ComplianceResourceType")]
    pub compliance_resource_type: String,
    /// <p>Indicates whether the AWS resource complies with the AWS Config rule that it was evaluated against.</p> <p>For the <code>Evaluation</code> data type, AWS Config supports only the <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>NOT_APPLICABLE</code> values. AWS Config does not support the <code>INSUFFICIENT_DATA</code> value for this data type.</p> <p>Similarly, AWS Config does not accept <code>INSUFFICIENT_DATA</code> as the value for <code>ComplianceType</code> from a <code>PutEvaluations</code> request. For example, an AWS Lambda function for a custom AWS Config rule cannot pass an <code>INSUFFICIENT_DATA</code> value to AWS Config.</p>
    #[serde(rename = "ComplianceType")]
    pub compliance_type: String,
    /// <p>The time of the event in AWS Config that triggered the evaluation. For event-based evaluations, the time indicates when AWS Config created the configuration item that triggered the evaluation. For periodic evaluations, the time indicates when AWS Config triggered the evaluation at the frequency that you specified (for example, every 24 hours).</p>
    #[serde(rename = "OrderingTimestamp")]
    pub ordering_timestamp: f64,
}

/// <p>The details of an AWS Config evaluation. Provides the AWS resource that was evaluated, the compliance of the resource, related time stamps, and supplementary information.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EvaluationResult {
    /// <p>Supplementary information about how the evaluation determined the compliance.</p>
    #[serde(rename = "Annotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<String>,
    /// <p>Indicates whether the AWS resource complies with the AWS Config rule that evaluated it.</p> <p>For the <code>EvaluationResult</code> data type, AWS Config supports only the <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>NOT_APPLICABLE</code> values. AWS Config does not support the <code>INSUFFICIENT_DATA</code> value for the <code>EvaluationResult</code> data type.</p>
    #[serde(rename = "ComplianceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    /// <p>The time when the AWS Config rule evaluated the AWS resource.</p>
    #[serde(rename = "ConfigRuleInvokedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_invoked_time: Option<f64>,
    /// <p>Uniquely identifies the evaluation result.</p>
    #[serde(rename = "EvaluationResultIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_result_identifier: Option<EvaluationResultIdentifier>,
    /// <p>The time when AWS Config recorded the evaluation result.</p>
    #[serde(rename = "ResultRecordedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_recorded_time: Option<f64>,
    /// <p>An encrypted token that associates an evaluation with an AWS Config rule. The token identifies the rule, the AWS resource being evaluated, and the event that triggered the evaluation.</p>
    #[serde(rename = "ResultToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_token: Option<String>,
}

/// <p>Uniquely identifies an evaluation result.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EvaluationResultIdentifier {
    /// <p>Identifies an AWS Config rule used to evaluate an AWS resource, and provides the type and ID of the evaluated resource.</p>
    #[serde(rename = "EvaluationResultQualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_result_qualifier: Option<EvaluationResultQualifier>,
    /// <p>The time of the event that triggered the evaluation of your AWS resources. The time can indicate when AWS Config delivered a configuration item change notification, or it can indicate when AWS Config delivered the configuration snapshot, depending on which event triggered the evaluation.</p>
    #[serde(rename = "OrderingTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordering_timestamp: Option<f64>,
}

/// <p>Identifies an AWS Config rule that evaluated an AWS resource, and provides the type and ID of the resource that the rule evaluated.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EvaluationResultQualifier {
    /// <p>The name of the AWS Config rule that was used in the evaluation.</p>
    #[serde(rename = "ConfigRuleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_name: Option<String>,
    /// <p>The ID of the evaluated AWS resource.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The type of AWS resource that was evaluated.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>List of each of the failed remediations with specific reasons.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FailedRemediationBatch {
    /// <p>Returns remediation configurations of the failed items.</p>
    #[serde(rename = "FailedItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_items: Option<Vec<RemediationConfiguration>>,
    /// <p>Returns a failure message. For example, the resource is already compliant.</p>
    #[serde(rename = "FailureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
}

/// <p>Details about the fields such as name of the field.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FieldInfo {
    /// <p>Name of the field.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAggregateComplianceDetailsByConfigRuleRequest {
    /// <p>The 12-digit account ID of the source account.</p>
    #[serde(rename = "AccountId")]
    pub account_id: String,
    /// <p>The source region from where the data is aggregated.</p>
    #[serde(rename = "AwsRegion")]
    pub aws_region: String,
    /// <p><p>The resource compliance status.</p> <note> <p>For the <code>GetAggregateComplianceDetailsByConfigRuleRequest</code> data type, AWS Config supports only the <code>COMPLIANT</code> and <code>NON<em>COMPLIANT</code>. AWS Config does not support the <code>NOT</em>APPLICABLE</code> and <code>INSUFFICIENT_DATA</code> values.</p> </note></p>
    #[serde(rename = "ComplianceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_type: Option<String>,
    /// <p>The name of the AWS Config rule for which you want compliance information.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
    /// <p>The name of the configuration aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: String,
    /// <p>The maximum number of evaluation results returned on each page. The default is 50. You cannot specify a number greater than 100. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAggregateComplianceDetailsByConfigRuleResponse {
    /// <p>Returns an AggregateEvaluationResults object.</p>
    #[serde(rename = "AggregateEvaluationResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_evaluation_results: Option<Vec<AggregateEvaluationResult>>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAggregateConfigRuleComplianceSummaryRequest {
    /// <p>The name of the configuration aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: String,
    /// <p>Filters the results based on the ConfigRuleComplianceSummaryFilters object.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ConfigRuleComplianceSummaryFilters>,
    /// <p>Groups the result based on ACCOUNT_ID or AWS_REGION.</p>
    #[serde(rename = "GroupByKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_key: Option<String>,
    /// <p>The maximum number of evaluation results returned on each page. The default is 1000. You cannot specify a number greater than 1000. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAggregateConfigRuleComplianceSummaryResponse {
    /// <p>Returns a list of AggregateComplianceCounts object.</p>
    #[serde(rename = "AggregateComplianceCounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_compliance_counts: Option<Vec<AggregateComplianceCount>>,
    /// <p>Groups the result based on ACCOUNT_ID or AWS_REGION.</p>
    #[serde(rename = "GroupByKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_key: Option<String>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAggregateDiscoveredResourceCountsRequest {
    /// <p>The name of the configuration aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: String,
    /// <p>Filters the results based on the <code>ResourceCountFilters</code> object.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ResourceCountFilters>,
    /// <p>The key to group the resource counts.</p>
    #[serde(rename = "GroupByKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_key: Option<String>,
    /// <p>The maximum number of <a>GroupedResourceCount</a> objects returned on each page. The default is 1000. You cannot specify a number greater than 1000. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAggregateDiscoveredResourceCountsResponse {
    /// <p>The key passed into the request object. If <code>GroupByKey</code> is not provided, the result will be empty.</p>
    #[serde(rename = "GroupByKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by_key: Option<String>,
    /// <p>Returns a list of GroupedResourceCount objects.</p>
    #[serde(rename = "GroupedResourceCounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouped_resource_counts: Option<Vec<GroupedResourceCount>>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The total number of resources that are present in an aggregator with the filters that you provide.</p>
    #[serde(rename = "TotalDiscoveredResources")]
    pub total_discovered_resources: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAggregateResourceConfigRequest {
    /// <p>The name of the configuration aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: String,
    /// <p>An object that identifies aggregate resource.</p>
    #[serde(rename = "ResourceIdentifier")]
    pub resource_identifier: AggregateResourceIdentifier,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAggregateResourceConfigResponse {
    /// <p>Returns a <code>ConfigurationItem</code> object.</p>
    #[serde(rename = "ConfigurationItem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_item: Option<ConfigurationItem>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetComplianceDetailsByConfigRuleRequest {
    /// <p>Filters the results by compliance.</p> <p>The allowed values are <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>NOT_APPLICABLE</code>.</p>
    #[serde(rename = "ComplianceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_types: Option<Vec<String>>,
    /// <p>The name of the AWS Config rule for which you want compliance information.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
    /// <p>The maximum number of evaluation results returned on each page. The default is 10. You cannot specify a number greater than 100. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetComplianceDetailsByConfigRuleResponse {
    /// <p>Indicates whether the AWS resource complies with the specified AWS Config rule.</p>
    #[serde(rename = "EvaluationResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_results: Option<Vec<EvaluationResult>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetComplianceDetailsByResourceRequest {
    /// <p>Filters the results by compliance.</p> <p>The allowed values are <code>COMPLIANT</code>, <code>NON_COMPLIANT</code>, and <code>NOT_APPLICABLE</code>.</p>
    #[serde(rename = "ComplianceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_types: Option<Vec<String>>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the AWS resource for which you want compliance information.</p>
    #[serde(rename = "ResourceId")]
    pub resource_id: String,
    /// <p>The type of the AWS resource for which you want compliance information.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetComplianceDetailsByResourceResponse {
    /// <p>Indicates whether the specified AWS resource complies each AWS Config rule.</p>
    #[serde(rename = "EvaluationResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_results: Option<Vec<EvaluationResult>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetComplianceSummaryByConfigRuleResponse {
    /// <p>The number of AWS Config rules that are compliant and the number that are noncompliant, up to a maximum of 25 for each.</p>
    #[serde(rename = "ComplianceSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summary: Option<ComplianceSummary>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetComplianceSummaryByResourceTypeRequest {
    /// <p>Specify one or more resource types to get the number of resources that are compliant and the number that are noncompliant for each resource type.</p> <p>For this request, you can specify an AWS resource type such as <code>AWS::EC2::Instance</code>. You can specify that the resource type is an AWS account by specifying <code>AWS::::Account</code>.</p>
    #[serde(rename = "ResourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetComplianceSummaryByResourceTypeResponse {
    /// <p>The number of resources that are compliant and the number that are noncompliant. If one or more resource types were provided with the request, the numbers are returned for each resource type. The maximum number returned is 100.</p>
    #[serde(rename = "ComplianceSummariesByResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_summaries_by_resource_type: Option<Vec<ComplianceSummaryByResourceType>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDiscoveredResourceCountsRequest {
    /// <p>The maximum number of <a>ResourceCount</a> objects returned on each page. The default is 100. You cannot specify a number greater than 100. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p><p>The comma-separated list that specifies the resource types that you want AWS Config to return (for example, <code>&quot;AWS::EC2::Instance&quot;</code>, <code>&quot;AWS::IAM::User&quot;</code>).</p> <p>If a value for <code>resourceTypes</code> is not specified, AWS Config returns all resource types that AWS Config is recording in the region for your account.</p> <note> <p>If the configuration recorder is turned off, AWS Config returns an empty list of <a>ResourceCount</a> objects. If the configuration recorder is not recording a specific resource type (for example, S3 buckets), that resource type is not returned in the list of <a>ResourceCount</a> objects.</p> </note></p>
    #[serde(rename = "resourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDiscoveredResourceCountsResponse {
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of <code>ResourceCount</code> objects. Each object is listed in descending order by the number of resources.</p>
    #[serde(rename = "resourceCounts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_counts: Option<Vec<ResourceCount>>,
    /// <p><p>The total number of resources that AWS Config is recording in the region for your account. If you specify resource types in the request, AWS Config returns only the total number of resources for those resource types.</p> <p class="title"> <b>Example</b> </p> <ol> <li> <p>AWS Config is recording three resource types in the US East (Ohio) Region for your account: 25 EC2 instances, 20 IAM users, and 15 S3 buckets, for a total of 60 resources.</p> </li> <li> <p>You make a call to the <code>GetDiscoveredResourceCounts</code> action and specify the resource type, <code>&quot;AWS::EC2::Instances&quot;</code>, in the request.</p> </li> <li> <p>AWS Config returns 25 for <code>totalDiscoveredResources</code>.</p> </li> </ol></p>
    #[serde(rename = "totalDiscoveredResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discovered_resources: Option<i64>,
}

/// <p>The input for the <a>GetResourceConfigHistory</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetResourceConfigHistoryRequest {
    /// <p>The chronological order for configuration items listed. By default, the results are listed in reverse chronological order.</p>
    #[serde(rename = "chronologicalOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chronological_order: Option<String>,
    /// <p>The time stamp that indicates an earlier time. If not specified, the action returns paginated results that contain configuration items that start when the first configuration item was recorded.</p>
    #[serde(rename = "earlierTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earlier_time: Option<f64>,
    /// <p>The time stamp that indicates a later time. If not specified, current time is taken.</p>
    #[serde(rename = "laterTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub later_time: Option<f64>,
    /// <p>The maximum number of configuration items returned on each page. The default is 10. You cannot specify a number greater than 100. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the resource (for example., <code>sg-xxxxxx</code>).</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>The resource type.</p>
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}

/// <p>The output for the <a>GetResourceConfigHistory</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetResourceConfigHistoryResponse {
    /// <p>A list that contains the configuration history of one or more resources.</p>
    #[serde(rename = "configurationItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_items: Option<Vec<ConfigurationItem>>,
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>The count of resources that are grouped by the group name.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GroupedResourceCount {
    /// <p>The name of the group that can be region, account ID, or resource type. For example, region1, region2 if the region was chosen as <code>GroupByKey</code>.</p>
    #[serde(rename = "GroupName")]
    pub group_name: String,
    /// <p>The number of resources in the group.</p>
    #[serde(rename = "ResourceCount")]
    pub resource_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAggregateDiscoveredResourcesRequest {
    /// <p>The name of the configuration aggregator. </p>
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: String,
    /// <p>Filters the results based on the <code>ResourceFilters</code> object.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ResourceFilters>,
    /// <p>The maximum number of resource identifiers returned on each page. The default is 100. You cannot specify a number greater than 100. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of resources that you want AWS Config to list in the response.</p>
    #[serde(rename = "ResourceType")]
    pub resource_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListAggregateDiscoveredResourcesResponse {
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns a list of <code>ResourceIdentifiers</code> objects.</p>
    #[serde(rename = "ResourceIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifiers: Option<Vec<AggregateResourceIdentifier>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDiscoveredResourcesRequest {
    /// <p>Specifies whether AWS Config includes deleted resources in the results. By default, deleted resources are not included.</p>
    #[serde(rename = "includeDeletedResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deleted_resources: Option<bool>,
    /// <p>The maximum number of resource identifiers returned on each page. The default is 100. You cannot specify a number greater than 100. If you specify 0, AWS Config uses the default.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned on a previous page that you use to get the next page of results in a paginated response.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The IDs of only those resources that you want AWS Config to list in the response. If you do not specify this parameter, AWS Config lists all resources of the specified type that it has discovered.</p>
    #[serde(rename = "resourceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_ids: Option<Vec<String>>,
    /// <p>The custom name of only those resources that you want AWS Config to list in the response. If you do not specify this parameter, AWS Config lists all resources of the specified type that it has discovered.</p>
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// <p>The type of resources that you want AWS Config to list in the response.</p>
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListDiscoveredResourcesResponse {
    /// <p>The string that you use in a subsequent request to get the next page of results in a paginated response.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The details that identify a resource that is discovered by AWS Config, including the resource type, ID, and (if available) the custom resource name.</p>
    #[serde(rename = "resourceIdentifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifiers: Option<Vec<ResourceIdentifier>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceRequest {
    /// <p>The maximum number of tags returned on each page. The limit maximum is 50. You cannot specify a number greater than 50. If you specify 0, AWS Config uses the default. </p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The Amazon Resource Name (ARN) that identifies the resource for which to list the tags. Currently, the supported resources are <code>ConfigRule</code>, <code>ConfigurationAggregator</code> and <code>AggregatorAuthorization</code>.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The nextToken string returned on a previous page that you use to get the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The tags for the resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>This object contains regions to set up the aggregator and an IAM role to retrieve organization details.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrganizationAggregationSource {
    /// <p>If true, aggregate existing AWS Config regions and future regions.</p>
    #[serde(rename = "AllAwsRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_aws_regions: Option<bool>,
    /// <p>The source regions being aggregated.</p>
    #[serde(rename = "AwsRegions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_regions: Option<Vec<String>>,
    /// <p>ARN of the IAM role used to retrieve AWS Organization details associated with the aggregator account.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
}

/// <p>An object that represents the account ID and region of an aggregator account that is requesting authorization but is not yet authorized.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PendingAggregationRequest {
    /// <p>The 12-digit account ID of the account requesting to aggregate data.</p>
    #[serde(rename = "RequesterAccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_account_id: Option<String>,
    /// <p>The region requesting to aggregate data. </p>
    #[serde(rename = "RequesterAwsRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requester_aws_region: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutAggregationAuthorizationRequest {
    /// <p>The 12-digit account ID of the account authorized to aggregate data.</p>
    #[serde(rename = "AuthorizedAccountId")]
    pub authorized_account_id: String,
    /// <p>The region authorized to collect aggregated data.</p>
    #[serde(rename = "AuthorizedAwsRegion")]
    pub authorized_aws_region: String,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutAggregationAuthorizationResponse {
    /// <p>Returns an AggregationAuthorization object. </p>
    #[serde(rename = "AggregationAuthorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregation_authorization: Option<AggregationAuthorization>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutConfigRuleRequest {
    /// <p>The rule that you want to add to your account.</p>
    #[serde(rename = "ConfigRule")]
    pub config_rule: ConfigRule,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutConfigurationAggregatorRequest {
    /// <p>A list of AccountAggregationSource object. </p>
    #[serde(rename = "AccountAggregationSources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_aggregation_sources: Option<Vec<AccountAggregationSource>>,
    /// <p>The name of the configuration aggregator.</p>
    #[serde(rename = "ConfigurationAggregatorName")]
    pub configuration_aggregator_name: String,
    /// <p>An OrganizationAggregationSource object.</p>
    #[serde(rename = "OrganizationAggregationSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_aggregation_source: Option<OrganizationAggregationSource>,
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutConfigurationAggregatorResponse {
    /// <p>Returns a ConfigurationAggregator object.</p>
    #[serde(rename = "ConfigurationAggregator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_aggregator: Option<ConfigurationAggregator>,
}

/// <p>The input for the <a>PutConfigurationRecorder</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutConfigurationRecorderRequest {
    /// <p>The configuration recorder object that records each configuration change made to the resources.</p>
    #[serde(rename = "ConfigurationRecorder")]
    pub configuration_recorder: ConfigurationRecorder,
}

/// <p>The input for the <a>PutDeliveryChannel</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutDeliveryChannelRequest {
    /// <p>The configuration delivery channel object that delivers the configuration information to an Amazon S3 bucket and to an Amazon SNS topic.</p>
    #[serde(rename = "DeliveryChannel")]
    pub delivery_channel: DeliveryChannel,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutEvaluationsRequest {
    /// <p>The assessments that the AWS Lambda function performs. Each evaluation identifies an AWS resource and indicates whether it complies with the AWS Config rule that invokes the AWS Lambda function.</p>
    #[serde(rename = "Evaluations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluations: Option<Vec<Evaluation>>,
    /// <p>An encrypted token that associates an evaluation with an AWS Config rule. Identifies the rule and the event that triggered the evaluation.</p>
    #[serde(rename = "ResultToken")]
    pub result_token: String,
    /// <p><p>Use this parameter to specify a test run for <code>PutEvaluations</code>. You can verify whether your AWS Lambda function will deliver evaluation results to AWS Config. No updates occur to your existing evaluations, and evaluation results are not sent to AWS Config.</p> <note> <p>When <code>TestMode</code> is <code>true</code>, <code>PutEvaluations</code> doesn&#39;t require a valid value for the <code>ResultToken</code> parameter, but the value cannot be null.</p> </note></p>
    #[serde(rename = "TestMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_mode: Option<bool>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutEvaluationsResponse {
    /// <p>Requests that failed because of a client or server error.</p>
    #[serde(rename = "FailedEvaluations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_evaluations: Option<Vec<Evaluation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutRemediationConfigurationsRequest {
    /// <p>A list of remediation configuration objects.</p>
    #[serde(rename = "RemediationConfigurations")]
    pub remediation_configurations: Vec<RemediationConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutRemediationConfigurationsResponse {
    /// <p>Returns a list of failed remediation batch objects.</p>
    #[serde(rename = "FailedBatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_batches: Option<Vec<FailedRemediationBatch>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutRetentionConfigurationRequest {
    /// <p><p>Number of days AWS Config stores your historical information.</p> <note> <p>Currently, only applicable to the configuration item history.</p> </note></p>
    #[serde(rename = "RetentionPeriodInDays")]
    pub retention_period_in_days: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutRetentionConfigurationResponse {
    /// <p>Returns a retention configuration object.</p>
    #[serde(rename = "RetentionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_configuration: Option<RetentionConfiguration>,
}

/// <p>Details about the query.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct QueryInfo {
    /// <p>Returns a <code>FieldInfo</code> object.</p>
    #[serde(rename = "SelectFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_fields: Option<Vec<FieldInfo>>,
}

/// <p>Specifies the types of AWS resource for which AWS Config records configuration changes.</p> <p>In the recording group, you specify whether all supported types or specific types of resources are recorded.</p> <p>By default, AWS Config records configuration changes for all supported types of regional resources that AWS Config discovers in the region in which it is running. Regional resources are tied to a region and can be used only in that region. Examples of regional resources are EC2 instances and EBS volumes.</p> <p>You can also have AWS Config record configuration changes for supported types of global resources (for example, IAM resources). Global resources are not tied to an individual region and can be used in all regions.</p> <important> <p>The configuration details for any global resource are the same in all regions. If you customize AWS Config in multiple regions to record global resources, it will create multiple configuration items each time a global resource changes: one configuration item for each region. These configuration items will contain identical data. To prevent duplicate configuration items, you should consider customizing AWS Config in only one region to record global resources, unless you want the configuration items to be available in multiple regions.</p> </important> <p>If you don't want AWS Config to record all resources, you can specify which types of resources it will record with the <code>resourceTypes</code> parameter.</p> <p>For a list of supported resource types, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/resource-config-reference.html#supported-resources">Supported Resource Types</a>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/select-resources.html">Selecting Which Resources AWS Config Records</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordingGroup {
    /// <p>Specifies whether AWS Config records configuration changes for every supported type of regional resource.</p> <p>If you set this option to <code>true</code>, when AWS Config adds support for a new type of regional resource, it starts recording resources of that type automatically.</p> <p>If you set this option to <code>true</code>, you cannot enumerate a list of <code>resourceTypes</code>.</p>
    #[serde(rename = "allSupported")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_supported: Option<bool>,
    /// <p>Specifies whether AWS Config includes all supported types of global resources (for example, IAM resources) with the resources that it records.</p> <p>Before you can set this option to <code>true</code>, you must set the <code>allSupported</code> option to <code>true</code>.</p> <p>If you set this option to <code>true</code>, when AWS Config adds support for a new type of global resource, it starts recording resources of that type automatically.</p> <p>The configuration details for any global resource are the same in all regions. To prevent duplicate configuration items, you should consider customizing AWS Config in only one region to record global resources.</p>
    #[serde(rename = "includeGlobalResourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_global_resource_types: Option<bool>,
    /// <p>A comma-separated list that specifies the types of AWS resources for which AWS Config records configuration changes (for example, <code>AWS::EC2::Instance</code> or <code>AWS::CloudTrail::Trail</code>).</p> <p>Before you can set this option to <code>true</code>, you must set the <code>allSupported</code> option to <code>false</code>.</p> <p>If you set this option to <code>true</code>, when AWS Config adds support for a new type of resource, it will not record resources of that type unless you manually add that type to your recording group.</p> <p>For a list of valid <code>resourceTypes</code> values, see the <b>resourceType Value</b> column in <a href="https://docs.aws.amazon.com/config/latest/developerguide/resource-config-reference.html#supported-resources">Supported AWS Resource Types</a>.</p>
    #[serde(rename = "resourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_types: Option<Vec<String>>,
}

/// <p>The relationship of the related resource to the main resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Relationship {
    /// <p>The type of relationship with the related resource.</p>
    #[serde(rename = "relationshipName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_name: Option<String>,
    /// <p>The ID of the related resource (for example, <code>sg-xxxxxx</code>).</p>
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The custom name of the related resource, if available.</p>
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// <p>The resource type of the related resource.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>An object that represents the details about the remediation configuration that includes the remediation action, parameters, and data to execute the action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemediationConfiguration {
    /// <p>The name of the AWS Config rule.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
    /// <p>An object of the RemediationParameterValue.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, RemediationParameterValue>>,
    /// <p>The type of a resource. </p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>Target ID is the name of the public document.</p>
    #[serde(rename = "TargetId")]
    pub target_id: String,
    /// <p>The type of the target. Target executes remediation. For example, SSM document.</p>
    #[serde(rename = "TargetType")]
    pub target_type: String,
    /// <p>Version of the target. For example, version of the SSM document.</p>
    #[serde(rename = "TargetVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_version: Option<String>,
}

/// <p>Provides details of the current status of the invoked remediation action for that resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RemediationExecutionStatus {
    /// <p>Start time when the remediation was executed.</p>
    #[serde(rename = "InvocationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_time: Option<f64>,
    /// <p>The time when the remediation execution was last updated.</p>
    #[serde(rename = "LastUpdatedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "ResourceKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_key: Option<ResourceKey>,
    /// <p>ENUM of the values.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>Details of every step.</p>
    #[serde(rename = "StepDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_details: Option<Vec<RemediationExecutionStep>>,
}

/// <p>Name of the step from the SSM document.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RemediationExecutionStep {
    /// <p>An error message if the step was interrupted during execution.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The details of the step.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The time when the step started.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The valid status of the step.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The time when the step stopped.</p>
    #[serde(rename = "StopTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<f64>,
}

/// <p>The value is either a dynamic (resource) value or a static value. You must select either a dynamic value or a static value.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemediationParameterValue {
    /// <p>The value is dynamic and changes at run-time.</p>
    #[serde(rename = "ResourceValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_value: Option<ResourceValue>,
    /// <p>The value is static and does not change at run-time.</p>
    #[serde(rename = "StaticValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_value: Option<StaticValue>,
}

/// <p>An object that contains the resource type and the number of resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResourceCount {
    /// <p>The number of resources.</p>
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>The resource type (for example, <code>"AWS::EC2::Instance"</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Filters the resource count based on account ID, region, and resource type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ResourceCountFilters {
    /// <p>The 12-digit ID of the account.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The region where the account is located.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The type of the AWS resource.</p>
    #[serde(rename = "ResourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>Filters the results by resource account ID, region, resource ID, and resource name.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ResourceFilters {
    /// <p>The 12-digit source account ID.</p>
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// <p>The source region.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The ID of the resource.</p>
    #[serde(rename = "ResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The name of the resource.</p>
    #[serde(rename = "ResourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
}

/// <p>The details that identify a resource that is discovered by AWS Config, including the resource type, ID, and (if available) the custom resource name.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResourceIdentifier {
    /// <p>The time that the resource was deleted.</p>
    #[serde(rename = "resourceDeletionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_deletion_time: Option<f64>,
    /// <p>The ID of the resource (for example, <code>sg-xxxxxx</code>).</p>
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>The custom name of the resource (if available).</p>
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// <p>The type of resource.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

/// <p>The details that identify a resource within AWS Config, including the resource type and resource ID.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceKey {
    /// <p>The ID of the resource (for example., sg-xxxxxx). </p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>The resource type.</p>
    #[serde(rename = "resourceType")]
    pub resource_type: String,
}

/// <p>The dynamic value of the resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceValue {
    /// <p>The value is a resource ID.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>An object with the name of the retention configuration and the retention period in days. The object stores the configuration for data retention in AWS Config.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RetentionConfiguration {
    /// <p>The name of the retention configuration object.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>Number of days AWS Config stores your historical information.</p> <note> <p>Currently, only applicable to the configuration item history.</p> </note></p>
    #[serde(rename = "RetentionPeriodInDays")]
    pub retention_period_in_days: i64,
}

/// <p>Defines which resources trigger an evaluation for an AWS Config rule. The scope can include one or more resource types, a combination of a tag key and value, or a combination of one resource type and one resource ID. Specify a scope to constrain which resources trigger an evaluation for a rule. Otherwise, evaluations for the rule are triggered when any resource in your recording group changes in configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scope {
    /// <p>The ID of the only AWS resource that you want to trigger an evaluation for the rule. If you specify a resource ID, you must specify one resource type for <code>ComplianceResourceTypes</code>.</p>
    #[serde(rename = "ComplianceResourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_resource_id: Option<String>,
    /// <p>The resource types of only those AWS resources that you want to trigger an evaluation for the rule. You can only specify one type if you also specify a resource ID for <code>ComplianceResourceId</code>.</p>
    #[serde(rename = "ComplianceResourceTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance_resource_types: Option<Vec<String>>,
    /// <p>The tag key that is applied to only those AWS resources that you want to trigger an evaluation for the rule.</p>
    #[serde(rename = "TagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_key: Option<String>,
    /// <p>The tag value applied to only those AWS resources that you want to trigger an evaluation for the rule. If you specify a value for <code>TagValue</code>, you must also specify a value for <code>TagKey</code>.</p>
    #[serde(rename = "TagValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SelectResourceConfigRequest {
    /// <p>The SQL query <code>SELECT</code> command.</p>
    #[serde(rename = "Expression")]
    pub expression: String,
    /// <p>The maximum number of query results returned on each page. </p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SelectResourceConfigResponse {
    /// <p>The <code>nextToken</code> string returned in a previous request that you use to request the next page of results in a paginated response. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Returns the <code>QueryInfo</code> object.</p>
    #[serde(rename = "QueryInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_info: Option<QueryInfo>,
    /// <p>Returns the results for the SQL query.</p>
    #[serde(rename = "Results")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<String>>,
}

/// <p>Provides the AWS Config rule owner (AWS or customer), the rule identifier, and the events that trigger the evaluation of your AWS resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source {
    /// <p>Indicates whether AWS or the customer owns and manages the AWS Config rule.</p>
    #[serde(rename = "Owner")]
    pub owner: String,
    /// <p>Provides the source and type of the event that causes AWS Config to evaluate your AWS resources.</p>
    #[serde(rename = "SourceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_details: Option<Vec<SourceDetail>>,
    /// <p>For AWS Config managed rules, a predefined identifier from a list. For example, <code>IAM_PASSWORD_POLICY</code> is a managed rule. To reference a managed rule, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html">Using AWS Managed Config Rules</a>.</p> <p>For custom rules, the identifier is the Amazon Resource Name (ARN) of the rule's AWS Lambda function, such as <code>arn:aws:lambda:us-east-2:123456789012:function:custom_rule_name</code>.</p>
    #[serde(rename = "SourceIdentifier")]
    pub source_identifier: String,
}

/// <p>Provides the source and the message types that trigger AWS Config to evaluate your AWS resources against a rule. It also provides the frequency with which you want AWS Config to run evaluations for the rule if the trigger type is periodic. You can specify the parameter values for <code>SourceDetail</code> only for custom rules. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceDetail {
    /// <p>The source of the event, such as an AWS service, that triggers AWS Config to evaluate your AWS resources.</p>
    #[serde(rename = "EventSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source: Option<String>,
    /// <p><p>The frequency at which you want AWS Config to run evaluations for a custom rule with a periodic trigger. If you specify a value for <code>MaximumExecutionFrequency</code>, then <code>MessageType</code> must use the <code>ScheduledNotification</code> value.</p> <note> <p>By default, rules with a periodic trigger are evaluated every 24 hours. To change the frequency, specify a valid value for the <code>MaximumExecutionFrequency</code> parameter.</p> <p>Based on the valid value you choose, AWS Config runs evaluations once for each valid value. For example, if you choose <code>Three<em>Hours</code>, AWS Config runs evaluations once every three hours. In this case, <code>Three</em>Hours</code> is the frequency of this rule. </p> </note></p>
    #[serde(rename = "MaximumExecutionFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_execution_frequency: Option<String>,
    /// <p>The type of notification that triggers AWS Config to run an evaluation for a rule. You can specify the following notification types:</p> <ul> <li> <p> <code>ConfigurationItemChangeNotification</code> - Triggers an evaluation when AWS Config delivers a configuration item as a result of a resource change.</p> </li> <li> <p> <code>OversizedConfigurationItemChangeNotification</code> - Triggers an evaluation when AWS Config delivers an oversized configuration item. AWS Config may generate this notification type when a resource changes and the notification exceeds the maximum size allowed by Amazon SNS.</p> </li> <li> <p> <code>ScheduledNotification</code> - Triggers a periodic evaluation at the frequency specified for <code>MaximumExecutionFrequency</code>.</p> </li> <li> <p> <code>ConfigurationSnapshotDeliveryCompleted</code> - Triggers a periodic evaluation when AWS Config delivers a configuration snapshot.</p> </li> </ul> <p>If you want your custom rule to be triggered by configuration changes, specify two SourceDetail objects, one for <code>ConfigurationItemChangeNotification</code> and one for <code>OversizedConfigurationItemChangeNotification</code>.</p>
    #[serde(rename = "MessageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartConfigRulesEvaluationRequest {
    /// <p>The list of names of AWS Config rules that you want to run evaluations for.</p>
    #[serde(rename = "ConfigRuleNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_rule_names: Option<Vec<String>>,
}

/// <p>The output when you start the evaluation for the specified AWS Config rule.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartConfigRulesEvaluationResponse {}

/// <p>The input for the <a>StartConfigurationRecorder</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartConfigurationRecorderRequest {
    /// <p>The name of the recorder object that records each configuration change made to the resources.</p>
    #[serde(rename = "ConfigurationRecorderName")]
    pub configuration_recorder_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartRemediationExecutionRequest {
    /// <p>The list of names of AWS Config rules that you want to run remediation execution for.</p>
    #[serde(rename = "ConfigRuleName")]
    pub config_rule_name: String,
    /// <p>A list of resource keys to be processed with the current request. Each element in the list consists of the resource type and resource ID. </p>
    #[serde(rename = "ResourceKeys")]
    pub resource_keys: Vec<ResourceKey>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartRemediationExecutionResponse {
    /// <p>For resources that have failed to start execution, the API returns a resource key object.</p>
    #[serde(rename = "FailedItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_items: Option<Vec<ResourceKey>>,
    /// <p>Returns a failure message. For example, the resource is already compliant.</p>
    #[serde(rename = "FailureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
}

/// <p>The static value of the resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StaticValue {
    /// <p>A list of values. For example, the ARN of the assumed role. </p>
    #[serde(rename = "Values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

/// <p>The input for the <a>StopConfigurationRecorder</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopConfigurationRecorderRequest {
    /// <p>The name of the recorder object that records each configuration change made to the resources.</p>
    #[serde(rename = "ConfigurationRecorderName")]
    pub configuration_recorder_name: String,
}

/// <p>The tags for the resource. The metadata that you apply to a resource to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>One part of a key-value pair that make up a tag. A key is a general label that acts like a category for more specific tag values.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The optional part of a key-value pair that make up a tag. A value acts as a descriptor within a tag category (key).</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the resource for which to list the tags. Currently, the supported resources are <code>ConfigRule</code>, <code>ConfigurationAggregator</code> and <code>AggregatorAuthorization</code>.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>An array of tag object.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the resource for which to list the tags. Currently, the supported resources are <code>ConfigRule</code>, <code>ConfigurationAggregator</code> and <code>AggregatorAuthorization</code>.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The keys of the tags to be removed.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// Errors returned by BatchGetAggregateResourceConfig
#[derive(Debug, PartialEq)]
pub enum BatchGetAggregateResourceConfigError {
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
}

impl BatchGetAggregateResourceConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<BatchGetAggregateResourceConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchConfigurationAggregatorException" => {
                    return RusotoError::Service(
                        BatchGetAggregateResourceConfigError::NoSuchConfigurationAggregator(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchGetAggregateResourceConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetAggregateResourceConfigError {
    fn description(&self) -> &str {
        match *self {
            BatchGetAggregateResourceConfigError::NoSuchConfigurationAggregator(ref cause) => cause,
        }
    }
}
/// Errors returned by BatchGetResourceConfig
#[derive(Debug, PartialEq)]
pub enum BatchGetResourceConfigError {
    /// <p>There are no configuration recorders available to provide the role needed to describe your resources. Create a configuration recorder.</p>
    NoAvailableConfigurationRecorder(String),
}

impl BatchGetResourceConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchGetResourceConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoAvailableConfigurationRecorderException" => {
                    return RusotoError::Service(
                        BatchGetResourceConfigError::NoAvailableConfigurationRecorder(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchGetResourceConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchGetResourceConfigError {
    fn description(&self) -> &str {
        match *self {
            BatchGetResourceConfigError::NoAvailableConfigurationRecorder(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAggregationAuthorization
#[derive(Debug, PartialEq)]
pub enum DeleteAggregationAuthorizationError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
}

impl DeleteAggregationAuthorizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteAggregationAuthorizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DeleteAggregationAuthorizationError::InvalidParameterValue(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteAggregationAuthorizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAggregationAuthorizationError {
    fn description(&self) -> &str {
        match *self {
            DeleteAggregationAuthorizationError::InvalidParameterValue(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteConfigRule
#[derive(Debug, PartialEq)]
pub enum DeleteConfigRuleError {
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
    /// <p>The rule is currently being deleted or the rule is deleting your evaluation results. Try your request again later.</p>
    ResourceInUse(String),
}

impl DeleteConfigRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteConfigRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchConfigRuleException" => {
                    return RusotoError::Service(DeleteConfigRuleError::NoSuchConfigRule(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteConfigRuleError::ResourceInUse(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteConfigRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConfigRuleError {
    fn description(&self) -> &str {
        match *self {
            DeleteConfigRuleError::NoSuchConfigRule(ref cause) => cause,
            DeleteConfigRuleError::ResourceInUse(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteConfigurationAggregator
#[derive(Debug, PartialEq)]
pub enum DeleteConfigurationAggregatorError {
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
}

impl DeleteConfigurationAggregatorError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteConfigurationAggregatorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchConfigurationAggregatorException" => {
                    return RusotoError::Service(
                        DeleteConfigurationAggregatorError::NoSuchConfigurationAggregator(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteConfigurationAggregatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConfigurationAggregatorError {
    fn description(&self) -> &str {
        match *self {
            DeleteConfigurationAggregatorError::NoSuchConfigurationAggregator(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteConfigurationRecorder
#[derive(Debug, PartialEq)]
pub enum DeleteConfigurationRecorderError {
    /// <p>You have specified a configuration recorder that does not exist.</p>
    NoSuchConfigurationRecorder(String),
}

impl DeleteConfigurationRecorderError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteConfigurationRecorderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchConfigurationRecorderException" => {
                    return RusotoError::Service(
                        DeleteConfigurationRecorderError::NoSuchConfigurationRecorder(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteConfigurationRecorderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteConfigurationRecorderError {
    fn description(&self) -> &str {
        match *self {
            DeleteConfigurationRecorderError::NoSuchConfigurationRecorder(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDeliveryChannel
#[derive(Debug, PartialEq)]
pub enum DeleteDeliveryChannelError {
    /// <p>You cannot delete the delivery channel you specified because the configuration recorder is running.</p>
    LastDeliveryChannelDeleteFailed(String),
    /// <p>You have specified a delivery channel that does not exist.</p>
    NoSuchDeliveryChannel(String),
}

impl DeleteDeliveryChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDeliveryChannelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "LastDeliveryChannelDeleteFailedException" => {
                    return RusotoError::Service(
                        DeleteDeliveryChannelError::LastDeliveryChannelDeleteFailed(err.msg),
                    )
                }
                "NoSuchDeliveryChannelException" => {
                    return RusotoError::Service(DeleteDeliveryChannelError::NoSuchDeliveryChannel(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteDeliveryChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDeliveryChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteDeliveryChannelError::LastDeliveryChannelDeleteFailed(ref cause) => cause,
            DeleteDeliveryChannelError::NoSuchDeliveryChannel(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteEvaluationResults
#[derive(Debug, PartialEq)]
pub enum DeleteEvaluationResultsError {
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
    /// <p>The rule is currently being deleted or the rule is deleting your evaluation results. Try your request again later.</p>
    ResourceInUse(String),
}

impl DeleteEvaluationResultsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEvaluationResultsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchConfigRuleException" => {
                    return RusotoError::Service(DeleteEvaluationResultsError::NoSuchConfigRule(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteEvaluationResultsError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteEvaluationResultsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEvaluationResultsError {
    fn description(&self) -> &str {
        match *self {
            DeleteEvaluationResultsError::NoSuchConfigRule(ref cause) => cause,
            DeleteEvaluationResultsError::ResourceInUse(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePendingAggregationRequest
#[derive(Debug, PartialEq)]
pub enum DeletePendingAggregationRequestError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
}

impl DeletePendingAggregationRequestError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeletePendingAggregationRequestError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DeletePendingAggregationRequestError::InvalidParameterValue(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeletePendingAggregationRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePendingAggregationRequestError {
    fn description(&self) -> &str {
        match *self {
            DeletePendingAggregationRequestError::InvalidParameterValue(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRemediationConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteRemediationConfigurationError {
    /// <p>You specified an AWS Config rule without a remediation configuration.</p>
    NoSuchRemediationConfiguration(String),
}

impl DeleteRemediationConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteRemediationConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchRemediationConfigurationException" => {
                    return RusotoError::Service(
                        DeleteRemediationConfigurationError::NoSuchRemediationConfiguration(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteRemediationConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRemediationConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteRemediationConfigurationError::NoSuchRemediationConfiguration(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRetentionConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteRetentionConfigurationError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>You have specified a retention configuration that does not exist.</p>
    NoSuchRetentionConfiguration(String),
}

impl DeleteRetentionConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteRetentionConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DeleteRetentionConfigurationError::InvalidParameterValue(err.msg),
                    )
                }
                "NoSuchRetentionConfigurationException" => {
                    return RusotoError::Service(
                        DeleteRetentionConfigurationError::NoSuchRetentionConfiguration(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteRetentionConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRetentionConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteRetentionConfigurationError::InvalidParameterValue(ref cause) => cause,
            DeleteRetentionConfigurationError::NoSuchRetentionConfiguration(ref cause) => cause,
        }
    }
}
/// Errors returned by DeliverConfigSnapshot
#[derive(Debug, PartialEq)]
pub enum DeliverConfigSnapshotError {
    /// <p>There are no configuration recorders available to provide the role needed to describe your resources. Create a configuration recorder.</p>
    NoAvailableConfigurationRecorder(String),
    /// <p>There is no configuration recorder running.</p>
    NoRunningConfigurationRecorder(String),
    /// <p>You have specified a delivery channel that does not exist.</p>
    NoSuchDeliveryChannel(String),
}

impl DeliverConfigSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeliverConfigSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoAvailableConfigurationRecorderException" => {
                    return RusotoError::Service(
                        DeliverConfigSnapshotError::NoAvailableConfigurationRecorder(err.msg),
                    )
                }
                "NoRunningConfigurationRecorderException" => {
                    return RusotoError::Service(
                        DeliverConfigSnapshotError::NoRunningConfigurationRecorder(err.msg),
                    )
                }
                "NoSuchDeliveryChannelException" => {
                    return RusotoError::Service(DeliverConfigSnapshotError::NoSuchDeliveryChannel(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeliverConfigSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeliverConfigSnapshotError {
    fn description(&self) -> &str {
        match *self {
            DeliverConfigSnapshotError::NoAvailableConfigurationRecorder(ref cause) => cause,
            DeliverConfigSnapshotError::NoRunningConfigurationRecorder(ref cause) => cause,
            DeliverConfigSnapshotError::NoSuchDeliveryChannel(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAggregateComplianceByConfigRules
#[derive(Debug, PartialEq)]
pub enum DescribeAggregateComplianceByConfigRulesError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
}

impl DescribeAggregateComplianceByConfigRulesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAggregateComplianceByConfigRulesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        DescribeAggregateComplianceByConfigRulesError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeAggregateComplianceByConfigRulesError::InvalidNextToken(err.msg),
                    )
                }
                "NoSuchConfigurationAggregatorException" => return RusotoError::Service(
                    DescribeAggregateComplianceByConfigRulesError::NoSuchConfigurationAggregator(
                        err.msg,
                    ),
                ),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeAggregateComplianceByConfigRulesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAggregateComplianceByConfigRulesError {
    fn description(&self) -> &str {
        match *self {
            DescribeAggregateComplianceByConfigRulesError::InvalidLimit(ref cause) => cause,
            DescribeAggregateComplianceByConfigRulesError::InvalidNextToken(ref cause) => cause,
            DescribeAggregateComplianceByConfigRulesError::NoSuchConfigurationAggregator(
                ref cause,
            ) => cause,
        }
    }
}
/// Errors returned by DescribeAggregationAuthorizations
#[derive(Debug, PartialEq)]
pub enum DescribeAggregationAuthorizationsError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
}

impl DescribeAggregationAuthorizationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAggregationAuthorizationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        DescribeAggregationAuthorizationsError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeAggregationAuthorizationsError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeAggregationAuthorizationsError::InvalidParameterValue(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeAggregationAuthorizationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAggregationAuthorizationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeAggregationAuthorizationsError::InvalidLimit(ref cause) => cause,
            DescribeAggregationAuthorizationsError::InvalidNextToken(ref cause) => cause,
            DescribeAggregationAuthorizationsError::InvalidParameterValue(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeComplianceByConfigRule
#[derive(Debug, PartialEq)]
pub enum DescribeComplianceByConfigRuleError {
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
}

impl DescribeComplianceByConfigRuleError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeComplianceByConfigRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeComplianceByConfigRuleError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeComplianceByConfigRuleError::InvalidParameterValue(err.msg),
                    )
                }
                "NoSuchConfigRuleException" => {
                    return RusotoError::Service(
                        DescribeComplianceByConfigRuleError::NoSuchConfigRule(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeComplianceByConfigRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeComplianceByConfigRuleError {
    fn description(&self) -> &str {
        match *self {
            DescribeComplianceByConfigRuleError::InvalidNextToken(ref cause) => cause,
            DescribeComplianceByConfigRuleError::InvalidParameterValue(ref cause) => cause,
            DescribeComplianceByConfigRuleError::NoSuchConfigRule(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeComplianceByResource
#[derive(Debug, PartialEq)]
pub enum DescribeComplianceByResourceError {
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
}

impl DescribeComplianceByResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeComplianceByResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeComplianceByResourceError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeComplianceByResourceError::InvalidParameterValue(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeComplianceByResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeComplianceByResourceError {
    fn description(&self) -> &str {
        match *self {
            DescribeComplianceByResourceError::InvalidNextToken(ref cause) => cause,
            DescribeComplianceByResourceError::InvalidParameterValue(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConfigRuleEvaluationStatus
#[derive(Debug, PartialEq)]
pub enum DescribeConfigRuleEvaluationStatusError {
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
}

impl DescribeConfigRuleEvaluationStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeConfigRuleEvaluationStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeConfigRuleEvaluationStatusError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeConfigRuleEvaluationStatusError::InvalidParameterValue(err.msg),
                    )
                }
                "NoSuchConfigRuleException" => {
                    return RusotoError::Service(
                        DescribeConfigRuleEvaluationStatusError::NoSuchConfigRule(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeConfigRuleEvaluationStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConfigRuleEvaluationStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribeConfigRuleEvaluationStatusError::InvalidNextToken(ref cause) => cause,
            DescribeConfigRuleEvaluationStatusError::InvalidParameterValue(ref cause) => cause,
            DescribeConfigRuleEvaluationStatusError::NoSuchConfigRule(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConfigRules
#[derive(Debug, PartialEq)]
pub enum DescribeConfigRulesError {
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
}

impl DescribeConfigRulesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeConfigRulesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeConfigRulesError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "NoSuchConfigRuleException" => {
                    return RusotoError::Service(DescribeConfigRulesError::NoSuchConfigRule(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeConfigRulesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConfigRulesError {
    fn description(&self) -> &str {
        match *self {
            DescribeConfigRulesError::InvalidNextToken(ref cause) => cause,
            DescribeConfigRulesError::NoSuchConfigRule(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConfigurationAggregatorSourcesStatus
#[derive(Debug, PartialEq)]
pub enum DescribeConfigurationAggregatorSourcesStatusError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
}

impl DescribeConfigurationAggregatorSourcesStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeConfigurationAggregatorSourcesStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                                "InvalidLimitException" => return RusotoError::Service(DescribeConfigurationAggregatorSourcesStatusError::InvalidLimit(err.msg)),
"InvalidNextTokenException" => return RusotoError::Service(DescribeConfigurationAggregatorSourcesStatusError::InvalidNextToken(err.msg)),
"InvalidParameterValueException" => return RusotoError::Service(DescribeConfigurationAggregatorSourcesStatusError::InvalidParameterValue(err.msg)),
"NoSuchConfigurationAggregatorException" => return RusotoError::Service(DescribeConfigurationAggregatorSourcesStatusError::NoSuchConfigurationAggregator(err.msg)),
"ValidationException" => return RusotoError::Validation(err.msg),
_ => {}
                            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeConfigurationAggregatorSourcesStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConfigurationAggregatorSourcesStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribeConfigurationAggregatorSourcesStatusError::InvalidLimit(ref cause) => cause,
            DescribeConfigurationAggregatorSourcesStatusError::InvalidNextToken(ref cause) => cause,
            DescribeConfigurationAggregatorSourcesStatusError::InvalidParameterValue(ref cause) => {
                cause
            }
            DescribeConfigurationAggregatorSourcesStatusError::NoSuchConfigurationAggregator(
                ref cause,
            ) => cause,
        }
    }
}
/// Errors returned by DescribeConfigurationAggregators
#[derive(Debug, PartialEq)]
pub enum DescribeConfigurationAggregatorsError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
}

impl DescribeConfigurationAggregatorsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeConfigurationAggregatorsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        DescribeConfigurationAggregatorsError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeConfigurationAggregatorsError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeConfigurationAggregatorsError::InvalidParameterValue(err.msg),
                    )
                }
                "NoSuchConfigurationAggregatorException" => {
                    return RusotoError::Service(
                        DescribeConfigurationAggregatorsError::NoSuchConfigurationAggregator(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeConfigurationAggregatorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConfigurationAggregatorsError {
    fn description(&self) -> &str {
        match *self {
            DescribeConfigurationAggregatorsError::InvalidLimit(ref cause) => cause,
            DescribeConfigurationAggregatorsError::InvalidNextToken(ref cause) => cause,
            DescribeConfigurationAggregatorsError::InvalidParameterValue(ref cause) => cause,
            DescribeConfigurationAggregatorsError::NoSuchConfigurationAggregator(ref cause) => {
                cause
            }
        }
    }
}
/// Errors returned by DescribeConfigurationRecorderStatus
#[derive(Debug, PartialEq)]
pub enum DescribeConfigurationRecorderStatusError {
    /// <p>You have specified a configuration recorder that does not exist.</p>
    NoSuchConfigurationRecorder(String),
}

impl DescribeConfigurationRecorderStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeConfigurationRecorderStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchConfigurationRecorderException" => {
                    return RusotoError::Service(
                        DescribeConfigurationRecorderStatusError::NoSuchConfigurationRecorder(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeConfigurationRecorderStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConfigurationRecorderStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribeConfigurationRecorderStatusError::NoSuchConfigurationRecorder(ref cause) => {
                cause
            }
        }
    }
}
/// Errors returned by DescribeConfigurationRecorders
#[derive(Debug, PartialEq)]
pub enum DescribeConfigurationRecordersError {
    /// <p>You have specified a configuration recorder that does not exist.</p>
    NoSuchConfigurationRecorder(String),
}

impl DescribeConfigurationRecordersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeConfigurationRecordersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchConfigurationRecorderException" => {
                    return RusotoError::Service(
                        DescribeConfigurationRecordersError::NoSuchConfigurationRecorder(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeConfigurationRecordersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConfigurationRecordersError {
    fn description(&self) -> &str {
        match *self {
            DescribeConfigurationRecordersError::NoSuchConfigurationRecorder(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDeliveryChannelStatus
#[derive(Debug, PartialEq)]
pub enum DescribeDeliveryChannelStatusError {
    /// <p>You have specified a delivery channel that does not exist.</p>
    NoSuchDeliveryChannel(String),
}

impl DescribeDeliveryChannelStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeDeliveryChannelStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchDeliveryChannelException" => {
                    return RusotoError::Service(
                        DescribeDeliveryChannelStatusError::NoSuchDeliveryChannel(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeDeliveryChannelStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDeliveryChannelStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribeDeliveryChannelStatusError::NoSuchDeliveryChannel(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDeliveryChannels
#[derive(Debug, PartialEq)]
pub enum DescribeDeliveryChannelsError {
    /// <p>You have specified a delivery channel that does not exist.</p>
    NoSuchDeliveryChannel(String),
}

impl DescribeDeliveryChannelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDeliveryChannelsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchDeliveryChannelException" => {
                    return RusotoError::Service(
                        DescribeDeliveryChannelsError::NoSuchDeliveryChannel(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeDeliveryChannelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDeliveryChannelsError {
    fn description(&self) -> &str {
        match *self {
            DescribeDeliveryChannelsError::NoSuchDeliveryChannel(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribePendingAggregationRequests
#[derive(Debug, PartialEq)]
pub enum DescribePendingAggregationRequestsError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
}

impl DescribePendingAggregationRequestsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribePendingAggregationRequestsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        DescribePendingAggregationRequestsError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribePendingAggregationRequestsError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribePendingAggregationRequestsError::InvalidParameterValue(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribePendingAggregationRequestsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePendingAggregationRequestsError {
    fn description(&self) -> &str {
        match *self {
            DescribePendingAggregationRequestsError::InvalidLimit(ref cause) => cause,
            DescribePendingAggregationRequestsError::InvalidNextToken(ref cause) => cause,
            DescribePendingAggregationRequestsError::InvalidParameterValue(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeRemediationConfigurations
#[derive(Debug, PartialEq)]
pub enum DescribeRemediationConfigurationsError {}

impl DescribeRemediationConfigurationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeRemediationConfigurationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeRemediationConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeRemediationConfigurationsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DescribeRemediationExecutionStatus
#[derive(Debug, PartialEq)]
pub enum DescribeRemediationExecutionStatusError {
    /// <p>You specified an AWS Config rule without a remediation configuration.</p>
    NoSuchRemediationConfiguration(String),
}

impl DescribeRemediationExecutionStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeRemediationExecutionStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchRemediationConfigurationException" => {
                    return RusotoError::Service(
                        DescribeRemediationExecutionStatusError::NoSuchRemediationConfiguration(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeRemediationExecutionStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeRemediationExecutionStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribeRemediationExecutionStatusError::NoSuchRemediationConfiguration(ref cause) => {
                cause
            }
        }
    }
}
/// Errors returned by DescribeRetentionConfigurations
#[derive(Debug, PartialEq)]
pub enum DescribeRetentionConfigurationsError {
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>You have specified a retention configuration that does not exist.</p>
    NoSuchRetentionConfiguration(String),
}

impl DescribeRetentionConfigurationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeRetentionConfigurationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        DescribeRetentionConfigurationsError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeRetentionConfigurationsError::InvalidParameterValue(err.msg),
                    )
                }
                "NoSuchRetentionConfigurationException" => {
                    return RusotoError::Service(
                        DescribeRetentionConfigurationsError::NoSuchRetentionConfiguration(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeRetentionConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeRetentionConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeRetentionConfigurationsError::InvalidNextToken(ref cause) => cause,
            DescribeRetentionConfigurationsError::InvalidParameterValue(ref cause) => cause,
            DescribeRetentionConfigurationsError::NoSuchRetentionConfiguration(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAggregateComplianceDetailsByConfigRule
#[derive(Debug, PartialEq)]
pub enum GetAggregateComplianceDetailsByConfigRuleError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
}

impl GetAggregateComplianceDetailsByConfigRuleError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetAggregateComplianceDetailsByConfigRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        GetAggregateComplianceDetailsByConfigRuleError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        GetAggregateComplianceDetailsByConfigRuleError::InvalidNextToken(err.msg),
                    )
                }
                "NoSuchConfigurationAggregatorException" => return RusotoError::Service(
                    GetAggregateComplianceDetailsByConfigRuleError::NoSuchConfigurationAggregator(
                        err.msg,
                    ),
                ),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAggregateComplianceDetailsByConfigRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAggregateComplianceDetailsByConfigRuleError {
    fn description(&self) -> &str {
        match *self {
            GetAggregateComplianceDetailsByConfigRuleError::InvalidLimit(ref cause) => cause,
            GetAggregateComplianceDetailsByConfigRuleError::InvalidNextToken(ref cause) => cause,
            GetAggregateComplianceDetailsByConfigRuleError::NoSuchConfigurationAggregator(
                ref cause,
            ) => cause,
        }
    }
}
/// Errors returned by GetAggregateConfigRuleComplianceSummary
#[derive(Debug, PartialEq)]
pub enum GetAggregateConfigRuleComplianceSummaryError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
}

impl GetAggregateConfigRuleComplianceSummaryError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetAggregateConfigRuleComplianceSummaryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        GetAggregateConfigRuleComplianceSummaryError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        GetAggregateConfigRuleComplianceSummaryError::InvalidNextToken(err.msg),
                    )
                }
                "NoSuchConfigurationAggregatorException" => {
                    return RusotoError::Service(
                        GetAggregateConfigRuleComplianceSummaryError::NoSuchConfigurationAggregator(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAggregateConfigRuleComplianceSummaryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAggregateConfigRuleComplianceSummaryError {
    fn description(&self) -> &str {
        match *self {
            GetAggregateConfigRuleComplianceSummaryError::InvalidLimit(ref cause) => cause,
            GetAggregateConfigRuleComplianceSummaryError::InvalidNextToken(ref cause) => cause,
            GetAggregateConfigRuleComplianceSummaryError::NoSuchConfigurationAggregator(
                ref cause,
            ) => cause,
        }
    }
}
/// Errors returned by GetAggregateDiscoveredResourceCounts
#[derive(Debug, PartialEq)]
pub enum GetAggregateDiscoveredResourceCountsError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
}

impl GetAggregateDiscoveredResourceCountsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetAggregateDiscoveredResourceCountsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        GetAggregateDiscoveredResourceCountsError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        GetAggregateDiscoveredResourceCountsError::InvalidNextToken(err.msg),
                    )
                }
                "NoSuchConfigurationAggregatorException" => {
                    return RusotoError::Service(
                        GetAggregateDiscoveredResourceCountsError::NoSuchConfigurationAggregator(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAggregateDiscoveredResourceCountsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAggregateDiscoveredResourceCountsError {
    fn description(&self) -> &str {
        match *self {
            GetAggregateDiscoveredResourceCountsError::InvalidLimit(ref cause) => cause,
            GetAggregateDiscoveredResourceCountsError::InvalidNextToken(ref cause) => cause,
            GetAggregateDiscoveredResourceCountsError::NoSuchConfigurationAggregator(ref cause) => {
                cause
            }
        }
    }
}
/// Errors returned by GetAggregateResourceConfig
#[derive(Debug, PartialEq)]
pub enum GetAggregateResourceConfigError {
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
    /// <p>The configuration item size is outside the allowable range.</p>
    OversizedConfigurationItem(String),
    /// <p>You have specified a resource that is either unknown or has not been discovered.</p>
    ResourceNotDiscovered(String),
}

impl GetAggregateResourceConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetAggregateResourceConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchConfigurationAggregatorException" => {
                    return RusotoError::Service(
                        GetAggregateResourceConfigError::NoSuchConfigurationAggregator(err.msg),
                    )
                }
                "OversizedConfigurationItemException" => {
                    return RusotoError::Service(
                        GetAggregateResourceConfigError::OversizedConfigurationItem(err.msg),
                    )
                }
                "ResourceNotDiscoveredException" => {
                    return RusotoError::Service(
                        GetAggregateResourceConfigError::ResourceNotDiscovered(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAggregateResourceConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAggregateResourceConfigError {
    fn description(&self) -> &str {
        match *self {
            GetAggregateResourceConfigError::NoSuchConfigurationAggregator(ref cause) => cause,
            GetAggregateResourceConfigError::OversizedConfigurationItem(ref cause) => cause,
            GetAggregateResourceConfigError::ResourceNotDiscovered(ref cause) => cause,
        }
    }
}
/// Errors returned by GetComplianceDetailsByConfigRule
#[derive(Debug, PartialEq)]
pub enum GetComplianceDetailsByConfigRuleError {
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
}

impl GetComplianceDetailsByConfigRuleError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetComplianceDetailsByConfigRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        GetComplianceDetailsByConfigRuleError::InvalidNextToken(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetComplianceDetailsByConfigRuleError::InvalidParameterValue(err.msg),
                    )
                }
                "NoSuchConfigRuleException" => {
                    return RusotoError::Service(
                        GetComplianceDetailsByConfigRuleError::NoSuchConfigRule(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetComplianceDetailsByConfigRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetComplianceDetailsByConfigRuleError {
    fn description(&self) -> &str {
        match *self {
            GetComplianceDetailsByConfigRuleError::InvalidNextToken(ref cause) => cause,
            GetComplianceDetailsByConfigRuleError::InvalidParameterValue(ref cause) => cause,
            GetComplianceDetailsByConfigRuleError::NoSuchConfigRule(ref cause) => cause,
        }
    }
}
/// Errors returned by GetComplianceDetailsByResource
#[derive(Debug, PartialEq)]
pub enum GetComplianceDetailsByResourceError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
}

impl GetComplianceDetailsByResourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetComplianceDetailsByResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetComplianceDetailsByResourceError::InvalidParameterValue(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetComplianceDetailsByResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetComplianceDetailsByResourceError {
    fn description(&self) -> &str {
        match *self {
            GetComplianceDetailsByResourceError::InvalidParameterValue(ref cause) => cause,
        }
    }
}
/// Errors returned by GetComplianceSummaryByConfigRule
#[derive(Debug, PartialEq)]
pub enum GetComplianceSummaryByConfigRuleError {}

impl GetComplianceSummaryByConfigRuleError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetComplianceSummaryByConfigRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetComplianceSummaryByConfigRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetComplianceSummaryByConfigRuleError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by GetComplianceSummaryByResourceType
#[derive(Debug, PartialEq)]
pub enum GetComplianceSummaryByResourceTypeError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
}

impl GetComplianceSummaryByResourceTypeError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetComplianceSummaryByResourceTypeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetComplianceSummaryByResourceTypeError::InvalidParameterValue(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetComplianceSummaryByResourceTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetComplianceSummaryByResourceTypeError {
    fn description(&self) -> &str {
        match *self {
            GetComplianceSummaryByResourceTypeError::InvalidParameterValue(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDiscoveredResourceCounts
#[derive(Debug, PartialEq)]
pub enum GetDiscoveredResourceCountsError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
}

impl GetDiscoveredResourceCountsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetDiscoveredResourceCountsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(GetDiscoveredResourceCountsError::InvalidLimit(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        GetDiscoveredResourceCountsError::InvalidNextToken(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDiscoveredResourceCountsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDiscoveredResourceCountsError {
    fn description(&self) -> &str {
        match *self {
            GetDiscoveredResourceCountsError::InvalidLimit(ref cause) => cause,
            GetDiscoveredResourceCountsError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by GetResourceConfigHistory
#[derive(Debug, PartialEq)]
pub enum GetResourceConfigHistoryError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>The specified time range is not valid. The earlier time is not chronologically before the later time.</p>
    InvalidTimeRange(String),
    /// <p>There are no configuration recorders available to provide the role needed to describe your resources. Create a configuration recorder.</p>
    NoAvailableConfigurationRecorder(String),
    /// <p>You have specified a resource that is either unknown or has not been discovered.</p>
    ResourceNotDiscovered(String),
}

impl GetResourceConfigHistoryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResourceConfigHistoryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(GetResourceConfigHistoryError::InvalidLimit(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(GetResourceConfigHistoryError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "InvalidTimeRangeException" => {
                    return RusotoError::Service(GetResourceConfigHistoryError::InvalidTimeRange(
                        err.msg,
                    ))
                }
                "NoAvailableConfigurationRecorderException" => {
                    return RusotoError::Service(
                        GetResourceConfigHistoryError::NoAvailableConfigurationRecorder(err.msg),
                    )
                }
                "ResourceNotDiscoveredException" => {
                    return RusotoError::Service(
                        GetResourceConfigHistoryError::ResourceNotDiscovered(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetResourceConfigHistoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetResourceConfigHistoryError {
    fn description(&self) -> &str {
        match *self {
            GetResourceConfigHistoryError::InvalidLimit(ref cause) => cause,
            GetResourceConfigHistoryError::InvalidNextToken(ref cause) => cause,
            GetResourceConfigHistoryError::InvalidTimeRange(ref cause) => cause,
            GetResourceConfigHistoryError::NoAvailableConfigurationRecorder(ref cause) => cause,
            GetResourceConfigHistoryError::ResourceNotDiscovered(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAggregateDiscoveredResources
#[derive(Debug, PartialEq)]
pub enum ListAggregateDiscoveredResourcesError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>You have specified a configuration aggregator that does not exist.</p>
    NoSuchConfigurationAggregator(String),
}

impl ListAggregateDiscoveredResourcesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListAggregateDiscoveredResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(
                        ListAggregateDiscoveredResourcesError::InvalidLimit(err.msg),
                    )
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(
                        ListAggregateDiscoveredResourcesError::InvalidNextToken(err.msg),
                    )
                }
                "NoSuchConfigurationAggregatorException" => {
                    return RusotoError::Service(
                        ListAggregateDiscoveredResourcesError::NoSuchConfigurationAggregator(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAggregateDiscoveredResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAggregateDiscoveredResourcesError {
    fn description(&self) -> &str {
        match *self {
            ListAggregateDiscoveredResourcesError::InvalidLimit(ref cause) => cause,
            ListAggregateDiscoveredResourcesError::InvalidNextToken(ref cause) => cause,
            ListAggregateDiscoveredResourcesError::NoSuchConfigurationAggregator(ref cause) => {
                cause
            }
        }
    }
}
/// Errors returned by ListDiscoveredResources
#[derive(Debug, PartialEq)]
pub enum ListDiscoveredResourcesError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>There are no configuration recorders available to provide the role needed to describe your resources. Create a configuration recorder.</p>
    NoAvailableConfigurationRecorder(String),
}

impl ListDiscoveredResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDiscoveredResourcesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(ListDiscoveredResourcesError::InvalidLimit(
                        err.msg,
                    ))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListDiscoveredResourcesError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "NoAvailableConfigurationRecorderException" => {
                    return RusotoError::Service(
                        ListDiscoveredResourcesError::NoAvailableConfigurationRecorder(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListDiscoveredResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDiscoveredResourcesError {
    fn description(&self) -> &str {
        match *self {
            ListDiscoveredResourcesError::InvalidLimit(ref cause) => cause,
            ListDiscoveredResourcesError::InvalidNextToken(ref cause) => cause,
            ListDiscoveredResourcesError::NoAvailableConfigurationRecorder(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
    /// <p>You have specified a resource that does not exist.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidLimitException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidLimit(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTagsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForResourceError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForResourceError::InvalidLimit(ref cause) => cause,
            ListTagsForResourceError::InvalidNextToken(ref cause) => cause,
            ListTagsForResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by PutAggregationAuthorization
#[derive(Debug, PartialEq)]
pub enum PutAggregationAuthorizationError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
}

impl PutAggregationAuthorizationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutAggregationAuthorizationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        PutAggregationAuthorizationError::InvalidParameterValue(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutAggregationAuthorizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutAggregationAuthorizationError {
    fn description(&self) -> &str {
        match *self {
            PutAggregationAuthorizationError::InvalidParameterValue(ref cause) => cause,
        }
    }
}
/// Errors returned by PutConfigRule
#[derive(Debug, PartialEq)]
pub enum PutConfigRuleError {
    /// <p><p>Indicates one of the following errors:</p> <ul> <li> <p>The rule cannot be created because the IAM role assigned to AWS Config lacks permissions to perform the config:Put* action.</p> </li> <li> <p>The AWS Lambda function cannot be invoked. Check the function ARN, and check the function&#39;s permissions.</p> </li> </ul></p>
    InsufficientPermissions(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>Failed to add the AWS Config rule because the account already contains the maximum number of 150 rules. Consider deleting any deactivated rules before you add new rules.</p>
    MaxNumberOfConfigRulesExceeded(String),
    /// <p>There are no configuration recorders available to provide the role needed to describe your resources. Create a configuration recorder.</p>
    NoAvailableConfigurationRecorder(String),
    /// <p>The rule is currently being deleted or the rule is deleting your evaluation results. Try your request again later.</p>
    ResourceInUse(String),
}

impl PutConfigRuleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutConfigRuleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InsufficientPermissionsException" => {
                    return RusotoError::Service(PutConfigRuleError::InsufficientPermissions(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(PutConfigRuleError::InvalidParameterValue(err.msg))
                }
                "MaxNumberOfConfigRulesExceededException" => {
                    return RusotoError::Service(
                        PutConfigRuleError::MaxNumberOfConfigRulesExceeded(err.msg),
                    )
                }
                "NoAvailableConfigurationRecorderException" => {
                    return RusotoError::Service(
                        PutConfigRuleError::NoAvailableConfigurationRecorder(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(PutConfigRuleError::ResourceInUse(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutConfigRuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutConfigRuleError {
    fn description(&self) -> &str {
        match *self {
            PutConfigRuleError::InsufficientPermissions(ref cause) => cause,
            PutConfigRuleError::InvalidParameterValue(ref cause) => cause,
            PutConfigRuleError::MaxNumberOfConfigRulesExceeded(ref cause) => cause,
            PutConfigRuleError::NoAvailableConfigurationRecorder(ref cause) => cause,
            PutConfigRuleError::ResourceInUse(ref cause) => cause,
        }
    }
}
/// Errors returned by PutConfigurationAggregator
#[derive(Debug, PartialEq)]
pub enum PutConfigurationAggregatorError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>You have provided a null or empty role ARN.</p>
    InvalidRole(String),
    /// <p>For <code>StartConfigRulesEvaluation</code> API, this exception is thrown if an evaluation is in progress or if you call the <a>StartConfigRulesEvaluation</a> API more than once per minute.</p> <p>For <code>PutConfigurationAggregator</code> API, this exception is thrown if the number of accounts and aggregators exceeds the limit.</p>
    LimitExceeded(String),
    /// <p>Organization does is no longer available.</p>
    NoAvailableOrganization(String),
    /// <p>No permission to call the EnableAWSServiceAccess API.</p>
    OrganizationAccessDenied(String),
    /// <p>The configuration aggregator cannot be created because organization does not have all features enabled.</p>
    OrganizationAllFeaturesNotEnabled(String),
}

impl PutConfigurationAggregatorError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutConfigurationAggregatorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        PutConfigurationAggregatorError::InvalidParameterValue(err.msg),
                    )
                }
                "InvalidRoleException" => {
                    return RusotoError::Service(PutConfigurationAggregatorError::InvalidRole(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutConfigurationAggregatorError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NoAvailableOrganizationException" => {
                    return RusotoError::Service(
                        PutConfigurationAggregatorError::NoAvailableOrganization(err.msg),
                    )
                }
                "OrganizationAccessDeniedException" => {
                    return RusotoError::Service(
                        PutConfigurationAggregatorError::OrganizationAccessDenied(err.msg),
                    )
                }
                "OrganizationAllFeaturesNotEnabledException" => {
                    return RusotoError::Service(
                        PutConfigurationAggregatorError::OrganizationAllFeaturesNotEnabled(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutConfigurationAggregatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutConfigurationAggregatorError {
    fn description(&self) -> &str {
        match *self {
            PutConfigurationAggregatorError::InvalidParameterValue(ref cause) => cause,
            PutConfigurationAggregatorError::InvalidRole(ref cause) => cause,
            PutConfigurationAggregatorError::LimitExceeded(ref cause) => cause,
            PutConfigurationAggregatorError::NoAvailableOrganization(ref cause) => cause,
            PutConfigurationAggregatorError::OrganizationAccessDenied(ref cause) => cause,
            PutConfigurationAggregatorError::OrganizationAllFeaturesNotEnabled(ref cause) => cause,
        }
    }
}
/// Errors returned by PutConfigurationRecorder
#[derive(Debug, PartialEq)]
pub enum PutConfigurationRecorderError {
    /// <p>You have provided a configuration recorder name that is not valid.</p>
    InvalidConfigurationRecorderName(String),
    /// <p>AWS Config throws an exception if the recording group does not contain a valid list of resource types. Invalid values might also be incorrectly formatted.</p>
    InvalidRecordingGroup(String),
    /// <p>You have provided a null or empty role ARN.</p>
    InvalidRole(String),
    /// <p>You have reached the limit of the number of recorders you can create.</p>
    MaxNumberOfConfigurationRecordersExceeded(String),
}

impl PutConfigurationRecorderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutConfigurationRecorderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidConfigurationRecorderNameException" => {
                    return RusotoError::Service(
                        PutConfigurationRecorderError::InvalidConfigurationRecorderName(err.msg),
                    )
                }
                "InvalidRecordingGroupException" => {
                    return RusotoError::Service(
                        PutConfigurationRecorderError::InvalidRecordingGroup(err.msg),
                    )
                }
                "InvalidRoleException" => {
                    return RusotoError::Service(PutConfigurationRecorderError::InvalidRole(
                        err.msg,
                    ))
                }
                "MaxNumberOfConfigurationRecordersExceededException" => {
                    return RusotoError::Service(
                        PutConfigurationRecorderError::MaxNumberOfConfigurationRecordersExceeded(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutConfigurationRecorderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutConfigurationRecorderError {
    fn description(&self) -> &str {
        match *self {
            PutConfigurationRecorderError::InvalidConfigurationRecorderName(ref cause) => cause,
            PutConfigurationRecorderError::InvalidRecordingGroup(ref cause) => cause,
            PutConfigurationRecorderError::InvalidRole(ref cause) => cause,
            PutConfigurationRecorderError::MaxNumberOfConfigurationRecordersExceeded(ref cause) => {
                cause
            }
        }
    }
}
/// Errors returned by PutDeliveryChannel
#[derive(Debug, PartialEq)]
pub enum PutDeliveryChannelError {
    /// <p>Your Amazon S3 bucket policy does not permit AWS Config to write to it.</p>
    InsufficientDeliveryPolicy(String),
    /// <p>The specified delivery channel name is not valid.</p>
    InvalidDeliveryChannelName(String),
    /// <p>The specified Amazon S3 key prefix is not valid.</p>
    InvalidS3KeyPrefix(String),
    /// <p>The specified Amazon SNS topic does not exist.</p>
    InvalidSNSTopicARN(String),
    /// <p>You have reached the limit of the number of delivery channels you can create.</p>
    MaxNumberOfDeliveryChannelsExceeded(String),
    /// <p>There are no configuration recorders available to provide the role needed to describe your resources. Create a configuration recorder.</p>
    NoAvailableConfigurationRecorder(String),
    /// <p>The specified Amazon S3 bucket does not exist.</p>
    NoSuchBucket(String),
}

impl PutDeliveryChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutDeliveryChannelError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InsufficientDeliveryPolicyException" => {
                    return RusotoError::Service(
                        PutDeliveryChannelError::InsufficientDeliveryPolicy(err.msg),
                    )
                }
                "InvalidDeliveryChannelNameException" => {
                    return RusotoError::Service(
                        PutDeliveryChannelError::InvalidDeliveryChannelName(err.msg),
                    )
                }
                "InvalidS3KeyPrefixException" => {
                    return RusotoError::Service(PutDeliveryChannelError::InvalidS3KeyPrefix(
                        err.msg,
                    ))
                }
                "InvalidSNSTopicARNException" => {
                    return RusotoError::Service(PutDeliveryChannelError::InvalidSNSTopicARN(
                        err.msg,
                    ))
                }
                "MaxNumberOfDeliveryChannelsExceededException" => {
                    return RusotoError::Service(
                        PutDeliveryChannelError::MaxNumberOfDeliveryChannelsExceeded(err.msg),
                    )
                }
                "NoAvailableConfigurationRecorderException" => {
                    return RusotoError::Service(
                        PutDeliveryChannelError::NoAvailableConfigurationRecorder(err.msg),
                    )
                }
                "NoSuchBucketException" => {
                    return RusotoError::Service(PutDeliveryChannelError::NoSuchBucket(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutDeliveryChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutDeliveryChannelError {
    fn description(&self) -> &str {
        match *self {
            PutDeliveryChannelError::InsufficientDeliveryPolicy(ref cause) => cause,
            PutDeliveryChannelError::InvalidDeliveryChannelName(ref cause) => cause,
            PutDeliveryChannelError::InvalidS3KeyPrefix(ref cause) => cause,
            PutDeliveryChannelError::InvalidSNSTopicARN(ref cause) => cause,
            PutDeliveryChannelError::MaxNumberOfDeliveryChannelsExceeded(ref cause) => cause,
            PutDeliveryChannelError::NoAvailableConfigurationRecorder(ref cause) => cause,
            PutDeliveryChannelError::NoSuchBucket(ref cause) => cause,
        }
    }
}
/// Errors returned by PutEvaluations
#[derive(Debug, PartialEq)]
pub enum PutEvaluationsError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>The specified <code>ResultToken</code> is invalid.</p>
    InvalidResultToken(String),
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
}

impl PutEvaluationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutEvaluationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(PutEvaluationsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "InvalidResultTokenException" => {
                    return RusotoError::Service(PutEvaluationsError::InvalidResultToken(err.msg))
                }
                "NoSuchConfigRuleException" => {
                    return RusotoError::Service(PutEvaluationsError::NoSuchConfigRule(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutEvaluationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutEvaluationsError {
    fn description(&self) -> &str {
        match *self {
            PutEvaluationsError::InvalidParameterValue(ref cause) => cause,
            PutEvaluationsError::InvalidResultToken(ref cause) => cause,
            PutEvaluationsError::NoSuchConfigRule(ref cause) => cause,
        }
    }
}
/// Errors returned by PutRemediationConfigurations
#[derive(Debug, PartialEq)]
pub enum PutRemediationConfigurationsError {
    /// <p><p>Indicates one of the following errors:</p> <ul> <li> <p>The rule cannot be created because the IAM role assigned to AWS Config lacks permissions to perform the config:Put* action.</p> </li> <li> <p>The AWS Lambda function cannot be invoked. Check the function ARN, and check the function&#39;s permissions.</p> </li> </ul></p>
    InsufficientPermissions(String),
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
}

impl PutRemediationConfigurationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutRemediationConfigurationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InsufficientPermissionsException" => {
                    return RusotoError::Service(
                        PutRemediationConfigurationsError::InsufficientPermissions(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        PutRemediationConfigurationsError::InvalidParameterValue(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutRemediationConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutRemediationConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            PutRemediationConfigurationsError::InsufficientPermissions(ref cause) => cause,
            PutRemediationConfigurationsError::InvalidParameterValue(ref cause) => cause,
        }
    }
}
/// Errors returned by PutRetentionConfiguration
#[derive(Debug, PartialEq)]
pub enum PutRetentionConfigurationError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>Failed to add the retention configuration because a retention configuration with that name already exists.</p>
    MaxNumberOfRetentionConfigurationsExceeded(String),
}

impl PutRetentionConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutRetentionConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        PutRetentionConfigurationError::InvalidParameterValue(err.msg),
                    )
                }
                "MaxNumberOfRetentionConfigurationsExceededException" => {
                    return RusotoError::Service(
                        PutRetentionConfigurationError::MaxNumberOfRetentionConfigurationsExceeded(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutRetentionConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutRetentionConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutRetentionConfigurationError::InvalidParameterValue(ref cause) => cause,
            PutRetentionConfigurationError::MaxNumberOfRetentionConfigurationsExceeded(
                ref cause,
            ) => cause,
        }
    }
}
/// Errors returned by SelectResourceConfig
#[derive(Debug, PartialEq)]
pub enum SelectResourceConfigError {
    /// <p>The syntax of the query is incorrect.</p>
    InvalidExpression(String),
    /// <p>The specified limit is outside the allowable range.</p>
    InvalidLimit(String),
    /// <p>The specified next token is invalid. Specify the <code>nextToken</code> string that was returned in the previous response to get the next page of results.</p>
    InvalidNextToken(String),
}

impl SelectResourceConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SelectResourceConfigError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidExpressionException" => {
                    return RusotoError::Service(SelectResourceConfigError::InvalidExpression(
                        err.msg,
                    ))
                }
                "InvalidLimitException" => {
                    return RusotoError::Service(SelectResourceConfigError::InvalidLimit(err.msg))
                }
                "InvalidNextTokenException" => {
                    return RusotoError::Service(SelectResourceConfigError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SelectResourceConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SelectResourceConfigError {
    fn description(&self) -> &str {
        match *self {
            SelectResourceConfigError::InvalidExpression(ref cause) => cause,
            SelectResourceConfigError::InvalidLimit(ref cause) => cause,
            SelectResourceConfigError::InvalidNextToken(ref cause) => cause,
        }
    }
}
/// Errors returned by StartConfigRulesEvaluation
#[derive(Debug, PartialEq)]
pub enum StartConfigRulesEvaluationError {
    /// <p>One or more of the specified parameters are invalid. Verify that your parameters are valid and try again.</p>
    InvalidParameterValue(String),
    /// <p>For <code>StartConfigRulesEvaluation</code> API, this exception is thrown if an evaluation is in progress or if you call the <a>StartConfigRulesEvaluation</a> API more than once per minute.</p> <p>For <code>PutConfigurationAggregator</code> API, this exception is thrown if the number of accounts and aggregators exceeds the limit.</p>
    LimitExceeded(String),
    /// <p>One or more AWS Config rules in the request are invalid. Verify that the rule names are correct and try again.</p>
    NoSuchConfigRule(String),
    /// <p>The rule is currently being deleted or the rule is deleting your evaluation results. Try your request again later.</p>
    ResourceInUse(String),
}

impl StartConfigRulesEvaluationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartConfigRulesEvaluationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        StartConfigRulesEvaluationError::InvalidParameterValue(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartConfigRulesEvaluationError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NoSuchConfigRuleException" => {
                    return RusotoError::Service(StartConfigRulesEvaluationError::NoSuchConfigRule(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(StartConfigRulesEvaluationError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartConfigRulesEvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartConfigRulesEvaluationError {
    fn description(&self) -> &str {
        match *self {
            StartConfigRulesEvaluationError::InvalidParameterValue(ref cause) => cause,
            StartConfigRulesEvaluationError::LimitExceeded(ref cause) => cause,
            StartConfigRulesEvaluationError::NoSuchConfigRule(ref cause) => cause,
            StartConfigRulesEvaluationError::ResourceInUse(ref cause) => cause,
        }
    }
}
/// Errors returned by StartConfigurationRecorder
#[derive(Debug, PartialEq)]
pub enum StartConfigurationRecorderError {
    /// <p>There is no delivery channel available to record configurations.</p>
    NoAvailableDeliveryChannel(String),
    /// <p>You have specified a configuration recorder that does not exist.</p>
    NoSuchConfigurationRecorder(String),
}

impl StartConfigurationRecorderError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartConfigurationRecorderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoAvailableDeliveryChannelException" => {
                    return RusotoError::Service(
                        StartConfigurationRecorderError::NoAvailableDeliveryChannel(err.msg),
                    )
                }
                "NoSuchConfigurationRecorderException" => {
                    return RusotoError::Service(
                        StartConfigurationRecorderError::NoSuchConfigurationRecorder(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartConfigurationRecorderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartConfigurationRecorderError {
    fn description(&self) -> &str {
        match *self {
            StartConfigurationRecorderError::NoAvailableDeliveryChannel(ref cause) => cause,
            StartConfigurationRecorderError::NoSuchConfigurationRecorder(ref cause) => cause,
        }
    }
}
/// Errors returned by StartRemediationExecution
#[derive(Debug, PartialEq)]
pub enum StartRemediationExecutionError {
    /// <p><p>Indicates one of the following errors:</p> <ul> <li> <p>The rule cannot be created because the IAM role assigned to AWS Config lacks permissions to perform the config:Put* action.</p> </li> <li> <p>The AWS Lambda function cannot be invoked. Check the function ARN, and check the function&#39;s permissions.</p> </li> </ul></p>
    InsufficientPermissions(String),
    /// <p>You specified an AWS Config rule without a remediation configuration.</p>
    NoSuchRemediationConfiguration(String),
}

impl StartRemediationExecutionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartRemediationExecutionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InsufficientPermissionsException" => {
                    return RusotoError::Service(
                        StartRemediationExecutionError::InsufficientPermissions(err.msg),
                    )
                }
                "NoSuchRemediationConfigurationException" => {
                    return RusotoError::Service(
                        StartRemediationExecutionError::NoSuchRemediationConfiguration(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartRemediationExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartRemediationExecutionError {
    fn description(&self) -> &str {
        match *self {
            StartRemediationExecutionError::InsufficientPermissions(ref cause) => cause,
            StartRemediationExecutionError::NoSuchRemediationConfiguration(ref cause) => cause,
        }
    }
}
/// Errors returned by StopConfigurationRecorder
#[derive(Debug, PartialEq)]
pub enum StopConfigurationRecorderError {
    /// <p>You have specified a configuration recorder that does not exist.</p>
    NoSuchConfigurationRecorder(String),
}

impl StopConfigurationRecorderError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopConfigurationRecorderError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "NoSuchConfigurationRecorderException" => {
                    return RusotoError::Service(
                        StopConfigurationRecorderError::NoSuchConfigurationRecorder(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopConfigurationRecorderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopConfigurationRecorderError {
    fn description(&self) -> &str {
        match *self {
            StopConfigurationRecorderError::NoSuchConfigurationRecorder(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>You have specified a resource that does not exist.</p>
    ResourceNotFound(String),
    /// <p>You have reached the limit of the number of tags you can use. You have more than 50 tags.</p>
    TooManyTags(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(TagResourceError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::ResourceNotFound(ref cause) => cause,
            TagResourceError::TooManyTags(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>You have specified a resource that does not exist.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::ResourceNotFound(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Config Service API. Config Service clients implement this trait.
pub trait ConfigService {
    /// <p><p>Returns the current configuration items for resources that are present in your AWS Config aggregator. The operation also returns a list of resources that are not processed in the current request. If there are no unprocessed resources, the operation returns an empty <code>unprocessedResourceIdentifiers</code> list. </p> <note> <ul> <li> <p>The API does not return results for deleted resources.</p> </li> <li> <p> The API does not return tags and relationships.</p> </li> </ul> </note></p>
    fn batch_get_aggregate_resource_config(
        &self,
        input: BatchGetAggregateResourceConfigRequest,
    ) -> RusotoFuture<BatchGetAggregateResourceConfigResponse, BatchGetAggregateResourceConfigError>;

    /// <p><p>Returns the current configuration for one or more requested resources. The operation also returns a list of resources that are not processed in the current request. If there are no unprocessed resources, the operation returns an empty unprocessedResourceKeys list. </p> <note> <ul> <li> <p>The API does not return results for deleted resources.</p> </li> <li> <p> The API does not return any tags for the requested resources. This information is filtered out of the supplementaryConfiguration section of the API response.</p> </li> </ul> </note></p>
    fn batch_get_resource_config(
        &self,
        input: BatchGetResourceConfigRequest,
    ) -> RusotoFuture<BatchGetResourceConfigResponse, BatchGetResourceConfigError>;

    /// <p>Deletes the authorization granted to the specified configuration aggregator account in a specified region.</p>
    fn delete_aggregation_authorization(
        &self,
        input: DeleteAggregationAuthorizationRequest,
    ) -> RusotoFuture<(), DeleteAggregationAuthorizationError>;

    /// <p>Deletes the specified AWS Config rule and all of its evaluation results.</p> <p>AWS Config sets the state of a rule to <code>DELETING</code> until the deletion is complete. You cannot update a rule while it is in this state. If you make a <code>PutConfigRule</code> or <code>DeleteConfigRule</code> request for the rule, you will receive a <code>ResourceInUseException</code>.</p> <p>You can check the state of a rule by using the <code>DescribeConfigRules</code> request.</p>
    fn delete_config_rule(
        &self,
        input: DeleteConfigRuleRequest,
    ) -> RusotoFuture<(), DeleteConfigRuleError>;

    /// <p>Deletes the specified configuration aggregator and the aggregated data associated with the aggregator.</p>
    fn delete_configuration_aggregator(
        &self,
        input: DeleteConfigurationAggregatorRequest,
    ) -> RusotoFuture<(), DeleteConfigurationAggregatorError>;

    /// <p>Deletes the configuration recorder.</p> <p>After the configuration recorder is deleted, AWS Config will not record resource configuration changes until you create a new configuration recorder.</p> <p>This action does not delete the configuration information that was previously recorded. You will be able to access the previously recorded information by using the <code>GetResourceConfigHistory</code> action, but you will not be able to access this information in the AWS Config console until you create a new configuration recorder.</p>
    fn delete_configuration_recorder(
        &self,
        input: DeleteConfigurationRecorderRequest,
    ) -> RusotoFuture<(), DeleteConfigurationRecorderError>;

    /// <p>Deletes the delivery channel.</p> <p>Before you can delete the delivery channel, you must stop the configuration recorder by using the <a>StopConfigurationRecorder</a> action.</p>
    fn delete_delivery_channel(
        &self,
        input: DeleteDeliveryChannelRequest,
    ) -> RusotoFuture<(), DeleteDeliveryChannelError>;

    /// <p>Deletes the evaluation results for the specified AWS Config rule. You can specify one AWS Config rule per request. After you delete the evaluation results, you can call the <a>StartConfigRulesEvaluation</a> API to start evaluating your AWS resources against the rule.</p>
    fn delete_evaluation_results(
        &self,
        input: DeleteEvaluationResultsRequest,
    ) -> RusotoFuture<DeleteEvaluationResultsResponse, DeleteEvaluationResultsError>;

    /// <p>Deletes pending authorization requests for a specified aggregator account in a specified region.</p>
    fn delete_pending_aggregation_request(
        &self,
        input: DeletePendingAggregationRequestRequest,
    ) -> RusotoFuture<(), DeletePendingAggregationRequestError>;

    /// <p>Deletes the remediation configuration.</p>
    fn delete_remediation_configuration(
        &self,
        input: DeleteRemediationConfigurationRequest,
    ) -> RusotoFuture<DeleteRemediationConfigurationResponse, DeleteRemediationConfigurationError>;

    /// <p>Deletes the retention configuration.</p>
    fn delete_retention_configuration(
        &self,
        input: DeleteRetentionConfigurationRequest,
    ) -> RusotoFuture<(), DeleteRetentionConfigurationError>;

    /// <p><p>Schedules delivery of a configuration snapshot to the Amazon S3 bucket in the specified delivery channel. After the delivery has started, AWS Config sends the following notifications using an Amazon SNS topic that you have specified.</p> <ul> <li> <p>Notification of the start of the delivery.</p> </li> <li> <p>Notification of the completion of the delivery, if the delivery was successfully completed.</p> </li> <li> <p>Notification of delivery failure, if the delivery failed.</p> </li> </ul></p>
    fn deliver_config_snapshot(
        &self,
        input: DeliverConfigSnapshotRequest,
    ) -> RusotoFuture<DeliverConfigSnapshotResponse, DeliverConfigSnapshotError>;

    /// <p><p>Returns a list of compliant and noncompliant rules with the number of resources for compliant and noncompliant rules. </p> <note> <p>The results can return an empty result page, but if you have a nextToken, the results are displayed on the next page.</p> </note></p>
    fn describe_aggregate_compliance_by_config_rules(
        &self,
        input: DescribeAggregateComplianceByConfigRulesRequest,
    ) -> RusotoFuture<
        DescribeAggregateComplianceByConfigRulesResponse,
        DescribeAggregateComplianceByConfigRulesError,
    >;

    /// <p>Returns a list of authorizations granted to various aggregator accounts and regions.</p>
    fn describe_aggregation_authorizations(
        &self,
        input: DescribeAggregationAuthorizationsRequest,
    ) -> RusotoFuture<
        DescribeAggregationAuthorizationsResponse,
        DescribeAggregationAuthorizationsError,
    >;

    /// <p><p>Indicates whether the specified AWS Config rules are compliant. If a rule is noncompliant, this action returns the number of AWS resources that do not comply with the rule.</p> <p>A rule is compliant if all of the evaluated resources comply with it. It is noncompliant if any of these resources do not comply.</p> <p>If AWS Config has no current evaluation results for the rule, it returns <code>INSUFFICIENT<em>DATA</code>. This result might indicate one of the following conditions:</p> <ul> <li> <p>AWS Config has never invoked an evaluation for the rule. To check whether it has, use the <code>DescribeConfigRuleEvaluationStatus</code> action to get the <code>LastSuccessfulInvocationTime</code> and <code>LastFailedInvocationTime</code>.</p> </li> <li> <p>The rule&#39;s AWS Lambda function is failing to send evaluation results to AWS Config. Verify that the role you assigned to your configuration recorder includes the <code>config:PutEvaluations</code> permission. If the rule is a custom rule, verify that the AWS Lambda execution role includes the <code>config:PutEvaluations</code> permission.</p> </li> <li> <p>The rule&#39;s AWS Lambda function has returned <code>NOT</em>APPLICABLE</code> for all evaluation results. This can occur if the resources were deleted or removed from the rule&#39;s scope.</p> </li> </ul></p>
    fn describe_compliance_by_config_rule(
        &self,
        input: DescribeComplianceByConfigRuleRequest,
    ) -> RusotoFuture<DescribeComplianceByConfigRuleResponse, DescribeComplianceByConfigRuleError>;

    /// <p><p>Indicates whether the specified AWS resources are compliant. If a resource is noncompliant, this action returns the number of AWS Config rules that the resource does not comply with.</p> <p>A resource is compliant if it complies with all the AWS Config rules that evaluate it. It is noncompliant if it does not comply with one or more of these rules.</p> <p>If AWS Config has no current evaluation results for the resource, it returns <code>INSUFFICIENT<em>DATA</code>. This result might indicate one of the following conditions about the rules that evaluate the resource:</p> <ul> <li> <p>AWS Config has never invoked an evaluation for the rule. To check whether it has, use the <code>DescribeConfigRuleEvaluationStatus</code> action to get the <code>LastSuccessfulInvocationTime</code> and <code>LastFailedInvocationTime</code>.</p> </li> <li> <p>The rule&#39;s AWS Lambda function is failing to send evaluation results to AWS Config. Verify that the role that you assigned to your configuration recorder includes the <code>config:PutEvaluations</code> permission. If the rule is a custom rule, verify that the AWS Lambda execution role includes the <code>config:PutEvaluations</code> permission.</p> </li> <li> <p>The rule&#39;s AWS Lambda function has returned <code>NOT</em>APPLICABLE</code> for all evaluation results. This can occur if the resources were deleted or removed from the rule&#39;s scope.</p> </li> </ul></p>
    fn describe_compliance_by_resource(
        &self,
        input: DescribeComplianceByResourceRequest,
    ) -> RusotoFuture<DescribeComplianceByResourceResponse, DescribeComplianceByResourceError>;

    /// <p>Returns status information for each of your AWS managed Config rules. The status includes information such as the last time AWS Config invoked the rule, the last time AWS Config failed to invoke the rule, and the related error for the last failure.</p>
    fn describe_config_rule_evaluation_status(
        &self,
        input: DescribeConfigRuleEvaluationStatusRequest,
    ) -> RusotoFuture<
        DescribeConfigRuleEvaluationStatusResponse,
        DescribeConfigRuleEvaluationStatusError,
    >;

    /// <p>Returns details about your AWS Config rules.</p>
    fn describe_config_rules(
        &self,
        input: DescribeConfigRulesRequest,
    ) -> RusotoFuture<DescribeConfigRulesResponse, DescribeConfigRulesError>;

    /// <p>Returns status information for sources within an aggregator. The status includes information about the last time AWS Config verified authorization between the source account and an aggregator account. In case of a failure, the status contains the related error code or message. </p>
    fn describe_configuration_aggregator_sources_status(
        &self,
        input: DescribeConfigurationAggregatorSourcesStatusRequest,
    ) -> RusotoFuture<
        DescribeConfigurationAggregatorSourcesStatusResponse,
        DescribeConfigurationAggregatorSourcesStatusError,
    >;

    /// <p>Returns the details of one or more configuration aggregators. If the configuration aggregator is not specified, this action returns the details for all the configuration aggregators associated with the account. </p>
    fn describe_configuration_aggregators(
        &self,
        input: DescribeConfigurationAggregatorsRequest,
    ) -> RusotoFuture<DescribeConfigurationAggregatorsResponse, DescribeConfigurationAggregatorsError>;

    /// <p><p>Returns the current status of the specified configuration recorder. If a configuration recorder is not specified, this action returns the status of all configuration recorders associated with the account.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> </note></p>
    fn describe_configuration_recorder_status(
        &self,
        input: DescribeConfigurationRecorderStatusRequest,
    ) -> RusotoFuture<
        DescribeConfigurationRecorderStatusResponse,
        DescribeConfigurationRecorderStatusError,
    >;

    /// <p><p>Returns the details for the specified configuration recorders. If the configuration recorder is not specified, this action returns the details for all configuration recorders associated with the account.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> </note></p>
    fn describe_configuration_recorders(
        &self,
        input: DescribeConfigurationRecordersRequest,
    ) -> RusotoFuture<DescribeConfigurationRecordersResponse, DescribeConfigurationRecordersError>;

    /// <p><p>Returns the current status of the specified delivery channel. If a delivery channel is not specified, this action returns the current status of all delivery channels associated with the account.</p> <note> <p>Currently, you can specify only one delivery channel per region in your account.</p> </note></p>
    fn describe_delivery_channel_status(
        &self,
        input: DescribeDeliveryChannelStatusRequest,
    ) -> RusotoFuture<DescribeDeliveryChannelStatusResponse, DescribeDeliveryChannelStatusError>;

    /// <p><p>Returns details about the specified delivery channel. If a delivery channel is not specified, this action returns the details of all delivery channels associated with the account.</p> <note> <p>Currently, you can specify only one delivery channel per region in your account.</p> </note></p>
    fn describe_delivery_channels(
        &self,
        input: DescribeDeliveryChannelsRequest,
    ) -> RusotoFuture<DescribeDeliveryChannelsResponse, DescribeDeliveryChannelsError>;

    /// <p>Returns a list of all pending aggregation requests.</p>
    fn describe_pending_aggregation_requests(
        &self,
        input: DescribePendingAggregationRequestsRequest,
    ) -> RusotoFuture<
        DescribePendingAggregationRequestsResponse,
        DescribePendingAggregationRequestsError,
    >;

    /// <p>Returns the details of one or more remediation configurations.</p>
    fn describe_remediation_configurations(
        &self,
        input: DescribeRemediationConfigurationsRequest,
    ) -> RusotoFuture<
        DescribeRemediationConfigurationsResponse,
        DescribeRemediationConfigurationsError,
    >;

    /// <p>Provides a detailed view of a Remediation Execution for a set of resources including state, timestamps for when steps for the remediation execution occur, and any error messages for steps that have failed. When you specify the limit and the next token, you receive a paginated response.</p>
    fn describe_remediation_execution_status(
        &self,
        input: DescribeRemediationExecutionStatusRequest,
    ) -> RusotoFuture<
        DescribeRemediationExecutionStatusResponse,
        DescribeRemediationExecutionStatusError,
    >;

    /// <p><p>Returns the details of one or more retention configurations. If the retention configuration name is not specified, this action returns the details for all the retention configurations for that account.</p> <note> <p>Currently, AWS Config supports only one retention configuration per region in your account.</p> </note></p>
    fn describe_retention_configurations(
        &self,
        input: DescribeRetentionConfigurationsRequest,
    ) -> RusotoFuture<DescribeRetentionConfigurationsResponse, DescribeRetentionConfigurationsError>;

    /// <p><p>Returns the evaluation results for the specified AWS Config rule for a specific resource in a rule. The results indicate which AWS resources were evaluated by the rule, when each resource was last evaluated, and whether each resource complies with the rule. </p> <note> <p>The results can return an empty result page. But if you have a nextToken, the results are displayed on the next page.</p> </note></p>
    fn get_aggregate_compliance_details_by_config_rule(
        &self,
        input: GetAggregateComplianceDetailsByConfigRuleRequest,
    ) -> RusotoFuture<
        GetAggregateComplianceDetailsByConfigRuleResponse,
        GetAggregateComplianceDetailsByConfigRuleError,
    >;

    /// <p><p>Returns the number of compliant and noncompliant rules for one or more accounts and regions in an aggregator.</p> <note> <p>The results can return an empty result page, but if you have a nextToken, the results are displayed on the next page.</p> </note></p>
    fn get_aggregate_config_rule_compliance_summary(
        &self,
        input: GetAggregateConfigRuleComplianceSummaryRequest,
    ) -> RusotoFuture<
        GetAggregateConfigRuleComplianceSummaryResponse,
        GetAggregateConfigRuleComplianceSummaryError,
    >;

    /// <p>Returns the resource counts across accounts and regions that are present in your AWS Config aggregator. You can request the resource counts by providing filters and GroupByKey.</p> <p>For example, if the input contains accountID 12345678910 and region us-east-1 in filters, the API returns the count of resources in account ID 12345678910 and region us-east-1. If the input contains ACCOUNT_ID as a GroupByKey, the API returns resource counts for all source accounts that are present in your aggregator.</p>
    fn get_aggregate_discovered_resource_counts(
        &self,
        input: GetAggregateDiscoveredResourceCountsRequest,
    ) -> RusotoFuture<
        GetAggregateDiscoveredResourceCountsResponse,
        GetAggregateDiscoveredResourceCountsError,
    >;

    /// <p>Returns configuration item that is aggregated for your specific resource in a specific source account and region.</p>
    fn get_aggregate_resource_config(
        &self,
        input: GetAggregateResourceConfigRequest,
    ) -> RusotoFuture<GetAggregateResourceConfigResponse, GetAggregateResourceConfigError>;

    /// <p>Returns the evaluation results for the specified AWS Config rule. The results indicate which AWS resources were evaluated by the rule, when each resource was last evaluated, and whether each resource complies with the rule.</p>
    fn get_compliance_details_by_config_rule(
        &self,
        input: GetComplianceDetailsByConfigRuleRequest,
    ) -> RusotoFuture<GetComplianceDetailsByConfigRuleResponse, GetComplianceDetailsByConfigRuleError>;

    /// <p>Returns the evaluation results for the specified AWS resource. The results indicate which AWS Config rules were used to evaluate the resource, when each rule was last used, and whether the resource complies with each rule.</p>
    fn get_compliance_details_by_resource(
        &self,
        input: GetComplianceDetailsByResourceRequest,
    ) -> RusotoFuture<GetComplianceDetailsByResourceResponse, GetComplianceDetailsByResourceError>;

    /// <p>Returns the number of AWS Config rules that are compliant and noncompliant, up to a maximum of 25 for each.</p>
    fn get_compliance_summary_by_config_rule(
        &self,
    ) -> RusotoFuture<GetComplianceSummaryByConfigRuleResponse, GetComplianceSummaryByConfigRuleError>;

    /// <p>Returns the number of resources that are compliant and the number that are noncompliant. You can specify one or more resource types to get these numbers for each resource type. The maximum number returned is 100.</p>
    fn get_compliance_summary_by_resource_type(
        &self,
        input: GetComplianceSummaryByResourceTypeRequest,
    ) -> RusotoFuture<
        GetComplianceSummaryByResourceTypeResponse,
        GetComplianceSummaryByResourceTypeError,
    >;

    /// <p><p>Returns the resource types, the number of each resource type, and the total number of resources that AWS Config is recording in this region for your AWS account. </p> <p class="title"> <b>Example</b> </p> <ol> <li> <p>AWS Config is recording three resource types in the US East (Ohio) Region for your account: 25 EC2 instances, 20 IAM users, and 15 S3 buckets.</p> </li> <li> <p>You make a call to the <code>GetDiscoveredResourceCounts</code> action and specify that you want all resource types. </p> </li> <li> <p>AWS Config returns the following:</p> <ul> <li> <p>The resource types (EC2 instances, IAM users, and S3 buckets).</p> </li> <li> <p>The number of each resource type (25, 20, and 15).</p> </li> <li> <p>The total number of all resources (60).</p> </li> </ul> </li> </ol> <p>The response is paginated. By default, AWS Config lists 100 <a>ResourceCount</a> objects on each page. You can customize this number with the <code>limit</code> parameter. The response includes a <code>nextToken</code> string. To get the next page of results, run the request again and specify the string for the <code>nextToken</code> parameter.</p> <note> <p>If you make a call to the <a>GetDiscoveredResourceCounts</a> action, you might not immediately receive resource counts in the following situations:</p> <ul> <li> <p>You are a new AWS Config customer.</p> </li> <li> <p>You just enabled resource recording.</p> </li> </ul> <p>It might take a few minutes for AWS Config to record and count your resources. Wait a few minutes and then retry the <a>GetDiscoveredResourceCounts</a> action. </p> </note></p>
    fn get_discovered_resource_counts(
        &self,
        input: GetDiscoveredResourceCountsRequest,
    ) -> RusotoFuture<GetDiscoveredResourceCountsResponse, GetDiscoveredResourceCountsError>;

    /// <p><p>Returns a list of configuration items for the specified resource. The list contains details about each state of the resource during the specified time interval. If you specified a retention period to retain your <code>ConfigurationItems</code> between a minimum of 30 days and a maximum of 7 years (2557 days), AWS Config returns the <code>ConfigurationItems</code> for the specified retention period. </p> <p>The response is paginated. By default, AWS Config returns a limit of 10 configuration items per page. You can customize this number with the <code>limit</code> parameter. The response includes a <code>nextToken</code> string. To get the next page of results, run the request again and specify the string for the <code>nextToken</code> parameter.</p> <note> <p>Each call to the API is limited to span a duration of seven days. It is likely that the number of records returned is smaller than the specified <code>limit</code>. In such cases, you can make another call, using the <code>nextToken</code>.</p> </note></p>
    fn get_resource_config_history(
        &self,
        input: GetResourceConfigHistoryRequest,
    ) -> RusotoFuture<GetResourceConfigHistoryResponse, GetResourceConfigHistoryError>;

    /// <p>Accepts a resource type and returns a list of resource identifiers that are aggregated for a specific resource type across accounts and regions. A resource identifier includes the resource type, ID, (if available) the custom resource name, source account, and source region. You can narrow the results to include only resources that have specific resource IDs, or a resource name, or source account ID, or source region.</p> <p>For example, if the input consists of accountID 12345678910 and the region is us-east-1 for resource type <code>AWS::EC2::Instance</code> then the API returns all the EC2 instance identifiers of accountID 12345678910 and region us-east-1.</p>
    fn list_aggregate_discovered_resources(
        &self,
        input: ListAggregateDiscoveredResourcesRequest,
    ) -> RusotoFuture<ListAggregateDiscoveredResourcesResponse, ListAggregateDiscoveredResourcesError>;

    /// <p>Accepts a resource type and returns a list of resource identifiers for the resources of that type. A resource identifier includes the resource type, ID, and (if available) the custom resource name. The results consist of resources that AWS Config has discovered, including those that AWS Config is not currently recording. You can narrow the results to include only resources that have specific resource IDs or a resource name.</p> <note> <p>You can specify either resource IDs or a resource name, but not both, in the same request.</p> </note> <p>The response is paginated. By default, AWS Config lists 100 resource identifiers on each page. You can customize this number with the <code>limit</code> parameter. The response includes a <code>nextToken</code> string. To get the next page of results, run the request again and specify the string for the <code>nextToken</code> parameter.</p>
    fn list_discovered_resources(
        &self,
        input: ListDiscoveredResourcesRequest,
    ) -> RusotoFuture<ListDiscoveredResourcesResponse, ListDiscoveredResourcesError>;

    /// <p>List the tags for AWS Config resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError>;

    /// <p>Authorizes the aggregator account and region to collect data from the source account and region. </p>
    fn put_aggregation_authorization(
        &self,
        input: PutAggregationAuthorizationRequest,
    ) -> RusotoFuture<PutAggregationAuthorizationResponse, PutAggregationAuthorizationError>;

    /// <p>Adds or updates an AWS Config rule for evaluating whether your AWS resources comply with your desired configurations.</p> <p>You can use this action for custom AWS Config rules and AWS managed Config rules. A custom AWS Config rule is a rule that you develop and maintain. An AWS managed Config rule is a customizable, predefined rule that AWS Config provides.</p> <p>If you are adding a new custom AWS Config rule, you must first create the AWS Lambda function that the rule invokes to evaluate your resources. When you use the <code>PutConfigRule</code> action to add the rule to AWS Config, you must specify the Amazon Resource Name (ARN) that AWS Lambda assigns to the function. Specify the ARN for the <code>SourceIdentifier</code> key. This key is part of the <code>Source</code> object, which is part of the <code>ConfigRule</code> object. </p> <p>If you are adding an AWS managed Config rule, specify the rule's identifier for the <code>SourceIdentifier</code> key. To reference AWS managed Config rule identifiers, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html">About AWS Managed Config Rules</a>.</p> <p>For any new rule that you add, specify the <code>ConfigRuleName</code> in the <code>ConfigRule</code> object. Do not specify the <code>ConfigRuleArn</code> or the <code>ConfigRuleId</code>. These values are generated by AWS Config for new rules.</p> <p>If you are updating a rule that you added previously, you can specify the rule by <code>ConfigRuleName</code>, <code>ConfigRuleId</code>, or <code>ConfigRuleArn</code> in the <code>ConfigRule</code> data type that you use in this request.</p> <p>The maximum number of rules that AWS Config supports is 150.</p> <p>For information about requesting a rule limit increase, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_config">AWS Config Limits</a> in the <i>AWS General Reference Guide</i>.</p> <p>For more information about developing and using AWS Config rules, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config.html">Evaluating AWS Resource Configurations with AWS Config</a> in the <i>AWS Config Developer Guide</i>.</p>
    fn put_config_rule(&self, input: PutConfigRuleRequest) -> RusotoFuture<(), PutConfigRuleError>;

    /// <p><p>Creates and updates the configuration aggregator with the selected source accounts and regions. The source account can be individual account(s) or an organization.</p> <note> <p>AWS Config should be enabled in source accounts and regions you want to aggregate.</p> <p>If your source type is an organization, you must be signed in to the master account and all features must be enabled in your organization. AWS Config calls <code>EnableAwsServiceAccess</code> API to enable integration between AWS Config and AWS Organizations. </p> </note></p>
    fn put_configuration_aggregator(
        &self,
        input: PutConfigurationAggregatorRequest,
    ) -> RusotoFuture<PutConfigurationAggregatorResponse, PutConfigurationAggregatorError>;

    /// <p><p>Creates a new configuration recorder to record the selected resource configurations.</p> <p>You can use this action to change the role <code>roleARN</code> or the <code>recordingGroup</code> of an existing recorder. To change the role, call the action on the existing configuration recorder and specify a role.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> <p>If <code>ConfigurationRecorder</code> does not have the <b>recordingGroup</b> parameter specified, the default is to record all supported resource types.</p> </note></p>
    fn put_configuration_recorder(
        &self,
        input: PutConfigurationRecorderRequest,
    ) -> RusotoFuture<(), PutConfigurationRecorderError>;

    /// <p><p>Creates a delivery channel object to deliver configuration information to an Amazon S3 bucket and Amazon SNS topic.</p> <p>Before you can create a delivery channel, you must create a configuration recorder.</p> <p>You can use this action to change the Amazon S3 bucket or an Amazon SNS topic of the existing delivery channel. To change the Amazon S3 bucket or an Amazon SNS topic, call this action and specify the changed values for the S3 bucket and the SNS topic. If you specify a different value for either the S3 bucket or the SNS topic, this action will keep the existing value for the parameter that is not changed.</p> <note> <p>You can have only one delivery channel per region in your account.</p> </note></p>
    fn put_delivery_channel(
        &self,
        input: PutDeliveryChannelRequest,
    ) -> RusotoFuture<(), PutDeliveryChannelError>;

    /// <p>Used by an AWS Lambda function to deliver evaluation results to AWS Config. This action is required in every AWS Lambda function that is invoked by an AWS Config rule.</p>
    fn put_evaluations(
        &self,
        input: PutEvaluationsRequest,
    ) -> RusotoFuture<PutEvaluationsResponse, PutEvaluationsError>;

    /// <p>Adds or updates the remediation configuration with a specific AWS Config rule with the selected target or action. The API creates the <code>RemediationConfiguration</code> object for the AWS Config rule. The AWS Config rule must already exist for you to add a remediation configuration. The target (SSM document) must exist and have permissions to use the target. </p>
    fn put_remediation_configurations(
        &self,
        input: PutRemediationConfigurationsRequest,
    ) -> RusotoFuture<PutRemediationConfigurationsResponse, PutRemediationConfigurationsError>;

    /// <p><p>Creates and updates the retention configuration with details about retention period (number of days) that AWS Config stores your historical information. The API creates the <code>RetentionConfiguration</code> object and names the object as <b>default</b>. When you have a <code>RetentionConfiguration</code> object named <b>default</b>, calling the API modifies the default object. </p> <note> <p>Currently, AWS Config supports only one retention configuration per region in your account.</p> </note></p>
    fn put_retention_configuration(
        &self,
        input: PutRetentionConfigurationRequest,
    ) -> RusotoFuture<PutRetentionConfigurationResponse, PutRetentionConfigurationError>;

    /// <p>Accepts a structured query language (SQL) <code>SELECT</code> command, performs the corresponding search, and returns resource configurations matching the properties.</p> <p>For more information about query components, see the <a href="https://docs.aws.amazon.com/config/latest/developerguide/query-components.html"> <b>Query Components</b> </a> section in the AWS Config Developer Guide.</p>
    fn select_resource_config(
        &self,
        input: SelectResourceConfigRequest,
    ) -> RusotoFuture<SelectResourceConfigResponse, SelectResourceConfigError>;

    /// <p><p>Runs an on-demand evaluation for the specified AWS Config rules against the last known configuration state of the resources. Use <code>StartConfigRulesEvaluation</code> when you want to test that a rule you updated is working as expected. <code>StartConfigRulesEvaluation</code> does not re-record the latest configuration state for your resources. It re-runs an evaluation against the last known state of your resources. </p> <p>You can specify up to 25 AWS Config rules per request. </p> <p>An existing <code>StartConfigRulesEvaluation</code> call for the specified rules must complete before you can call the API again. If you chose to have AWS Config stream to an Amazon SNS topic, you will receive a <code>ConfigRuleEvaluationStarted</code> notification when the evaluation starts.</p> <note> <p>You don&#39;t need to call the <code>StartConfigRulesEvaluation</code> API to run an evaluation for a new rule. When you create a rule, AWS Config evaluates your resources against the rule automatically. </p> </note> <p>The <code>StartConfigRulesEvaluation</code> API is useful if you want to run on-demand evaluations, such as the following example:</p> <ol> <li> <p>You have a custom rule that evaluates your IAM resources every 24 hours.</p> </li> <li> <p>You update your Lambda function to add additional conditions to your rule.</p> </li> <li> <p>Instead of waiting for the next periodic evaluation, you call the <code>StartConfigRulesEvaluation</code> API.</p> </li> <li> <p>AWS Config invokes your Lambda function and evaluates your IAM resources.</p> </li> <li> <p>Your custom rule will still run periodic evaluations every 24 hours.</p> </li> </ol></p>
    fn start_config_rules_evaluation(
        &self,
        input: StartConfigRulesEvaluationRequest,
    ) -> RusotoFuture<StartConfigRulesEvaluationResponse, StartConfigRulesEvaluationError>;

    /// <p>Starts recording configurations of the AWS resources you have selected to record in your AWS account.</p> <p>You must have created at least one delivery channel to successfully start the configuration recorder.</p>
    fn start_configuration_recorder(
        &self,
        input: StartConfigurationRecorderRequest,
    ) -> RusotoFuture<(), StartConfigurationRecorderError>;

    /// <p>Runs an on-demand remediation for the specified AWS Config rules against the last known remediation configuration. It runs an execution against the current state of your resources. Remediation execution is asynchronous.</p> <p>You can specify up to 100 resource keys per request. An existing StartRemediationExecution call for the specified resource keys must complete before you can call the API again.</p>
    fn start_remediation_execution(
        &self,
        input: StartRemediationExecutionRequest,
    ) -> RusotoFuture<StartRemediationExecutionResponse, StartRemediationExecutionError>;

    /// <p>Stops recording configurations of the AWS resources you have selected to record in your AWS account.</p>
    fn stop_configuration_recorder(
        &self,
        input: StopConfigurationRecorderRequest,
    ) -> RusotoFuture<(), StopConfigurationRecorderError>;

    /// <p>Associates the specified tags to a resource with the specified resourceArn. If existing tags on a resource are not specified in the request parameters, they are not changed. When a resource is deleted, the tags associated with that resource are deleted as well.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError>;

    /// <p>Deletes specified tags from a resource.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError>;
}
/// A client for the Config Service API.
#[derive(Clone)]
pub struct ConfigServiceClient {
    client: Client,
    region: region::Region,
}

impl ConfigServiceClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ConfigServiceClient {
        ConfigServiceClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ConfigServiceClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        ConfigServiceClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl ConfigService for ConfigServiceClient {
    /// <p><p>Returns the current configuration items for resources that are present in your AWS Config aggregator. The operation also returns a list of resources that are not processed in the current request. If there are no unprocessed resources, the operation returns an empty <code>unprocessedResourceIdentifiers</code> list. </p> <note> <ul> <li> <p>The API does not return results for deleted resources.</p> </li> <li> <p> The API does not return tags and relationships.</p> </li> </ul> </note></p>
    fn batch_get_aggregate_resource_config(
        &self,
        input: BatchGetAggregateResourceConfigRequest,
    ) -> RusotoFuture<BatchGetAggregateResourceConfigResponse, BatchGetAggregateResourceConfigError>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.BatchGetAggregateResourceConfig",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchGetAggregateResourceConfigResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(BatchGetAggregateResourceConfigError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p><p>Returns the current configuration for one or more requested resources. The operation also returns a list of resources that are not processed in the current request. If there are no unprocessed resources, the operation returns an empty unprocessedResourceKeys list. </p> <note> <ul> <li> <p>The API does not return results for deleted resources.</p> </li> <li> <p> The API does not return any tags for the requested resources. This information is filtered out of the supplementaryConfiguration section of the API response.</p> </li> </ul> </note></p>
    fn batch_get_resource_config(
        &self,
        input: BatchGetResourceConfigRequest,
    ) -> RusotoFuture<BatchGetResourceConfigResponse, BatchGetResourceConfigError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.BatchGetResourceConfig");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchGetResourceConfigResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(BatchGetResourceConfigError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes the authorization granted to the specified configuration aggregator account in a specified region.</p>
    fn delete_aggregation_authorization(
        &self,
        input: DeleteAggregationAuthorizationRequest,
    ) -> RusotoFuture<(), DeleteAggregationAuthorizationError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DeleteAggregationAuthorization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteAggregationAuthorizationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes the specified AWS Config rule and all of its evaluation results.</p> <p>AWS Config sets the state of a rule to <code>DELETING</code> until the deletion is complete. You cannot update a rule while it is in this state. If you make a <code>PutConfigRule</code> or <code>DeleteConfigRule</code> request for the rule, you will receive a <code>ResourceInUseException</code>.</p> <p>You can check the state of a rule by using the <code>DescribeConfigRules</code> request.</p>
    fn delete_config_rule(
        &self,
        input: DeleteConfigRuleRequest,
    ) -> RusotoFuture<(), DeleteConfigRuleError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.DeleteConfigRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteConfigRuleError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified configuration aggregator and the aggregated data associated with the aggregator.</p>
    fn delete_configuration_aggregator(
        &self,
        input: DeleteConfigurationAggregatorRequest,
    ) -> RusotoFuture<(), DeleteConfigurationAggregatorError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DeleteConfigurationAggregator",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteConfigurationAggregatorError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes the configuration recorder.</p> <p>After the configuration recorder is deleted, AWS Config will not record resource configuration changes until you create a new configuration recorder.</p> <p>This action does not delete the configuration information that was previously recorded. You will be able to access the previously recorded information by using the <code>GetResourceConfigHistory</code> action, but you will not be able to access this information in the AWS Config console until you create a new configuration recorder.</p>
    fn delete_configuration_recorder(
        &self,
        input: DeleteConfigurationRecorderRequest,
    ) -> RusotoFuture<(), DeleteConfigurationRecorderError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DeleteConfigurationRecorder",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteConfigurationRecorderError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes the delivery channel.</p> <p>Before you can delete the delivery channel, you must stop the configuration recorder by using the <a>StopConfigurationRecorder</a> action.</p>
    fn delete_delivery_channel(
        &self,
        input: DeleteDeliveryChannelRequest,
    ) -> RusotoFuture<(), DeleteDeliveryChannelError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.DeleteDeliveryChannel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteDeliveryChannelError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes the evaluation results for the specified AWS Config rule. You can specify one AWS Config rule per request. After you delete the evaluation results, you can call the <a>StartConfigRulesEvaluation</a> API to start evaluating your AWS resources against the rule.</p>
    fn delete_evaluation_results(
        &self,
        input: DeleteEvaluationResultsRequest,
    ) -> RusotoFuture<DeleteEvaluationResultsResponse, DeleteEvaluationResultsError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DeleteEvaluationResults",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteEvaluationResultsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteEvaluationResultsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes pending authorization requests for a specified aggregator account in a specified region.</p>
    fn delete_pending_aggregation_request(
        &self,
        input: DeletePendingAggregationRequestRequest,
    ) -> RusotoFuture<(), DeletePendingAggregationRequestError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DeletePendingAggregationRequest",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeletePendingAggregationRequestError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Deletes the remediation configuration.</p>
    fn delete_remediation_configuration(
        &self,
        input: DeleteRemediationConfigurationRequest,
    ) -> RusotoFuture<DeleteRemediationConfigurationResponse, DeleteRemediationConfigurationError>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DeleteRemediationConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteRemediationConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteRemediationConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes the retention configuration.</p>
    fn delete_retention_configuration(
        &self,
        input: DeleteRetentionConfigurationRequest,
    ) -> RusotoFuture<(), DeleteRetentionConfigurationError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DeleteRetentionConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteRetentionConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Schedules delivery of a configuration snapshot to the Amazon S3 bucket in the specified delivery channel. After the delivery has started, AWS Config sends the following notifications using an Amazon SNS topic that you have specified.</p> <ul> <li> <p>Notification of the start of the delivery.</p> </li> <li> <p>Notification of the completion of the delivery, if the delivery was successfully completed.</p> </li> <li> <p>Notification of delivery failure, if the delivery failed.</p> </li> </ul></p>
    fn deliver_config_snapshot(
        &self,
        input: DeliverConfigSnapshotRequest,
    ) -> RusotoFuture<DeliverConfigSnapshotResponse, DeliverConfigSnapshotError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.DeliverConfigSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeliverConfigSnapshotResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeliverConfigSnapshotError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Returns a list of compliant and noncompliant rules with the number of resources for compliant and noncompliant rules. </p> <note> <p>The results can return an empty result page, but if you have a nextToken, the results are displayed on the next page.</p> </note></p>
    fn describe_aggregate_compliance_by_config_rules(
        &self,
        input: DescribeAggregateComplianceByConfigRulesRequest,
    ) -> RusotoFuture<
        DescribeAggregateComplianceByConfigRulesResponse,
        DescribeAggregateComplianceByConfigRulesError,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeAggregateComplianceByConfigRules",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeAggregateComplianceByConfigRulesResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAggregateComplianceByConfigRulesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns a list of authorizations granted to various aggregator accounts and regions.</p>
    fn describe_aggregation_authorizations(
        &self,
        input: DescribeAggregationAuthorizationsRequest,
    ) -> RusotoFuture<
        DescribeAggregationAuthorizationsResponse,
        DescribeAggregationAuthorizationsError,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeAggregationAuthorizations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeAggregationAuthorizationsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAggregationAuthorizationsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p><p>Indicates whether the specified AWS Config rules are compliant. If a rule is noncompliant, this action returns the number of AWS resources that do not comply with the rule.</p> <p>A rule is compliant if all of the evaluated resources comply with it. It is noncompliant if any of these resources do not comply.</p> <p>If AWS Config has no current evaluation results for the rule, it returns <code>INSUFFICIENT<em>DATA</code>. This result might indicate one of the following conditions:</p> <ul> <li> <p>AWS Config has never invoked an evaluation for the rule. To check whether it has, use the <code>DescribeConfigRuleEvaluationStatus</code> action to get the <code>LastSuccessfulInvocationTime</code> and <code>LastFailedInvocationTime</code>.</p> </li> <li> <p>The rule&#39;s AWS Lambda function is failing to send evaluation results to AWS Config. Verify that the role you assigned to your configuration recorder includes the <code>config:PutEvaluations</code> permission. If the rule is a custom rule, verify that the AWS Lambda execution role includes the <code>config:PutEvaluations</code> permission.</p> </li> <li> <p>The rule&#39;s AWS Lambda function has returned <code>NOT</em>APPLICABLE</code> for all evaluation results. This can occur if the resources were deleted or removed from the rule&#39;s scope.</p> </li> </ul></p>
    fn describe_compliance_by_config_rule(
        &self,
        input: DescribeComplianceByConfigRuleRequest,
    ) -> RusotoFuture<DescribeComplianceByConfigRuleResponse, DescribeComplianceByConfigRuleError>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeComplianceByConfigRule",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeComplianceByConfigRuleResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeComplianceByConfigRuleError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Indicates whether the specified AWS resources are compliant. If a resource is noncompliant, this action returns the number of AWS Config rules that the resource does not comply with.</p> <p>A resource is compliant if it complies with all the AWS Config rules that evaluate it. It is noncompliant if it does not comply with one or more of these rules.</p> <p>If AWS Config has no current evaluation results for the resource, it returns <code>INSUFFICIENT<em>DATA</code>. This result might indicate one of the following conditions about the rules that evaluate the resource:</p> <ul> <li> <p>AWS Config has never invoked an evaluation for the rule. To check whether it has, use the <code>DescribeConfigRuleEvaluationStatus</code> action to get the <code>LastSuccessfulInvocationTime</code> and <code>LastFailedInvocationTime</code>.</p> </li> <li> <p>The rule&#39;s AWS Lambda function is failing to send evaluation results to AWS Config. Verify that the role that you assigned to your configuration recorder includes the <code>config:PutEvaluations</code> permission. If the rule is a custom rule, verify that the AWS Lambda execution role includes the <code>config:PutEvaluations</code> permission.</p> </li> <li> <p>The rule&#39;s AWS Lambda function has returned <code>NOT</em>APPLICABLE</code> for all evaluation results. This can occur if the resources were deleted or removed from the rule&#39;s scope.</p> </li> </ul></p>
    fn describe_compliance_by_resource(
        &self,
        input: DescribeComplianceByResourceRequest,
    ) -> RusotoFuture<DescribeComplianceByResourceResponse, DescribeComplianceByResourceError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeComplianceByResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeComplianceByResourceResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeComplianceByResourceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns status information for each of your AWS managed Config rules. The status includes information such as the last time AWS Config invoked the rule, the last time AWS Config failed to invoke the rule, and the related error for the last failure.</p>
    fn describe_config_rule_evaluation_status(
        &self,
        input: DescribeConfigRuleEvaluationStatusRequest,
    ) -> RusotoFuture<
        DescribeConfigRuleEvaluationStatusResponse,
        DescribeConfigRuleEvaluationStatusError,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeConfigRuleEvaluationStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeConfigRuleEvaluationStatusResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeConfigRuleEvaluationStatusError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns details about your AWS Config rules.</p>
    fn describe_config_rules(
        &self,
        input: DescribeConfigRulesRequest,
    ) -> RusotoFuture<DescribeConfigRulesResponse, DescribeConfigRulesError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.DescribeConfigRules");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeConfigRulesResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeConfigRulesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns status information for sources within an aggregator. The status includes information about the last time AWS Config verified authorization between the source account and an aggregator account. In case of a failure, the status contains the related error code or message. </p>
    fn describe_configuration_aggregator_sources_status(
        &self,
        input: DescribeConfigurationAggregatorSourcesStatusRequest,
    ) -> RusotoFuture<
        DescribeConfigurationAggregatorSourcesStatusResponse,
        DescribeConfigurationAggregatorSourcesStatusError,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeConfigurationAggregatorSourcesStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
                        if response.status.is_success() {
                            Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<DescribeConfigurationAggregatorSourcesStatusResponse, _>()
                }))
                        } else {
                            Box::new(response.buffer().from_err().and_then(|response| {
                                Err(DescribeConfigurationAggregatorSourcesStatusError::from_response(response))
                            }))
                        }
                    })
    }

    /// <p>Returns the details of one or more configuration aggregators. If the configuration aggregator is not specified, this action returns the details for all the configuration aggregators associated with the account. </p>
    fn describe_configuration_aggregators(
        &self,
        input: DescribeConfigurationAggregatorsRequest,
    ) -> RusotoFuture<DescribeConfigurationAggregatorsResponse, DescribeConfigurationAggregatorsError>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeConfigurationAggregators",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeConfigurationAggregatorsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeConfigurationAggregatorsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p><p>Returns the current status of the specified configuration recorder. If a configuration recorder is not specified, this action returns the status of all configuration recorders associated with the account.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> </note></p>
    fn describe_configuration_recorder_status(
        &self,
        input: DescribeConfigurationRecorderStatusRequest,
    ) -> RusotoFuture<
        DescribeConfigurationRecorderStatusResponse,
        DescribeConfigurationRecorderStatusError,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeConfigurationRecorderStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeConfigurationRecorderStatusResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeConfigurationRecorderStatusError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p><p>Returns the details for the specified configuration recorders. If the configuration recorder is not specified, this action returns the details for all configuration recorders associated with the account.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> </note></p>
    fn describe_configuration_recorders(
        &self,
        input: DescribeConfigurationRecordersRequest,
    ) -> RusotoFuture<DescribeConfigurationRecordersResponse, DescribeConfigurationRecordersError>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeConfigurationRecorders",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeConfigurationRecordersResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeConfigurationRecordersError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Returns the current status of the specified delivery channel. If a delivery channel is not specified, this action returns the current status of all delivery channels associated with the account.</p> <note> <p>Currently, you can specify only one delivery channel per region in your account.</p> </note></p>
    fn describe_delivery_channel_status(
        &self,
        input: DescribeDeliveryChannelStatusRequest,
    ) -> RusotoFuture<DescribeDeliveryChannelStatusResponse, DescribeDeliveryChannelStatusError>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeDeliveryChannelStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeDeliveryChannelStatusResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDeliveryChannelStatusError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Returns details about the specified delivery channel. If a delivery channel is not specified, this action returns the details of all delivery channels associated with the account.</p> <note> <p>Currently, you can specify only one delivery channel per region in your account.</p> </note></p>
    fn describe_delivery_channels(
        &self,
        input: DescribeDeliveryChannelsRequest,
    ) -> RusotoFuture<DescribeDeliveryChannelsResponse, DescribeDeliveryChannelsError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeDeliveryChannels",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeDeliveryChannelsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDeliveryChannelsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns a list of all pending aggregation requests.</p>
    fn describe_pending_aggregation_requests(
        &self,
        input: DescribePendingAggregationRequestsRequest,
    ) -> RusotoFuture<
        DescribePendingAggregationRequestsResponse,
        DescribePendingAggregationRequestsError,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribePendingAggregationRequests",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribePendingAggregationRequestsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribePendingAggregationRequestsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns the details of one or more remediation configurations.</p>
    fn describe_remediation_configurations(
        &self,
        input: DescribeRemediationConfigurationsRequest,
    ) -> RusotoFuture<
        DescribeRemediationConfigurationsResponse,
        DescribeRemediationConfigurationsError,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeRemediationConfigurations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeRemediationConfigurationsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeRemediationConfigurationsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Provides a detailed view of a Remediation Execution for a set of resources including state, timestamps for when steps for the remediation execution occur, and any error messages for steps that have failed. When you specify the limit and the next token, you receive a paginated response.</p>
    fn describe_remediation_execution_status(
        &self,
        input: DescribeRemediationExecutionStatusRequest,
    ) -> RusotoFuture<
        DescribeRemediationExecutionStatusResponse,
        DescribeRemediationExecutionStatusError,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeRemediationExecutionStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeRemediationExecutionStatusResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeRemediationExecutionStatusError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p><p>Returns the details of one or more retention configurations. If the retention configuration name is not specified, this action returns the details for all the retention configurations for that account.</p> <note> <p>Currently, AWS Config supports only one retention configuration per region in your account.</p> </note></p>
    fn describe_retention_configurations(
        &self,
        input: DescribeRetentionConfigurationsRequest,
    ) -> RusotoFuture<DescribeRetentionConfigurationsResponse, DescribeRetentionConfigurationsError>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.DescribeRetentionConfigurations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeRetentionConfigurationsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeRetentionConfigurationsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p><p>Returns the evaluation results for the specified AWS Config rule for a specific resource in a rule. The results indicate which AWS resources were evaluated by the rule, when each resource was last evaluated, and whether each resource complies with the rule. </p> <note> <p>The results can return an empty result page. But if you have a nextToken, the results are displayed on the next page.</p> </note></p>
    fn get_aggregate_compliance_details_by_config_rule(
        &self,
        input: GetAggregateComplianceDetailsByConfigRuleRequest,
    ) -> RusotoFuture<
        GetAggregateComplianceDetailsByConfigRuleResponse,
        GetAggregateComplianceDetailsByConfigRuleError,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetAggregateComplianceDetailsByConfigRule",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetAggregateComplianceDetailsByConfigRuleResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetAggregateComplianceDetailsByConfigRuleError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Returns the number of compliant and noncompliant rules for one or more accounts and regions in an aggregator.</p> <note> <p>The results can return an empty result page, but if you have a nextToken, the results are displayed on the next page.</p> </note></p>
    fn get_aggregate_config_rule_compliance_summary(
        &self,
        input: GetAggregateConfigRuleComplianceSummaryRequest,
    ) -> RusotoFuture<
        GetAggregateConfigRuleComplianceSummaryResponse,
        GetAggregateConfigRuleComplianceSummaryError,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetAggregateConfigRuleComplianceSummary",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetAggregateConfigRuleComplianceSummaryResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetAggregateConfigRuleComplianceSummaryError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns the resource counts across accounts and regions that are present in your AWS Config aggregator. You can request the resource counts by providing filters and GroupByKey.</p> <p>For example, if the input contains accountID 12345678910 and region us-east-1 in filters, the API returns the count of resources in account ID 12345678910 and region us-east-1. If the input contains ACCOUNT_ID as a GroupByKey, the API returns resource counts for all source accounts that are present in your aggregator.</p>
    fn get_aggregate_discovered_resource_counts(
        &self,
        input: GetAggregateDiscoveredResourceCountsRequest,
    ) -> RusotoFuture<
        GetAggregateDiscoveredResourceCountsResponse,
        GetAggregateDiscoveredResourceCountsError,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetAggregateDiscoveredResourceCounts",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetAggregateDiscoveredResourceCountsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetAggregateDiscoveredResourceCountsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns configuration item that is aggregated for your specific resource in a specific source account and region.</p>
    fn get_aggregate_resource_config(
        &self,
        input: GetAggregateResourceConfigRequest,
    ) -> RusotoFuture<GetAggregateResourceConfigResponse, GetAggregateResourceConfigError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetAggregateResourceConfig",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetAggregateResourceConfigResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetAggregateResourceConfigError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns the evaluation results for the specified AWS Config rule. The results indicate which AWS resources were evaluated by the rule, when each resource was last evaluated, and whether each resource complies with the rule.</p>
    fn get_compliance_details_by_config_rule(
        &self,
        input: GetComplianceDetailsByConfigRuleRequest,
    ) -> RusotoFuture<GetComplianceDetailsByConfigRuleResponse, GetComplianceDetailsByConfigRuleError>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetComplianceDetailsByConfigRule",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetComplianceDetailsByConfigRuleResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetComplianceDetailsByConfigRuleError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns the evaluation results for the specified AWS resource. The results indicate which AWS Config rules were used to evaluate the resource, when each rule was last used, and whether the resource complies with each rule.</p>
    fn get_compliance_details_by_resource(
        &self,
        input: GetComplianceDetailsByResourceRequest,
    ) -> RusotoFuture<GetComplianceDetailsByResourceResponse, GetComplianceDetailsByResourceError>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetComplianceDetailsByResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetComplianceDetailsByResourceResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetComplianceDetailsByResourceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns the number of AWS Config rules that are compliant and noncompliant, up to a maximum of 25 for each.</p>
    fn get_compliance_summary_by_config_rule(
        &self,
    ) -> RusotoFuture<GetComplianceSummaryByConfigRuleResponse, GetComplianceSummaryByConfigRuleError>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetComplianceSummaryByConfigRule",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetComplianceSummaryByConfigRuleResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetComplianceSummaryByConfigRuleError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns the number of resources that are compliant and the number that are noncompliant. You can specify one or more resource types to get these numbers for each resource type. The maximum number returned is 100.</p>
    fn get_compliance_summary_by_resource_type(
        &self,
        input: GetComplianceSummaryByResourceTypeRequest,
    ) -> RusotoFuture<
        GetComplianceSummaryByResourceTypeResponse,
        GetComplianceSummaryByResourceTypeError,
    > {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetComplianceSummaryByResourceType",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetComplianceSummaryByResourceTypeResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetComplianceSummaryByResourceTypeError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p><p>Returns the resource types, the number of each resource type, and the total number of resources that AWS Config is recording in this region for your AWS account. </p> <p class="title"> <b>Example</b> </p> <ol> <li> <p>AWS Config is recording three resource types in the US East (Ohio) Region for your account: 25 EC2 instances, 20 IAM users, and 15 S3 buckets.</p> </li> <li> <p>You make a call to the <code>GetDiscoveredResourceCounts</code> action and specify that you want all resource types. </p> </li> <li> <p>AWS Config returns the following:</p> <ul> <li> <p>The resource types (EC2 instances, IAM users, and S3 buckets).</p> </li> <li> <p>The number of each resource type (25, 20, and 15).</p> </li> <li> <p>The total number of all resources (60).</p> </li> </ul> </li> </ol> <p>The response is paginated. By default, AWS Config lists 100 <a>ResourceCount</a> objects on each page. You can customize this number with the <code>limit</code> parameter. The response includes a <code>nextToken</code> string. To get the next page of results, run the request again and specify the string for the <code>nextToken</code> parameter.</p> <note> <p>If you make a call to the <a>GetDiscoveredResourceCounts</a> action, you might not immediately receive resource counts in the following situations:</p> <ul> <li> <p>You are a new AWS Config customer.</p> </li> <li> <p>You just enabled resource recording.</p> </li> </ul> <p>It might take a few minutes for AWS Config to record and count your resources. Wait a few minutes and then retry the <a>GetDiscoveredResourceCounts</a> action. </p> </note></p>
    fn get_discovered_resource_counts(
        &self,
        input: GetDiscoveredResourceCountsRequest,
    ) -> RusotoFuture<GetDiscoveredResourceCountsResponse, GetDiscoveredResourceCountsError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetDiscoveredResourceCounts",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDiscoveredResourceCountsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDiscoveredResourceCountsError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Returns a list of configuration items for the specified resource. The list contains details about each state of the resource during the specified time interval. If you specified a retention period to retain your <code>ConfigurationItems</code> between a minimum of 30 days and a maximum of 7 years (2557 days), AWS Config returns the <code>ConfigurationItems</code> for the specified retention period. </p> <p>The response is paginated. By default, AWS Config returns a limit of 10 configuration items per page. You can customize this number with the <code>limit</code> parameter. The response includes a <code>nextToken</code> string. To get the next page of results, run the request again and specify the string for the <code>nextToken</code> parameter.</p> <note> <p>Each call to the API is limited to span a duration of seven days. It is likely that the number of records returned is smaller than the specified <code>limit</code>. In such cases, you can make another call, using the <code>nextToken</code>.</p> </note></p>
    fn get_resource_config_history(
        &self,
        input: GetResourceConfigHistoryRequest,
    ) -> RusotoFuture<GetResourceConfigHistoryResponse, GetResourceConfigHistoryError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.GetResourceConfigHistory",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetResourceConfigHistoryResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetResourceConfigHistoryError::from_response(response))
                }))
            }
        })
    }

    /// <p>Accepts a resource type and returns a list of resource identifiers that are aggregated for a specific resource type across accounts and regions. A resource identifier includes the resource type, ID, (if available) the custom resource name, source account, and source region. You can narrow the results to include only resources that have specific resource IDs, or a resource name, or source account ID, or source region.</p> <p>For example, if the input consists of accountID 12345678910 and the region is us-east-1 for resource type <code>AWS::EC2::Instance</code> then the API returns all the EC2 instance identifiers of accountID 12345678910 and region us-east-1.</p>
    fn list_aggregate_discovered_resources(
        &self,
        input: ListAggregateDiscoveredResourcesRequest,
    ) -> RusotoFuture<ListAggregateDiscoveredResourcesResponse, ListAggregateDiscoveredResourcesError>
    {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.ListAggregateDiscoveredResources",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListAggregateDiscoveredResourcesResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListAggregateDiscoveredResourcesError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Accepts a resource type and returns a list of resource identifiers for the resources of that type. A resource identifier includes the resource type, ID, and (if available) the custom resource name. The results consist of resources that AWS Config has discovered, including those that AWS Config is not currently recording. You can narrow the results to include only resources that have specific resource IDs or a resource name.</p> <note> <p>You can specify either resource IDs or a resource name, but not both, in the same request.</p> </note> <p>The response is paginated. By default, AWS Config lists 100 resource identifiers on each page. You can customize this number with the <code>limit</code> parameter. The response includes a <code>nextToken</code> string. To get the next page of results, run the request again and specify the string for the <code>nextToken</code> parameter.</p>
    fn list_discovered_resources(
        &self,
        input: ListDiscoveredResourcesRequest,
    ) -> RusotoFuture<ListDiscoveredResourcesResponse, ListDiscoveredResourcesError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.ListDiscoveredResources",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListDiscoveredResourcesResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListDiscoveredResourcesError::from_response(response))
                }))
            }
        })
    }

    /// <p>List the tags for AWS Config resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTagsForResourceResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListTagsForResourceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Authorizes the aggregator account and region to collect data from the source account and region. </p>
    fn put_aggregation_authorization(
        &self,
        input: PutAggregationAuthorizationRequest,
    ) -> RusotoFuture<PutAggregationAuthorizationResponse, PutAggregationAuthorizationError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.PutAggregationAuthorization",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutAggregationAuthorizationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutAggregationAuthorizationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Adds or updates an AWS Config rule for evaluating whether your AWS resources comply with your desired configurations.</p> <p>You can use this action for custom AWS Config rules and AWS managed Config rules. A custom AWS Config rule is a rule that you develop and maintain. An AWS managed Config rule is a customizable, predefined rule that AWS Config provides.</p> <p>If you are adding a new custom AWS Config rule, you must first create the AWS Lambda function that the rule invokes to evaluate your resources. When you use the <code>PutConfigRule</code> action to add the rule to AWS Config, you must specify the Amazon Resource Name (ARN) that AWS Lambda assigns to the function. Specify the ARN for the <code>SourceIdentifier</code> key. This key is part of the <code>Source</code> object, which is part of the <code>ConfigRule</code> object. </p> <p>If you are adding an AWS managed Config rule, specify the rule's identifier for the <code>SourceIdentifier</code> key. To reference AWS managed Config rule identifiers, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html">About AWS Managed Config Rules</a>.</p> <p>For any new rule that you add, specify the <code>ConfigRuleName</code> in the <code>ConfigRule</code> object. Do not specify the <code>ConfigRuleArn</code> or the <code>ConfigRuleId</code>. These values are generated by AWS Config for new rules.</p> <p>If you are updating a rule that you added previously, you can specify the rule by <code>ConfigRuleName</code>, <code>ConfigRuleId</code>, or <code>ConfigRuleArn</code> in the <code>ConfigRule</code> data type that you use in this request.</p> <p>The maximum number of rules that AWS Config supports is 150.</p> <p>For information about requesting a rule limit increase, see <a href="http://docs.aws.amazon.com/general/latest/gr/aws_service_limits.html#limits_config">AWS Config Limits</a> in the <i>AWS General Reference Guide</i>.</p> <p>For more information about developing and using AWS Config rules, see <a href="https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config.html">Evaluating AWS Resource Configurations with AWS Config</a> in the <i>AWS Config Developer Guide</i>.</p>
    fn put_config_rule(&self, input: PutConfigRuleRequest) -> RusotoFuture<(), PutConfigRuleError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.PutConfigRule");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutConfigRuleError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Creates and updates the configuration aggregator with the selected source accounts and regions. The source account can be individual account(s) or an organization.</p> <note> <p>AWS Config should be enabled in source accounts and regions you want to aggregate.</p> <p>If your source type is an organization, you must be signed in to the master account and all features must be enabled in your organization. AWS Config calls <code>EnableAwsServiceAccess</code> API to enable integration between AWS Config and AWS Organizations. </p> </note></p>
    fn put_configuration_aggregator(
        &self,
        input: PutConfigurationAggregatorRequest,
    ) -> RusotoFuture<PutConfigurationAggregatorResponse, PutConfigurationAggregatorError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.PutConfigurationAggregator",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutConfigurationAggregatorResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutConfigurationAggregatorError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Creates a new configuration recorder to record the selected resource configurations.</p> <p>You can use this action to change the role <code>roleARN</code> or the <code>recordingGroup</code> of an existing recorder. To change the role, call the action on the existing configuration recorder and specify a role.</p> <note> <p>Currently, you can specify only one configuration recorder per region in your account.</p> <p>If <code>ConfigurationRecorder</code> does not have the <b>recordingGroup</b> parameter specified, the default is to record all supported resource types.</p> </note></p>
    fn put_configuration_recorder(
        &self,
        input: PutConfigurationRecorderRequest,
    ) -> RusotoFuture<(), PutConfigurationRecorderError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.PutConfigurationRecorder",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutConfigurationRecorderError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Creates a delivery channel object to deliver configuration information to an Amazon S3 bucket and Amazon SNS topic.</p> <p>Before you can create a delivery channel, you must create a configuration recorder.</p> <p>You can use this action to change the Amazon S3 bucket or an Amazon SNS topic of the existing delivery channel. To change the Amazon S3 bucket or an Amazon SNS topic, call this action and specify the changed values for the S3 bucket and the SNS topic. If you specify a different value for either the S3 bucket or the SNS topic, this action will keep the existing value for the parameter that is not changed.</p> <note> <p>You can have only one delivery channel per region in your account.</p> </note></p>
    fn put_delivery_channel(
        &self,
        input: PutDeliveryChannelRequest,
    ) -> RusotoFuture<(), PutDeliveryChannelError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.PutDeliveryChannel");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutDeliveryChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Used by an AWS Lambda function to deliver evaluation results to AWS Config. This action is required in every AWS Lambda function that is invoked by an AWS Config rule.</p>
    fn put_evaluations(
        &self,
        input: PutEvaluationsRequest,
    ) -> RusotoFuture<PutEvaluationsResponse, PutEvaluationsError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.PutEvaluations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutEvaluationsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutEvaluationsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Adds or updates the remediation configuration with a specific AWS Config rule with the selected target or action. The API creates the <code>RemediationConfiguration</code> object for the AWS Config rule. The AWS Config rule must already exist for you to add a remediation configuration. The target (SSM document) must exist and have permissions to use the target. </p>
    fn put_remediation_configurations(
        &self,
        input: PutRemediationConfigurationsRequest,
    ) -> RusotoFuture<PutRemediationConfigurationsResponse, PutRemediationConfigurationsError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.PutRemediationConfigurations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutRemediationConfigurationsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutRemediationConfigurationsError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Creates and updates the retention configuration with details about retention period (number of days) that AWS Config stores your historical information. The API creates the <code>RetentionConfiguration</code> object and names the object as <b>default</b>. When you have a <code>RetentionConfiguration</code> object named <b>default</b>, calling the API modifies the default object. </p> <note> <p>Currently, AWS Config supports only one retention configuration per region in your account.</p> </note></p>
    fn put_retention_configuration(
        &self,
        input: PutRetentionConfigurationRequest,
    ) -> RusotoFuture<PutRetentionConfigurationResponse, PutRetentionConfigurationError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.PutRetentionConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutRetentionConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutRetentionConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Accepts a structured query language (SQL) <code>SELECT</code> command, performs the corresponding search, and returns resource configurations matching the properties.</p> <p>For more information about query components, see the <a href="https://docs.aws.amazon.com/config/latest/developerguide/query-components.html"> <b>Query Components</b> </a> section in the AWS Config Developer Guide.</p>
    fn select_resource_config(
        &self,
        input: SelectResourceConfigRequest,
    ) -> RusotoFuture<SelectResourceConfigResponse, SelectResourceConfigError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.SelectResourceConfig");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<SelectResourceConfigResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(SelectResourceConfigError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Runs an on-demand evaluation for the specified AWS Config rules against the last known configuration state of the resources. Use <code>StartConfigRulesEvaluation</code> when you want to test that a rule you updated is working as expected. <code>StartConfigRulesEvaluation</code> does not re-record the latest configuration state for your resources. It re-runs an evaluation against the last known state of your resources. </p> <p>You can specify up to 25 AWS Config rules per request. </p> <p>An existing <code>StartConfigRulesEvaluation</code> call for the specified rules must complete before you can call the API again. If you chose to have AWS Config stream to an Amazon SNS topic, you will receive a <code>ConfigRuleEvaluationStarted</code> notification when the evaluation starts.</p> <note> <p>You don&#39;t need to call the <code>StartConfigRulesEvaluation</code> API to run an evaluation for a new rule. When you create a rule, AWS Config evaluates your resources against the rule automatically. </p> </note> <p>The <code>StartConfigRulesEvaluation</code> API is useful if you want to run on-demand evaluations, such as the following example:</p> <ol> <li> <p>You have a custom rule that evaluates your IAM resources every 24 hours.</p> </li> <li> <p>You update your Lambda function to add additional conditions to your rule.</p> </li> <li> <p>Instead of waiting for the next periodic evaluation, you call the <code>StartConfigRulesEvaluation</code> API.</p> </li> <li> <p>AWS Config invokes your Lambda function and evaluates your IAM resources.</p> </li> <li> <p>Your custom rule will still run periodic evaluations every 24 hours.</p> </li> </ol></p>
    fn start_config_rules_evaluation(
        &self,
        input: StartConfigRulesEvaluationRequest,
    ) -> RusotoFuture<StartConfigRulesEvaluationResponse, StartConfigRulesEvaluationError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.StartConfigRulesEvaluation",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartConfigRulesEvaluationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartConfigRulesEvaluationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Starts recording configurations of the AWS resources you have selected to record in your AWS account.</p> <p>You must have created at least one delivery channel to successfully start the configuration recorder.</p>
    fn start_configuration_recorder(
        &self,
        input: StartConfigurationRecorderRequest,
    ) -> RusotoFuture<(), StartConfigurationRecorderError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.StartConfigurationRecorder",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartConfigurationRecorderError::from_response(response))
                }))
            }
        })
    }

    /// <p>Runs an on-demand remediation for the specified AWS Config rules against the last known remediation configuration. It runs an execution against the current state of your resources. Remediation execution is asynchronous.</p> <p>You can specify up to 100 resource keys per request. An existing StartRemediationExecution call for the specified resource keys must complete before you can call the API again.</p>
    fn start_remediation_execution(
        &self,
        input: StartRemediationExecutionRequest,
    ) -> RusotoFuture<StartRemediationExecutionResponse, StartRemediationExecutionError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.StartRemediationExecution",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartRemediationExecutionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartRemediationExecutionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Stops recording configurations of the AWS resources you have selected to record in your AWS account.</p>
    fn stop_configuration_recorder(
        &self,
        input: StopConfigurationRecorderRequest,
    ) -> RusotoFuture<(), StopConfigurationRecorderError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "StarlingDoveService.StopConfigurationRecorder",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StopConfigurationRecorderError::from_response(response))
                }))
            }
        })
    }

    /// <p>Associates the specified tags to a resource with the specified resourceArn. If existing tags on a resource are not specified in the request parameters, they are not changed. When a resource is deleted, the tags associated with that resource are deleted as well.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes specified tags from a resource.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError> {
        let mut request = SignedRequest::new("POST", "config", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "StarlingDoveService.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UntagResourceError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
