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
/// <p>Information about the application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AppSummary {
    /// <p>Unique ID of the application.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>Time of creation of this application.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>Description of the application.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Timestamp of the application's creation.</p>
    #[serde(rename = "lastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    /// <p>Timestamp of the application's most recent successful replication.</p>
    #[serde(rename = "latestReplicationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_replication_time: Option<f64>,
    /// <p>Details about the latest launch of the application.</p>
    #[serde(rename = "launchDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_details: Option<LaunchDetails>,
    /// <p>Launch status of the application.</p>
    #[serde(rename = "launchStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_status: Option<String>,
    /// <p>A message related to the launch status of the application.</p>
    #[serde(rename = "launchStatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_status_message: Option<String>,
    /// <p>Name of the application.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Replication status of the application.</p>
    #[serde(rename = "replicationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_status: Option<String>,
    /// <p>A message related to the replication status of the application.</p>
    #[serde(rename = "replicationStatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_status_message: Option<String>,
    /// <p>Name of the service role in the customer's account used by AWS SMS.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p>Status of the application.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A message related to the status of the application</p>
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Number of server groups present in the application.</p>
    #[serde(rename = "totalServerGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_server_groups: Option<i64>,
    /// <p>Number of servers present in the application.</p>
    #[serde(rename = "totalServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_servers: Option<i64>,
}

/// <p>Represents a connector.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Connector {
    /// <p>The time the connector was associated.</p>
    #[serde(rename = "associatedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_on: Option<f64>,
    /// <p>The capabilities of the connector.</p>
    #[serde(rename = "capabilityList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_list: Option<Vec<String>>,
    /// <p>The identifier of the connector.</p>
    #[serde(rename = "connectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    /// <p>The IP address of the connector.</p>
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>The MAC address of the connector.</p>
    #[serde(rename = "macAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// <p>The status of the connector.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The connector version.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The identifier of the VM manager.</p>
    #[serde(rename = "vmManagerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_id: Option<String>,
    /// <p>The name of the VM manager.</p>
    #[serde(rename = "vmManagerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_name: Option<String>,
    /// <p>The VM management product.</p>
    #[serde(rename = "vmManagerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAppRequest {
    /// <p>A unique, case-sensitive identifier you provide to ensure idempotency of application creation.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>Description of the new application</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Name of the new application.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Name of service role in customer's account to be used by AWS SMS.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p>List of server groups to include in the application.</p>
    #[serde(rename = "serverGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_groups: Option<Vec<ServerGroup>>,
    /// <p>List of tags to be associated with the application.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateAppResponse {
    /// <p>Summary description of the application.</p>
    #[serde(rename = "appSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_summary: Option<AppSummary>,
    /// <p>List of server groups included in the application.</p>
    #[serde(rename = "serverGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_groups: Option<Vec<ServerGroup>>,
    /// <p>List of taags associated with the application.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateReplicationJobRequest {
    /// <p>The description of the replication job.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>When <i>true</i>, the replication job produces encrypted AMIs. See also <code>KmsKeyId</code> below.</p>
    #[serde(rename = "encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>The time between consecutive replication runs, in hours.</p>
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i64>,
    /// <p>KMS key ID for replication jobs that produce encrypted AMIs. Can be any of the following: </p> <ul> <li> <p>KMS key ID</p> </li> <li> <p>KMS key alias</p> </li> <li> <p>ARN referring to KMS key ID</p> </li> <li> <p>ARN referring to KMS key alias</p> </li> </ul> <p> If encrypted is <i>true</i> but a KMS key id is not specified, the customer's default KMS key for EBS is used. </p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The license type to be used for the AMI created by a successful replication run.</p>
    #[serde(rename = "licenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    /// <p>The maximum number of SMS-created AMIs to retain. The oldest will be deleted once the maximum number is reached and a new AMI is created.</p>
    #[serde(rename = "numberOfRecentAmisToKeep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_recent_amis_to_keep: Option<i64>,
    /// <p>The name of the IAM role to be used by the AWS SMS.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "runOnce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_once: Option<bool>,
    /// <p>The seed replication time.</p>
    #[serde(rename = "seedReplicationTime")]
    pub seed_replication_time: f64,
    /// <p>The identifier of the server.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateReplicationJobResponse {
    /// <p>The unique identifier of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAppLaunchConfigurationRequest {
    /// <p>ID of the application associated with the launch configuration.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteAppLaunchConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAppReplicationConfigurationRequest {
    /// <p>ID of the application associated with the replication configuration.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteAppReplicationConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAppRequest {
    /// <p>ID of the application to delete.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>While deleting the application, stop all replication jobs corresponding to the servers in the application.</p>
    #[serde(rename = "forceStopAppReplication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_stop_app_replication: Option<bool>,
    /// <p>While deleting the application, terminate the stack corresponding to the application.</p>
    #[serde(rename = "forceTerminateApp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_terminate_app: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteAppResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteReplicationJobRequest {
    /// <p>The identifier of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    pub replication_job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteReplicationJobResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteServerCatalogRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteServerCatalogResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateConnectorRequest {
    /// <p>The identifier of the connector.</p>
    #[serde(rename = "connectorId")]
    pub connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisassociateConnectorResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GenerateChangeSetRequest {
    /// <p>ID of the application associated with the change set.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>Format for the change set.</p>
    #[serde(rename = "changesetFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changeset_format: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GenerateChangeSetResponse {
    /// <p>Location of the Amazon S3 object.</p>
    #[serde(rename = "s3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_location: Option<S3Location>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GenerateTemplateRequest {
    /// <p>ID of the application associated with the Amazon CloudFormation template.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>Format for generating the Amazon CloudFormation template.</p>
    #[serde(rename = "templateFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_format: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GenerateTemplateResponse {
    /// <p>Location of the Amazon S3 object.</p>
    #[serde(rename = "s3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_location: Option<S3Location>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAppLaunchConfigurationRequest {
    /// <p>ID of the application launch configuration.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAppLaunchConfigurationResponse {
    /// <p>ID of the application associated with the launch configuration.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>Name of the service role in the customer's account that Amazon CloudFormation uses to launch the application.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p>List of launch configurations for server groups in this application.</p>
    #[serde(rename = "serverGroupLaunchConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_launch_configurations: Option<Vec<ServerGroupLaunchConfiguration>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAppReplicationConfigurationRequest {
    /// <p>ID of the application associated with the replication configuration.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAppReplicationConfigurationResponse {
    /// <p>Replication configurations associated with server groups in this application.</p>
    #[serde(rename = "serverGroupReplicationConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_replication_configurations: Option<Vec<ServerGroupReplicationConfiguration>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAppRequest {
    /// <p>ID of the application whose information is being retrieved.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAppResponse {
    /// <p>Information about the application.</p>
    #[serde(rename = "appSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_summary: Option<AppSummary>,
    /// <p>List of server groups belonging to the application.</p>
    #[serde(rename = "serverGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_groups: Option<Vec<ServerGroup>>,
    /// <p>List of tags associated with the application.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetConnectorsRequest {
    /// <p>The maximum number of results to return in a single call. The default value is 50. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetConnectorsResponse {
    /// <p>Information about the registered connectors.</p>
    #[serde(rename = "connectorList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_list: Option<Vec<Connector>>,
    /// <p>The token required to retrieve the next set of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetReplicationJobsRequest {
    /// <p>The maximum number of results to return in a single call. The default value is 50. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetReplicationJobsResponse {
    /// <p>The token required to retrieve the next set of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the replication jobs.</p>
    #[serde(rename = "replicationJobList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_list: Option<Vec<ReplicationJob>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetReplicationRunsRequest {
    /// <p>The maximum number of results to return in a single call. The default value is 50. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    pub replication_job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetReplicationRunsResponse {
    /// <p>The token required to retrieve the next set of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the replication job.</p>
    #[serde(rename = "replicationJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job: Option<ReplicationJob>,
    /// <p>Information about the replication runs.</p>
    #[serde(rename = "replicationRunList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_run_list: Option<Vec<ReplicationRun>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetServersRequest {
    /// <p>The maximum number of results to return in a single call. The default value is 50. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of <code>VmServerAddress</code> objects</p>
    #[serde(rename = "vmServerAddressList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_server_address_list: Option<Vec<VmServerAddress>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetServersResponse {
    /// <p>The time when the server was last modified.</p>
    #[serde(rename = "lastModifiedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    /// <p>The token required to retrieve the next set of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The status of the server catalog.</p>
    #[serde(rename = "serverCatalogStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_catalog_status: Option<String>,
    /// <p>Information about the servers.</p>
    #[serde(rename = "serverList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_list: Option<Vec<Server>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportServerCatalogRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ImportServerCatalogResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct LaunchAppRequest {
    /// <p>ID of the application to launch.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LaunchAppResponse {}

/// <p>Details about the latest launch of an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LaunchDetails {
    /// <p>Latest time this application was launched successfully.</p>
    #[serde(rename = "latestLaunchTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_launch_time: Option<f64>,
    /// <p>Identifier of the latest stack launched for this application.</p>
    #[serde(rename = "stackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    /// <p>Name of the latest stack launched for this application.</p>
    #[serde(rename = "stackName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAppsRequest {
    /// <p><p/></p>
    #[serde(rename = "appIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_ids: Option<Vec<String>>,
    /// <p>The maximum number of results to return in a single call. The default value is 50. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value. </p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListAppsResponse {
    /// <p>A list of application summaries.</p>
    #[serde(rename = "apps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<AppSummary>>,
    /// <p>The token required to retrieve the next set of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutAppLaunchConfigurationRequest {
    /// <p>ID of the application associated with the launch configuration.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>Name of service role in the customer's account that Amazon CloudFormation uses to launch the application.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p>Launch configurations for server groups in the application.</p>
    #[serde(rename = "serverGroupLaunchConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_launch_configurations: Option<Vec<ServerGroupLaunchConfiguration>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutAppLaunchConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutAppReplicationConfigurationRequest {
    /// <p>ID of the application tassociated with the replication configuration.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>Replication configurations for server groups in the application.</p>
    #[serde(rename = "serverGroupReplicationConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_replication_configurations: Option<Vec<ServerGroupReplicationConfiguration>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutAppReplicationConfigurationResponse {}

/// <p>Represents a replication job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ReplicationJob {
    /// <p>The description of the replication job.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Whether the replication job should produce encrypted AMIs or not. See also <code>KmsKeyId</code> below.</p>
    #[serde(rename = "encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>The time between consecutive replication runs, in hours.</p>
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i64>,
    /// <p>KMS key ID for replication jobs that produce encrypted AMIs. Can be any of the following: </p> <ul> <li> <p>KMS key ID</p> </li> <li> <p>KMS key alias</p> </li> <li> <p>ARN referring to KMS key ID</p> </li> <li> <p>ARN referring to KMS key alias</p> </li> </ul> <p> If encrypted is <i>true</i> but a KMS key id is not specified, the customer's default KMS key for EBS is used. </p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The ID of the latest Amazon Machine Image (AMI).</p>
    #[serde(rename = "latestAmiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_ami_id: Option<String>,
    /// <p>The license type to be used for the AMI created by a successful replication run.</p>
    #[serde(rename = "licenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    /// <p>The start time of the next replication run.</p>
    #[serde(rename = "nextReplicationRunStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_replication_run_start_time: Option<f64>,
    /// <p>Number of recent AMIs to keep in the customer's account for a replication job. By default the value is set to zero, meaning that all AMIs are kept.</p>
    #[serde(rename = "numberOfRecentAmisToKeep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_recent_amis_to_keep: Option<i64>,
    /// <p>The identifier of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_id: Option<String>,
    /// <p>Information about the replication runs.</p>
    #[serde(rename = "replicationRunList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_run_list: Option<Vec<ReplicationRun>>,
    /// <p>The name of the IAM role to be used by the Server Migration Service.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "runOnce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_once: Option<bool>,
    /// <p>The seed replication time.</p>
    #[serde(rename = "seedReplicationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_replication_time: Option<f64>,
    /// <p>The identifier of the server.</p>
    #[serde(rename = "serverId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    /// <p>The type of server.</p>
    #[serde(rename = "serverType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    /// <p>The state of the replication job.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The description of the current status of the replication job.</p>
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Information about the VM server.</p>
    #[serde(rename = "vmServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_server: Option<VmServer>,
}

/// <p>Represents a replication run.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ReplicationRun {
    /// <p>The identifier of the Amazon Machine Image (AMI) from the replication run.</p>
    #[serde(rename = "amiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_id: Option<String>,
    /// <p>The completion time of the last replication run.</p>
    #[serde(rename = "completedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_time: Option<f64>,
    /// <p>The description of the replication run.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Whether the replication run should produce encrypted AMI or not. See also <code>KmsKeyId</code> below.</p>
    #[serde(rename = "encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>KMS key ID for replication jobs that produce encrypted AMIs. Can be any of the following: </p> <ul> <li> <p>KMS key ID</p> </li> <li> <p>KMS key alias</p> </li> <li> <p>ARN referring to KMS key ID</p> </li> <li> <p>ARN referring to KMS key alias</p> </li> </ul> <p> If encrypted is <i>true</i> but a KMS key id is not specified, the customer's default KMS key for EBS is used. </p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The identifier of the replication run.</p>
    #[serde(rename = "replicationRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_run_id: Option<String>,
    /// <p>The start time of the next replication run.</p>
    #[serde(rename = "scheduledStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_start_time: Option<f64>,
    /// <p>Details of the current stage of the replication run.</p>
    #[serde(rename = "stageDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_details: Option<ReplicationRunStageDetails>,
    /// <p>The state of the replication run.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The description of the current status of the replication job.</p>
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>The type of replication run.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Details of the current stage of a replication run.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ReplicationRunStageDetails {
    /// <p>String describing the current stage of a replication run.</p>
    #[serde(rename = "stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    /// <p>String describing the progress of the current stage of a replication run.</p>
    #[serde(rename = "stageProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_progress: Option<String>,
}

/// <p>Location of the Amazon S3 object in the customer's account.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3Location {
    /// <p>Amazon S3 bucket name.</p>
    #[serde(rename = "bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// <p>Amazon S3 bucket key.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

/// <p>Represents a server.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Server {
    /// <p>The identifier of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_id: Option<String>,
    /// <p>Indicates whether the replication job is deleted or failed.</p>
    #[serde(rename = "replicationJobTerminated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_terminated: Option<bool>,
    /// <p>The identifier of the server.</p>
    #[serde(rename = "serverId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    /// <p>The type of server.</p>
    #[serde(rename = "serverType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    /// <p>Information about the VM server.</p>
    #[serde(rename = "vmServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_server: Option<VmServer>,
}

/// <p>A logical grouping of servers.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerGroup {
    /// <p>Name of a server group.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Identifier of a server group.</p>
    #[serde(rename = "serverGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_id: Option<String>,
    /// <p>List of servers belonging to a server group.</p>
    #[serde(rename = "serverList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_list: Option<Vec<Server>>,
}

/// <p>Launch configuration for a server group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerGroupLaunchConfiguration {
    /// <p>Launch order of servers in the server group.</p>
    #[serde(rename = "launchOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_order: Option<i64>,
    /// <p>Identifier of the server group the launch configuration is associated with.</p>
    #[serde(rename = "serverGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_id: Option<String>,
    /// <p>Launch configuration for servers in the server group.</p>
    #[serde(rename = "serverLaunchConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_launch_configurations: Option<Vec<ServerLaunchConfiguration>>,
}

/// <p>Replication configuration for a server group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerGroupReplicationConfiguration {
    /// <p>Identifier of the server group this replication configuration is associated with.</p>
    #[serde(rename = "serverGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_id: Option<String>,
    /// <p>Replication configuration for servers in the server group.</p>
    #[serde(rename = "serverReplicationConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_replication_configurations: Option<Vec<ServerReplicationConfiguration>>,
}

/// <p>Launch configuration for a server.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerLaunchConfiguration {
    /// <p>If true, a publicly accessible IP address is created when launching the server.</p>
    #[serde(rename = "associatePublicIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associate_public_ip_address: Option<bool>,
    /// <p>Name of the EC2 SSH Key to be used for connecting to the launched server.</p>
    #[serde(rename = "ec2KeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_key_name: Option<String>,
    /// <p>Instance type to be used for launching the server.</p>
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>Logical ID of the server in the Amazon CloudFormation template.</p>
    #[serde(rename = "logicalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_id: Option<String>,
    /// <p>Identifier of the security group that applies to the launched server.</p>
    #[serde(rename = "securityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group: Option<String>,
    /// <p>Identifier of the server the launch configuration is associated with.</p>
    #[serde(rename = "server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,
    /// <p>Identifier of the subnet the server should be launched into.</p>
    #[serde(rename = "subnet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
    /// <p>Location of the user-data script to be executed when launching the server.</p>
    #[serde(rename = "userData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<UserData>,
    /// <p>Identifier of the VPC the server should be launched into.</p>
    #[serde(rename = "vpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<String>,
}

/// <p>Replication configuration of a server.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerReplicationConfiguration {
    /// <p>Identifier of the server this replication configuration is associated with.</p>
    #[serde(rename = "server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,
    /// <p>Parameters for replicating the server.</p>
    #[serde(rename = "serverReplicationParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_replication_parameters: Option<ServerReplicationParameters>,
}

/// <p>Replication parameters for replicating a server.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerReplicationParameters {
    /// <p>When true, the replication job produces encrypted AMIs. See also <code>KmsKeyId</code> below.</p>
    #[serde(rename = "encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>Frequency of creating replication jobs for the server.</p>
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i64>,
    /// <p><p/> <p>KMS key ID for replication jobs that produce encrypted AMIs. Can be any of the following: </p> <ul> <li> <p>KMS key ID</p> </li> <li> <p>KMS key alias</p> </li> <li> <p>ARN referring to KMS key ID</p> </li> <li> <p>ARN referring to KMS key alias</p> </li> </ul> <p> If encrypted is <i>true</i> but a KMS key id is not specified, the customer&#39;s default KMS key for EBS is used. </p></p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>License type for creating a replication job for the server.</p>
    #[serde(rename = "licenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    /// <p>Number of recent AMIs to keep when creating a replication job for this server.</p>
    #[serde(rename = "numberOfRecentAmisToKeep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_recent_amis_to_keep: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "runOnce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_once: Option<bool>,
    /// <p>Seed time for creating a replication job for the server.</p>
    #[serde(rename = "seedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartAppReplicationRequest {
    /// <p>ID of the application to replicate.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartAppReplicationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartOnDemandReplicationRunRequest {
    /// <p>The description of the replication run.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    pub replication_job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartOnDemandReplicationRunResponse {
    /// <p>The identifier of the replication run.</p>
    #[serde(rename = "replicationRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_run_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopAppReplicationRequest {
    /// <p>ID of the application to stop replicating.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StopAppReplicationResponse {}

/// <p>A label that can be assigned to an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>Tag key.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>Tag value.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TerminateAppRequest {
    /// <p>ID of the application to terminate.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TerminateAppResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAppRequest {
    /// <p>ID of the application to update.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>New description of the application.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>New name of the application.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Name of the service role in the customer's account used by AWS SMS.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p>List of server groups in the application to update.</p>
    #[serde(rename = "serverGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_groups: Option<Vec<ServerGroup>>,
    /// <p>List of tags to associate with the application.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateAppResponse {
    /// <p>Summary description of the application.</p>
    #[serde(rename = "appSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_summary: Option<AppSummary>,
    /// <p>List of updated server groups in the application.</p>
    #[serde(rename = "serverGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_groups: Option<Vec<ServerGroup>>,
    /// <p>List of tags associated with the application.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateReplicationJobRequest {
    /// <p>The description of the replication job.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>When true, the replication job produces encrypted AMIs . See also <code>KmsKeyId</code> below.</p>
    #[serde(rename = "encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>The time between consecutive replication runs, in hours.</p>
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i64>,
    /// <p><p/> <p>KMS key ID for replication jobs that produce encrypted AMIs. Can be any of the following: </p> <ul> <li> <p>KMS key ID</p> </li> <li> <p>KMS key alias</p> </li> <li> <p>ARN referring to KMS key ID</p> </li> <li> <p>ARN referring to KMS key alias</p> </li> </ul> <p> If encrypted is <i>true</i> but a KMS key id is not specified, the customer&#39;s default KMS key for EBS is used. </p></p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The license type to be used for the AMI created by a successful replication run.</p>
    #[serde(rename = "licenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    /// <p>The start time of the next replication run.</p>
    #[serde(rename = "nextReplicationRunStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_replication_run_start_time: Option<f64>,
    /// <p>The maximum number of SMS-created AMIs to retain. The oldest will be deleted once the maximum number is reached and a new AMI is created.</p>
    #[serde(rename = "numberOfRecentAmisToKeep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_recent_amis_to_keep: Option<i64>,
    /// <p>The identifier of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    pub replication_job_id: String,
    /// <p>The name of the IAM role to be used by AWS SMS.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateReplicationJobResponse {}

/// <p>A script that runs on first launch of an Amazon EC2 instance. Used for configuring the server during launch.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserData {
    /// <p>Amazon S3 location of the user-data script.</p>
    #[serde(rename = "s3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_location: Option<S3Location>,
}

/// <p>Represents a VM server.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VmServer {
    /// <p>The name of the VM manager.</p>
    #[serde(rename = "vmManagerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_name: Option<String>,
    /// <p>The type of VM management product.</p>
    #[serde(rename = "vmManagerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_type: Option<String>,
    /// <p>The name of the VM.</p>
    #[serde(rename = "vmName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_name: Option<String>,
    /// <p>The VM folder path in the vCenter Server virtual machine inventory tree.</p>
    #[serde(rename = "vmPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_path: Option<String>,
    /// <p>Information about the VM server location.</p>
    #[serde(rename = "vmServerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_server_address: Option<VmServerAddress>,
}

/// <p>Represents a VM server location.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VmServerAddress {
    /// <p>The identifier of the VM.</p>
    #[serde(rename = "vmId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    /// <p>The identifier of the VM manager.</p>
    #[serde(rename = "vmManagerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_id: Option<String>,
}

/// Errors returned by CreateApp
#[derive(Debug, PartialEq)]
pub enum CreateAppError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl CreateAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAppError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(CreateAppError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateAppError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(CreateAppError::MissingRequiredParameter(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(CreateAppError::OperationNotPermitted(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(CreateAppError::UnauthorizedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAppError {
    fn description(&self) -> &str {
        match *self {
            CreateAppError::InternalError(ref cause) => cause,
            CreateAppError::InvalidParameter(ref cause) => cause,
            CreateAppError::MissingRequiredParameter(ref cause) => cause,
            CreateAppError::OperationNotPermitted(ref cause) => cause,
            CreateAppError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateReplicationJob
#[derive(Debug, PartialEq)]
pub enum CreateReplicationJobError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>There are no connectors available.</p>
    NoConnectorsAvailable(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>The specified replication job already exists.</p>
    ReplicationJobAlreadyExists(String),
    /// <p>The specified server cannot be replicated.</p>
    ServerCannotBeReplicated(String),
    /// <p>The service is temporarily unavailable.</p>
    TemporarilyUnavailable(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl CreateReplicationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateReplicationJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(CreateReplicationJobError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateReplicationJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        CreateReplicationJobError::MissingRequiredParameter(err.msg),
                    )
                }
                "NoConnectorsAvailableException" => {
                    return RusotoError::Service(CreateReplicationJobError::NoConnectorsAvailable(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(CreateReplicationJobError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "ReplicationJobAlreadyExistsException" => {
                    return RusotoError::Service(
                        CreateReplicationJobError::ReplicationJobAlreadyExists(err.msg),
                    )
                }
                "ServerCannotBeReplicatedException" => {
                    return RusotoError::Service(
                        CreateReplicationJobError::ServerCannotBeReplicated(err.msg),
                    )
                }
                "TemporarilyUnavailableException" => {
                    return RusotoError::Service(CreateReplicationJobError::TemporarilyUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(CreateReplicationJobError::UnauthorizedOperation(
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
impl fmt::Display for CreateReplicationJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateReplicationJobError {
    fn description(&self) -> &str {
        match *self {
            CreateReplicationJobError::InternalError(ref cause) => cause,
            CreateReplicationJobError::InvalidParameter(ref cause) => cause,
            CreateReplicationJobError::MissingRequiredParameter(ref cause) => cause,
            CreateReplicationJobError::NoConnectorsAvailable(ref cause) => cause,
            CreateReplicationJobError::OperationNotPermitted(ref cause) => cause,
            CreateReplicationJobError::ReplicationJobAlreadyExists(ref cause) => cause,
            CreateReplicationJobError::ServerCannotBeReplicated(ref cause) => cause,
            CreateReplicationJobError::TemporarilyUnavailable(ref cause) => cause,
            CreateReplicationJobError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApp
#[derive(Debug, PartialEq)]
pub enum DeleteAppError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl DeleteAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAppError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(DeleteAppError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteAppError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(DeleteAppError::MissingRequiredParameter(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(DeleteAppError::OperationNotPermitted(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(DeleteAppError::UnauthorizedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAppError {
    fn description(&self) -> &str {
        match *self {
            DeleteAppError::InternalError(ref cause) => cause,
            DeleteAppError::InvalidParameter(ref cause) => cause,
            DeleteAppError::MissingRequiredParameter(ref cause) => cause,
            DeleteAppError::OperationNotPermitted(ref cause) => cause,
            DeleteAppError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAppLaunchConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteAppLaunchConfigurationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl DeleteAppLaunchConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteAppLaunchConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(DeleteAppLaunchConfigurationError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DeleteAppLaunchConfigurationError::InvalidParameter(err.msg),
                    )
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        DeleteAppLaunchConfigurationError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        DeleteAppLaunchConfigurationError::OperationNotPermitted(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        DeleteAppLaunchConfigurationError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteAppLaunchConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAppLaunchConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteAppLaunchConfigurationError::InternalError(ref cause) => cause,
            DeleteAppLaunchConfigurationError::InvalidParameter(ref cause) => cause,
            DeleteAppLaunchConfigurationError::MissingRequiredParameter(ref cause) => cause,
            DeleteAppLaunchConfigurationError::OperationNotPermitted(ref cause) => cause,
            DeleteAppLaunchConfigurationError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAppReplicationConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteAppReplicationConfigurationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl DeleteAppReplicationConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteAppReplicationConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(
                        DeleteAppReplicationConfigurationError::InternalError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DeleteAppReplicationConfigurationError::InvalidParameter(err.msg),
                    )
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        DeleteAppReplicationConfigurationError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        DeleteAppReplicationConfigurationError::OperationNotPermitted(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        DeleteAppReplicationConfigurationError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteAppReplicationConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAppReplicationConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteAppReplicationConfigurationError::InternalError(ref cause) => cause,
            DeleteAppReplicationConfigurationError::InvalidParameter(ref cause) => cause,
            DeleteAppReplicationConfigurationError::MissingRequiredParameter(ref cause) => cause,
            DeleteAppReplicationConfigurationError::OperationNotPermitted(ref cause) => cause,
            DeleteAppReplicationConfigurationError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteReplicationJob
#[derive(Debug, PartialEq)]
pub enum DeleteReplicationJobError {
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>The specified replication job does not exist.</p>
    ReplicationJobNotFound(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl DeleteReplicationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteReplicationJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteReplicationJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        DeleteReplicationJobError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(DeleteReplicationJobError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "ReplicationJobNotFoundException" => {
                    return RusotoError::Service(DeleteReplicationJobError::ReplicationJobNotFound(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(DeleteReplicationJobError::UnauthorizedOperation(
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
impl fmt::Display for DeleteReplicationJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteReplicationJobError {
    fn description(&self) -> &str {
        match *self {
            DeleteReplicationJobError::InvalidParameter(ref cause) => cause,
            DeleteReplicationJobError::MissingRequiredParameter(ref cause) => cause,
            DeleteReplicationJobError::OperationNotPermitted(ref cause) => cause,
            DeleteReplicationJobError::ReplicationJobNotFound(ref cause) => cause,
            DeleteReplicationJobError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteServerCatalog
#[derive(Debug, PartialEq)]
pub enum DeleteServerCatalogError {
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl DeleteServerCatalogError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteServerCatalogError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteServerCatalogError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        DeleteServerCatalogError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(DeleteServerCatalogError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(DeleteServerCatalogError::UnauthorizedOperation(
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
impl fmt::Display for DeleteServerCatalogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteServerCatalogError {
    fn description(&self) -> &str {
        match *self {
            DeleteServerCatalogError::InvalidParameter(ref cause) => cause,
            DeleteServerCatalogError::MissingRequiredParameter(ref cause) => cause,
            DeleteServerCatalogError::OperationNotPermitted(ref cause) => cause,
            DeleteServerCatalogError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateConnector
#[derive(Debug, PartialEq)]
pub enum DisassociateConnectorError {
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl DisassociateConnectorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateConnectorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DisassociateConnectorError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        DisassociateConnectorError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(DisassociateConnectorError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(DisassociateConnectorError::UnauthorizedOperation(
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
impl fmt::Display for DisassociateConnectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateConnectorError {
    fn description(&self) -> &str {
        match *self {
            DisassociateConnectorError::InvalidParameter(ref cause) => cause,
            DisassociateConnectorError::MissingRequiredParameter(ref cause) => cause,
            DisassociateConnectorError::OperationNotPermitted(ref cause) => cause,
            DisassociateConnectorError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by GenerateChangeSet
#[derive(Debug, PartialEq)]
pub enum GenerateChangeSetError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl GenerateChangeSetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GenerateChangeSetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(GenerateChangeSetError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GenerateChangeSetError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(GenerateChangeSetError::MissingRequiredParameter(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(GenerateChangeSetError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(GenerateChangeSetError::UnauthorizedOperation(
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
impl fmt::Display for GenerateChangeSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GenerateChangeSetError {
    fn description(&self) -> &str {
        match *self {
            GenerateChangeSetError::InternalError(ref cause) => cause,
            GenerateChangeSetError::InvalidParameter(ref cause) => cause,
            GenerateChangeSetError::MissingRequiredParameter(ref cause) => cause,
            GenerateChangeSetError::OperationNotPermitted(ref cause) => cause,
            GenerateChangeSetError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by GenerateTemplate
#[derive(Debug, PartialEq)]
pub enum GenerateTemplateError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl GenerateTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GenerateTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(GenerateTemplateError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GenerateTemplateError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(GenerateTemplateError::MissingRequiredParameter(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(GenerateTemplateError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(GenerateTemplateError::UnauthorizedOperation(
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
impl fmt::Display for GenerateTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GenerateTemplateError {
    fn description(&self) -> &str {
        match *self {
            GenerateTemplateError::InternalError(ref cause) => cause,
            GenerateTemplateError::InvalidParameter(ref cause) => cause,
            GenerateTemplateError::MissingRequiredParameter(ref cause) => cause,
            GenerateTemplateError::OperationNotPermitted(ref cause) => cause,
            GenerateTemplateError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApp
#[derive(Debug, PartialEq)]
pub enum GetAppError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl GetAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAppError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(GetAppError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetAppError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(GetAppError::MissingRequiredParameter(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(GetAppError::OperationNotPermitted(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(GetAppError::UnauthorizedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAppError {
    fn description(&self) -> &str {
        match *self {
            GetAppError::InternalError(ref cause) => cause,
            GetAppError::InvalidParameter(ref cause) => cause,
            GetAppError::MissingRequiredParameter(ref cause) => cause,
            GetAppError::OperationNotPermitted(ref cause) => cause,
            GetAppError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAppLaunchConfiguration
#[derive(Debug, PartialEq)]
pub enum GetAppLaunchConfigurationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl GetAppLaunchConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAppLaunchConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(GetAppLaunchConfigurationError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetAppLaunchConfigurationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        GetAppLaunchConfigurationError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        GetAppLaunchConfigurationError::OperationNotPermitted(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        GetAppLaunchConfigurationError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAppLaunchConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAppLaunchConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetAppLaunchConfigurationError::InternalError(ref cause) => cause,
            GetAppLaunchConfigurationError::InvalidParameter(ref cause) => cause,
            GetAppLaunchConfigurationError::MissingRequiredParameter(ref cause) => cause,
            GetAppLaunchConfigurationError::OperationNotPermitted(ref cause) => cause,
            GetAppLaunchConfigurationError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAppReplicationConfiguration
#[derive(Debug, PartialEq)]
pub enum GetAppReplicationConfigurationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl GetAppReplicationConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetAppReplicationConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(
                        GetAppReplicationConfigurationError::InternalError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        GetAppReplicationConfigurationError::InvalidParameter(err.msg),
                    )
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        GetAppReplicationConfigurationError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        GetAppReplicationConfigurationError::OperationNotPermitted(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        GetAppReplicationConfigurationError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAppReplicationConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAppReplicationConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetAppReplicationConfigurationError::InternalError(ref cause) => cause,
            GetAppReplicationConfigurationError::InvalidParameter(ref cause) => cause,
            GetAppReplicationConfigurationError::MissingRequiredParameter(ref cause) => cause,
            GetAppReplicationConfigurationError::OperationNotPermitted(ref cause) => cause,
            GetAppReplicationConfigurationError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by GetConnectors
#[derive(Debug, PartialEq)]
pub enum GetConnectorsError {
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl GetConnectorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetConnectorsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(GetConnectorsError::UnauthorizedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetConnectorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetConnectorsError {
    fn description(&self) -> &str {
        match *self {
            GetConnectorsError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by GetReplicationJobs
#[derive(Debug, PartialEq)]
pub enum GetReplicationJobsError {
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl GetReplicationJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetReplicationJobsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(GetReplicationJobsError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(GetReplicationJobsError::MissingRequiredParameter(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(GetReplicationJobsError::UnauthorizedOperation(
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
impl fmt::Display for GetReplicationJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetReplicationJobsError {
    fn description(&self) -> &str {
        match *self {
            GetReplicationJobsError::InvalidParameter(ref cause) => cause,
            GetReplicationJobsError::MissingRequiredParameter(ref cause) => cause,
            GetReplicationJobsError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by GetReplicationRuns
#[derive(Debug, PartialEq)]
pub enum GetReplicationRunsError {
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl GetReplicationRunsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetReplicationRunsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(GetReplicationRunsError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(GetReplicationRunsError::MissingRequiredParameter(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(GetReplicationRunsError::UnauthorizedOperation(
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
impl fmt::Display for GetReplicationRunsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetReplicationRunsError {
    fn description(&self) -> &str {
        match *self {
            GetReplicationRunsError::InvalidParameter(ref cause) => cause,
            GetReplicationRunsError::MissingRequiredParameter(ref cause) => cause,
            GetReplicationRunsError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by GetServers
#[derive(Debug, PartialEq)]
pub enum GetServersError {
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl GetServersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetServersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(GetServersError::UnauthorizedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetServersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetServersError {
    fn description(&self) -> &str {
        match *self {
            GetServersError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by ImportServerCatalog
#[derive(Debug, PartialEq)]
pub enum ImportServerCatalogError {
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>There are no connectors available.</p>
    NoConnectorsAvailable(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl ImportServerCatalogError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportServerCatalogError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(ImportServerCatalogError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        ImportServerCatalogError::MissingRequiredParameter(err.msg),
                    )
                }
                "NoConnectorsAvailableException" => {
                    return RusotoError::Service(ImportServerCatalogError::NoConnectorsAvailable(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(ImportServerCatalogError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(ImportServerCatalogError::UnauthorizedOperation(
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
impl fmt::Display for ImportServerCatalogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ImportServerCatalogError {
    fn description(&self) -> &str {
        match *self {
            ImportServerCatalogError::InvalidParameter(ref cause) => cause,
            ImportServerCatalogError::MissingRequiredParameter(ref cause) => cause,
            ImportServerCatalogError::NoConnectorsAvailable(ref cause) => cause,
            ImportServerCatalogError::OperationNotPermitted(ref cause) => cause,
            ImportServerCatalogError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by LaunchApp
#[derive(Debug, PartialEq)]
pub enum LaunchAppError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl LaunchAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<LaunchAppError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(LaunchAppError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(LaunchAppError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(LaunchAppError::MissingRequiredParameter(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(LaunchAppError::OperationNotPermitted(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(LaunchAppError::UnauthorizedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for LaunchAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for LaunchAppError {
    fn description(&self) -> &str {
        match *self {
            LaunchAppError::InternalError(ref cause) => cause,
            LaunchAppError::InvalidParameter(ref cause) => cause,
            LaunchAppError::MissingRequiredParameter(ref cause) => cause,
            LaunchAppError::OperationNotPermitted(ref cause) => cause,
            LaunchAppError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by ListApps
#[derive(Debug, PartialEq)]
pub enum ListAppsError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl ListAppsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAppsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(ListAppsError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListAppsError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(ListAppsError::MissingRequiredParameter(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(ListAppsError::OperationNotPermitted(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(ListAppsError::UnauthorizedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAppsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAppsError {
    fn description(&self) -> &str {
        match *self {
            ListAppsError::InternalError(ref cause) => cause,
            ListAppsError::InvalidParameter(ref cause) => cause,
            ListAppsError::MissingRequiredParameter(ref cause) => cause,
            ListAppsError::OperationNotPermitted(ref cause) => cause,
            ListAppsError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by PutAppLaunchConfiguration
#[derive(Debug, PartialEq)]
pub enum PutAppLaunchConfigurationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl PutAppLaunchConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutAppLaunchConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(PutAppLaunchConfigurationError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(PutAppLaunchConfigurationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        PutAppLaunchConfigurationError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        PutAppLaunchConfigurationError::OperationNotPermitted(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        PutAppLaunchConfigurationError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutAppLaunchConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutAppLaunchConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutAppLaunchConfigurationError::InternalError(ref cause) => cause,
            PutAppLaunchConfigurationError::InvalidParameter(ref cause) => cause,
            PutAppLaunchConfigurationError::MissingRequiredParameter(ref cause) => cause,
            PutAppLaunchConfigurationError::OperationNotPermitted(ref cause) => cause,
            PutAppLaunchConfigurationError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by PutAppReplicationConfiguration
#[derive(Debug, PartialEq)]
pub enum PutAppReplicationConfigurationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl PutAppReplicationConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutAppReplicationConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(
                        PutAppReplicationConfigurationError::InternalError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        PutAppReplicationConfigurationError::InvalidParameter(err.msg),
                    )
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        PutAppReplicationConfigurationError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        PutAppReplicationConfigurationError::OperationNotPermitted(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        PutAppReplicationConfigurationError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutAppReplicationConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutAppReplicationConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutAppReplicationConfigurationError::InternalError(ref cause) => cause,
            PutAppReplicationConfigurationError::InvalidParameter(ref cause) => cause,
            PutAppReplicationConfigurationError::MissingRequiredParameter(ref cause) => cause,
            PutAppReplicationConfigurationError::OperationNotPermitted(ref cause) => cause,
            PutAppReplicationConfigurationError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by StartAppReplication
#[derive(Debug, PartialEq)]
pub enum StartAppReplicationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl StartAppReplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartAppReplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(StartAppReplicationError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartAppReplicationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        StartAppReplicationError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(StartAppReplicationError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(StartAppReplicationError::UnauthorizedOperation(
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
impl fmt::Display for StartAppReplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartAppReplicationError {
    fn description(&self) -> &str {
        match *self {
            StartAppReplicationError::InternalError(ref cause) => cause,
            StartAppReplicationError::InvalidParameter(ref cause) => cause,
            StartAppReplicationError::MissingRequiredParameter(ref cause) => cause,
            StartAppReplicationError::OperationNotPermitted(ref cause) => cause,
            StartAppReplicationError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by StartOnDemandReplicationRun
#[derive(Debug, PartialEq)]
pub enum StartOnDemandReplicationRunError {
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You have exceeded the number of on-demand replication runs you can request in a 24-hour period.</p>
    ReplicationRunLimitExceeded(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl StartOnDemandReplicationRunError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartOnDemandReplicationRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        StartOnDemandReplicationRunError::InvalidParameter(err.msg),
                    )
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        StartOnDemandReplicationRunError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        StartOnDemandReplicationRunError::OperationNotPermitted(err.msg),
                    )
                }
                "ReplicationRunLimitExceededException" => {
                    return RusotoError::Service(
                        StartOnDemandReplicationRunError::ReplicationRunLimitExceeded(err.msg),
                    )
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(
                        StartOnDemandReplicationRunError::UnauthorizedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartOnDemandReplicationRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartOnDemandReplicationRunError {
    fn description(&self) -> &str {
        match *self {
            StartOnDemandReplicationRunError::InvalidParameter(ref cause) => cause,
            StartOnDemandReplicationRunError::MissingRequiredParameter(ref cause) => cause,
            StartOnDemandReplicationRunError::OperationNotPermitted(ref cause) => cause,
            StartOnDemandReplicationRunError::ReplicationRunLimitExceeded(ref cause) => cause,
            StartOnDemandReplicationRunError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by StopAppReplication
#[derive(Debug, PartialEq)]
pub enum StopAppReplicationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl StopAppReplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopAppReplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(StopAppReplicationError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StopAppReplicationError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(StopAppReplicationError::MissingRequiredParameter(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(StopAppReplicationError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(StopAppReplicationError::UnauthorizedOperation(
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
impl fmt::Display for StopAppReplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopAppReplicationError {
    fn description(&self) -> &str {
        match *self {
            StopAppReplicationError::InternalError(ref cause) => cause,
            StopAppReplicationError::InvalidParameter(ref cause) => cause,
            StopAppReplicationError::MissingRequiredParameter(ref cause) => cause,
            StopAppReplicationError::OperationNotPermitted(ref cause) => cause,
            StopAppReplicationError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by TerminateApp
#[derive(Debug, PartialEq)]
pub enum TerminateAppError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl TerminateAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TerminateAppError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(TerminateAppError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(TerminateAppError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(TerminateAppError::MissingRequiredParameter(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(TerminateAppError::OperationNotPermitted(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(TerminateAppError::UnauthorizedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for TerminateAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TerminateAppError {
    fn description(&self) -> &str {
        match *self {
            TerminateAppError::InternalError(ref cause) => cause,
            TerminateAppError::InvalidParameter(ref cause) => cause,
            TerminateAppError::MissingRequiredParameter(ref cause) => cause,
            TerminateAppError::OperationNotPermitted(ref cause) => cause,
            TerminateAppError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApp
#[derive(Debug, PartialEq)]
pub enum UpdateAppError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl UpdateAppError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAppError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(UpdateAppError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateAppError::InvalidParameter(err.msg))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(UpdateAppError::MissingRequiredParameter(err.msg))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(UpdateAppError::OperationNotPermitted(err.msg))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(UpdateAppError::UnauthorizedOperation(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAppError {
    fn description(&self) -> &str {
        match *self {
            UpdateAppError::InternalError(ref cause) => cause,
            UpdateAppError::InvalidParameter(ref cause) => cause,
            UpdateAppError::MissingRequiredParameter(ref cause) => cause,
            UpdateAppError::OperationNotPermitted(ref cause) => cause,
            UpdateAppError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateReplicationJob
#[derive(Debug, PartialEq)]
pub enum UpdateReplicationJobError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>The specified replication job does not exist.</p>
    ReplicationJobNotFound(String),
    /// <p>The specified server cannot be replicated.</p>
    ServerCannotBeReplicated(String),
    /// <p>The service is temporarily unavailable.</p>
    TemporarilyUnavailable(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
}

impl UpdateReplicationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateReplicationJobError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalError" => {
                    return RusotoError::Service(UpdateReplicationJobError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateReplicationJobError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MissingRequiredParameterException" => {
                    return RusotoError::Service(
                        UpdateReplicationJobError::MissingRequiredParameter(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(UpdateReplicationJobError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "ReplicationJobNotFoundException" => {
                    return RusotoError::Service(UpdateReplicationJobError::ReplicationJobNotFound(
                        err.msg,
                    ))
                }
                "ServerCannotBeReplicatedException" => {
                    return RusotoError::Service(
                        UpdateReplicationJobError::ServerCannotBeReplicated(err.msg),
                    )
                }
                "TemporarilyUnavailableException" => {
                    return RusotoError::Service(UpdateReplicationJobError::TemporarilyUnavailable(
                        err.msg,
                    ))
                }
                "UnauthorizedOperationException" => {
                    return RusotoError::Service(UpdateReplicationJobError::UnauthorizedOperation(
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
impl fmt::Display for UpdateReplicationJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateReplicationJobError {
    fn description(&self) -> &str {
        match *self {
            UpdateReplicationJobError::InternalError(ref cause) => cause,
            UpdateReplicationJobError::InvalidParameter(ref cause) => cause,
            UpdateReplicationJobError::MissingRequiredParameter(ref cause) => cause,
            UpdateReplicationJobError::OperationNotPermitted(ref cause) => cause,
            UpdateReplicationJobError::ReplicationJobNotFound(ref cause) => cause,
            UpdateReplicationJobError::ServerCannotBeReplicated(ref cause) => cause,
            UpdateReplicationJobError::TemporarilyUnavailable(ref cause) => cause,
            UpdateReplicationJobError::UnauthorizedOperation(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the SMS API. SMS clients implement this trait.
pub trait ServerMigrationService {
    /// <p>Creates an application. An application consists of one or more server groups. Each server group contain one or more servers.</p>
    fn create_app(
        &self,
        input: CreateAppRequest,
    ) -> RusotoFuture<CreateAppResponse, CreateAppError>;

    /// <p>Creates a replication job. The replication job schedules periodic replication runs to replicate your server to AWS. Each replication run creates an Amazon Machine Image (AMI).</p>
    fn create_replication_job(
        &self,
        input: CreateReplicationJobRequest,
    ) -> RusotoFuture<CreateReplicationJobResponse, CreateReplicationJobError>;

    /// <p>Deletes an existing application. Optionally deletes the launched stack associated with the application and all AWS SMS replication jobs for servers in the application.</p>
    fn delete_app(
        &self,
        input: DeleteAppRequest,
    ) -> RusotoFuture<DeleteAppResponse, DeleteAppError>;

    /// <p>Deletes existing launch configuration for an application.</p>
    fn delete_app_launch_configuration(
        &self,
        input: DeleteAppLaunchConfigurationRequest,
    ) -> RusotoFuture<DeleteAppLaunchConfigurationResponse, DeleteAppLaunchConfigurationError>;

    /// <p>Deletes existing replication configuration for an application.</p>
    fn delete_app_replication_configuration(
        &self,
        input: DeleteAppReplicationConfigurationRequest,
    ) -> RusotoFuture<
        DeleteAppReplicationConfigurationResponse,
        DeleteAppReplicationConfigurationError,
    >;

    /// <p>Deletes the specified replication job.</p> <p>After you delete a replication job, there are no further replication runs. AWS deletes the contents of the Amazon S3 bucket used to store AWS SMS artifacts. The AMIs created by the replication runs are not deleted.</p>
    fn delete_replication_job(
        &self,
        input: DeleteReplicationJobRequest,
    ) -> RusotoFuture<DeleteReplicationJobResponse, DeleteReplicationJobError>;

    /// <p>Deletes all servers from your server catalog.</p>
    fn delete_server_catalog(
        &self,
    ) -> RusotoFuture<DeleteServerCatalogResponse, DeleteServerCatalogError>;

    /// <p>Disassociates the specified connector from AWS SMS.</p> <p>After you disassociate a connector, it is no longer available to support replication jobs.</p>
    fn disassociate_connector(
        &self,
        input: DisassociateConnectorRequest,
    ) -> RusotoFuture<DisassociateConnectorResponse, DisassociateConnectorError>;

    /// <p>Generates a target change set for a currently launched stack and writes it to an Amazon S3 object in the customer’s Amazon S3 bucket.</p>
    fn generate_change_set(
        &self,
        input: GenerateChangeSetRequest,
    ) -> RusotoFuture<GenerateChangeSetResponse, GenerateChangeSetError>;

    /// <p>Generates an Amazon CloudFormation template based on the current launch configuration and writes it to an Amazon S3 object in the customer’s Amazon S3 bucket.</p>
    fn generate_template(
        &self,
        input: GenerateTemplateRequest,
    ) -> RusotoFuture<GenerateTemplateResponse, GenerateTemplateError>;

    /// <p>Retrieve information about an application.</p>
    fn get_app(&self, input: GetAppRequest) -> RusotoFuture<GetAppResponse, GetAppError>;

    /// <p>Retrieves the application launch configuration associated with an application.</p>
    fn get_app_launch_configuration(
        &self,
        input: GetAppLaunchConfigurationRequest,
    ) -> RusotoFuture<GetAppLaunchConfigurationResponse, GetAppLaunchConfigurationError>;

    /// <p>Retrieves an application replication configuration associatd with an application.</p>
    fn get_app_replication_configuration(
        &self,
        input: GetAppReplicationConfigurationRequest,
    ) -> RusotoFuture<GetAppReplicationConfigurationResponse, GetAppReplicationConfigurationError>;

    /// <p>Describes the connectors registered with the AWS SMS.</p>
    fn get_connectors(
        &self,
        input: GetConnectorsRequest,
    ) -> RusotoFuture<GetConnectorsResponse, GetConnectorsError>;

    /// <p>Describes the specified replication job or all of your replication jobs.</p>
    fn get_replication_jobs(
        &self,
        input: GetReplicationJobsRequest,
    ) -> RusotoFuture<GetReplicationJobsResponse, GetReplicationJobsError>;

    /// <p>Describes the replication runs for the specified replication job.</p>
    fn get_replication_runs(
        &self,
        input: GetReplicationRunsRequest,
    ) -> RusotoFuture<GetReplicationRunsResponse, GetReplicationRunsError>;

    /// <p>Describes the servers in your server catalog.</p> <p>Before you can describe your servers, you must import them using <a>ImportServerCatalog</a>.</p>
    fn get_servers(
        &self,
        input: GetServersRequest,
    ) -> RusotoFuture<GetServersResponse, GetServersError>;

    /// <p>Gathers a complete list of on-premises servers. Connectors must be installed and monitoring all servers that you want to import.</p> <p>This call returns immediately, but might take additional time to retrieve all the servers.</p>
    fn import_server_catalog(
        &self,
    ) -> RusotoFuture<ImportServerCatalogResponse, ImportServerCatalogError>;

    /// <p>Launches an application stack.</p>
    fn launch_app(
        &self,
        input: LaunchAppRequest,
    ) -> RusotoFuture<LaunchAppResponse, LaunchAppError>;

    /// <p>Returns a list of summaries for all applications.</p>
    fn list_apps(&self, input: ListAppsRequest) -> RusotoFuture<ListAppsResponse, ListAppsError>;

    /// <p>Creates a launch configuration for an application.</p>
    fn put_app_launch_configuration(
        &self,
        input: PutAppLaunchConfigurationRequest,
    ) -> RusotoFuture<PutAppLaunchConfigurationResponse, PutAppLaunchConfigurationError>;

    /// <p>Creates or updates a replication configuration for an application.</p>
    fn put_app_replication_configuration(
        &self,
        input: PutAppReplicationConfigurationRequest,
    ) -> RusotoFuture<PutAppReplicationConfigurationResponse, PutAppReplicationConfigurationError>;

    /// <p>Starts replicating an application.</p>
    fn start_app_replication(
        &self,
        input: StartAppReplicationRequest,
    ) -> RusotoFuture<StartAppReplicationResponse, StartAppReplicationError>;

    /// <p>Starts an on-demand replication run for the specified replication job. This replication run starts immediately. This replication run is in addition to the ones already scheduled.</p> <p>There is a limit on the number of on-demand replications runs you can request in a 24-hour period.</p>
    fn start_on_demand_replication_run(
        &self,
        input: StartOnDemandReplicationRunRequest,
    ) -> RusotoFuture<StartOnDemandReplicationRunResponse, StartOnDemandReplicationRunError>;

    /// <p>Stops replicating an application.</p>
    fn stop_app_replication(
        &self,
        input: StopAppReplicationRequest,
    ) -> RusotoFuture<StopAppReplicationResponse, StopAppReplicationError>;

    /// <p>Terminates the stack for an application.</p>
    fn terminate_app(
        &self,
        input: TerminateAppRequest,
    ) -> RusotoFuture<TerminateAppResponse, TerminateAppError>;

    /// <p>Updates an application.</p>
    fn update_app(
        &self,
        input: UpdateAppRequest,
    ) -> RusotoFuture<UpdateAppResponse, UpdateAppError>;

    /// <p>Updates the specified settings for the specified replication job.</p>
    fn update_replication_job(
        &self,
        input: UpdateReplicationJobRequest,
    ) -> RusotoFuture<UpdateReplicationJobResponse, UpdateReplicationJobError>;
}
/// A client for the SMS API.
#[derive(Clone)]
pub struct ServerMigrationServiceClient {
    client: Client,
    region: region::Region,
}

impl ServerMigrationServiceClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ServerMigrationServiceClient {
        ServerMigrationServiceClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ServerMigrationServiceClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        ServerMigrationServiceClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }
}

impl ServerMigrationService for ServerMigrationServiceClient {
    /// <p>Creates an application. An application consists of one or more server groups. Each server group contain one or more servers.</p>
    fn create_app(
        &self,
        input: CreateAppRequest,
    ) -> RusotoFuture<CreateAppResponse, CreateAppError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.CreateApp",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateAppResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateAppError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a replication job. The replication job schedules periodic replication runs to replicate your server to AWS. Each replication run creates an Amazon Machine Image (AMI).</p>
    fn create_replication_job(
        &self,
        input: CreateReplicationJobRequest,
    ) -> RusotoFuture<CreateReplicationJobResponse, CreateReplicationJobError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.CreateReplicationJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateReplicationJobResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateReplicationJobError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes an existing application. Optionally deletes the launched stack associated with the application and all AWS SMS replication jobs for servers in the application.</p>
    fn delete_app(
        &self,
        input: DeleteAppRequest,
    ) -> RusotoFuture<DeleteAppResponse, DeleteAppError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.DeleteApp",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteAppResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteAppError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes existing launch configuration for an application.</p>
    fn delete_app_launch_configuration(
        &self,
        input: DeleteAppLaunchConfigurationRequest,
    ) -> RusotoFuture<DeleteAppLaunchConfigurationResponse, DeleteAppLaunchConfigurationError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.DeleteAppLaunchConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteAppLaunchConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteAppLaunchConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes existing replication configuration for an application.</p>
    fn delete_app_replication_configuration(
        &self,
        input: DeleteAppReplicationConfigurationRequest,
    ) -> RusotoFuture<
        DeleteAppReplicationConfigurationResponse,
        DeleteAppReplicationConfigurationError,
    > {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.DeleteAppReplicationConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteAppReplicationConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteAppReplicationConfigurationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Deletes the specified replication job.</p> <p>After you delete a replication job, there are no further replication runs. AWS deletes the contents of the Amazon S3 bucket used to store AWS SMS artifacts. The AMIs created by the replication runs are not deleted.</p>
    fn delete_replication_job(
        &self,
        input: DeleteReplicationJobRequest,
    ) -> RusotoFuture<DeleteReplicationJobResponse, DeleteReplicationJobError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.DeleteReplicationJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteReplicationJobResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteReplicationJobError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes all servers from your server catalog.</p>
    fn delete_server_catalog(
        &self,
    ) -> RusotoFuture<DeleteServerCatalogResponse, DeleteServerCatalogError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.DeleteServerCatalog",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteServerCatalogResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteServerCatalogError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Disassociates the specified connector from AWS SMS.</p> <p>After you disassociate a connector, it is no longer available to support replication jobs.</p>
    fn disassociate_connector(
        &self,
        input: DisassociateConnectorRequest,
    ) -> RusotoFuture<DisassociateConnectorResponse, DisassociateConnectorError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.DisassociateConnector",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DisassociateConnectorResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DisassociateConnectorError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Generates a target change set for a currently launched stack and writes it to an Amazon S3 object in the customer’s Amazon S3 bucket.</p>
    fn generate_change_set(
        &self,
        input: GenerateChangeSetRequest,
    ) -> RusotoFuture<GenerateChangeSetResponse, GenerateChangeSetError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GenerateChangeSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GenerateChangeSetResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GenerateChangeSetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Generates an Amazon CloudFormation template based on the current launch configuration and writes it to an Amazon S3 object in the customer’s Amazon S3 bucket.</p>
    fn generate_template(
        &self,
        input: GenerateTemplateRequest,
    ) -> RusotoFuture<GenerateTemplateResponse, GenerateTemplateError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GenerateTemplate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GenerateTemplateResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GenerateTemplateError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieve information about an application.</p>
    fn get_app(&self, input: GetAppRequest) -> RusotoFuture<GetAppResponse, GetAppError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetApp",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<GetAppResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAppError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the application launch configuration associated with an application.</p>
    fn get_app_launch_configuration(
        &self,
        input: GetAppLaunchConfigurationRequest,
    ) -> RusotoFuture<GetAppLaunchConfigurationResponse, GetAppLaunchConfigurationError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetAppLaunchConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetAppLaunchConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetAppLaunchConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves an application replication configuration associatd with an application.</p>
    fn get_app_replication_configuration(
        &self,
        input: GetAppReplicationConfigurationRequest,
    ) -> RusotoFuture<GetAppReplicationConfigurationResponse, GetAppReplicationConfigurationError>
    {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetAppReplicationConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetAppReplicationConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetAppReplicationConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Describes the connectors registered with the AWS SMS.</p>
    fn get_connectors(
        &self,
        input: GetConnectorsRequest,
    ) -> RusotoFuture<GetConnectorsResponse, GetConnectorsError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetConnectors",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetConnectorsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetConnectorsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes the specified replication job or all of your replication jobs.</p>
    fn get_replication_jobs(
        &self,
        input: GetReplicationJobsRequest,
    ) -> RusotoFuture<GetReplicationJobsResponse, GetReplicationJobsError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetReplicationJobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetReplicationJobsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetReplicationJobsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes the replication runs for the specified replication job.</p>
    fn get_replication_runs(
        &self,
        input: GetReplicationRunsRequest,
    ) -> RusotoFuture<GetReplicationRunsResponse, GetReplicationRunsError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetReplicationRuns",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetReplicationRunsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetReplicationRunsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes the servers in your server catalog.</p> <p>Before you can describe your servers, you must import them using <a>ImportServerCatalog</a>.</p>
    fn get_servers(
        &self,
        input: GetServersRequest,
    ) -> RusotoFuture<GetServersResponse, GetServersError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetServers",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetServersResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetServersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gathers a complete list of on-premises servers. Connectors must be installed and monitoring all servers that you want to import.</p> <p>This call returns immediately, but might take additional time to retrieve all the servers.</p>
    fn import_server_catalog(
        &self,
    ) -> RusotoFuture<ImportServerCatalogResponse, ImportServerCatalogError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.ImportServerCatalog",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ImportServerCatalogResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ImportServerCatalogError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Launches an application stack.</p>
    fn launch_app(
        &self,
        input: LaunchAppRequest,
    ) -> RusotoFuture<LaunchAppResponse, LaunchAppError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.LaunchApp",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<LaunchAppResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(LaunchAppError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of summaries for all applications.</p>
    fn list_apps(&self, input: ListAppsRequest) -> RusotoFuture<ListAppsResponse, ListAppsError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.ListApps",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListAppsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListAppsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a launch configuration for an application.</p>
    fn put_app_launch_configuration(
        &self,
        input: PutAppLaunchConfigurationRequest,
    ) -> RusotoFuture<PutAppLaunchConfigurationResponse, PutAppLaunchConfigurationError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.PutAppLaunchConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutAppLaunchConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutAppLaunchConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates or updates a replication configuration for an application.</p>
    fn put_app_replication_configuration(
        &self,
        input: PutAppReplicationConfigurationRequest,
    ) -> RusotoFuture<PutAppReplicationConfigurationResponse, PutAppReplicationConfigurationError>
    {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.PutAppReplicationConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutAppReplicationConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutAppReplicationConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Starts replicating an application.</p>
    fn start_app_replication(
        &self,
        input: StartAppReplicationRequest,
    ) -> RusotoFuture<StartAppReplicationResponse, StartAppReplicationError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.StartAppReplication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartAppReplicationResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartAppReplicationError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Starts an on-demand replication run for the specified replication job. This replication run starts immediately. This replication run is in addition to the ones already scheduled.</p> <p>There is a limit on the number of on-demand replications runs you can request in a 24-hour period.</p>
    fn start_on_demand_replication_run(
        &self,
        input: StartOnDemandReplicationRunRequest,
    ) -> RusotoFuture<StartOnDemandReplicationRunResponse, StartOnDemandReplicationRunError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.StartOnDemandReplicationRun",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartOnDemandReplicationRunResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartOnDemandReplicationRunError::from_response(response))
                }))
            }
        })
    }

    /// <p>Stops replicating an application.</p>
    fn stop_app_replication(
        &self,
        input: StopAppReplicationRequest,
    ) -> RusotoFuture<StopAppReplicationResponse, StopAppReplicationError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.StopAppReplication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StopAppReplicationResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StopAppReplicationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Terminates the stack for an application.</p>
    fn terminate_app(
        &self,
        input: TerminateAppRequest,
    ) -> RusotoFuture<TerminateAppResponse, TerminateAppError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.TerminateApp",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<TerminateAppResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TerminateAppError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates an application.</p>
    fn update_app(
        &self,
        input: UpdateAppRequest,
    ) -> RusotoFuture<UpdateAppResponse, UpdateAppError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.UpdateApp",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateAppResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateAppError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the specified settings for the specified replication job.</p>
    fn update_replication_job(
        &self,
        input: UpdateReplicationJobRequest,
    ) -> RusotoFuture<UpdateReplicationJobResponse, UpdateReplicationJobError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.UpdateReplicationJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateReplicationJobResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateReplicationJobError::from_response(response))
                    }),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
