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
/// <p>Describes a quota for an AWS account, for example, the number of replication instances allowed.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AccountQuota {
    /// <p>The name of the AWS DMS quota for this AWS account.</p>
    #[serde(rename = "AccountQuotaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_quota_name: Option<String>,
    /// <p>The maximum allowed value for the quota.</p>
    #[serde(rename = "Max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    /// <p>The amount currently used toward the quota maximum.</p>
    #[serde(rename = "Used")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<i64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddTagsToResourceMessage {
    /// <p>The Amazon Resource Name (ARN) of the AWS DMS resource the tag is to be added to. AWS DMS resources include a replication instance, endpoint, and a replication task.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tag to be assigned to the DMS resource.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AddTagsToResourceResponse {}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ApplyPendingMaintenanceActionMessage {
    /// <p>The pending maintenance action to apply to this resource.</p>
    #[serde(rename = "ApplyAction")]
    pub apply_action: String,
    /// <p><p>A value that specifies the type of opt-in request, or undoes an opt-in request. An opt-in request of type <code>immediate</code> cannot be undone.</p> <p>Valid values:</p> <ul> <li> <p> <code>immediate</code> - Apply the maintenance action immediately.</p> </li> <li> <p> <code>next-maintenance</code> - Apply the maintenance action during the next maintenance window for the resource.</p> </li> <li> <p> <code>undo-opt-in</code> - Cancel any existing <code>next-maintenance</code> opt-in requests.</p> </li> </ul></p>
    #[serde(rename = "OptInType")]
    pub opt_in_type: String,
    /// <p>The Amazon Resource Name (ARN) of the AWS DMS resource that the pending maintenance action applies to.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ApplyPendingMaintenanceActionResponse {
    /// <p>The AWS DMS resource that the pending maintenance action will be applied to.</p>
    #[serde(rename = "ResourcePendingMaintenanceActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_pending_maintenance_actions: Option<ResourcePendingMaintenanceActions>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AvailabilityZone {
    /// <p>The name of the availability zone.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>The SSL certificate that can be used to encrypt connections between the endpoints and the replication instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Certificate {
    /// <p>The Amazon Resource Name (ARN) for the certificate.</p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The date that the certificate was created.</p>
    #[serde(rename = "CertificateCreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_creation_date: Option<f64>,
    /// <p>The customer-assigned name of the certificate. Valid characters are A-z and 0-9.</p>
    #[serde(rename = "CertificateIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_identifier: Option<String>,
    /// <p>The owner of the certificate.</p>
    #[serde(rename = "CertificateOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_owner: Option<String>,
    /// <p>The contents of the .pem X.509 certificate file for the certificate.</p>
    #[serde(rename = "CertificatePem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_pem: Option<String>,
    /// <p>The location of the imported Oracle Wallet certificate for use with SSL.</p>
    #[serde(rename = "CertificateWallet")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_wallet: Option<bytes::Bytes>,
    /// <p>The key length of the cryptographic algorithm being used.</p>
    #[serde(rename = "KeyLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_length: Option<i64>,
    /// <p>The signing algorithm for the certificate.</p>
    #[serde(rename = "SigningAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_algorithm: Option<String>,
    /// <p>The beginning date that the certificate is valid.</p>
    #[serde(rename = "ValidFromDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from_date: Option<f64>,
    /// <p>The final date that the certificate is valid.</p>
    #[serde(rename = "ValidToDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to_date: Option<f64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Connection {
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_arn: Option<String>,
    /// <p>The identifier of the endpoint. Identifiers must begin with a letter; must contain only ASCII letters, digits, and hyphens; and must not end with a hyphen or contain two consecutive hyphens.</p>
    #[serde(rename = "EndpointIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_identifier: Option<String>,
    /// <p>The error message when the connection last failed.</p>
    #[serde(rename = "LastFailureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_message: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_arn: Option<String>,
    /// <p>The replication instance identifier. This parameter is stored as a lowercase string.</p>
    #[serde(rename = "ReplicationInstanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_identifier: Option<String>,
    /// <p>The connection status.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateEndpointMessage {
    /// <p>The Amazon Resource Name (ARN) for the certificate.</p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The name of the endpoint database.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>The settings in JSON format for the DMS transfer type of source endpoint. </p> <p>Possible attributes include the following:</p> <ul> <li> <p> <code>serviceAccessRoleArn</code> - The IAM role that has permission to access the Amazon S3 bucket.</p> </li> <li> <p> <code>bucketName</code> - The name of the S3 bucket to use.</p> </li> <li> <p> <code>compressionType</code> - An optional parameter to use GZIP to compress the target files. To use GZIP, set this value to <code>NONE</code> (the default). To keep the files uncompressed, don't use this value. </p> </li> </ul> <p>Shorthand syntax for these attributes is as follows: <code>ServiceAccessRoleArn=string,BucketName=string,CompressionType=string</code> </p> <p>JSON syntax for these attributes is as follows: <code>{ "ServiceAccessRoleArn": "string", "BucketName": "string", "CompressionType": "none"|"gzip" } </code> </p>
    #[serde(rename = "DmsTransferSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dms_transfer_settings: Option<DmsTransferSettings>,
    /// <p>Settings in JSON format for the target Amazon DynamoDB endpoint. For more information about the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.DynamoDB.html">Using Object Mapping to Migrate Data to DynamoDB</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "DynamoDbSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_db_settings: Option<DynamoDbSettings>,
    /// <p>Settings in JSON format for the target Elasticsearch endpoint. For more information about the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.Elasticsearch.html#CHAP_Target.Elasticsearch.Configuration">Extra Connection Attributes When Using Elasticsearch as a Target for AWS DMS</a> in the <i>AWS Database Migration User Guide.</i> </p>
    #[serde(rename = "ElasticsearchSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_settings: Option<ElasticsearchSettings>,
    /// <p>The database endpoint identifier. Identifiers must begin with a letter; must contain only ASCII letters, digits, and hyphens; and must not end with a hyphen or contain two consecutive hyphens.</p>
    #[serde(rename = "EndpointIdentifier")]
    pub endpoint_identifier: String,
    /// <p>The type of endpoint.</p>
    #[serde(rename = "EndpointType")]
    pub endpoint_type: String,
    /// <p>The type of engine for the endpoint. Valid values, depending on the <code>EndPointType</code> value, include <code>mysql</code>, <code>oracle</code>, <code>postgres</code>, <code>mariadb</code>, <code>aurora</code>, <code>aurora-postgresql</code>, <code>redshift</code>, <code>s3</code>, <code>db2</code>, <code>azuredb</code>, <code>sybase</code>, <code>dynamodb</code>, <code>mongodb</code>, and <code>sqlserver</code>.</p>
    #[serde(rename = "EngineName")]
    pub engine_name: String,
    /// <p>The external table definition. </p>
    #[serde(rename = "ExternalTableDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_table_definition: Option<String>,
    /// <p>Additional attributes associated with the connection.</p>
    #[serde(rename = "ExtraConnectionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_connection_attributes: Option<String>,
    /// <p>Settings in JSON format for the target Amazon Kinesis Data Streams endpoint. For more information about the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.Kinesis.html#CHAP_Target.Kinesis.ObjectMapping ">Using Object Mapping to Migrate Data to a Kinesis Data Stream</a> in the <i>AWS Database Migration User Guide.</i> </p>
    #[serde(rename = "KinesisSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_settings: Option<KinesisSettings>,
    /// <p>The AWS KMS key identifier to use to encrypt the connection parameters. If you don't specify a value for the <code>KmsKeyId</code> parameter, then AWS DMS uses your default encryption key. AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS Region.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>Settings in JSON format for the source MongoDB endpoint. For more information about the available settings, see the configuration properties section in <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.MongoDB.html"> Using MongoDB as a Target for AWS Database Migration Service</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "MongoDbSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongo_db_settings: Option<MongoDbSettings>,
    /// <p>The password to be used to log in to the endpoint database.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The port used by the endpoint database.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(rename = "RedshiftSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_settings: Option<RedshiftSettings>,
    /// <p>Settings in JSON format for the target Amazon S3 endpoint. For more information about the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.S3.html#CHAP_Target.S3.Configuring">Extra Connection Attributes When Using Amazon S3 as a Target for AWS DMS</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "S3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_settings: Option<S3Settings>,
    /// <p>The name of the server where the endpoint database resides.</p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// <p> The Amazon Resource Name (ARN) for the service access role that you want to use to create the endpoint. </p>
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    /// <p>The Secure Sockets Layer (SSL) mode to use for the SSL connection. The SSL mode can be one of four values: <code>none</code>, <code>require</code>, <code>verify-ca</code>, <code>verify-full</code>. The default value is <code>none</code>.</p>
    #[serde(rename = "SslMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<String>,
    /// <p>Tags to be added to the endpoint.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The user name to be used to log in to the endpoint database.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateEndpointResponse {
    /// <p>The endpoint that was created.</p>
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateEventSubscriptionMessage {
    /// <p> A Boolean value; set to <code>true</code> to activate the subscription, or set to <code>false</code> to create the subscription but not activate it. </p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p> A list of event categories for a source type that you want to subscribe to. You can see a list of the categories for a given source type by calling the <code>DescribeEventCategories</code> action or in the topic <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html">Working with Events and Notifications</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "EventCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<Vec<String>>,
    /// <p> The Amazon Resource Name (ARN) of the Amazon SNS topic created for event notification. The ARN is created by Amazon SNS when you create a topic and subscribe to it. </p>
    #[serde(rename = "SnsTopicArn")]
    pub sns_topic_arn: String,
    /// <p> The list of identifiers of the event sources for which events will be returned. If not specified, then all sources are included in the response. An identifier must begin with a letter and must contain only ASCII letters, digits, and hyphens; it cannot end with a hyphen or contain two consecutive hyphens. </p>
    #[serde(rename = "SourceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ids: Option<Vec<String>>,
    /// <p> The type of AWS DMS resource that generates the events. For example, if you want to be notified of events generated by a replication instance, you set this parameter to <code>replication-instance</code>. If this value is not specified, all events are returned. </p> <p>Valid values: replication-instance | migration-task</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// <p>The name of the AWS DMS event notification subscription. </p> <p>Constraints: The name must be less than 255 characters. </p>
    #[serde(rename = "SubscriptionName")]
    pub subscription_name: String,
    /// <p>A tag to be attached to the event subscription.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateEventSubscriptionResponse {
    /// <p>The event subscription that was created.</p>
    #[serde(rename = "EventSubscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscription: Option<EventSubscription>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateReplicationInstanceMessage {
    /// <p>The amount of storage (in gigabytes) to be initially allocated for the replication instance.</p>
    #[serde(rename = "AllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i64>,
    /// <p>Indicates that minor engine upgrades will be applied automatically to the replication instance during the maintenance window.</p> <p>Default: <code>true</code> </p>
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>The EC2 Availability Zone that the replication instance will be created in.</p> <p>Default: A random, system-chosen Availability Zone in the endpoint's region.</p> <p> Example: <code>us-east-1d</code> </p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>A list of DNS name servers supported for the replication instance.</p>
    #[serde(rename = "DnsNameServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name_servers: Option<String>,
    /// <p>The engine version number of the replication instance.</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>The AWS KMS key identifier that is used to encrypt the content on the replication instance. If you don't specify a value for the <code>KmsKeyId</code> parameter, then AWS DMS uses your default encryption key. AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS Region.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p> Specifies if the replication instance is a Multi-AZ deployment. You cannot set the <code>AvailabilityZone</code> parameter if the Multi-AZ parameter is set to <code>true</code>. </p>
    #[serde(rename = "MultiAZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_az: Option<bool>,
    /// <p>The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).</p> <p> Format: <code>ddd:hh24:mi-ddd:hh24:mi</code> </p> <p>Default: A 30-minute window selected at random from an 8-hour block of time per region, occurring on a random day of the week.</p> <p>Valid Days: Mon, Tue, Wed, Thu, Fri, Sat, Sun</p> <p>Constraints: Minimum 30-minute window.</p>
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p> Specifies the accessibility options for the replication instance. A value of <code>true</code> represents an instance with a public IP address. A value of <code>false</code> represents an instance with a private IP address. The default value is <code>true</code>. </p>
    #[serde(rename = "PubliclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    /// <p>The compute and memory capacity of the replication instance as specified by the replication instance class.</p> <p> Valid Values: <code>dms.t2.micro | dms.t2.small | dms.t2.medium | dms.t2.large | dms.c4.large | dms.c4.xlarge | dms.c4.2xlarge | dms.c4.4xlarge </code> </p>
    #[serde(rename = "ReplicationInstanceClass")]
    pub replication_instance_class: String,
    /// <p>The replication instance identifier. This parameter is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 alphanumeric characters or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>myrepinstance</code> </p>
    #[serde(rename = "ReplicationInstanceIdentifier")]
    pub replication_instance_identifier: String,
    /// <p>A subnet group to associate with the replication instance.</p>
    #[serde(rename = "ReplicationSubnetGroupIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group_identifier: Option<String>,
    /// <p>Tags to be associated with the replication instance.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p> Specifies the VPC security group to be used with the replication instance. The VPC security group must work with the VPC containing the replication instance. </p>
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<Vec<String>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateReplicationInstanceResponse {
    /// <p>The replication instance that was created.</p>
    #[serde(rename = "ReplicationInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance: Option<ReplicationInstance>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateReplicationSubnetGroupMessage {
    /// <p>The description for the subnet group.</p>
    #[serde(rename = "ReplicationSubnetGroupDescription")]
    pub replication_subnet_group_description: String,
    /// <p>The name for the replication subnet group. This value is stored as a lowercase string.</p> <p>Constraints: Must contain no more than 255 alphanumeric characters, periods, spaces, underscores, or hyphens. Must not be "default".</p> <p>Example: <code>mySubnetgroup</code> </p>
    #[serde(rename = "ReplicationSubnetGroupIdentifier")]
    pub replication_subnet_group_identifier: String,
    /// <p>The EC2 subnet IDs for the subnet group.</p>
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// <p>The tag to be assigned to the subnet group.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateReplicationSubnetGroupResponse {
    /// <p>The replication subnet group that was created.</p>
    #[serde(rename = "ReplicationSubnetGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group: Option<ReplicationSubnetGroup>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateReplicationTaskMessage {
    /// <p>Indicates when you want a change data capture (CDC) operation to start. Use either CdcStartPosition or CdcStartTime to specify when you want a CDC operation to start. Specifying both values results in an error.</p> <p> The value can be in date, checkpoint, or LSN/SCN format.</p> <p>Date Example: --cdc-start-position “2018-03-08T12:12:12”</p> <p>Checkpoint Example: --cdc-start-position "checkpoint:V1#27#mysql-bin-changelog.157832:1975:-1:2002:677883278264080:mysql-bin-changelog.157832:1876#0#0#*#0#93"</p> <p>LSN Example: --cdc-start-position “mysql-bin-changelog.000024:373”</p>
    #[serde(rename = "CdcStartPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_position: Option<String>,
    /// <p>Indicates the start time for a change data capture (CDC) operation. Use either CdcStartTime or CdcStartPosition to specify when you want a CDC operation to start. Specifying both values results in an error.</p> <p>Timestamp Example: --cdc-start-time “2018-03-08T12:12:12”</p>
    #[serde(rename = "CdcStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_time: Option<f64>,
    /// <p>Indicates when you want a change data capture (CDC) operation to stop. The value can be either server time or commit time.</p> <p>Server time example: --cdc-stop-position “server_time:3018-02-09T12:12:12”</p> <p>Commit time example: --cdc-stop-position “commit_time: 3018-02-09T12:12:12 “</p>
    #[serde(rename = "CdcStopPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_stop_position: Option<String>,
    /// <p>The migration type.</p>
    #[serde(rename = "MigrationType")]
    pub migration_type: String,
    /// <p>The Amazon Resource Name (ARN) of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: String,
    /// <p><p>The replication task identifier.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 255 alphanumeric characters or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul></p>
    #[serde(rename = "ReplicationTaskIdentifier")]
    pub replication_task_identifier: String,
    /// <p>Settings for the task, such as target metadata settings. For a complete list of task settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.CustomizingTasks.TaskSettings.html">Task Settings for AWS Database Migration Service Tasks</a> in the <i>AWS Database Migration User Guide.</i> </p>
    #[serde(rename = "ReplicationTaskSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_settings: Option<String>,
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    #[serde(rename = "SourceEndpointArn")]
    pub source_endpoint_arn: String,
    /// <p>When using the AWS CLI or boto3, provide the path of the JSON file that contains the table mappings. Precede the path with "file://". When working with the DMS API, provide the JSON as the parameter value.</p> <p>For example, --table-mappings file://mappingfile.json</p>
    #[serde(rename = "TableMappings")]
    pub table_mappings: String,
    /// <p>Tags to be added to the replication instance.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    #[serde(rename = "TargetEndpointArn")]
    pub target_endpoint_arn: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateReplicationTaskResponse {
    /// <p>The replication task that was created.</p>
    #[serde(rename = "ReplicationTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteCertificateMessage {
    /// <p>The Amazon Resource Name (ARN) of the deleted certificate.</p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteCertificateResponse {
    /// <p>The Secure Sockets Layer (SSL) certificate.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<Certificate>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteEndpointMessage {
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteEndpointResponse {
    /// <p>The endpoint that was deleted.</p>
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteEventSubscriptionMessage {
    /// <p>The name of the DMS event notification subscription to be deleted.</p>
    #[serde(rename = "SubscriptionName")]
    pub subscription_name: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteEventSubscriptionResponse {
    /// <p>The event subscription that was deleted.</p>
    #[serde(rename = "EventSubscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscription: Option<EventSubscription>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteReplicationInstanceMessage {
    /// <p>The Amazon Resource Name (ARN) of the replication instance to be deleted.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteReplicationInstanceResponse {
    /// <p>The replication instance that was deleted.</p>
    #[serde(rename = "ReplicationInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance: Option<ReplicationInstance>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteReplicationSubnetGroupMessage {
    /// <p>The subnet group name of the replication instance.</p>
    #[serde(rename = "ReplicationSubnetGroupIdentifier")]
    pub replication_subnet_group_identifier: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteReplicationSubnetGroupResponse {}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteReplicationTaskMessage {
    /// <p>The Amazon Resource Name (ARN) of the replication task to be deleted.</p>
    #[serde(rename = "ReplicationTaskArn")]
    pub replication_task_arn: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteReplicationTaskResponse {
    /// <p>The deleted replication task.</p>
    #[serde(rename = "ReplicationTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAccountAttributesMessage {}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeAccountAttributesResponse {
    /// <p>Account quota information.</p>
    #[serde(rename = "AccountQuotas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_quotas: Option<Vec<AccountQuota>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeCertificatesMessage {
    /// <p>Filters applied to the certificate described in the form of key-value pairs.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 10</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeCertificatesResponse {
    /// <p>The Secure Sockets Layer (SSL) certificates associated with the replication instance.</p>
    #[serde(rename = "Certificates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<Certificate>>,
    /// <p>The pagination token.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeConnectionsMessage {
    /// <p>The filters applied to the connection.</p> <p>Valid filter names: endpoint-arn | replication-instance-arn</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeConnectionsResponse {
    /// <p>A description of the connections.</p>
    #[serde(rename = "Connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<Connection>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEndpointTypesMessage {
    /// <p>Filters applied to the describe action.</p> <p>Valid filter names: engine-name | endpoint-type</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeEndpointTypesResponse {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The type of endpoints that are supported.</p>
    #[serde(rename = "SupportedEndpointTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_endpoint_types: Option<Vec<SupportedEndpointType>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEndpointsMessage {
    /// <p>Filters applied to the describe action.</p> <p>Valid filter names: endpoint-arn | endpoint-type | endpoint-id | engine-name</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeEndpointsResponse {
    /// <p>Endpoint description.</p>
    #[serde(rename = "Endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<Endpoint>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEventCategoriesMessage {
    /// <p>Filters applied to the action.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> The type of AWS DMS resource that generates events. </p> <p>Valid values: replication-instance | migration-task</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeEventCategoriesResponse {
    /// <p>A list of event categories.</p>
    #[serde(rename = "EventCategoryGroupList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_category_group_list: Option<Vec<EventCategoryGroup>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEventSubscriptionsMessage {
    /// <p>Filters applied to the action.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
    /// <p>The name of the AWS DMS event subscription to be described.</p>
    #[serde(rename = "SubscriptionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeEventSubscriptionsResponse {
    /// <p>A list of event subscriptions.</p>
    #[serde(rename = "EventSubscriptionsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscriptions_list: Option<Vec<EventSubscription>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeEventsMessage {
    /// <p>The duration of the events to be listed.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>The end time for the events to be listed.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>A list of event categories for a source type that you want to subscribe to.</p>
    #[serde(rename = "EventCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<Vec<String>>,
    /// <p>Filters applied to the action.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
    /// <p> The identifier of the event source. An identifier must begin with a letter and must contain only ASCII letters, digits, and hyphens. It cannot end with a hyphen or contain two consecutive hyphens. </p>
    #[serde(rename = "SourceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identifier: Option<String>,
    /// <p>The type of AWS DMS resource that generates events.</p> <p>Valid values: replication-instance | migration-task</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// <p>The start time for the events to be listed.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeEventsResponse {
    /// <p>The events described.</p>
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeOrderableReplicationInstancesMessage {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeOrderableReplicationInstancesResponse {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The order-able replication instances available.</p>
    #[serde(rename = "OrderableReplicationInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderable_replication_instances: Option<Vec<OrderableReplicationInstance>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribePendingMaintenanceActionsMessage {
    /// <p><p/></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
    /// <p>The ARN of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_arn: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribePendingMaintenanceActionsResponse {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The pending maintenance action.</p>
    #[serde(rename = "PendingMaintenanceActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_maintenance_actions: Option<Vec<ResourcePendingMaintenanceActions>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeRefreshSchemasStatusMessage {
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeRefreshSchemasStatusResponse {
    /// <p>The status of the schema.</p>
    #[serde(rename = "RefreshSchemasStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_schemas_status: Option<RefreshSchemasStatus>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeReplicationInstanceTaskLogsMessage {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeReplicationInstanceTaskLogsResponse {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_arn: Option<String>,
    /// <p>An array of replication task log metadata. Each member of the array contains the replication task name, ARN, and task log size (in bytes). </p>
    #[serde(rename = "ReplicationInstanceTaskLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_task_logs: Option<Vec<ReplicationInstanceTaskLog>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeReplicationInstancesMessage {
    /// <p>Filters applied to the describe action.</p> <p>Valid filter names: replication-instance-arn | replication-instance-id | replication-instance-class | engine-version</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeReplicationInstancesResponse {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The replication instances described.</p>
    #[serde(rename = "ReplicationInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instances: Option<Vec<ReplicationInstance>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeReplicationSubnetGroupsMessage {
    /// <p>Filters applied to the describe action.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeReplicationSubnetGroupsResponse {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>A description of the replication subnet groups.</p>
    #[serde(rename = "ReplicationSubnetGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_groups: Option<Vec<ReplicationSubnetGroup>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeReplicationTaskAssessmentResultsMessage {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
    /// <p>- The Amazon Resource Name (ARN) string that uniquely identifies the task. When this input parameter is specified the API will return only one result and ignore the values of the max-records and marker parameters. </p>
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_arn: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeReplicationTaskAssessmentResultsResponse {
    /// <p>- The Amazon S3 bucket where the task assessment report is located. </p>
    #[serde(rename = "BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The task assessment report. </p>
    #[serde(rename = "ReplicationTaskAssessmentResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_assessment_results: Option<Vec<ReplicationTaskAssessmentResult>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeReplicationTasksMessage {
    /// <p>Filters applied to the describe action.</p> <p>Valid filter names: replication-task-arn | replication-task-id | migration-type | endpoint-arn | replication-instance-arn</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
    /// <p>Set this flag to avoid returning setting information. Use this to reduce overhead when settings are too large. Choose TRUE to use this flag, otherwise choose FALSE (default).</p>
    #[serde(rename = "WithoutSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub without_settings: Option<bool>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeReplicationTasksResponse {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>A description of the replication tasks.</p>
    #[serde(rename = "ReplicationTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_tasks: Option<Vec<ReplicationTask>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeSchemasMessage {
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeSchemasResponse {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The described schema.</p>
    #[serde(rename = "Schemas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<String>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeTableStatisticsMessage {
    /// <p>Filters applied to the describe table statistics action.</p> <p>Valid filter names: schema-name | table-name | table-state</p> <p>A combination of filters creates an AND condition where each record matches all specified filters.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 500.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the replication task.</p>
    #[serde(rename = "ReplicationTaskArn")]
    pub replication_task_arn: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeTableStatisticsResponse {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the replication task.</p>
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_arn: Option<String>,
    /// <p>The table statistics.</p>
    #[serde(rename = "TableStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_statistics: Option<Vec<TableStatistics>>,
}

/// <p> The settings in JSON format for the DMS Transfer type source endpoint. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DmsTransferSettings {
    /// <p> The name of the S3 bucket to use. </p>
    #[serde(rename = "BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    /// <p> The IAM role that has permission to access the Amazon S3 bucket. </p>
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamoDbSettings {
    /// <p> The Amazon Resource Name (ARN) used by the service access IAM role. </p>
    #[serde(rename = "ServiceAccessRoleArn")]
    pub service_access_role_arn: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElasticsearchSettings {
    /// <p>The endpoint for the ElasticSearch cluster.</p>
    #[serde(rename = "EndpointUri")]
    pub endpoint_uri: String,
    /// <p>The maximum number of seconds that DMS retries failed API requests to the Elasticsearch cluster.</p>
    #[serde(rename = "ErrorRetryDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_retry_duration: Option<i64>,
    /// <p>The maximum percentage of records that can fail to be written before a full load operation stops. </p>
    #[serde(rename = "FullLoadErrorPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_error_percentage: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) used by service to access the IAM role.</p>
    #[serde(rename = "ServiceAccessRoleArn")]
    pub service_access_role_arn: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Endpoint {
    /// <p>The Amazon Resource Name (ARN) used for SSL connection to the endpoint.</p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The name of the database at the endpoint.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>The settings in JSON format for the DMS transfer type of source endpoint. </p> <p>Possible attributes include the following:</p> <ul> <li> <p> <code>serviceAccessRoleArn</code> - The IAM role that has permission to access the Amazon S3 bucket.</p> </li> <li> <p> <code>bucketName</code> - The name of the S3 bucket to use.</p> </li> <li> <p> <code>compressionType</code> - An optional parameter to use GZIP to compress the target files. To use GZIP, set this value to <code>NONE</code> (the default). To keep the files uncompressed, don't use this value.</p> </li> </ul> <p>Shorthand syntax for these attributes is as follows: <code>ServiceAccessRoleArn=string,BucketName=string,CompressionType=string</code> </p> <p>JSON syntax for these attributes is as follows: <code>{ "ServiceAccessRoleArn": "string", "BucketName": "string", "CompressionType": "none"|"gzip" } </code> </p>
    #[serde(rename = "DmsTransferSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dms_transfer_settings: Option<DmsTransferSettings>,
    /// <p>The settings for the target DynamoDB database. For more information, see the <code>DynamoDBSettings</code> structure.</p>
    #[serde(rename = "DynamoDbSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_db_settings: Option<DynamoDbSettings>,
    /// <p>The settings for the Elasticsearch source endpoint. For more information, see the <code>ElasticsearchSettings</code> structure.</p>
    #[serde(rename = "ElasticsearchSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_settings: Option<ElasticsearchSettings>,
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_arn: Option<String>,
    /// <p>The database endpoint identifier. Identifiers must begin with a letter; must contain only ASCII letters, digits, and hyphens; and must not end with a hyphen or contain two consecutive hyphens.</p>
    #[serde(rename = "EndpointIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_identifier: Option<String>,
    /// <p>The type of endpoint.</p>
    #[serde(rename = "EndpointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    /// <p>The expanded name for the engine name. For example, if the <code>EngineName</code> parameter is "aurora," this value would be "Amazon Aurora MySQL."</p>
    #[serde(rename = "EngineDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_display_name: Option<String>,
    /// <p>The database engine name. Valid values, depending on the EndPointType, include mysql, oracle, postgres, mariadb, aurora, aurora-postgresql, redshift, s3, db2, azuredb, sybase, sybase, dynamodb, mongodb, and sqlserver.</p>
    #[serde(rename = "EngineName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_name: Option<String>,
    /// <p> Value returned by a call to CreateEndpoint that can be used for cross-account validation. Use it on a subsequent call to CreateEndpoint to create the endpoint with a cross-account. </p>
    #[serde(rename = "ExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// <p>The external table definition.</p>
    #[serde(rename = "ExternalTableDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_table_definition: Option<String>,
    /// <p>Additional connection attributes used to connect to the endpoint.</p>
    #[serde(rename = "ExtraConnectionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_connection_attributes: Option<String>,
    /// <p>The settings for the Amazon Kinesis source endpoint. For more information, see the <code>KinesisSettings</code> structure.</p>
    #[serde(rename = "KinesisSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_settings: Option<KinesisSettings>,
    /// <p>The AWS KMS key identifier that is used to encrypt the content on the replication instance. If you don't specify a value for the <code>KmsKeyId</code> parameter, then AWS DMS uses your default encryption key. AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS Region.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The settings for the MongoDB source endpoint. For more information, see the <code>MongoDbSettings</code> structure.</p>
    #[serde(rename = "MongoDbSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongo_db_settings: Option<MongoDbSettings>,
    /// <p>The port value used to access the endpoint.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>Settings for the Amazon Redshift endpoint</p>
    #[serde(rename = "RedshiftSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_settings: Option<RedshiftSettings>,
    /// <p>The settings for the S3 target endpoint. For more information, see the <code>S3Settings</code> structure.</p>
    #[serde(rename = "S3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_settings: Option<S3Settings>,
    /// <p>The name of the server at the endpoint.</p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) used by the service access IAM role.</p>
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    /// <p>The SSL mode used to connect to the endpoint.</p> <p>SSL mode can be one of four values: none, require, verify-ca, verify-full. </p> <p>The default value is none.</p>
    #[serde(rename = "SslMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<String>,
    /// <p>The status of the endpoint.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The user name used to connect to the endpoint.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Event {
    /// <p>The date of the event.</p>
    #[serde(rename = "Date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<f64>,
    /// <p>The event categories available for the specified source type.</p>
    #[serde(rename = "EventCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<Vec<String>>,
    /// <p>The event message.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p> The identifier of the event source. An identifier must begin with a letter and must contain only ASCII letters, digits, and hyphens; it cannot end with a hyphen or contain two consecutive hyphens. </p> <p>Constraints:replication instance, endpoint, migration task</p>
    #[serde(rename = "SourceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identifier: Option<String>,
    /// <p> The type of AWS DMS resource that generates events. </p> <p>Valid values: replication-instance | endpoint | migration-task</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EventCategoryGroup {
    /// <p> A list of event categories for a <code>SourceType</code> that you want to subscribe to. </p>
    #[serde(rename = "EventCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<Vec<String>>,
    /// <p> The type of AWS DMS resource that generates events. </p> <p>Valid values: replication-instance | replication-server | security-group | migration-task</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EventSubscription {
    /// <p>The AWS DMS event notification subscription Id.</p>
    #[serde(rename = "CustSubscriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cust_subscription_id: Option<String>,
    /// <p>The AWS customer account associated with the AWS DMS event notification subscription.</p>
    #[serde(rename = "CustomerAwsId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_aws_id: Option<String>,
    /// <p>Boolean value that indicates if the event subscription is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>A lists of event categories.</p>
    #[serde(rename = "EventCategoriesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories_list: Option<Vec<String>>,
    /// <p>The topic ARN of the AWS DMS event notification subscription.</p>
    #[serde(rename = "SnsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    /// <p>A list of source Ids for the event subscription.</p>
    #[serde(rename = "SourceIdsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ids_list: Option<Vec<String>>,
    /// <p> The type of AWS DMS resource that generates events. </p> <p>Valid values: replication-instance | replication-server | security-group | migration-task</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// <p>The status of the AWS DMS event notification subscription.</p> <p>Constraints:</p> <p>Can be one of the following: creating | modifying | deleting | active | no-permission | topic-not-exist</p> <p>The status "no-permission" indicates that AWS DMS no longer has permission to post to the SNS topic. The status "topic-not-exist" indicates that the topic was deleted after the subscription was created.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The time the RDS event notification subscription was created.</p>
    #[serde(rename = "SubscriptionCreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_creation_time: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Filter {
    /// <p>The name of the filter.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The filter value.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportCertificateMessage {
    /// <p>The customer-assigned name of the certificate. Valid characters are A-z and 0-9.</p>
    #[serde(rename = "CertificateIdentifier")]
    pub certificate_identifier: String,
    /// <p>The contents of the .pem X.509 certificate file for the certificate.</p>
    #[serde(rename = "CertificatePem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_pem: Option<String>,
    /// <p>The location of the imported Oracle Wallet certificate for use with SSL.</p>
    #[serde(rename = "CertificateWallet")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_wallet: Option<bytes::Bytes>,
    /// <p>The tags associated with the certificate.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ImportCertificateResponse {
    /// <p>The certificate to be uploaded.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<Certificate>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KinesisSettings {
    /// <p>The output format for the records created on the endpoint. The message format is <code>JSON</code>.</p>
    #[serde(rename = "MessageFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_format: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the IAM role that DMS uses to write to the Amazon Kinesis data stream.</p>
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the Amazon Kinesis Data Streams endpoint.</p>
    #[serde(rename = "StreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceMessage {
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the AWS DMS resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>A list of tags for the resource.</p>
    #[serde(rename = "TagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ModifyEndpointMessage {
    /// <p>The Amazon Resource Name (ARN) of the certificate used for SSL connection.</p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The name of the endpoint database.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>The settings in JSON format for the DMS transfer type of source endpoint. </p> <p>Attributes include the following:</p> <ul> <li> <p>serviceAccessRoleArn - The IAM role that has permission to access the Amazon S3 bucket.</p> </li> <li> <p>BucketName - The name of the S3 bucket to use.</p> </li> <li> <p>compressionType - An optional parameter to use GZIP to compress the target files. Set to NONE (the default) or do not use to leave the files uncompressed.</p> </li> </ul> <p>Shorthand syntax: ServiceAccessRoleArn=string ,BucketName=string,CompressionType=string</p> <p>JSON syntax:</p> <p> { "ServiceAccessRoleArn": "string", "BucketName": "string", "CompressionType": "none"|"gzip" } </p>
    #[serde(rename = "DmsTransferSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dms_transfer_settings: Option<DmsTransferSettings>,
    /// <p>Settings in JSON format for the target Amazon DynamoDB endpoint. For more information about the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.DynamoDB.html">Using Object Mapping to Migrate Data to DynamoDB</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "DynamoDbSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_db_settings: Option<DynamoDbSettings>,
    /// <p>Settings in JSON format for the target Elasticsearch endpoint. For more information about the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.Elasticsearch.html#CHAP_Target.Elasticsearch.Configuration">Extra Connection Attributes When Using Elasticsearch as a Target for AWS DMS</a> in the <i>AWS Database Migration User Guide.</i> </p>
    #[serde(rename = "ElasticsearchSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_settings: Option<ElasticsearchSettings>,
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
    /// <p>The database endpoint identifier. Identifiers must begin with a letter; must contain only ASCII letters, digits, and hyphens; and must not end with a hyphen or contain two consecutive hyphens.</p>
    #[serde(rename = "EndpointIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_identifier: Option<String>,
    /// <p>The type of endpoint.</p>
    #[serde(rename = "EndpointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    /// <p>The type of engine for the endpoint. Valid values, depending on the EndPointType, include mysql, oracle, postgres, mariadb, aurora, aurora-postgresql, redshift, s3, db2, azuredb, sybase, sybase, dynamodb, mongodb, and sqlserver.</p>
    #[serde(rename = "EngineName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_name: Option<String>,
    /// <p>The external table definition.</p>
    #[serde(rename = "ExternalTableDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_table_definition: Option<String>,
    /// <p>Additional attributes associated with the connection. To reset this parameter, pass the empty string ("") as an argument.</p>
    #[serde(rename = "ExtraConnectionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_connection_attributes: Option<String>,
    /// <p>Settings in JSON format for the target Amazon Kinesis Data Streams endpoint. For more information about the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.Kinesis.html#CHAP_Target.Kinesis.ObjectMapping ">Using Object Mapping to Migrate Data to a Kinesis Data Stream</a> in the <i>AWS Database Migration User Guide.</i> </p>
    #[serde(rename = "KinesisSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_settings: Option<KinesisSettings>,
    /// <p>Settings in JSON format for the source MongoDB endpoint. For more information about the available settings, see the configuration properties section in <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.MongoDB.html"> Using MongoDB as a Target for AWS Database Migration Service</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "MongoDbSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongo_db_settings: Option<MongoDbSettings>,
    /// <p>The password to be used to login to the endpoint database.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The port used by the endpoint database.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[serde(rename = "RedshiftSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_settings: Option<RedshiftSettings>,
    /// <p>Settings in JSON format for the target Amazon S3 endpoint. For more information about the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.S3.html#CHAP_Target.S3.Configuring">Extra Connection Attributes When Using Amazon S3 as a Target for AWS DMS</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "S3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_settings: Option<S3Settings>,
    /// <p>The name of the server where the endpoint database resides.</p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// <p> The Amazon Resource Name (ARN) for the service access role you want to use to modify the endpoint. </p>
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    /// <p>The SSL mode to be used.</p> <p>SSL mode can be one of four values: none, require, verify-ca, verify-full. </p> <p>The default value is none.</p>
    #[serde(rename = "SslMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<String>,
    /// <p>The user name to be used to login to the endpoint database.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ModifyEndpointResponse {
    /// <p>The modified endpoint.</p>
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ModifyEventSubscriptionMessage {
    /// <p> A Boolean value; set to <b>true</b> to activate the subscription. </p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p> A list of event categories for a source type that you want to subscribe to. Use the <code>DescribeEventCategories</code> action to see a list of event categories. </p>
    #[serde(rename = "EventCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<Vec<String>>,
    /// <p> The Amazon Resource Name (ARN) of the Amazon SNS topic created for event notification. The ARN is created by Amazon SNS when you create a topic and subscribe to it.</p>
    #[serde(rename = "SnsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    /// <p> The type of AWS DMS resource that generates the events you want to subscribe to. </p> <p>Valid values: replication-instance | migration-task</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// <p>The name of the AWS DMS event notification subscription to be modified.</p>
    #[serde(rename = "SubscriptionName")]
    pub subscription_name: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ModifyEventSubscriptionResponse {
    /// <p>The modified event subscription.</p>
    #[serde(rename = "EventSubscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscription: Option<EventSubscription>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ModifyReplicationInstanceMessage {
    /// <p>The amount of storage (in gigabytes) to be allocated for the replication instance.</p>
    #[serde(rename = "AllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i64>,
    /// <p>Indicates that major version upgrades are allowed. Changing this parameter does not result in an outage and the change is asynchronously applied as soon as possible.</p> <p>Constraints: This parameter must be set to true when specifying a value for the <code>EngineVersion</code> parameter that is a different major version than the replication instance's current version.</p>
    #[serde(rename = "AllowMajorVersionUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_major_version_upgrade: Option<bool>,
    /// <p>Indicates whether the changes should be applied immediately or during the next maintenance window.</p>
    #[serde(rename = "ApplyImmediately")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_immediately: Option<bool>,
    /// <p> Indicates that minor version upgrades will be applied automatically to the replication instance during the maintenance window. Changing this parameter does not result in an outage except in the following case and the change is asynchronously applied as soon as possible. An outage will result if this parameter is set to <code>true</code> during the maintenance window, and a newer minor version is available, and AWS DMS has enabled auto patching for that engine version. </p>
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>The engine version number of the replication instance.</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p> Specifies if the replication instance is a Multi-AZ deployment. You cannot set the <code>AvailabilityZone</code> parameter if the Multi-AZ parameter is set to <code>true</code>. </p>
    #[serde(rename = "MultiAZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_az: Option<bool>,
    /// <p>The weekly time range (in UTC) during which system maintenance can occur, which might result in an outage. Changing this parameter does not result in an outage, except in the following situation, and the change is asynchronously applied as soon as possible. If moving this window to the current time, there must be at least 30 minutes between the current time and end of the window to ensure pending changes are applied.</p> <p>Default: Uses existing setting</p> <p>Format: ddd:hh24:mi-ddd:hh24:mi</p> <p>Valid Days: Mon | Tue | Wed | Thu | Fri | Sat | Sun</p> <p>Constraints: Must be at least 30 minutes</p>
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: String,
    /// <p>The compute and memory capacity of the replication instance.</p> <p> Valid Values: <code>dms.t2.micro | dms.t2.small | dms.t2.medium | dms.t2.large | dms.c4.large | dms.c4.xlarge | dms.c4.2xlarge | dms.c4.4xlarge </code> </p>
    #[serde(rename = "ReplicationInstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_class: Option<String>,
    /// <p>The replication instance identifier. This parameter is stored as a lowercase string.</p>
    #[serde(rename = "ReplicationInstanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_identifier: Option<String>,
    /// <p> Specifies the VPC security group to be used with the replication instance. The VPC security group must work with the VPC containing the replication instance. </p>
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<Vec<String>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ModifyReplicationInstanceResponse {
    /// <p>The modified replication instance.</p>
    #[serde(rename = "ReplicationInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance: Option<ReplicationInstance>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ModifyReplicationSubnetGroupMessage {
    /// <p>The description of the replication instance subnet group.</p>
    #[serde(rename = "ReplicationSubnetGroupDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group_description: Option<String>,
    /// <p>The name of the replication instance subnet group.</p>
    #[serde(rename = "ReplicationSubnetGroupIdentifier")]
    pub replication_subnet_group_identifier: String,
    /// <p>A list of subnet IDs.</p>
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ModifyReplicationSubnetGroupResponse {
    /// <p>The modified replication subnet group.</p>
    #[serde(rename = "ReplicationSubnetGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group: Option<ReplicationSubnetGroup>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ModifyReplicationTaskMessage {
    /// <p>Indicates when you want a change data capture (CDC) operation to start. Use either CdcStartPosition or CdcStartTime to specify when you want a CDC operation to start. Specifying both values results in an error.</p> <p> The value can be in date, checkpoint, or LSN/SCN format.</p> <p>Date Example: --cdc-start-position “2018-03-08T12:12:12”</p> <p>Checkpoint Example: --cdc-start-position "checkpoint:V1#27#mysql-bin-changelog.157832:1975:-1:2002:677883278264080:mysql-bin-changelog.157832:1876#0#0#*#0#93"</p> <p>LSN Example: --cdc-start-position “mysql-bin-changelog.000024:373”</p>
    #[serde(rename = "CdcStartPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_position: Option<String>,
    /// <p>Indicates the start time for a change data capture (CDC) operation. Use either CdcStartTime or CdcStartPosition to specify when you want a CDC operation to start. Specifying both values results in an error.</p> <p>Timestamp Example: --cdc-start-time “2018-03-08T12:12:12”</p>
    #[serde(rename = "CdcStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_time: Option<f64>,
    /// <p>Indicates when you want a change data capture (CDC) operation to stop. The value can be either server time or commit time.</p> <p>Server time example: --cdc-stop-position “server_time:3018-02-09T12:12:12”</p> <p>Commit time example: --cdc-stop-position “commit_time: 3018-02-09T12:12:12 “</p>
    #[serde(rename = "CdcStopPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_stop_position: Option<String>,
    /// <p>The migration type.</p> <p>Valid values: full-load | cdc | full-load-and-cdc</p>
    #[serde(rename = "MigrationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_type: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the replication task.</p>
    #[serde(rename = "ReplicationTaskArn")]
    pub replication_task_arn: String,
    /// <p><p>The replication task identifier.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 255 alphanumeric characters or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul></p>
    #[serde(rename = "ReplicationTaskIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_identifier: Option<String>,
    /// <p>JSON file that contains settings for the task, such as target metadata settings.</p>
    #[serde(rename = "ReplicationTaskSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_settings: Option<String>,
    /// <p>When using the AWS CLI or boto3, provide the path of the JSON file that contains the table mappings. Precede the path with "file://". When working with the DMS API, provide the JSON as the parameter value.</p> <p>For example, --table-mappings file://mappingfile.json</p>
    #[serde(rename = "TableMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_mappings: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ModifyReplicationTaskResponse {
    /// <p>The replication task that was modified.</p>
    #[serde(rename = "ReplicationTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MongoDbSettings {
    /// <p> The authentication mechanism you use to access the MongoDB source endpoint.</p> <p>Valid values: DEFAULT, MONGODB_CR, SCRAM_SHA_1 </p> <p>DEFAULT – For MongoDB version 2.x, use MONGODB_CR. For MongoDB version 3.x, use SCRAM_SHA_1. This attribute is not used when authType=No.</p>
    #[serde(rename = "AuthMechanism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_mechanism: Option<String>,
    /// <p> The MongoDB database name. This attribute is not used when <code>authType=NO</code>. </p> <p>The default is admin.</p>
    #[serde(rename = "AuthSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_source: Option<String>,
    /// <p> The authentication type you use to access the MongoDB source endpoint.</p> <p>Valid values: NO, PASSWORD </p> <p>When NO is selected, user name and password parameters are not used and can be empty. </p>
    #[serde(rename = "AuthType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// <p> The database name on the MongoDB source endpoint. </p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p> Indicates the number of documents to preview to determine the document organization. Use this attribute when <code>NestingLevel</code> is set to ONE. </p> <p>Must be a positive value greater than 0. Default value is 1000.</p>
    #[serde(rename = "DocsToInvestigate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs_to_investigate: Option<String>,
    /// <p> Specifies the document ID. Use this attribute when <code>NestingLevel</code> is set to NONE. </p> <p>Default value is false. </p>
    #[serde(rename = "ExtractDocId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_doc_id: Option<String>,
    /// <p>The AWS KMS key identifier that is used to encrypt the content on the replication instance. If you don't specify a value for the <code>KmsKeyId</code> parameter, then AWS DMS uses your default encryption key. AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS Region.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p> Specifies either document or table mode. </p> <p>Valid values: NONE, ONE</p> <p>Default value is NONE. Specify NONE to use document mode. Specify ONE to use table mode.</p>
    #[serde(rename = "NestingLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nesting_level: Option<String>,
    /// <p> The password for the user account you use to access the MongoDB source endpoint. </p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p> The port value for the MongoDB source endpoint. </p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p> The name of the server on the MongoDB source endpoint. </p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// <p>The user name you use to access the MongoDB source endpoint. </p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct OrderableReplicationInstance {
    /// <p>List of availability zones for this replication instance.</p>
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>The default amount of storage (in gigabytes) that is allocated for the replication instance.</p>
    #[serde(rename = "DefaultAllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_allocated_storage: Option<i64>,
    /// <p>The version of the replication engine.</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>The amount of storage (in gigabytes) that is allocated for the replication instance.</p>
    #[serde(rename = "IncludedAllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_allocated_storage: Option<i64>,
    /// <p>The minimum amount of storage (in gigabytes) that can be allocated for the replication instance.</p>
    #[serde(rename = "MaxAllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_allocated_storage: Option<i64>,
    /// <p>The minimum amount of storage (in gigabytes) that can be allocated for the replication instance.</p>
    #[serde(rename = "MinAllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_allocated_storage: Option<i64>,
    /// <p>The compute and memory capacity of the replication instance.</p> <p> Valid Values: <code>dms.t2.micro | dms.t2.small | dms.t2.medium | dms.t2.large | dms.c4.large | dms.c4.xlarge | dms.c4.2xlarge | dms.c4.4xlarge </code> </p>
    #[serde(rename = "ReplicationInstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_class: Option<String>,
    /// <p>The type of storage used by the replication instance.</p>
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PendingMaintenanceAction {
    /// <p>The type of pending maintenance action that is available for the resource.</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The date of the maintenance window when the action will be applied. The maintenance action will be applied to the resource during its first maintenance window after this date. If this date is specified, any <code>next-maintenance</code> opt-in requests are ignored.</p>
    #[serde(rename = "AutoAppliedAfterDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_applied_after_date: Option<f64>,
    /// <p>The effective date when the pending maintenance action will be applied to the resource. This date takes into account opt-in requests received from the <code>ApplyPendingMaintenanceAction</code> API, the <code>AutoAppliedAfterDate</code>, and the <code>ForcedApplyDate</code>. This value is blank if an opt-in request has not been received and nothing has been specified as <code>AutoAppliedAfterDate</code> or <code>ForcedApplyDate</code>.</p>
    #[serde(rename = "CurrentApplyDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_apply_date: Option<f64>,
    /// <p>A description providing more detail about the maintenance action.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The date when the maintenance action will be automatically applied. The maintenance action will be applied to the resource on this date regardless of the maintenance window for the resource. If this date is specified, any <code>immediate</code> opt-in requests are ignored.</p>
    #[serde(rename = "ForcedApplyDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forced_apply_date: Option<f64>,
    /// <p>Indicates the type of opt-in request that has been received for the resource.</p>
    #[serde(rename = "OptInStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_in_status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RebootReplicationInstanceMessage {
    /// <p>If this parameter is <code>true</code>, the reboot is conducted through a Multi-AZ failover. (If the instance isn't configured for Multi-AZ, then you can't specify <code>true</code>.)</p>
    #[serde(rename = "ForceFailover")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_failover: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RebootReplicationInstanceResponse {
    /// <p>The replication instance that is being rebooted. </p>
    #[serde(rename = "ReplicationInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance: Option<ReplicationInstance>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RedshiftSettings {
    /// <p>Allows any date format, including invalid formats such as 00/00/00 00:00:00, to be loaded without generating an error. You can choose TRUE or FALSE (default).</p> <p>This parameter applies only to TIMESTAMP and DATE columns. Always use ACCEPTANYDATE with the DATEFORMAT parameter. If the date format for the data does not match the DATEFORMAT specification, Amazon Redshift inserts a NULL value into that field. </p>
    #[serde(rename = "AcceptAnyDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_any_date: Option<bool>,
    /// <p>Code to run after connecting. This should be the code, not a filename.</p>
    #[serde(rename = "AfterConnectScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_connect_script: Option<String>,
    /// <p>The location where the CSV files are stored before being uploaded to the S3 bucket. </p>
    #[serde(rename = "BucketFolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_folder: Option<String>,
    /// <p>The name of the S3 bucket you want to use</p>
    #[serde(rename = "BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    /// <p>Sets the amount of time to wait (in milliseconds) before timing out, beginning from when you initially establish a connection.</p>
    #[serde(rename = "ConnectionTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_timeout: Option<i64>,
    /// <p>The name of the Amazon Redshift data warehouse (service) you are working with.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>The date format you are using. Valid values are <code>auto</code> (case-sensitive), your date format string enclosed in quotes, or NULL. If this is left unset (NULL), it defaults to a format of 'YYYY-MM-DD'. Using <code>auto</code> recognizes most strings, even some that are not supported when you use a date format string. </p> <p>If your date and time values use formats different from each other, set this to <code>auto</code>. </p>
    #[serde(rename = "DateFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_format: Option<String>,
    /// <p>Specifies whether AWS DMS should migrate empty CHAR and VARCHAR fields as NULL. A value of TRUE sets empty CHAR and VARCHAR fields to null. The default is FALSE.</p>
    #[serde(rename = "EmptyAsNull")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_as_null: Option<bool>,
    /// <p>The type of server side encryption you want to use for your data. This is part of the endpoint settings or the extra connections attributes for Amazon S3. You can choose either SSE_S3 (default) or SSE_KMS. To use SSE_S3, create an IAM role with a policy that allows <code>"arn:aws:s3:::*"</code> to use the following actions: <code>"s3:PutObject", "s3:ListBucket"</code>.</p>
    #[serde(rename = "EncryptionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_mode: Option<String>,
    /// <p>Specifies the number of threads used to upload a single file. This accepts a value between 1 and 64. It defaults to 10.</p>
    #[serde(rename = "FileTransferUploadStreams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_transfer_upload_streams: Option<i64>,
    /// <p>Sets the amount of time to wait (in milliseconds) before timing out, beginning from when you begin loading.</p>
    #[serde(rename = "LoadTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_timeout: Option<i64>,
    /// <p>Specifies the maximum size (in KB) of any CSV file used to transfer data to Amazon Redshift. This accepts a value between 1 and 1048576. It defaults to 32768 KB (32 MB).</p>
    #[serde(rename = "MaxFileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<i64>,
    /// <p>The password for the user named in the username property.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The port number for Amazon Redshift. The default value is 5439.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>Removes surrounding quotation marks from strings in the incoming data. All characters within the quotation marks, including delimiters, are retained. Choose TRUE to remove quotation marks. The default is FALSE.</p>
    #[serde(rename = "RemoveQuotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_quotes: Option<bool>,
    /// <p>Replaces invalid characters specified in <code>ReplaceInvalidChars</code>, substituting the specified value instead. The default is "?".</p>
    #[serde(rename = "ReplaceChars")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_chars: Option<String>,
    /// <p>A list of chars you want to replace. Use with <code>ReplaceChars</code>.</p>
    #[serde(rename = "ReplaceInvalidChars")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_invalid_chars: Option<String>,
    /// <p>The name of the Amazon Redshift cluster you are using.</p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// <p>If you are using SSE_KMS for the <code>EncryptionMode</code>, provide the KMS Key ID. The key you use needs an attached policy that enables IAM user permissions and allows use of the key.</p>
    #[serde(rename = "ServerSideEncryptionKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_kms_key_id: Option<String>,
    /// <p>The ARN of the role that has access to the Redshift service.</p>
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    /// <p>The time format you want to use. Valid values are <code>auto</code> (case-sensitive), 'timeformat_string', 'epochsecs', or 'epochmillisecs'. It defaults to 10. Using <code>auto</code> recognizes most strings, even some that are not supported when you use a time format string. </p> <p>If your date and time values use formats different from each other, set this to <code>auto</code>. </p>
    #[serde(rename = "TimeFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_format: Option<String>,
    /// <p>Removes the trailing white space characters from a VARCHAR string. This parameter applies only to columns with a VARCHAR data type. Choose TRUE to remove unneeded white space. The default is FALSE.</p>
    #[serde(rename = "TrimBlanks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim_blanks: Option<bool>,
    /// <p>Truncates data in columns to the appropriate number of characters, so that it fits in the column. Applies only to columns with a VARCHAR or CHAR data type, and rows with a size of 4 MB or less. Choose TRUE to truncate data. The default is FALSE.</p>
    #[serde(rename = "TruncateColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncate_columns: Option<bool>,
    /// <p>An Amazon Redshift user name for a registered user.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// <p>The size of the write buffer to use in rows. Valid values range from 1 to 2048. Defaults to 1024. Use this setting to tune performance. </p>
    #[serde(rename = "WriteBufferSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_buffer_size: Option<i64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RefreshSchemasMessage {
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RefreshSchemasResponse {
    /// <p>The status of the refreshed schema.</p>
    #[serde(rename = "RefreshSchemasStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_schemas_status: Option<RefreshSchemasStatus>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RefreshSchemasStatus {
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_arn: Option<String>,
    /// <p>The last failure message for the schema.</p>
    #[serde(rename = "LastFailureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_message: Option<String>,
    /// <p>The date the schema was last refreshed.</p>
    #[serde(rename = "LastRefreshDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refresh_date: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_arn: Option<String>,
    /// <p>The status of the schema.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ReloadTablesMessage {
    /// <p>Options for reload. Specify <code>data-reload</code> to reload the data and re-validate it if validation is enabled. Specify <code>validate-only</code> to re-validate the table. This option applies only when validation is enabled for the task. </p> <p>Valid values: data-reload, validate-only</p> <p>Default value is data-reload.</p>
    #[serde(rename = "ReloadOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reload_option: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the replication task. </p>
    #[serde(rename = "ReplicationTaskArn")]
    pub replication_task_arn: String,
    /// <p>The name and schema of the table to be reloaded. </p>
    #[serde(rename = "TablesToReload")]
    pub tables_to_reload: Vec<TableToReload>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ReloadTablesResponse {
    /// <p>The Amazon Resource Name (ARN) of the replication task. </p>
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_arn: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RemoveTagsFromResourceMessage {
    /// <p>&gt;The Amazon Resource Name (ARN) of the AWS DMS resource the tag is to be removed from.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tag key (name) of the tag to be removed.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RemoveTagsFromResourceResponse {}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ReplicationInstance {
    /// <p>The amount of storage (in gigabytes) that is allocated for the replication instance.</p>
    #[serde(rename = "AllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i64>,
    /// <p>Boolean value indicating if minor version upgrades will be automatically applied to the instance.</p>
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>The Availability Zone for the instance.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The DNS name servers for the replication instance.</p>
    #[serde(rename = "DnsNameServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name_servers: Option<String>,
    /// <p>The engine version number of the replication instance.</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p> The expiration date of the free replication instance that is part of the Free DMS program. </p>
    #[serde(rename = "FreeUntil")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_until: Option<f64>,
    /// <p>The time the replication instance was created.</p>
    #[serde(rename = "InstanceCreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_create_time: Option<f64>,
    /// <p>The AWS KMS key identifier that is used to encrypt the content on the replication instance. If you don't specify a value for the <code>KmsKeyId</code> parameter, then AWS DMS uses your default encryption key. AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS Region.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p> Specifies if the replication instance is a Multi-AZ deployment. You cannot set the <code>AvailabilityZone</code> parameter if the Multi-AZ parameter is set to <code>true</code>. </p>
    #[serde(rename = "MultiAZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_az: Option<bool>,
    /// <p>The pending modification values.</p>
    #[serde(rename = "PendingModifiedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_modified_values: Option<ReplicationPendingModifiedValues>,
    /// <p>The maintenance window times for the replication instance.</p>
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p> Specifies the accessibility options for the replication instance. A value of <code>true</code> represents an instance with a public IP address. A value of <code>false</code> represents an instance with a private IP address. The default value is <code>true</code>. </p>
    #[serde(rename = "PubliclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_arn: Option<String>,
    /// <p>The compute and memory capacity of the replication instance.</p> <p> Valid Values: <code>dms.t2.micro | dms.t2.small | dms.t2.medium | dms.t2.large | dms.c4.large | dms.c4.xlarge | dms.c4.2xlarge | dms.c4.4xlarge </code> </p>
    #[serde(rename = "ReplicationInstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_class: Option<String>,
    /// <p>The replication instance identifier. This parameter is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 alphanumeric characters or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>myrepinstance</code> </p>
    #[serde(rename = "ReplicationInstanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_identifier: Option<String>,
    /// <p>The private IP address of the replication instance.</p>
    #[serde(rename = "ReplicationInstancePrivateIpAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_private_ip_addresses: Option<Vec<String>>,
    /// <p>The public IP address of the replication instance.</p>
    #[serde(rename = "ReplicationInstancePublicIpAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_public_ip_addresses: Option<Vec<String>>,
    /// <p>The status of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_status: Option<String>,
    /// <p>The subnet group for the replication instance.</p>
    #[serde(rename = "ReplicationSubnetGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group: Option<ReplicationSubnetGroup>,
    /// <p>The availability zone of the standby replication instance in a Multi-AZ deployment.</p>
    #[serde(rename = "SecondaryAvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_availability_zone: Option<String>,
    /// <p>The VPC security group for the instance.</p>
    #[serde(rename = "VpcSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_groups: Option<Vec<VpcSecurityGroupMembership>>,
}

/// <p>Contains metadata for a replication instance task log.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ReplicationInstanceTaskLog {
    /// <p>The size, in bytes, of the replication task log.</p>
    #[serde(rename = "ReplicationInstanceTaskLogSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_task_log_size: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the replication task.</p>
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_arn: Option<String>,
    /// <p>The name of the replication task.</p>
    #[serde(rename = "ReplicationTaskName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_name: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ReplicationPendingModifiedValues {
    /// <p>The amount of storage (in gigabytes) that is allocated for the replication instance.</p>
    #[serde(rename = "AllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i64>,
    /// <p>The engine version number of the replication instance.</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p> Specifies if the replication instance is a Multi-AZ deployment. You cannot set the <code>AvailabilityZone</code> parameter if the Multi-AZ parameter is set to <code>true</code>. </p>
    #[serde(rename = "MultiAZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_az: Option<bool>,
    /// <p>The compute and memory capacity of the replication instance.</p> <p> Valid Values: <code>dms.t2.micro | dms.t2.small | dms.t2.medium | dms.t2.large | dms.c4.large | dms.c4.xlarge | dms.c4.2xlarge | dms.c4.4xlarge </code> </p>
    #[serde(rename = "ReplicationInstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_class: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ReplicationSubnetGroup {
    /// <p>The description of the replication subnet group.</p>
    #[serde(rename = "ReplicationSubnetGroupDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group_description: Option<String>,
    /// <p>The identifier of the replication instance subnet group.</p>
    #[serde(rename = "ReplicationSubnetGroupIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group_identifier: Option<String>,
    /// <p>The status of the subnet group.</p>
    #[serde(rename = "SubnetGroupStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_status: Option<String>,
    /// <p>The subnets that are in the subnet group.</p>
    #[serde(rename = "Subnets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<Subnet>>,
    /// <p>The ID of the VPC.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ReplicationTask {
    /// <p>Indicates when you want a change data capture (CDC) operation to start. Use either CdcStartPosition or CdcStartTime to specify when you want a CDC operation to start. Specifying both values results in an error.</p> <p> The value can be in date, checkpoint, or LSN/SCN format.</p> <p>Date Example: --cdc-start-position “2018-03-08T12:12:12”</p> <p>Checkpoint Example: --cdc-start-position "checkpoint:V1#27#mysql-bin-changelog.157832:1975:-1:2002:677883278264080:mysql-bin-changelog.157832:1876#0#0#*#0#93"</p> <p>LSN Example: --cdc-start-position “mysql-bin-changelog.000024:373”</p>
    #[serde(rename = "CdcStartPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_position: Option<String>,
    /// <p>Indicates when you want a change data capture (CDC) operation to stop. The value can be either server time or commit time.</p> <p>Server time example: --cdc-stop-position “server_time:3018-02-09T12:12:12”</p> <p>Commit time example: --cdc-stop-position “commit_time: 3018-02-09T12:12:12 “</p>
    #[serde(rename = "CdcStopPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_stop_position: Option<String>,
    /// <p>The last error (failure) message generated for the replication instance.</p>
    #[serde(rename = "LastFailureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_message: Option<String>,
    /// <p>The type of migration.</p>
    #[serde(rename = "MigrationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_type: Option<String>,
    /// <p>Indicates the last checkpoint that occurred during a change data capture (CDC) operation. You can provide this value to the <code>CdcStartPosition</code> parameter to start a CDC operation that begins at that checkpoint.</p>
    #[serde(rename = "RecoveryCheckpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_checkpoint: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the replication task.</p>
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_arn: Option<String>,
    /// <p>The date the replication task was created.</p>
    #[serde(rename = "ReplicationTaskCreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_creation_date: Option<f64>,
    /// <p><p>The user-assigned replication task identifier or name.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 255 alphanumeric characters or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul></p>
    #[serde(rename = "ReplicationTaskIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_identifier: Option<String>,
    /// <p>The settings for the replication task.</p>
    #[serde(rename = "ReplicationTaskSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_settings: Option<String>,
    /// <p>The date the replication task is scheduled to start.</p>
    #[serde(rename = "ReplicationTaskStartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_start_date: Option<f64>,
    /// <p>The statistics for the task, including elapsed time, tables loaded, and table errors.</p>
    #[serde(rename = "ReplicationTaskStats")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_stats: Option<ReplicationTaskStats>,
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    #[serde(rename = "SourceEndpointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_endpoint_arn: Option<String>,
    /// <p>The status of the replication task.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The reason the replication task was stopped.</p>
    #[serde(rename = "StopReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<String>,
    /// <p>Table mappings specified in the task.</p>
    #[serde(rename = "TableMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_mappings: Option<String>,
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    #[serde(rename = "TargetEndpointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_endpoint_arn: Option<String>,
}

/// <p> The task assessment report in JSON format. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ReplicationTaskAssessmentResult {
    /// <p> The task assessment results in JSON format. </p>
    #[serde(rename = "AssessmentResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_results: Option<String>,
    /// <p> The file containing the results of the task assessment. </p>
    #[serde(rename = "AssessmentResultsFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_results_file: Option<String>,
    /// <p> The status of the task assessment. </p>
    #[serde(rename = "AssessmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_status: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the replication task. </p>
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_arn: Option<String>,
    /// <p> The replication task identifier of the task on which the task assessment was run. </p>
    #[serde(rename = "ReplicationTaskIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_identifier: Option<String>,
    /// <p>The date the task assessment was completed. </p>
    #[serde(rename = "ReplicationTaskLastAssessmentDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_last_assessment_date: Option<f64>,
    /// <p> The URL of the S3 object containing the task assessment results. </p>
    #[serde(rename = "S3ObjectUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_url: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ReplicationTaskStats {
    /// <p>The elapsed time of the task, in milliseconds.</p>
    #[serde(rename = "ElapsedTimeMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_time_millis: Option<i64>,
    /// <p>The percent complete for the full load migration task.</p>
    #[serde(rename = "FullLoadProgressPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_progress_percent: Option<i64>,
    /// <p>The number of errors that have occurred during this task.</p>
    #[serde(rename = "TablesErrored")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_errored: Option<i64>,
    /// <p>The number of tables loaded for this task.</p>
    #[serde(rename = "TablesLoaded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_loaded: Option<i64>,
    /// <p>The number of tables currently loading for this task.</p>
    #[serde(rename = "TablesLoading")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_loading: Option<i64>,
    /// <p>The number of tables queued for this task.</p>
    #[serde(rename = "TablesQueued")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_queued: Option<i64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResourcePendingMaintenanceActions {
    /// <p>Detailed information about the pending maintenance action.</p>
    #[serde(rename = "PendingMaintenanceActionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_maintenance_action_details: Option<Vec<PendingMaintenanceAction>>,
    /// <p>The Amazon Resource Name (ARN) of the DMS resource that the pending maintenance action applies to. For information about creating an ARN, see <a href="https://docs.aws.amazon.com/dms/latest/UserGuide/USER_Tagging.html#USER_Tagging.ARN"> Constructing an Amazon Resource Name (ARN)</a> in the DMS documentation.</p>
    #[serde(rename = "ResourceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
}

/// <p>Settings for exporting data to Amazon S3. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3Settings {
    /// <p> An optional parameter to set a folder name in the S3 bucket. If provided, tables are created in the path <code>&lt;bucketFolder&gt;/&lt;schema_name&gt;/&lt;table_name&gt;/</code>. If this parameter is not specified, then the path used is <code>&lt;schema_name&gt;/&lt;table_name&gt;/</code>. </p>
    #[serde(rename = "BucketFolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_folder: Option<String>,
    /// <p> The name of the S3 bucket. </p>
    #[serde(rename = "BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    /// <p>Option to write only <code>INSERT</code> operations to the comma-separated value (CSV) output files. By default, the first field in a CSV record contains the letter <code>I</code> (insert), <code>U</code> (update) or <code>D</code> (delete) to indicate whether the row was inserted, updated, or deleted at the source database. If <code>cdcInsertsOnly</code> is set to true, then only <code>INSERT</code>s are recorded in the CSV file, without the <code>I</code> annotation on each line. Valid values are <code>TRUE</code> and <code>FALSE</code>.</p>
    #[serde(rename = "CdcInsertsOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_inserts_only: Option<bool>,
    /// <p> An optional parameter to use GZIP to compress the target files. Set to GZIP to compress the target files. Set to NONE (the default) or do not use to leave the files uncompressed. Applies to both CSV and PARQUET data formats. </p>
    #[serde(rename = "CompressionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_type: Option<String>,
    /// <p> The delimiter used to separate columns in the source files. The default is a comma. </p>
    #[serde(rename = "CsvDelimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_delimiter: Option<String>,
    /// <p> The delimiter used to separate rows in the source files. The default is a carriage return (<code>\n</code>). </p>
    #[serde(rename = "CsvRowDelimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_row_delimiter: Option<String>,
    /// <p><p>The format of the data which you want to use for output. You can choose one of the following: </p> <ul> <li> <p> <code>CSV</code> : This is a row-based format with comma-separated values. </p> </li> <li> <p> <code>PARQUET</code> : Apache Parquet is a columnar storage format that features efficient compression and provides faster query response. </p> </li> </ul></p>
    #[serde(rename = "DataFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format: Option<String>,
    /// <p>The size of one data page in bytes. Defaults to 1024 * 1024 bytes (1MiB). For <code>PARQUET</code> format only. </p>
    #[serde(rename = "DataPageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_page_size: Option<i64>,
    /// <p>The maximum size of an encoded dictionary page of a column. If the dictionary page exceeds this, this column is stored using an encoding type of <code>PLAIN</code>. Defaults to 1024 * 1024 bytes (1MiB), the maximum size of a dictionary page before it reverts to <code>PLAIN</code> encoding. For <code>PARQUET</code> format only. </p>
    #[serde(rename = "DictPageSizeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dict_page_size_limit: Option<i64>,
    /// <p>Enables statistics for Parquet pages and rowGroups. Choose <code>TRUE</code> to enable statistics, choose <code>FALSE</code> to disable. Statistics include <code>NULL</code>, <code>DISTINCT</code>, <code>MAX</code>, and <code>MIN</code> values. Defaults to <code>TRUE</code>. For <code>PARQUET</code> format only.</p>
    #[serde(rename = "EnableStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_statistics: Option<bool>,
    /// <p><p>The type of encoding you are using: <code>RLE<em>DICTIONARY</code> (default), <code>PLAIN</code>, or <code>PLAIN</em>DICTIONARY</code>.</p> <ul> <li> <p> <code>RLE<em>DICTIONARY</code> uses a combination of bit-packing and run-length encoding to store repeated values more efficiently.</p> </li> <li> <p> <code>PLAIN</code> does not use encoding at all. Values are stored as they are.</p> </li> <li> <p> <code>PLAIN</em>DICTIONARY</code> builds a dictionary of the values encountered in a given column. The dictionary is stored in a dictionary page for each column chunk.</p> </li> </ul></p>
    #[serde(rename = "EncodingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    /// <p><p>The type of server side encryption you want to use for your data. This is part of the endpoint settings or the extra connections attributes for Amazon S3. You can choose either <code>SSE<em>S3</code> (default) or <code>SSE</em>KMS</code>. To use <code>SSE_S3</code>, you need an IAM role with permission to allow <code>&quot;arn:aws:s3:::dms-*&quot;</code> to use the following actions:</p> <ul> <li> <p>s3:CreateBucket</p> </li> <li> <p>s3:ListBucket</p> </li> <li> <p>s3:DeleteBucket</p> </li> <li> <p>s3:GetBucketLocation</p> </li> <li> <p>s3:GetObject</p> </li> <li> <p>s3:PutObject</p> </li> <li> <p>s3:DeleteObject</p> </li> <li> <p>s3:GetObjectVersion</p> </li> <li> <p>s3:GetBucketPolicy</p> </li> <li> <p>s3:PutBucketPolicy</p> </li> <li> <p>s3:DeleteBucketPolicy</p> </li> </ul></p>
    #[serde(rename = "EncryptionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_mode: Option<String>,
    /// <p> The external table definition. </p>
    #[serde(rename = "ExternalTableDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_table_definition: Option<String>,
    /// <p>The version of Apache Parquet format you want to use: <code>PARQUET_1_0</code> (default) or <code>PARQUET_2_0</code>.</p>
    #[serde(rename = "ParquetVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parquet_version: Option<String>,
    /// <p>The number of rows in a row group. A smaller row group size provides faster reads. But as the number of row groups grows, the slower writes become. Defaults to 10,000 (ten thousand) rows. For <code>PARQUET</code> format only. </p> <p>If you choose a value larger than the maximum, <code>RowGroupLength</code> is set to the max row group length in bytes (64 * 1024 * 1024). </p>
    #[serde(rename = "RowGroupLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_group_length: Option<i64>,
    /// <p>If you are using SSE_KMS for the <code>EncryptionMode</code>, provide the KMS Key ID. The key you use needs an attached policy that enables IAM user permissions and allows use of the key.</p> <p>Here is a CLI example: <code>aws dms create-endpoint --endpoint-identifier &lt;value&gt; --endpoint-type target --engine-name s3 --s3-settings ServiceAccessRoleArn=&lt;value&gt;,BucketFolder=&lt;value&gt;,BucketName=&lt;value&gt;,EncryptionMode=SSE_KMS,ServerSideEncryptionKmsKeyId=&lt;value&gt; </code> </p>
    #[serde(rename = "ServerSideEncryptionKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_kms_key_id: Option<String>,
    /// <p> The Amazon Resource Name (ARN) used by the service access IAM role. </p>
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartReplicationTaskAssessmentMessage {
    /// <p> The Amazon Resource Name (ARN) of the replication task. </p>
    #[serde(rename = "ReplicationTaskArn")]
    pub replication_task_arn: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartReplicationTaskAssessmentResponse {
    /// <p> The assessed replication task. </p>
    #[serde(rename = "ReplicationTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartReplicationTaskMessage {
    /// <p>Indicates when you want a change data capture (CDC) operation to start. Use either CdcStartPosition or CdcStartTime to specify when you want a CDC operation to start. Specifying both values results in an error.</p> <p> The value can be in date, checkpoint, or LSN/SCN format.</p> <p>Date Example: --cdc-start-position “2018-03-08T12:12:12”</p> <p>Checkpoint Example: --cdc-start-position "checkpoint:V1#27#mysql-bin-changelog.157832:1975:-1:2002:677883278264080:mysql-bin-changelog.157832:1876#0#0#*#0#93"</p> <p>LSN Example: --cdc-start-position “mysql-bin-changelog.000024:373”</p>
    #[serde(rename = "CdcStartPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_position: Option<String>,
    /// <p>Indicates the start time for a change data capture (CDC) operation. Use either CdcStartTime or CdcStartPosition to specify when you want a CDC operation to start. Specifying both values results in an error.</p> <p>Timestamp Example: --cdc-start-time “2018-03-08T12:12:12”</p>
    #[serde(rename = "CdcStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_time: Option<f64>,
    /// <p>Indicates when you want a change data capture (CDC) operation to stop. The value can be either server time or commit time.</p> <p>Server time example: --cdc-stop-position “server_time:3018-02-09T12:12:12”</p> <p>Commit time example: --cdc-stop-position “commit_time: 3018-02-09T12:12:12 “</p>
    #[serde(rename = "CdcStopPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_stop_position: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the replication task to be started.</p>
    #[serde(rename = "ReplicationTaskArn")]
    pub replication_task_arn: String,
    /// <p>The type of replication task.</p>
    #[serde(rename = "StartReplicationTaskType")]
    pub start_replication_task_type: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartReplicationTaskResponse {
    /// <p>The replication task started.</p>
    #[serde(rename = "ReplicationTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopReplicationTaskMessage {
    /// <p>The Amazon Resource Name(ARN) of the replication task to be stopped.</p>
    #[serde(rename = "ReplicationTaskArn")]
    pub replication_task_arn: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StopReplicationTaskResponse {
    /// <p>The replication task stopped.</p>
    #[serde(rename = "ReplicationTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Subnet {
    /// <p>The Availability Zone of the subnet.</p>
    #[serde(rename = "SubnetAvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_availability_zone: Option<AvailabilityZone>,
    /// <p>The subnet identifier.</p>
    #[serde(rename = "SubnetIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_identifier: Option<String>,
    /// <p>The status of the subnet.</p>
    #[serde(rename = "SubnetStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_status: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SupportedEndpointType {
    /// <p>The type of endpoint.</p>
    #[serde(rename = "EndpointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    /// <p>The expanded name for the engine name. For example, if the <code>EngineName</code> parameter is "aurora," this value would be "Amazon Aurora MySQL."</p>
    #[serde(rename = "EngineDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_display_name: Option<String>,
    /// <p>The database engine name. Valid values, depending on the EndPointType, include mysql, oracle, postgres, mariadb, aurora, aurora-postgresql, redshift, s3, db2, azuredb, sybase, sybase, dynamodb, mongodb, and sqlserver.</p>
    #[serde(rename = "EngineName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_name: Option<String>,
    /// <p>Indicates if Change Data Capture (CDC) is supported.</p>
    #[serde(rename = "SupportsCDC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_cdc: Option<bool>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TableStatistics {
    /// <p>The Data Definition Language (DDL) used to build and modify the structure of your tables.</p>
    #[serde(rename = "Ddls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ddls: Option<i64>,
    /// <p>The number of delete actions performed on a table.</p>
    #[serde(rename = "Deletes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletes: Option<i64>,
    /// <p>The number of rows that failed conditional checks during the Full Load operation (valid only for DynamoDB as a target migrations).</p>
    #[serde(rename = "FullLoadCondtnlChkFailedRows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_condtnl_chk_failed_rows: Option<i64>,
    /// <p>The number of rows that failed to load during the Full Load operation (valid only for DynamoDB as a target migrations).</p>
    #[serde(rename = "FullLoadErrorRows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_error_rows: Option<i64>,
    /// <p>The number of rows added during the Full Load operation.</p>
    #[serde(rename = "FullLoadRows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_rows: Option<i64>,
    /// <p>The number of insert actions performed on a table.</p>
    #[serde(rename = "Inserts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inserts: Option<i64>,
    /// <p>The last time the table was updated.</p>
    #[serde(rename = "LastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The schema name.</p>
    #[serde(rename = "SchemaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    /// <p>The name of the table.</p>
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// <p>The state of the tables described.</p> <p>Valid states: Table does not exist | Before load | Full load | Table completed | Table cancelled | Table error | Table all | Table updates | Table is being reloaded</p>
    #[serde(rename = "TableState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_state: Option<String>,
    /// <p>The number of update actions performed on a table.</p>
    #[serde(rename = "Updates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates: Option<i64>,
    /// <p>The number of records that failed validation.</p>
    #[serde(rename = "ValidationFailedRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_failed_records: Option<i64>,
    /// <p>The number of records that have yet to be validated.</p>
    #[serde(rename = "ValidationPendingRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_pending_records: Option<i64>,
    /// <p><p>The validation state of the table.</p> <p>The parameter can have the following values</p> <ul> <li> <p>Not enabled—Validation is not enabled for the table in the migration task.</p> </li> <li> <p>Pending records—Some records in the table are waiting for validation.</p> </li> <li> <p>Mismatched records—Some records in the table do not match between the source and target.</p> </li> <li> <p>Suspended records—Some records in the table could not be validated.</p> </li> <li> <p>No primary key—The table could not be validated because it had no primary key.</p> </li> <li> <p>Table error—The table was not validated because it was in an error state and some data was not migrated.</p> </li> <li> <p>Validated—All rows in the table were validated. If the table is updated, the status can change from Validated.</p> </li> <li> <p>Error—The table could not be validated because of an unexpected error.</p> </li> </ul></p>
    #[serde(rename = "ValidationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_state: Option<String>,
    /// <p>Additional details about the state of validation.</p>
    #[serde(rename = "ValidationStateDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_state_details: Option<String>,
    /// <p>The number of records that could not be validated.</p>
    #[serde(rename = "ValidationSuspendedRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_suspended_records: Option<i64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TableToReload {
    /// <p>The schema name of the table to be reloaded.</p>
    #[serde(rename = "SchemaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    /// <p>The table name of the table to be reloaded.</p>
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>A key is the required name of the tag. The string value can be from 1 to 128 Unicode characters in length and cannot be prefixed with "aws:" or "dms:". The string can only contain only the set of Unicode letters, digits, white-space, '_', '.', '/', '=', '+', '-' (Java regex: "^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-]*)$").</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>A value is the optional value of the tag. The string value can be from 1 to 256 Unicode characters in length and cannot be prefixed with "aws:" or "dms:". The string can only contain only the set of Unicode letters, digits, white-space, '_', '.', '/', '=', '+', '-' (Java regex: "^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-]*)$").</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TestConnectionMessage {
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TestConnectionResponse {
    /// <p>The connection tested.</p>
    #[serde(rename = "Connection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<Connection>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct VpcSecurityGroupMembership {
    /// <p>The status of the VPC security group.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The VPC security group Id.</p>
    #[serde(rename = "VpcSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_id: Option<String>,
}

/// Errors returned by AddTagsToResource
#[derive(Debug, PartialEq)]
pub enum AddTagsToResourceError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl AddTagsToResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddTagsToResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(AddTagsToResourceError::ResourceNotFoundFault(
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
impl fmt::Display for AddTagsToResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddTagsToResourceError {
    fn description(&self) -> &str {
        match *self {
            AddTagsToResourceError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by ApplyPendingMaintenanceAction
#[derive(Debug, PartialEq)]
pub enum ApplyPendingMaintenanceActionError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl ApplyPendingMaintenanceActionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ApplyPendingMaintenanceActionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        ApplyPendingMaintenanceActionError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ApplyPendingMaintenanceActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ApplyPendingMaintenanceActionError {
    fn description(&self) -> &str {
        match *self {
            ApplyPendingMaintenanceActionError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateEndpoint
#[derive(Debug, PartialEq)]
pub enum CreateEndpointError {
    /// <p>AWS DMS was denied access to the endpoint. Check that the role is correctly configured.</p>
    AccessDeniedFault(String),
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>AWS DMS cannot access the KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// <p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
}

impl CreateEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedFault" => {
                    return RusotoError::Service(CreateEndpointError::AccessDeniedFault(err.msg))
                }
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(CreateEndpointError::InvalidResourceStateFault(
                        err.msg,
                    ))
                }
                "KMSKeyNotAccessibleFault" => {
                    return RusotoError::Service(CreateEndpointError::KMSKeyNotAccessibleFault(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsFault" => {
                    return RusotoError::Service(CreateEndpointError::ResourceAlreadyExistsFault(
                        err.msg,
                    ))
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(CreateEndpointError::ResourceNotFoundFault(
                        err.msg,
                    ))
                }
                "ResourceQuotaExceededFault" => {
                    return RusotoError::Service(CreateEndpointError::ResourceQuotaExceededFault(
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
impl fmt::Display for CreateEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateEndpointError {
    fn description(&self) -> &str {
        match *self {
            CreateEndpointError::AccessDeniedFault(ref cause) => cause,
            CreateEndpointError::InvalidResourceStateFault(ref cause) => cause,
            CreateEndpointError::KMSKeyNotAccessibleFault(ref cause) => cause,
            CreateEndpointError::ResourceAlreadyExistsFault(ref cause) => cause,
            CreateEndpointError::ResourceNotFoundFault(ref cause) => cause,
            CreateEndpointError::ResourceQuotaExceededFault(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateEventSubscription
#[derive(Debug, PartialEq)]
pub enum CreateEventSubscriptionError {
    /// <p>The ciphertext references a key that doesn't exist or DMS account doesn't have an access to</p>
    KMSAccessDeniedFault(String),
    /// <p>The specified master key (CMK) isn't enabled.</p>
    KMSDisabledFault(String),
    /// <p>The state of the specified KMS resource isn't valid for this request.</p>
    KMSInvalidStateFault(String),
    /// <p>The specified KMS entity or resource can't be found.</p>
    KMSNotFoundFault(String),
    /// <p>This request triggered KMS request throttling.</p>
    KMSThrottlingFault(String),
    /// <p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// <p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
    /// <p>The SNS topic is invalid.</p>
    SNSInvalidTopicFault(String),
    /// <p>You are not authorized for the SNS subscription.</p>
    SNSNoAuthorizationFault(String),
}

impl CreateEventSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateEventSubscriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "KMSAccessDeniedFault" => {
                    return RusotoError::Service(
                        CreateEventSubscriptionError::KMSAccessDeniedFault(err.msg),
                    )
                }
                "KMSDisabledFault" => {
                    return RusotoError::Service(CreateEventSubscriptionError::KMSDisabledFault(
                        err.msg,
                    ))
                }
                "KMSInvalidStateFault" => {
                    return RusotoError::Service(
                        CreateEventSubscriptionError::KMSInvalidStateFault(err.msg),
                    )
                }
                "KMSNotFoundFault" => {
                    return RusotoError::Service(CreateEventSubscriptionError::KMSNotFoundFault(
                        err.msg,
                    ))
                }
                "KMSThrottlingFault" => {
                    return RusotoError::Service(CreateEventSubscriptionError::KMSThrottlingFault(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsFault" => {
                    return RusotoError::Service(
                        CreateEventSubscriptionError::ResourceAlreadyExistsFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        CreateEventSubscriptionError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ResourceQuotaExceededFault" => {
                    return RusotoError::Service(
                        CreateEventSubscriptionError::ResourceQuotaExceededFault(err.msg),
                    )
                }
                "SNSInvalidTopicFault" => {
                    return RusotoError::Service(
                        CreateEventSubscriptionError::SNSInvalidTopicFault(err.msg),
                    )
                }
                "SNSNoAuthorizationFault" => {
                    return RusotoError::Service(
                        CreateEventSubscriptionError::SNSNoAuthorizationFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateEventSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateEventSubscriptionError {
    fn description(&self) -> &str {
        match *self {
            CreateEventSubscriptionError::KMSAccessDeniedFault(ref cause) => cause,
            CreateEventSubscriptionError::KMSDisabledFault(ref cause) => cause,
            CreateEventSubscriptionError::KMSInvalidStateFault(ref cause) => cause,
            CreateEventSubscriptionError::KMSNotFoundFault(ref cause) => cause,
            CreateEventSubscriptionError::KMSThrottlingFault(ref cause) => cause,
            CreateEventSubscriptionError::ResourceAlreadyExistsFault(ref cause) => cause,
            CreateEventSubscriptionError::ResourceNotFoundFault(ref cause) => cause,
            CreateEventSubscriptionError::ResourceQuotaExceededFault(ref cause) => cause,
            CreateEventSubscriptionError::SNSInvalidTopicFault(ref cause) => cause,
            CreateEventSubscriptionError::SNSNoAuthorizationFault(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateReplicationInstance
#[derive(Debug, PartialEq)]
pub enum CreateReplicationInstanceError {
    /// <p>AWS DMS was denied access to the endpoint. Check that the role is correctly configured.</p>
    AccessDeniedFault(String),
    /// <p>There are not enough resources allocated to the database migration.</p>
    InsufficientResourceCapacityFault(String),
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The subnet provided is invalid.</p>
    InvalidSubnet(String),
    /// <p>AWS DMS cannot access the KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The replication subnet group does not cover enough Availability Zones (AZs). Edit the replication subnet group and add more AZs.</p>
    ReplicationSubnetGroupDoesNotCoverEnoughAZs(String),
    /// <p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// <p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
    /// <p>The storage quota has been exceeded.</p>
    StorageQuotaExceededFault(String),
}

impl CreateReplicationInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateReplicationInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedFault" => {
                    return RusotoError::Service(CreateReplicationInstanceError::AccessDeniedFault(
                        err.msg,
                    ))
                }
                "InsufficientResourceCapacityFault" => {
                    return RusotoError::Service(
                        CreateReplicationInstanceError::InsufficientResourceCapacityFault(err.msg),
                    )
                }
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        CreateReplicationInstanceError::InvalidResourceStateFault(err.msg),
                    )
                }
                "InvalidSubnet" => {
                    return RusotoError::Service(CreateReplicationInstanceError::InvalidSubnet(
                        err.msg,
                    ))
                }
                "KMSKeyNotAccessibleFault" => {
                    return RusotoError::Service(
                        CreateReplicationInstanceError::KMSKeyNotAccessibleFault(err.msg),
                    )
                }
                "ReplicationSubnetGroupDoesNotCoverEnoughAZs" => {
                    return RusotoError::Service(
                        CreateReplicationInstanceError::ReplicationSubnetGroupDoesNotCoverEnoughAZs(
                            err.msg,
                        ),
                    )
                }
                "ResourceAlreadyExistsFault" => {
                    return RusotoError::Service(
                        CreateReplicationInstanceError::ResourceAlreadyExistsFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        CreateReplicationInstanceError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ResourceQuotaExceededFault" => {
                    return RusotoError::Service(
                        CreateReplicationInstanceError::ResourceQuotaExceededFault(err.msg),
                    )
                }
                "StorageQuotaExceededFault" => {
                    return RusotoError::Service(
                        CreateReplicationInstanceError::StorageQuotaExceededFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateReplicationInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateReplicationInstanceError {
    fn description(&self) -> &str {
        match *self {
            CreateReplicationInstanceError::AccessDeniedFault(ref cause) => cause,
            CreateReplicationInstanceError::InsufficientResourceCapacityFault(ref cause) => cause,
            CreateReplicationInstanceError::InvalidResourceStateFault(ref cause) => cause,
            CreateReplicationInstanceError::InvalidSubnet(ref cause) => cause,
            CreateReplicationInstanceError::KMSKeyNotAccessibleFault(ref cause) => cause,
            CreateReplicationInstanceError::ReplicationSubnetGroupDoesNotCoverEnoughAZs(
                ref cause,
            ) => cause,
            CreateReplicationInstanceError::ResourceAlreadyExistsFault(ref cause) => cause,
            CreateReplicationInstanceError::ResourceNotFoundFault(ref cause) => cause,
            CreateReplicationInstanceError::ResourceQuotaExceededFault(ref cause) => cause,
            CreateReplicationInstanceError::StorageQuotaExceededFault(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateReplicationSubnetGroup
#[derive(Debug, PartialEq)]
pub enum CreateReplicationSubnetGroupError {
    /// <p>AWS DMS was denied access to the endpoint. Check that the role is correctly configured.</p>
    AccessDeniedFault(String),
    /// <p>The subnet provided is invalid.</p>
    InvalidSubnet(String),
    /// <p>The replication subnet group does not cover enough Availability Zones (AZs). Edit the replication subnet group and add more AZs.</p>
    ReplicationSubnetGroupDoesNotCoverEnoughAZs(String),
    /// <p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// <p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
}

impl CreateReplicationSubnetGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateReplicationSubnetGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedFault" => {
                    return RusotoError::Service(
                        CreateReplicationSubnetGroupError::AccessDeniedFault(err.msg),
                    )
                }
                "InvalidSubnet" => {
                    return RusotoError::Service(CreateReplicationSubnetGroupError::InvalidSubnet(
                        err.msg,
                    ))
                }
                "ReplicationSubnetGroupDoesNotCoverEnoughAZs" => return RusotoError::Service(
                    CreateReplicationSubnetGroupError::ReplicationSubnetGroupDoesNotCoverEnoughAZs(
                        err.msg,
                    ),
                ),
                "ResourceAlreadyExistsFault" => {
                    return RusotoError::Service(
                        CreateReplicationSubnetGroupError::ResourceAlreadyExistsFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        CreateReplicationSubnetGroupError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ResourceQuotaExceededFault" => {
                    return RusotoError::Service(
                        CreateReplicationSubnetGroupError::ResourceQuotaExceededFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateReplicationSubnetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateReplicationSubnetGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateReplicationSubnetGroupError::AccessDeniedFault(ref cause) => cause,
            CreateReplicationSubnetGroupError::InvalidSubnet(ref cause) => cause,
            CreateReplicationSubnetGroupError::ReplicationSubnetGroupDoesNotCoverEnoughAZs(
                ref cause,
            ) => cause,
            CreateReplicationSubnetGroupError::ResourceAlreadyExistsFault(ref cause) => cause,
            CreateReplicationSubnetGroupError::ResourceNotFoundFault(ref cause) => cause,
            CreateReplicationSubnetGroupError::ResourceQuotaExceededFault(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateReplicationTask
#[derive(Debug, PartialEq)]
pub enum CreateReplicationTaskError {
    /// <p>AWS DMS was denied access to the endpoint. Check that the role is correctly configured.</p>
    AccessDeniedFault(String),
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>AWS DMS cannot access the KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// <p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
}

impl CreateReplicationTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateReplicationTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedFault" => {
                    return RusotoError::Service(CreateReplicationTaskError::AccessDeniedFault(
                        err.msg,
                    ))
                }
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        CreateReplicationTaskError::InvalidResourceStateFault(err.msg),
                    )
                }
                "KMSKeyNotAccessibleFault" => {
                    return RusotoError::Service(
                        CreateReplicationTaskError::KMSKeyNotAccessibleFault(err.msg),
                    )
                }
                "ResourceAlreadyExistsFault" => {
                    return RusotoError::Service(
                        CreateReplicationTaskError::ResourceAlreadyExistsFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(CreateReplicationTaskError::ResourceNotFoundFault(
                        err.msg,
                    ))
                }
                "ResourceQuotaExceededFault" => {
                    return RusotoError::Service(
                        CreateReplicationTaskError::ResourceQuotaExceededFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateReplicationTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateReplicationTaskError {
    fn description(&self) -> &str {
        match *self {
            CreateReplicationTaskError::AccessDeniedFault(ref cause) => cause,
            CreateReplicationTaskError::InvalidResourceStateFault(ref cause) => cause,
            CreateReplicationTaskError::KMSKeyNotAccessibleFault(ref cause) => cause,
            CreateReplicationTaskError::ResourceAlreadyExistsFault(ref cause) => cause,
            CreateReplicationTaskError::ResourceNotFoundFault(ref cause) => cause,
            CreateReplicationTaskError::ResourceQuotaExceededFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCertificate
#[derive(Debug, PartialEq)]
pub enum DeleteCertificateError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DeleteCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(DeleteCertificateError::InvalidResourceStateFault(
                        err.msg,
                    ))
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(DeleteCertificateError::ResourceNotFoundFault(
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
impl fmt::Display for DeleteCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCertificateError {
    fn description(&self) -> &str {
        match *self {
            DeleteCertificateError::InvalidResourceStateFault(ref cause) => cause,
            DeleteCertificateError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteEndpoint
#[derive(Debug, PartialEq)]
pub enum DeleteEndpointError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DeleteEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(DeleteEndpointError::InvalidResourceStateFault(
                        err.msg,
                    ))
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(DeleteEndpointError::ResourceNotFoundFault(
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
impl fmt::Display for DeleteEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEndpointError {
    fn description(&self) -> &str {
        match *self {
            DeleteEndpointError::InvalidResourceStateFault(ref cause) => cause,
            DeleteEndpointError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteEventSubscription
#[derive(Debug, PartialEq)]
pub enum DeleteEventSubscriptionError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DeleteEventSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEventSubscriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        DeleteEventSubscriptionError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DeleteEventSubscriptionError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteEventSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEventSubscriptionError {
    fn description(&self) -> &str {
        match *self {
            DeleteEventSubscriptionError::InvalidResourceStateFault(ref cause) => cause,
            DeleteEventSubscriptionError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteReplicationInstance
#[derive(Debug, PartialEq)]
pub enum DeleteReplicationInstanceError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DeleteReplicationInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteReplicationInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        DeleteReplicationInstanceError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DeleteReplicationInstanceError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteReplicationInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteReplicationInstanceError {
    fn description(&self) -> &str {
        match *self {
            DeleteReplicationInstanceError::InvalidResourceStateFault(ref cause) => cause,
            DeleteReplicationInstanceError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteReplicationSubnetGroup
#[derive(Debug, PartialEq)]
pub enum DeleteReplicationSubnetGroupError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DeleteReplicationSubnetGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteReplicationSubnetGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        DeleteReplicationSubnetGroupError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DeleteReplicationSubnetGroupError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteReplicationSubnetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteReplicationSubnetGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteReplicationSubnetGroupError::InvalidResourceStateFault(ref cause) => cause,
            DeleteReplicationSubnetGroupError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteReplicationTask
#[derive(Debug, PartialEq)]
pub enum DeleteReplicationTaskError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DeleteReplicationTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteReplicationTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        DeleteReplicationTaskError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(DeleteReplicationTaskError::ResourceNotFoundFault(
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
impl fmt::Display for DeleteReplicationTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteReplicationTaskError {
    fn description(&self) -> &str {
        match *self {
            DeleteReplicationTaskError::InvalidResourceStateFault(ref cause) => cause,
            DeleteReplicationTaskError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAccountAttributes
#[derive(Debug, PartialEq)]
pub enum DescribeAccountAttributesError {}

impl DescribeAccountAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAccountAttributesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeAccountAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAccountAttributesError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DescribeCertificates
#[derive(Debug, PartialEq)]
pub enum DescribeCertificatesError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeCertificatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCertificatesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(DescribeCertificatesError::ResourceNotFoundFault(
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
impl fmt::Display for DescribeCertificatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCertificatesError {
    fn description(&self) -> &str {
        match *self {
            DescribeCertificatesError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConnections
#[derive(Debug, PartialEq)]
pub enum DescribeConnectionsError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeConnectionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeConnectionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(DescribeConnectionsError::ResourceNotFoundFault(
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
impl fmt::Display for DescribeConnectionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConnectionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeConnectionsError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEndpointTypes
#[derive(Debug, PartialEq)]
pub enum DescribeEndpointTypesError {}

impl DescribeEndpointTypesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEndpointTypesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeEndpointTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEndpointTypesError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DescribeEndpoints
#[derive(Debug, PartialEq)]
pub enum DescribeEndpointsError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeEndpointsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEndpointsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(DescribeEndpointsError::ResourceNotFoundFault(
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
impl fmt::Display for DescribeEndpointsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEndpointsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEndpointsError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEventCategories
#[derive(Debug, PartialEq)]
pub enum DescribeEventCategoriesError {}

impl DescribeEventCategoriesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEventCategoriesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeEventCategoriesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEventCategoriesError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DescribeEventSubscriptions
#[derive(Debug, PartialEq)]
pub enum DescribeEventSubscriptionsError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeEventSubscriptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeEventSubscriptionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeEventSubscriptionsError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeEventSubscriptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEventSubscriptionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEventSubscriptionsError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEvents
#[derive(Debug, PartialEq)]
pub enum DescribeEventsError {}

impl DescribeEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEventsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEventsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DescribeOrderableReplicationInstances
#[derive(Debug, PartialEq)]
pub enum DescribeOrderableReplicationInstancesError {}

impl DescribeOrderableReplicationInstancesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeOrderableReplicationInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeOrderableReplicationInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeOrderableReplicationInstancesError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by DescribePendingMaintenanceActions
#[derive(Debug, PartialEq)]
pub enum DescribePendingMaintenanceActionsError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribePendingMaintenanceActionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribePendingMaintenanceActionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DescribePendingMaintenanceActionsError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribePendingMaintenanceActionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePendingMaintenanceActionsError {
    fn description(&self) -> &str {
        match *self {
            DescribePendingMaintenanceActionsError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeRefreshSchemasStatus
#[derive(Debug, PartialEq)]
pub enum DescribeRefreshSchemasStatusError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeRefreshSchemasStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeRefreshSchemasStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        DescribeRefreshSchemasStatusError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeRefreshSchemasStatusError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeRefreshSchemasStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeRefreshSchemasStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribeRefreshSchemasStatusError::InvalidResourceStateFault(ref cause) => cause,
            DescribeRefreshSchemasStatusError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeReplicationInstanceTaskLogs
#[derive(Debug, PartialEq)]
pub enum DescribeReplicationInstanceTaskLogsError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeReplicationInstanceTaskLogsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeReplicationInstanceTaskLogsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        DescribeReplicationInstanceTaskLogsError::InvalidResourceStateFault(
                            err.msg,
                        ),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeReplicationInstanceTaskLogsError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeReplicationInstanceTaskLogsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeReplicationInstanceTaskLogsError {
    fn description(&self) -> &str {
        match *self {
            DescribeReplicationInstanceTaskLogsError::InvalidResourceStateFault(ref cause) => cause,
            DescribeReplicationInstanceTaskLogsError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeReplicationInstances
#[derive(Debug, PartialEq)]
pub enum DescribeReplicationInstancesError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeReplicationInstancesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeReplicationInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeReplicationInstancesError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeReplicationInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeReplicationInstancesError {
    fn description(&self) -> &str {
        match *self {
            DescribeReplicationInstancesError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeReplicationSubnetGroups
#[derive(Debug, PartialEq)]
pub enum DescribeReplicationSubnetGroupsError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeReplicationSubnetGroupsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeReplicationSubnetGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeReplicationSubnetGroupsError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeReplicationSubnetGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeReplicationSubnetGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeReplicationSubnetGroupsError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeReplicationTaskAssessmentResults
#[derive(Debug, PartialEq)]
pub enum DescribeReplicationTaskAssessmentResultsError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeReplicationTaskAssessmentResultsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeReplicationTaskAssessmentResultsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeReplicationTaskAssessmentResultsError::ResourceNotFoundFault(
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
impl fmt::Display for DescribeReplicationTaskAssessmentResultsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeReplicationTaskAssessmentResultsError {
    fn description(&self) -> &str {
        match *self {
            DescribeReplicationTaskAssessmentResultsError::ResourceNotFoundFault(ref cause) => {
                cause
            }
        }
    }
}
/// Errors returned by DescribeReplicationTasks
#[derive(Debug, PartialEq)]
pub enum DescribeReplicationTasksError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeReplicationTasksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeReplicationTasksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeReplicationTasksError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeReplicationTasksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeReplicationTasksError {
    fn description(&self) -> &str {
        match *self {
            DescribeReplicationTasksError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeSchemas
#[derive(Debug, PartialEq)]
pub enum DescribeSchemasError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeSchemasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSchemasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(DescribeSchemasError::InvalidResourceStateFault(
                        err.msg,
                    ))
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(DescribeSchemasError::ResourceNotFoundFault(
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
impl fmt::Display for DescribeSchemasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeSchemasError {
    fn description(&self) -> &str {
        match *self {
            DescribeSchemasError::InvalidResourceStateFault(ref cause) => cause,
            DescribeSchemasError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTableStatistics
#[derive(Debug, PartialEq)]
pub enum DescribeTableStatisticsError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeTableStatisticsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTableStatisticsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        DescribeTableStatisticsError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeTableStatisticsError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeTableStatisticsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTableStatisticsError {
    fn description(&self) -> &str {
        match *self {
            DescribeTableStatisticsError::InvalidResourceStateFault(ref cause) => cause,
            DescribeTableStatisticsError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by ImportCertificate
#[derive(Debug, PartialEq)]
pub enum ImportCertificateError {
    /// <p>The certificate was not valid.</p>
    InvalidCertificateFault(String),
    /// <p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    /// <p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
}

impl ImportCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidCertificateFault" => {
                    return RusotoError::Service(ImportCertificateError::InvalidCertificateFault(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsFault" => {
                    return RusotoError::Service(
                        ImportCertificateError::ResourceAlreadyExistsFault(err.msg),
                    )
                }
                "ResourceQuotaExceededFault" => {
                    return RusotoError::Service(
                        ImportCertificateError::ResourceQuotaExceededFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ImportCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ImportCertificateError {
    fn description(&self) -> &str {
        match *self {
            ImportCertificateError::InvalidCertificateFault(ref cause) => cause,
            ImportCertificateError::ResourceAlreadyExistsFault(ref cause) => cause,
            ImportCertificateError::ResourceQuotaExceededFault(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFoundFault(
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
            ListTagsForResourceError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyEndpoint
#[derive(Debug, PartialEq)]
pub enum ModifyEndpointError {
    /// <p>AWS DMS was denied access to the endpoint. Check that the role is correctly configured.</p>
    AccessDeniedFault(String),
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>AWS DMS cannot access the KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl ModifyEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedFault" => {
                    return RusotoError::Service(ModifyEndpointError::AccessDeniedFault(err.msg))
                }
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(ModifyEndpointError::InvalidResourceStateFault(
                        err.msg,
                    ))
                }
                "KMSKeyNotAccessibleFault" => {
                    return RusotoError::Service(ModifyEndpointError::KMSKeyNotAccessibleFault(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsFault" => {
                    return RusotoError::Service(ModifyEndpointError::ResourceAlreadyExistsFault(
                        err.msg,
                    ))
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(ModifyEndpointError::ResourceNotFoundFault(
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
impl fmt::Display for ModifyEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyEndpointError {
    fn description(&self) -> &str {
        match *self {
            ModifyEndpointError::AccessDeniedFault(ref cause) => cause,
            ModifyEndpointError::InvalidResourceStateFault(ref cause) => cause,
            ModifyEndpointError::KMSKeyNotAccessibleFault(ref cause) => cause,
            ModifyEndpointError::ResourceAlreadyExistsFault(ref cause) => cause,
            ModifyEndpointError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyEventSubscription
#[derive(Debug, PartialEq)]
pub enum ModifyEventSubscriptionError {
    /// <p>The ciphertext references a key that doesn't exist or DMS account doesn't have an access to</p>
    KMSAccessDeniedFault(String),
    /// <p>The specified master key (CMK) isn't enabled.</p>
    KMSDisabledFault(String),
    /// <p>The state of the specified KMS resource isn't valid for this request.</p>
    KMSInvalidStateFault(String),
    /// <p>The specified KMS entity or resource can't be found.</p>
    KMSNotFoundFault(String),
    /// <p>This request triggered KMS request throttling.</p>
    KMSThrottlingFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// <p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
    /// <p>The SNS topic is invalid.</p>
    SNSInvalidTopicFault(String),
    /// <p>You are not authorized for the SNS subscription.</p>
    SNSNoAuthorizationFault(String),
}

impl ModifyEventSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyEventSubscriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "KMSAccessDeniedFault" => {
                    return RusotoError::Service(
                        ModifyEventSubscriptionError::KMSAccessDeniedFault(err.msg),
                    )
                }
                "KMSDisabledFault" => {
                    return RusotoError::Service(ModifyEventSubscriptionError::KMSDisabledFault(
                        err.msg,
                    ))
                }
                "KMSInvalidStateFault" => {
                    return RusotoError::Service(
                        ModifyEventSubscriptionError::KMSInvalidStateFault(err.msg),
                    )
                }
                "KMSNotFoundFault" => {
                    return RusotoError::Service(ModifyEventSubscriptionError::KMSNotFoundFault(
                        err.msg,
                    ))
                }
                "KMSThrottlingFault" => {
                    return RusotoError::Service(ModifyEventSubscriptionError::KMSThrottlingFault(
                        err.msg,
                    ))
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        ModifyEventSubscriptionError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ResourceQuotaExceededFault" => {
                    return RusotoError::Service(
                        ModifyEventSubscriptionError::ResourceQuotaExceededFault(err.msg),
                    )
                }
                "SNSInvalidTopicFault" => {
                    return RusotoError::Service(
                        ModifyEventSubscriptionError::SNSInvalidTopicFault(err.msg),
                    )
                }
                "SNSNoAuthorizationFault" => {
                    return RusotoError::Service(
                        ModifyEventSubscriptionError::SNSNoAuthorizationFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ModifyEventSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyEventSubscriptionError {
    fn description(&self) -> &str {
        match *self {
            ModifyEventSubscriptionError::KMSAccessDeniedFault(ref cause) => cause,
            ModifyEventSubscriptionError::KMSDisabledFault(ref cause) => cause,
            ModifyEventSubscriptionError::KMSInvalidStateFault(ref cause) => cause,
            ModifyEventSubscriptionError::KMSNotFoundFault(ref cause) => cause,
            ModifyEventSubscriptionError::KMSThrottlingFault(ref cause) => cause,
            ModifyEventSubscriptionError::ResourceNotFoundFault(ref cause) => cause,
            ModifyEventSubscriptionError::ResourceQuotaExceededFault(ref cause) => cause,
            ModifyEventSubscriptionError::SNSInvalidTopicFault(ref cause) => cause,
            ModifyEventSubscriptionError::SNSNoAuthorizationFault(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyReplicationInstance
#[derive(Debug, PartialEq)]
pub enum ModifyReplicationInstanceError {
    /// <p>AWS DMS was denied access to the endpoint. Check that the role is correctly configured.</p>
    AccessDeniedFault(String),
    /// <p>There are not enough resources allocated to the database migration.</p>
    InsufficientResourceCapacityFault(String),
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// <p>The storage quota has been exceeded.</p>
    StorageQuotaExceededFault(String),
    /// <p>An upgrade dependency is preventing the database migration.</p>
    UpgradeDependencyFailureFault(String),
}

impl ModifyReplicationInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyReplicationInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedFault" => {
                    return RusotoError::Service(ModifyReplicationInstanceError::AccessDeniedFault(
                        err.msg,
                    ))
                }
                "InsufficientResourceCapacityFault" => {
                    return RusotoError::Service(
                        ModifyReplicationInstanceError::InsufficientResourceCapacityFault(err.msg),
                    )
                }
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        ModifyReplicationInstanceError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceAlreadyExistsFault" => {
                    return RusotoError::Service(
                        ModifyReplicationInstanceError::ResourceAlreadyExistsFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        ModifyReplicationInstanceError::ResourceNotFoundFault(err.msg),
                    )
                }
                "StorageQuotaExceededFault" => {
                    return RusotoError::Service(
                        ModifyReplicationInstanceError::StorageQuotaExceededFault(err.msg),
                    )
                }
                "UpgradeDependencyFailureFault" => {
                    return RusotoError::Service(
                        ModifyReplicationInstanceError::UpgradeDependencyFailureFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ModifyReplicationInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyReplicationInstanceError {
    fn description(&self) -> &str {
        match *self {
            ModifyReplicationInstanceError::AccessDeniedFault(ref cause) => cause,
            ModifyReplicationInstanceError::InsufficientResourceCapacityFault(ref cause) => cause,
            ModifyReplicationInstanceError::InvalidResourceStateFault(ref cause) => cause,
            ModifyReplicationInstanceError::ResourceAlreadyExistsFault(ref cause) => cause,
            ModifyReplicationInstanceError::ResourceNotFoundFault(ref cause) => cause,
            ModifyReplicationInstanceError::StorageQuotaExceededFault(ref cause) => cause,
            ModifyReplicationInstanceError::UpgradeDependencyFailureFault(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyReplicationSubnetGroup
#[derive(Debug, PartialEq)]
pub enum ModifyReplicationSubnetGroupError {
    /// <p>AWS DMS was denied access to the endpoint. Check that the role is correctly configured.</p>
    AccessDeniedFault(String),
    /// <p>The subnet provided is invalid.</p>
    InvalidSubnet(String),
    /// <p>The replication subnet group does not cover enough Availability Zones (AZs). Edit the replication subnet group and add more AZs.</p>
    ReplicationSubnetGroupDoesNotCoverEnoughAZs(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// <p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
    /// <p>The specified subnet is already in use.</p>
    SubnetAlreadyInUse(String),
}

impl ModifyReplicationSubnetGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ModifyReplicationSubnetGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedFault" => {
                    return RusotoError::Service(
                        ModifyReplicationSubnetGroupError::AccessDeniedFault(err.msg),
                    )
                }
                "InvalidSubnet" => {
                    return RusotoError::Service(ModifyReplicationSubnetGroupError::InvalidSubnet(
                        err.msg,
                    ))
                }
                "ReplicationSubnetGroupDoesNotCoverEnoughAZs" => return RusotoError::Service(
                    ModifyReplicationSubnetGroupError::ReplicationSubnetGroupDoesNotCoverEnoughAZs(
                        err.msg,
                    ),
                ),
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        ModifyReplicationSubnetGroupError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ResourceQuotaExceededFault" => {
                    return RusotoError::Service(
                        ModifyReplicationSubnetGroupError::ResourceQuotaExceededFault(err.msg),
                    )
                }
                "SubnetAlreadyInUse" => {
                    return RusotoError::Service(
                        ModifyReplicationSubnetGroupError::SubnetAlreadyInUse(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ModifyReplicationSubnetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyReplicationSubnetGroupError {
    fn description(&self) -> &str {
        match *self {
            ModifyReplicationSubnetGroupError::AccessDeniedFault(ref cause) => cause,
            ModifyReplicationSubnetGroupError::InvalidSubnet(ref cause) => cause,
            ModifyReplicationSubnetGroupError::ReplicationSubnetGroupDoesNotCoverEnoughAZs(
                ref cause,
            ) => cause,
            ModifyReplicationSubnetGroupError::ResourceNotFoundFault(ref cause) => cause,
            ModifyReplicationSubnetGroupError::ResourceQuotaExceededFault(ref cause) => cause,
            ModifyReplicationSubnetGroupError::SubnetAlreadyInUse(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyReplicationTask
#[derive(Debug, PartialEq)]
pub enum ModifyReplicationTaskError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>AWS DMS cannot access the KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl ModifyReplicationTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyReplicationTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        ModifyReplicationTaskError::InvalidResourceStateFault(err.msg),
                    )
                }
                "KMSKeyNotAccessibleFault" => {
                    return RusotoError::Service(
                        ModifyReplicationTaskError::KMSKeyNotAccessibleFault(err.msg),
                    )
                }
                "ResourceAlreadyExistsFault" => {
                    return RusotoError::Service(
                        ModifyReplicationTaskError::ResourceAlreadyExistsFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(ModifyReplicationTaskError::ResourceNotFoundFault(
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
impl fmt::Display for ModifyReplicationTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyReplicationTaskError {
    fn description(&self) -> &str {
        match *self {
            ModifyReplicationTaskError::InvalidResourceStateFault(ref cause) => cause,
            ModifyReplicationTaskError::KMSKeyNotAccessibleFault(ref cause) => cause,
            ModifyReplicationTaskError::ResourceAlreadyExistsFault(ref cause) => cause,
            ModifyReplicationTaskError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by RebootReplicationInstance
#[derive(Debug, PartialEq)]
pub enum RebootReplicationInstanceError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl RebootReplicationInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RebootReplicationInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        RebootReplicationInstanceError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        RebootReplicationInstanceError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RebootReplicationInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RebootReplicationInstanceError {
    fn description(&self) -> &str {
        match *self {
            RebootReplicationInstanceError::InvalidResourceStateFault(ref cause) => cause,
            RebootReplicationInstanceError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by RefreshSchemas
#[derive(Debug, PartialEq)]
pub enum RefreshSchemasError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>AWS DMS cannot access the KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// <p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
}

impl RefreshSchemasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RefreshSchemasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(RefreshSchemasError::InvalidResourceStateFault(
                        err.msg,
                    ))
                }
                "KMSKeyNotAccessibleFault" => {
                    return RusotoError::Service(RefreshSchemasError::KMSKeyNotAccessibleFault(
                        err.msg,
                    ))
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(RefreshSchemasError::ResourceNotFoundFault(
                        err.msg,
                    ))
                }
                "ResourceQuotaExceededFault" => {
                    return RusotoError::Service(RefreshSchemasError::ResourceQuotaExceededFault(
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
impl fmt::Display for RefreshSchemasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RefreshSchemasError {
    fn description(&self) -> &str {
        match *self {
            RefreshSchemasError::InvalidResourceStateFault(ref cause) => cause,
            RefreshSchemasError::KMSKeyNotAccessibleFault(ref cause) => cause,
            RefreshSchemasError::ResourceNotFoundFault(ref cause) => cause,
            RefreshSchemasError::ResourceQuotaExceededFault(ref cause) => cause,
        }
    }
}
/// Errors returned by ReloadTables
#[derive(Debug, PartialEq)]
pub enum ReloadTablesError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl ReloadTablesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ReloadTablesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(ReloadTablesError::InvalidResourceStateFault(
                        err.msg,
                    ))
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(ReloadTablesError::ResourceNotFoundFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ReloadTablesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ReloadTablesError {
    fn description(&self) -> &str {
        match *self {
            ReloadTablesError::InvalidResourceStateFault(ref cause) => cause,
            ReloadTablesError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveTagsFromResource
#[derive(Debug, PartialEq)]
pub enum RemoveTagsFromResourceError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl RemoveTagsFromResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveTagsFromResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        RemoveTagsFromResourceError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RemoveTagsFromResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveTagsFromResourceError {
    fn description(&self) -> &str {
        match *self {
            RemoveTagsFromResourceError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by StartReplicationTask
#[derive(Debug, PartialEq)]
pub enum StartReplicationTaskError {
    /// <p>AWS DMS was denied access to the endpoint. Check that the role is correctly configured.</p>
    AccessDeniedFault(String),
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl StartReplicationTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartReplicationTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedFault" => {
                    return RusotoError::Service(StartReplicationTaskError::AccessDeniedFault(
                        err.msg,
                    ))
                }
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        StartReplicationTaskError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(StartReplicationTaskError::ResourceNotFoundFault(
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
impl fmt::Display for StartReplicationTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartReplicationTaskError {
    fn description(&self) -> &str {
        match *self {
            StartReplicationTaskError::AccessDeniedFault(ref cause) => cause,
            StartReplicationTaskError::InvalidResourceStateFault(ref cause) => cause,
            StartReplicationTaskError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by StartReplicationTaskAssessment
#[derive(Debug, PartialEq)]
pub enum StartReplicationTaskAssessmentError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl StartReplicationTaskAssessmentError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartReplicationTaskAssessmentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        StartReplicationTaskAssessmentError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        StartReplicationTaskAssessmentError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartReplicationTaskAssessmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartReplicationTaskAssessmentError {
    fn description(&self) -> &str {
        match *self {
            StartReplicationTaskAssessmentError::InvalidResourceStateFault(ref cause) => cause,
            StartReplicationTaskAssessmentError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by StopReplicationTask
#[derive(Debug, PartialEq)]
pub enum StopReplicationTaskError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl StopReplicationTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopReplicationTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        StopReplicationTaskError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(StopReplicationTaskError::ResourceNotFoundFault(
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
impl fmt::Display for StopReplicationTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopReplicationTaskError {
    fn description(&self) -> &str {
        match *self {
            StopReplicationTaskError::InvalidResourceStateFault(ref cause) => cause,
            StopReplicationTaskError::ResourceNotFoundFault(ref cause) => cause,
        }
    }
}
/// Errors returned by TestConnection
#[derive(Debug, PartialEq)]
pub enum TestConnectionError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>AWS DMS cannot access the KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// <p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
}

impl TestConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TestConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(TestConnectionError::InvalidResourceStateFault(
                        err.msg,
                    ))
                }
                "KMSKeyNotAccessibleFault" => {
                    return RusotoError::Service(TestConnectionError::KMSKeyNotAccessibleFault(
                        err.msg,
                    ))
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(TestConnectionError::ResourceNotFoundFault(
                        err.msg,
                    ))
                }
                "ResourceQuotaExceededFault" => {
                    return RusotoError::Service(TestConnectionError::ResourceQuotaExceededFault(
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
impl fmt::Display for TestConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TestConnectionError {
    fn description(&self) -> &str {
        match *self {
            TestConnectionError::InvalidResourceStateFault(ref cause) => cause,
            TestConnectionError::KMSKeyNotAccessibleFault(ref cause) => cause,
            TestConnectionError::ResourceNotFoundFault(ref cause) => cause,
            TestConnectionError::ResourceQuotaExceededFault(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Database Migration Service API. AWS Database Migration Service clients implement this trait.
pub trait DatabaseMigrationService {
    /// <p>Adds metadata tags to an AWS DMS resource, including replication instance, endpoint, security group, and migration task. These tags can also be used with cost allocation reporting to track cost associated with DMS resources, or used in a Condition statement in an IAM policy for DMS.</p>
    fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceMessage,
    ) -> RusotoFuture<AddTagsToResourceResponse, AddTagsToResourceError>;

    /// <p>Applies a pending maintenance action to a resource (for example, to a replication instance).</p>
    fn apply_pending_maintenance_action(
        &self,
        input: ApplyPendingMaintenanceActionMessage,
    ) -> RusotoFuture<ApplyPendingMaintenanceActionResponse, ApplyPendingMaintenanceActionError>;

    /// <p>Creates an endpoint using the provided settings.</p>
    fn create_endpoint(
        &self,
        input: CreateEndpointMessage,
    ) -> RusotoFuture<CreateEndpointResponse, CreateEndpointError>;

    /// <p> Creates an AWS DMS event notification subscription. </p> <p>You can specify the type of source (<code>SourceType</code>) you want to be notified of, provide a list of AWS DMS source IDs (<code>SourceIds</code>) that triggers the events, and provide a list of event categories (<code>EventCategories</code>) for events you want to be notified of. If you specify both the <code>SourceType</code> and <code>SourceIds</code>, such as <code>SourceType = replication-instance</code> and <code>SourceIdentifier = my-replinstance</code>, you will be notified of all the replication instance events for the specified source. If you specify a <code>SourceType</code> but don't specify a <code>SourceIdentifier</code>, you receive notice of the events for that source type for all your AWS DMS sources. If you don't specify either <code>SourceType</code> nor <code>SourceIdentifier</code>, you will be notified of events generated from all AWS DMS sources belonging to your customer account.</p> <p>For more information about AWS DMS events, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html">Working with Events and Notifications</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    fn create_event_subscription(
        &self,
        input: CreateEventSubscriptionMessage,
    ) -> RusotoFuture<CreateEventSubscriptionResponse, CreateEventSubscriptionError>;

    /// <p>Creates the replication instance using the specified parameters.</p>
    fn create_replication_instance(
        &self,
        input: CreateReplicationInstanceMessage,
    ) -> RusotoFuture<CreateReplicationInstanceResponse, CreateReplicationInstanceError>;

    /// <p>Creates a replication subnet group given a list of the subnet IDs in a VPC.</p>
    fn create_replication_subnet_group(
        &self,
        input: CreateReplicationSubnetGroupMessage,
    ) -> RusotoFuture<CreateReplicationSubnetGroupResponse, CreateReplicationSubnetGroupError>;

    /// <p>Creates a replication task using the specified parameters.</p>
    fn create_replication_task(
        &self,
        input: CreateReplicationTaskMessage,
    ) -> RusotoFuture<CreateReplicationTaskResponse, CreateReplicationTaskError>;

    /// <p>Deletes the specified certificate. </p>
    fn delete_certificate(
        &self,
        input: DeleteCertificateMessage,
    ) -> RusotoFuture<DeleteCertificateResponse, DeleteCertificateError>;

    /// <p><p>Deletes the specified endpoint.</p> <note> <p>All tasks associated with the endpoint must be deleted before you can delete the endpoint.</p> </note> <p/></p>
    fn delete_endpoint(
        &self,
        input: DeleteEndpointMessage,
    ) -> RusotoFuture<DeleteEndpointResponse, DeleteEndpointError>;

    /// <p> Deletes an AWS DMS event subscription. </p>
    fn delete_event_subscription(
        &self,
        input: DeleteEventSubscriptionMessage,
    ) -> RusotoFuture<DeleteEventSubscriptionResponse, DeleteEventSubscriptionError>;

    /// <p><p>Deletes the specified replication instance.</p> <note> <p>You must delete any migration tasks that are associated with the replication instance before you can delete it.</p> </note> <p/></p>
    fn delete_replication_instance(
        &self,
        input: DeleteReplicationInstanceMessage,
    ) -> RusotoFuture<DeleteReplicationInstanceResponse, DeleteReplicationInstanceError>;

    /// <p>Deletes a subnet group.</p>
    fn delete_replication_subnet_group(
        &self,
        input: DeleteReplicationSubnetGroupMessage,
    ) -> RusotoFuture<DeleteReplicationSubnetGroupResponse, DeleteReplicationSubnetGroupError>;

    /// <p>Deletes the specified replication task.</p>
    fn delete_replication_task(
        &self,
        input: DeleteReplicationTaskMessage,
    ) -> RusotoFuture<DeleteReplicationTaskResponse, DeleteReplicationTaskError>;

    /// <p>Lists all of the AWS DMS attributes for a customer account. The attributes include AWS DMS quotas for the account, such as the number of replication instances allowed. The description for a quota includes the quota name, current usage toward that quota, and the quota's maximum value.</p> <p>This command does not take any parameters.</p>
    fn describe_account_attributes(
        &self,
    ) -> RusotoFuture<DescribeAccountAttributesResponse, DescribeAccountAttributesError>;

    /// <p>Provides a description of the certificate.</p>
    fn describe_certificates(
        &self,
        input: DescribeCertificatesMessage,
    ) -> RusotoFuture<DescribeCertificatesResponse, DescribeCertificatesError>;

    /// <p>Describes the status of the connections that have been made between the replication instance and an endpoint. Connections are created when you test an endpoint.</p>
    fn describe_connections(
        &self,
        input: DescribeConnectionsMessage,
    ) -> RusotoFuture<DescribeConnectionsResponse, DescribeConnectionsError>;

    /// <p>Returns information about the type of endpoints available.</p>
    fn describe_endpoint_types(
        &self,
        input: DescribeEndpointTypesMessage,
    ) -> RusotoFuture<DescribeEndpointTypesResponse, DescribeEndpointTypesError>;

    /// <p>Returns information about the endpoints for your account in the current region.</p>
    fn describe_endpoints(
        &self,
        input: DescribeEndpointsMessage,
    ) -> RusotoFuture<DescribeEndpointsResponse, DescribeEndpointsError>;

    /// <p>Lists categories for all event source types, or, if specified, for a specified source type. You can see a list of the event categories and source types in <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html">Working with Events and Notifications</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    fn describe_event_categories(
        &self,
        input: DescribeEventCategoriesMessage,
    ) -> RusotoFuture<DescribeEventCategoriesResponse, DescribeEventCategoriesError>;

    /// <p>Lists all the event subscriptions for a customer account. The description of a subscription includes <code>SubscriptionName</code>, <code>SNSTopicARN</code>, <code>CustomerID</code>, <code>SourceType</code>, <code>SourceID</code>, <code>CreationTime</code>, and <code>Status</code>. </p> <p>If you specify <code>SubscriptionName</code>, this action lists the description for that subscription.</p>
    fn describe_event_subscriptions(
        &self,
        input: DescribeEventSubscriptionsMessage,
    ) -> RusotoFuture<DescribeEventSubscriptionsResponse, DescribeEventSubscriptionsError>;

    /// <p> Lists events for a given source identifier and source type. You can also specify a start and end time. For more information on AWS DMS events, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html">Working with Events and Notifications</a> in the <i>AWS Database Migration User Guide.</i> </p>
    fn describe_events(
        &self,
        input: DescribeEventsMessage,
    ) -> RusotoFuture<DescribeEventsResponse, DescribeEventsError>;

    /// <p>Returns information about the replication instance types that can be created in the specified region.</p>
    fn describe_orderable_replication_instances(
        &self,
        input: DescribeOrderableReplicationInstancesMessage,
    ) -> RusotoFuture<
        DescribeOrderableReplicationInstancesResponse,
        DescribeOrderableReplicationInstancesError,
    >;

    /// <p>For internal use only</p>
    fn describe_pending_maintenance_actions(
        &self,
        input: DescribePendingMaintenanceActionsMessage,
    ) -> RusotoFuture<
        DescribePendingMaintenanceActionsResponse,
        DescribePendingMaintenanceActionsError,
    >;

    /// <p>Returns the status of the RefreshSchemas operation.</p>
    fn describe_refresh_schemas_status(
        &self,
        input: DescribeRefreshSchemasStatusMessage,
    ) -> RusotoFuture<DescribeRefreshSchemasStatusResponse, DescribeRefreshSchemasStatusError>;

    /// <p>Returns information about the task logs for the specified task.</p>
    fn describe_replication_instance_task_logs(
        &self,
        input: DescribeReplicationInstanceTaskLogsMessage,
    ) -> RusotoFuture<
        DescribeReplicationInstanceTaskLogsResponse,
        DescribeReplicationInstanceTaskLogsError,
    >;

    /// <p>Returns information about replication instances for your account in the current region.</p>
    fn describe_replication_instances(
        &self,
        input: DescribeReplicationInstancesMessage,
    ) -> RusotoFuture<DescribeReplicationInstancesResponse, DescribeReplicationInstancesError>;

    /// <p>Returns information about the replication subnet groups.</p>
    fn describe_replication_subnet_groups(
        &self,
        input: DescribeReplicationSubnetGroupsMessage,
    ) -> RusotoFuture<DescribeReplicationSubnetGroupsResponse, DescribeReplicationSubnetGroupsError>;

    /// <p>Returns the task assessment results from Amazon S3. This action always returns the latest results.</p>
    fn describe_replication_task_assessment_results(
        &self,
        input: DescribeReplicationTaskAssessmentResultsMessage,
    ) -> RusotoFuture<
        DescribeReplicationTaskAssessmentResultsResponse,
        DescribeReplicationTaskAssessmentResultsError,
    >;

    /// <p>Returns information about replication tasks for your account in the current region.</p>
    fn describe_replication_tasks(
        &self,
        input: DescribeReplicationTasksMessage,
    ) -> RusotoFuture<DescribeReplicationTasksResponse, DescribeReplicationTasksError>;

    /// <p><p>Returns information about the schema for the specified endpoint.</p> <p/></p>
    fn describe_schemas(
        &self,
        input: DescribeSchemasMessage,
    ) -> RusotoFuture<DescribeSchemasResponse, DescribeSchemasError>;

    /// <p>Returns table statistics on the database migration task, including table name, rows inserted, rows updated, and rows deleted.</p> <p>Note that the "last updated" column the DMS console only indicates the time that AWS DMS last updated the table statistics record for a table. It does not indicate the time of the last update to the table.</p>
    fn describe_table_statistics(
        &self,
        input: DescribeTableStatisticsMessage,
    ) -> RusotoFuture<DescribeTableStatisticsResponse, DescribeTableStatisticsError>;

    /// <p>Uploads the specified certificate.</p>
    fn import_certificate(
        &self,
        input: ImportCertificateMessage,
    ) -> RusotoFuture<ImportCertificateResponse, ImportCertificateError>;

    /// <p>Lists all tags for an AWS DMS resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceMessage,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError>;

    /// <p>Modifies the specified endpoint.</p>
    fn modify_endpoint(
        &self,
        input: ModifyEndpointMessage,
    ) -> RusotoFuture<ModifyEndpointResponse, ModifyEndpointError>;

    /// <p>Modifies an existing AWS DMS event notification subscription. </p>
    fn modify_event_subscription(
        &self,
        input: ModifyEventSubscriptionMessage,
    ) -> RusotoFuture<ModifyEventSubscriptionResponse, ModifyEventSubscriptionError>;

    /// <p><p>Modifies the replication instance to apply new settings. You can change one or more parameters by specifying these parameters and the new values in the request.</p> <p>Some settings are applied during the maintenance window.</p> <p/></p>
    fn modify_replication_instance(
        &self,
        input: ModifyReplicationInstanceMessage,
    ) -> RusotoFuture<ModifyReplicationInstanceResponse, ModifyReplicationInstanceError>;

    /// <p>Modifies the settings for the specified replication subnet group.</p>
    fn modify_replication_subnet_group(
        &self,
        input: ModifyReplicationSubnetGroupMessage,
    ) -> RusotoFuture<ModifyReplicationSubnetGroupResponse, ModifyReplicationSubnetGroupError>;

    /// <p>Modifies the specified replication task.</p> <p>You can't modify the task endpoints. The task must be stopped before you can modify it. </p> <p>For more information about AWS DMS tasks, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.html">Working with Migration Tasks</a> in the <i>AWS Database Migration Service User Guide</i>.</p>
    fn modify_replication_task(
        &self,
        input: ModifyReplicationTaskMessage,
    ) -> RusotoFuture<ModifyReplicationTaskResponse, ModifyReplicationTaskError>;

    /// <p>Reboots a replication instance. Rebooting results in a momentary outage, until the replication instance becomes available again.</p>
    fn reboot_replication_instance(
        &self,
        input: RebootReplicationInstanceMessage,
    ) -> RusotoFuture<RebootReplicationInstanceResponse, RebootReplicationInstanceError>;

    /// <p>Populates the schema for the specified endpoint. This is an asynchronous operation and can take several minutes. You can check the status of this operation by calling the DescribeRefreshSchemasStatus operation.</p>
    fn refresh_schemas(
        &self,
        input: RefreshSchemasMessage,
    ) -> RusotoFuture<RefreshSchemasResponse, RefreshSchemasError>;

    /// <p>Reloads the target database table with the source data. </p>
    fn reload_tables(
        &self,
        input: ReloadTablesMessage,
    ) -> RusotoFuture<ReloadTablesResponse, ReloadTablesError>;

    /// <p>Removes metadata tags from a DMS resource.</p>
    fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceMessage,
    ) -> RusotoFuture<RemoveTagsFromResourceResponse, RemoveTagsFromResourceError>;

    /// <p>Starts the replication task.</p> <p>For more information about AWS DMS tasks, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.html">Working with Migration Tasks </a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    fn start_replication_task(
        &self,
        input: StartReplicationTaskMessage,
    ) -> RusotoFuture<StartReplicationTaskResponse, StartReplicationTaskError>;

    /// <p> Starts the replication task assessment for unsupported data types in the source database. </p>
    fn start_replication_task_assessment(
        &self,
        input: StartReplicationTaskAssessmentMessage,
    ) -> RusotoFuture<StartReplicationTaskAssessmentResponse, StartReplicationTaskAssessmentError>;

    /// <p><p>Stops the replication task.</p> <p/></p>
    fn stop_replication_task(
        &self,
        input: StopReplicationTaskMessage,
    ) -> RusotoFuture<StopReplicationTaskResponse, StopReplicationTaskError>;

    /// <p>Tests the connection between the replication instance and the endpoint.</p>
    fn test_connection(
        &self,
        input: TestConnectionMessage,
    ) -> RusotoFuture<TestConnectionResponse, TestConnectionError>;
}
/// A client for the AWS Database Migration Service API.
#[derive(Clone)]
pub struct DatabaseMigrationServiceClient {
    client: Client,
    region: region::Region,
}

impl DatabaseMigrationServiceClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> DatabaseMigrationServiceClient {
        DatabaseMigrationServiceClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> DatabaseMigrationServiceClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        DatabaseMigrationServiceClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl DatabaseMigrationService for DatabaseMigrationServiceClient {
    /// <p>Adds metadata tags to an AWS DMS resource, including replication instance, endpoint, security group, and migration task. These tags can also be used with cost allocation reporting to track cost associated with DMS resources, or used in a Condition statement in an IAM policy for DMS.</p>
    fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceMessage,
    ) -> RusotoFuture<AddTagsToResourceResponse, AddTagsToResourceError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.AddTagsToResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AddTagsToResourceResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AddTagsToResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Applies a pending maintenance action to a resource (for example, to a replication instance).</p>
    fn apply_pending_maintenance_action(
        &self,
        input: ApplyPendingMaintenanceActionMessage,
    ) -> RusotoFuture<ApplyPendingMaintenanceActionResponse, ApplyPendingMaintenanceActionError>
    {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.ApplyPendingMaintenanceAction",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ApplyPendingMaintenanceActionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ApplyPendingMaintenanceActionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates an endpoint using the provided settings.</p>
    fn create_endpoint(
        &self,
        input: CreateEndpointMessage,
    ) -> RusotoFuture<CreateEndpointResponse, CreateEndpointError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.CreateEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateEndpointResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateEndpointError::from_response(response))),
                )
            }
        })
    }

    /// <p> Creates an AWS DMS event notification subscription. </p> <p>You can specify the type of source (<code>SourceType</code>) you want to be notified of, provide a list of AWS DMS source IDs (<code>SourceIds</code>) that triggers the events, and provide a list of event categories (<code>EventCategories</code>) for events you want to be notified of. If you specify both the <code>SourceType</code> and <code>SourceIds</code>, such as <code>SourceType = replication-instance</code> and <code>SourceIdentifier = my-replinstance</code>, you will be notified of all the replication instance events for the specified source. If you specify a <code>SourceType</code> but don't specify a <code>SourceIdentifier</code>, you receive notice of the events for that source type for all your AWS DMS sources. If you don't specify either <code>SourceType</code> nor <code>SourceIdentifier</code>, you will be notified of events generated from all AWS DMS sources belonging to your customer account.</p> <p>For more information about AWS DMS events, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html">Working with Events and Notifications</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    fn create_event_subscription(
        &self,
        input: CreateEventSubscriptionMessage,
    ) -> RusotoFuture<CreateEventSubscriptionResponse, CreateEventSubscriptionError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.CreateEventSubscription");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateEventSubscriptionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateEventSubscriptionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates the replication instance using the specified parameters.</p>
    fn create_replication_instance(
        &self,
        input: CreateReplicationInstanceMessage,
    ) -> RusotoFuture<CreateReplicationInstanceResponse, CreateReplicationInstanceError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.CreateReplicationInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateReplicationInstanceResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateReplicationInstanceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a replication subnet group given a list of the subnet IDs in a VPC.</p>
    fn create_replication_subnet_group(
        &self,
        input: CreateReplicationSubnetGroupMessage,
    ) -> RusotoFuture<CreateReplicationSubnetGroupResponse, CreateReplicationSubnetGroupError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.CreateReplicationSubnetGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateReplicationSubnetGroupResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateReplicationSubnetGroupError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a replication task using the specified parameters.</p>
    fn create_replication_task(
        &self,
        input: CreateReplicationTaskMessage,
    ) -> RusotoFuture<CreateReplicationTaskResponse, CreateReplicationTaskError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.CreateReplicationTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateReplicationTaskResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateReplicationTaskError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes the specified certificate. </p>
    fn delete_certificate(
        &self,
        input: DeleteCertificateMessage,
    ) -> RusotoFuture<DeleteCertificateResponse, DeleteCertificateError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DeleteCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteCertificateResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteCertificateError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Deletes the specified endpoint.</p> <note> <p>All tasks associated with the endpoint must be deleted before you can delete the endpoint.</p> </note> <p/></p>
    fn delete_endpoint(
        &self,
        input: DeleteEndpointMessage,
    ) -> RusotoFuture<DeleteEndpointResponse, DeleteEndpointError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DeleteEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteEndpointResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteEndpointError::from_response(response))),
                )
            }
        })
    }

    /// <p> Deletes an AWS DMS event subscription. </p>
    fn delete_event_subscription(
        &self,
        input: DeleteEventSubscriptionMessage,
    ) -> RusotoFuture<DeleteEventSubscriptionResponse, DeleteEventSubscriptionError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DeleteEventSubscription");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteEventSubscriptionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteEventSubscriptionError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Deletes the specified replication instance.</p> <note> <p>You must delete any migration tasks that are associated with the replication instance before you can delete it.</p> </note> <p/></p>
    fn delete_replication_instance(
        &self,
        input: DeleteReplicationInstanceMessage,
    ) -> RusotoFuture<DeleteReplicationInstanceResponse, DeleteReplicationInstanceError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DeleteReplicationInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteReplicationInstanceResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteReplicationInstanceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes a subnet group.</p>
    fn delete_replication_subnet_group(
        &self,
        input: DeleteReplicationSubnetGroupMessage,
    ) -> RusotoFuture<DeleteReplicationSubnetGroupResponse, DeleteReplicationSubnetGroupError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DeleteReplicationSubnetGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteReplicationSubnetGroupResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteReplicationSubnetGroupError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes the specified replication task.</p>
    fn delete_replication_task(
        &self,
        input: DeleteReplicationTaskMessage,
    ) -> RusotoFuture<DeleteReplicationTaskResponse, DeleteReplicationTaskError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DeleteReplicationTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteReplicationTaskResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteReplicationTaskError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists all of the AWS DMS attributes for a customer account. The attributes include AWS DMS quotas for the account, such as the number of replication instances allowed. The description for a quota includes the quota name, current usage toward that quota, and the quota's maximum value.</p> <p>This command does not take any parameters.</p>
    fn describe_account_attributes(
        &self,
    ) -> RusotoFuture<DescribeAccountAttributesResponse, DescribeAccountAttributesError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribeAccountAttributes",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeAccountAttributesResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAccountAttributesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Provides a description of the certificate.</p>
    fn describe_certificates(
        &self,
        input: DescribeCertificatesMessage,
    ) -> RusotoFuture<DescribeCertificatesResponse, DescribeCertificatesError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeCertificates");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeCertificatesResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeCertificatesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Describes the status of the connections that have been made between the replication instance and an endpoint. Connections are created when you test an endpoint.</p>
    fn describe_connections(
        &self,
        input: DescribeConnectionsMessage,
    ) -> RusotoFuture<DescribeConnectionsResponse, DescribeConnectionsError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeConnections");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeConnectionsResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeConnectionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns information about the type of endpoints available.</p>
    fn describe_endpoint_types(
        &self,
        input: DescribeEndpointTypesMessage,
    ) -> RusotoFuture<DescribeEndpointTypesResponse, DescribeEndpointTypesError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeEndpointTypes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeEndpointTypesResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeEndpointTypesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns information about the endpoints for your account in the current region.</p>
    fn describe_endpoints(
        &self,
        input: DescribeEndpointsMessage,
    ) -> RusotoFuture<DescribeEndpointsResponse, DescribeEndpointsError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeEndpoints");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeEndpointsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeEndpointsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists categories for all event source types, or, if specified, for a specified source type. You can see a list of the event categories and source types in <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html">Working with Events and Notifications</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    fn describe_event_categories(
        &self,
        input: DescribeEventCategoriesMessage,
    ) -> RusotoFuture<DescribeEventCategoriesResponse, DescribeEventCategoriesError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeEventCategories");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeEventCategoriesResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEventCategoriesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists all the event subscriptions for a customer account. The description of a subscription includes <code>SubscriptionName</code>, <code>SNSTopicARN</code>, <code>CustomerID</code>, <code>SourceType</code>, <code>SourceID</code>, <code>CreationTime</code>, and <code>Status</code>. </p> <p>If you specify <code>SubscriptionName</code>, this action lists the description for that subscription.</p>
    fn describe_event_subscriptions(
        &self,
        input: DescribeEventSubscriptionsMessage,
    ) -> RusotoFuture<DescribeEventSubscriptionsResponse, DescribeEventSubscriptionsError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribeEventSubscriptions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeEventSubscriptionsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeEventSubscriptionsError::from_response(response))
                }))
            }
        })
    }

    /// <p> Lists events for a given source identifier and source type. You can also specify a start and end time. For more information on AWS DMS events, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html">Working with Events and Notifications</a> in the <i>AWS Database Migration User Guide.</i> </p>
    fn describe_events(
        &self,
        input: DescribeEventsMessage,
    ) -> RusotoFuture<DescribeEventsResponse, DescribeEventsError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeEvents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeEventsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeEventsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns information about the replication instance types that can be created in the specified region.</p>
    fn describe_orderable_replication_instances(
        &self,
        input: DescribeOrderableReplicationInstancesMessage,
    ) -> RusotoFuture<
        DescribeOrderableReplicationInstancesResponse,
        DescribeOrderableReplicationInstancesError,
    > {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribeOrderableReplicationInstances",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeOrderableReplicationInstancesResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeOrderableReplicationInstancesError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>For internal use only</p>
    fn describe_pending_maintenance_actions(
        &self,
        input: DescribePendingMaintenanceActionsMessage,
    ) -> RusotoFuture<
        DescribePendingMaintenanceActionsResponse,
        DescribePendingMaintenanceActionsError,
    > {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribePendingMaintenanceActions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribePendingMaintenanceActionsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribePendingMaintenanceActionsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns the status of the RefreshSchemas operation.</p>
    fn describe_refresh_schemas_status(
        &self,
        input: DescribeRefreshSchemasStatusMessage,
    ) -> RusotoFuture<DescribeRefreshSchemasStatusResponse, DescribeRefreshSchemasStatusError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribeRefreshSchemasStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeRefreshSchemasStatusResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeRefreshSchemasStatusError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns information about the task logs for the specified task.</p>
    fn describe_replication_instance_task_logs(
        &self,
        input: DescribeReplicationInstanceTaskLogsMessage,
    ) -> RusotoFuture<
        DescribeReplicationInstanceTaskLogsResponse,
        DescribeReplicationInstanceTaskLogsError,
    > {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribeReplicationInstanceTaskLogs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeReplicationInstanceTaskLogsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeReplicationInstanceTaskLogsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns information about replication instances for your account in the current region.</p>
    fn describe_replication_instances(
        &self,
        input: DescribeReplicationInstancesMessage,
    ) -> RusotoFuture<DescribeReplicationInstancesResponse, DescribeReplicationInstancesError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribeReplicationInstances",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeReplicationInstancesResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeReplicationInstancesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns information about the replication subnet groups.</p>
    fn describe_replication_subnet_groups(
        &self,
        input: DescribeReplicationSubnetGroupsMessage,
    ) -> RusotoFuture<DescribeReplicationSubnetGroupsResponse, DescribeReplicationSubnetGroupsError>
    {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribeReplicationSubnetGroups",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeReplicationSubnetGroupsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeReplicationSubnetGroupsError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Returns the task assessment results from Amazon S3. This action always returns the latest results.</p>
    fn describe_replication_task_assessment_results(
        &self,
        input: DescribeReplicationTaskAssessmentResultsMessage,
    ) -> RusotoFuture<
        DescribeReplicationTaskAssessmentResultsResponse,
        DescribeReplicationTaskAssessmentResultsError,
    > {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribeReplicationTaskAssessmentResults",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeReplicationTaskAssessmentResultsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeReplicationTaskAssessmentResultsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns information about replication tasks for your account in the current region.</p>
    fn describe_replication_tasks(
        &self,
        input: DescribeReplicationTasksMessage,
    ) -> RusotoFuture<DescribeReplicationTasksResponse, DescribeReplicationTasksError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribeReplicationTasks",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeReplicationTasksResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeReplicationTasksError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Returns information about the schema for the specified endpoint.</p> <p/></p>
    fn describe_schemas(
        &self,
        input: DescribeSchemasMessage,
    ) -> RusotoFuture<DescribeSchemasResponse, DescribeSchemasError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeSchemas");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeSchemasResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeSchemasError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns table statistics on the database migration task, including table name, rows inserted, rows updated, and rows deleted.</p> <p>Note that the "last updated" column the DMS console only indicates the time that AWS DMS last updated the table statistics record for a table. It does not indicate the time of the last update to the table.</p>
    fn describe_table_statistics(
        &self,
        input: DescribeTableStatisticsMessage,
    ) -> RusotoFuture<DescribeTableStatisticsResponse, DescribeTableStatisticsError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeTableStatistics");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeTableStatisticsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTableStatisticsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Uploads the specified certificate.</p>
    fn import_certificate(
        &self,
        input: ImportCertificateMessage,
    ) -> RusotoFuture<ImportCertificateResponse, ImportCertificateError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.ImportCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ImportCertificateResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ImportCertificateError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists all tags for an AWS DMS resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceMessage,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.ListTagsForResource");
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

    /// <p>Modifies the specified endpoint.</p>
    fn modify_endpoint(
        &self,
        input: ModifyEndpointMessage,
    ) -> RusotoFuture<ModifyEndpointResponse, ModifyEndpointError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.ModifyEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ModifyEndpointResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ModifyEndpointError::from_response(response))),
                )
            }
        })
    }

    /// <p>Modifies an existing AWS DMS event notification subscription. </p>
    fn modify_event_subscription(
        &self,
        input: ModifyEventSubscriptionMessage,
    ) -> RusotoFuture<ModifyEventSubscriptionResponse, ModifyEventSubscriptionError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.ModifyEventSubscription");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ModifyEventSubscriptionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ModifyEventSubscriptionError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Modifies the replication instance to apply new settings. You can change one or more parameters by specifying these parameters and the new values in the request.</p> <p>Some settings are applied during the maintenance window.</p> <p/></p>
    fn modify_replication_instance(
        &self,
        input: ModifyReplicationInstanceMessage,
    ) -> RusotoFuture<ModifyReplicationInstanceResponse, ModifyReplicationInstanceError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.ModifyReplicationInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ModifyReplicationInstanceResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ModifyReplicationInstanceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Modifies the settings for the specified replication subnet group.</p>
    fn modify_replication_subnet_group(
        &self,
        input: ModifyReplicationSubnetGroupMessage,
    ) -> RusotoFuture<ModifyReplicationSubnetGroupResponse, ModifyReplicationSubnetGroupError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.ModifyReplicationSubnetGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ModifyReplicationSubnetGroupResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ModifyReplicationSubnetGroupError::from_response(response))
                }))
            }
        })
    }

    /// <p>Modifies the specified replication task.</p> <p>You can't modify the task endpoints. The task must be stopped before you can modify it. </p> <p>For more information about AWS DMS tasks, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.html">Working with Migration Tasks</a> in the <i>AWS Database Migration Service User Guide</i>.</p>
    fn modify_replication_task(
        &self,
        input: ModifyReplicationTaskMessage,
    ) -> RusotoFuture<ModifyReplicationTaskResponse, ModifyReplicationTaskError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.ModifyReplicationTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ModifyReplicationTaskResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ModifyReplicationTaskError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Reboots a replication instance. Rebooting results in a momentary outage, until the replication instance becomes available again.</p>
    fn reboot_replication_instance(
        &self,
        input: RebootReplicationInstanceMessage,
    ) -> RusotoFuture<RebootReplicationInstanceResponse, RebootReplicationInstanceError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.RebootReplicationInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RebootReplicationInstanceResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RebootReplicationInstanceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Populates the schema for the specified endpoint. This is an asynchronous operation and can take several minutes. You can check the status of this operation by calling the DescribeRefreshSchemasStatus operation.</p>
    fn refresh_schemas(
        &self,
        input: RefreshSchemasMessage,
    ) -> RusotoFuture<RefreshSchemasResponse, RefreshSchemasError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.RefreshSchemas");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RefreshSchemasResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RefreshSchemasError::from_response(response))),
                )
            }
        })
    }

    /// <p>Reloads the target database table with the source data. </p>
    fn reload_tables(
        &self,
        input: ReloadTablesMessage,
    ) -> RusotoFuture<ReloadTablesResponse, ReloadTablesError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.ReloadTables");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ReloadTablesResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ReloadTablesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes metadata tags from a DMS resource.</p>
    fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceMessage,
    ) -> RusotoFuture<RemoveTagsFromResourceResponse, RemoveTagsFromResourceError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.RemoveTagsFromResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RemoveTagsFromResourceResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(RemoveTagsFromResourceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Starts the replication task.</p> <p>For more information about AWS DMS tasks, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.html">Working with Migration Tasks </a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    fn start_replication_task(
        &self,
        input: StartReplicationTaskMessage,
    ) -> RusotoFuture<StartReplicationTaskResponse, StartReplicationTaskError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.StartReplicationTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartReplicationTaskResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartReplicationTaskError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p> Starts the replication task assessment for unsupported data types in the source database. </p>
    fn start_replication_task_assessment(
        &self,
        input: StartReplicationTaskAssessmentMessage,
    ) -> RusotoFuture<StartReplicationTaskAssessmentResponse, StartReplicationTaskAssessmentError>
    {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.StartReplicationTaskAssessment",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartReplicationTaskAssessmentResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartReplicationTaskAssessmentError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Stops the replication task.</p> <p/></p>
    fn stop_replication_task(
        &self,
        input: StopReplicationTaskMessage,
    ) -> RusotoFuture<StopReplicationTaskResponse, StopReplicationTaskError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.StopReplicationTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StopReplicationTaskResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StopReplicationTaskError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Tests the connection between the replication instance and the endpoint.</p>
    fn test_connection(
        &self,
        input: TestConnectionMessage,
    ) -> RusotoFuture<TestConnectionResponse, TestConnectionError> {
        let mut request = SignedRequest::new("POST", "dms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.TestConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<TestConnectionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TestConnectionError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
