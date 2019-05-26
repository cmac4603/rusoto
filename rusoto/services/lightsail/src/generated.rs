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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AllocateStaticIpRequest {
    /// <p>The name of the static IP address.</p>
    #[serde(rename = "staticIpName")]
    pub static_ip_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AllocateStaticIpResult {
    /// <p>An array of key-value pairs containing information about the static IP address you allocated.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AttachDiskRequest {
    /// <p>The unique Lightsail disk name (e.g., <code>my-disk</code>).</p>
    #[serde(rename = "diskName")]
    pub disk_name: String,
    /// <p>The disk path to expose to the instance (e.g., <code>/dev/xvdf</code>).</p>
    #[serde(rename = "diskPath")]
    pub disk_path: String,
    /// <p>The name of the Lightsail instance where you want to utilize the storage disk.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AttachDiskResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AttachInstancesToLoadBalancerRequest {
    /// <p>An array of strings representing the instance name(s) you want to attach to your load balancer.</p> <p>An instance must be <code>running</code> before you can attach it to your load balancer.</p> <p>There are no additional limits on the number of instances you can attach to your load balancer, aside from the limit of Lightsail instances you can create in your account (20).</p>
    #[serde(rename = "instanceNames")]
    pub instance_names: Vec<String>,
    /// <p>The name of the load balancer.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AttachInstancesToLoadBalancerResult {
    /// <p>An object representing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AttachLoadBalancerTlsCertificateRequest {
    /// <p>The name of your SSL/TLS certificate.</p>
    #[serde(rename = "certificateName")]
    pub certificate_name: String,
    /// <p>The name of the load balancer to which you want to associate the SSL/TLS certificate.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AttachLoadBalancerTlsCertificateResult {
    /// <p>An object representing the API operations.</p> <p>These SSL/TLS certificates are only usable by Lightsail load balancers. You can't get the certificate and use it for another purpose.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AttachStaticIpRequest {
    /// <p>The instance name to which you want to attach the static IP address.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>The name of the static IP.</p>
    #[serde(rename = "staticIpName")]
    pub static_ip_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AttachStaticIpResult {
    /// <p>An array of key-value pairs containing information about your API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes an Availability Zone.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AvailabilityZone {
    /// <p>The state of the Availability Zone.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The name of the Availability Zone. The format is <code>us-east-2a</code> (case-sensitive).</p>
    #[serde(rename = "zoneName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_name: Option<String>,
}

/// <p>Describes a blueprint (a virtual private server image).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Blueprint {
    /// <p>The ID for the virtual private server image (e.g., <code>app_wordpress_4_4</code> or <code>app_lamp_7_0</code>).</p>
    #[serde(rename = "blueprintId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_id: Option<String>,
    /// <p>The description of the blueprint.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The group name of the blueprint (e.g., <code>amazon-linux</code>).</p>
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// <p>A Boolean value indicating whether the blueprint is active. Inactive blueprints are listed to support customers with existing instances but are not necessarily available for launch of new instances. Blueprints are marked inactive when they become outdated due to operating system updates or new application releases.</p>
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// <p>The end-user license agreement URL for the image or blueprint.</p>
    #[serde(rename = "licenseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_url: Option<String>,
    /// <p>The minimum bundle power required to run this blueprint. For example, you need a bundle with a power value of 500 or more to create an instance that uses a blueprint with a minimum power value of 500. <code>0</code> indicates that the blueprint runs on all instance sizes. </p>
    #[serde(rename = "minPower")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_power: Option<i64>,
    /// <p>The friendly name of the blueprint (e.g., <code>Amazon Linux</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The operating system platform (either Linux/Unix-based or Windows Server-based) of the blueprint.</p>
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    /// <p>The product URL to learn more about the image or blueprint.</p>
    #[serde(rename = "productUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_url: Option<String>,
    /// <p>The type of the blueprint (e.g., <code>os</code> or <code>app</code>).</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The version number of the operating system, application, or stack (e.g., <code>2016.03.0</code>).</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The version code.</p>
    #[serde(rename = "versionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_code: Option<String>,
}

/// <p>Describes a bundle, which is a set of specs describing your virtual private server (or <i>instance</i>).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Bundle {
    /// <p>The bundle ID (e.g., <code>micro_1_0</code>).</p>
    #[serde(rename = "bundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The number of vCPUs included in the bundle (e.g., <code>2</code>).</p>
    #[serde(rename = "cpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i64>,
    /// <p>The size of the SSD (e.g., <code>30</code>).</p>
    #[serde(rename = "diskSizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size_in_gb: Option<i64>,
    /// <p>The Amazon EC2 instance type (e.g., <code>t2.micro</code>).</p>
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>A Boolean value indicating whether the bundle is active.</p>
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// <p>A friendly name for the bundle (e.g., <code>Micro</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A numeric value that represents the power of the bundle (e.g., <code>500</code>). You can use the bundle's power value in conjunction with a blueprint's minimum power value to determine whether the blueprint will run on the bundle. For example, you need a bundle with a power value of 500 or more to create an instance that uses a blueprint with a minimum power value of 500.</p>
    #[serde(rename = "power")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power: Option<i64>,
    /// <p>The price in US dollars (e.g., <code>5.0</code>).</p>
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f32>,
    /// <p>The amount of RAM in GB (e.g., <code>2.0</code>).</p>
    #[serde(rename = "ramSizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_size_in_gb: Option<f32>,
    /// <p>The operating system platform (Linux/Unix-based or Windows Server-based) that the bundle supports. You can only launch a <code>WINDOWS</code> bundle on a blueprint that supports the <code>WINDOWS</code> platform. <code>LINUX_UNIX</code> blueprints require a <code>LINUX_UNIX</code> bundle.</p>
    #[serde(rename = "supportedPlatforms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_platforms: Option<Vec<String>>,
    /// <p>The data transfer rate per month in GB (e.g., <code>2000</code>).</p>
    #[serde(rename = "transferPerMonthInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_per_month_in_gb: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CloseInstancePublicPortsRequest {
    /// <p>The name of the instance on which you're attempting to close the public ports.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>Information about the public port you are trying to close.</p>
    #[serde(rename = "portInfo")]
    pub port_info: PortInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CloseInstancePublicPortsResult {
    /// <p>An array of key-value pairs that contains information about the operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

/// <p>Describes a CloudFormation stack record created as a result of the <code>create cloud formation stack</code> operation.</p> <p>A CloudFormation stack record provides information about the AWS CloudFormation stack used to create a new Amazon Elastic Compute Cloud instance from an exported Lightsail instance snapshot.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CloudFormationStackRecord {
    /// <p>The Amazon Resource Name (ARN) of the CloudFormation stack record.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date when the CloudFormation stack record was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>A list of objects describing the destination service, which is AWS CloudFormation, and the Amazon Resource Name (ARN) of the AWS CloudFormation stack.</p>
    #[serde(rename = "destinationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_info: Option<DestinationInfo>,
    /// <p>A list of objects describing the Availability Zone and AWS Region of the CloudFormation stack record.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The name of the CloudFormation stack record. It starts with <code>CloudFormationStackRecord</code> followed by a GUID.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Lightsail resource type (e.g., <code>CloudFormationStackRecord</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>A list of objects describing the source of the CloudFormation stack record.</p>
    #[serde(rename = "sourceInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_info: Option<Vec<CloudFormationStackRecordSourceInfo>>,
    /// <p>The current state of the CloudFormation stack record.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Describes the source of a CloudFormation stack record (i.e., the export snapshot record).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CloudFormationStackRecordSourceInfo {
    /// <p>The Amazon Resource Name (ARN) of the export snapshot record.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The name of the record.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Lightsail resource type (e.g., <code>ExportSnapshotRecord</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CopySnapshotRequest {
    /// <p>The AWS Region where the source snapshot is located.</p>
    #[serde(rename = "sourceRegion")]
    pub source_region: String,
    /// <p>The name of the source instance or disk snapshot to be copied.</p>
    #[serde(rename = "sourceSnapshotName")]
    pub source_snapshot_name: String,
    /// <p>The name of the new instance or disk snapshot to be created as a copy.</p>
    #[serde(rename = "targetSnapshotName")]
    pub target_snapshot_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CopySnapshotResult {
    /// <p>A list of objects describing the API operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCloudFormationStackRequest {
    /// <p>An array of parameters that will be used to create the new Amazon EC2 instance. You can only pass one instance entry at a time in this array. You will get an invalid parameter error if you pass more than one instance entry in this array.</p>
    #[serde(rename = "instances")]
    pub instances: Vec<InstanceEntry>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateCloudFormationStackResult {
    /// <p>A list of objects describing the API operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDiskFromSnapshotRequest {
    /// <p>The Availability Zone where you want to create the disk (e.g., <code>us-east-2a</code>). Choose the same Availability Zone as the Lightsail instance where you want to create the disk.</p> <p>Use the GetRegions operation to list the Availability Zones where Lightsail is currently available.</p>
    #[serde(rename = "availabilityZone")]
    pub availability_zone: String,
    /// <p>The unique Lightsail disk name (e.g., <code>my-disk</code>).</p>
    #[serde(rename = "diskName")]
    pub disk_name: String,
    /// <p>The name of the disk snapshot (e.g., <code>my-snapshot</code>) from which to create the new storage disk.</p>
    #[serde(rename = "diskSnapshotName")]
    pub disk_snapshot_name: String,
    /// <p>The size of the disk in GB (e.g., <code>32</code>).</p>
    #[serde(rename = "sizeInGb")]
    pub size_in_gb: i64,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateDiskFromSnapshotResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDiskRequest {
    /// <p>The Availability Zone where you want to create the disk (e.g., <code>us-east-2a</code>). Choose the same Availability Zone as the Lightsail instance where you want to create the disk.</p> <p>Use the GetRegions operation to list the Availability Zones where Lightsail is currently available.</p>
    #[serde(rename = "availabilityZone")]
    pub availability_zone: String,
    /// <p>The unique Lightsail disk name (e.g., <code>my-disk</code>).</p>
    #[serde(rename = "diskName")]
    pub disk_name: String,
    /// <p>The size of the disk in GB (e.g., <code>32</code>).</p>
    #[serde(rename = "sizeInGb")]
    pub size_in_gb: i64,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateDiskResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDiskSnapshotRequest {
    /// <p><p>The unique name of the source disk (e.g., <code>Disk-Virginia-1</code>).</p> <note> <p>This parameter cannot be defined together with the <code>instance name</code> parameter. The <code>disk name</code> and <code>instance name</code> parameters are mutually exclusive.</p> </note></p>
    #[serde(rename = "diskName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_name: Option<String>,
    /// <p>The name of the destination disk snapshot (e.g., <code>my-disk-snapshot</code>) based on the source disk.</p>
    #[serde(rename = "diskSnapshotName")]
    pub disk_snapshot_name: String,
    /// <p><p>The unique name of the source instance (e.g., <code>Amazon_Linux-512MB-Virginia-1</code>). When this is defined, a snapshot of the instance&#39;s system volume is created.</p> <note> <p>This parameter cannot be defined together with the <code>disk name</code> parameter. The <code>instance name</code> and <code>disk name</code> parameters are mutually exclusive.</p> </note></p>
    #[serde(rename = "instanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateDiskSnapshotResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDomainEntryRequest {
    /// <p>An array of key-value pairs containing information about the domain entry request.</p>
    #[serde(rename = "domainEntry")]
    pub domain_entry: DomainEntry,
    /// <p>The domain name (e.g., <code>example.com</code>) for which you want to create the domain entry.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateDomainEntryResult {
    /// <p>An array of key-value pairs containing information about the operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDomainRequest {
    /// <p><p>The domain name to manage (e.g., <code>example.com</code>).</p> <note> <p>You cannot register a new domain name using Lightsail. You must register a domain name using Amazon Route 53 or another domain name registrar. If you have already registered your domain, you can enter its name in this parameter to manage the DNS records for that domain.</p> </note></p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateDomainResult {
    /// <p>An array of key-value pairs containing information about the domain resource you created.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateInstanceSnapshotRequest {
    /// <p>The Lightsail instance on which to base your snapshot.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>The name for your new snapshot.</p>
    #[serde(rename = "instanceSnapshotName")]
    pub instance_snapshot_name: String,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateInstanceSnapshotResult {
    /// <p>An array of key-value pairs containing information about the results of your create instances snapshot request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateInstancesFromSnapshotRequest {
    /// <p>An object containing information about one or more disk mappings.</p>
    #[serde(rename = "attachedDiskMapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_disk_mapping: Option<::std::collections::HashMap<String, Vec<DiskMap>>>,
    /// <p>The Availability Zone where you want to create your instances. Use the following formatting: <code>us-east-2a</code> (case sensitive). You can get a list of Availability Zones by using the <a href="http://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetRegions.html">get regions</a> operation. Be sure to add the <code>include Availability Zones</code> parameter to your request.</p>
    #[serde(rename = "availabilityZone")]
    pub availability_zone: String,
    /// <p>The bundle of specification information for your virtual private server (or <i>instance</i>), including the pricing plan (e.g., <code>micro_1_0</code>).</p>
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    /// <p>The names for your new instances.</p>
    #[serde(rename = "instanceNames")]
    pub instance_names: Vec<String>,
    /// <p>The name of the instance snapshot on which you are basing your new instances. Use the get instance snapshots operation to return information about your existing snapshots.</p>
    #[serde(rename = "instanceSnapshotName")]
    pub instance_snapshot_name: String,
    /// <p>The name for your key pair.</p>
    #[serde(rename = "keyPairName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair_name: Option<String>,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p><p>You can create a launch script that configures a server with additional user data. For example, <code>apt-get -y update</code>.</p> <note> <p>Depending on the machine image you choose, the command to get software on your instance varies. Amazon Linux and CentOS use <code>yum</code>, Debian and Ubuntu use <code>apt-get</code>, and FreeBSD uses <code>pkg</code>. For a complete list, see the <a href="https://lightsail.aws.amazon.com/ls/docs/getting-started/article/compare-options-choose-lightsail-instance-image">Dev Guide</a>.</p> </note></p>
    #[serde(rename = "userData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateInstancesFromSnapshotResult {
    /// <p>An array of key-value pairs containing information about the results of your create instances from snapshot request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateInstancesRequest {
    /// <p>The Availability Zone in which to create your instance. Use the following format: <code>us-east-2a</code> (case sensitive). You can get a list of Availability Zones by using the <a href="http://docs.aws.amazon.com/lightsail/2016-11-28/api-reference/API_GetRegions.html">get regions</a> operation. Be sure to add the <code>include Availability Zones</code> parameter to your request.</p>
    #[serde(rename = "availabilityZone")]
    pub availability_zone: String,
    /// <p>The ID for a virtual private server image (e.g., <code>app_wordpress_4_4</code> or <code>app_lamp_7_0</code>). Use the get blueprints operation to return a list of available images (or <i>blueprints</i>).</p>
    #[serde(rename = "blueprintId")]
    pub blueprint_id: String,
    /// <p>The bundle of specification information for your virtual private server (or <i>instance</i>), including the pricing plan (e.g., <code>micro_1_0</code>).</p>
    #[serde(rename = "bundleId")]
    pub bundle_id: String,
    /// <p>The names to use for your new Lightsail instances. Separate multiple values using quotation marks and commas, for example: <code>["MyFirstInstance","MySecondInstance"]</code> </p>
    #[serde(rename = "instanceNames")]
    pub instance_names: Vec<String>,
    /// <p>The name of your key pair.</p>
    #[serde(rename = "keyPairName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair_name: Option<String>,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p><p>A launch script you can create that configures a server with additional user data. For example, you might want to run <code>apt-get -y update</code>.</p> <note> <p>Depending on the machine image you choose, the command to get software on your instance varies. Amazon Linux and CentOS use <code>yum</code>, Debian and Ubuntu use <code>apt-get</code>, and FreeBSD uses <code>pkg</code>. For a complete list, see the <a href="https://lightsail.aws.amazon.com/ls/docs/getting-started/article/compare-options-choose-lightsail-instance-image">Dev Guide</a>.</p> </note></p>
    #[serde(rename = "userData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateInstancesResult {
    /// <p>An array of key-value pairs containing information about the results of your create instances request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateKeyPairRequest {
    /// <p>The name for your new key pair.</p>
    #[serde(rename = "keyPairName")]
    pub key_pair_name: String,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateKeyPairResult {
    /// <p>An array of key-value pairs containing information about the new key pair you just created.</p>
    #[serde(rename = "keyPair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<KeyPair>,
    /// <p>An array of key-value pairs containing information about the results of your create key pair request.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
    /// <p>A base64-encoded RSA private key.</p>
    #[serde(rename = "privateKeyBase64")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_base_64: Option<String>,
    /// <p>A base64-encoded public key of the <code>ssh-rsa</code> type.</p>
    #[serde(rename = "publicKeyBase64")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_base_64: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateLoadBalancerRequest {
    /// <p>The optional alternative domains and subdomains to use with your SSL/TLS certificate (e.g., <code>www.example.com</code>, <code>example.com</code>, <code>m.example.com</code>, <code>blog.example.com</code>).</p>
    #[serde(rename = "certificateAlternativeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_alternative_names: Option<Vec<String>>,
    /// <p>The domain name with which your certificate is associated (e.g., <code>example.com</code>).</p> <p>If you specify <code>certificateDomainName</code>, then <code>certificateName</code> is required (and vice-versa).</p>
    #[serde(rename = "certificateDomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_domain_name: Option<String>,
    /// <p>The name of the SSL/TLS certificate.</p> <p>If you specify <code>certificateName</code>, then <code>certificateDomainName</code> is required (and vice-versa).</p>
    #[serde(rename = "certificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_name: Option<String>,
    /// <p>The path you provided to perform the load balancer health check. If you didn't specify a health check path, Lightsail uses the root path of your website (e.g., <code>"/"</code>).</p> <p>You may want to specify a custom health check path other than the root of your application if your home page loads slowly or has a lot of media or scripting on it.</p>
    #[serde(rename = "healthCheckPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_path: Option<String>,
    /// <p>The instance port where you're creating your load balancer.</p>
    #[serde(rename = "instancePort")]
    pub instance_port: i64,
    /// <p>The name of your load balancer.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateLoadBalancerResult {
    /// <p>An object containing information about the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateLoadBalancerTlsCertificateRequest {
    /// <p>An array of strings listing alternative domains and subdomains for your SSL/TLS certificate. Lightsail will de-dupe the names for you. You can have a maximum of 9 alternative names (in addition to the 1 primary domain). We do not support wildcards (e.g., <code>*.example.com</code>).</p>
    #[serde(rename = "certificateAlternativeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_alternative_names: Option<Vec<String>>,
    /// <p>The domain name (e.g., <code>example.com</code>) for your SSL/TLS certificate.</p>
    #[serde(rename = "certificateDomainName")]
    pub certificate_domain_name: String,
    /// <p>The SSL/TLS certificate name.</p> <p>You can have up to 10 certificates in your account at one time. Each Lightsail load balancer can have up to 2 certificates associated with it at one time. There is also an overall limit to the number of certificates that can be issue in a 365-day period. For more information, see <a href="http://docs.aws.amazon.com/acm/latest/userguide/acm-limits.html">Limits</a>.</p>
    #[serde(rename = "certificateName")]
    pub certificate_name: String,
    /// <p>The load balancer name where you want to create the SSL/TLS certificate.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateLoadBalancerTlsCertificateResult {
    /// <p>An object containing information about the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateRelationalDatabaseFromSnapshotRequest {
    /// <p>The Availability Zone in which to create your new database. Use the <code>us-east-2a</code> case-sensitive format.</p> <p>You can get a list of Availability Zones by using the <code>get regions</code> operation. Be sure to add the <code>include relational database Availability Zones</code> parameter to your request.</p>
    #[serde(rename = "availabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>Specifies the accessibility options for your new database. A value of <code>true</code> specifies a database that is available to resources outside of your Lightsail account. A value of <code>false</code> specifies a database that is available only to your Lightsail resources in the same region as your database.</p>
    #[serde(rename = "publiclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    /// <p>The bundle ID for your new database. A bundle describes the performance specifications for your database.</p> <p>You can get a list of database bundle IDs by using the <code>get relational database bundles</code> operation.</p> <p>When creating a new database from a snapshot, you cannot choose a bundle that is smaller than the bundle of the source database.</p>
    #[serde(rename = "relationalDatabaseBundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_bundle_id: Option<String>,
    /// <p><p>The name to use for your new database.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 2 to 255 alphanumeric characters, or hyphens.</p> </li> <li> <p>The first and last character must be a letter or number.</p> </li> </ul></p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
    /// <p>The name of the database snapshot from which to create your new database.</p>
    #[serde(rename = "relationalDatabaseSnapshotName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_snapshot_name: Option<String>,
    /// <p><p>The date and time to restore your database from.</p> <p>Constraints:</p> <ul> <li> <p>Must be before the latest restorable time for the database.</p> </li> <li> <p>Cannot be specified if the <code>use latest restorable time</code> parameter is <code>true</code>.</p> </li> <li> <p>Specified in Universal Coordinated Time (UTC).</p> </li> <li> <p>Specified in the Unix time format.</p> <p>For example, if you wish to use a restore time of October 1, 2018, at 8 PM UTC, then you input <code>1538424000</code> as the restore time.</p> </li> </ul></p>
    #[serde(rename = "restoreTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_time: Option<f64>,
    /// <p>The name of the source database.</p>
    #[serde(rename = "sourceRelationalDatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_relational_database_name: Option<String>,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>Specifies whether your database is restored from the latest backup time. A value of <code>true</code> restores from the latest backup time. </p> <p>Default: <code>false</code> </p> <p>Constraints: Cannot be specified if the <code>restore time</code> parameter is provided.</p>
    #[serde(rename = "useLatestRestorableTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_latest_restorable_time: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateRelationalDatabaseFromSnapshotResult {
    /// <p>An object describing the result of your create relational database from snapshot request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateRelationalDatabaseRequest {
    /// <p>The Availability Zone in which to create your new database. Use the <code>us-east-2a</code> case-sensitive format.</p> <p>You can get a list of Availability Zones by using the <code>get regions</code> operation. Be sure to add the <code>include relational database Availability Zones</code> parameter to your request.</p>
    #[serde(rename = "availabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p><p>The name of the master database created when the Lightsail database resource is created.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 64 alphanumeric characters.</p> </li> <li> <p>Cannot be a word reserved by the specified database engine</p> </li> </ul></p>
    #[serde(rename = "masterDatabaseName")]
    pub master_database_name: String,
    /// <p>The password for the master user of your new database. The password can include any printable ASCII character except "/", """, or "@".</p> <p>Constraints: Must contain 8 to 41 characters.</p>
    #[serde(rename = "masterUserPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    /// <p><p>The master user name for your new database.</p> <p>Constraints:</p> <ul> <li> <p>Master user name is required.</p> </li> <li> <p>Must contain from 1 to 16 alphanumeric characters.</p> </li> <li> <p>The first character must be a letter.</p> </li> <li> <p>Cannot be a reserved word for the database engine you choose.</p> <p>For more information about reserved words in MySQL 5.6 or 5.7, see the Keywords and Reserved Words articles for <a href="https://dev.mysql.com/doc/refman/5.6/en/keywords.html">MySQL 5.6</a> or <a href="https://dev.mysql.com/doc/refman/5.7/en/keywords.html">MySQL 5.7</a> respectively.</p> </li> </ul></p>
    #[serde(rename = "masterUsername")]
    pub master_username: String,
    /// <p><p>The daily time range during which automated backups are created for your new database if automated backups are enabled.</p> <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region. For more information about the preferred backup window time blocks for each region, see the <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/USER_WorkingWithAutomatedBackups.html#USER_WorkingWithAutomatedBackups.BackupWindow">Working With Backups</a> guide in the Amazon Relational Database Service (Amazon RDS) documentation.</p> <p>Constraints:</p> <ul> <li> <p>Must be in the <code>hh24:mi-hh24:mi</code> format.</p> <p>Example: <code>16:00-16:30</code> </p> </li> <li> <p>Specified in Universal Coordinated Time (UTC).</p> </li> <li> <p>Must not conflict with the preferred maintenance window.</p> </li> <li> <p>Must be at least 30 minutes.</p> </li> </ul></p>
    #[serde(rename = "preferredBackupWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    /// <p><p>The weekly time range during which system maintenance can occur on your new database.</p> <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region, occurring on a random day of the week.</p> <p>Constraints:</p> <ul> <li> <p>Must be in the <code>ddd:hh24:mi-ddd:hh24:mi</code> format.</p> </li> <li> <p>Valid days: Mon, Tue, Wed, Thu, Fri, Sat, Sun.</p> </li> <li> <p>Must be at least 30 minutes.</p> </li> <li> <p>Specified in Universal Coordinated Time (UTC).</p> </li> <li> <p>Example: <code>Tue:17:00-Tue:17:30</code> </p> </li> </ul></p>
    #[serde(rename = "preferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p>Specifies the accessibility options for your new database. A value of <code>true</code> specifies a database that is available to resources outside of your Lightsail account. A value of <code>false</code> specifies a database that is available only to your Lightsail resources in the same region as your database.</p>
    #[serde(rename = "publiclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    /// <p>The blueprint ID for your new database. A blueprint describes the major engine version of a database.</p> <p>You can get a list of database blueprints IDs by using the <code>get relational database blueprints</code> operation.</p>
    #[serde(rename = "relationalDatabaseBlueprintId")]
    pub relational_database_blueprint_id: String,
    /// <p>The bundle ID for your new database. A bundle describes the performance specifications for your database.</p> <p>You can get a list of database bundle IDs by using the <code>get relational database bundles</code> operation.</p>
    #[serde(rename = "relationalDatabaseBundleId")]
    pub relational_database_bundle_id: String,
    /// <p><p>The name to use for your new database.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 2 to 255 alphanumeric characters, or hyphens.</p> </li> <li> <p>The first and last character must be a letter or number.</p> </li> </ul></p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateRelationalDatabaseResult {
    /// <p>An object describing the result of your create relational database request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateRelationalDatabaseSnapshotRequest {
    /// <p>The name of the database on which to base your new snapshot.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
    /// <p><p>The name for your new database snapshot.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 2 to 255 alphanumeric characters, or hyphens.</p> </li> <li> <p>The first and last character must be a letter or number.</p> </li> </ul></p>
    #[serde(rename = "relationalDatabaseSnapshotName")]
    pub relational_database_snapshot_name: String,
    /// <p>The tag keys and optional values to add to the resource during create.</p> <p>To tag a resource after it has been created, see the <code>tag resource</code> operation.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateRelationalDatabaseSnapshotResult {
    /// <p>An object describing the result of your create relational database snapshot request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDiskRequest {
    /// <p>The unique name of the disk you want to delete (e.g., <code>my-disk</code>).</p>
    #[serde(rename = "diskName")]
    pub disk_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteDiskResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDiskSnapshotRequest {
    /// <p>The name of the disk snapshot you want to delete (e.g., <code>my-disk-snapshot</code>).</p>
    #[serde(rename = "diskSnapshotName")]
    pub disk_snapshot_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteDiskSnapshotResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDomainEntryRequest {
    /// <p>An array of key-value pairs containing information about your domain entries.</p>
    #[serde(rename = "domainEntry")]
    pub domain_entry: DomainEntry,
    /// <p>The name of the domain entry to delete.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteDomainEntryResult {
    /// <p>An array of key-value pairs containing information about the results of your delete domain entry request.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDomainRequest {
    /// <p>The specific domain name to delete.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteDomainResult {
    /// <p>An array of key-value pairs containing information about the results of your delete domain request.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteInstanceRequest {
    /// <p>The name of the instance to delete.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteInstanceResult {
    /// <p>An array of key-value pairs containing information about the results of your delete instance request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteInstanceSnapshotRequest {
    /// <p>The name of the snapshot to delete.</p>
    #[serde(rename = "instanceSnapshotName")]
    pub instance_snapshot_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteInstanceSnapshotResult {
    /// <p>An array of key-value pairs containing information about the results of your delete instance snapshot request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteKeyPairRequest {
    /// <p>The name of the key pair to delete.</p>
    #[serde(rename = "keyPairName")]
    pub key_pair_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteKeyPairResult {
    /// <p>An array of key-value pairs containing information about the results of your delete key pair request.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteKnownHostKeysRequest {
    /// <p>The name of the instance for which you want to reset the host key or certificate.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteKnownHostKeysResult {
    /// <p>A list of objects describing the API operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteLoadBalancerRequest {
    /// <p>The name of the load balancer you want to delete.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteLoadBalancerResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteLoadBalancerTlsCertificateRequest {
    /// <p>The SSL/TLS certificate name.</p>
    #[serde(rename = "certificateName")]
    pub certificate_name: String,
    /// <p>When <code>true</code>, forces the deletion of an SSL/TLS certificate.</p> <p>There can be two certificates associated with a Lightsail load balancer: the primary and the backup. The <code>force</code> parameter is required when the primary SSL/TLS certificate is in use by an instance attached to the load balancer.</p>
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// <p>The load balancer name.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteLoadBalancerTlsCertificateResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRelationalDatabaseRequest {
    /// <p><p>The name of the database snapshot created if <code>skip final snapshot</code> is <code>false</code>, which is the default value for that parameter.</p> <note> <p>Specifying this parameter and also specifying the <code>skip final snapshot</code> parameter to <code>true</code> results in an error.</p> </note> <p>Constraints:</p> <ul> <li> <p>Must contain from 2 to 255 alphanumeric characters, or hyphens.</p> </li> <li> <p>The first and last character must be a letter or number.</p> </li> </ul></p>
    #[serde(rename = "finalRelationalDatabaseSnapshotName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_relational_database_snapshot_name: Option<String>,
    /// <p>The name of the database that you are deleting.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
    /// <p>Determines whether a final database snapshot is created before your database is deleted. If <code>true</code> is specified, no database snapshot is created. If <code>false</code> is specified, a database snapshot is created before your database is deleted.</p> <p>You must specify the <code>final relational database snapshot name</code> parameter if the <code>skip final snapshot</code> parameter is <code>false</code>.</p> <p>Default: <code>false</code> </p>
    #[serde(rename = "skipFinalSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_final_snapshot: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteRelationalDatabaseResult {
    /// <p>An object describing the result of your delete relational database request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRelationalDatabaseSnapshotRequest {
    /// <p>The name of the database snapshot that you are deleting.</p>
    #[serde(rename = "relationalDatabaseSnapshotName")]
    pub relational_database_snapshot_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteRelationalDatabaseSnapshotResult {
    /// <p>An object describing the result of your delete relational database snapshot request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes the destination of a record.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DestinationInfo {
    /// <p>The ID of the resource created at the destination.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The destination service of the record.</p>
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetachDiskRequest {
    /// <p>The unique name of the disk you want to detach from your instance (e.g., <code>my-disk</code>).</p>
    #[serde(rename = "diskName")]
    pub disk_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DetachDiskResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetachInstancesFromLoadBalancerRequest {
    /// <p>An array of strings containing the names of the instances you want to detach from the load balancer.</p>
    #[serde(rename = "instanceNames")]
    pub instance_names: Vec<String>,
    /// <p>The name of the Lightsail load balancer.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DetachInstancesFromLoadBalancerResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetachStaticIpRequest {
    /// <p>The name of the static IP to detach from the instance.</p>
    #[serde(rename = "staticIpName")]
    pub static_ip_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DetachStaticIpResult {
    /// <p>An array of key-value pairs containing information about the results of your detach static IP request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes a system disk or an block storage disk.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Disk {
    /// <p>The Amazon Resource Name (ARN) of the disk.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The resources to which the disk is attached.</p>
    #[serde(rename = "attachedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_to: Option<String>,
    /// <p>The date when the disk was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The input/output operations per second (IOPS) of the disk.</p>
    #[serde(rename = "iops")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,
    /// <p>A Boolean value indicating whether the disk is attached.</p>
    #[serde(rename = "isAttached")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_attached: Option<bool>,
    /// <p>A Boolean value indicating whether this disk is a system disk (has an operating system loaded on it).</p>
    #[serde(rename = "isSystemDisk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_system_disk: Option<bool>,
    /// <p>The AWS Region and Availability Zone where the disk is located.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The unique name of the disk.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The disk path.</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The Lightsail resource type (e.g., <code>Disk</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The size of the disk in GB.</p>
    #[serde(rename = "sizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_gb: Option<i64>,
    /// <p>Describes the status of the disk.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The support code. Include this code in your email to support when you have questions about an instance or another resource in Lightsail. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
    /// <p>The tag keys and optional values for the resource. For more information about tags in Lightsail, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Describes a disk.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DiskInfo {
    /// <p>A Boolean value indicating whether this disk is a system disk (has an operating system loaded on it).</p>
    #[serde(rename = "isSystemDisk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_system_disk: Option<bool>,
    /// <p>The disk name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The disk path.</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The size of the disk in GB (e.g., <code>32</code>).</p>
    #[serde(rename = "sizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_gb: Option<i64>,
}

/// <p>Describes a block storage disk mapping.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DiskMap {
    /// <p>The new disk name (e.g., <code>my-new-disk</code>).</p>
    #[serde(rename = "newDiskName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_disk_name: Option<String>,
    /// <p>The original disk path exposed to the instance (for example, <code>/dev/sdh</code>).</p>
    #[serde(rename = "originalDiskPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_disk_path: Option<String>,
}

/// <p>Describes a block storage disk snapshot.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DiskSnapshot {
    /// <p>The Amazon Resource Name (ARN) of the disk snapshot.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date when the disk snapshot was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the source disk from which the disk snapshot was created.</p>
    #[serde(rename = "fromDiskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_disk_arn: Option<String>,
    /// <p>The unique name of the source disk from which the disk snapshot was created.</p>
    #[serde(rename = "fromDiskName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_disk_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the source instance from which the disk (system volume) snapshot was created.</p>
    #[serde(rename = "fromInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_instance_arn: Option<String>,
    /// <p>The unique name of the source instance from which the disk (system volume) snapshot was created.</p>
    #[serde(rename = "fromInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_instance_name: Option<String>,
    /// <p>The AWS Region and Availability Zone where the disk snapshot was created.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The name of the disk snapshot (e.g., <code>my-disk-snapshot</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The progress of the disk snapshot operation.</p>
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    /// <p>The Lightsail resource type (e.g., <code>DiskSnapshot</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The size of the disk in GB.</p>
    #[serde(rename = "sizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_gb: Option<i64>,
    /// <p>The status of the disk snapshot operation.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The support code. Include this code in your email to support when you have questions about an instance or another resource in Lightsail. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
    /// <p>The tag keys and optional values for the resource. For more information about tags in Lightsail, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Describes a disk snapshot.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DiskSnapshotInfo {
    /// <p>The size of the disk in GB (e.g., <code>32</code>).</p>
    #[serde(rename = "sizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_gb: Option<i64>,
}

/// <p>Describes a domain where you are storing recordsets in Lightsail.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Domain {
    /// <p>The Amazon Resource Name (ARN) of the domain recordset (e.g., <code>arn:aws:lightsail:global:123456789101:Domain/824cede0-abc7-4f84-8dbc-12345EXAMPLE</code>).</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date when the domain recordset was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>An array of key-value pairs containing information about the domain entries.</p>
    #[serde(rename = "domainEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_entries: Option<Vec<DomainEntry>>,
    /// <p>The AWS Region and Availability Zones where the domain recordset was created.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The name of the domain.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The resource type. </p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The support code. Include this code in your email to support when you have questions about an instance or another resource in Lightsail. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
    /// <p>The tag keys and optional values for the resource. For more information about tags in Lightsail, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Describes a domain recordset entry.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomainEntry {
    /// <p>The ID of the domain recordset entry.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>When <code>true</code>, specifies whether the domain entry is an alias used by the Lightsail load balancer. You can include an alias (A type) record in your request, which points to a load balancer DNS name and routes traffic to your load balancer</p>
    #[serde(rename = "isAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_alias: Option<bool>,
    /// <p>The name of the domain.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The target AWS name server (e.g., <code>ns-111.awsdns-22.com.</code>).</p> <p>For Lightsail load balancers, the value looks like <code>ab1234c56789c6b86aba6fb203d443bc-123456789.us-east-2.elb.amazonaws.com</code>. Be sure to also set <code>isAlias</code> to <code>true</code> when setting up an A record for a load balancer.</p>
    #[serde(rename = "target")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// <p><p>The type of domain entry, such as address (A), canonical name (CNAME), mail exchanger (MX), name server (NS), start of authority (SOA), service locator (SRV), or text (TXT).</p> <p>The following domain entry types can be used:</p> <ul> <li> <p> <code>A</code> </p> </li> <li> <p> <code>CNAME</code> </p> </li> <li> <p> <code>MX</code> </p> </li> <li> <p> <code>NS</code> </p> </li> <li> <p> <code>SOA</code> </p> </li> <li> <p> <code>SRV</code> </p> </li> <li> <p> <code>TXT</code> </p> </li> </ul></p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DownloadDefaultKeyPairRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DownloadDefaultKeyPairResult {
    /// <p>A base64-encoded RSA private key.</p>
    #[serde(rename = "privateKeyBase64")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key_base_64: Option<String>,
    /// <p>A base64-encoded public key of the <code>ssh-rsa</code> type.</p>
    #[serde(rename = "publicKeyBase64")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_base_64: Option<String>,
}

/// <p>Describes an export snapshot record.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ExportSnapshotRecord {
    /// <p>The Amazon Resource Name (ARN) of the export snapshot record.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date when the export snapshot record was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>A list of objects describing the destination of the export snapshot record.</p>
    #[serde(rename = "destinationInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_info: Option<DestinationInfo>,
    /// <p>The AWS Region and Availability Zone where the export snapshot record is located.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The export snapshot record name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Lightsail resource type (e.g., <code>ExportSnapshotRecord</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>A list of objects describing the source of the export snapshot record.</p>
    #[serde(rename = "sourceInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_info: Option<ExportSnapshotRecordSourceInfo>,
    /// <p>The state of the export snapshot record.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Describes the source of an export snapshot record.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ExportSnapshotRecordSourceInfo {
    /// <p>The Amazon Resource Name (ARN) of the source instance or disk snapshot.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date when the source instance or disk snapshot was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>A list of objects describing a disk snapshot.</p>
    #[serde(rename = "diskSnapshotInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_snapshot_info: Option<DiskSnapshotInfo>,
    /// <p>The Amazon Resource Name (ARN) of the snapshot's source instance or disk.</p>
    #[serde(rename = "fromResourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_resource_arn: Option<String>,
    /// <p>The name of the snapshot's source instance or disk.</p>
    #[serde(rename = "fromResourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_resource_name: Option<String>,
    /// <p>A list of objects describing an instance snapshot.</p>
    #[serde(rename = "instanceSnapshotInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_snapshot_info: Option<InstanceSnapshotInfo>,
    /// <p>The name of the source instance or disk snapshot.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Lightsail resource type (e.g., <code>InstanceSnapshot</code> or <code>DiskSnapshot</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ExportSnapshotRequest {
    /// <p>The name of the instance or disk snapshot to be exported to Amazon EC2.</p>
    #[serde(rename = "sourceSnapshotName")]
    pub source_snapshot_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ExportSnapshotResult {
    /// <p>A list of objects describing the API operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetActiveNamesRequest {
    /// <p>A token used for paginating results from your get active names request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetActiveNamesResult {
    /// <p>The list of active names returned by the get active names request.</p>
    #[serde(rename = "activeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_names: Option<Vec<String>>,
    /// <p>A token used for advancing to the next page of results from your get active names request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBlueprintsRequest {
    /// <p>A Boolean value indicating whether to include inactive results in your request.</p>
    #[serde(rename = "includeInactive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_inactive: Option<bool>,
    /// <p>A token used for advancing to the next page of results from your get blueprints request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetBlueprintsResult {
    /// <p>An array of key-value pairs that contains information about the available blueprints.</p>
    #[serde(rename = "blueprints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprints: Option<Vec<Blueprint>>,
    /// <p>A token used for advancing to the next page of results from your get blueprints request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBundlesRequest {
    /// <p>A Boolean value that indicates whether to include inactive bundle results in your request.</p>
    #[serde(rename = "includeInactive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_inactive: Option<bool>,
    /// <p>A token used for advancing to the next page of results from your get bundles request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetBundlesResult {
    /// <p>An array of key-value pairs that contains information about the available bundles.</p>
    #[serde(rename = "bundles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundles: Option<Vec<Bundle>>,
    /// <p>A token used for advancing to the next page of results from your get active names request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCloudFormationStackRecordsRequest {
    /// <p>A token used for advancing to a specific page of results for your <code>get cloud formation stack records</code> request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCloudFormationStackRecordsResult {
    /// <p>A list of objects describing the CloudFormation stack records.</p>
    #[serde(rename = "cloudFormationStackRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_formation_stack_records: Option<Vec<CloudFormationStackRecord>>,
    /// <p>A token used for advancing to the next page of results of your get relational database bundles request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDiskRequest {
    /// <p>The name of the disk (e.g., <code>my-disk</code>).</p>
    #[serde(rename = "diskName")]
    pub disk_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDiskResult {
    /// <p>An object containing information about the disk.</p>
    #[serde(rename = "disk")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk: Option<Disk>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDiskSnapshotRequest {
    /// <p>The name of the disk snapshot (e.g., <code>my-disk-snapshot</code>).</p>
    #[serde(rename = "diskSnapshotName")]
    pub disk_snapshot_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDiskSnapshotResult {
    /// <p>An object containing information about the disk snapshot.</p>
    #[serde(rename = "diskSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_snapshot: Option<DiskSnapshot>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDiskSnapshotsRequest {
    /// <p>A token used for advancing to the next page of results from your GetDiskSnapshots request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDiskSnapshotsResult {
    /// <p>An array of objects containing information about all block storage disk snapshots.</p>
    #[serde(rename = "diskSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_snapshots: Option<Vec<DiskSnapshot>>,
    /// <p>A token used for advancing to the next page of results from your GetDiskSnapshots request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDisksRequest {
    /// <p>A token used for advancing to the next page of results from your GetDisks request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDisksResult {
    /// <p>An array of objects containing information about all block storage disks.</p>
    #[serde(rename = "disks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<Disk>>,
    /// <p>A token used for advancing to the next page of results from your GetDisks request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDomainRequest {
    /// <p>The domain name for which your want to return information about.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDomainResult {
    /// <p>An array of key-value pairs containing information about your get domain request.</p>
    #[serde(rename = "domain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<Domain>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDomainsRequest {
    /// <p>A token used for advancing to the next page of results from your get domains request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetDomainsResult {
    /// <p>An array of key-value pairs containing information about each of the domain entries in the user's account.</p>
    #[serde(rename = "domains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<Domain>>,
    /// <p>A token used for advancing to the next page of results from your get active names request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetExportSnapshotRecordsRequest {
    /// <p>A token used for advancing to a specific page of results for your <code>get export snapshot records</code> request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetExportSnapshotRecordsResult {
    /// <p>A list of objects describing the export snapshot records.</p>
    #[serde(rename = "exportSnapshotRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_snapshot_records: Option<Vec<ExportSnapshotRecord>>,
    /// <p>A token used for advancing to the next page of results of your get relational database bundles request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetInstanceAccessDetailsRequest {
    /// <p>The name of the instance to access.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>The protocol to use to connect to your instance. Defaults to <code>ssh</code>.</p>
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetInstanceAccessDetailsResult {
    /// <p>An array of key-value pairs containing information about a get instance access request.</p>
    #[serde(rename = "accessDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_details: Option<InstanceAccessDetails>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetInstanceMetricDataRequest {
    /// <p>The end time of the time period.</p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p>The name of the instance for which you want to get metrics data.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>The metric name to get data about. </p>
    #[serde(rename = "metricName")]
    pub metric_name: String,
    /// <p>The granularity, in seconds, of the returned data points.</p>
    #[serde(rename = "period")]
    pub period: i64,
    /// <p>The start time of the time period.</p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
    /// <p>The instance statistics. </p>
    #[serde(rename = "statistics")]
    pub statistics: Vec<String>,
    /// <p>The unit. The list of valid values is below.</p>
    #[serde(rename = "unit")]
    pub unit: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetInstanceMetricDataResult {
    /// <p>An array of key-value pairs containing information about the results of your get instance metric data request.</p>
    #[serde(rename = "metricData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_data: Option<Vec<MetricDatapoint>>,
    /// <p>The metric name to return data for.</p>
    #[serde(rename = "metricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetInstancePortStatesRequest {
    /// <p>The name of the instance.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetInstancePortStatesResult {
    /// <p>Information about the port states resulting from your request.</p>
    #[serde(rename = "portStates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_states: Option<Vec<InstancePortState>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetInstanceRequest {
    /// <p>The name of the instance.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetInstanceResult {
    /// <p>An array of key-value pairs containing information about the specified instance.</p>
    #[serde(rename = "instance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<Instance>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetInstanceSnapshotRequest {
    /// <p>The name of the snapshot for which you are requesting information.</p>
    #[serde(rename = "instanceSnapshotName")]
    pub instance_snapshot_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetInstanceSnapshotResult {
    /// <p>An array of key-value pairs containing information about the results of your get instance snapshot request.</p>
    #[serde(rename = "instanceSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_snapshot: Option<InstanceSnapshot>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetInstanceSnapshotsRequest {
    /// <p>A token used for advancing to the next page of results from your get instance snapshots request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetInstanceSnapshotsResult {
    /// <p>An array of key-value pairs containing information about the results of your get instance snapshots request.</p>
    #[serde(rename = "instanceSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_snapshots: Option<Vec<InstanceSnapshot>>,
    /// <p>A token used for advancing to the next page of results from your get instance snapshots request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetInstanceStateRequest {
    /// <p>The name of the instance to get state information about.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetInstanceStateResult {
    /// <p>The state of the instance.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<InstanceState>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetInstancesRequest {
    /// <p>A token used for advancing to the next page of results from your get instances request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetInstancesResult {
    /// <p>An array of key-value pairs containing information about your instances.</p>
    #[serde(rename = "instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<Instance>>,
    /// <p>A token used for advancing to the next page of results from your get instances request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetKeyPairRequest {
    /// <p>The name of the key pair for which you are requesting information.</p>
    #[serde(rename = "keyPairName")]
    pub key_pair_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetKeyPairResult {
    /// <p>An array of key-value pairs containing information about the key pair.</p>
    #[serde(rename = "keyPair")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair: Option<KeyPair>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetKeyPairsRequest {
    /// <p>A token used for advancing to the next page of results from your get key pairs request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetKeyPairsResult {
    /// <p>An array of key-value pairs containing information about the key pairs.</p>
    #[serde(rename = "keyPairs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pairs: Option<Vec<KeyPair>>,
    /// <p>A token used for advancing to the next page of results from your get key pairs request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetLoadBalancerMetricDataRequest {
    /// <p>The end time of the period.</p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p>The name of the load balancer.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
    /// <p><p>The metric about which you want to return information. Valid values are listed below, along with the most useful <code>statistics</code> to include in your request.</p> <ul> <li> <p> <b> <code>ClientTLSNegotiationErrorCount</code> </b> - The number of TLS connections initiated by the client that did not establish a session with the load balancer. Possible causes include a mismatch of ciphers or protocols.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>.</p> </li> <li> <p> <b> <code>HealthyHostCount</code> </b> - The number of target instances that are considered healthy.</p> <p> <code>Statistics</code>: The most useful statistic are <code>Average</code>, <code>Minimum</code>, and <code>Maximum</code>.</p> </li> <li> <p> <b> <code>UnhealthyHostCount</code> </b> - The number of target instances that are considered unhealthy.</p> <p> <code>Statistics</code>: The most useful statistic are <code>Average</code>, <code>Minimum</code>, and <code>Maximum</code>.</p> </li> <li> <p> <b> <code>HTTPCode<em>LB</em>4XX<em>Count</code> </b> - The number of HTTP 4XX client error codes that originate from the load balancer. Client errors are generated when requests are malformed or incomplete. These requests have not been received by the target instance. This count does not include any response codes generated by the target instances.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>HTTPCode</em>LB<em>5XX</em>Count</code> </b> - The number of HTTP 5XX server error codes that originate from the load balancer. This count does not include any response codes generated by the target instances.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>HTTPCode<em>Instance</em>2XX<em>Count</code> </b> - The number of HTTP response codes generated by the target instances. This does not include any response codes generated by the load balancer.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>HTTPCode</em>Instance<em>3XX</em>Count</code> </b> - The number of HTTP response codes generated by the target instances. This does not include any response codes generated by the load balancer. </p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>HTTPCode<em>Instance</em>4XX<em>Count</code> </b> - The number of HTTP response codes generated by the target instances. This does not include any response codes generated by the load balancer.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>HTTPCode</em>Instance<em>5XX</em>Count</code> </b> - The number of HTTP response codes generated by the target instances. This does not include any response codes generated by the load balancer.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>InstanceResponseTime</code> </b> - The time elapsed, in seconds, after the request leaves the load balancer until a response from the target instance is received.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Average</code>.</p> </li> <li> <p> <b> <code>RejectedConnectionCount</code> </b> - The number of connections that were rejected because the load balancer had reached its maximum number of connections.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>.</p> </li> <li> <p> <b> <code>RequestCount</code> </b> - The number of requests processed over IPv4. This count includes only the requests with a response generated by a target instance of the load balancer.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> </ul></p>
    #[serde(rename = "metricName")]
    pub metric_name: String,
    /// <p>The granularity, in seconds, of the returned data points.</p>
    #[serde(rename = "period")]
    pub period: i64,
    /// <p>The start time of the period.</p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
    /// <p><p>An array of statistics that you want to request metrics for. Valid values are listed below.</p> <ul> <li> <p> <b> <code>SampleCount</code> </b> - The count (number) of data points used for the statistical calculation.</p> </li> <li> <p> <b> <code>Average</code> </b> - The value of Sum / SampleCount during the specified period. By comparing this statistic with the Minimum and Maximum, you can determine the full scope of a metric and how close the average use is to the Minimum and Maximum. This comparison helps you to know when to increase or decrease your resources as needed.</p> </li> <li> <p> <b> <code>Sum</code> </b> - All values submitted for the matching metric added together. This statistic can be useful for determining the total volume of a metric.</p> </li> <li> <p> <b> <code>Minimum</code> </b> - The lowest value observed during the specified period. You can use this value to determine low volumes of activity for your application.</p> </li> <li> <p> <b> <code>Maximum</code> </b> - The highest value observed during the specified period. You can use this value to determine high volumes of activity for your application.</p> </li> </ul></p>
    #[serde(rename = "statistics")]
    pub statistics: Vec<String>,
    /// <p>The unit for the time period request. Valid values are listed below.</p>
    #[serde(rename = "unit")]
    pub unit: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetLoadBalancerMetricDataResult {
    /// <p>An array of metric datapoint objects.</p>
    #[serde(rename = "metricData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_data: Option<Vec<MetricDatapoint>>,
    /// <p><p>The metric about which you are receiving information. Valid values are listed below, along with the most useful <code>statistics</code> to include in your request.</p> <ul> <li> <p> <b> <code>ClientTLSNegotiationErrorCount</code> </b> - The number of TLS connections initiated by the client that did not establish a session with the load balancer. Possible causes include a mismatch of ciphers or protocols.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>.</p> </li> <li> <p> <b> <code>HealthyHostCount</code> </b> - The number of target instances that are considered healthy.</p> <p> <code>Statistics</code>: The most useful statistic are <code>Average</code>, <code>Minimum</code>, and <code>Maximum</code>.</p> </li> <li> <p> <b> <code>UnhealthyHostCount</code> </b> - The number of target instances that are considered unhealthy.</p> <p> <code>Statistics</code>: The most useful statistic are <code>Average</code>, <code>Minimum</code>, and <code>Maximum</code>.</p> </li> <li> <p> <b> <code>HTTPCode<em>LB</em>4XX<em>Count</code> </b> - The number of HTTP 4XX client error codes that originate from the load balancer. Client errors are generated when requests are malformed or incomplete. These requests have not been received by the target instance. This count does not include any response codes generated by the target instances.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>HTTPCode</em>LB<em>5XX</em>Count</code> </b> - The number of HTTP 5XX server error codes that originate from the load balancer. This count does not include any response codes generated by the target instances.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>HTTPCode<em>Instance</em>2XX<em>Count</code> </b> - The number of HTTP response codes generated by the target instances. This does not include any response codes generated by the load balancer.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>HTTPCode</em>Instance<em>3XX</em>Count</code> </b> - The number of HTTP response codes generated by the target instances. This does not include any response codes generated by the load balancer. </p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>HTTPCode<em>Instance</em>4XX<em>Count</code> </b> - The number of HTTP response codes generated by the target instances. This does not include any response codes generated by the load balancer.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>HTTPCode</em>Instance<em>5XX</em>Count</code> </b> - The number of HTTP response codes generated by the target instances. This does not include any response codes generated by the load balancer.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> <li> <p> <b> <code>InstanceResponseTime</code> </b> - The time elapsed, in seconds, after the request leaves the load balancer until a response from the target instance is received.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Average</code>.</p> </li> <li> <p> <b> <code>RejectedConnectionCount</code> </b> - The number of connections that were rejected because the load balancer had reached its maximum number of connections.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>.</p> </li> <li> <p> <b> <code>RequestCount</code> </b> - The number of requests processed over IPv4. This count includes only the requests with a response generated by a target instance of the load balancer.</p> <p> <code>Statistics</code>: The most useful statistic is <code>Sum</code>. Note that <code>Minimum</code>, <code>Maximum</code>, and <code>Average</code> all return <code>1</code>.</p> </li> </ul></p>
    #[serde(rename = "metricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetLoadBalancerRequest {
    /// <p>The name of the load balancer.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetLoadBalancerResult {
    /// <p>An object containing information about your load balancer.</p>
    #[serde(rename = "loadBalancer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer: Option<LoadBalancer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetLoadBalancerTlsCertificatesRequest {
    /// <p>The name of the load balancer you associated with your SSL/TLS certificate.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetLoadBalancerTlsCertificatesResult {
    /// <p>An array of LoadBalancerTlsCertificate objects describing your SSL/TLS certificates.</p>
    #[serde(rename = "tlsCertificates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_certificates: Option<Vec<LoadBalancerTlsCertificate>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetLoadBalancersRequest {
    /// <p>A token used for paginating the results from your GetLoadBalancers request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetLoadBalancersResult {
    /// <p>An array of LoadBalancer objects describing your load balancers.</p>
    #[serde(rename = "loadBalancers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<LoadBalancer>>,
    /// <p>A token used for advancing to the next page of results from your GetLoadBalancers request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetOperationRequest {
    /// <p>A GUID used to identify the operation.</p>
    #[serde(rename = "operationId")]
    pub operation_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetOperationResult {
    /// <p>An array of key-value pairs containing information about the results of your get operation request.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetOperationsForResourceRequest {
    /// <p>A token used for advancing to the next page of results from your get operations for resource request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The name of the resource for which you are requesting information.</p>
    #[serde(rename = "resourceName")]
    pub resource_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetOperationsForResourceResult {
    /// <p>An identifier that was returned from the previous call to this operation, which can be used to return the next set of items in the list.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>An array of key-value pairs containing information about the results of your get operations for resource request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetOperationsRequest {
    /// <p>A token used for advancing to the next page of results from your get operations request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetOperationsResult {
    /// <p>A token used for advancing to the next page of results from your get operations request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>An array of key-value pairs containing information about the results of your get operations request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRegionsRequest {
    /// <p>A Boolean value indicating whether to also include Availability Zones in your get regions request. Availability Zones are indicated with a letter: e.g., <code>us-east-2a</code>.</p>
    #[serde(rename = "includeAvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_availability_zones: Option<bool>,
    /// <p>&gt;A Boolean value indicating whether to also include Availability Zones for databases in your get regions request. Availability Zones are indicated with a letter (e.g., <code>us-east-2a</code>).</p>
    #[serde(rename = "includeRelationalDatabaseAvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_relational_database_availability_zones: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRegionsResult {
    /// <p>An array of key-value pairs containing information about your get regions request.</p>
    #[serde(rename = "regions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<Region>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRelationalDatabaseBlueprintsRequest {
    /// <p>A token used for advancing to a specific page of results for your <code>get relational database blueprints</code> request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRelationalDatabaseBlueprintsResult {
    /// <p>An object describing the result of your get relational database blueprints request.</p>
    #[serde(rename = "blueprints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprints: Option<Vec<RelationalDatabaseBlueprint>>,
    /// <p>A token used for advancing to the next page of results of your get relational database blueprints request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRelationalDatabaseBundlesRequest {
    /// <p>A token used for advancing to a specific page of results for your <code>get relational database bundles</code> request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRelationalDatabaseBundlesResult {
    /// <p>An object describing the result of your get relational database bundles request.</p>
    #[serde(rename = "bundles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundles: Option<Vec<RelationalDatabaseBundle>>,
    /// <p>A token used for advancing to the next page of results of your get relational database bundles request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRelationalDatabaseEventsRequest {
    /// <p>The number of minutes in the past from which to retrieve events. For example, to get all events from the past 2 hours, enter 120.</p> <p>Default: <code>60</code> </p> <p>The minimum is 1 and the maximum is 14 days (20160 minutes).</p>
    #[serde(rename = "durationInMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_minutes: Option<i64>,
    /// <p>A token used for advancing to a specific page of results from for get relational database events request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The name of the database from which to get events.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRelationalDatabaseEventsResult {
    /// <p>A token used for advancing to the next page of results from your get relational database events request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>An object describing the result of your get relational database events request.</p>
    #[serde(rename = "relationalDatabaseEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_events: Option<Vec<RelationalDatabaseEvent>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRelationalDatabaseLogEventsRequest {
    /// <p><p>The end of the time interval from which to get log events.</p> <p>Constraints:</p> <ul> <li> <p>Specified in Universal Coordinated Time (UTC).</p> </li> <li> <p>Specified in the Unix time format.</p> <p>For example, if you wish to use an end time of October 1, 2018, at 8 PM UTC, then you input <code>1538424000</code> as the end time.</p> </li> </ul></p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The name of the log stream.</p> <p>Use the <code>get relational database log streams</code> operation to get a list of available log streams.</p>
    #[serde(rename = "logStreamName")]
    pub log_stream_name: String,
    /// <p>A token used for advancing to a specific page of results for your <code>get relational database log events</code> request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The name of your database for which to get log events.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
    /// <p>Parameter to specify if the log should start from head or tail. If <code>true</code> is specified, the log event starts from the head of the log. If <code>false</code> is specified, the log event starts from the tail of the log.</p> <p>Default: <code>false</code> </p>
    #[serde(rename = "startFromHead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_from_head: Option<bool>,
    /// <p><p>The start of the time interval from which to get log events.</p> <p>Constraints:</p> <ul> <li> <p>Specified in Universal Coordinated Time (UTC).</p> </li> <li> <p>Specified in the Unix time format.</p> <p>For example, if you wish to use a start time of October 1, 2018, at 8 PM UTC, then you input <code>1538424000</code> as the start time.</p> </li> </ul></p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRelationalDatabaseLogEventsResult {
    /// <p>A token used for advancing to the previous page of results from your get relational database log events request.</p>
    #[serde(rename = "nextBackwardToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_backward_token: Option<String>,
    /// <p>A token used for advancing to the next page of results from your get relational database log events request.</p>
    #[serde(rename = "nextForwardToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_forward_token: Option<String>,
    /// <p>An object describing the result of your get relational database log events request.</p>
    #[serde(rename = "resourceLogEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_log_events: Option<Vec<LogEvent>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRelationalDatabaseLogStreamsRequest {
    /// <p>The name of your database for which to get log streams.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRelationalDatabaseLogStreamsResult {
    /// <p>An object describing the result of your get relational database log streams request.</p>
    #[serde(rename = "logStreams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_streams: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRelationalDatabaseMasterUserPasswordRequest {
    /// <p>The password version to return.</p> <p>Specifying <code>CURRENT</code> or <code>PREVIOUS</code> returns the current or previous passwords respectively. Specifying <code>PENDING</code> returns the newest version of the password that will rotate to <code>CURRENT</code>. After the <code>PENDING</code> password rotates to <code>CURRENT</code>, the <code>PENDING</code> password is no longer available.</p> <p>Default: <code>CURRENT</code> </p>
    #[serde(rename = "passwordVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_version: Option<String>,
    /// <p>The name of your database for which to get the master user password.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRelationalDatabaseMasterUserPasswordResult {
    /// <p>The timestamp when the specified version of the master user password was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The master user password for the <code>password version</code> specified.</p>
    #[serde(rename = "masterUserPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRelationalDatabaseMetricDataRequest {
    /// <p><p>The end of the time interval from which to get metric data.</p> <p>Constraints:</p> <ul> <li> <p>Specified in Universal Coordinated Time (UTC).</p> </li> <li> <p>Specified in the Unix time format.</p> <p>For example, if you wish to use an end time of October 1, 2018, at 8 PM UTC, then you input <code>1538424000</code> as the end time.</p> </li> </ul></p>
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// <p>The name of the metric data to return.</p>
    #[serde(rename = "metricName")]
    pub metric_name: String,
    /// <p>The granularity, in seconds, of the returned data points.</p>
    #[serde(rename = "period")]
    pub period: i64,
    /// <p>The name of your database from which to get metric data.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
    /// <p><p>The start of the time interval from which to get metric data.</p> <p>Constraints:</p> <ul> <li> <p>Specified in Universal Coordinated Time (UTC).</p> </li> <li> <p>Specified in the Unix time format.</p> <p>For example, if you wish to use a start time of October 1, 2018, at 8 PM UTC, then you input <code>1538424000</code> as the start time.</p> </li> </ul></p>
    #[serde(rename = "startTime")]
    pub start_time: f64,
    /// <p>The array of statistics for your metric data request.</p>
    #[serde(rename = "statistics")]
    pub statistics: Vec<String>,
    /// <p>The unit for the metric data request.</p>
    #[serde(rename = "unit")]
    pub unit: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRelationalDatabaseMetricDataResult {
    /// <p>An object describing the result of your get relational database metric data request.</p>
    #[serde(rename = "metricData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_data: Option<Vec<MetricDatapoint>>,
    /// <p>The name of the metric.</p>
    #[serde(rename = "metricName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metric_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRelationalDatabaseParametersRequest {
    /// <p>A token used for advancing to a specific page of results for your <code>get relational database parameters</code> request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// <p>The name of your database for which to get parameters.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRelationalDatabaseParametersResult {
    /// <p>A token used for advancing to the next page of results from your get static IPs request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>An object describing the result of your get relational database parameters request.</p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<RelationalDatabaseParameter>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRelationalDatabaseRequest {
    /// <p>The name of the database that you are looking up.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRelationalDatabaseResult {
    /// <p>An object describing the specified database.</p>
    #[serde(rename = "relationalDatabase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database: Option<RelationalDatabase>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRelationalDatabaseSnapshotRequest {
    /// <p>The name of the database snapshot for which to get information.</p>
    #[serde(rename = "relationalDatabaseSnapshotName")]
    pub relational_database_snapshot_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRelationalDatabaseSnapshotResult {
    /// <p>An object describing the specified database snapshot.</p>
    #[serde(rename = "relationalDatabaseSnapshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_snapshot: Option<RelationalDatabaseSnapshot>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRelationalDatabaseSnapshotsRequest {
    /// <p>A token used for advancing to a specific page of results for your <code>get relational database snapshots</code> request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRelationalDatabaseSnapshotsResult {
    /// <p>A token used for advancing to the next page of results from your get relational database snapshots request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>An object describing the result of your get relational database snapshots request.</p>
    #[serde(rename = "relationalDatabaseSnapshots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_snapshots: Option<Vec<RelationalDatabaseSnapshot>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRelationalDatabasesRequest {
    /// <p>A token used for advancing to a specific page of results for your <code>get relational database</code> request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetRelationalDatabasesResult {
    /// <p>A token used for advancing to the next page of results from your get relational databases request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>An object describing the result of your get relational databases request.</p>
    #[serde(rename = "relationalDatabases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_databases: Option<Vec<RelationalDatabase>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetStaticIpRequest {
    /// <p>The name of the static IP in Lightsail.</p>
    #[serde(rename = "staticIpName")]
    pub static_ip_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetStaticIpResult {
    /// <p>An array of key-value pairs containing information about the requested static IP.</p>
    #[serde(rename = "staticIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_ip: Option<StaticIp>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetStaticIpsRequest {
    /// <p>A token used for advancing to the next page of results from your get static IPs request.</p>
    #[serde(rename = "pageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetStaticIpsResult {
    /// <p>A token used for advancing to the next page of results from your get static IPs request.</p>
    #[serde(rename = "nextPageToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// <p>An array of key-value pairs containing information about your get static IPs request.</p>
    #[serde(rename = "staticIps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_ips: Option<Vec<StaticIp>>,
}

/// <p>Describes the public SSH host keys or the RDP certificate.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct HostKeyAttributes {
    /// <p>The SSH host key algorithm or the RDP certificate format.</p> <p>For SSH host keys, the algorithm may be <code>ssh-rsa</code>, <code>ecdsa-sha2-nistp256</code>, <code>ssh-ed25519</code>, etc. For RDP certificates, the algorithm is always <code>x509-cert</code>.</p>
    #[serde(rename = "algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// <p><p>The SHA-1 fingerprint of the returned SSH host key or RDP certificate.</p> <ul> <li> <p>Example of an SHA-1 SSH fingerprint:</p> <p> <code>SHA1:1CHH6FaAaXjtFOsR/t83vf91SR0</code> </p> </li> <li> <p>Example of an SHA-1 RDP fingerprint:</p> <p> <code>af:34:51:fe:09:f0:e0:da:b8:4e:56:ca:60:c2:10:ff:38:06:db:45</code> </p> </li> </ul></p>
    #[serde(rename = "fingerprintSHA1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint_sha1: Option<String>,
    /// <p><p>The SHA-256 fingerprint of the returned SSH host key or RDP certificate.</p> <ul> <li> <p>Example of an SHA-256 SSH fingerprint:</p> <p> <code>SHA256:KTsMnRBh1IhD17HpdfsbzeGA4jOijm5tyXsMjKVbB8o</code> </p> </li> <li> <p>Example of an SHA-256 RDP fingerprint:</p> <p> <code>03:9b:36:9f:4b:de:4e:61:70:fc:7c:c9:78:e7:d2:1a:1c:25:a8:0c:91:f6:7c:e4:d6:a0:85:c8:b4:53:99:68</code> </p> </li> </ul></p>
    #[serde(rename = "fingerprintSHA256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint_sha256: Option<String>,
    /// <p>The returned RDP certificate is not valid after this point in time.</p> <p>This value is listed only for RDP certificates.</p>
    #[serde(rename = "notValidAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_valid_after: Option<f64>,
    /// <p>The returned RDP certificate is valid after this point in time.</p> <p>This value is listed only for RDP certificates.</p>
    #[serde(rename = "notValidBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_valid_before: Option<f64>,
    /// <p>The public SSH host key or the RDP certificate.</p>
    #[serde(rename = "publicKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    /// <p>The time that the SSH host key or RDP certificate was recorded by Lightsail.</p>
    #[serde(rename = "witnessedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub witnessed_at: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportKeyPairRequest {
    /// <p>The name of the key pair for which you want to import the public key.</p>
    #[serde(rename = "keyPairName")]
    pub key_pair_name: String,
    /// <p>A base64-encoded public key of the <code>ssh-rsa</code> type.</p>
    #[serde(rename = "publicKeyBase64")]
    pub public_key_base_64: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ImportKeyPairResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

/// <p>Describes an instance (a virtual private server).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Instance {
    /// <p>The Amazon Resource Name (ARN) of the instance (e.g., <code>arn:aws:lightsail:us-east-2:123456789101:Instance/244ad76f-8aad-4741-809f-12345EXAMPLE</code>).</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The blueprint ID (e.g., <code>os_amlinux_2016_03</code>).</p>
    #[serde(rename = "blueprintId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_id: Option<String>,
    /// <p>The friendly name of the blueprint (e.g., <code>Amazon Linux</code>).</p>
    #[serde(rename = "blueprintName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_name: Option<String>,
    /// <p>The bundle for the instance (e.g., <code>micro_1_0</code>).</p>
    #[serde(rename = "bundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The timestamp when the instance was created (e.g., <code>1479734909.17</code>).</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The size of the vCPU and the amount of RAM for the instance.</p>
    #[serde(rename = "hardware")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware: Option<InstanceHardware>,
    /// <p>The IPv6 address of the instance.</p>
    #[serde(rename = "ipv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_6_address: Option<String>,
    /// <p>A Boolean value indicating whether this instance has a static IP assigned to it.</p>
    #[serde(rename = "isStaticIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_static_ip: Option<bool>,
    /// <p>The region name and Availability Zone where the instance is located.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The name the user gave the instance (e.g., <code>Amazon_Linux-1GB-Ohio-1</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Information about the public ports and monthly data transfer rates for the instance.</p>
    #[serde(rename = "networking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networking: Option<InstanceNetworking>,
    /// <p>The private IP address of the instance.</p>
    #[serde(rename = "privateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    /// <p>The public IP address of the instance.</p>
    #[serde(rename = "publicIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
    /// <p>The type of resource (usually <code>Instance</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The name of the SSH key being used to connect to the instance (e.g., <code>LightsailDefaultKeyPair</code>).</p>
    #[serde(rename = "sshKeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssh_key_name: Option<String>,
    /// <p>The status code and the state (e.g., <code>running</code>) for the instance.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<InstanceState>,
    /// <p>The support code. Include this code in your email to support when you have questions about an instance or another resource in Lightsail. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
    /// <p>The tag keys and optional values for the resource. For more information about tags in Lightsail, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The user name for connecting to the instance (e.g., <code>ec2-user</code>).</p>
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>The parameters for gaining temporary access to one of your Amazon Lightsail instances.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InstanceAccessDetails {
    /// <p>For SSH access, the public key to use when accessing your instance For OpenSSH clients (e.g., command line SSH), you should save this value to <code>tempkey-cert.pub</code>.</p>
    #[serde(rename = "certKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_key: Option<String>,
    /// <p>For SSH access, the date on which the temporary keys expire.</p>
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<f64>,
    /// <p>Describes the public SSH host keys or the RDP certificate.</p>
    #[serde(rename = "hostKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_keys: Option<Vec<HostKeyAttributes>>,
    /// <p>The name of this Amazon Lightsail instance.</p>
    #[serde(rename = "instanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    /// <p>The public IP address of the Amazon Lightsail instance.</p>
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p><p>For RDP access, the password for your Amazon Lightsail instance. Password will be an empty string if the password for your new instance is not ready yet. When you create an instance, it can take up to 15 minutes for the instance to be ready.</p> <note> <p>If you create an instance using any key pair other than the default (<code>LightsailDefaultKeyPair</code>), <code>password</code> will always be an empty string.</p> <p>If you change the Administrator password on the instance, Lightsail will continue to return the original password value. When accessing the instance using RDP, you need to manually enter the Administrator password after changing it from the default.</p> </note></p>
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>For a Windows Server-based instance, an object with the data you can use to retrieve your password. This is only needed if <code>password</code> is empty and the instance is not new (and therefore the password is not ready yet). When you create an instance, it can take up to 15 minutes for the instance to be ready.</p>
    #[serde(rename = "passwordData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_data: Option<PasswordData>,
    /// <p>For SSH access, the temporary private key. For OpenSSH clients (e.g., command line SSH), you should save this value to <code>tempkey</code>).</p>
    #[serde(rename = "privateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
    /// <p>The protocol for these Amazon Lightsail instance access details.</p>
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>The user name to use when logging in to the Amazon Lightsail instance.</p>
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Describes the Amazon Elastic Compute Cloud instance and related resources to be created using the <code>create cloud formation stack</code> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InstanceEntry {
    /// <p>The Availability Zone for the new Amazon EC2 instance.</p>
    #[serde(rename = "availabilityZone")]
    pub availability_zone: String,
    /// <p>The instance type (e.g., <code>t2.micro</code>) to use for the new Amazon EC2 instance.</p>
    #[serde(rename = "instanceType")]
    pub instance_type: String,
    /// <p><p>The port configuration to use for the new Amazon EC2 instance.</p> <p>The following configuration options are available:</p> <ul> <li> <p>DEFAULT — Use the default firewall settings from the image.</p> </li> <li> <p>INSTANCE — Use the firewall settings from the source Lightsail instance.</p> </li> <li> <p>NONE — Default to Amazon EC2.</p> </li> <li> <p>CLOSED — All ports closed.</p> </li> </ul></p>
    #[serde(rename = "portInfoSource")]
    pub port_info_source: String,
    /// <p>The name of the export snapshot record, which contains the exported Lightsail instance snapshot that will be used as the source of the new Amazon EC2 instance.</p> <p>Use the <code>get export snapshot records</code> operation to get a list of export snapshot records that you can use to create a CloudFormation stack.</p>
    #[serde(rename = "sourceName")]
    pub source_name: String,
    /// <p><p>A launch script you can create that configures a server with additional user data. For example, you might want to run <code>apt-get -y update</code>.</p> <note> <p>Depending on the machine image you choose, the command to get software on your instance varies. Amazon Linux and CentOS use <code>yum</code>, Debian and Ubuntu use <code>apt-get</code>, and FreeBSD uses <code>pkg</code>.</p> </note></p>
    #[serde(rename = "userData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<String>,
}

/// <p>Describes the hardware for the instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InstanceHardware {
    /// <p>The number of vCPUs the instance has.</p>
    #[serde(rename = "cpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i64>,
    /// <p>The disks attached to the instance.</p>
    #[serde(rename = "disks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disks: Option<Vec<Disk>>,
    /// <p>The amount of RAM in GB on the instance (e.g., <code>1.0</code>).</p>
    #[serde(rename = "ramSizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_size_in_gb: Option<f32>,
}

/// <p>Describes information about the health of the instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InstanceHealthSummary {
    /// <p>Describes the overall instance health. Valid values are below.</p>
    #[serde(rename = "instanceHealth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_health: Option<String>,
    /// <p><p>More information about the instance health. If the <code>instanceHealth</code> is <code>healthy</code>, then an <code>instanceHealthReason</code> value is not provided.</p> <p>If <b> <code>instanceHealth</code> </b> is <code>initial</code>, the <b> <code>instanceHealthReason</code> </b> value can be one of the following:</p> <ul> <li> <p> <b> <code>Lb.RegistrationInProgress</code> </b> - The target instance is in the process of being registered with the load balancer.</p> </li> <li> <p> <b> <code>Lb.InitialHealthChecking</code> </b> - The Lightsail load balancer is still sending the target instance the minimum number of health checks required to determine its health status.</p> </li> </ul> <p>If <b> <code>instanceHealth</code> </b> is <code>unhealthy</code>, the <b> <code>instanceHealthReason</code> </b> value can be one of the following:</p> <ul> <li> <p> <b> <code>Instance.ResponseCodeMismatch</code> </b> - The health checks did not return an expected HTTP code.</p> </li> <li> <p> <b> <code>Instance.Timeout</code> </b> - The health check requests timed out.</p> </li> <li> <p> <b> <code>Instance.FailedHealthChecks</code> </b> - The health checks failed because the connection to the target instance timed out, the target instance response was malformed, or the target instance failed the health check for an unknown reason.</p> </li> <li> <p> <b> <code>Lb.InternalError</code> </b> - The health checks failed due to an internal error.</p> </li> </ul> <p>If <b> <code>instanceHealth</code> </b> is <code>unused</code>, the <b> <code>instanceHealthReason</code> </b> value can be one of the following:</p> <ul> <li> <p> <b> <code>Instance.NotRegistered</code> </b> - The target instance is not registered with the target group.</p> </li> <li> <p> <b> <code>Instance.NotInUse</code> </b> - The target group is not used by any load balancer, or the target instance is in an Availability Zone that is not enabled for its load balancer.</p> </li> <li> <p> <b> <code>Instance.IpUnusable</code> </b> - The target IP address is reserved for use by a Lightsail load balancer.</p> </li> <li> <p> <b> <code>Instance.InvalidState</code> </b> - The target is in the stopped or terminated state.</p> </li> </ul> <p>If <b> <code>instanceHealth</code> </b> is <code>draining</code>, the <b> <code>instanceHealthReason</code> </b> value can be one of the following:</p> <ul> <li> <p> <b> <code>Instance.DeregistrationInProgress</code> </b> - The target instance is in the process of being deregistered and the deregistration delay period has not expired.</p> </li> </ul></p>
    #[serde(rename = "instanceHealthReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_health_reason: Option<String>,
    /// <p>The name of the Lightsail instance for which you are requesting health check data.</p>
    #[serde(rename = "instanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
}

/// <p>Describes monthly data transfer rates and port information for an instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InstanceNetworking {
    /// <p>The amount of data in GB allocated for monthly data transfers.</p>
    #[serde(rename = "monthlyTransfer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_transfer: Option<MonthlyTransfer>,
    /// <p>An array of key-value pairs containing information about the ports on the instance.</p>
    #[serde(rename = "ports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ports: Option<Vec<InstancePortInfo>>,
}

/// <p>Describes information about the instance ports.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InstancePortInfo {
    /// <p>The access direction (<code>inbound</code> or <code>outbound</code>).</p>
    #[serde(rename = "accessDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_direction: Option<String>,
    /// <p>The location from which access is allowed (e.g., <code>Anywhere (0.0.0.0/0)</code>).</p>
    #[serde(rename = "accessFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_from: Option<String>,
    /// <p>The type of access (<code>Public</code> or <code>Private</code>).</p>
    #[serde(rename = "accessType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<String>,
    /// <p>The common name.</p>
    #[serde(rename = "commonName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_name: Option<String>,
    /// <p>The first port in the range.</p>
    #[serde(rename = "fromPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_port: Option<i64>,
    /// <p><p>The protocol being used. Can be one of the following.</p> <ul> <li> <p> <code>tcp</code> - Transmission Control Protocol (TCP) provides reliable, ordered, and error-checked delivery of streamed data between applications running on hosts communicating by an IP network. If you have an application that doesn&#39;t require reliable data stream service, use UDP instead.</p> </li> <li> <p> <code>all</code> - All transport layer protocol types. For more general information, see <a href="https://en.wikipedia.org/wiki/Transport_layer">Transport layer</a> on Wikipedia.</p> </li> <li> <p> <code>udp</code> - With User Datagram Protocol (UDP), computer applications can send messages (or datagrams) to other hosts on an Internet Protocol (IP) network. Prior communications are not required to set up transmission channels or data paths. Applications that don&#39;t require reliable data stream service can use UDP, which provides a connectionless datagram service that emphasizes reduced latency over reliability. If you do require reliable data stream service, use TCP instead.</p> </li> </ul></p>
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>The last port in the range.</p>
    #[serde(rename = "toPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_port: Option<i64>,
}

/// <p>Describes the port state.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InstancePortState {
    /// <p>The first port in the range.</p>
    #[serde(rename = "fromPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_port: Option<i64>,
    /// <p><p>The protocol being used. Can be one of the following.</p> <ul> <li> <p> <code>tcp</code> - Transmission Control Protocol (TCP) provides reliable, ordered, and error-checked delivery of streamed data between applications running on hosts communicating by an IP network. If you have an application that doesn&#39;t require reliable data stream service, use UDP instead.</p> </li> <li> <p> <code>all</code> - All transport layer protocol types. For more general information, see <a href="https://en.wikipedia.org/wiki/Transport_layer">Transport layer</a> on Wikipedia.</p> </li> <li> <p> <code>udp</code> - With User Datagram Protocol (UDP), computer applications can send messages (or datagrams) to other hosts on an Internet Protocol (IP) network. Prior communications are not required to set up transmission channels or data paths. Applications that don&#39;t require reliable data stream service can use UDP, which provides a connectionless datagram service that emphasizes reduced latency over reliability. If you do require reliable data stream service, use TCP instead.</p> </li> </ul></p>
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>Specifies whether the instance port is <code>open</code> or <code>closed</code>.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The last port in the range.</p>
    #[serde(rename = "toPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_port: Option<i64>,
}

/// <p>Describes the snapshot of the virtual private server, or <i>instance</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InstanceSnapshot {
    /// <p>The Amazon Resource Name (ARN) of the snapshot (e.g., <code>arn:aws:lightsail:us-east-2:123456789101:InstanceSnapshot/d23b5706-3322-4d83-81e5-12345EXAMPLE</code>).</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The timestamp when the snapshot was created (e.g., <code>1479907467.024</code>).</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>An array of disk objects containing information about all block storage disks.</p>
    #[serde(rename = "fromAttachedDisks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_attached_disks: Option<Vec<Disk>>,
    /// <p>The blueprint ID from which you created the snapshot (e.g., <code>os_debian_8_3</code>). A blueprint is a virtual private server (or <i>instance</i>) image used to create instances quickly.</p>
    #[serde(rename = "fromBlueprintId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_blueprint_id: Option<String>,
    /// <p>The bundle ID from which you created the snapshot (e.g., <code>micro_1_0</code>).</p>
    #[serde(rename = "fromBundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_bundle_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the instance from which the snapshot was created (e.g., <code>arn:aws:lightsail:us-east-2:123456789101:Instance/64b8404c-ccb1-430b-8daf-12345EXAMPLE</code>).</p>
    #[serde(rename = "fromInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_instance_arn: Option<String>,
    /// <p>The instance from which the snapshot was created.</p>
    #[serde(rename = "fromInstanceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_instance_name: Option<String>,
    /// <p>The region name and Availability Zone where you created the snapshot.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The name of the snapshot.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The progress of the snapshot.</p>
    #[serde(rename = "progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    /// <p>The type of resource (usually <code>InstanceSnapshot</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The size in GB of the SSD.</p>
    #[serde(rename = "sizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_gb: Option<i64>,
    /// <p>The state the snapshot is in.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The support code. Include this code in your email to support when you have questions about an instance or another resource in Lightsail. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
    /// <p>The tag keys and optional values for the resource. For more information about tags in Lightsail, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Describes an instance snapshot.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InstanceSnapshotInfo {
    /// <p>The blueprint ID from which the source instance (e.g., <code>os_debian_8_3</code>).</p>
    #[serde(rename = "fromBlueprintId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_blueprint_id: Option<String>,
    /// <p>The bundle ID from which the source instance was created (e.g., <code>micro_1_0</code>).</p>
    #[serde(rename = "fromBundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_bundle_id: Option<String>,
    /// <p>A list of objects describing the disks that were attached to the source instance.</p>
    #[serde(rename = "fromDiskInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_disk_info: Option<Vec<DiskInfo>>,
}

/// <p>Describes the virtual private server (or <i>instance</i>) status.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InstanceState {
    /// <p>The status code for the instance.</p>
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i64>,
    /// <p>The state of the instance (e.g., <code>running</code> or <code>pending</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct IsVpcPeeredRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct IsVpcPeeredResult {
    /// <p>Returns <code>true</code> if the Lightsail VPC is peered; otherwise, <code>false</code>.</p>
    #[serde(rename = "isPeered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_peered: Option<bool>,
}

/// <p>Describes the SSH key pair.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct KeyPair {
    /// <p>The Amazon Resource Name (ARN) of the key pair (e.g., <code>arn:aws:lightsail:us-east-2:123456789101:KeyPair/05859e3d-331d-48ba-9034-12345EXAMPLE</code>).</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The timestamp when the key pair was created (e.g., <code>1479816991.349</code>).</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The RSA fingerprint of the key pair.</p>
    #[serde(rename = "fingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// <p>The region name and Availability Zone where the key pair was created.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The friendly name of the SSH key pair.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The resource type (usually <code>KeyPair</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The support code. Include this code in your email to support when you have questions about an instance or another resource in Lightsail. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
    /// <p>The tag keys and optional values for the resource. For more information about tags in Lightsail, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Describes the Lightsail load balancer.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LoadBalancer {
    /// <p>The Amazon Resource Name (ARN) of the load balancer.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A string to string map of the configuration options for your load balancer. Valid values are listed below.</p>
    #[serde(rename = "configurationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_options: Option<::std::collections::HashMap<String, String>>,
    /// <p>The date when your load balancer was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The DNS name of your Lightsail load balancer.</p>
    #[serde(rename = "dnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name: Option<String>,
    /// <p>The path you specified to perform your health checks. If no path is specified, the load balancer tries to make a request to the default (root) page.</p>
    #[serde(rename = "healthCheckPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_path: Option<String>,
    /// <p>An array of InstanceHealthSummary objects describing the health of the load balancer.</p>
    #[serde(rename = "instanceHealthSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_health_summary: Option<Vec<InstanceHealthSummary>>,
    /// <p>The port where the load balancer will direct traffic to your Lightsail instances. For HTTP traffic, it's port 80. For HTTPS traffic, it's port 443.</p>
    #[serde(rename = "instancePort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_port: Option<i64>,
    /// <p>The AWS Region where your load balancer was created (e.g., <code>us-east-2a</code>). Lightsail automatically creates your load balancer across Availability Zones.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The name of the load balancer (e.g., <code>my-load-balancer</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The protocol you have enabled for your load balancer. Valid values are below.</p> <p>You can't just have <code>HTTP_HTTPS</code>, but you can have just <code>HTTP</code>.</p>
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>An array of public port settings for your load balancer. For HTTP, use port 80. For HTTPS, use port 443.</p>
    #[serde(rename = "publicPorts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ports: Option<Vec<i64>>,
    /// <p>The resource type (e.g., <code>LoadBalancer</code>.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The status of your load balancer. Valid values are below.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The support code. Include this code in your email to support when you have questions about your Lightsail load balancer. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
    /// <p>The tag keys and optional values for the resource. For more information about tags in Lightsail, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>An array of LoadBalancerTlsCertificateSummary objects that provide additional information about the SSL/TLS certificates. For example, if <code>true</code>, the certificate is attached to the load balancer.</p>
    #[serde(rename = "tlsCertificateSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_certificate_summaries: Option<Vec<LoadBalancerTlsCertificateSummary>>,
}

/// <p>Describes a load balancer SSL/TLS certificate.</p> <p>TLS is just an updated, more secure version of Secure Socket Layer (SSL).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LoadBalancerTlsCertificate {
    /// <p>The Amazon Resource Name (ARN) of the SSL/TLS certificate.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The time when you created your SSL/TLS certificate.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The domain name for your SSL/TLS certificate.</p>
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>An array of LoadBalancerTlsCertificateDomainValidationRecord objects describing the records.</p>
    #[serde(rename = "domainValidationRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_validation_records: Option<Vec<LoadBalancerTlsCertificateDomainValidationRecord>>,
    /// <p>The reason for the SSL/TLS certificate validation failure.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>When <code>true</code>, the SSL/TLS certificate is attached to the Lightsail load balancer.</p>
    #[serde(rename = "isAttached")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_attached: Option<bool>,
    /// <p>The time when the SSL/TLS certificate was issued.</p>
    #[serde(rename = "issuedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<f64>,
    /// <p>The issuer of the certificate.</p>
    #[serde(rename = "issuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,
    /// <p>The algorithm that was used to generate the key pair (the public and private key).</p>
    #[serde(rename = "keyAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_algorithm: Option<String>,
    /// <p>The load balancer name where your SSL/TLS certificate is attached.</p>
    #[serde(rename = "loadBalancerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
    /// <p>The AWS Region and Availability Zone where you created your certificate.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The name of the SSL/TLS certificate (e.g., <code>my-certificate</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The timestamp when the SSL/TLS certificate expires.</p>
    #[serde(rename = "notAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_after: Option<f64>,
    /// <p>The timestamp when the SSL/TLS certificate is first valid.</p>
    #[serde(rename = "notBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_before: Option<f64>,
    /// <p>An object containing information about the status of Lightsail's managed renewal for the certificate.</p>
    #[serde(rename = "renewalSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_summary: Option<LoadBalancerTlsCertificateRenewalSummary>,
    /// <p><p>The resource type (e.g., <code>LoadBalancerTlsCertificate</code>).</p> <ul> <li> <p> <b> <code>Instance</code> </b> - A Lightsail instance (a virtual private server)</p> </li> <li> <p> <b> <code>StaticIp</code> </b> - A static IP address</p> </li> <li> <p> <b> <code>KeyPair</code> </b> - The key pair used to connect to a Lightsail instance</p> </li> <li> <p> <b> <code>InstanceSnapshot</code> </b> - A Lightsail instance snapshot</p> </li> <li> <p> <b> <code>Domain</code> </b> - A DNS zone</p> </li> <li> <p> <b> <code>PeeredVpc</code> </b> - A peered VPC</p> </li> <li> <p> <b> <code>LoadBalancer</code> </b> - A Lightsail load balancer</p> </li> <li> <p> <b> <code>LoadBalancerTlsCertificate</code> </b> - An SSL/TLS certificate associated with a Lightsail load balancer</p> </li> <li> <p> <b> <code>Disk</code> </b> - A Lightsail block storage disk</p> </li> <li> <p> <b> <code>DiskSnapshot</code> </b> - A block storage disk snapshot</p> </li> </ul></p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The reason the certificate was revoked. Valid values are below.</p>
    #[serde(rename = "revocationReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revocation_reason: Option<String>,
    /// <p>The timestamp when the SSL/TLS certificate was revoked.</p>
    #[serde(rename = "revokedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked_at: Option<f64>,
    /// <p>The serial number of the certificate.</p>
    #[serde(rename = "serial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    /// <p>The algorithm that was used to sign the certificate.</p>
    #[serde(rename = "signatureAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_algorithm: Option<String>,
    /// <p>The status of the SSL/TLS certificate. Valid values are below.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The name of the entity that is associated with the public key contained in the certificate.</p>
    #[serde(rename = "subject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// <p>One or more domains or subdomains included in the certificate. This list contains the domain names that are bound to the public key that is contained in the certificate. The subject alternative names include the canonical domain name (CNAME) of the certificate and additional domain names that can be used to connect to the website, such as <code>example.com</code>, <code>www.example.com</code>, or <code>m.example.com</code>.</p>
    #[serde(rename = "subjectAlternativeNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_alternative_names: Option<Vec<String>>,
    /// <p>The support code. Include this code in your email to support when you have questions about your Lightsail load balancer or SSL/TLS certificate. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
    /// <p>The tag keys and optional values for the resource. For more information about tags in Lightsail, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Contains information about the domain names on an SSL/TLS certificate that you will use to validate domain ownership.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LoadBalancerTlsCertificateDomainValidationOption {
    /// <p>The fully qualified domain name in the certificate request.</p>
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The status of the domain validation. Valid values are listed below.</p>
    #[serde(rename = "validationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
}

/// <p>Describes the validation record of each domain name in the SSL/TLS certificate.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LoadBalancerTlsCertificateDomainValidationRecord {
    /// <p>The domain name against which your SSL/TLS certificate was validated.</p>
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>A fully qualified domain name in the certificate. For example, <code>example.com</code>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of validation record. For example, <code>CNAME</code> for domain validation.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The validation status. Valid values are listed below.</p>
    #[serde(rename = "validationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<String>,
    /// <p>The value for that type.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Contains information about the status of Lightsail's managed renewal for the certificate.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LoadBalancerTlsCertificateRenewalSummary {
    /// <p>Contains information about the validation of each domain name in the certificate, as it pertains to Lightsail's managed renewal. This is different from the initial validation that occurs as a result of the RequestCertificate request.</p>
    #[serde(rename = "domainValidationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_validation_options: Option<Vec<LoadBalancerTlsCertificateDomainValidationOption>>,
    /// <p>The status of Lightsail's managed renewal of the certificate. Valid values are listed below.</p>
    #[serde(rename = "renewalStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_status: Option<String>,
}

/// <p>Provides a summary of SSL/TLS certificate metadata.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LoadBalancerTlsCertificateSummary {
    /// <p>When <code>true</code>, the SSL/TLS certificate is attached to the Lightsail load balancer.</p>
    #[serde(rename = "isAttached")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_attached: Option<bool>,
    /// <p>The name of the SSL/TLS certificate.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Describes a database log event.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LogEvent {
    /// <p>The timestamp when the database log event was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The message of the database log event.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>Describes the metric data point.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct MetricDatapoint {
    /// <p>The average.</p>
    #[serde(rename = "average")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average: Option<f64>,
    /// <p>The maximum.</p>
    #[serde(rename = "maximum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    /// <p>The minimum.</p>
    #[serde(rename = "minimum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    /// <p>The sample count.</p>
    #[serde(rename = "sampleCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_count: Option<f64>,
    /// <p>The sum.</p>
    #[serde(rename = "sum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum: Option<f64>,
    /// <p>The timestamp (e.g., <code>1479816991.349</code>).</p>
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
    /// <p>The unit. </p>
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

/// <p>Describes the monthly data transfer in and out of your virtual private server (or <i>instance</i>).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct MonthlyTransfer {
    /// <p>The amount allocated per month (in GB).</p>
    #[serde(rename = "gbPerMonthAllocated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gb_per_month_allocated: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct OpenInstancePublicPortsRequest {
    /// <p>The name of the instance for which you want to open the public ports.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>An array of key-value pairs containing information about the port mappings.</p>
    #[serde(rename = "portInfo")]
    pub port_info: PortInfo,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct OpenInstancePublicPortsResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

/// <p>Describes the API operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Operation {
    /// <p>The timestamp when the operation was initialized (e.g., <code>1479816991.349</code>).</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The error code.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error details.</p>
    #[serde(rename = "errorDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<String>,
    /// <p>The ID of the operation.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A Boolean value indicating whether the operation is terminal.</p>
    #[serde(rename = "isTerminal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_terminal: Option<bool>,
    /// <p>The region and Availability Zone.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>Details about the operation (e.g., <code>Debian-1GB-Ohio-1</code>).</p>
    #[serde(rename = "operationDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_details: Option<String>,
    /// <p>The type of operation. </p>
    #[serde(rename = "operationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<String>,
    /// <p>The resource name.</p>
    #[serde(rename = "resourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    /// <p>The resource type. </p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The status of the operation. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The timestamp when the status was changed (e.g., <code>1479816991.349</code>).</p>
    #[serde(rename = "statusChangedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_changed_at: Option<f64>,
}

/// <p>The password data for the Windows Server-based instance, including the ciphertext and the key pair name.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PasswordData {
    /// <p><p>The encrypted password. Ciphertext will be an empty string if access to your new instance is not ready yet. When you create an instance, it can take up to 15 minutes for the instance to be ready.</p> <note> <p>If you use the default key pair (<code>LightsailDefaultKeyPair</code>), the decrypted password will be available in the password field.</p> <p>If you are using a custom key pair, you need to use your own means of decryption.</p> <p>If you change the Administrator password on the instance, Lightsail will continue to return the original ciphertext value. When accessing the instance using RDP, you need to manually enter the Administrator password after changing it from the default.</p> </note></p>
    #[serde(rename = "ciphertext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphertext: Option<String>,
    /// <p>The name of the key pair that you used when creating your instance. If no key pair name was specified when creating the instance, Lightsail uses the default key pair (<code>LightsailDefaultKeyPair</code>).</p> <p>If you are using a custom key pair, you need to use your own means of decrypting your password using the <code>ciphertext</code>. Lightsail creates the ciphertext by encrypting your password with the public key part of this key pair.</p>
    #[serde(rename = "keyPairName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_pair_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PeerVpcRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PeerVpcResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

/// <p>Describes a pending database maintenance action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PendingMaintenanceAction {
    /// <p>The type of pending database maintenance action.</p>
    #[serde(rename = "action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The effective date of the pending database maintenance action.</p>
    #[serde(rename = "currentApplyDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_apply_date: Option<f64>,
    /// <p>Additional detail about the pending database maintenance action.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// <p>Describes a pending database value modification.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PendingModifiedRelationalDatabaseValues {
    /// <p>A Boolean value indicating whether automated backup retention is enabled.</p>
    #[serde(rename = "backupRetentionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_enabled: Option<bool>,
    /// <p>The database engine version.</p>
    #[serde(rename = "engineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>The password for the master user of the database.</p>
    #[serde(rename = "masterUserPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
}

/// <p>Describes information about the ports on your virtual private server (or <i>instance</i>).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PortInfo {
    /// <p>The first port in the range.</p>
    #[serde(rename = "fromPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_port: Option<i64>,
    /// <p>The protocol. </p>
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// <p>The last port in the range.</p>
    #[serde(rename = "toPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_port: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutInstancePublicPortsRequest {
    /// <p>The Lightsail instance name of the public port(s) you are setting.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
    /// <p>Specifies information about the public port(s).</p>
    #[serde(rename = "portInfos")]
    pub port_infos: Vec<PortInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutInstancePublicPortsResult {
    /// <p>Describes metadata about the operation you just executed.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RebootInstanceRequest {
    /// <p>The name of the instance to reboot.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RebootInstanceResult {
    /// <p>An array of key-value pairs containing information about the request operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RebootRelationalDatabaseRequest {
    /// <p>The name of your database to reboot.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RebootRelationalDatabaseResult {
    /// <p>An object describing the result of your reboot relational database request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes the AWS Region.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Region {
    /// <p>The Availability Zones. Follows the format <code>us-east-2a</code> (case-sensitive).</p>
    #[serde(rename = "availabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<AvailabilityZone>>,
    /// <p>The continent code (e.g., <code>NA</code>, meaning North America).</p>
    #[serde(rename = "continentCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continent_code: Option<String>,
    /// <p>The description of the AWS Region (e.g., <code>This region is recommended to serve users in the eastern United States and eastern Canada</code>).</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The display name (e.g., <code>Ohio</code>).</p>
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// <p>The region name (e.g., <code>us-east-2</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Availability Zones for databases. Follows the format <code>us-east-2a</code> (case-sensitive).</p>
    #[serde(rename = "relationalDatabaseAvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_availability_zones: Option<Vec<AvailabilityZone>>,
}

/// <p>Describes a database.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RelationalDatabase {
    /// <p>The Amazon Resource Name (ARN) of the database.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A Boolean value indicating whether automated backup retention is enabled for the database.</p>
    #[serde(rename = "backupRetentionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_enabled: Option<bool>,
    /// <p>The timestamp when the database was created. Formatted in Unix time.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The database software (for example, <code>MySQL</code>).</p>
    #[serde(rename = "engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// <p>The database engine version (for example, <code>5.7.23</code>).</p>
    #[serde(rename = "engineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>Describes the hardware of the database.</p>
    #[serde(rename = "hardware")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware: Option<RelationalDatabaseHardware>,
    /// <p>The latest point in time to which the database can be restored. Formatted in Unix time.</p>
    #[serde(rename = "latestRestorableTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_restorable_time: Option<f64>,
    /// <p>The Region name and Availability Zone where the database is located.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The name of the master database created when the Lightsail database resource is created.</p>
    #[serde(rename = "masterDatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_database_name: Option<String>,
    /// <p>The master endpoint for the database.</p>
    #[serde(rename = "masterEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_endpoint: Option<RelationalDatabaseEndpoint>,
    /// <p>The master user name of the database.</p>
    #[serde(rename = "masterUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    /// <p>The unique name of the database resource in Lightsail.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of parameter updates for the database.</p>
    #[serde(rename = "parameterApplyStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_apply_status: Option<String>,
    /// <p>Describes the pending maintenance actions for the database.</p>
    #[serde(rename = "pendingMaintenanceActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_maintenance_actions: Option<Vec<PendingMaintenanceAction>>,
    /// <p>Describes pending database value modifications.</p>
    #[serde(rename = "pendingModifiedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_modified_values: Option<PendingModifiedRelationalDatabaseValues>,
    /// <p>The daily time range during which automated backups are created for the database (for example, <code>16:00-16:30</code>).</p>
    #[serde(rename = "preferredBackupWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    /// <p>The weekly time range during which system maintenance can occur on the database.</p> <p>In the format <code>ddd:hh24:mi-ddd:hh24:mi</code>. For example, <code>Tue:17:00-Tue:17:30</code>.</p>
    #[serde(rename = "preferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p>A Boolean value indicating whether the database is publicly accessible.</p>
    #[serde(rename = "publiclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    /// <p>The blueprint ID for the database. A blueprint describes the major engine version of a database.</p>
    #[serde(rename = "relationalDatabaseBlueprintId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_blueprint_id: Option<String>,
    /// <p>The bundle ID for the database. A bundle describes the performance specifications for your database.</p>
    #[serde(rename = "relationalDatabaseBundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_bundle_id: Option<String>,
    /// <p>The Lightsail resource type for the database (for example, <code>RelationalDatabase</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>Describes the secondary Availability Zone of a high availability database.</p> <p>The secondary database is used for failover support of a high availability database.</p>
    #[serde(rename = "secondaryAvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_availability_zone: Option<String>,
    /// <p>Describes the current state of the database.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The support code for the database. Include this code in your email to support when you have questions about a database in Lightsail. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
    /// <p>The tag keys and optional values for the resource. For more information about tags in Lightsail, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Describes a database image, or blueprint. A blueprint describes the major engine version of a database.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RelationalDatabaseBlueprint {
    /// <p>The ID for the database blueprint.</p>
    #[serde(rename = "blueprintId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_id: Option<String>,
    /// <p>The database software of the database blueprint (for example, <code>MySQL</code>).</p>
    #[serde(rename = "engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// <p>The description of the database engine for the database blueprint.</p>
    #[serde(rename = "engineDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_description: Option<String>,
    /// <p>The database engine version for the database blueprint (for example, <code>5.7.23</code>).</p>
    #[serde(rename = "engineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>The description of the database engine version for the database blueprint.</p>
    #[serde(rename = "engineVersionDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version_description: Option<String>,
    /// <p>A Boolean value indicating whether the engine version is the default for the database blueprint.</p>
    #[serde(rename = "isEngineDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_engine_default: Option<bool>,
}

/// <p>Describes a database bundle. A bundle describes the performance specifications of the database.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RelationalDatabaseBundle {
    /// <p>The ID for the database bundle.</p>
    #[serde(rename = "bundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    /// <p>The number of virtual CPUs (vCPUs) for the database bundle.</p>
    #[serde(rename = "cpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i64>,
    /// <p>The size of the disk for the database bundle.</p>
    #[serde(rename = "diskSizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size_in_gb: Option<i64>,
    /// <p>A Boolean value indicating whether the database bundle is active.</p>
    #[serde(rename = "isActive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    /// <p>A Boolean value indicating whether the database bundle is encrypted.</p>
    #[serde(rename = "isEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_encrypted: Option<bool>,
    /// <p>The name for the database bundle.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The cost of the database bundle in US currency.</p>
    #[serde(rename = "price")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f32>,
    /// <p>The amount of RAM in GB (for example, <code>2.0</code>) for the database bundle.</p>
    #[serde(rename = "ramSizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_size_in_gb: Option<f32>,
    /// <p>The data transfer rate per month in GB for the database bundle.</p>
    #[serde(rename = "transferPerMonthInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_per_month_in_gb: Option<i64>,
}

/// <p>Describes an endpoint for a database.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RelationalDatabaseEndpoint {
    /// <p>Specifies the DNS address of the database.</p>
    #[serde(rename = "address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// <p>Specifies the port that the database is listening on.</p>
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
}

/// <p>Describes an event for a database.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RelationalDatabaseEvent {
    /// <p>The timestamp when the database event was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The category that the database event belongs to.</p>
    #[serde(rename = "eventCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<Vec<String>>,
    /// <p>The message of the database event.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The database that the database event relates to.</p>
    #[serde(rename = "resource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
}

/// <p>Describes the hardware of a database.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RelationalDatabaseHardware {
    /// <p>The number of vCPUs for the database.</p>
    #[serde(rename = "cpuCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_count: Option<i64>,
    /// <p>The size of the disk for the database.</p>
    #[serde(rename = "diskSizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_size_in_gb: Option<i64>,
    /// <p>The amount of RAM in GB for the database.</p>
    #[serde(rename = "ramSizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ram_size_in_gb: Option<f32>,
}

/// <p>Describes the parameters of a database.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationalDatabaseParameter {
    /// <p>Specifies the valid range of values for the parameter.</p>
    #[serde(rename = "allowedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<String>,
    /// <p>Indicates when parameter updates are applied.</p> <p>Can be <code>immediate</code> or <code>pending-reboot</code>.</p>
    #[serde(rename = "applyMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_method: Option<String>,
    /// <p>Specifies the engine-specific parameter type.</p>
    #[serde(rename = "applyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_type: Option<String>,
    /// <p>Specifies the valid data type for the parameter.</p>
    #[serde(rename = "dataType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    /// <p>Provides a description of the parameter.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>A Boolean value indicating whether the parameter can be modified.</p>
    #[serde(rename = "isModifiable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_modifiable: Option<bool>,
    /// <p>Specifies the name of the parameter.</p>
    #[serde(rename = "parameterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    /// <p>Specifies the value of the parameter.</p>
    #[serde(rename = "parameterValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<String>,
}

/// <p>Describes a database snapshot.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RelationalDatabaseSnapshot {
    /// <p>The Amazon Resource Name (ARN) of the database snapshot.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The timestamp when the database snapshot was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The software of the database snapshot (for example, <code>MySQL</code>)</p>
    #[serde(rename = "engine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    /// <p>The database engine version for the database snapshot (for example, <code>5.7.23</code>).</p>
    #[serde(rename = "engineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the database from which the database snapshot was created.</p>
    #[serde(rename = "fromRelationalDatabaseArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_relational_database_arn: Option<String>,
    /// <p>The blueprint ID of the database from which the database snapshot was created. A blueprint describes the major engine version of a database.</p>
    #[serde(rename = "fromRelationalDatabaseBlueprintId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_relational_database_blueprint_id: Option<String>,
    /// <p>The bundle ID of the database from which the database snapshot was created.</p>
    #[serde(rename = "fromRelationalDatabaseBundleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_relational_database_bundle_id: Option<String>,
    /// <p>The name of the source database from which the database snapshot was created.</p>
    #[serde(rename = "fromRelationalDatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_relational_database_name: Option<String>,
    /// <p>The Region name and Availability Zone where the database snapshot is located.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The name of the database snapshot.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Lightsail resource type.</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The size of the disk in GB (for example, <code>32</code>) for the database snapshot.</p>
    #[serde(rename = "sizeInGb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_gb: Option<i64>,
    /// <p>The state of the database snapshot.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The support code for the database snapshot. Include this code in your email to support when you have questions about a database snapshot in Lightsail. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
    /// <p>The tag keys and optional values for the resource. For more information about tags in Lightsail, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ReleaseStaticIpRequest {
    /// <p>The name of the static IP to delete.</p>
    #[serde(rename = "staticIpName")]
    pub static_ip_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ReleaseStaticIpResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes the resource location.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResourceLocation {
    /// <p>The Availability Zone. Follows the format <code>us-east-2a</code> (case-sensitive).</p>
    #[serde(rename = "availabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The AWS Region name.</p>
    #[serde(rename = "regionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartInstanceRequest {
    /// <p>The name of the instance (a virtual private server) to start.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartInstanceResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartRelationalDatabaseRequest {
    /// <p>The name of your database to start.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartRelationalDatabaseResult {
    /// <p>An object describing the result of your start relational database request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes the static IP.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StaticIp {
    /// <p>The Amazon Resource Name (ARN) of the static IP (e.g., <code>arn:aws:lightsail:us-east-2:123456789101:StaticIp/9cbb4a9e-f8e3-4dfe-b57e-12345EXAMPLE</code>).</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The instance where the static IP is attached (e.g., <code>Amazon_Linux-1GB-Ohio-1</code>).</p>
    #[serde(rename = "attachedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_to: Option<String>,
    /// <p>The timestamp when the static IP was created (e.g., <code>1479735304.222</code>).</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The static IP address.</p>
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>A Boolean value indicating whether the static IP is attached.</p>
    #[serde(rename = "isAttached")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_attached: Option<bool>,
    /// <p>The region and Availability Zone where the static IP was created.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ResourceLocation>,
    /// <p>The name of the static IP (e.g., <code>StaticIP-Ohio-EXAMPLE</code>).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The resource type (usually <code>StaticIp</code>).</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>The support code. Include this code in your email to support when you have questions about an instance or another resource in Lightsail. This code enables our support team to look up your Lightsail information more easily.</p>
    #[serde(rename = "supportCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_code: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopInstanceRequest {
    /// <p><p>When set to <code>True</code>, forces a Lightsail instance that is stuck in a <code>stopping</code> state to stop.</p> <important> <p>Only use the <code>force</code> parameter if your instance is stuck in the <code>stopping</code> state. In any other state, your instance should stop normally without adding this parameter to your API request.</p> </important></p>
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// <p>The name of the instance (a virtual private server) to stop.</p>
    #[serde(rename = "instanceName")]
    pub instance_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StopInstanceResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopRelationalDatabaseRequest {
    /// <p>The name of your database to stop.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
    /// <p>The name of your new database snapshot to be created before stopping your database.</p>
    #[serde(rename = "relationalDatabaseSnapshotName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_snapshot_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StopRelationalDatabaseResult {
    /// <p>An object describing the result of your stop relational database request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// <p>Describes a tag key and optional value assigned to an Amazon Lightsail resource.</p> <p>For more information about tags in Lightsail, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key of the tag.</p> <p>Constraints: Tag keys accept a maximum of 128 letters, numbers, spaces in UTF-8, or the following characters: + - = . _ : / @</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value of the tag.</p> <p>Constraints: Tag values accept a maximum of 256 letters, numbers, spaces in UTF-8, or the following characters: + - = . _ : / @</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The name of the resource to which you are adding tags.</p>
    #[serde(rename = "resourceName")]
    pub resource_name: String,
    /// <p>The tag key and optional value.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TagResourceResult {
    /// <p>A list of objects describing the API operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UnpeerVpcRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UnpeerVpcResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The name of the resource from which you are removing a tag.</p>
    #[serde(rename = "resourceName")]
    pub resource_name: String,
    /// <p>The tag keys to delete from the specified resource.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UntagResourceResult {
    /// <p>A list of objects describing the API operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDomainEntryRequest {
    /// <p>An array of key-value pairs containing information about the domain entry.</p>
    #[serde(rename = "domainEntry")]
    pub domain_entry: DomainEntry,
    /// <p>The name of the domain recordset to update.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateDomainEntryResult {
    /// <p>An array of key-value pairs containing information about the request operation.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateLoadBalancerAttributeRequest {
    /// <p>The name of the attribute you want to update. Valid values are below.</p>
    #[serde(rename = "attributeName")]
    pub attribute_name: String,
    /// <p>The value that you want to specify for the attribute name.</p>
    #[serde(rename = "attributeValue")]
    pub attribute_value: String,
    /// <p>The name of the load balancer that you want to modify (e.g., <code>my-load-balancer</code>.</p>
    #[serde(rename = "loadBalancerName")]
    pub load_balancer_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateLoadBalancerAttributeResult {
    /// <p>An object describing the API operations.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateRelationalDatabaseParametersRequest {
    /// <p>The database parameters to update.</p>
    #[serde(rename = "parameters")]
    pub parameters: Vec<RelationalDatabaseParameter>,
    /// <p>The name of your database for which to update parameters.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateRelationalDatabaseParametersResult {
    /// <p>An object describing the result of your update relational database parameters request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateRelationalDatabaseRequest {
    /// <p>When <code>true</code>, applies changes immediately. When <code>false</code>, applies changes during the preferred maintenance window. Some changes may cause an outage.</p> <p>Default: <code>false</code> </p>
    #[serde(rename = "applyImmediately")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_immediately: Option<bool>,
    /// <p>When <code>true</code>, disables automated backup retention for your database.</p> <p>Disabling backup retention deletes all automated database backups. Before disabling this, you may want to create a snapshot of your database using the <code>create relational database snapshot</code> operation.</p> <p>Updates are applied during the next maintenance window because this can result in an outage.</p>
    #[serde(rename = "disableBackupRetention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_backup_retention: Option<bool>,
    /// <p>When <code>true</code>, enables automated backup retention for your database.</p> <p>Updates are applied during the next maintenance window because this can result in an outage.</p>
    #[serde(rename = "enableBackupRetention")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_backup_retention: Option<bool>,
    /// <p>The password for the master user of your database. The password can include any printable ASCII character except "/", """, or "@".</p> <p>Constraints: Must contain 8 to 41 characters.</p>
    #[serde(rename = "masterUserPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    /// <p><p>The daily time range during which automated backups are created for your database if automated backups are enabled.</p> <p>Constraints:</p> <ul> <li> <p>Must be in the <code>hh24:mi-hh24:mi</code> format.</p> <p>Example: <code>16:00-16:30</code> </p> </li> <li> <p>Specified in Universal Coordinated Time (UTC).</p> </li> <li> <p>Must not conflict with the preferred maintenance window.</p> </li> <li> <p>Must be at least 30 minutes.</p> </li> </ul></p>
    #[serde(rename = "preferredBackupWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    /// <p><p>The weekly time range during which system maintenance can occur on your database.</p> <p>The default is a 30-minute window selected at random from an 8-hour block of time for each AWS Region, occurring on a random day of the week.</p> <p>Constraints:</p> <ul> <li> <p>Must be in the <code>ddd:hh24:mi-ddd:hh24:mi</code> format.</p> </li> <li> <p>Valid days: Mon, Tue, Wed, Thu, Fri, Sat, Sun.</p> </li> <li> <p>Must be at least 30 minutes.</p> </li> <li> <p>Specified in Universal Coordinated Time (UTC).</p> </li> <li> <p>Example: <code>Tue:17:00-Tue:17:30</code> </p> </li> </ul></p>
    #[serde(rename = "preferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p>Specifies the accessibility options for your database. A value of <code>true</code> specifies a database that is available to resources outside of your Lightsail account. A value of <code>false</code> specifies a database that is available only to your Lightsail resources in the same region as your database.</p>
    #[serde(rename = "publiclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    /// <p>The name of your database to update.</p>
    #[serde(rename = "relationalDatabaseName")]
    pub relational_database_name: String,
    /// <p>When <code>true</code>, the master user password is changed to a new strong password generated by Lightsail.</p> <p>Use the <code>get relational database master user password</code> operation to get the new password.</p>
    #[serde(rename = "rotateMasterUserPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_master_user_password: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateRelationalDatabaseResult {
    /// <p>An object describing the result of your update relational database request.</p>
    #[serde(rename = "operations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Vec<Operation>>,
}

/// Errors returned by AllocateStaticIp
#[derive(Debug, PartialEq)]
pub enum AllocateStaticIpError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl AllocateStaticIpError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AllocateStaticIpError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AllocateStaticIpError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(AllocateStaticIpError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AllocateStaticIpError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(AllocateStaticIpError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(AllocateStaticIpError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(AllocateStaticIpError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(AllocateStaticIpError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AllocateStaticIpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AllocateStaticIpError {
    fn description(&self) -> &str {
        match *self {
            AllocateStaticIpError::AccessDenied(ref cause) => cause,
            AllocateStaticIpError::AccountSetupInProgress(ref cause) => cause,
            AllocateStaticIpError::InvalidInput(ref cause) => cause,
            AllocateStaticIpError::NotFound(ref cause) => cause,
            AllocateStaticIpError::OperationFailure(ref cause) => cause,
            AllocateStaticIpError::Service(ref cause) => cause,
            AllocateStaticIpError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by AttachDisk
#[derive(Debug, PartialEq)]
pub enum AttachDiskError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl AttachDiskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AttachDiskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AttachDiskError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(AttachDiskError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AttachDiskError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(AttachDiskError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(AttachDiskError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(AttachDiskError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(AttachDiskError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AttachDiskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachDiskError {
    fn description(&self) -> &str {
        match *self {
            AttachDiskError::AccessDenied(ref cause) => cause,
            AttachDiskError::AccountSetupInProgress(ref cause) => cause,
            AttachDiskError::InvalidInput(ref cause) => cause,
            AttachDiskError::NotFound(ref cause) => cause,
            AttachDiskError::OperationFailure(ref cause) => cause,
            AttachDiskError::Service(ref cause) => cause,
            AttachDiskError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by AttachInstancesToLoadBalancer
#[derive(Debug, PartialEq)]
pub enum AttachInstancesToLoadBalancerError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl AttachInstancesToLoadBalancerError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AttachInstancesToLoadBalancerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AttachInstancesToLoadBalancerError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        AttachInstancesToLoadBalancerError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AttachInstancesToLoadBalancerError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(AttachInstancesToLoadBalancerError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        AttachInstancesToLoadBalancerError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(AttachInstancesToLoadBalancerError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        AttachInstancesToLoadBalancerError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AttachInstancesToLoadBalancerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachInstancesToLoadBalancerError {
    fn description(&self) -> &str {
        match *self {
            AttachInstancesToLoadBalancerError::AccessDenied(ref cause) => cause,
            AttachInstancesToLoadBalancerError::AccountSetupInProgress(ref cause) => cause,
            AttachInstancesToLoadBalancerError::InvalidInput(ref cause) => cause,
            AttachInstancesToLoadBalancerError::NotFound(ref cause) => cause,
            AttachInstancesToLoadBalancerError::OperationFailure(ref cause) => cause,
            AttachInstancesToLoadBalancerError::Service(ref cause) => cause,
            AttachInstancesToLoadBalancerError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by AttachLoadBalancerTlsCertificate
#[derive(Debug, PartialEq)]
pub enum AttachLoadBalancerTlsCertificateError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl AttachLoadBalancerTlsCertificateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AttachLoadBalancerTlsCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        AttachLoadBalancerTlsCertificateError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        AttachLoadBalancerTlsCertificateError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        AttachLoadBalancerTlsCertificateError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(AttachLoadBalancerTlsCertificateError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        AttachLoadBalancerTlsCertificateError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(AttachLoadBalancerTlsCertificateError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        AttachLoadBalancerTlsCertificateError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AttachLoadBalancerTlsCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachLoadBalancerTlsCertificateError {
    fn description(&self) -> &str {
        match *self {
            AttachLoadBalancerTlsCertificateError::AccessDenied(ref cause) => cause,
            AttachLoadBalancerTlsCertificateError::AccountSetupInProgress(ref cause) => cause,
            AttachLoadBalancerTlsCertificateError::InvalidInput(ref cause) => cause,
            AttachLoadBalancerTlsCertificateError::NotFound(ref cause) => cause,
            AttachLoadBalancerTlsCertificateError::OperationFailure(ref cause) => cause,
            AttachLoadBalancerTlsCertificateError::Service(ref cause) => cause,
            AttachLoadBalancerTlsCertificateError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by AttachStaticIp
#[derive(Debug, PartialEq)]
pub enum AttachStaticIpError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl AttachStaticIpError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AttachStaticIpError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AttachStaticIpError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(AttachStaticIpError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AttachStaticIpError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(AttachStaticIpError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(AttachStaticIpError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(AttachStaticIpError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(AttachStaticIpError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AttachStaticIpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AttachStaticIpError {
    fn description(&self) -> &str {
        match *self {
            AttachStaticIpError::AccessDenied(ref cause) => cause,
            AttachStaticIpError::AccountSetupInProgress(ref cause) => cause,
            AttachStaticIpError::InvalidInput(ref cause) => cause,
            AttachStaticIpError::NotFound(ref cause) => cause,
            AttachStaticIpError::OperationFailure(ref cause) => cause,
            AttachStaticIpError::Service(ref cause) => cause,
            AttachStaticIpError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by CloseInstancePublicPorts
#[derive(Debug, PartialEq)]
pub enum CloseInstancePublicPortsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CloseInstancePublicPortsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CloseInstancePublicPortsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CloseInstancePublicPortsError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        CloseInstancePublicPortsError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CloseInstancePublicPortsError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CloseInstancePublicPortsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CloseInstancePublicPortsError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(CloseInstancePublicPortsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CloseInstancePublicPortsError::Unauthenticated(
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
impl fmt::Display for CloseInstancePublicPortsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CloseInstancePublicPortsError {
    fn description(&self) -> &str {
        match *self {
            CloseInstancePublicPortsError::AccessDenied(ref cause) => cause,
            CloseInstancePublicPortsError::AccountSetupInProgress(ref cause) => cause,
            CloseInstancePublicPortsError::InvalidInput(ref cause) => cause,
            CloseInstancePublicPortsError::NotFound(ref cause) => cause,
            CloseInstancePublicPortsError::OperationFailure(ref cause) => cause,
            CloseInstancePublicPortsError::Service(ref cause) => cause,
            CloseInstancePublicPortsError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by CopySnapshot
#[derive(Debug, PartialEq)]
pub enum CopySnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CopySnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CopySnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CopySnapshotError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(CopySnapshotError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CopySnapshotError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CopySnapshotError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CopySnapshotError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CopySnapshotError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CopySnapshotError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CopySnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CopySnapshotError {
    fn description(&self) -> &str {
        match *self {
            CopySnapshotError::AccessDenied(ref cause) => cause,
            CopySnapshotError::AccountSetupInProgress(ref cause) => cause,
            CopySnapshotError::InvalidInput(ref cause) => cause,
            CopySnapshotError::NotFound(ref cause) => cause,
            CopySnapshotError::OperationFailure(ref cause) => cause,
            CopySnapshotError::Service(ref cause) => cause,
            CopySnapshotError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCloudFormationStack
#[derive(Debug, PartialEq)]
pub enum CreateCloudFormationStackError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateCloudFormationStackError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCloudFormationStackError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateCloudFormationStackError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        CreateCloudFormationStackError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateCloudFormationStackError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateCloudFormationStackError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CreateCloudFormationStackError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateCloudFormationStackError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateCloudFormationStackError::Unauthenticated(
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
impl fmt::Display for CreateCloudFormationStackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCloudFormationStackError {
    fn description(&self) -> &str {
        match *self {
            CreateCloudFormationStackError::AccessDenied(ref cause) => cause,
            CreateCloudFormationStackError::AccountSetupInProgress(ref cause) => cause,
            CreateCloudFormationStackError::InvalidInput(ref cause) => cause,
            CreateCloudFormationStackError::NotFound(ref cause) => cause,
            CreateCloudFormationStackError::OperationFailure(ref cause) => cause,
            CreateCloudFormationStackError::Service(ref cause) => cause,
            CreateCloudFormationStackError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDisk
#[derive(Debug, PartialEq)]
pub enum CreateDiskError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateDiskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDiskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateDiskError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(CreateDiskError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateDiskError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateDiskError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CreateDiskError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateDiskError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateDiskError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateDiskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDiskError {
    fn description(&self) -> &str {
        match *self {
            CreateDiskError::AccessDenied(ref cause) => cause,
            CreateDiskError::AccountSetupInProgress(ref cause) => cause,
            CreateDiskError::InvalidInput(ref cause) => cause,
            CreateDiskError::NotFound(ref cause) => cause,
            CreateDiskError::OperationFailure(ref cause) => cause,
            CreateDiskError::Service(ref cause) => cause,
            CreateDiskError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDiskFromSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateDiskFromSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateDiskFromSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDiskFromSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateDiskFromSnapshotError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        CreateDiskFromSnapshotError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateDiskFromSnapshotError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateDiskFromSnapshotError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CreateDiskFromSnapshotError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateDiskFromSnapshotError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateDiskFromSnapshotError::Unauthenticated(
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
impl fmt::Display for CreateDiskFromSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDiskFromSnapshotError {
    fn description(&self) -> &str {
        match *self {
            CreateDiskFromSnapshotError::AccessDenied(ref cause) => cause,
            CreateDiskFromSnapshotError::AccountSetupInProgress(ref cause) => cause,
            CreateDiskFromSnapshotError::InvalidInput(ref cause) => cause,
            CreateDiskFromSnapshotError::NotFound(ref cause) => cause,
            CreateDiskFromSnapshotError::OperationFailure(ref cause) => cause,
            CreateDiskFromSnapshotError::Service(ref cause) => cause,
            CreateDiskFromSnapshotError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDiskSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateDiskSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateDiskSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDiskSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateDiskSnapshotError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(CreateDiskSnapshotError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateDiskSnapshotError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateDiskSnapshotError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CreateDiskSnapshotError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateDiskSnapshotError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateDiskSnapshotError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateDiskSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDiskSnapshotError {
    fn description(&self) -> &str {
        match *self {
            CreateDiskSnapshotError::AccessDenied(ref cause) => cause,
            CreateDiskSnapshotError::AccountSetupInProgress(ref cause) => cause,
            CreateDiskSnapshotError::InvalidInput(ref cause) => cause,
            CreateDiskSnapshotError::NotFound(ref cause) => cause,
            CreateDiskSnapshotError::OperationFailure(ref cause) => cause,
            CreateDiskSnapshotError::Service(ref cause) => cause,
            CreateDiskSnapshotError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDomain
#[derive(Debug, PartialEq)]
pub enum CreateDomainError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDomainError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateDomainError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(CreateDomainError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateDomainError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateDomainError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CreateDomainError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateDomainError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateDomainError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDomainError {
    fn description(&self) -> &str {
        match *self {
            CreateDomainError::AccessDenied(ref cause) => cause,
            CreateDomainError::AccountSetupInProgress(ref cause) => cause,
            CreateDomainError::InvalidInput(ref cause) => cause,
            CreateDomainError::NotFound(ref cause) => cause,
            CreateDomainError::OperationFailure(ref cause) => cause,
            CreateDomainError::Service(ref cause) => cause,
            CreateDomainError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDomainEntry
#[derive(Debug, PartialEq)]
pub enum CreateDomainEntryError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateDomainEntryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDomainEntryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateDomainEntryError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(CreateDomainEntryError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateDomainEntryError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateDomainEntryError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CreateDomainEntryError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateDomainEntryError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateDomainEntryError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateDomainEntryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDomainEntryError {
    fn description(&self) -> &str {
        match *self {
            CreateDomainEntryError::AccessDenied(ref cause) => cause,
            CreateDomainEntryError::AccountSetupInProgress(ref cause) => cause,
            CreateDomainEntryError::InvalidInput(ref cause) => cause,
            CreateDomainEntryError::NotFound(ref cause) => cause,
            CreateDomainEntryError::OperationFailure(ref cause) => cause,
            CreateDomainEntryError::Service(ref cause) => cause,
            CreateDomainEntryError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateInstanceSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateInstanceSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateInstanceSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateInstanceSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateInstanceSnapshotError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        CreateInstanceSnapshotError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateInstanceSnapshotError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateInstanceSnapshotError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CreateInstanceSnapshotError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateInstanceSnapshotError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateInstanceSnapshotError::Unauthenticated(
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
impl fmt::Display for CreateInstanceSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateInstanceSnapshotError {
    fn description(&self) -> &str {
        match *self {
            CreateInstanceSnapshotError::AccessDenied(ref cause) => cause,
            CreateInstanceSnapshotError::AccountSetupInProgress(ref cause) => cause,
            CreateInstanceSnapshotError::InvalidInput(ref cause) => cause,
            CreateInstanceSnapshotError::NotFound(ref cause) => cause,
            CreateInstanceSnapshotError::OperationFailure(ref cause) => cause,
            CreateInstanceSnapshotError::Service(ref cause) => cause,
            CreateInstanceSnapshotError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateInstances
#[derive(Debug, PartialEq)]
pub enum CreateInstancesError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateInstancesError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(CreateInstancesError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateInstancesError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateInstancesError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CreateInstancesError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateInstancesError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateInstancesError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateInstancesError {
    fn description(&self) -> &str {
        match *self {
            CreateInstancesError::AccessDenied(ref cause) => cause,
            CreateInstancesError::AccountSetupInProgress(ref cause) => cause,
            CreateInstancesError::InvalidInput(ref cause) => cause,
            CreateInstancesError::NotFound(ref cause) => cause,
            CreateInstancesError::OperationFailure(ref cause) => cause,
            CreateInstancesError::Service(ref cause) => cause,
            CreateInstancesError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateInstancesFromSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateInstancesFromSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateInstancesFromSnapshotError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateInstancesFromSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateInstancesFromSnapshotError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        CreateInstancesFromSnapshotError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateInstancesFromSnapshotError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateInstancesFromSnapshotError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        CreateInstancesFromSnapshotError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateInstancesFromSnapshotError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateInstancesFromSnapshotError::Unauthenticated(
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
impl fmt::Display for CreateInstancesFromSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateInstancesFromSnapshotError {
    fn description(&self) -> &str {
        match *self {
            CreateInstancesFromSnapshotError::AccessDenied(ref cause) => cause,
            CreateInstancesFromSnapshotError::AccountSetupInProgress(ref cause) => cause,
            CreateInstancesFromSnapshotError::InvalidInput(ref cause) => cause,
            CreateInstancesFromSnapshotError::NotFound(ref cause) => cause,
            CreateInstancesFromSnapshotError::OperationFailure(ref cause) => cause,
            CreateInstancesFromSnapshotError::Service(ref cause) => cause,
            CreateInstancesFromSnapshotError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateKeyPair
#[derive(Debug, PartialEq)]
pub enum CreateKeyPairError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateKeyPairError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateKeyPairError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateKeyPairError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(CreateKeyPairError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateKeyPairError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateKeyPairError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CreateKeyPairError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateKeyPairError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateKeyPairError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateKeyPairError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateKeyPairError {
    fn description(&self) -> &str {
        match *self {
            CreateKeyPairError::AccessDenied(ref cause) => cause,
            CreateKeyPairError::AccountSetupInProgress(ref cause) => cause,
            CreateKeyPairError::InvalidInput(ref cause) => cause,
            CreateKeyPairError::NotFound(ref cause) => cause,
            CreateKeyPairError::OperationFailure(ref cause) => cause,
            CreateKeyPairError::Service(ref cause) => cause,
            CreateKeyPairError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateLoadBalancer
#[derive(Debug, PartialEq)]
pub enum CreateLoadBalancerError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateLoadBalancerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLoadBalancerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateLoadBalancerError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(CreateLoadBalancerError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateLoadBalancerError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateLoadBalancerError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CreateLoadBalancerError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateLoadBalancerError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateLoadBalancerError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateLoadBalancerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLoadBalancerError {
    fn description(&self) -> &str {
        match *self {
            CreateLoadBalancerError::AccessDenied(ref cause) => cause,
            CreateLoadBalancerError::AccountSetupInProgress(ref cause) => cause,
            CreateLoadBalancerError::InvalidInput(ref cause) => cause,
            CreateLoadBalancerError::NotFound(ref cause) => cause,
            CreateLoadBalancerError::OperationFailure(ref cause) => cause,
            CreateLoadBalancerError::Service(ref cause) => cause,
            CreateLoadBalancerError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateLoadBalancerTlsCertificate
#[derive(Debug, PartialEq)]
pub enum CreateLoadBalancerTlsCertificateError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateLoadBalancerTlsCertificateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateLoadBalancerTlsCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        CreateLoadBalancerTlsCertificateError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        CreateLoadBalancerTlsCertificateError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        CreateLoadBalancerTlsCertificateError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateLoadBalancerTlsCertificateError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        CreateLoadBalancerTlsCertificateError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateLoadBalancerTlsCertificateError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        CreateLoadBalancerTlsCertificateError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateLoadBalancerTlsCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateLoadBalancerTlsCertificateError {
    fn description(&self) -> &str {
        match *self {
            CreateLoadBalancerTlsCertificateError::AccessDenied(ref cause) => cause,
            CreateLoadBalancerTlsCertificateError::AccountSetupInProgress(ref cause) => cause,
            CreateLoadBalancerTlsCertificateError::InvalidInput(ref cause) => cause,
            CreateLoadBalancerTlsCertificateError::NotFound(ref cause) => cause,
            CreateLoadBalancerTlsCertificateError::OperationFailure(ref cause) => cause,
            CreateLoadBalancerTlsCertificateError::Service(ref cause) => cause,
            CreateLoadBalancerTlsCertificateError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateRelationalDatabase
#[derive(Debug, PartialEq)]
pub enum CreateRelationalDatabaseError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateRelationalDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRelationalDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateRelationalDatabaseError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateRelationalDatabaseError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateRelationalDatabaseError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(CreateRelationalDatabaseError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateRelationalDatabaseError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(CreateRelationalDatabaseError::Unauthenticated(
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
impl fmt::Display for CreateRelationalDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRelationalDatabaseError {
    fn description(&self) -> &str {
        match *self {
            CreateRelationalDatabaseError::AccessDenied(ref cause) => cause,
            CreateRelationalDatabaseError::AccountSetupInProgress(ref cause) => cause,
            CreateRelationalDatabaseError::InvalidInput(ref cause) => cause,
            CreateRelationalDatabaseError::NotFound(ref cause) => cause,
            CreateRelationalDatabaseError::OperationFailure(ref cause) => cause,
            CreateRelationalDatabaseError::Service(ref cause) => cause,
            CreateRelationalDatabaseError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateRelationalDatabaseFromSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateRelationalDatabaseFromSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateRelationalDatabaseFromSnapshotError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateRelationalDatabaseFromSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseFromSnapshotError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseFromSnapshotError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseFromSnapshotError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseFromSnapshotError::NotFound(err.msg),
                    )
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseFromSnapshotError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseFromSnapshotError::Service(err.msg),
                    )
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseFromSnapshotError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateRelationalDatabaseFromSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRelationalDatabaseFromSnapshotError {
    fn description(&self) -> &str {
        match *self {
            CreateRelationalDatabaseFromSnapshotError::AccessDenied(ref cause) => cause,
            CreateRelationalDatabaseFromSnapshotError::AccountSetupInProgress(ref cause) => cause,
            CreateRelationalDatabaseFromSnapshotError::InvalidInput(ref cause) => cause,
            CreateRelationalDatabaseFromSnapshotError::NotFound(ref cause) => cause,
            CreateRelationalDatabaseFromSnapshotError::OperationFailure(ref cause) => cause,
            CreateRelationalDatabaseFromSnapshotError::Service(ref cause) => cause,
            CreateRelationalDatabaseFromSnapshotError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateRelationalDatabaseSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateRelationalDatabaseSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl CreateRelationalDatabaseSnapshotError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateRelationalDatabaseSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseSnapshotError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseSnapshotError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseSnapshotError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateRelationalDatabaseSnapshotError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseSnapshotError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateRelationalDatabaseSnapshotError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        CreateRelationalDatabaseSnapshotError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateRelationalDatabaseSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRelationalDatabaseSnapshotError {
    fn description(&self) -> &str {
        match *self {
            CreateRelationalDatabaseSnapshotError::AccessDenied(ref cause) => cause,
            CreateRelationalDatabaseSnapshotError::AccountSetupInProgress(ref cause) => cause,
            CreateRelationalDatabaseSnapshotError::InvalidInput(ref cause) => cause,
            CreateRelationalDatabaseSnapshotError::NotFound(ref cause) => cause,
            CreateRelationalDatabaseSnapshotError::OperationFailure(ref cause) => cause,
            CreateRelationalDatabaseSnapshotError::Service(ref cause) => cause,
            CreateRelationalDatabaseSnapshotError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDisk
#[derive(Debug, PartialEq)]
pub enum DeleteDiskError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteDiskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDiskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteDiskError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(DeleteDiskError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteDiskError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteDiskError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DeleteDiskError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteDiskError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DeleteDiskError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteDiskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDiskError {
    fn description(&self) -> &str {
        match *self {
            DeleteDiskError::AccessDenied(ref cause) => cause,
            DeleteDiskError::AccountSetupInProgress(ref cause) => cause,
            DeleteDiskError::InvalidInput(ref cause) => cause,
            DeleteDiskError::NotFound(ref cause) => cause,
            DeleteDiskError::OperationFailure(ref cause) => cause,
            DeleteDiskError::Service(ref cause) => cause,
            DeleteDiskError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDiskSnapshot
#[derive(Debug, PartialEq)]
pub enum DeleteDiskSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteDiskSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDiskSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteDiskSnapshotError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(DeleteDiskSnapshotError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteDiskSnapshotError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteDiskSnapshotError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DeleteDiskSnapshotError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteDiskSnapshotError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DeleteDiskSnapshotError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteDiskSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDiskSnapshotError {
    fn description(&self) -> &str {
        match *self {
            DeleteDiskSnapshotError::AccessDenied(ref cause) => cause,
            DeleteDiskSnapshotError::AccountSetupInProgress(ref cause) => cause,
            DeleteDiskSnapshotError::InvalidInput(ref cause) => cause,
            DeleteDiskSnapshotError::NotFound(ref cause) => cause,
            DeleteDiskSnapshotError::OperationFailure(ref cause) => cause,
            DeleteDiskSnapshotError::Service(ref cause) => cause,
            DeleteDiskSnapshotError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDomain
#[derive(Debug, PartialEq)]
pub enum DeleteDomainError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDomainError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteDomainError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(DeleteDomainError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteDomainError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteDomainError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DeleteDomainError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteDomainError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DeleteDomainError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDomainError {
    fn description(&self) -> &str {
        match *self {
            DeleteDomainError::AccessDenied(ref cause) => cause,
            DeleteDomainError::AccountSetupInProgress(ref cause) => cause,
            DeleteDomainError::InvalidInput(ref cause) => cause,
            DeleteDomainError::NotFound(ref cause) => cause,
            DeleteDomainError::OperationFailure(ref cause) => cause,
            DeleteDomainError::Service(ref cause) => cause,
            DeleteDomainError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDomainEntry
#[derive(Debug, PartialEq)]
pub enum DeleteDomainEntryError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteDomainEntryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDomainEntryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteDomainEntryError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(DeleteDomainEntryError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteDomainEntryError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteDomainEntryError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DeleteDomainEntryError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteDomainEntryError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DeleteDomainEntryError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteDomainEntryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDomainEntryError {
    fn description(&self) -> &str {
        match *self {
            DeleteDomainEntryError::AccessDenied(ref cause) => cause,
            DeleteDomainEntryError::AccountSetupInProgress(ref cause) => cause,
            DeleteDomainEntryError::InvalidInput(ref cause) => cause,
            DeleteDomainEntryError::NotFound(ref cause) => cause,
            DeleteDomainEntryError::OperationFailure(ref cause) => cause,
            DeleteDomainEntryError::Service(ref cause) => cause,
            DeleteDomainEntryError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteInstance
#[derive(Debug, PartialEq)]
pub enum DeleteInstanceError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteInstanceError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(DeleteInstanceError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteInstanceError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteInstanceError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DeleteInstanceError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteInstanceError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DeleteInstanceError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteInstanceError {
    fn description(&self) -> &str {
        match *self {
            DeleteInstanceError::AccessDenied(ref cause) => cause,
            DeleteInstanceError::AccountSetupInProgress(ref cause) => cause,
            DeleteInstanceError::InvalidInput(ref cause) => cause,
            DeleteInstanceError::NotFound(ref cause) => cause,
            DeleteInstanceError::OperationFailure(ref cause) => cause,
            DeleteInstanceError::Service(ref cause) => cause,
            DeleteInstanceError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteInstanceSnapshot
#[derive(Debug, PartialEq)]
pub enum DeleteInstanceSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteInstanceSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteInstanceSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteInstanceSnapshotError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        DeleteInstanceSnapshotError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteInstanceSnapshotError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteInstanceSnapshotError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DeleteInstanceSnapshotError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteInstanceSnapshotError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DeleteInstanceSnapshotError::Unauthenticated(
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
impl fmt::Display for DeleteInstanceSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteInstanceSnapshotError {
    fn description(&self) -> &str {
        match *self {
            DeleteInstanceSnapshotError::AccessDenied(ref cause) => cause,
            DeleteInstanceSnapshotError::AccountSetupInProgress(ref cause) => cause,
            DeleteInstanceSnapshotError::InvalidInput(ref cause) => cause,
            DeleteInstanceSnapshotError::NotFound(ref cause) => cause,
            DeleteInstanceSnapshotError::OperationFailure(ref cause) => cause,
            DeleteInstanceSnapshotError::Service(ref cause) => cause,
            DeleteInstanceSnapshotError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteKeyPair
#[derive(Debug, PartialEq)]
pub enum DeleteKeyPairError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteKeyPairError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteKeyPairError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteKeyPairError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(DeleteKeyPairError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteKeyPairError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteKeyPairError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DeleteKeyPairError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteKeyPairError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DeleteKeyPairError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteKeyPairError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteKeyPairError {
    fn description(&self) -> &str {
        match *self {
            DeleteKeyPairError::AccessDenied(ref cause) => cause,
            DeleteKeyPairError::AccountSetupInProgress(ref cause) => cause,
            DeleteKeyPairError::InvalidInput(ref cause) => cause,
            DeleteKeyPairError::NotFound(ref cause) => cause,
            DeleteKeyPairError::OperationFailure(ref cause) => cause,
            DeleteKeyPairError::Service(ref cause) => cause,
            DeleteKeyPairError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteKnownHostKeys
#[derive(Debug, PartialEq)]
pub enum DeleteKnownHostKeysError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteKnownHostKeysError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteKnownHostKeysError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteKnownHostKeysError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(DeleteKnownHostKeysError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteKnownHostKeysError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteKnownHostKeysError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DeleteKnownHostKeysError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteKnownHostKeysError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DeleteKnownHostKeysError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteKnownHostKeysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteKnownHostKeysError {
    fn description(&self) -> &str {
        match *self {
            DeleteKnownHostKeysError::AccessDenied(ref cause) => cause,
            DeleteKnownHostKeysError::AccountSetupInProgress(ref cause) => cause,
            DeleteKnownHostKeysError::InvalidInput(ref cause) => cause,
            DeleteKnownHostKeysError::NotFound(ref cause) => cause,
            DeleteKnownHostKeysError::OperationFailure(ref cause) => cause,
            DeleteKnownHostKeysError::Service(ref cause) => cause,
            DeleteKnownHostKeysError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteLoadBalancer
#[derive(Debug, PartialEq)]
pub enum DeleteLoadBalancerError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteLoadBalancerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLoadBalancerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteLoadBalancerError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(DeleteLoadBalancerError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteLoadBalancerError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteLoadBalancerError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DeleteLoadBalancerError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteLoadBalancerError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DeleteLoadBalancerError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteLoadBalancerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLoadBalancerError {
    fn description(&self) -> &str {
        match *self {
            DeleteLoadBalancerError::AccessDenied(ref cause) => cause,
            DeleteLoadBalancerError::AccountSetupInProgress(ref cause) => cause,
            DeleteLoadBalancerError::InvalidInput(ref cause) => cause,
            DeleteLoadBalancerError::NotFound(ref cause) => cause,
            DeleteLoadBalancerError::OperationFailure(ref cause) => cause,
            DeleteLoadBalancerError::Service(ref cause) => cause,
            DeleteLoadBalancerError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteLoadBalancerTlsCertificate
#[derive(Debug, PartialEq)]
pub enum DeleteLoadBalancerTlsCertificateError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteLoadBalancerTlsCertificateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteLoadBalancerTlsCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        DeleteLoadBalancerTlsCertificateError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        DeleteLoadBalancerTlsCertificateError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        DeleteLoadBalancerTlsCertificateError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteLoadBalancerTlsCertificateError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        DeleteLoadBalancerTlsCertificateError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteLoadBalancerTlsCertificateError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        DeleteLoadBalancerTlsCertificateError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteLoadBalancerTlsCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLoadBalancerTlsCertificateError {
    fn description(&self) -> &str {
        match *self {
            DeleteLoadBalancerTlsCertificateError::AccessDenied(ref cause) => cause,
            DeleteLoadBalancerTlsCertificateError::AccountSetupInProgress(ref cause) => cause,
            DeleteLoadBalancerTlsCertificateError::InvalidInput(ref cause) => cause,
            DeleteLoadBalancerTlsCertificateError::NotFound(ref cause) => cause,
            DeleteLoadBalancerTlsCertificateError::OperationFailure(ref cause) => cause,
            DeleteLoadBalancerTlsCertificateError::Service(ref cause) => cause,
            DeleteLoadBalancerTlsCertificateError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRelationalDatabase
#[derive(Debug, PartialEq)]
pub enum DeleteRelationalDatabaseError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteRelationalDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRelationalDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteRelationalDatabaseError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        DeleteRelationalDatabaseError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteRelationalDatabaseError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteRelationalDatabaseError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DeleteRelationalDatabaseError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteRelationalDatabaseError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DeleteRelationalDatabaseError::Unauthenticated(
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
impl fmt::Display for DeleteRelationalDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRelationalDatabaseError {
    fn description(&self) -> &str {
        match *self {
            DeleteRelationalDatabaseError::AccessDenied(ref cause) => cause,
            DeleteRelationalDatabaseError::AccountSetupInProgress(ref cause) => cause,
            DeleteRelationalDatabaseError::InvalidInput(ref cause) => cause,
            DeleteRelationalDatabaseError::NotFound(ref cause) => cause,
            DeleteRelationalDatabaseError::OperationFailure(ref cause) => cause,
            DeleteRelationalDatabaseError::Service(ref cause) => cause,
            DeleteRelationalDatabaseError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRelationalDatabaseSnapshot
#[derive(Debug, PartialEq)]
pub enum DeleteRelationalDatabaseSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DeleteRelationalDatabaseSnapshotError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteRelationalDatabaseSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        DeleteRelationalDatabaseSnapshotError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        DeleteRelationalDatabaseSnapshotError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        DeleteRelationalDatabaseSnapshotError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteRelationalDatabaseSnapshotError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        DeleteRelationalDatabaseSnapshotError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteRelationalDatabaseSnapshotError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        DeleteRelationalDatabaseSnapshotError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteRelationalDatabaseSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRelationalDatabaseSnapshotError {
    fn description(&self) -> &str {
        match *self {
            DeleteRelationalDatabaseSnapshotError::AccessDenied(ref cause) => cause,
            DeleteRelationalDatabaseSnapshotError::AccountSetupInProgress(ref cause) => cause,
            DeleteRelationalDatabaseSnapshotError::InvalidInput(ref cause) => cause,
            DeleteRelationalDatabaseSnapshotError::NotFound(ref cause) => cause,
            DeleteRelationalDatabaseSnapshotError::OperationFailure(ref cause) => cause,
            DeleteRelationalDatabaseSnapshotError::Service(ref cause) => cause,
            DeleteRelationalDatabaseSnapshotError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by DetachDisk
#[derive(Debug, PartialEq)]
pub enum DetachDiskError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DetachDiskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetachDiskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DetachDiskError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(DetachDiskError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DetachDiskError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DetachDiskError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DetachDiskError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DetachDiskError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DetachDiskError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetachDiskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachDiskError {
    fn description(&self) -> &str {
        match *self {
            DetachDiskError::AccessDenied(ref cause) => cause,
            DetachDiskError::AccountSetupInProgress(ref cause) => cause,
            DetachDiskError::InvalidInput(ref cause) => cause,
            DetachDiskError::NotFound(ref cause) => cause,
            DetachDiskError::OperationFailure(ref cause) => cause,
            DetachDiskError::Service(ref cause) => cause,
            DetachDiskError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by DetachInstancesFromLoadBalancer
#[derive(Debug, PartialEq)]
pub enum DetachInstancesFromLoadBalancerError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DetachInstancesFromLoadBalancerError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DetachInstancesFromLoadBalancerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        DetachInstancesFromLoadBalancerError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        DetachInstancesFromLoadBalancerError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        DetachInstancesFromLoadBalancerError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DetachInstancesFromLoadBalancerError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        DetachInstancesFromLoadBalancerError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(DetachInstancesFromLoadBalancerError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        DetachInstancesFromLoadBalancerError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetachInstancesFromLoadBalancerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachInstancesFromLoadBalancerError {
    fn description(&self) -> &str {
        match *self {
            DetachInstancesFromLoadBalancerError::AccessDenied(ref cause) => cause,
            DetachInstancesFromLoadBalancerError::AccountSetupInProgress(ref cause) => cause,
            DetachInstancesFromLoadBalancerError::InvalidInput(ref cause) => cause,
            DetachInstancesFromLoadBalancerError::NotFound(ref cause) => cause,
            DetachInstancesFromLoadBalancerError::OperationFailure(ref cause) => cause,
            DetachInstancesFromLoadBalancerError::Service(ref cause) => cause,
            DetachInstancesFromLoadBalancerError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by DetachStaticIp
#[derive(Debug, PartialEq)]
pub enum DetachStaticIpError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DetachStaticIpError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetachStaticIpError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DetachStaticIpError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(DetachStaticIpError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DetachStaticIpError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DetachStaticIpError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DetachStaticIpError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DetachStaticIpError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DetachStaticIpError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetachStaticIpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetachStaticIpError {
    fn description(&self) -> &str {
        match *self {
            DetachStaticIpError::AccessDenied(ref cause) => cause,
            DetachStaticIpError::AccountSetupInProgress(ref cause) => cause,
            DetachStaticIpError::InvalidInput(ref cause) => cause,
            DetachStaticIpError::NotFound(ref cause) => cause,
            DetachStaticIpError::OperationFailure(ref cause) => cause,
            DetachStaticIpError::Service(ref cause) => cause,
            DetachStaticIpError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by DownloadDefaultKeyPair
#[derive(Debug, PartialEq)]
pub enum DownloadDefaultKeyPairError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl DownloadDefaultKeyPairError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DownloadDefaultKeyPairError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DownloadDefaultKeyPairError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        DownloadDefaultKeyPairError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DownloadDefaultKeyPairError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DownloadDefaultKeyPairError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(DownloadDefaultKeyPairError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DownloadDefaultKeyPairError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(DownloadDefaultKeyPairError::Unauthenticated(
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
impl fmt::Display for DownloadDefaultKeyPairError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DownloadDefaultKeyPairError {
    fn description(&self) -> &str {
        match *self {
            DownloadDefaultKeyPairError::AccessDenied(ref cause) => cause,
            DownloadDefaultKeyPairError::AccountSetupInProgress(ref cause) => cause,
            DownloadDefaultKeyPairError::InvalidInput(ref cause) => cause,
            DownloadDefaultKeyPairError::NotFound(ref cause) => cause,
            DownloadDefaultKeyPairError::OperationFailure(ref cause) => cause,
            DownloadDefaultKeyPairError::Service(ref cause) => cause,
            DownloadDefaultKeyPairError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by ExportSnapshot
#[derive(Debug, PartialEq)]
pub enum ExportSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl ExportSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ExportSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ExportSnapshotError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(ExportSnapshotError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ExportSnapshotError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ExportSnapshotError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(ExportSnapshotError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ExportSnapshotError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(ExportSnapshotError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ExportSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ExportSnapshotError {
    fn description(&self) -> &str {
        match *self {
            ExportSnapshotError::AccessDenied(ref cause) => cause,
            ExportSnapshotError::AccountSetupInProgress(ref cause) => cause,
            ExportSnapshotError::InvalidInput(ref cause) => cause,
            ExportSnapshotError::NotFound(ref cause) => cause,
            ExportSnapshotError::OperationFailure(ref cause) => cause,
            ExportSnapshotError::Service(ref cause) => cause,
            ExportSnapshotError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetActiveNames
#[derive(Debug, PartialEq)]
pub enum GetActiveNamesError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetActiveNamesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetActiveNamesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetActiveNamesError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetActiveNamesError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetActiveNamesError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetActiveNamesError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetActiveNamesError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetActiveNamesError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetActiveNamesError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetActiveNamesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetActiveNamesError {
    fn description(&self) -> &str {
        match *self {
            GetActiveNamesError::AccessDenied(ref cause) => cause,
            GetActiveNamesError::AccountSetupInProgress(ref cause) => cause,
            GetActiveNamesError::InvalidInput(ref cause) => cause,
            GetActiveNamesError::NotFound(ref cause) => cause,
            GetActiveNamesError::OperationFailure(ref cause) => cause,
            GetActiveNamesError::Service(ref cause) => cause,
            GetActiveNamesError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBlueprints
#[derive(Debug, PartialEq)]
pub enum GetBlueprintsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetBlueprintsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBlueprintsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetBlueprintsError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetBlueprintsError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetBlueprintsError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetBlueprintsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetBlueprintsError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetBlueprintsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetBlueprintsError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetBlueprintsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBlueprintsError {
    fn description(&self) -> &str {
        match *self {
            GetBlueprintsError::AccessDenied(ref cause) => cause,
            GetBlueprintsError::AccountSetupInProgress(ref cause) => cause,
            GetBlueprintsError::InvalidInput(ref cause) => cause,
            GetBlueprintsError::NotFound(ref cause) => cause,
            GetBlueprintsError::OperationFailure(ref cause) => cause,
            GetBlueprintsError::Service(ref cause) => cause,
            GetBlueprintsError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBundles
#[derive(Debug, PartialEq)]
pub enum GetBundlesError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetBundlesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBundlesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetBundlesError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetBundlesError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetBundlesError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetBundlesError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetBundlesError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetBundlesError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetBundlesError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetBundlesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBundlesError {
    fn description(&self) -> &str {
        match *self {
            GetBundlesError::AccessDenied(ref cause) => cause,
            GetBundlesError::AccountSetupInProgress(ref cause) => cause,
            GetBundlesError::InvalidInput(ref cause) => cause,
            GetBundlesError::NotFound(ref cause) => cause,
            GetBundlesError::OperationFailure(ref cause) => cause,
            GetBundlesError::Service(ref cause) => cause,
            GetBundlesError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCloudFormationStackRecords
#[derive(Debug, PartialEq)]
pub enum GetCloudFormationStackRecordsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetCloudFormationStackRecordsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetCloudFormationStackRecordsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetCloudFormationStackRecordsError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetCloudFormationStackRecordsError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetCloudFormationStackRecordsError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetCloudFormationStackRecordsError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetCloudFormationStackRecordsError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetCloudFormationStackRecordsError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        GetCloudFormationStackRecordsError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetCloudFormationStackRecordsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCloudFormationStackRecordsError {
    fn description(&self) -> &str {
        match *self {
            GetCloudFormationStackRecordsError::AccessDenied(ref cause) => cause,
            GetCloudFormationStackRecordsError::AccountSetupInProgress(ref cause) => cause,
            GetCloudFormationStackRecordsError::InvalidInput(ref cause) => cause,
            GetCloudFormationStackRecordsError::NotFound(ref cause) => cause,
            GetCloudFormationStackRecordsError::OperationFailure(ref cause) => cause,
            GetCloudFormationStackRecordsError::Service(ref cause) => cause,
            GetCloudFormationStackRecordsError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDisk
#[derive(Debug, PartialEq)]
pub enum GetDiskError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetDiskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDiskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetDiskError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetDiskError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDiskError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDiskError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetDiskError::OperationFailure(err.msg))
                }
                "ServiceException" => return RusotoError::Service(GetDiskError::Service(err.msg)),
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetDiskError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDiskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDiskError {
    fn description(&self) -> &str {
        match *self {
            GetDiskError::AccessDenied(ref cause) => cause,
            GetDiskError::AccountSetupInProgress(ref cause) => cause,
            GetDiskError::InvalidInput(ref cause) => cause,
            GetDiskError::NotFound(ref cause) => cause,
            GetDiskError::OperationFailure(ref cause) => cause,
            GetDiskError::Service(ref cause) => cause,
            GetDiskError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDiskSnapshot
#[derive(Debug, PartialEq)]
pub enum GetDiskSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetDiskSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDiskSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetDiskSnapshotError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetDiskSnapshotError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDiskSnapshotError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDiskSnapshotError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetDiskSnapshotError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetDiskSnapshotError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetDiskSnapshotError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDiskSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDiskSnapshotError {
    fn description(&self) -> &str {
        match *self {
            GetDiskSnapshotError::AccessDenied(ref cause) => cause,
            GetDiskSnapshotError::AccountSetupInProgress(ref cause) => cause,
            GetDiskSnapshotError::InvalidInput(ref cause) => cause,
            GetDiskSnapshotError::NotFound(ref cause) => cause,
            GetDiskSnapshotError::OperationFailure(ref cause) => cause,
            GetDiskSnapshotError::Service(ref cause) => cause,
            GetDiskSnapshotError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDiskSnapshots
#[derive(Debug, PartialEq)]
pub enum GetDiskSnapshotsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetDiskSnapshotsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDiskSnapshotsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetDiskSnapshotsError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetDiskSnapshotsError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDiskSnapshotsError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDiskSnapshotsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetDiskSnapshotsError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetDiskSnapshotsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetDiskSnapshotsError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDiskSnapshotsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDiskSnapshotsError {
    fn description(&self) -> &str {
        match *self {
            GetDiskSnapshotsError::AccessDenied(ref cause) => cause,
            GetDiskSnapshotsError::AccountSetupInProgress(ref cause) => cause,
            GetDiskSnapshotsError::InvalidInput(ref cause) => cause,
            GetDiskSnapshotsError::NotFound(ref cause) => cause,
            GetDiskSnapshotsError::OperationFailure(ref cause) => cause,
            GetDiskSnapshotsError::Service(ref cause) => cause,
            GetDiskSnapshotsError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDisks
#[derive(Debug, PartialEq)]
pub enum GetDisksError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetDisksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDisksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetDisksError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetDisksError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDisksError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDisksError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetDisksError::OperationFailure(err.msg))
                }
                "ServiceException" => return RusotoError::Service(GetDisksError::Service(err.msg)),
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetDisksError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDisksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDisksError {
    fn description(&self) -> &str {
        match *self {
            GetDisksError::AccessDenied(ref cause) => cause,
            GetDisksError::AccountSetupInProgress(ref cause) => cause,
            GetDisksError::InvalidInput(ref cause) => cause,
            GetDisksError::NotFound(ref cause) => cause,
            GetDisksError::OperationFailure(ref cause) => cause,
            GetDisksError::Service(ref cause) => cause,
            GetDisksError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDomain
#[derive(Debug, PartialEq)]
pub enum GetDomainError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetDomainError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDomainError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetDomainError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetDomainError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDomainError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDomainError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetDomainError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetDomainError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetDomainError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDomainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDomainError {
    fn description(&self) -> &str {
        match *self {
            GetDomainError::AccessDenied(ref cause) => cause,
            GetDomainError::AccountSetupInProgress(ref cause) => cause,
            GetDomainError::InvalidInput(ref cause) => cause,
            GetDomainError::NotFound(ref cause) => cause,
            GetDomainError::OperationFailure(ref cause) => cause,
            GetDomainError::Service(ref cause) => cause,
            GetDomainError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDomains
#[derive(Debug, PartialEq)]
pub enum GetDomainsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetDomainsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDomainsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetDomainsError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetDomainsError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetDomainsError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDomainsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetDomainsError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetDomainsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetDomainsError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDomainsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDomainsError {
    fn description(&self) -> &str {
        match *self {
            GetDomainsError::AccessDenied(ref cause) => cause,
            GetDomainsError::AccountSetupInProgress(ref cause) => cause,
            GetDomainsError::InvalidInput(ref cause) => cause,
            GetDomainsError::NotFound(ref cause) => cause,
            GetDomainsError::OperationFailure(ref cause) => cause,
            GetDomainsError::Service(ref cause) => cause,
            GetDomainsError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetExportSnapshotRecords
#[derive(Debug, PartialEq)]
pub enum GetExportSnapshotRecordsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetExportSnapshotRecordsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetExportSnapshotRecordsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetExportSnapshotRecordsError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetExportSnapshotRecordsError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetExportSnapshotRecordsError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetExportSnapshotRecordsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetExportSnapshotRecordsError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetExportSnapshotRecordsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetExportSnapshotRecordsError::Unauthenticated(
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
impl fmt::Display for GetExportSnapshotRecordsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetExportSnapshotRecordsError {
    fn description(&self) -> &str {
        match *self {
            GetExportSnapshotRecordsError::AccessDenied(ref cause) => cause,
            GetExportSnapshotRecordsError::AccountSetupInProgress(ref cause) => cause,
            GetExportSnapshotRecordsError::InvalidInput(ref cause) => cause,
            GetExportSnapshotRecordsError::NotFound(ref cause) => cause,
            GetExportSnapshotRecordsError::OperationFailure(ref cause) => cause,
            GetExportSnapshotRecordsError::Service(ref cause) => cause,
            GetExportSnapshotRecordsError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInstance
#[derive(Debug, PartialEq)]
pub enum GetInstanceError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetInstanceError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetInstanceError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetInstanceError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetInstanceError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetInstanceError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetInstanceError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetInstanceError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInstanceError {
    fn description(&self) -> &str {
        match *self {
            GetInstanceError::AccessDenied(ref cause) => cause,
            GetInstanceError::AccountSetupInProgress(ref cause) => cause,
            GetInstanceError::InvalidInput(ref cause) => cause,
            GetInstanceError::NotFound(ref cause) => cause,
            GetInstanceError::OperationFailure(ref cause) => cause,
            GetInstanceError::Service(ref cause) => cause,
            GetInstanceError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInstanceAccessDetails
#[derive(Debug, PartialEq)]
pub enum GetInstanceAccessDetailsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetInstanceAccessDetailsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInstanceAccessDetailsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetInstanceAccessDetailsError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetInstanceAccessDetailsError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetInstanceAccessDetailsError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetInstanceAccessDetailsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetInstanceAccessDetailsError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetInstanceAccessDetailsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetInstanceAccessDetailsError::Unauthenticated(
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
impl fmt::Display for GetInstanceAccessDetailsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInstanceAccessDetailsError {
    fn description(&self) -> &str {
        match *self {
            GetInstanceAccessDetailsError::AccessDenied(ref cause) => cause,
            GetInstanceAccessDetailsError::AccountSetupInProgress(ref cause) => cause,
            GetInstanceAccessDetailsError::InvalidInput(ref cause) => cause,
            GetInstanceAccessDetailsError::NotFound(ref cause) => cause,
            GetInstanceAccessDetailsError::OperationFailure(ref cause) => cause,
            GetInstanceAccessDetailsError::Service(ref cause) => cause,
            GetInstanceAccessDetailsError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInstanceMetricData
#[derive(Debug, PartialEq)]
pub enum GetInstanceMetricDataError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetInstanceMetricDataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInstanceMetricDataError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetInstanceMetricDataError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetInstanceMetricDataError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetInstanceMetricDataError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetInstanceMetricDataError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetInstanceMetricDataError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetInstanceMetricDataError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetInstanceMetricDataError::Unauthenticated(
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
impl fmt::Display for GetInstanceMetricDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInstanceMetricDataError {
    fn description(&self) -> &str {
        match *self {
            GetInstanceMetricDataError::AccessDenied(ref cause) => cause,
            GetInstanceMetricDataError::AccountSetupInProgress(ref cause) => cause,
            GetInstanceMetricDataError::InvalidInput(ref cause) => cause,
            GetInstanceMetricDataError::NotFound(ref cause) => cause,
            GetInstanceMetricDataError::OperationFailure(ref cause) => cause,
            GetInstanceMetricDataError::Service(ref cause) => cause,
            GetInstanceMetricDataError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInstancePortStates
#[derive(Debug, PartialEq)]
pub enum GetInstancePortStatesError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetInstancePortStatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInstancePortStatesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetInstancePortStatesError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetInstancePortStatesError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetInstancePortStatesError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetInstancePortStatesError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetInstancePortStatesError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetInstancePortStatesError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetInstancePortStatesError::Unauthenticated(
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
impl fmt::Display for GetInstancePortStatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInstancePortStatesError {
    fn description(&self) -> &str {
        match *self {
            GetInstancePortStatesError::AccessDenied(ref cause) => cause,
            GetInstancePortStatesError::AccountSetupInProgress(ref cause) => cause,
            GetInstancePortStatesError::InvalidInput(ref cause) => cause,
            GetInstancePortStatesError::NotFound(ref cause) => cause,
            GetInstancePortStatesError::OperationFailure(ref cause) => cause,
            GetInstancePortStatesError::Service(ref cause) => cause,
            GetInstancePortStatesError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInstanceSnapshot
#[derive(Debug, PartialEq)]
pub enum GetInstanceSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetInstanceSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInstanceSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetInstanceSnapshotError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetInstanceSnapshotError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetInstanceSnapshotError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetInstanceSnapshotError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetInstanceSnapshotError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetInstanceSnapshotError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetInstanceSnapshotError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetInstanceSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInstanceSnapshotError {
    fn description(&self) -> &str {
        match *self {
            GetInstanceSnapshotError::AccessDenied(ref cause) => cause,
            GetInstanceSnapshotError::AccountSetupInProgress(ref cause) => cause,
            GetInstanceSnapshotError::InvalidInput(ref cause) => cause,
            GetInstanceSnapshotError::NotFound(ref cause) => cause,
            GetInstanceSnapshotError::OperationFailure(ref cause) => cause,
            GetInstanceSnapshotError::Service(ref cause) => cause,
            GetInstanceSnapshotError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInstanceSnapshots
#[derive(Debug, PartialEq)]
pub enum GetInstanceSnapshotsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetInstanceSnapshotsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInstanceSnapshotsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetInstanceSnapshotsError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetInstanceSnapshotsError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetInstanceSnapshotsError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetInstanceSnapshotsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetInstanceSnapshotsError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetInstanceSnapshotsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetInstanceSnapshotsError::Unauthenticated(
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
impl fmt::Display for GetInstanceSnapshotsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInstanceSnapshotsError {
    fn description(&self) -> &str {
        match *self {
            GetInstanceSnapshotsError::AccessDenied(ref cause) => cause,
            GetInstanceSnapshotsError::AccountSetupInProgress(ref cause) => cause,
            GetInstanceSnapshotsError::InvalidInput(ref cause) => cause,
            GetInstanceSnapshotsError::NotFound(ref cause) => cause,
            GetInstanceSnapshotsError::OperationFailure(ref cause) => cause,
            GetInstanceSnapshotsError::Service(ref cause) => cause,
            GetInstanceSnapshotsError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInstanceState
#[derive(Debug, PartialEq)]
pub enum GetInstanceStateError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetInstanceStateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInstanceStateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetInstanceStateError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetInstanceStateError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetInstanceStateError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetInstanceStateError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetInstanceStateError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetInstanceStateError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetInstanceStateError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetInstanceStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInstanceStateError {
    fn description(&self) -> &str {
        match *self {
            GetInstanceStateError::AccessDenied(ref cause) => cause,
            GetInstanceStateError::AccountSetupInProgress(ref cause) => cause,
            GetInstanceStateError::InvalidInput(ref cause) => cause,
            GetInstanceStateError::NotFound(ref cause) => cause,
            GetInstanceStateError::OperationFailure(ref cause) => cause,
            GetInstanceStateError::Service(ref cause) => cause,
            GetInstanceStateError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetInstances
#[derive(Debug, PartialEq)]
pub enum GetInstancesError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetInstancesError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetInstancesError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetInstancesError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetInstancesError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetInstancesError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetInstancesError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetInstancesError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetInstancesError {
    fn description(&self) -> &str {
        match *self {
            GetInstancesError::AccessDenied(ref cause) => cause,
            GetInstancesError::AccountSetupInProgress(ref cause) => cause,
            GetInstancesError::InvalidInput(ref cause) => cause,
            GetInstancesError::NotFound(ref cause) => cause,
            GetInstancesError::OperationFailure(ref cause) => cause,
            GetInstancesError::Service(ref cause) => cause,
            GetInstancesError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetKeyPair
#[derive(Debug, PartialEq)]
pub enum GetKeyPairError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetKeyPairError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetKeyPairError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetKeyPairError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetKeyPairError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetKeyPairError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetKeyPairError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetKeyPairError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetKeyPairError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetKeyPairError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetKeyPairError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetKeyPairError {
    fn description(&self) -> &str {
        match *self {
            GetKeyPairError::AccessDenied(ref cause) => cause,
            GetKeyPairError::AccountSetupInProgress(ref cause) => cause,
            GetKeyPairError::InvalidInput(ref cause) => cause,
            GetKeyPairError::NotFound(ref cause) => cause,
            GetKeyPairError::OperationFailure(ref cause) => cause,
            GetKeyPairError::Service(ref cause) => cause,
            GetKeyPairError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetKeyPairs
#[derive(Debug, PartialEq)]
pub enum GetKeyPairsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetKeyPairsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetKeyPairsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetKeyPairsError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetKeyPairsError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetKeyPairsError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetKeyPairsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetKeyPairsError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetKeyPairsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetKeyPairsError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetKeyPairsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetKeyPairsError {
    fn description(&self) -> &str {
        match *self {
            GetKeyPairsError::AccessDenied(ref cause) => cause,
            GetKeyPairsError::AccountSetupInProgress(ref cause) => cause,
            GetKeyPairsError::InvalidInput(ref cause) => cause,
            GetKeyPairsError::NotFound(ref cause) => cause,
            GetKeyPairsError::OperationFailure(ref cause) => cause,
            GetKeyPairsError::Service(ref cause) => cause,
            GetKeyPairsError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetLoadBalancer
#[derive(Debug, PartialEq)]
pub enum GetLoadBalancerError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetLoadBalancerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLoadBalancerError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetLoadBalancerError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetLoadBalancerError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetLoadBalancerError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetLoadBalancerError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetLoadBalancerError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetLoadBalancerError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetLoadBalancerError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetLoadBalancerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLoadBalancerError {
    fn description(&self) -> &str {
        match *self {
            GetLoadBalancerError::AccessDenied(ref cause) => cause,
            GetLoadBalancerError::AccountSetupInProgress(ref cause) => cause,
            GetLoadBalancerError::InvalidInput(ref cause) => cause,
            GetLoadBalancerError::NotFound(ref cause) => cause,
            GetLoadBalancerError::OperationFailure(ref cause) => cause,
            GetLoadBalancerError::Service(ref cause) => cause,
            GetLoadBalancerError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetLoadBalancerMetricData
#[derive(Debug, PartialEq)]
pub enum GetLoadBalancerMetricDataError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetLoadBalancerMetricDataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLoadBalancerMetricDataError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetLoadBalancerMetricDataError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetLoadBalancerMetricDataError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetLoadBalancerMetricDataError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetLoadBalancerMetricDataError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetLoadBalancerMetricDataError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetLoadBalancerMetricDataError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetLoadBalancerMetricDataError::Unauthenticated(
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
impl fmt::Display for GetLoadBalancerMetricDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLoadBalancerMetricDataError {
    fn description(&self) -> &str {
        match *self {
            GetLoadBalancerMetricDataError::AccessDenied(ref cause) => cause,
            GetLoadBalancerMetricDataError::AccountSetupInProgress(ref cause) => cause,
            GetLoadBalancerMetricDataError::InvalidInput(ref cause) => cause,
            GetLoadBalancerMetricDataError::NotFound(ref cause) => cause,
            GetLoadBalancerMetricDataError::OperationFailure(ref cause) => cause,
            GetLoadBalancerMetricDataError::Service(ref cause) => cause,
            GetLoadBalancerMetricDataError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetLoadBalancerTlsCertificates
#[derive(Debug, PartialEq)]
pub enum GetLoadBalancerTlsCertificatesError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetLoadBalancerTlsCertificatesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetLoadBalancerTlsCertificatesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetLoadBalancerTlsCertificatesError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetLoadBalancerTlsCertificatesError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetLoadBalancerTlsCertificatesError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetLoadBalancerTlsCertificatesError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetLoadBalancerTlsCertificatesError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetLoadBalancerTlsCertificatesError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        GetLoadBalancerTlsCertificatesError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetLoadBalancerTlsCertificatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLoadBalancerTlsCertificatesError {
    fn description(&self) -> &str {
        match *self {
            GetLoadBalancerTlsCertificatesError::AccessDenied(ref cause) => cause,
            GetLoadBalancerTlsCertificatesError::AccountSetupInProgress(ref cause) => cause,
            GetLoadBalancerTlsCertificatesError::InvalidInput(ref cause) => cause,
            GetLoadBalancerTlsCertificatesError::NotFound(ref cause) => cause,
            GetLoadBalancerTlsCertificatesError::OperationFailure(ref cause) => cause,
            GetLoadBalancerTlsCertificatesError::Service(ref cause) => cause,
            GetLoadBalancerTlsCertificatesError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetLoadBalancers
#[derive(Debug, PartialEq)]
pub enum GetLoadBalancersError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetLoadBalancersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLoadBalancersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetLoadBalancersError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetLoadBalancersError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetLoadBalancersError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetLoadBalancersError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetLoadBalancersError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetLoadBalancersError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetLoadBalancersError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetLoadBalancersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLoadBalancersError {
    fn description(&self) -> &str {
        match *self {
            GetLoadBalancersError::AccessDenied(ref cause) => cause,
            GetLoadBalancersError::AccountSetupInProgress(ref cause) => cause,
            GetLoadBalancersError::InvalidInput(ref cause) => cause,
            GetLoadBalancersError::NotFound(ref cause) => cause,
            GetLoadBalancersError::OperationFailure(ref cause) => cause,
            GetLoadBalancersError::Service(ref cause) => cause,
            GetLoadBalancersError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetOperation
#[derive(Debug, PartialEq)]
pub enum GetOperationError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetOperationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetOperationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetOperationError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetOperationError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetOperationError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetOperationError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetOperationError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetOperationError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetOperationError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetOperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetOperationError {
    fn description(&self) -> &str {
        match *self {
            GetOperationError::AccessDenied(ref cause) => cause,
            GetOperationError::AccountSetupInProgress(ref cause) => cause,
            GetOperationError::InvalidInput(ref cause) => cause,
            GetOperationError::NotFound(ref cause) => cause,
            GetOperationError::OperationFailure(ref cause) => cause,
            GetOperationError::Service(ref cause) => cause,
            GetOperationError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetOperations
#[derive(Debug, PartialEq)]
pub enum GetOperationsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetOperationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetOperationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetOperationsError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetOperationsError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetOperationsError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetOperationsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetOperationsError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetOperationsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetOperationsError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetOperationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetOperationsError {
    fn description(&self) -> &str {
        match *self {
            GetOperationsError::AccessDenied(ref cause) => cause,
            GetOperationsError::AccountSetupInProgress(ref cause) => cause,
            GetOperationsError::InvalidInput(ref cause) => cause,
            GetOperationsError::NotFound(ref cause) => cause,
            GetOperationsError::OperationFailure(ref cause) => cause,
            GetOperationsError::Service(ref cause) => cause,
            GetOperationsError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetOperationsForResource
#[derive(Debug, PartialEq)]
pub enum GetOperationsForResourceError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetOperationsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetOperationsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetOperationsForResourceError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetOperationsForResourceError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetOperationsForResourceError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetOperationsForResourceError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetOperationsForResourceError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetOperationsForResourceError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetOperationsForResourceError::Unauthenticated(
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
impl fmt::Display for GetOperationsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetOperationsForResourceError {
    fn description(&self) -> &str {
        match *self {
            GetOperationsForResourceError::AccessDenied(ref cause) => cause,
            GetOperationsForResourceError::AccountSetupInProgress(ref cause) => cause,
            GetOperationsForResourceError::InvalidInput(ref cause) => cause,
            GetOperationsForResourceError::NotFound(ref cause) => cause,
            GetOperationsForResourceError::OperationFailure(ref cause) => cause,
            GetOperationsForResourceError::Service(ref cause) => cause,
            GetOperationsForResourceError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRegions
#[derive(Debug, PartialEq)]
pub enum GetRegionsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRegionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRegionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetRegionsError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetRegionsError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetRegionsError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRegionsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetRegionsError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRegionsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetRegionsError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRegionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRegionsError {
    fn description(&self) -> &str {
        match *self {
            GetRegionsError::AccessDenied(ref cause) => cause,
            GetRegionsError::AccountSetupInProgress(ref cause) => cause,
            GetRegionsError::InvalidInput(ref cause) => cause,
            GetRegionsError::NotFound(ref cause) => cause,
            GetRegionsError::OperationFailure(ref cause) => cause,
            GetRegionsError::Service(ref cause) => cause,
            GetRegionsError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRelationalDatabase
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabaseError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRelationalDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetRelationalDatabaseError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetRelationalDatabaseError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRelationalDatabaseError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetRelationalDatabaseError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRelationalDatabaseError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetRelationalDatabaseError::Unauthenticated(
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
impl fmt::Display for GetRelationalDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRelationalDatabaseError {
    fn description(&self) -> &str {
        match *self {
            GetRelationalDatabaseError::AccessDenied(ref cause) => cause,
            GetRelationalDatabaseError::AccountSetupInProgress(ref cause) => cause,
            GetRelationalDatabaseError::InvalidInput(ref cause) => cause,
            GetRelationalDatabaseError::NotFound(ref cause) => cause,
            GetRelationalDatabaseError::OperationFailure(ref cause) => cause,
            GetRelationalDatabaseError::Service(ref cause) => cause,
            GetRelationalDatabaseError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRelationalDatabaseBlueprints
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabaseBlueprintsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabaseBlueprintsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRelationalDatabaseBlueprintsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseBlueprintsError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseBlueprintsError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseBlueprintsError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRelationalDatabaseBlueprintsError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseBlueprintsError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRelationalDatabaseBlueprintsError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseBlueprintsError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRelationalDatabaseBlueprintsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRelationalDatabaseBlueprintsError {
    fn description(&self) -> &str {
        match *self {
            GetRelationalDatabaseBlueprintsError::AccessDenied(ref cause) => cause,
            GetRelationalDatabaseBlueprintsError::AccountSetupInProgress(ref cause) => cause,
            GetRelationalDatabaseBlueprintsError::InvalidInput(ref cause) => cause,
            GetRelationalDatabaseBlueprintsError::NotFound(ref cause) => cause,
            GetRelationalDatabaseBlueprintsError::OperationFailure(ref cause) => cause,
            GetRelationalDatabaseBlueprintsError::Service(ref cause) => cause,
            GetRelationalDatabaseBlueprintsError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRelationalDatabaseBundles
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabaseBundlesError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabaseBundlesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRelationalDatabaseBundlesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetRelationalDatabaseBundlesError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseBundlesError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetRelationalDatabaseBundlesError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRelationalDatabaseBundlesError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseBundlesError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRelationalDatabaseBundlesError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseBundlesError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRelationalDatabaseBundlesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRelationalDatabaseBundlesError {
    fn description(&self) -> &str {
        match *self {
            GetRelationalDatabaseBundlesError::AccessDenied(ref cause) => cause,
            GetRelationalDatabaseBundlesError::AccountSetupInProgress(ref cause) => cause,
            GetRelationalDatabaseBundlesError::InvalidInput(ref cause) => cause,
            GetRelationalDatabaseBundlesError::NotFound(ref cause) => cause,
            GetRelationalDatabaseBundlesError::OperationFailure(ref cause) => cause,
            GetRelationalDatabaseBundlesError::Service(ref cause) => cause,
            GetRelationalDatabaseBundlesError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRelationalDatabaseEvents
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabaseEventsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabaseEventsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRelationalDatabaseEventsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetRelationalDatabaseEventsError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseEventsError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetRelationalDatabaseEventsError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRelationalDatabaseEventsError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseEventsError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRelationalDatabaseEventsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetRelationalDatabaseEventsError::Unauthenticated(
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
impl fmt::Display for GetRelationalDatabaseEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRelationalDatabaseEventsError {
    fn description(&self) -> &str {
        match *self {
            GetRelationalDatabaseEventsError::AccessDenied(ref cause) => cause,
            GetRelationalDatabaseEventsError::AccountSetupInProgress(ref cause) => cause,
            GetRelationalDatabaseEventsError::InvalidInput(ref cause) => cause,
            GetRelationalDatabaseEventsError::NotFound(ref cause) => cause,
            GetRelationalDatabaseEventsError::OperationFailure(ref cause) => cause,
            GetRelationalDatabaseEventsError::Service(ref cause) => cause,
            GetRelationalDatabaseEventsError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRelationalDatabaseLogEvents
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabaseLogEventsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabaseLogEventsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRelationalDatabaseLogEventsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetRelationalDatabaseLogEventsError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseLogEventsError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetRelationalDatabaseLogEventsError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRelationalDatabaseLogEventsError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseLogEventsError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRelationalDatabaseLogEventsError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseLogEventsError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRelationalDatabaseLogEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRelationalDatabaseLogEventsError {
    fn description(&self) -> &str {
        match *self {
            GetRelationalDatabaseLogEventsError::AccessDenied(ref cause) => cause,
            GetRelationalDatabaseLogEventsError::AccountSetupInProgress(ref cause) => cause,
            GetRelationalDatabaseLogEventsError::InvalidInput(ref cause) => cause,
            GetRelationalDatabaseLogEventsError::NotFound(ref cause) => cause,
            GetRelationalDatabaseLogEventsError::OperationFailure(ref cause) => cause,
            GetRelationalDatabaseLogEventsError::Service(ref cause) => cause,
            GetRelationalDatabaseLogEventsError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRelationalDatabaseLogStreams
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabaseLogStreamsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabaseLogStreamsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRelationalDatabaseLogStreamsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseLogStreamsError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseLogStreamsError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseLogStreamsError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRelationalDatabaseLogStreamsError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseLogStreamsError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRelationalDatabaseLogStreamsError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseLogStreamsError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRelationalDatabaseLogStreamsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRelationalDatabaseLogStreamsError {
    fn description(&self) -> &str {
        match *self {
            GetRelationalDatabaseLogStreamsError::AccessDenied(ref cause) => cause,
            GetRelationalDatabaseLogStreamsError::AccountSetupInProgress(ref cause) => cause,
            GetRelationalDatabaseLogStreamsError::InvalidInput(ref cause) => cause,
            GetRelationalDatabaseLogStreamsError::NotFound(ref cause) => cause,
            GetRelationalDatabaseLogStreamsError::OperationFailure(ref cause) => cause,
            GetRelationalDatabaseLogStreamsError::Service(ref cause) => cause,
            GetRelationalDatabaseLogStreamsError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRelationalDatabaseMasterUserPassword
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabaseMasterUserPasswordError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabaseMasterUserPasswordError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRelationalDatabaseMasterUserPasswordError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMasterUserPasswordError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMasterUserPasswordError::AccountSetupInProgress(
                            err.msg,
                        ),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMasterUserPasswordError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMasterUserPasswordError::NotFound(err.msg),
                    )
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMasterUserPasswordError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMasterUserPasswordError::Service(err.msg),
                    )
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMasterUserPasswordError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRelationalDatabaseMasterUserPasswordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRelationalDatabaseMasterUserPasswordError {
    fn description(&self) -> &str {
        match *self {
            GetRelationalDatabaseMasterUserPasswordError::AccessDenied(ref cause) => cause,
            GetRelationalDatabaseMasterUserPasswordError::AccountSetupInProgress(ref cause) => {
                cause
            }
            GetRelationalDatabaseMasterUserPasswordError::InvalidInput(ref cause) => cause,
            GetRelationalDatabaseMasterUserPasswordError::NotFound(ref cause) => cause,
            GetRelationalDatabaseMasterUserPasswordError::OperationFailure(ref cause) => cause,
            GetRelationalDatabaseMasterUserPasswordError::Service(ref cause) => cause,
            GetRelationalDatabaseMasterUserPasswordError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRelationalDatabaseMetricData
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabaseMetricDataError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabaseMetricDataError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRelationalDatabaseMetricDataError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMetricDataError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMetricDataError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMetricDataError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRelationalDatabaseMetricDataError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMetricDataError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRelationalDatabaseMetricDataError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseMetricDataError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRelationalDatabaseMetricDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRelationalDatabaseMetricDataError {
    fn description(&self) -> &str {
        match *self {
            GetRelationalDatabaseMetricDataError::AccessDenied(ref cause) => cause,
            GetRelationalDatabaseMetricDataError::AccountSetupInProgress(ref cause) => cause,
            GetRelationalDatabaseMetricDataError::InvalidInput(ref cause) => cause,
            GetRelationalDatabaseMetricDataError::NotFound(ref cause) => cause,
            GetRelationalDatabaseMetricDataError::OperationFailure(ref cause) => cause,
            GetRelationalDatabaseMetricDataError::Service(ref cause) => cause,
            GetRelationalDatabaseMetricDataError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRelationalDatabaseParameters
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabaseParametersError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabaseParametersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRelationalDatabaseParametersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseParametersError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseParametersError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseParametersError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRelationalDatabaseParametersError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseParametersError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRelationalDatabaseParametersError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseParametersError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRelationalDatabaseParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRelationalDatabaseParametersError {
    fn description(&self) -> &str {
        match *self {
            GetRelationalDatabaseParametersError::AccessDenied(ref cause) => cause,
            GetRelationalDatabaseParametersError::AccountSetupInProgress(ref cause) => cause,
            GetRelationalDatabaseParametersError::InvalidInput(ref cause) => cause,
            GetRelationalDatabaseParametersError::NotFound(ref cause) => cause,
            GetRelationalDatabaseParametersError::OperationFailure(ref cause) => cause,
            GetRelationalDatabaseParametersError::Service(ref cause) => cause,
            GetRelationalDatabaseParametersError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRelationalDatabaseSnapshot
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabaseSnapshotError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabaseSnapshotError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRelationalDatabaseSnapshotError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetRelationalDatabaseSnapshotError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseSnapshotError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetRelationalDatabaseSnapshotError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRelationalDatabaseSnapshotError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseSnapshotError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRelationalDatabaseSnapshotError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseSnapshotError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRelationalDatabaseSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRelationalDatabaseSnapshotError {
    fn description(&self) -> &str {
        match *self {
            GetRelationalDatabaseSnapshotError::AccessDenied(ref cause) => cause,
            GetRelationalDatabaseSnapshotError::AccountSetupInProgress(ref cause) => cause,
            GetRelationalDatabaseSnapshotError::InvalidInput(ref cause) => cause,
            GetRelationalDatabaseSnapshotError::NotFound(ref cause) => cause,
            GetRelationalDatabaseSnapshotError::OperationFailure(ref cause) => cause,
            GetRelationalDatabaseSnapshotError::Service(ref cause) => cause,
            GetRelationalDatabaseSnapshotError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRelationalDatabaseSnapshots
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabaseSnapshotsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabaseSnapshotsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetRelationalDatabaseSnapshotsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetRelationalDatabaseSnapshotsError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseSnapshotsError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetRelationalDatabaseSnapshotsError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRelationalDatabaseSnapshotsError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseSnapshotsError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRelationalDatabaseSnapshotsError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        GetRelationalDatabaseSnapshotsError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRelationalDatabaseSnapshotsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRelationalDatabaseSnapshotsError {
    fn description(&self) -> &str {
        match *self {
            GetRelationalDatabaseSnapshotsError::AccessDenied(ref cause) => cause,
            GetRelationalDatabaseSnapshotsError::AccountSetupInProgress(ref cause) => cause,
            GetRelationalDatabaseSnapshotsError::InvalidInput(ref cause) => cause,
            GetRelationalDatabaseSnapshotsError::NotFound(ref cause) => cause,
            GetRelationalDatabaseSnapshotsError::OperationFailure(ref cause) => cause,
            GetRelationalDatabaseSnapshotsError::Service(ref cause) => cause,
            GetRelationalDatabaseSnapshotsError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRelationalDatabases
#[derive(Debug, PartialEq)]
pub enum GetRelationalDatabasesError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetRelationalDatabasesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRelationalDatabasesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetRelationalDatabasesError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        GetRelationalDatabasesError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetRelationalDatabasesError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRelationalDatabasesError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetRelationalDatabasesError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetRelationalDatabasesError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetRelationalDatabasesError::Unauthenticated(
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
impl fmt::Display for GetRelationalDatabasesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRelationalDatabasesError {
    fn description(&self) -> &str {
        match *self {
            GetRelationalDatabasesError::AccessDenied(ref cause) => cause,
            GetRelationalDatabasesError::AccountSetupInProgress(ref cause) => cause,
            GetRelationalDatabasesError::InvalidInput(ref cause) => cause,
            GetRelationalDatabasesError::NotFound(ref cause) => cause,
            GetRelationalDatabasesError::OperationFailure(ref cause) => cause,
            GetRelationalDatabasesError::Service(ref cause) => cause,
            GetRelationalDatabasesError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetStaticIp
#[derive(Debug, PartialEq)]
pub enum GetStaticIpError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetStaticIpError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetStaticIpError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetStaticIpError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetStaticIpError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetStaticIpError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetStaticIpError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetStaticIpError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetStaticIpError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetStaticIpError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetStaticIpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetStaticIpError {
    fn description(&self) -> &str {
        match *self {
            GetStaticIpError::AccessDenied(ref cause) => cause,
            GetStaticIpError::AccountSetupInProgress(ref cause) => cause,
            GetStaticIpError::InvalidInput(ref cause) => cause,
            GetStaticIpError::NotFound(ref cause) => cause,
            GetStaticIpError::OperationFailure(ref cause) => cause,
            GetStaticIpError::Service(ref cause) => cause,
            GetStaticIpError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by GetStaticIps
#[derive(Debug, PartialEq)]
pub enum GetStaticIpsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl GetStaticIpsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetStaticIpsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetStaticIpsError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(GetStaticIpsError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetStaticIpsError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetStaticIpsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(GetStaticIpsError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetStaticIpsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(GetStaticIpsError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetStaticIpsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetStaticIpsError {
    fn description(&self) -> &str {
        match *self {
            GetStaticIpsError::AccessDenied(ref cause) => cause,
            GetStaticIpsError::AccountSetupInProgress(ref cause) => cause,
            GetStaticIpsError::InvalidInput(ref cause) => cause,
            GetStaticIpsError::NotFound(ref cause) => cause,
            GetStaticIpsError::OperationFailure(ref cause) => cause,
            GetStaticIpsError::Service(ref cause) => cause,
            GetStaticIpsError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by ImportKeyPair
#[derive(Debug, PartialEq)]
pub enum ImportKeyPairError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl ImportKeyPairError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportKeyPairError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ImportKeyPairError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(ImportKeyPairError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ImportKeyPairError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ImportKeyPairError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(ImportKeyPairError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ImportKeyPairError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(ImportKeyPairError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ImportKeyPairError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ImportKeyPairError {
    fn description(&self) -> &str {
        match *self {
            ImportKeyPairError::AccessDenied(ref cause) => cause,
            ImportKeyPairError::AccountSetupInProgress(ref cause) => cause,
            ImportKeyPairError::InvalidInput(ref cause) => cause,
            ImportKeyPairError::NotFound(ref cause) => cause,
            ImportKeyPairError::OperationFailure(ref cause) => cause,
            ImportKeyPairError::Service(ref cause) => cause,
            ImportKeyPairError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by IsVpcPeered
#[derive(Debug, PartialEq)]
pub enum IsVpcPeeredError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl IsVpcPeeredError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<IsVpcPeeredError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(IsVpcPeeredError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(IsVpcPeeredError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(IsVpcPeeredError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(IsVpcPeeredError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(IsVpcPeeredError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(IsVpcPeeredError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(IsVpcPeeredError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for IsVpcPeeredError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for IsVpcPeeredError {
    fn description(&self) -> &str {
        match *self {
            IsVpcPeeredError::AccessDenied(ref cause) => cause,
            IsVpcPeeredError::AccountSetupInProgress(ref cause) => cause,
            IsVpcPeeredError::InvalidInput(ref cause) => cause,
            IsVpcPeeredError::NotFound(ref cause) => cause,
            IsVpcPeeredError::OperationFailure(ref cause) => cause,
            IsVpcPeeredError::Service(ref cause) => cause,
            IsVpcPeeredError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by OpenInstancePublicPorts
#[derive(Debug, PartialEq)]
pub enum OpenInstancePublicPortsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl OpenInstancePublicPortsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<OpenInstancePublicPortsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(OpenInstancePublicPortsError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        OpenInstancePublicPortsError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(OpenInstancePublicPortsError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(OpenInstancePublicPortsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(OpenInstancePublicPortsError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(OpenInstancePublicPortsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(OpenInstancePublicPortsError::Unauthenticated(
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
impl fmt::Display for OpenInstancePublicPortsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for OpenInstancePublicPortsError {
    fn description(&self) -> &str {
        match *self {
            OpenInstancePublicPortsError::AccessDenied(ref cause) => cause,
            OpenInstancePublicPortsError::AccountSetupInProgress(ref cause) => cause,
            OpenInstancePublicPortsError::InvalidInput(ref cause) => cause,
            OpenInstancePublicPortsError::NotFound(ref cause) => cause,
            OpenInstancePublicPortsError::OperationFailure(ref cause) => cause,
            OpenInstancePublicPortsError::Service(ref cause) => cause,
            OpenInstancePublicPortsError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by PeerVpc
#[derive(Debug, PartialEq)]
pub enum PeerVpcError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl PeerVpcError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PeerVpcError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(PeerVpcError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(PeerVpcError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(PeerVpcError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PeerVpcError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(PeerVpcError::OperationFailure(err.msg))
                }
                "ServiceException" => return RusotoError::Service(PeerVpcError::Service(err.msg)),
                "UnauthenticatedException" => {
                    return RusotoError::Service(PeerVpcError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PeerVpcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PeerVpcError {
    fn description(&self) -> &str {
        match *self {
            PeerVpcError::AccessDenied(ref cause) => cause,
            PeerVpcError::AccountSetupInProgress(ref cause) => cause,
            PeerVpcError::InvalidInput(ref cause) => cause,
            PeerVpcError::NotFound(ref cause) => cause,
            PeerVpcError::OperationFailure(ref cause) => cause,
            PeerVpcError::Service(ref cause) => cause,
            PeerVpcError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by PutInstancePublicPorts
#[derive(Debug, PartialEq)]
pub enum PutInstancePublicPortsError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl PutInstancePublicPortsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutInstancePublicPortsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(PutInstancePublicPortsError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        PutInstancePublicPortsError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(PutInstancePublicPortsError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutInstancePublicPortsError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(PutInstancePublicPortsError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(PutInstancePublicPortsError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(PutInstancePublicPortsError::Unauthenticated(
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
impl fmt::Display for PutInstancePublicPortsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutInstancePublicPortsError {
    fn description(&self) -> &str {
        match *self {
            PutInstancePublicPortsError::AccessDenied(ref cause) => cause,
            PutInstancePublicPortsError::AccountSetupInProgress(ref cause) => cause,
            PutInstancePublicPortsError::InvalidInput(ref cause) => cause,
            PutInstancePublicPortsError::NotFound(ref cause) => cause,
            PutInstancePublicPortsError::OperationFailure(ref cause) => cause,
            PutInstancePublicPortsError::Service(ref cause) => cause,
            PutInstancePublicPortsError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by RebootInstance
#[derive(Debug, PartialEq)]
pub enum RebootInstanceError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl RebootInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RebootInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(RebootInstanceError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(RebootInstanceError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(RebootInstanceError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RebootInstanceError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(RebootInstanceError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(RebootInstanceError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(RebootInstanceError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RebootInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RebootInstanceError {
    fn description(&self) -> &str {
        match *self {
            RebootInstanceError::AccessDenied(ref cause) => cause,
            RebootInstanceError::AccountSetupInProgress(ref cause) => cause,
            RebootInstanceError::InvalidInput(ref cause) => cause,
            RebootInstanceError::NotFound(ref cause) => cause,
            RebootInstanceError::OperationFailure(ref cause) => cause,
            RebootInstanceError::Service(ref cause) => cause,
            RebootInstanceError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by RebootRelationalDatabase
#[derive(Debug, PartialEq)]
pub enum RebootRelationalDatabaseError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl RebootRelationalDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RebootRelationalDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(RebootRelationalDatabaseError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        RebootRelationalDatabaseError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(RebootRelationalDatabaseError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(RebootRelationalDatabaseError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(RebootRelationalDatabaseError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(RebootRelationalDatabaseError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(RebootRelationalDatabaseError::Unauthenticated(
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
impl fmt::Display for RebootRelationalDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RebootRelationalDatabaseError {
    fn description(&self) -> &str {
        match *self {
            RebootRelationalDatabaseError::AccessDenied(ref cause) => cause,
            RebootRelationalDatabaseError::AccountSetupInProgress(ref cause) => cause,
            RebootRelationalDatabaseError::InvalidInput(ref cause) => cause,
            RebootRelationalDatabaseError::NotFound(ref cause) => cause,
            RebootRelationalDatabaseError::OperationFailure(ref cause) => cause,
            RebootRelationalDatabaseError::Service(ref cause) => cause,
            RebootRelationalDatabaseError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by ReleaseStaticIp
#[derive(Debug, PartialEq)]
pub enum ReleaseStaticIpError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl ReleaseStaticIpError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ReleaseStaticIpError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ReleaseStaticIpError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(ReleaseStaticIpError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ReleaseStaticIpError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ReleaseStaticIpError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(ReleaseStaticIpError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ReleaseStaticIpError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(ReleaseStaticIpError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ReleaseStaticIpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ReleaseStaticIpError {
    fn description(&self) -> &str {
        match *self {
            ReleaseStaticIpError::AccessDenied(ref cause) => cause,
            ReleaseStaticIpError::AccountSetupInProgress(ref cause) => cause,
            ReleaseStaticIpError::InvalidInput(ref cause) => cause,
            ReleaseStaticIpError::NotFound(ref cause) => cause,
            ReleaseStaticIpError::OperationFailure(ref cause) => cause,
            ReleaseStaticIpError::Service(ref cause) => cause,
            ReleaseStaticIpError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by StartInstance
#[derive(Debug, PartialEq)]
pub enum StartInstanceError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl StartInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartInstanceError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(StartInstanceError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(StartInstanceError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StartInstanceError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(StartInstanceError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(StartInstanceError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(StartInstanceError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartInstanceError {
    fn description(&self) -> &str {
        match *self {
            StartInstanceError::AccessDenied(ref cause) => cause,
            StartInstanceError::AccountSetupInProgress(ref cause) => cause,
            StartInstanceError::InvalidInput(ref cause) => cause,
            StartInstanceError::NotFound(ref cause) => cause,
            StartInstanceError::OperationFailure(ref cause) => cause,
            StartInstanceError::Service(ref cause) => cause,
            StartInstanceError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by StartRelationalDatabase
#[derive(Debug, PartialEq)]
pub enum StartRelationalDatabaseError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl StartRelationalDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartRelationalDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartRelationalDatabaseError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        StartRelationalDatabaseError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(StartRelationalDatabaseError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StartRelationalDatabaseError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(StartRelationalDatabaseError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(StartRelationalDatabaseError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(StartRelationalDatabaseError::Unauthenticated(
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
impl fmt::Display for StartRelationalDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartRelationalDatabaseError {
    fn description(&self) -> &str {
        match *self {
            StartRelationalDatabaseError::AccessDenied(ref cause) => cause,
            StartRelationalDatabaseError::AccountSetupInProgress(ref cause) => cause,
            StartRelationalDatabaseError::InvalidInput(ref cause) => cause,
            StartRelationalDatabaseError::NotFound(ref cause) => cause,
            StartRelationalDatabaseError::OperationFailure(ref cause) => cause,
            StartRelationalDatabaseError::Service(ref cause) => cause,
            StartRelationalDatabaseError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by StopInstance
#[derive(Debug, PartialEq)]
pub enum StopInstanceError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl StopInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StopInstanceError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(StopInstanceError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(StopInstanceError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StopInstanceError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(StopInstanceError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(StopInstanceError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(StopInstanceError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopInstanceError {
    fn description(&self) -> &str {
        match *self {
            StopInstanceError::AccessDenied(ref cause) => cause,
            StopInstanceError::AccountSetupInProgress(ref cause) => cause,
            StopInstanceError::InvalidInput(ref cause) => cause,
            StopInstanceError::NotFound(ref cause) => cause,
            StopInstanceError::OperationFailure(ref cause) => cause,
            StopInstanceError::Service(ref cause) => cause,
            StopInstanceError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by StopRelationalDatabase
#[derive(Debug, PartialEq)]
pub enum StopRelationalDatabaseError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl StopRelationalDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopRelationalDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StopRelationalDatabaseError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        StopRelationalDatabaseError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(StopRelationalDatabaseError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StopRelationalDatabaseError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(StopRelationalDatabaseError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(StopRelationalDatabaseError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(StopRelationalDatabaseError::Unauthenticated(
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
impl fmt::Display for StopRelationalDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopRelationalDatabaseError {
    fn description(&self) -> &str {
        match *self {
            StopRelationalDatabaseError::AccessDenied(ref cause) => cause,
            StopRelationalDatabaseError::AccountSetupInProgress(ref cause) => cause,
            StopRelationalDatabaseError::InvalidInput(ref cause) => cause,
            StopRelationalDatabaseError::NotFound(ref cause) => cause,
            StopRelationalDatabaseError::OperationFailure(ref cause) => cause,
            StopRelationalDatabaseError::Service(ref cause) => cause,
            StopRelationalDatabaseError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(TagResourceError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(TagResourceError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(TagResourceError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TagResourceError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(TagResourceError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(TagResourceError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(TagResourceError::Unauthenticated(err.msg))
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
            TagResourceError::AccessDenied(ref cause) => cause,
            TagResourceError::AccountSetupInProgress(ref cause) => cause,
            TagResourceError::InvalidInput(ref cause) => cause,
            TagResourceError::NotFound(ref cause) => cause,
            TagResourceError::OperationFailure(ref cause) => cause,
            TagResourceError::Service(ref cause) => cause,
            TagResourceError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by UnpeerVpc
#[derive(Debug, PartialEq)]
pub enum UnpeerVpcError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl UnpeerVpcError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UnpeerVpcError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UnpeerVpcError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(UnpeerVpcError::AccountSetupInProgress(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UnpeerVpcError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UnpeerVpcError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(UnpeerVpcError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(UnpeerVpcError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(UnpeerVpcError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UnpeerVpcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UnpeerVpcError {
    fn description(&self) -> &str {
        match *self {
            UnpeerVpcError::AccessDenied(ref cause) => cause,
            UnpeerVpcError::AccountSetupInProgress(ref cause) => cause,
            UnpeerVpcError::InvalidInput(ref cause) => cause,
            UnpeerVpcError::NotFound(ref cause) => cause,
            UnpeerVpcError::OperationFailure(ref cause) => cause,
            UnpeerVpcError::Service(ref cause) => cause,
            UnpeerVpcError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UntagResourceError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(UntagResourceError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UntagResourceError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UntagResourceError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(UntagResourceError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(UntagResourceError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(UntagResourceError::Unauthenticated(err.msg))
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
            UntagResourceError::AccessDenied(ref cause) => cause,
            UntagResourceError::AccountSetupInProgress(ref cause) => cause,
            UntagResourceError::InvalidInput(ref cause) => cause,
            UntagResourceError::NotFound(ref cause) => cause,
            UntagResourceError::OperationFailure(ref cause) => cause,
            UntagResourceError::Service(ref cause) => cause,
            UntagResourceError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDomainEntry
#[derive(Debug, PartialEq)]
pub enum UpdateDomainEntryError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl UpdateDomainEntryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDomainEntryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateDomainEntryError::AccessDenied(err.msg))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(UpdateDomainEntryError::AccountSetupInProgress(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateDomainEntryError::InvalidInput(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateDomainEntryError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(UpdateDomainEntryError::OperationFailure(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateDomainEntryError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(UpdateDomainEntryError::Unauthenticated(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateDomainEntryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDomainEntryError {
    fn description(&self) -> &str {
        match *self {
            UpdateDomainEntryError::AccessDenied(ref cause) => cause,
            UpdateDomainEntryError::AccountSetupInProgress(ref cause) => cause,
            UpdateDomainEntryError::InvalidInput(ref cause) => cause,
            UpdateDomainEntryError::NotFound(ref cause) => cause,
            UpdateDomainEntryError::OperationFailure(ref cause) => cause,
            UpdateDomainEntryError::Service(ref cause) => cause,
            UpdateDomainEntryError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateLoadBalancerAttribute
#[derive(Debug, PartialEq)]
pub enum UpdateLoadBalancerAttributeError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl UpdateLoadBalancerAttributeError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateLoadBalancerAttributeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateLoadBalancerAttributeError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        UpdateLoadBalancerAttributeError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateLoadBalancerAttributeError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateLoadBalancerAttributeError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        UpdateLoadBalancerAttributeError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateLoadBalancerAttributeError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(UpdateLoadBalancerAttributeError::Unauthenticated(
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
impl fmt::Display for UpdateLoadBalancerAttributeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateLoadBalancerAttributeError {
    fn description(&self) -> &str {
        match *self {
            UpdateLoadBalancerAttributeError::AccessDenied(ref cause) => cause,
            UpdateLoadBalancerAttributeError::AccountSetupInProgress(ref cause) => cause,
            UpdateLoadBalancerAttributeError::InvalidInput(ref cause) => cause,
            UpdateLoadBalancerAttributeError::NotFound(ref cause) => cause,
            UpdateLoadBalancerAttributeError::OperationFailure(ref cause) => cause,
            UpdateLoadBalancerAttributeError::Service(ref cause) => cause,
            UpdateLoadBalancerAttributeError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateRelationalDatabase
#[derive(Debug, PartialEq)]
pub enum UpdateRelationalDatabaseError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl UpdateRelationalDatabaseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRelationalDatabaseError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateRelationalDatabaseError::AccessDenied(
                        err.msg,
                    ))
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        UpdateRelationalDatabaseError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateRelationalDatabaseError::InvalidInput(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateRelationalDatabaseError::NotFound(err.msg))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(UpdateRelationalDatabaseError::OperationFailure(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateRelationalDatabaseError::Service(err.msg))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(UpdateRelationalDatabaseError::Unauthenticated(
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
impl fmt::Display for UpdateRelationalDatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRelationalDatabaseError {
    fn description(&self) -> &str {
        match *self {
            UpdateRelationalDatabaseError::AccessDenied(ref cause) => cause,
            UpdateRelationalDatabaseError::AccountSetupInProgress(ref cause) => cause,
            UpdateRelationalDatabaseError::InvalidInput(ref cause) => cause,
            UpdateRelationalDatabaseError::NotFound(ref cause) => cause,
            UpdateRelationalDatabaseError::OperationFailure(ref cause) => cause,
            UpdateRelationalDatabaseError::Service(ref cause) => cause,
            UpdateRelationalDatabaseError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateRelationalDatabaseParameters
#[derive(Debug, PartialEq)]
pub enum UpdateRelationalDatabaseParametersError {
    /// <p>Lightsail throws this exception when the user cannot be authenticated or uses invalid credentials to access a resource.</p>
    AccessDenied(String),
    /// <p>Lightsail throws this exception when an account is still in the setup in progress state.</p>
    AccountSetupInProgress(String),
    /// <p><p>Lightsail throws this exception when user input does not conform to the validation rules of an input field.</p> <note> <p>Domain-related APIs are only available in the N. Virginia (us-east-1) Region. Please set your AWS Region configuration to us-east-1 to create, view, or edit these resources.</p> </note></p>
    InvalidInput(String),
    /// <p>Lightsail throws this exception when it cannot find a resource.</p>
    NotFound(String),
    /// <p>Lightsail throws this exception when an operation fails to execute.</p>
    OperationFailure(String),
    /// <p>A general service exception.</p>
    Service(String),
    /// <p>Lightsail throws this exception when the user has not been authenticated.</p>
    Unauthenticated(String),
}

impl UpdateRelationalDatabaseParametersError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateRelationalDatabaseParametersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(
                        UpdateRelationalDatabaseParametersError::AccessDenied(err.msg),
                    )
                }
                "AccountSetupInProgressException" => {
                    return RusotoError::Service(
                        UpdateRelationalDatabaseParametersError::AccountSetupInProgress(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(
                        UpdateRelationalDatabaseParametersError::InvalidInput(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateRelationalDatabaseParametersError::NotFound(
                        err.msg,
                    ))
                }
                "OperationFailureException" => {
                    return RusotoError::Service(
                        UpdateRelationalDatabaseParametersError::OperationFailure(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateRelationalDatabaseParametersError::Service(
                        err.msg,
                    ))
                }
                "UnauthenticatedException" => {
                    return RusotoError::Service(
                        UpdateRelationalDatabaseParametersError::Unauthenticated(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateRelationalDatabaseParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRelationalDatabaseParametersError {
    fn description(&self) -> &str {
        match *self {
            UpdateRelationalDatabaseParametersError::AccessDenied(ref cause) => cause,
            UpdateRelationalDatabaseParametersError::AccountSetupInProgress(ref cause) => cause,
            UpdateRelationalDatabaseParametersError::InvalidInput(ref cause) => cause,
            UpdateRelationalDatabaseParametersError::NotFound(ref cause) => cause,
            UpdateRelationalDatabaseParametersError::OperationFailure(ref cause) => cause,
            UpdateRelationalDatabaseParametersError::Service(ref cause) => cause,
            UpdateRelationalDatabaseParametersError::Unauthenticated(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Lightsail API. Amazon Lightsail clients implement this trait.
pub trait Lightsail {
    /// <p>Allocates a static IP address.</p>
    fn allocate_static_ip(
        &self,
        input: AllocateStaticIpRequest,
    ) -> RusotoFuture<AllocateStaticIpResult, AllocateStaticIpError>;

    /// <p>Attaches a block storage disk to a running or stopped Lightsail instance and exposes it to the instance with the specified disk name.</p> <p>The <code>attach disk</code> operation supports tag-based access control via resource tags applied to the resource identified by diskName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn attach_disk(
        &self,
        input: AttachDiskRequest,
    ) -> RusotoFuture<AttachDiskResult, AttachDiskError>;

    /// <p>Attaches one or more Lightsail instances to a load balancer.</p> <p>After some time, the instances are attached to the load balancer and the health check status is available.</p> <p>The <code>attach instances to load balancer</code> operation supports tag-based access control via resource tags applied to the resource identified by loadBalancerName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn attach_instances_to_load_balancer(
        &self,
        input: AttachInstancesToLoadBalancerRequest,
    ) -> RusotoFuture<AttachInstancesToLoadBalancerResult, AttachInstancesToLoadBalancerError>;

    /// <p>Attaches a Transport Layer Security (TLS) certificate to your load balancer. TLS is just an updated, more secure version of Secure Socket Layer (SSL).</p> <p>Once you create and validate your certificate, you can attach it to your load balancer. You can also use this API to rotate the certificates on your account. Use the <code>AttachLoadBalancerTlsCertificate</code> operation with the non-attached certificate, and it will replace the existing one and become the attached certificate.</p> <p>The <code>attach load balancer tls certificate</code> operation supports tag-based access control via resource tags applied to the resource identified by loadBalancerName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn attach_load_balancer_tls_certificate(
        &self,
        input: AttachLoadBalancerTlsCertificateRequest,
    ) -> RusotoFuture<AttachLoadBalancerTlsCertificateResult, AttachLoadBalancerTlsCertificateError>;

    /// <p>Attaches a static IP address to a specific Amazon Lightsail instance.</p>
    fn attach_static_ip(
        &self,
        input: AttachStaticIpRequest,
    ) -> RusotoFuture<AttachStaticIpResult, AttachStaticIpError>;

    /// <p>Closes the public ports on a specific Amazon Lightsail instance.</p> <p>The <code>close instance public ports</code> operation supports tag-based access control via resource tags applied to the resource identified by instanceName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn close_instance_public_ports(
        &self,
        input: CloseInstancePublicPortsRequest,
    ) -> RusotoFuture<CloseInstancePublicPortsResult, CloseInstancePublicPortsError>;

    /// <p>Copies an instance or disk snapshot from one AWS Region to another in Amazon Lightsail.</p>
    fn copy_snapshot(
        &self,
        input: CopySnapshotRequest,
    ) -> RusotoFuture<CopySnapshotResult, CopySnapshotError>;

    /// <p><p>Creates an AWS CloudFormation stack, which creates a new Amazon EC2 instance from an exported Amazon Lightsail snapshot. This operation results in a CloudFormation stack record that can be used to track the AWS CloudFormation stack created. Use the <code>get cloud formation stack records</code> operation to get a list of the CloudFormation stacks created.</p> <important> <p>Wait until after your new Amazon EC2 instance is created before running the <code>create cloud formation stack</code> operation again with the same export snapshot record.</p> </important></p>
    fn create_cloud_formation_stack(
        &self,
        input: CreateCloudFormationStackRequest,
    ) -> RusotoFuture<CreateCloudFormationStackResult, CreateCloudFormationStackError>;

    /// <p>Creates a block storage disk that can be attached to a Lightsail instance in the same Availability Zone (e.g., <code>us-east-2a</code>). The disk is created in the regional endpoint that you send the HTTP request to. For more information, see <a href="https://lightsail.aws.amazon.com/ls/docs/overview/article/understanding-regions-and-availability-zones-in-amazon-lightsail">Regions and Availability Zones in Lightsail</a>.</p> <p>The <code>create disk</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_disk(
        &self,
        input: CreateDiskRequest,
    ) -> RusotoFuture<CreateDiskResult, CreateDiskError>;

    /// <p>Creates a block storage disk from a disk snapshot that can be attached to a Lightsail instance in the same Availability Zone (e.g., <code>us-east-2a</code>). The disk is created in the regional endpoint that you send the HTTP request to. For more information, see <a href="https://lightsail.aws.amazon.com/ls/docs/overview/article/understanding-regions-and-availability-zones-in-amazon-lightsail">Regions and Availability Zones in Lightsail</a>.</p> <p>The <code>create disk from snapshot</code> operation supports tag-based access control via request tags and resource tags applied to the resource identified by diskSnapshotName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_disk_from_snapshot(
        &self,
        input: CreateDiskFromSnapshotRequest,
    ) -> RusotoFuture<CreateDiskFromSnapshotResult, CreateDiskFromSnapshotError>;

    /// <p>Creates a snapshot of a block storage disk. You can use snapshots for backups, to make copies of disks, and to save data before shutting down a Lightsail instance.</p> <p>You can take a snapshot of an attached disk that is in use; however, snapshots only capture data that has been written to your disk at the time the snapshot command is issued. This may exclude any data that has been cached by any applications or the operating system. If you can pause any file systems on the disk long enough to take a snapshot, your snapshot should be complete. Nevertheless, if you cannot pause all file writes to the disk, you should unmount the disk from within the Lightsail instance, issue the create disk snapshot command, and then remount the disk to ensure a consistent and complete snapshot. You may remount and use your disk while the snapshot status is pending.</p> <p>You can also use this operation to create a snapshot of an instance's system volume. You might want to do this, for example, to recover data from the system volume of a botched instance or to create a backup of the system volume like you would for a block storage disk. To create a snapshot of a system volume, just define the <code>instance name</code> parameter when issuing the snapshot command, and a snapshot of the defined instance's system volume will be created. After the snapshot is available, you can create a block storage disk from the snapshot and attach it to a running instance to access the data on the disk.</p> <p>The <code>create disk snapshot</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_disk_snapshot(
        &self,
        input: CreateDiskSnapshotRequest,
    ) -> RusotoFuture<CreateDiskSnapshotResult, CreateDiskSnapshotError>;

    /// <p>Creates a domain resource for the specified domain (e.g., example.com).</p> <p>The <code>create domain</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_domain(
        &self,
        input: CreateDomainRequest,
    ) -> RusotoFuture<CreateDomainResult, CreateDomainError>;

    /// <p>Creates one of the following entry records associated with the domain: Address (A), canonical name (CNAME), mail exchanger (MX), name server (NS), start of authority (SOA), service locator (SRV), or text (TXT).</p> <p>The <code>create domain entry</code> operation supports tag-based access control via resource tags applied to the resource identified by domainName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_domain_entry(
        &self,
        input: CreateDomainEntryRequest,
    ) -> RusotoFuture<CreateDomainEntryResult, CreateDomainEntryError>;

    /// <p>Creates a snapshot of a specific virtual private server, or <i>instance</i>. You can use a snapshot to create a new instance that is based on that snapshot.</p> <p>The <code>create instance snapshot</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_instance_snapshot(
        &self,
        input: CreateInstanceSnapshotRequest,
    ) -> RusotoFuture<CreateInstanceSnapshotResult, CreateInstanceSnapshotError>;

    /// <p>Creates one or more Amazon Lightsail virtual private servers, or <i>instances</i>. Create instances using active blueprints. Inactive blueprints are listed to support customers with existing instances but are not necessarily available for launch of new instances. Blueprints are marked inactive when they become outdated due to operating system updates or new application releases. Use the get blueprints operation to return a list of available blueprints.</p> <p>The <code>create instances</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_instances(
        &self,
        input: CreateInstancesRequest,
    ) -> RusotoFuture<CreateInstancesResult, CreateInstancesError>;

    /// <p>Uses a specific snapshot as a blueprint for creating one or more new instances that are based on that identical configuration.</p> <p>The <code>create instances from snapshot</code> operation supports tag-based access control via request tags and resource tags applied to the resource identified by instanceSnapshotName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_instances_from_snapshot(
        &self,
        input: CreateInstancesFromSnapshotRequest,
    ) -> RusotoFuture<CreateInstancesFromSnapshotResult, CreateInstancesFromSnapshotError>;

    /// <p>Creates an SSH key pair.</p> <p>The <code>create key pair</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_key_pair(
        &self,
        input: CreateKeyPairRequest,
    ) -> RusotoFuture<CreateKeyPairResult, CreateKeyPairError>;

    /// <p>Creates a Lightsail load balancer. To learn more about deciding whether to load balance your application, see <a href="https://lightsail.aws.amazon.com/ls/docs/how-to/article/configure-lightsail-instances-for-load-balancing">Configure your Lightsail instances for load balancing</a>. You can create up to 5 load balancers per AWS Region in your account.</p> <p>When you create a load balancer, you can specify a unique name and port settings. To change additional load balancer settings, use the <code>UpdateLoadBalancerAttribute</code> operation.</p> <p>The <code>create load balancer</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_load_balancer(
        &self,
        input: CreateLoadBalancerRequest,
    ) -> RusotoFuture<CreateLoadBalancerResult, CreateLoadBalancerError>;

    /// <p>Creates a Lightsail load balancer TLS certificate.</p> <p>TLS is just an updated, more secure version of Secure Socket Layer (SSL).</p> <p>The <code>create load balancer tls certificate</code> operation supports tag-based access control via resource tags applied to the resource identified by loadBalancerName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_load_balancer_tls_certificate(
        &self,
        input: CreateLoadBalancerTlsCertificateRequest,
    ) -> RusotoFuture<CreateLoadBalancerTlsCertificateResult, CreateLoadBalancerTlsCertificateError>;

    /// <p>Creates a new database in Amazon Lightsail.</p> <p>The <code>create relational database</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_relational_database(
        &self,
        input: CreateRelationalDatabaseRequest,
    ) -> RusotoFuture<CreateRelationalDatabaseResult, CreateRelationalDatabaseError>;

    /// <p>Creates a new database from an existing database snapshot in Amazon Lightsail.</p> <p>You can create a new database from a snapshot in if something goes wrong with your original database, or to change it to a different plan, such as a high availability or standard plan.</p> <p>The <code>create relational database from snapshot</code> operation supports tag-based access control via request tags and resource tags applied to the resource identified by relationalDatabaseSnapshotName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_relational_database_from_snapshot(
        &self,
        input: CreateRelationalDatabaseFromSnapshotRequest,
    ) -> RusotoFuture<
        CreateRelationalDatabaseFromSnapshotResult,
        CreateRelationalDatabaseFromSnapshotError,
    >;

    /// <p>Creates a snapshot of your database in Amazon Lightsail. You can use snapshots for backups, to make copies of a database, and to save data before deleting a database.</p> <p>The <code>create relational database snapshot</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_relational_database_snapshot(
        &self,
        input: CreateRelationalDatabaseSnapshotRequest,
    ) -> RusotoFuture<CreateRelationalDatabaseSnapshotResult, CreateRelationalDatabaseSnapshotError>;

    /// <p>Deletes the specified block storage disk. The disk must be in the <code>available</code> state (not attached to a Lightsail instance).</p> <note> <p>The disk may remain in the <code>deleting</code> state for several minutes.</p> </note> <p>The <code>delete disk</code> operation supports tag-based access control via resource tags applied to the resource identified by diskName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn delete_disk(
        &self,
        input: DeleteDiskRequest,
    ) -> RusotoFuture<DeleteDiskResult, DeleteDiskError>;

    /// <p>Deletes the specified disk snapshot.</p> <p>When you make periodic snapshots of a disk, the snapshots are incremental, and only the blocks on the device that have changed since your last snapshot are saved in the new snapshot. When you delete a snapshot, only the data not needed for any other snapshot is removed. So regardless of which prior snapshots have been deleted, all active snapshots will have access to all the information needed to restore the disk.</p> <p>The <code>delete disk snapshot</code> operation supports tag-based access control via resource tags applied to the resource identified by diskSnapshotName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn delete_disk_snapshot(
        &self,
        input: DeleteDiskSnapshotRequest,
    ) -> RusotoFuture<DeleteDiskSnapshotResult, DeleteDiskSnapshotError>;

    /// <p>Deletes the specified domain recordset and all of its domain records.</p> <p>The <code>delete domain</code> operation supports tag-based access control via resource tags applied to the resource identified by domainName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn delete_domain(
        &self,
        input: DeleteDomainRequest,
    ) -> RusotoFuture<DeleteDomainResult, DeleteDomainError>;

    /// <p>Deletes a specific domain entry.</p> <p>The <code>delete domain entry</code> operation supports tag-based access control via resource tags applied to the resource identified by domainName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn delete_domain_entry(
        &self,
        input: DeleteDomainEntryRequest,
    ) -> RusotoFuture<DeleteDomainEntryResult, DeleteDomainEntryError>;

    /// <p>Deletes a specific Amazon Lightsail virtual private server, or <i>instance</i>.</p> <p>The <code>delete instance</code> operation supports tag-based access control via resource tags applied to the resource identified by instanceName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn delete_instance(
        &self,
        input: DeleteInstanceRequest,
    ) -> RusotoFuture<DeleteInstanceResult, DeleteInstanceError>;

    /// <p>Deletes a specific snapshot of a virtual private server (or <i>instance</i>).</p> <p>The <code>delete instance snapshot</code> operation supports tag-based access control via resource tags applied to the resource identified by instanceSnapshotName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn delete_instance_snapshot(
        &self,
        input: DeleteInstanceSnapshotRequest,
    ) -> RusotoFuture<DeleteInstanceSnapshotResult, DeleteInstanceSnapshotError>;

    /// <p>Deletes a specific SSH key pair.</p> <p>The <code>delete key pair</code> operation supports tag-based access control via resource tags applied to the resource identified by keyPairName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn delete_key_pair(
        &self,
        input: DeleteKeyPairRequest,
    ) -> RusotoFuture<DeleteKeyPairResult, DeleteKeyPairError>;

    /// <p><p>Deletes the known host key or certificate used by the Amazon Lightsail browser-based SSH or RDP clients to authenticate an instance. This operation enables the Lightsail browser-based SSH or RDP clients to connect to the instance after a host key mismatch.</p> <important> <p>Perform this operation only if you were expecting the host key or certificate mismatch or if you are familiar with the new host key or certificate on the instance. For more information, see <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-troubleshooting-browser-based-ssh-rdp-client-connection">Troubleshooting connection issues when using the Amazon Lightsail browser-based SSH or RDP client</a>.</p> </important></p>
    fn delete_known_host_keys(
        &self,
        input: DeleteKnownHostKeysRequest,
    ) -> RusotoFuture<DeleteKnownHostKeysResult, DeleteKnownHostKeysError>;

    /// <p>Deletes a Lightsail load balancer and all its associated SSL/TLS certificates. Once the load balancer is deleted, you will need to create a new load balancer, create a new certificate, and verify domain ownership again.</p> <p>The <code>delete load balancer</code> operation supports tag-based access control via resource tags applied to the resource identified by loadBalancerName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn delete_load_balancer(
        &self,
        input: DeleteLoadBalancerRequest,
    ) -> RusotoFuture<DeleteLoadBalancerResult, DeleteLoadBalancerError>;

    /// <p>Deletes an SSL/TLS certificate associated with a Lightsail load balancer.</p> <p>The <code>delete load balancer tls certificate</code> operation supports tag-based access control via resource tags applied to the resource identified by loadBalancerName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn delete_load_balancer_tls_certificate(
        &self,
        input: DeleteLoadBalancerTlsCertificateRequest,
    ) -> RusotoFuture<DeleteLoadBalancerTlsCertificateResult, DeleteLoadBalancerTlsCertificateError>;

    /// <p>Deletes a database in Amazon Lightsail.</p> <p>The <code>delete relational database</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn delete_relational_database(
        &self,
        input: DeleteRelationalDatabaseRequest,
    ) -> RusotoFuture<DeleteRelationalDatabaseResult, DeleteRelationalDatabaseError>;

    /// <p>Deletes a database snapshot in Amazon Lightsail.</p> <p>The <code>delete relational database snapshot</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn delete_relational_database_snapshot(
        &self,
        input: DeleteRelationalDatabaseSnapshotRequest,
    ) -> RusotoFuture<DeleteRelationalDatabaseSnapshotResult, DeleteRelationalDatabaseSnapshotError>;

    /// <p>Detaches a stopped block storage disk from a Lightsail instance. Make sure to unmount any file systems on the device within your operating system before stopping the instance and detaching the disk.</p> <p>The <code>detach disk</code> operation supports tag-based access control via resource tags applied to the resource identified by diskName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn detach_disk(
        &self,
        input: DetachDiskRequest,
    ) -> RusotoFuture<DetachDiskResult, DetachDiskError>;

    /// <p>Detaches the specified instances from a Lightsail load balancer.</p> <p>This operation waits until the instances are no longer needed before they are detached from the load balancer.</p> <p>The <code>detach instances from load balancer</code> operation supports tag-based access control via resource tags applied to the resource identified by loadBalancerName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn detach_instances_from_load_balancer(
        &self,
        input: DetachInstancesFromLoadBalancerRequest,
    ) -> RusotoFuture<DetachInstancesFromLoadBalancerResult, DetachInstancesFromLoadBalancerError>;

    /// <p>Detaches a static IP from the Amazon Lightsail instance to which it is attached.</p>
    fn detach_static_ip(
        &self,
        input: DetachStaticIpRequest,
    ) -> RusotoFuture<DetachStaticIpResult, DetachStaticIpError>;

    /// <p>Downloads the default SSH key pair from the user's account.</p>
    fn download_default_key_pair(
        &self,
    ) -> RusotoFuture<DownloadDefaultKeyPairResult, DownloadDefaultKeyPairError>;

    /// <p><p>Exports an Amazon Lightsail instance or block storage disk snapshot to Amazon Elastic Compute Cloud (Amazon EC2). This operation results in an export snapshot record that can be used with the <code>create cloud formation stack</code> operation to create new Amazon EC2 instances.</p> <p>Exported instance snapshots appear in Amazon EC2 as Amazon Machine Images (AMIs), and the instance system disk appears as an Amazon Elastic Block Store (Amazon EBS) volume. Exported disk snapshots appear in Amazon EC2 as Amazon EBS volumes. Snapshots are exported to the same Amazon Web Services Region in Amazon EC2 as the source Lightsail snapshot.</p> <p/> <p>The <code>export snapshot</code> operation supports tag-based access control via resource tags applied to the resource identified by sourceSnapshotName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p> <note> <p>Use the <code>get instance snapshots</code> or <code>get disk snapshots</code> operations to get a list of snapshots that you can export to Amazon EC2.</p> </note></p>
    fn export_snapshot(
        &self,
        input: ExportSnapshotRequest,
    ) -> RusotoFuture<ExportSnapshotResult, ExportSnapshotError>;

    /// <p>Returns the names of all active (not deleted) resources.</p>
    fn get_active_names(
        &self,
        input: GetActiveNamesRequest,
    ) -> RusotoFuture<GetActiveNamesResult, GetActiveNamesError>;

    /// <p>Returns the list of available instance images, or <i>blueprints</i>. You can use a blueprint to create a new virtual private server already running a specific operating system, as well as a preinstalled app or development stack. The software each instance is running depends on the blueprint image you choose.</p>
    fn get_blueprints(
        &self,
        input: GetBlueprintsRequest,
    ) -> RusotoFuture<GetBlueprintsResult, GetBlueprintsError>;

    /// <p>Returns the list of bundles that are available for purchase. A bundle describes the specs for your virtual private server (or <i>instance</i>).</p>
    fn get_bundles(
        &self,
        input: GetBundlesRequest,
    ) -> RusotoFuture<GetBundlesResult, GetBundlesError>;

    /// <p>Returns the CloudFormation stack record created as a result of the <code>create cloud formation stack</code> operation.</p> <p>An AWS CloudFormation stack is used to create a new Amazon EC2 instance from an exported Lightsail snapshot.</p>
    fn get_cloud_formation_stack_records(
        &self,
        input: GetCloudFormationStackRecordsRequest,
    ) -> RusotoFuture<GetCloudFormationStackRecordsResult, GetCloudFormationStackRecordsError>;

    /// <p>Returns information about a specific block storage disk.</p>
    fn get_disk(&self, input: GetDiskRequest) -> RusotoFuture<GetDiskResult, GetDiskError>;

    /// <p>Returns information about a specific block storage disk snapshot.</p>
    fn get_disk_snapshot(
        &self,
        input: GetDiskSnapshotRequest,
    ) -> RusotoFuture<GetDiskSnapshotResult, GetDiskSnapshotError>;

    /// <p>Returns information about all block storage disk snapshots in your AWS account and region.</p> <p>If you are describing a long list of disk snapshots, you can paginate the output to make the list more manageable. You can use the pageToken and nextPageToken values to retrieve the next items in the list.</p>
    fn get_disk_snapshots(
        &self,
        input: GetDiskSnapshotsRequest,
    ) -> RusotoFuture<GetDiskSnapshotsResult, GetDiskSnapshotsError>;

    /// <p>Returns information about all block storage disks in your AWS account and region.</p> <p>If you are describing a long list of disks, you can paginate the output to make the list more manageable. You can use the pageToken and nextPageToken values to retrieve the next items in the list.</p>
    fn get_disks(&self, input: GetDisksRequest) -> RusotoFuture<GetDisksResult, GetDisksError>;

    /// <p>Returns information about a specific domain recordset.</p>
    fn get_domain(&self, input: GetDomainRequest) -> RusotoFuture<GetDomainResult, GetDomainError>;

    /// <p>Returns a list of all domains in the user's account.</p>
    fn get_domains(
        &self,
        input: GetDomainsRequest,
    ) -> RusotoFuture<GetDomainsResult, GetDomainsError>;

    /// <p>Returns the export snapshot record created as a result of the <code>export snapshot</code> operation.</p> <p>An export snapshot record can be used to create a new Amazon EC2 instance and its related resources with the <code>create cloud formation stack</code> operation.</p>
    fn get_export_snapshot_records(
        &self,
        input: GetExportSnapshotRecordsRequest,
    ) -> RusotoFuture<GetExportSnapshotRecordsResult, GetExportSnapshotRecordsError>;

    /// <p>Returns information about a specific Amazon Lightsail instance, which is a virtual private server.</p>
    fn get_instance(
        &self,
        input: GetInstanceRequest,
    ) -> RusotoFuture<GetInstanceResult, GetInstanceError>;

    /// <p>Returns temporary SSH keys you can use to connect to a specific virtual private server, or <i>instance</i>.</p> <p>The <code>get instance access details</code> operation supports tag-based access control via resource tags applied to the resource identified by instanceName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn get_instance_access_details(
        &self,
        input: GetInstanceAccessDetailsRequest,
    ) -> RusotoFuture<GetInstanceAccessDetailsResult, GetInstanceAccessDetailsError>;

    /// <p>Returns the data points for the specified Amazon Lightsail instance metric, given an instance name.</p>
    fn get_instance_metric_data(
        &self,
        input: GetInstanceMetricDataRequest,
    ) -> RusotoFuture<GetInstanceMetricDataResult, GetInstanceMetricDataError>;

    /// <p>Returns the port states for a specific virtual private server, or <i>instance</i>.</p>
    fn get_instance_port_states(
        &self,
        input: GetInstancePortStatesRequest,
    ) -> RusotoFuture<GetInstancePortStatesResult, GetInstancePortStatesError>;

    /// <p>Returns information about a specific instance snapshot.</p>
    fn get_instance_snapshot(
        &self,
        input: GetInstanceSnapshotRequest,
    ) -> RusotoFuture<GetInstanceSnapshotResult, GetInstanceSnapshotError>;

    /// <p>Returns all instance snapshots for the user's account.</p>
    fn get_instance_snapshots(
        &self,
        input: GetInstanceSnapshotsRequest,
    ) -> RusotoFuture<GetInstanceSnapshotsResult, GetInstanceSnapshotsError>;

    /// <p>Returns the state of a specific instance. Works on one instance at a time.</p>
    fn get_instance_state(
        &self,
        input: GetInstanceStateRequest,
    ) -> RusotoFuture<GetInstanceStateResult, GetInstanceStateError>;

    /// <p>Returns information about all Amazon Lightsail virtual private servers, or <i>instances</i>.</p>
    fn get_instances(
        &self,
        input: GetInstancesRequest,
    ) -> RusotoFuture<GetInstancesResult, GetInstancesError>;

    /// <p>Returns information about a specific key pair.</p>
    fn get_key_pair(
        &self,
        input: GetKeyPairRequest,
    ) -> RusotoFuture<GetKeyPairResult, GetKeyPairError>;

    /// <p>Returns information about all key pairs in the user's account.</p>
    fn get_key_pairs(
        &self,
        input: GetKeyPairsRequest,
    ) -> RusotoFuture<GetKeyPairsResult, GetKeyPairsError>;

    /// <p>Returns information about the specified Lightsail load balancer.</p>
    fn get_load_balancer(
        &self,
        input: GetLoadBalancerRequest,
    ) -> RusotoFuture<GetLoadBalancerResult, GetLoadBalancerError>;

    /// <p>Returns information about health metrics for your Lightsail load balancer.</p>
    fn get_load_balancer_metric_data(
        &self,
        input: GetLoadBalancerMetricDataRequest,
    ) -> RusotoFuture<GetLoadBalancerMetricDataResult, GetLoadBalancerMetricDataError>;

    /// <p>Returns information about the TLS certificates that are associated with the specified Lightsail load balancer.</p> <p>TLS is just an updated, more secure version of Secure Socket Layer (SSL).</p> <p>You can have a maximum of 2 certificates associated with a Lightsail load balancer. One is active and the other is inactive.</p>
    fn get_load_balancer_tls_certificates(
        &self,
        input: GetLoadBalancerTlsCertificatesRequest,
    ) -> RusotoFuture<GetLoadBalancerTlsCertificatesResult, GetLoadBalancerTlsCertificatesError>;

    /// <p>Returns information about all load balancers in an account.</p> <p>If you are describing a long list of load balancers, you can paginate the output to make the list more manageable. You can use the pageToken and nextPageToken values to retrieve the next items in the list.</p>
    fn get_load_balancers(
        &self,
        input: GetLoadBalancersRequest,
    ) -> RusotoFuture<GetLoadBalancersResult, GetLoadBalancersError>;

    /// <p>Returns information about a specific operation. Operations include events such as when you create an instance, allocate a static IP, attach a static IP, and so on.</p>
    fn get_operation(
        &self,
        input: GetOperationRequest,
    ) -> RusotoFuture<GetOperationResult, GetOperationError>;

    /// <p>Returns information about all operations.</p> <p>Results are returned from oldest to newest, up to a maximum of 200. Results can be paged by making each subsequent call to <code>GetOperations</code> use the maximum (last) <code>statusChangedAt</code> value from the previous request.</p>
    fn get_operations(
        &self,
        input: GetOperationsRequest,
    ) -> RusotoFuture<GetOperationsResult, GetOperationsError>;

    /// <p>Gets operations for a specific resource (e.g., an instance or a static IP).</p>
    fn get_operations_for_resource(
        &self,
        input: GetOperationsForResourceRequest,
    ) -> RusotoFuture<GetOperationsForResourceResult, GetOperationsForResourceError>;

    /// <p>Returns a list of all valid regions for Amazon Lightsail. Use the <code>include availability zones</code> parameter to also return the Availability Zones in a region.</p>
    fn get_regions(
        &self,
        input: GetRegionsRequest,
    ) -> RusotoFuture<GetRegionsResult, GetRegionsError>;

    /// <p>Returns information about a specific database in Amazon Lightsail.</p>
    fn get_relational_database(
        &self,
        input: GetRelationalDatabaseRequest,
    ) -> RusotoFuture<GetRelationalDatabaseResult, GetRelationalDatabaseError>;

    /// <p>Returns a list of available database blueprints in Amazon Lightsail. A blueprint describes the major engine version of a database.</p> <p>You can use a blueprint ID to create a new database that runs a specific database engine.</p>
    fn get_relational_database_blueprints(
        &self,
        input: GetRelationalDatabaseBlueprintsRequest,
    ) -> RusotoFuture<GetRelationalDatabaseBlueprintsResult, GetRelationalDatabaseBlueprintsError>;

    /// <p>Returns the list of bundles that are available in Amazon Lightsail. A bundle describes the performance specifications for a database.</p> <p>You can use a bundle ID to create a new database with explicit performance specifications.</p>
    fn get_relational_database_bundles(
        &self,
        input: GetRelationalDatabaseBundlesRequest,
    ) -> RusotoFuture<GetRelationalDatabaseBundlesResult, GetRelationalDatabaseBundlesError>;

    /// <p>Returns a list of events for a specific database in Amazon Lightsail.</p>
    fn get_relational_database_events(
        &self,
        input: GetRelationalDatabaseEventsRequest,
    ) -> RusotoFuture<GetRelationalDatabaseEventsResult, GetRelationalDatabaseEventsError>;

    /// <p>Returns a list of log events for a database in Amazon Lightsail.</p>
    fn get_relational_database_log_events(
        &self,
        input: GetRelationalDatabaseLogEventsRequest,
    ) -> RusotoFuture<GetRelationalDatabaseLogEventsResult, GetRelationalDatabaseLogEventsError>;

    /// <p>Returns a list of available log streams for a specific database in Amazon Lightsail.</p>
    fn get_relational_database_log_streams(
        &self,
        input: GetRelationalDatabaseLogStreamsRequest,
    ) -> RusotoFuture<GetRelationalDatabaseLogStreamsResult, GetRelationalDatabaseLogStreamsError>;

    /// <p>Returns the current, previous, or pending versions of the master user password for a Lightsail database.</p> <p>The <code>asdf</code> operation GetRelationalDatabaseMasterUserPassword supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName.</p>
    fn get_relational_database_master_user_password(
        &self,
        input: GetRelationalDatabaseMasterUserPasswordRequest,
    ) -> RusotoFuture<
        GetRelationalDatabaseMasterUserPasswordResult,
        GetRelationalDatabaseMasterUserPasswordError,
    >;

    /// <p>Returns the data points of the specified metric for a database in Amazon Lightsail.</p>
    fn get_relational_database_metric_data(
        &self,
        input: GetRelationalDatabaseMetricDataRequest,
    ) -> RusotoFuture<GetRelationalDatabaseMetricDataResult, GetRelationalDatabaseMetricDataError>;

    /// <p>Returns all of the runtime parameters offered by the underlying database software, or engine, for a specific database in Amazon Lightsail.</p> <p>In addition to the parameter names and values, this operation returns other information about each parameter. This information includes whether changes require a reboot, whether the parameter is modifiable, the allowed values, and the data types.</p>
    fn get_relational_database_parameters(
        &self,
        input: GetRelationalDatabaseParametersRequest,
    ) -> RusotoFuture<GetRelationalDatabaseParametersResult, GetRelationalDatabaseParametersError>;

    /// <p>Returns information about a specific database snapshot in Amazon Lightsail.</p>
    fn get_relational_database_snapshot(
        &self,
        input: GetRelationalDatabaseSnapshotRequest,
    ) -> RusotoFuture<GetRelationalDatabaseSnapshotResult, GetRelationalDatabaseSnapshotError>;

    /// <p>Returns information about all of your database snapshots in Amazon Lightsail.</p>
    fn get_relational_database_snapshots(
        &self,
        input: GetRelationalDatabaseSnapshotsRequest,
    ) -> RusotoFuture<GetRelationalDatabaseSnapshotsResult, GetRelationalDatabaseSnapshotsError>;

    /// <p>Returns information about all of your databases in Amazon Lightsail.</p>
    fn get_relational_databases(
        &self,
        input: GetRelationalDatabasesRequest,
    ) -> RusotoFuture<GetRelationalDatabasesResult, GetRelationalDatabasesError>;

    /// <p>Returns information about a specific static IP.</p>
    fn get_static_ip(
        &self,
        input: GetStaticIpRequest,
    ) -> RusotoFuture<GetStaticIpResult, GetStaticIpError>;

    /// <p>Returns information about all static IPs in the user's account.</p>
    fn get_static_ips(
        &self,
        input: GetStaticIpsRequest,
    ) -> RusotoFuture<GetStaticIpsResult, GetStaticIpsError>;

    /// <p>Imports a public SSH key from a specific key pair.</p>
    fn import_key_pair(
        &self,
        input: ImportKeyPairRequest,
    ) -> RusotoFuture<ImportKeyPairResult, ImportKeyPairError>;

    /// <p>Returns a Boolean value indicating whether your Lightsail VPC is peered.</p>
    fn is_vpc_peered(&self) -> RusotoFuture<IsVpcPeeredResult, IsVpcPeeredError>;

    /// <p>Adds public ports to an Amazon Lightsail instance.</p> <p>The <code>open instance public ports</code> operation supports tag-based access control via resource tags applied to the resource identified by instanceName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn open_instance_public_ports(
        &self,
        input: OpenInstancePublicPortsRequest,
    ) -> RusotoFuture<OpenInstancePublicPortsResult, OpenInstancePublicPortsError>;

    /// <p>Tries to peer the Lightsail VPC with the user's default VPC.</p>
    fn peer_vpc(&self) -> RusotoFuture<PeerVpcResult, PeerVpcError>;

    /// <p>Sets the specified open ports for an Amazon Lightsail instance, and closes all ports for every protocol not included in the current request.</p> <p>The <code>put instance public ports</code> operation supports tag-based access control via resource tags applied to the resource identified by instanceName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn put_instance_public_ports(
        &self,
        input: PutInstancePublicPortsRequest,
    ) -> RusotoFuture<PutInstancePublicPortsResult, PutInstancePublicPortsError>;

    /// <p>Restarts a specific instance.</p> <p>The <code>reboot instance</code> operation supports tag-based access control via resource tags applied to the resource identified by instanceName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn reboot_instance(
        &self,
        input: RebootInstanceRequest,
    ) -> RusotoFuture<RebootInstanceResult, RebootInstanceError>;

    /// <p>Restarts a specific database in Amazon Lightsail.</p> <p>The <code>reboot relational database</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn reboot_relational_database(
        &self,
        input: RebootRelationalDatabaseRequest,
    ) -> RusotoFuture<RebootRelationalDatabaseResult, RebootRelationalDatabaseError>;

    /// <p>Deletes a specific static IP from your account.</p>
    fn release_static_ip(
        &self,
        input: ReleaseStaticIpRequest,
    ) -> RusotoFuture<ReleaseStaticIpResult, ReleaseStaticIpError>;

    /// <p>Starts a specific Amazon Lightsail instance from a stopped state. To restart an instance, use the <code>reboot instance</code> operation.</p> <note> <p>When you start a stopped instance, Lightsail assigns a new public IP address to the instance. To use the same IP address after stopping and starting an instance, create a static IP address and attach it to the instance. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/lightsail-create-static-ip">Lightsail Dev Guide</a>.</p> </note> <p>The <code>start instance</code> operation supports tag-based access control via resource tags applied to the resource identified by instanceName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn start_instance(
        &self,
        input: StartInstanceRequest,
    ) -> RusotoFuture<StartInstanceResult, StartInstanceError>;

    /// <p>Starts a specific database from a stopped state in Amazon Lightsail. To restart a database, use the <code>reboot relational database</code> operation.</p> <p>The <code>start relational database</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn start_relational_database(
        &self,
        input: StartRelationalDatabaseRequest,
    ) -> RusotoFuture<StartRelationalDatabaseResult, StartRelationalDatabaseError>;

    /// <p>Stops a specific Amazon Lightsail instance that is currently running.</p> <note> <p>When you start a stopped instance, Lightsail assigns a new public IP address to the instance. To use the same IP address after stopping and starting an instance, create a static IP address and attach it to the instance. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/lightsail-create-static-ip">Lightsail Dev Guide</a>.</p> </note> <p>The <code>stop instance</code> operation supports tag-based access control via resource tags applied to the resource identified by instanceName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn stop_instance(
        &self,
        input: StopInstanceRequest,
    ) -> RusotoFuture<StopInstanceResult, StopInstanceError>;

    /// <p>Stops a specific database that is currently running in Amazon Lightsail.</p> <p>The <code>stop relational database</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn stop_relational_database(
        &self,
        input: StopRelationalDatabaseRequest,
    ) -> RusotoFuture<StopRelationalDatabaseResult, StopRelationalDatabaseError>;

    /// <p>Adds one or more tags to the specified Amazon Lightsail resource. Each resource can have a maximum of 50 tags. Each tag consists of a key and an optional value. Tag keys must be unique per resource. For more information about tags, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p> <p>The <code>tag resource</code> operation supports tag-based access control via request tags and resource tags applied to the resource identified by resourceName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResult, TagResourceError>;

    /// <p>Attempts to unpeer the Lightsail VPC from the user's default VPC.</p>
    fn unpeer_vpc(&self) -> RusotoFuture<UnpeerVpcResult, UnpeerVpcError>;

    /// <p>Deletes the specified set of tag keys and their values from the specified Amazon Lightsail resource.</p> <p>The <code>untag resource</code> operation supports tag-based access control via request tags and resource tags applied to the resource identified by resourceName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResult, UntagResourceError>;

    /// <p>Updates a domain recordset after it is created.</p> <p>The <code>update domain entry</code> operation supports tag-based access control via resource tags applied to the resource identified by domainName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn update_domain_entry(
        &self,
        input: UpdateDomainEntryRequest,
    ) -> RusotoFuture<UpdateDomainEntryResult, UpdateDomainEntryError>;

    /// <p>Updates the specified attribute for a load balancer. You can only update one attribute at a time.</p> <p>The <code>update load balancer attribute</code> operation supports tag-based access control via resource tags applied to the resource identified by loadBalancerName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn update_load_balancer_attribute(
        &self,
        input: UpdateLoadBalancerAttributeRequest,
    ) -> RusotoFuture<UpdateLoadBalancerAttributeResult, UpdateLoadBalancerAttributeError>;

    /// <p>Allows the update of one or more attributes of a database in Amazon Lightsail.</p> <p>Updates are applied immediately, or in cases where the updates could result in an outage, are applied during the database's predefined maintenance window.</p> <p>The <code>update relational database</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn update_relational_database(
        &self,
        input: UpdateRelationalDatabaseRequest,
    ) -> RusotoFuture<UpdateRelationalDatabaseResult, UpdateRelationalDatabaseError>;

    /// <p>Allows the update of one or more parameters of a database in Amazon Lightsail.</p> <p>Parameter updates don't cause outages; therefore, their application is not subject to the preferred maintenance window. However, there are two ways in which paramater updates are applied: <code>dynamic</code> or <code>pending-reboot</code>. Parameters marked with a <code>dynamic</code> apply type are applied immediately. Parameters marked with a <code>pending-reboot</code> apply type are applied only after the database is rebooted using the <code>reboot relational database</code> operation.</p> <p>The <code>update relational database parameters</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn update_relational_database_parameters(
        &self,
        input: UpdateRelationalDatabaseParametersRequest,
    ) -> RusotoFuture<
        UpdateRelationalDatabaseParametersResult,
        UpdateRelationalDatabaseParametersError,
    >;
}
/// A client for the Amazon Lightsail API.
#[derive(Clone)]
pub struct LightsailClient {
    client: Client,
    region: region::Region,
}

impl LightsailClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> LightsailClient {
        LightsailClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> LightsailClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        LightsailClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Lightsail for LightsailClient {
    /// <p>Allocates a static IP address.</p>
    fn allocate_static_ip(
        &self,
        input: AllocateStaticIpRequest,
    ) -> RusotoFuture<AllocateStaticIpResult, AllocateStaticIpError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.AllocateStaticIp");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AllocateStaticIpResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AllocateStaticIpError::from_response(response))),
                )
            }
        })
    }

    /// <p>Attaches a block storage disk to a running or stopped Lightsail instance and exposes it to the instance with the specified disk name.</p> <p>The <code>attach disk</code> operation supports tag-based access control via resource tags applied to the resource identified by diskName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn attach_disk(
        &self,
        input: AttachDiskRequest,
    ) -> RusotoFuture<AttachDiskResult, AttachDiskError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.AttachDisk");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AttachDiskResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AttachDiskError::from_response(response))),
                )
            }
        })
    }

    /// <p>Attaches one or more Lightsail instances to a load balancer.</p> <p>After some time, the instances are attached to the load balancer and the health check status is available.</p> <p>The <code>attach instances to load balancer</code> operation supports tag-based access control via resource tags applied to the resource identified by loadBalancerName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn attach_instances_to_load_balancer(
        &self,
        input: AttachInstancesToLoadBalancerRequest,
    ) -> RusotoFuture<AttachInstancesToLoadBalancerResult, AttachInstancesToLoadBalancerError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.AttachInstancesToLoadBalancer",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AttachInstancesToLoadBalancerResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AttachInstancesToLoadBalancerError::from_response(response))
                }))
            }
        })
    }

    /// <p>Attaches a Transport Layer Security (TLS) certificate to your load balancer. TLS is just an updated, more secure version of Secure Socket Layer (SSL).</p> <p>Once you create and validate your certificate, you can attach it to your load balancer. You can also use this API to rotate the certificates on your account. Use the <code>AttachLoadBalancerTlsCertificate</code> operation with the non-attached certificate, and it will replace the existing one and become the attached certificate.</p> <p>The <code>attach load balancer tls certificate</code> operation supports tag-based access control via resource tags applied to the resource identified by loadBalancerName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn attach_load_balancer_tls_certificate(
        &self,
        input: AttachLoadBalancerTlsCertificateRequest,
    ) -> RusotoFuture<AttachLoadBalancerTlsCertificateResult, AttachLoadBalancerTlsCertificateError>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.AttachLoadBalancerTlsCertificate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AttachLoadBalancerTlsCertificateResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AttachLoadBalancerTlsCertificateError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Attaches a static IP address to a specific Amazon Lightsail instance.</p>
    fn attach_static_ip(
        &self,
        input: AttachStaticIpRequest,
    ) -> RusotoFuture<AttachStaticIpResult, AttachStaticIpError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.AttachStaticIp");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AttachStaticIpResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AttachStaticIpError::from_response(response))),
                )
            }
        })
    }

    /// <p>Closes the public ports on a specific Amazon Lightsail instance.</p> <p>The <code>close instance public ports</code> operation supports tag-based access control via resource tags applied to the resource identified by instanceName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn close_instance_public_ports(
        &self,
        input: CloseInstancePublicPortsRequest,
    ) -> RusotoFuture<CloseInstancePublicPortsResult, CloseInstancePublicPortsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.CloseInstancePublicPorts",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CloseInstancePublicPortsResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CloseInstancePublicPortsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Copies an instance or disk snapshot from one AWS Region to another in Amazon Lightsail.</p>
    fn copy_snapshot(
        &self,
        input: CopySnapshotRequest,
    ) -> RusotoFuture<CopySnapshotResult, CopySnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CopySnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CopySnapshotResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CopySnapshotError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Creates an AWS CloudFormation stack, which creates a new Amazon EC2 instance from an exported Amazon Lightsail snapshot. This operation results in a CloudFormation stack record that can be used to track the AWS CloudFormation stack created. Use the <code>get cloud formation stack records</code> operation to get a list of the CloudFormation stacks created.</p> <important> <p>Wait until after your new Amazon EC2 instance is created before running the <code>create cloud formation stack</code> operation again with the same export snapshot record.</p> </important></p>
    fn create_cloud_formation_stack(
        &self,
        input: CreateCloudFormationStackRequest,
    ) -> RusotoFuture<CreateCloudFormationStackResult, CreateCloudFormationStackError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.CreateCloudFormationStack",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateCloudFormationStackResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateCloudFormationStackError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a block storage disk that can be attached to a Lightsail instance in the same Availability Zone (e.g., <code>us-east-2a</code>). The disk is created in the regional endpoint that you send the HTTP request to. For more information, see <a href="https://lightsail.aws.amazon.com/ls/docs/overview/article/understanding-regions-and-availability-zones-in-amazon-lightsail">Regions and Availability Zones in Lightsail</a>.</p> <p>The <code>create disk</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_disk(
        &self,
        input: CreateDiskRequest,
    ) -> RusotoFuture<CreateDiskResult, CreateDiskError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateDisk");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateDiskResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDiskError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a block storage disk from a disk snapshot that can be attached to a Lightsail instance in the same Availability Zone (e.g., <code>us-east-2a</code>). The disk is created in the regional endpoint that you send the HTTP request to. For more information, see <a href="https://lightsail.aws.amazon.com/ls/docs/overview/article/understanding-regions-and-availability-zones-in-amazon-lightsail">Regions and Availability Zones in Lightsail</a>.</p> <p>The <code>create disk from snapshot</code> operation supports tag-based access control via request tags and resource tags applied to the resource identified by diskSnapshotName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_disk_from_snapshot(
        &self,
        input: CreateDiskFromSnapshotRequest,
    ) -> RusotoFuture<CreateDiskFromSnapshotResult, CreateDiskFromSnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateDiskFromSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateDiskFromSnapshotResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateDiskFromSnapshotError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a snapshot of a block storage disk. You can use snapshots for backups, to make copies of disks, and to save data before shutting down a Lightsail instance.</p> <p>You can take a snapshot of an attached disk that is in use; however, snapshots only capture data that has been written to your disk at the time the snapshot command is issued. This may exclude any data that has been cached by any applications or the operating system. If you can pause any file systems on the disk long enough to take a snapshot, your snapshot should be complete. Nevertheless, if you cannot pause all file writes to the disk, you should unmount the disk from within the Lightsail instance, issue the create disk snapshot command, and then remount the disk to ensure a consistent and complete snapshot. You may remount and use your disk while the snapshot status is pending.</p> <p>You can also use this operation to create a snapshot of an instance's system volume. You might want to do this, for example, to recover data from the system volume of a botched instance or to create a backup of the system volume like you would for a block storage disk. To create a snapshot of a system volume, just define the <code>instance name</code> parameter when issuing the snapshot command, and a snapshot of the defined instance's system volume will be created. After the snapshot is available, you can create a block storage disk from the snapshot and attach it to a running instance to access the data on the disk.</p> <p>The <code>create disk snapshot</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_disk_snapshot(
        &self,
        input: CreateDiskSnapshotRequest,
    ) -> RusotoFuture<CreateDiskSnapshotResult, CreateDiskSnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateDiskSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateDiskSnapshotResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDiskSnapshotError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a domain resource for the specified domain (e.g., example.com).</p> <p>The <code>create domain</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_domain(
        &self,
        input: CreateDomainRequest,
    ) -> RusotoFuture<CreateDomainResult, CreateDomainError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateDomainResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDomainError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates one of the following entry records associated with the domain: Address (A), canonical name (CNAME), mail exchanger (MX), name server (NS), start of authority (SOA), service locator (SRV), or text (TXT).</p> <p>The <code>create domain entry</code> operation supports tag-based access control via resource tags applied to the resource identified by domainName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_domain_entry(
        &self,
        input: CreateDomainEntryRequest,
    ) -> RusotoFuture<CreateDomainEntryResult, CreateDomainEntryError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateDomainEntry");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateDomainEntryResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDomainEntryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a snapshot of a specific virtual private server, or <i>instance</i>. You can use a snapshot to create a new instance that is based on that snapshot.</p> <p>The <code>create instance snapshot</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_instance_snapshot(
        &self,
        input: CreateInstanceSnapshotRequest,
    ) -> RusotoFuture<CreateInstanceSnapshotResult, CreateInstanceSnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateInstanceSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateInstanceSnapshotResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateInstanceSnapshotError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates one or more Amazon Lightsail virtual private servers, or <i>instances</i>. Create instances using active blueprints. Inactive blueprints are listed to support customers with existing instances but are not necessarily available for launch of new instances. Blueprints are marked inactive when they become outdated due to operating system updates or new application releases. Use the get blueprints operation to return a list of available blueprints.</p> <p>The <code>create instances</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_instances(
        &self,
        input: CreateInstancesRequest,
    ) -> RusotoFuture<CreateInstancesResult, CreateInstancesError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateInstances");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateInstancesResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateInstancesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Uses a specific snapshot as a blueprint for creating one or more new instances that are based on that identical configuration.</p> <p>The <code>create instances from snapshot</code> operation supports tag-based access control via request tags and resource tags applied to the resource identified by instanceSnapshotName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_instances_from_snapshot(
        &self,
        input: CreateInstancesFromSnapshotRequest,
    ) -> RusotoFuture<CreateInstancesFromSnapshotResult, CreateInstancesFromSnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.CreateInstancesFromSnapshot",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateInstancesFromSnapshotResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateInstancesFromSnapshotError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates an SSH key pair.</p> <p>The <code>create key pair</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_key_pair(
        &self,
        input: CreateKeyPairRequest,
    ) -> RusotoFuture<CreateKeyPairResult, CreateKeyPairError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateKeyPair");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateKeyPairResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateKeyPairError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a Lightsail load balancer. To learn more about deciding whether to load balance your application, see <a href="https://lightsail.aws.amazon.com/ls/docs/how-to/article/configure-lightsail-instances-for-load-balancing">Configure your Lightsail instances for load balancing</a>. You can create up to 5 load balancers per AWS Region in your account.</p> <p>When you create a load balancer, you can specify a unique name and port settings. To change additional load balancer settings, use the <code>UpdateLoadBalancerAttribute</code> operation.</p> <p>The <code>create load balancer</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_load_balancer(
        &self,
        input: CreateLoadBalancerRequest,
    ) -> RusotoFuture<CreateLoadBalancerResult, CreateLoadBalancerError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.CreateLoadBalancer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateLoadBalancerResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateLoadBalancerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a Lightsail load balancer TLS certificate.</p> <p>TLS is just an updated, more secure version of Secure Socket Layer (SSL).</p> <p>The <code>create load balancer tls certificate</code> operation supports tag-based access control via resource tags applied to the resource identified by loadBalancerName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_load_balancer_tls_certificate(
        &self,
        input: CreateLoadBalancerTlsCertificateRequest,
    ) -> RusotoFuture<CreateLoadBalancerTlsCertificateResult, CreateLoadBalancerTlsCertificateError>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.CreateLoadBalancerTlsCertificate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateLoadBalancerTlsCertificateResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateLoadBalancerTlsCertificateError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Creates a new database in Amazon Lightsail.</p> <p>The <code>create relational database</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_relational_database(
        &self,
        input: CreateRelationalDatabaseRequest,
    ) -> RusotoFuture<CreateRelationalDatabaseResult, CreateRelationalDatabaseError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.CreateRelationalDatabase",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateRelationalDatabaseResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateRelationalDatabaseError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a new database from an existing database snapshot in Amazon Lightsail.</p> <p>You can create a new database from a snapshot in if something goes wrong with your original database, or to change it to a different plan, such as a high availability or standard plan.</p> <p>The <code>create relational database from snapshot</code> operation supports tag-based access control via request tags and resource tags applied to the resource identified by relationalDatabaseSnapshotName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_relational_database_from_snapshot(
        &self,
        input: CreateRelationalDatabaseFromSnapshotRequest,
    ) -> RusotoFuture<
        CreateRelationalDatabaseFromSnapshotResult,
        CreateRelationalDatabaseFromSnapshotError,
    > {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.CreateRelationalDatabaseFromSnapshot",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateRelationalDatabaseFromSnapshotResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateRelationalDatabaseFromSnapshotError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Creates a snapshot of your database in Amazon Lightsail. You can use snapshots for backups, to make copies of a database, and to save data before deleting a database.</p> <p>The <code>create relational database snapshot</code> operation supports tag-based access control via request tags. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn create_relational_database_snapshot(
        &self,
        input: CreateRelationalDatabaseSnapshotRequest,
    ) -> RusotoFuture<CreateRelationalDatabaseSnapshotResult, CreateRelationalDatabaseSnapshotError>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.CreateRelationalDatabaseSnapshot",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateRelationalDatabaseSnapshotResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateRelationalDatabaseSnapshotError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Deletes the specified block storage disk. The disk must be in the <code>available</code> state (not attached to a Lightsail instance).</p> <note> <p>The disk may remain in the <code>deleting</code> state for several minutes.</p> </note> <p>The <code>delete disk</code> operation supports tag-based access control via resource tags applied to the resource identified by diskName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn delete_disk(
        &self,
        input: DeleteDiskRequest,
    ) -> RusotoFuture<DeleteDiskResult, DeleteDiskError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteDisk");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteDiskResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDiskError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified disk snapshot.</p> <p>When you make periodic snapshots of a disk, the snapshots are incremental, and only the blocks on the device that have changed since your last snapshot are saved in the new snapshot. When you delete a snapshot, only the data not needed for any other snapshot is removed. So regardless of which prior snapshots have been deleted, all active snapshots will have access to all the information needed to restore the disk.</p> <p>The <code>delete disk snapshot</code> operation supports tag-based access control via resource tags applied to the resource identified by diskSnapshotName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn delete_disk_snapshot(
        &self,
        input: DeleteDiskSnapshotRequest,
    ) -> RusotoFuture<DeleteDiskSnapshotResult, DeleteDiskSnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteDiskSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteDiskSnapshotResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDiskSnapshotError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified domain recordset and all of its domain records.</p> <p>The <code>delete domain</code> operation supports tag-based access control via resource tags applied to the resource identified by domainName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn delete_domain(
        &self,
        input: DeleteDomainRequest,
    ) -> RusotoFuture<DeleteDomainResult, DeleteDomainError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteDomainResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDomainError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a specific domain entry.</p> <p>The <code>delete domain entry</code> operation supports tag-based access control via resource tags applied to the resource identified by domainName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn delete_domain_entry(
        &self,
        input: DeleteDomainEntryRequest,
    ) -> RusotoFuture<DeleteDomainEntryResult, DeleteDomainEntryError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteDomainEntry");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteDomainEntryResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDomainEntryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a specific Amazon Lightsail virtual private server, or <i>instance</i>.</p> <p>The <code>delete instance</code> operation supports tag-based access control via resource tags applied to the resource identified by instanceName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn delete_instance(
        &self,
        input: DeleteInstanceRequest,
    ) -> RusotoFuture<DeleteInstanceResult, DeleteInstanceError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteInstanceResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteInstanceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a specific snapshot of a virtual private server (or <i>instance</i>).</p> <p>The <code>delete instance snapshot</code> operation supports tag-based access control via resource tags applied to the resource identified by instanceSnapshotName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn delete_instance_snapshot(
        &self,
        input: DeleteInstanceSnapshotRequest,
    ) -> RusotoFuture<DeleteInstanceSnapshotResult, DeleteInstanceSnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteInstanceSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteInstanceSnapshotResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteInstanceSnapshotError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a specific SSH key pair.</p> <p>The <code>delete key pair</code> operation supports tag-based access control via resource tags applied to the resource identified by keyPairName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn delete_key_pair(
        &self,
        input: DeleteKeyPairRequest,
    ) -> RusotoFuture<DeleteKeyPairResult, DeleteKeyPairError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteKeyPair");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteKeyPairResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteKeyPairError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Deletes the known host key or certificate used by the Amazon Lightsail browser-based SSH or RDP clients to authenticate an instance. This operation enables the Lightsail browser-based SSH or RDP clients to connect to the instance after a host key mismatch.</p> <important> <p>Perform this operation only if you were expecting the host key or certificate mismatch or if you are familiar with the new host key or certificate on the instance. For more information, see <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-troubleshooting-browser-based-ssh-rdp-client-connection">Troubleshooting connection issues when using the Amazon Lightsail browser-based SSH or RDP client</a>.</p> </important></p>
    fn delete_known_host_keys(
        &self,
        input: DeleteKnownHostKeysRequest,
    ) -> RusotoFuture<DeleteKnownHostKeysResult, DeleteKnownHostKeysError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteKnownHostKeys");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteKnownHostKeysResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteKnownHostKeysError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a Lightsail load balancer and all its associated SSL/TLS certificates. Once the load balancer is deleted, you will need to create a new load balancer, create a new certificate, and verify domain ownership again.</p> <p>The <code>delete load balancer</code> operation supports tag-based access control via resource tags applied to the resource identified by loadBalancerName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn delete_load_balancer(
        &self,
        input: DeleteLoadBalancerRequest,
    ) -> RusotoFuture<DeleteLoadBalancerResult, DeleteLoadBalancerError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DeleteLoadBalancer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteLoadBalancerResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteLoadBalancerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an SSL/TLS certificate associated with a Lightsail load balancer.</p> <p>The <code>delete load balancer tls certificate</code> operation supports tag-based access control via resource tags applied to the resource identified by loadBalancerName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn delete_load_balancer_tls_certificate(
        &self,
        input: DeleteLoadBalancerTlsCertificateRequest,
    ) -> RusotoFuture<DeleteLoadBalancerTlsCertificateResult, DeleteLoadBalancerTlsCertificateError>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.DeleteLoadBalancerTlsCertificate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteLoadBalancerTlsCertificateResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteLoadBalancerTlsCertificateError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a database in Amazon Lightsail.</p> <p>The <code>delete relational database</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn delete_relational_database(
        &self,
        input: DeleteRelationalDatabaseRequest,
    ) -> RusotoFuture<DeleteRelationalDatabaseResult, DeleteRelationalDatabaseError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.DeleteRelationalDatabase",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteRelationalDatabaseResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteRelationalDatabaseError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes a database snapshot in Amazon Lightsail.</p> <p>The <code>delete relational database snapshot</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn delete_relational_database_snapshot(
        &self,
        input: DeleteRelationalDatabaseSnapshotRequest,
    ) -> RusotoFuture<DeleteRelationalDatabaseSnapshotResult, DeleteRelationalDatabaseSnapshotError>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.DeleteRelationalDatabaseSnapshot",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteRelationalDatabaseSnapshotResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteRelationalDatabaseSnapshotError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Detaches a stopped block storage disk from a Lightsail instance. Make sure to unmount any file systems on the device within your operating system before stopping the instance and detaching the disk.</p> <p>The <code>detach disk</code> operation supports tag-based access control via resource tags applied to the resource identified by diskName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn detach_disk(
        &self,
        input: DetachDiskRequest,
    ) -> RusotoFuture<DetachDiskResult, DetachDiskError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DetachDisk");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DetachDiskResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DetachDiskError::from_response(response))),
                )
            }
        })
    }

    /// <p>Detaches the specified instances from a Lightsail load balancer.</p> <p>This operation waits until the instances are no longer needed before they are detached from the load balancer.</p> <p>The <code>detach instances from load balancer</code> operation supports tag-based access control via resource tags applied to the resource identified by loadBalancerName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn detach_instances_from_load_balancer(
        &self,
        input: DetachInstancesFromLoadBalancerRequest,
    ) -> RusotoFuture<DetachInstancesFromLoadBalancerResult, DetachInstancesFromLoadBalancerError>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.DetachInstancesFromLoadBalancer",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DetachInstancesFromLoadBalancerResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DetachInstancesFromLoadBalancerError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Detaches a static IP from the Amazon Lightsail instance to which it is attached.</p>
    fn detach_static_ip(
        &self,
        input: DetachStaticIpRequest,
    ) -> RusotoFuture<DetachStaticIpResult, DetachStaticIpError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DetachStaticIp");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DetachStaticIpResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DetachStaticIpError::from_response(response))),
                )
            }
        })
    }

    /// <p>Downloads the default SSH key pair from the user's account.</p>
    fn download_default_key_pair(
        &self,
    ) -> RusotoFuture<DownloadDefaultKeyPairResult, DownloadDefaultKeyPairError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.DownloadDefaultKeyPair");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DownloadDefaultKeyPairResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DownloadDefaultKeyPairError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Exports an Amazon Lightsail instance or block storage disk snapshot to Amazon Elastic Compute Cloud (Amazon EC2). This operation results in an export snapshot record that can be used with the <code>create cloud formation stack</code> operation to create new Amazon EC2 instances.</p> <p>Exported instance snapshots appear in Amazon EC2 as Amazon Machine Images (AMIs), and the instance system disk appears as an Amazon Elastic Block Store (Amazon EBS) volume. Exported disk snapshots appear in Amazon EC2 as Amazon EBS volumes. Snapshots are exported to the same Amazon Web Services Region in Amazon EC2 as the source Lightsail snapshot.</p> <p/> <p>The <code>export snapshot</code> operation supports tag-based access control via resource tags applied to the resource identified by sourceSnapshotName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p> <note> <p>Use the <code>get instance snapshots</code> or <code>get disk snapshots</code> operations to get a list of snapshots that you can export to Amazon EC2.</p> </note></p>
    fn export_snapshot(
        &self,
        input: ExportSnapshotRequest,
    ) -> RusotoFuture<ExportSnapshotResult, ExportSnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.ExportSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ExportSnapshotResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ExportSnapshotError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the names of all active (not deleted) resources.</p>
    fn get_active_names(
        &self,
        input: GetActiveNamesRequest,
    ) -> RusotoFuture<GetActiveNamesResult, GetActiveNamesError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetActiveNames");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetActiveNamesResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetActiveNamesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the list of available instance images, or <i>blueprints</i>. You can use a blueprint to create a new virtual private server already running a specific operating system, as well as a preinstalled app or development stack. The software each instance is running depends on the blueprint image you choose.</p>
    fn get_blueprints(
        &self,
        input: GetBlueprintsRequest,
    ) -> RusotoFuture<GetBlueprintsResult, GetBlueprintsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetBlueprints");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetBlueprintsResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBlueprintsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the list of bundles that are available for purchase. A bundle describes the specs for your virtual private server (or <i>instance</i>).</p>
    fn get_bundles(
        &self,
        input: GetBundlesRequest,
    ) -> RusotoFuture<GetBundlesResult, GetBundlesError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetBundles");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetBundlesResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBundlesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the CloudFormation stack record created as a result of the <code>create cloud formation stack</code> operation.</p> <p>An AWS CloudFormation stack is used to create a new Amazon EC2 instance from an exported Lightsail snapshot.</p>
    fn get_cloud_formation_stack_records(
        &self,
        input: GetCloudFormationStackRecordsRequest,
    ) -> RusotoFuture<GetCloudFormationStackRecordsResult, GetCloudFormationStackRecordsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetCloudFormationStackRecords",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCloudFormationStackRecordsResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetCloudFormationStackRecordsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns information about a specific block storage disk.</p>
    fn get_disk(&self, input: GetDiskRequest) -> RusotoFuture<GetDiskResult, GetDiskError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetDisk");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<GetDiskResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDiskError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about a specific block storage disk snapshot.</p>
    fn get_disk_snapshot(
        &self,
        input: GetDiskSnapshotRequest,
    ) -> RusotoFuture<GetDiskSnapshotResult, GetDiskSnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetDiskSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDiskSnapshotResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDiskSnapshotError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about all block storage disk snapshots in your AWS account and region.</p> <p>If you are describing a long list of disk snapshots, you can paginate the output to make the list more manageable. You can use the pageToken and nextPageToken values to retrieve the next items in the list.</p>
    fn get_disk_snapshots(
        &self,
        input: GetDiskSnapshotsRequest,
    ) -> RusotoFuture<GetDiskSnapshotsResult, GetDiskSnapshotsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetDiskSnapshots");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDiskSnapshotsResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDiskSnapshotsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about all block storage disks in your AWS account and region.</p> <p>If you are describing a long list of disks, you can paginate the output to make the list more manageable. You can use the pageToken and nextPageToken values to retrieve the next items in the list.</p>
    fn get_disks(&self, input: GetDisksRequest) -> RusotoFuture<GetDisksResult, GetDisksError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetDisks");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<GetDisksResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDisksError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about a specific domain recordset.</p>
    fn get_domain(&self, input: GetDomainRequest) -> RusotoFuture<GetDomainResult, GetDomainError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetDomain");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<GetDomainResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDomainError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of all domains in the user's account.</p>
    fn get_domains(
        &self,
        input: GetDomainsRequest,
    ) -> RusotoFuture<GetDomainsResult, GetDomainsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetDomains");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDomainsResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDomainsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the export snapshot record created as a result of the <code>export snapshot</code> operation.</p> <p>An export snapshot record can be used to create a new Amazon EC2 instance and its related resources with the <code>create cloud formation stack</code> operation.</p>
    fn get_export_snapshot_records(
        &self,
        input: GetExportSnapshotRecordsRequest,
    ) -> RusotoFuture<GetExportSnapshotRecordsResult, GetExportSnapshotRecordsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetExportSnapshotRecords",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetExportSnapshotRecordsResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetExportSnapshotRecordsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns information about a specific Amazon Lightsail instance, which is a virtual private server.</p>
    fn get_instance(
        &self,
        input: GetInstanceRequest,
    ) -> RusotoFuture<GetInstanceResult, GetInstanceError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetInstanceResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetInstanceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns temporary SSH keys you can use to connect to a specific virtual private server, or <i>instance</i>.</p> <p>The <code>get instance access details</code> operation supports tag-based access control via resource tags applied to the resource identified by instanceName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn get_instance_access_details(
        &self,
        input: GetInstanceAccessDetailsRequest,
    ) -> RusotoFuture<GetInstanceAccessDetailsResult, GetInstanceAccessDetailsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetInstanceAccessDetails",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetInstanceAccessDetailsResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetInstanceAccessDetailsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns the data points for the specified Amazon Lightsail instance metric, given an instance name.</p>
    fn get_instance_metric_data(
        &self,
        input: GetInstanceMetricDataRequest,
    ) -> RusotoFuture<GetInstanceMetricDataResult, GetInstanceMetricDataError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstanceMetricData");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetInstanceMetricDataResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetInstanceMetricDataError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns the port states for a specific virtual private server, or <i>instance</i>.</p>
    fn get_instance_port_states(
        &self,
        input: GetInstancePortStatesRequest,
    ) -> RusotoFuture<GetInstancePortStatesResult, GetInstancePortStatesError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstancePortStates");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetInstancePortStatesResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetInstancePortStatesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns information about a specific instance snapshot.</p>
    fn get_instance_snapshot(
        &self,
        input: GetInstanceSnapshotRequest,
    ) -> RusotoFuture<GetInstanceSnapshotResult, GetInstanceSnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstanceSnapshot");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetInstanceSnapshotResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetInstanceSnapshotError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns all instance snapshots for the user's account.</p>
    fn get_instance_snapshots(
        &self,
        input: GetInstanceSnapshotsRequest,
    ) -> RusotoFuture<GetInstanceSnapshotsResult, GetInstanceSnapshotsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstanceSnapshots");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetInstanceSnapshotsResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetInstanceSnapshotsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns the state of a specific instance. Works on one instance at a time.</p>
    fn get_instance_state(
        &self,
        input: GetInstanceStateRequest,
    ) -> RusotoFuture<GetInstanceStateResult, GetInstanceStateError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstanceState");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetInstanceStateResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetInstanceStateError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about all Amazon Lightsail virtual private servers, or <i>instances</i>.</p>
    fn get_instances(
        &self,
        input: GetInstancesRequest,
    ) -> RusotoFuture<GetInstancesResult, GetInstancesError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetInstances");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetInstancesResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetInstancesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about a specific key pair.</p>
    fn get_key_pair(
        &self,
        input: GetKeyPairRequest,
    ) -> RusotoFuture<GetKeyPairResult, GetKeyPairError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetKeyPair");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetKeyPairResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetKeyPairError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about all key pairs in the user's account.</p>
    fn get_key_pairs(
        &self,
        input: GetKeyPairsRequest,
    ) -> RusotoFuture<GetKeyPairsResult, GetKeyPairsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetKeyPairs");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetKeyPairsResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetKeyPairsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about the specified Lightsail load balancer.</p>
    fn get_load_balancer(
        &self,
        input: GetLoadBalancerRequest,
    ) -> RusotoFuture<GetLoadBalancerResult, GetLoadBalancerError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetLoadBalancer");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetLoadBalancerResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetLoadBalancerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about health metrics for your Lightsail load balancer.</p>
    fn get_load_balancer_metric_data(
        &self,
        input: GetLoadBalancerMetricDataRequest,
    ) -> RusotoFuture<GetLoadBalancerMetricDataResult, GetLoadBalancerMetricDataError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetLoadBalancerMetricData",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetLoadBalancerMetricDataResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetLoadBalancerMetricDataError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns information about the TLS certificates that are associated with the specified Lightsail load balancer.</p> <p>TLS is just an updated, more secure version of Secure Socket Layer (SSL).</p> <p>You can have a maximum of 2 certificates associated with a Lightsail load balancer. One is active and the other is inactive.</p>
    fn get_load_balancer_tls_certificates(
        &self,
        input: GetLoadBalancerTlsCertificatesRequest,
    ) -> RusotoFuture<GetLoadBalancerTlsCertificatesResult, GetLoadBalancerTlsCertificatesError>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetLoadBalancerTlsCertificates",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetLoadBalancerTlsCertificatesResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetLoadBalancerTlsCertificatesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns information about all load balancers in an account.</p> <p>If you are describing a long list of load balancers, you can paginate the output to make the list more manageable. You can use the pageToken and nextPageToken values to retrieve the next items in the list.</p>
    fn get_load_balancers(
        &self,
        input: GetLoadBalancersRequest,
    ) -> RusotoFuture<GetLoadBalancersResult, GetLoadBalancersError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetLoadBalancers");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetLoadBalancersResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetLoadBalancersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about a specific operation. Operations include events such as when you create an instance, allocate a static IP, attach a static IP, and so on.</p>
    fn get_operation(
        &self,
        input: GetOperationRequest,
    ) -> RusotoFuture<GetOperationResult, GetOperationError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetOperation");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetOperationResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetOperationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about all operations.</p> <p>Results are returned from oldest to newest, up to a maximum of 200. Results can be paged by making each subsequent call to <code>GetOperations</code> use the maximum (last) <code>statusChangedAt</code> value from the previous request.</p>
    fn get_operations(
        &self,
        input: GetOperationsRequest,
    ) -> RusotoFuture<GetOperationsResult, GetOperationsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetOperations");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetOperationsResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetOperationsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets operations for a specific resource (e.g., an instance or a static IP).</p>
    fn get_operations_for_resource(
        &self,
        input: GetOperationsForResourceRequest,
    ) -> RusotoFuture<GetOperationsForResourceResult, GetOperationsForResourceError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetOperationsForResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetOperationsForResourceResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetOperationsForResourceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns a list of all valid regions for Amazon Lightsail. Use the <code>include availability zones</code> parameter to also return the Availability Zones in a region.</p>
    fn get_regions(
        &self,
        input: GetRegionsRequest,
    ) -> RusotoFuture<GetRegionsResult, GetRegionsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetRegions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetRegionsResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetRegionsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about a specific database in Amazon Lightsail.</p>
    fn get_relational_database(
        &self,
        input: GetRelationalDatabaseRequest,
    ) -> RusotoFuture<GetRelationalDatabaseResult, GetRelationalDatabaseError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetRelationalDatabase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetRelationalDatabaseResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetRelationalDatabaseError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns a list of available database blueprints in Amazon Lightsail. A blueprint describes the major engine version of a database.</p> <p>You can use a blueprint ID to create a new database that runs a specific database engine.</p>
    fn get_relational_database_blueprints(
        &self,
        input: GetRelationalDatabaseBlueprintsRequest,
    ) -> RusotoFuture<GetRelationalDatabaseBlueprintsResult, GetRelationalDatabaseBlueprintsError>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetRelationalDatabaseBlueprints",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetRelationalDatabaseBlueprintsResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetRelationalDatabaseBlueprintsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns the list of bundles that are available in Amazon Lightsail. A bundle describes the performance specifications for a database.</p> <p>You can use a bundle ID to create a new database with explicit performance specifications.</p>
    fn get_relational_database_bundles(
        &self,
        input: GetRelationalDatabaseBundlesRequest,
    ) -> RusotoFuture<GetRelationalDatabaseBundlesResult, GetRelationalDatabaseBundlesError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetRelationalDatabaseBundles",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetRelationalDatabaseBundlesResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetRelationalDatabaseBundlesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns a list of events for a specific database in Amazon Lightsail.</p>
    fn get_relational_database_events(
        &self,
        input: GetRelationalDatabaseEventsRequest,
    ) -> RusotoFuture<GetRelationalDatabaseEventsResult, GetRelationalDatabaseEventsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetRelationalDatabaseEvents",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetRelationalDatabaseEventsResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetRelationalDatabaseEventsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns a list of log events for a database in Amazon Lightsail.</p>
    fn get_relational_database_log_events(
        &self,
        input: GetRelationalDatabaseLogEventsRequest,
    ) -> RusotoFuture<GetRelationalDatabaseLogEventsResult, GetRelationalDatabaseLogEventsError>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetRelationalDatabaseLogEvents",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetRelationalDatabaseLogEventsResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetRelationalDatabaseLogEventsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns a list of available log streams for a specific database in Amazon Lightsail.</p>
    fn get_relational_database_log_streams(
        &self,
        input: GetRelationalDatabaseLogStreamsRequest,
    ) -> RusotoFuture<GetRelationalDatabaseLogStreamsResult, GetRelationalDatabaseLogStreamsError>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetRelationalDatabaseLogStreams",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetRelationalDatabaseLogStreamsResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetRelationalDatabaseLogStreamsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns the current, previous, or pending versions of the master user password for a Lightsail database.</p> <p>The <code>asdf</code> operation GetRelationalDatabaseMasterUserPassword supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName.</p>
    fn get_relational_database_master_user_password(
        &self,
        input: GetRelationalDatabaseMasterUserPasswordRequest,
    ) -> RusotoFuture<
        GetRelationalDatabaseMasterUserPasswordResult,
        GetRelationalDatabaseMasterUserPasswordError,
    > {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetRelationalDatabaseMasterUserPassword",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetRelationalDatabaseMasterUserPasswordResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetRelationalDatabaseMasterUserPasswordError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns the data points of the specified metric for a database in Amazon Lightsail.</p>
    fn get_relational_database_metric_data(
        &self,
        input: GetRelationalDatabaseMetricDataRequest,
    ) -> RusotoFuture<GetRelationalDatabaseMetricDataResult, GetRelationalDatabaseMetricDataError>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetRelationalDatabaseMetricData",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetRelationalDatabaseMetricDataResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetRelationalDatabaseMetricDataError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns all of the runtime parameters offered by the underlying database software, or engine, for a specific database in Amazon Lightsail.</p> <p>In addition to the parameter names and values, this operation returns other information about each parameter. This information includes whether changes require a reboot, whether the parameter is modifiable, the allowed values, and the data types.</p>
    fn get_relational_database_parameters(
        &self,
        input: GetRelationalDatabaseParametersRequest,
    ) -> RusotoFuture<GetRelationalDatabaseParametersResult, GetRelationalDatabaseParametersError>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetRelationalDatabaseParameters",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetRelationalDatabaseParametersResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetRelationalDatabaseParametersError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns information about a specific database snapshot in Amazon Lightsail.</p>
    fn get_relational_database_snapshot(
        &self,
        input: GetRelationalDatabaseSnapshotRequest,
    ) -> RusotoFuture<GetRelationalDatabaseSnapshotResult, GetRelationalDatabaseSnapshotError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetRelationalDatabaseSnapshot",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetRelationalDatabaseSnapshotResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetRelationalDatabaseSnapshotError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns information about all of your database snapshots in Amazon Lightsail.</p>
    fn get_relational_database_snapshots(
        &self,
        input: GetRelationalDatabaseSnapshotsRequest,
    ) -> RusotoFuture<GetRelationalDatabaseSnapshotsResult, GetRelationalDatabaseSnapshotsError>
    {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.GetRelationalDatabaseSnapshots",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetRelationalDatabaseSnapshotsResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetRelationalDatabaseSnapshotsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns information about all of your databases in Amazon Lightsail.</p>
    fn get_relational_databases(
        &self,
        input: GetRelationalDatabasesRequest,
    ) -> RusotoFuture<GetRelationalDatabasesResult, GetRelationalDatabasesError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetRelationalDatabases");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetRelationalDatabasesResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetRelationalDatabasesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns information about a specific static IP.</p>
    fn get_static_ip(
        &self,
        input: GetStaticIpRequest,
    ) -> RusotoFuture<GetStaticIpResult, GetStaticIpError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetStaticIp");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetStaticIpResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetStaticIpError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about all static IPs in the user's account.</p>
    fn get_static_ips(
        &self,
        input: GetStaticIpsRequest,
    ) -> RusotoFuture<GetStaticIpsResult, GetStaticIpsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.GetStaticIps");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetStaticIpsResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetStaticIpsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Imports a public SSH key from a specific key pair.</p>
    fn import_key_pair(
        &self,
        input: ImportKeyPairRequest,
    ) -> RusotoFuture<ImportKeyPairResult, ImportKeyPairError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.ImportKeyPair");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ImportKeyPairResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ImportKeyPairError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a Boolean value indicating whether your Lightsail VPC is peered.</p>
    fn is_vpc_peered(&self) -> RusotoFuture<IsVpcPeeredResult, IsVpcPeeredError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.IsVpcPeered");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<IsVpcPeeredResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(IsVpcPeeredError::from_response(response))),
                )
            }
        })
    }

    /// <p>Adds public ports to an Amazon Lightsail instance.</p> <p>The <code>open instance public ports</code> operation supports tag-based access control via resource tags applied to the resource identified by instanceName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn open_instance_public_ports(
        &self,
        input: OpenInstancePublicPortsRequest,
    ) -> RusotoFuture<OpenInstancePublicPortsResult, OpenInstancePublicPortsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.OpenInstancePublicPorts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<OpenInstancePublicPortsResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(OpenInstancePublicPortsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Tries to peer the Lightsail VPC with the user's default VPC.</p>
    fn peer_vpc(&self) -> RusotoFuture<PeerVpcResult, PeerVpcError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.PeerVpc");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<PeerVpcResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PeerVpcError::from_response(response))),
                )
            }
        })
    }

    /// <p>Sets the specified open ports for an Amazon Lightsail instance, and closes all ports for every protocol not included in the current request.</p> <p>The <code>put instance public ports</code> operation supports tag-based access control via resource tags applied to the resource identified by instanceName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn put_instance_public_ports(
        &self,
        input: PutInstancePublicPortsRequest,
    ) -> RusotoFuture<PutInstancePublicPortsResult, PutInstancePublicPortsError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.PutInstancePublicPorts");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutInstancePublicPortsResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutInstancePublicPortsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Restarts a specific instance.</p> <p>The <code>reboot instance</code> operation supports tag-based access control via resource tags applied to the resource identified by instanceName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn reboot_instance(
        &self,
        input: RebootInstanceRequest,
    ) -> RusotoFuture<RebootInstanceResult, RebootInstanceError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.RebootInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RebootInstanceResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RebootInstanceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Restarts a specific database in Amazon Lightsail.</p> <p>The <code>reboot relational database</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn reboot_relational_database(
        &self,
        input: RebootRelationalDatabaseRequest,
    ) -> RusotoFuture<RebootRelationalDatabaseResult, RebootRelationalDatabaseError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.RebootRelationalDatabase",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RebootRelationalDatabaseResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RebootRelationalDatabaseError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes a specific static IP from your account.</p>
    fn release_static_ip(
        &self,
        input: ReleaseStaticIpRequest,
    ) -> RusotoFuture<ReleaseStaticIpResult, ReleaseStaticIpError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.ReleaseStaticIp");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ReleaseStaticIpResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ReleaseStaticIpError::from_response(response))),
                )
            }
        })
    }

    /// <p>Starts a specific Amazon Lightsail instance from a stopped state. To restart an instance, use the <code>reboot instance</code> operation.</p> <note> <p>When you start a stopped instance, Lightsail assigns a new public IP address to the instance. To use the same IP address after stopping and starting an instance, create a static IP address and attach it to the instance. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/lightsail-create-static-ip">Lightsail Dev Guide</a>.</p> </note> <p>The <code>start instance</code> operation supports tag-based access control via resource tags applied to the resource identified by instanceName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn start_instance(
        &self,
        input: StartInstanceRequest,
    ) -> RusotoFuture<StartInstanceResult, StartInstanceError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.StartInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartInstanceResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StartInstanceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Starts a specific database from a stopped state in Amazon Lightsail. To restart a database, use the <code>reboot relational database</code> operation.</p> <p>The <code>start relational database</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn start_relational_database(
        &self,
        input: StartRelationalDatabaseRequest,
    ) -> RusotoFuture<StartRelationalDatabaseResult, StartRelationalDatabaseError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.StartRelationalDatabase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartRelationalDatabaseResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartRelationalDatabaseError::from_response(response))
                }))
            }
        })
    }

    /// <p>Stops a specific Amazon Lightsail instance that is currently running.</p> <note> <p>When you start a stopped instance, Lightsail assigns a new public IP address to the instance. To use the same IP address after stopping and starting an instance, create a static IP address and attach it to the instance. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/lightsail-create-static-ip">Lightsail Dev Guide</a>.</p> </note> <p>The <code>stop instance</code> operation supports tag-based access control via resource tags applied to the resource identified by instanceName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn stop_instance(
        &self,
        input: StopInstanceRequest,
    ) -> RusotoFuture<StopInstanceResult, StopInstanceError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.StopInstance");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StopInstanceResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StopInstanceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Stops a specific database that is currently running in Amazon Lightsail.</p> <p>The <code>stop relational database</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn stop_relational_database(
        &self,
        input: StopRelationalDatabaseRequest,
    ) -> RusotoFuture<StopRelationalDatabaseResult, StopRelationalDatabaseError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.StopRelationalDatabase");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StopRelationalDatabaseResult, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StopRelationalDatabaseError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Adds one or more tags to the specified Amazon Lightsail resource. Each resource can have a maximum of 50 tags. Each tag consists of a key and an optional value. Tag keys must be unique per resource. For more information about tags, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-tags">Lightsail Dev Guide</a>.</p> <p>The <code>tag resource</code> operation supports tag-based access control via request tags and resource tags applied to the resource identified by resourceName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResult, TagResourceError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.TagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<TagResourceResult, _>()
                }))
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

    /// <p>Attempts to unpeer the Lightsail VPC from the user's default VPC.</p>
    fn unpeer_vpc(&self) -> RusotoFuture<UnpeerVpcResult, UnpeerVpcError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.UnpeerVpc");
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<UnpeerVpcResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UnpeerVpcError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified set of tag keys and their values from the specified Amazon Lightsail resource.</p> <p>The <code>untag resource</code> operation supports tag-based access control via request tags and resource tags applied to the resource identified by resourceName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResult, UntagResourceError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.UntagResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UntagResourceResult, _>()
                }))
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

    /// <p>Updates a domain recordset after it is created.</p> <p>The <code>update domain entry</code> operation supports tag-based access control via resource tags applied to the resource identified by domainName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn update_domain_entry(
        &self,
        input: UpdateDomainEntryRequest,
    ) -> RusotoFuture<UpdateDomainEntryResult, UpdateDomainEntryError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Lightsail_20161128.UpdateDomainEntry");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateDomainEntryResult, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateDomainEntryError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the specified attribute for a load balancer. You can only update one attribute at a time.</p> <p>The <code>update load balancer attribute</code> operation supports tag-based access control via resource tags applied to the resource identified by loadBalancerName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn update_load_balancer_attribute(
        &self,
        input: UpdateLoadBalancerAttributeRequest,
    ) -> RusotoFuture<UpdateLoadBalancerAttributeResult, UpdateLoadBalancerAttributeError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.UpdateLoadBalancerAttribute",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateLoadBalancerAttributeResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateLoadBalancerAttributeError::from_response(response))
                }))
            }
        })
    }

    /// <p>Allows the update of one or more attributes of a database in Amazon Lightsail.</p> <p>Updates are applied immediately, or in cases where the updates could result in an outage, are applied during the database's predefined maintenance window.</p> <p>The <code>update relational database</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn update_relational_database(
        &self,
        input: UpdateRelationalDatabaseRequest,
    ) -> RusotoFuture<UpdateRelationalDatabaseResult, UpdateRelationalDatabaseError> {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.UpdateRelationalDatabase",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateRelationalDatabaseResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateRelationalDatabaseError::from_response(response))
                }))
            }
        })
    }

    /// <p>Allows the update of one or more parameters of a database in Amazon Lightsail.</p> <p>Parameter updates don't cause outages; therefore, their application is not subject to the preferred maintenance window. However, there are two ways in which paramater updates are applied: <code>dynamic</code> or <code>pending-reboot</code>. Parameters marked with a <code>dynamic</code> apply type are applied immediately. Parameters marked with a <code>pending-reboot</code> apply type are applied only after the database is rebooted using the <code>reboot relational database</code> operation.</p> <p>The <code>update relational database parameters</code> operation supports tag-based access control via resource tags applied to the resource identified by relationalDatabaseName. For more information, see the <a href="https://lightsail.aws.amazon.com/ls/docs/en/articles/amazon-lightsail-controlling-access-using-tags">Lightsail Dev Guide</a>.</p>
    fn update_relational_database_parameters(
        &self,
        input: UpdateRelationalDatabaseParametersRequest,
    ) -> RusotoFuture<
        UpdateRelationalDatabaseParametersResult,
        UpdateRelationalDatabaseParametersError,
    > {
        let mut request = SignedRequest::new("POST", "lightsail", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Lightsail_20161128.UpdateRelationalDatabaseParameters",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateRelationalDatabaseParametersResult, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateRelationalDatabaseParametersError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
