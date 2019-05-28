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
/// <p>An object representing a container instance or task attachment.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Attachment {
    /// <p>Details of the attachment. For elastic network interfaces, this includes the network interface ID, the MAC address, the subnet ID, and the private IPv4 address.</p>
    #[serde(rename = "details")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<KeyValuePair>>,
    /// <p>The unique identifier for the attachment.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p> The status of the attachment. Valid values are <code>PRECREATED</code>, <code>CREATED</code>, <code>ATTACHING</code>, <code>ATTACHED</code>, <code>DETACHING</code>, <code>DETACHED</code>, and <code>DELETED</code>.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The type of the attachment, such as <code>ElasticNetworkInterface</code>.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>An object representing a change in state for a task attachment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AttachmentStateChange {
    /// <p>The Amazon Resource Name (ARN) of the attachment.</p>
    #[serde(rename = "attachmentArn")]
    pub attachment_arn: String,
    /// <p>The status of the attachment.</p>
    #[serde(rename = "status")]
    pub status: String,
}

/// <p>An attribute is a name-value pair associated with an Amazon ECS object. Attributes enable you to extend the Amazon ECS data model by adding custom metadata to your resources. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-placement-constraints.html#attributes">Attributes</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attribute {
    /// <p>The name of the attribute. Up to 128 letters (uppercase and lowercase), numbers, hyphens, underscores, and periods are allowed.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The ID of the target. You can specify the short form ID for a resource or the full Amazon Resource Name (ARN).</p>
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    /// <p>The type of the target with which to attach the attribute. This parameter is required if you use the short form ID for a resource instead of the full ARN.</p>
    #[serde(rename = "targetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<String>,
    /// <p>The value of the attribute. Up to 128 letters (uppercase and lowercase), numbers, hyphens, underscores, periods, at signs (@), forward slashes, colons, and spaces are allowed.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>An object representing the networking details for a task or service.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsVpcConfiguration {
    /// <p>Whether the task's elastic network interface receives a public IP address. The default value is <code>DISABLED</code>.</p>
    #[serde(rename = "assignPublicIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_public_ip: Option<String>,
    /// <p><p>The security groups associated with the task or service. If you do not specify a security group, the default security group for the VPC is used. There is a limit of 5 security groups that can be specified per <code>AwsVpcConfiguration</code>.</p> <note> <p>All specified security groups must be from the same VPC.</p> </note></p>
    #[serde(rename = "securityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// <p><p>The subnets associated with the task or service. There is a limit of 16 subnets that can be specified per <code>AwsVpcConfiguration</code>.</p> <note> <p>All specified subnets must be from the same VPC.</p> </note></p>
    #[serde(rename = "subnets")]
    pub subnets: Vec<String>,
}

/// <p>A regional grouping of one or more container instances on which you can run task requests. Each account receives a default cluster the first time you use the Amazon ECS service, but you may also create other clusters. Clusters may contain more than one instance type simultaneously.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Cluster {
    /// <p>The number of services that are running on the cluster in an <code>ACTIVE</code> state. You can view these services with <a>ListServices</a>.</p>
    #[serde(rename = "activeServicesCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_services_count: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) that identifies the cluster. The ARN contains the <code>arn:aws:ecs</code> namespace, followed by the Region of the cluster, the AWS account ID of the cluster owner, the <code>cluster</code> namespace, and then the cluster name. For example, <code>arn:aws:ecs:<i>region</i>:<i>012345678910</i>:cluster/<i>test</i> </code>..</p>
    #[serde(rename = "clusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <p>A user-generated string that you use to identify your cluster.</p>
    #[serde(rename = "clusterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// <p>The number of tasks in the cluster that are in the <code>PENDING</code> state.</p>
    #[serde(rename = "pendingTasksCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_tasks_count: Option<i64>,
    /// <p>The number of container instances registered into the cluster. This includes container instances in both <code>ACTIVE</code> and <code>DRAINING</code> status.</p>
    #[serde(rename = "registeredContainerInstancesCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_container_instances_count: Option<i64>,
    /// <p>The number of tasks in the cluster that are in the <code>RUNNING</code> state.</p>
    #[serde(rename = "runningTasksCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_tasks_count: Option<i64>,
    /// <p><p>Additional information about your clusters that are separated by launch type, including:</p> <ul> <li> <p>runningEC2TasksCount</p> </li> <li> <p>RunningFargateTasksCount</p> </li> <li> <p>pendingEC2TasksCount</p> </li> <li> <p>pendingFargateTasksCount</p> </li> <li> <p>activeEC2ServiceCount</p> </li> <li> <p>activeFargateServiceCount</p> </li> <li> <p>drainingEC2ServiceCount</p> </li> <li> <p>drainingFargateServiceCount</p> </li> </ul></p>
    #[serde(rename = "statistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<Vec<KeyValuePair>>,
    /// <p>The status of the cluster. The valid values are <code>ACTIVE</code> or <code>INACTIVE</code>. <code>ACTIVE</code> indicates that you can register container instances with the cluster and the associated instances can accept tasks.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The metadata that you apply to the cluster to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>A Docker container that is part of a task.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Container {
    /// <p>The Amazon Resource Name (ARN) of the container.</p>
    #[serde(rename = "containerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_arn: Option<String>,
    /// <p>The number of CPU units set for the container. The value will be <code>0</code> if no value was specified in the container definition when the task definition was registered.</p>
    #[serde(rename = "cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    /// <p>The exit code returned from the container.</p>
    #[serde(rename = "exitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    /// <p>The IDs of each GPU assigned to the container.</p>
    #[serde(rename = "gpuIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gpu_ids: Option<Vec<String>>,
    /// <p>The health status of the container. If health checks are not configured for this container in its task definition, then it reports the health status as <code>UNKNOWN</code>.</p>
    #[serde(rename = "healthStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
    /// <p>The last known status of the container.</p>
    #[serde(rename = "lastStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    /// <p>The hard limit (in MiB) of memory set for the container.</p>
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    /// <p>The soft limit (in MiB) of memory set for the container.</p>
    #[serde(rename = "memoryReservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<String>,
    /// <p>The name of the container.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The network bindings associated with the container.</p>
    #[serde(rename = "networkBindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_bindings: Option<Vec<NetworkBinding>>,
    /// <p>The network interfaces associated with the container.</p>
    #[serde(rename = "networkInterfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    /// <p>A short (255 max characters) human-readable string to provide additional details about a running or stopped container.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The ARN of the task.</p>
    #[serde(rename = "taskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
}

/// <p>Container definitions are used in task definitions to describe the different containers that are launched as part of a task.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerDefinition {
    /// <p>The command that is passed to the container. This parameter maps to <code>Cmd</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>COMMAND</code> parameter to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>. For more information, see <a href="https://docs.docker.com/engine/reference/builder/#cmd">https://docs.docker.com/engine/reference/builder/#cmd</a>. If there are multiple arguments, each argument should be a separated string in the array.</p>
    #[serde(rename = "command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// <p>The number of <code>cpu</code> units reserved for the container. This parameter maps to <code>CpuShares</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--cpu-shares</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <p>This field is optional for tasks using the Fargate launch type, and the only requirement is that the total amount of CPU reserved for all containers within a task be lower than the task-level <code>cpu</code> value.</p> <note> <p>You can determine the number of CPU units that are available per EC2 instance type by multiplying the vCPUs listed for that instance type on the <a href="http://aws.amazon.com/ec2/instance-types/">Amazon EC2 Instances</a> detail page by 1,024.</p> </note> <p>For example, if you run a single-container task on a single-core instance type with 512 CPU units specified for that container, and that is the only task running on the container instance, that container could use the full 1,024 CPU unit share at any given time. However, if you launched another copy of the same task on that container instance, each task would be guaranteed a minimum of 512 CPU units when needed, and each container could float to higher CPU usage if the other container was not using it, but if both tasks were 100% active all of the time, they would be limited to 512 CPU units.</p> <p>Linux containers share unallocated CPU units with other containers on the container instance with the same ratio as their allocated amount. For example, if you run a single-container task on a single-core instance type with 512 CPU units specified for that container, and that is the only task running on the container instance, that container could use the full 1,024 CPU unit share at any given time. However, if you launched another copy of the same task on that container instance, each task would be guaranteed a minimum of 512 CPU units when needed, and each container could float to higher CPU usage if the other container was not using it, but if both tasks were 100% active all of the time, they would be limited to 512 CPU units.</p> <p>On Linux container instances, the Docker daemon on the container instance uses the CPU value to calculate the relative CPU share ratios for running containers. For more information, see <a href="https://docs.docker.com/engine/reference/run/#cpu-share-constraint">CPU share constraint</a> in the Docker documentation. The minimum valid CPU share value that the Linux kernel allows is 2. However, the CPU parameter is not required, and you can use CPU values below 2 in your container definitions. For CPU values below 2 (including null), the behavior varies based on your Amazon ECS container agent version:</p> <ul> <li> <p> <b>Agent versions less than or equal to 1.1.0:</b> Null and zero CPU values are passed to Docker as 0, which Docker then converts to 1,024 CPU shares. CPU values of 1 are passed to Docker as 1, which the Linux kernel converts to two CPU shares.</p> </li> <li> <p> <b>Agent versions greater than or equal to 1.2.0:</b> Null, zero, and CPU values of 1 are passed to Docker as 2.</p> </li> </ul> <p>On Windows container instances, the CPU limit is enforced as an absolute limit, or a quota. Windows containers only have access to the specified amount of CPU that is described in the task definition.</p>
    #[serde(rename = "cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<i64>,
    /// <p><p>When this parameter is true, networking is disabled within the container. This parameter maps to <code>NetworkDisabled</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a>.</p> <note> <p>This parameter is not supported for Windows containers.</p> </note></p>
    #[serde(rename = "disableNetworking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_networking: Option<bool>,
    /// <p><p>A list of DNS search domains that are presented to the container. This parameter maps to <code>DnsSearch</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--dns-search</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <note> <p>This parameter is not supported for Windows containers.</p> </note></p>
    #[serde(rename = "dnsSearchDomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_search_domains: Option<Vec<String>>,
    /// <p><p>A list of DNS servers that are presented to the container. This parameter maps to <code>Dns</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--dns</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <note> <p>This parameter is not supported for Windows containers.</p> </note></p>
    #[serde(rename = "dnsServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_servers: Option<Vec<String>>,
    /// <p>A key/value map of labels to add to the container. This parameter maps to <code>Labels</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--label</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>. This parameter requires version 1.18 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: <code>sudo docker version --format '{{.Server.APIVersion}}'</code> </p>
    #[serde(rename = "dockerLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_labels: Option<::std::collections::HashMap<String, String>>,
    /// <p><p>A list of strings to provide custom labels for SELinux and AppArmor multi-level security systems. This field is not valid for containers in tasks using the Fargate launch type.</p> <p>This parameter maps to <code>SecurityOpt</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--security-opt</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <note> <p>The Amazon ECS container agent running on a container instance must register with the <code>ECS<em>SELINUX</em>CAPABLE=true</code> or <code>ECS<em>APPARMOR</em>CAPABLE=true</code> environment variables before containers placed on that instance can use these security options. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-config.html">Amazon ECS Container Agent Configuration</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>This parameter is not supported for Windows containers.</p> </note></p>
    #[serde(rename = "dockerSecurityOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_security_options: Option<Vec<String>>,
    /// <p><important> <p>Early versions of the Amazon ECS container agent do not properly handle <code>entryPoint</code> parameters. If you have problems using <code>entryPoint</code>, update your container agent or enter your commands and arguments as <code>command</code> array items instead.</p> </important> <p>The entry point that is passed to the container. This parameter maps to <code>Entrypoint</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--entrypoint</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>. For more information, see <a href="https://docs.docker.com/engine/reference/builder/#entrypoint">https://docs.docker.com/engine/reference/builder/#entrypoint</a>.</p></p>
    #[serde(rename = "entryPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<Vec<String>>,
    /// <p><p>The environment variables to pass to a container. This parameter maps to <code>Env</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--env</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <important> <p>We do not recommend using plaintext environment variables for sensitive information, such as credential data.</p> </important></p>
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    /// <p>If the <code>essential</code> parameter of a container is marked as <code>true</code>, and that container fails or stops for any reason, all other containers that are part of the task are stopped. If the <code>essential</code> parameter of a container is marked as <code>false</code>, then its failure does not affect the rest of the containers in a task. If this parameter is omitted, a container is assumed to be essential.</p> <p>All tasks must have at least one essential container. If you have an application that is composed of multiple containers, you should group containers that are used for a common purpose into components, and separate the different components into multiple task definitions. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/application_architecture.html">Application Architecture</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "essential")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub essential: Option<bool>,
    /// <p><p>A list of hostnames and IP address mappings to append to the <code>/etc/hosts</code> file on the container. This parameter maps to <code>ExtraHosts</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--add-host</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <note> <p>This parameter is not supported for Windows containers or tasks that use the <code>awsvpc</code> network mode.</p> </note></p>
    #[serde(rename = "extraHosts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_hosts: Option<Vec<HostEntry>>,
    /// <p>The health check command and associated configuration parameters for the container. This parameter maps to <code>HealthCheck</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>HEALTHCHECK</code> parameter of <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p>
    #[serde(rename = "healthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check: Option<HealthCheck>,
    /// <p><p>The hostname to use for your container. This parameter maps to <code>Hostname</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--hostname</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <note> <p>The <code>hostname</code> parameter is not supported if you are using the <code>awsvpc</code> network mode.</p> </note></p>
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// <p><p>The image used to start a container. This string is passed directly to the Docker daemon. Images in the Docker Hub registry are available by default. Other repositories are specified with either <code> <i>repository-url</i>/<i>image</i>:<i>tag</i> </code> or <code> <i>repository-url</i>/<i>image</i>@<i>digest</i> </code>. Up to 255 letters (uppercase and lowercase), numbers, hyphens, underscores, colons, periods, forward slashes, and number signs are allowed. This parameter maps to <code>Image</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>IMAGE</code> parameter of <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <ul> <li> <p>When a new task starts, the Amazon ECS container agent pulls the latest version of the specified image and tag for the container to use. However, subsequent updates to a repository image are not propagated to already running tasks.</p> </li> <li> <p>Images in Amazon ECR repositories can be specified by either using the full <code>registry/repository:tag</code> or <code>registry/repository@digest</code>. For example, <code>012345678910.dkr.ecr.&lt;region-name&gt;.amazonaws.com/&lt;repository-name&gt;:latest</code> or <code>012345678910.dkr.ecr.&lt;region-name&gt;.amazonaws.com/&lt;repository-name&gt;@sha256:94afd1f2e64d908bc90dbca0035a5b567EXAMPLE</code>. </p> </li> <li> <p>Images in official repositories on Docker Hub use a single name (for example, <code>ubuntu</code> or <code>mongo</code>).</p> </li> <li> <p>Images in other repositories on Docker Hub are qualified with an organization name (for example, <code>amazon/amazon-ecs-agent</code>).</p> </li> <li> <p>Images in other online repositories are qualified further by a domain name (for example, <code>quay.io/assemblyline/ubuntu</code>).</p> </li> </ul></p>
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// <p>When this parameter is <code>true</code>, this allows you to deploy containerized applications that require <code>stdin</code> or a <code>tty</code> to be allocated. This parameter maps to <code>OpenStdin</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--interactive</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p>
    #[serde(rename = "interactive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive: Option<bool>,
    /// <p><p>The <code>link</code> parameter allows containers to communicate with each other without the need for port mappings. Only supported if the network mode of a task definition is set to <code>bridge</code>. The <code>name:internalName</code> construct is analogous to <code>name:alias</code> in Docker links. Up to 255 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed. For more information about linking Docker containers, go to <a href="https://docs.docker.com/engine/userguide/networking/default_network/dockerlinks/">https://docs.docker.com/engine/userguide/networking/default_network/dockerlinks/</a>. This parameter maps to <code>Links</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--link</code> option to <a href="https://docs.docker.com/engine/reference/commandline/run/"> <code>docker run</code> </a>.</p> <note> <p>This parameter is not supported for Windows containers.</p> </note> <important> <p>Containers that are collocated on a single container instance may be able to communicate with each other without requiring links or host port mappings. Network isolation is achieved on the container instance using security groups and VPC settings.</p> </important></p>
    #[serde(rename = "links")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    /// <p><p>Linux-specific modifications that are applied to the container, such as Linux <a>KernelCapabilities</a>.</p> <note> <p>This parameter is not supported for Windows containers.</p> </note></p>
    #[serde(rename = "linuxParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux_parameters: Option<LinuxParameters>,
    /// <p><p>The log configuration specification for the container.</p> <p>If you are using the Fargate launch type, the only supported value is <code>awslogs</code>.</p> <p>This parameter maps to <code>LogConfig</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--log-driver</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>. By default, containers use the same logging driver that the Docker daemon uses. However the container may use a different logging driver than the Docker daemon by specifying a log driver with this parameter in the container definition. To use a different logging driver for a container, the log system must be configured properly on the container instance (or on a different log server for remote logging options). For more information on the options for different supported log drivers, see <a href="https://docs.docker.com/engine/admin/logging/overview/">Configure logging drivers</a> in the Docker documentation.</p> <note> <p>Amazon ECS currently supports a subset of the logging drivers available to the Docker daemon (shown in the <a>LogConfiguration</a> data type). Additional log drivers may be available in future releases of the Amazon ECS container agent.</p> </note> <p>This parameter requires version 1.18 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: <code>sudo docker version --format &#39;{{.Server.APIVersion}}&#39;</code> </p> <note> <p>The Amazon ECS container agent running on a container instance must register the logging drivers available on that instance with the <code>ECS<em>AVAILABLE</em>LOGGING_DRIVERS</code> environment variable before containers placed on that instance can use these log configuration options. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-config.html">Amazon ECS Container Agent Configuration</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </note></p>
    #[serde(rename = "logConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LogConfiguration>,
    /// <p>The hard limit (in MiB) of memory to present to the container. If your container attempts to exceed the memory specified here, the container is killed. This parameter maps to <code>Memory</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--memory</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <p>If your containers are part of a task using the Fargate launch type, this field is optional and the only requirement is that the total amount of memory reserved for all containers within a task be lower than the task <code>memory</code> value.</p> <p>For containers that are part of a task using the EC2 launch type, you must specify a non-zero integer for one or both of <code>memory</code> or <code>memoryReservation</code> in container definitions. If you specify both, <code>memory</code> must be greater than <code>memoryReservation</code>. If you specify <code>memoryReservation</code>, then that value is subtracted from the available memory resources for the container instance on which the container is placed. Otherwise, the value of <code>memory</code> is used.</p> <p>The Docker daemon reserves a minimum of 4 MiB of memory for a container, so you should not specify fewer than 4 MiB of memory for your containers. </p>
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// <p>The soft limit (in MiB) of memory to reserve for the container. When system memory is under heavy contention, Docker attempts to keep the container memory to this soft limit. However, your container can consume more memory when it needs to, up to either the hard limit specified with the <code>memory</code> parameter (if applicable), or all of the available memory on the container instance, whichever comes first. This parameter maps to <code>MemoryReservation</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--memory-reservation</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <p>You must specify a non-zero integer for one or both of <code>memory</code> or <code>memoryReservation</code> in container definitions. If you specify both, <code>memory</code> must be greater than <code>memoryReservation</code>. If you specify <code>memoryReservation</code>, then that value is subtracted from the available memory resources for the container instance on which the container is placed. Otherwise, the value of <code>memory</code> is used.</p> <p>For example, if your container normally uses 128 MiB of memory, but occasionally bursts to 256 MiB of memory for short periods of time, you can set a <code>memoryReservation</code> of 128 MiB, and a <code>memory</code> hard limit of 300 MiB. This configuration would allow the container to only reserve 128 MiB of memory from the remaining resources on the container instance, but also allow the container to consume more memory resources when needed.</p> <p>The Docker daemon reserves a minimum of 4 MiB of memory for a container, so you should not specify fewer than 4 MiB of memory for your containers. </p>
    #[serde(rename = "memoryReservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i64>,
    /// <p>The mount points for data volumes in your container.</p> <p>This parameter maps to <code>Volumes</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--volume</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <p>Windows containers can mount whole directories on the same drive as <code>$env:ProgramData</code>. Windows containers cannot mount directories on a different drive, and mount point cannot be across drives.</p>
    #[serde(rename = "mountPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_points: Option<Vec<MountPoint>>,
    /// <p>The name of a container. If you are linking multiple containers together in a task definition, the <code>name</code> of one container can be entered in the <code>links</code> of another container to connect the containers. Up to 255 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed. This parameter maps to <code>name</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--name</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>. </p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The list of port mappings for the container. Port mappings allow containers to access ports on the host container instance to send or receive traffic.</p> <p>For task definitions that use the <code>awsvpc</code> network mode, you should only specify the <code>containerPort</code>. The <code>hostPort</code> can be left blank or it must be the same value as the <code>containerPort</code>.</p> <p>Port mappings on Windows use the <code>NetNAT</code> gateway address rather than <code>localhost</code>. There is no loopback for port mappings on Windows, so you cannot access a container&#39;s mapped port from the host itself. </p> <p>This parameter maps to <code>PortBindings</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--publish</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>. If the network mode of a task definition is set to <code>none</code>, then you can&#39;t specify port mappings. If the network mode of a task definition is set to <code>host</code>, then host ports must either be undefined or they must match the container port in the port mapping.</p> <note> <p>After a task reaches the <code>RUNNING</code> status, manual and automatic host and container port assignments are visible in the <b>Network Bindings</b> section of a container description for a selected task in the Amazon ECS console. The assignments are also visible in the <code>networkBindings</code> section <a>DescribeTasks</a> responses.</p> </note></p>
    #[serde(rename = "portMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_mappings: Option<Vec<PortMapping>>,
    /// <p><p>When this parameter is true, the container is given elevated privileges on the host container instance (similar to the <code>root</code> user). This parameter maps to <code>Privileged</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--privileged</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <note> <p>This parameter is not supported for Windows containers or tasks using the Fargate launch type.</p> </note></p>
    #[serde(rename = "privileged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    /// <p>When this parameter is <code>true</code>, a TTY is allocated. This parameter maps to <code>Tty</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--tty</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p>
    #[serde(rename = "pseudoTerminal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pseudo_terminal: Option<bool>,
    /// <p><p>When this parameter is true, the container is given read-only access to its root file system. This parameter maps to <code>ReadonlyRootfs</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--read-only</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <note> <p>This parameter is not supported for Windows containers.</p> </note></p>
    #[serde(rename = "readonlyRootFilesystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_root_filesystem: Option<bool>,
    /// <p>The private repository authentication credentials to use.</p>
    #[serde(rename = "repositoryCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_credentials: Option<RepositoryCredentials>,
    /// <p>The type and amount of a resource to assign to a container. The only supported resource is a GPU.</p>
    #[serde(rename = "resourceRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<Vec<ResourceRequirement>>,
    /// <p>The secrets to pass to the container. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/specifying-sensitive-data.html">Specifying Sensitive Data</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "secrets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<Secret>>,
    /// <p><p>A list of namespaced kernel parameters to set in the container. This parameter maps to <code>Sysctls</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--sysctl</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <note> <p>It is not recommended that you specify network-related <code>systemControls</code> parameters for multiple containers in a single task that also uses either the <code>awsvpc</code> or <code>host</code> network modes. For tasks that use the <code>awsvpc</code> network mode, the container that is started last determines which <code>systemControls</code> parameters take effect. For tasks that use the <code>host</code> network mode, it changes the container instance&#39;s namespaced kernel parameters as well as the containers.</p> </note></p>
    #[serde(rename = "systemControls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_controls: Option<Vec<SystemControl>>,
    /// <p><p>A list of <code>ulimits</code> to set in the container. This parameter maps to <code>Ulimits</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--ulimit</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>. Valid naming values are displayed in the <a>Ulimit</a> data type. This parameter requires version 1.18 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: <code>sudo docker version --format &#39;{{.Server.APIVersion}}&#39;</code> </p> <note> <p>This parameter is not supported for Windows containers.</p> </note></p>
    #[serde(rename = "ulimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,
    /// <p><p>The user name to use inside the container. This parameter maps to <code>User</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--user</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <note> <p>This parameter is not supported for Windows containers.</p> </note></p>
    #[serde(rename = "user")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// <p>Data volumes to mount from another container. This parameter maps to <code>VolumesFrom</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--volumes-from</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p>
    #[serde(rename = "volumesFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes_from: Option<Vec<VolumeFrom>>,
    /// <p>The working directory in which to run commands inside the container. This parameter maps to <code>WorkingDir</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--workdir</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p>
    #[serde(rename = "workingDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

/// <p>An EC2 instance that is running the Amazon ECS agent and has been registered with a cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ContainerInstance {
    /// <p>This parameter returns <code>true</code> if the agent is connected to Amazon ECS. Registered instances with an agent that may be unhealthy or stopped return <code>false</code>. Only instances connected to an agent can accept placement requests.</p>
    #[serde(rename = "agentConnected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_connected: Option<bool>,
    /// <p>The status of the most recent agent update. If an update has never been requested, this value is <code>NULL</code>.</p>
    #[serde(rename = "agentUpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_update_status: Option<String>,
    /// <p>The elastic network interfaces associated with the container instance.</p>
    #[serde(rename = "attachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    /// <p>The attributes set for the container instance, either by the Amazon ECS container agent at instance registration or manually with the <a>PutAttributes</a> operation.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    /// <p>The Amazon Resource Name (ARN) of the container instance. The ARN contains the <code>arn:aws:ecs</code> namespace, followed by the Region of the container instance, the AWS account ID of the container instance owner, the <code>container-instance</code> namespace, and then the container instance ID. For example, <code>arn:aws:ecs:<i>region</i>:<i>aws_account_id</i>:container-instance/<i>container_instance_ID</i> </code>.</p>
    #[serde(rename = "containerInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_arn: Option<String>,
    /// <p>The EC2 instance ID of the container instance.</p>
    #[serde(rename = "ec2InstanceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_instance_id: Option<String>,
    /// <p>The number of tasks on the container instance that are in the <code>PENDING</code> status.</p>
    #[serde(rename = "pendingTasksCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_tasks_count: Option<i64>,
    /// <p>The Unix timestamp for when the container instance was registered.</p>
    #[serde(rename = "registeredAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_at: Option<f64>,
    /// <p>For CPU and memory resource types, this parameter describes the amount of each resource that was available on the container instance when the container agent registered it with Amazon ECS. This value represents the total amount of CPU and memory that can be allocated on this container instance to tasks. For port resource types, this parameter describes the ports that were reserved by the Amazon ECS container agent when it registered the container instance with Amazon ECS.</p>
    #[serde(rename = "registeredResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_resources: Option<Vec<Resource>>,
    /// <p>For CPU and memory resource types, this parameter describes the remaining CPU and memory that has not already been allocated to tasks and is therefore available for new tasks. For port resource types, this parameter describes the ports that were reserved by the Amazon ECS container agent (at instance registration time) and any task containers that have reserved port mappings on the host (with the <code>host</code> or <code>bridge</code> network mode). Any port that is not specified here is available for new tasks.</p>
    #[serde(rename = "remainingResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_resources: Option<Vec<Resource>>,
    /// <p>The number of tasks on the container instance that are in the <code>RUNNING</code> status.</p>
    #[serde(rename = "runningTasksCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_tasks_count: Option<i64>,
    /// <p>The status of the container instance. The valid values are <code>ACTIVE</code>, <code>INACTIVE</code>, or <code>DRAINING</code>. <code>ACTIVE</code> indicates that the container instance can accept tasks. <code>DRAINING</code> indicates that new tasks are not placed on the container instance and any service tasks running on the container instance are removed if possible. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/container-instance-draining.html">Container Instance Draining</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The metadata that you apply to the container instance to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The version counter for the container instance. Every time a container instance experiences a change that triggers a CloudWatch event, the version counter is incremented. If you are replicating your Amazon ECS container instance state with CloudWatch Events, you can compare the version of a container instance reported by the Amazon ECS APIs with the version reported in CloudWatch Events for the container instance (inside the <code>detail</code> object) to verify that the version in your event stream is current.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    /// <p>The version information for the Amazon ECS container agent and Docker daemon running on the container instance.</p>
    #[serde(rename = "versionInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_info: Option<VersionInfo>,
}

/// <p>The overrides that should be sent to a container.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerOverride {
    /// <p>The command to send to the container that overrides the default command from the Docker image or the task definition. You must also specify a container name.</p>
    #[serde(rename = "command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// <p>The number of <code>cpu</code> units reserved for the container, instead of the default value from the task definition. You must also specify a container name.</p>
    #[serde(rename = "cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<i64>,
    /// <p>The environment variables to send to the container. You can add new environment variables, which are added to the container at launch, or you can override the existing environment variables from the Docker image or the task definition. You must also specify a container name.</p>
    #[serde(rename = "environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    /// <p>The hard limit (in MiB) of memory to present to the container, instead of the default value from the task definition. If your container attempts to exceed the memory specified here, the container is killed. You must also specify a container name.</p>
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i64>,
    /// <p>The soft limit (in MiB) of memory to reserve for the container, instead of the default value from the task definition. You must also specify a container name.</p>
    #[serde(rename = "memoryReservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reservation: Option<i64>,
    /// <p>The name of the container that receives the override. This parameter is required if any override is specified.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type and amount of a resource to assign to a container, instead of the default value from the task definition. The only supported resource is a GPU.</p>
    #[serde(rename = "resourceRequirements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<Vec<ResourceRequirement>>,
}

/// <p>An object representing a change in state for a container.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ContainerStateChange {
    /// <p>The name of the container.</p>
    #[serde(rename = "containerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    /// <p>The exit code for the container, if the state change is a result of the container exiting.</p>
    #[serde(rename = "exitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    /// <p>Any network bindings associated with the container.</p>
    #[serde(rename = "networkBindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_bindings: Option<Vec<NetworkBinding>>,
    /// <p>The reason for the state change.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The status of the container.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateClusterRequest {
    /// <p>The name of your cluster. If you do not specify a name for your cluster, you create a cluster named <code>default</code>. Up to 255 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "clusterName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    /// <p>The metadata that you apply to the cluster to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateClusterResponse {
    /// <p>The full description of your new cluster.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateServiceRequest {
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Up to 32 ASCII characters are allowed.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster on which to run your service. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>Optional deployment parameters that control how many tasks run during the deployment and the ordering of stopping and starting tasks.</p>
    #[serde(rename = "deploymentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_configuration: Option<DeploymentConfiguration>,
    /// <p>The deployment controller to use for the service.</p>
    #[serde(rename = "deploymentController")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_controller: Option<DeploymentController>,
    /// <p>The number of instantiations of the specified task definition to place and keep running on your cluster.</p>
    #[serde(rename = "desiredCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<i64>,
    /// <p>Specifies whether to enable Amazon ECS managed tags for the tasks within the service. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-using-tags.html">Tagging Your Amazon ECS Resources</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "enableECSManagedTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ecs_managed_tags: Option<bool>,
    /// <p>The period of time, in seconds, that the Amazon ECS service scheduler should ignore unhealthy Elastic Load Balancing target health checks after a task has first started. This is only valid if your service is configured to use a load balancer. If your service's tasks take a while to start and respond to Elastic Load Balancing health checks, you can specify a health check grace period of up to 7,200 seconds. During that time, the ECS service scheduler ignores health check status. This grace period can prevent the ECS service scheduler from marking tasks as unhealthy and stopping them before they have time to come up.</p>
    #[serde(rename = "healthCheckGracePeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_grace_period_seconds: Option<i64>,
    /// <p>The launch type on which to run your service. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_types.html">Amazon ECS Launch Types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "launchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    /// <p>A load balancer object representing the load balancer to use with your service.</p> <p>If the service is using the <code>ECS</code> deployment controller, you are limited to one load balancer or target group.</p> <p>If the service is using the <code>CODE_DEPLOY</code> deployment controller, the service is required to use either an Application Load Balancer or Network Load Balancer. When creating an AWS CodeDeploy deployment group, you specify two target groups (referred to as a <code>targetGroupPair</code>). During a deployment, AWS CodeDeploy determines which task set in your service has the status <code>PRIMARY</code> and associates one target group with it, and then associates the other target group with the replacement task set. The load balancer can also have up to two listeners: a required listener for production traffic and an optional listener that allows you perform validation tests with Lambda functions before routing production traffic to it.</p> <p>After you create a service using the <code>ECS</code> deployment controller, the load balancer name or target group ARN, container name, and container port specified in the service definition are immutable. If you are using the <code>CODE_DEPLOY</code> deployment controller, these values can be changed when updating the service.</p> <p>For Classic Load Balancers, this object must contain the load balancer name, the container name (as it appears in a container definition), and the container port to access from the load balancer. When a task from this service is placed on a container instance, the container instance is registered with the load balancer specified here.</p> <p>For Application Load Balancers and Network Load Balancers, this object must contain the load balancer target group ARN, the container name (as it appears in a container definition), and the container port to access from the load balancer. When a task from this service is placed on a container instance, the container instance and port combination is registered as a target in the target group specified here.</p> <p>Services with tasks that use the <code>awsvpc</code> network mode (for example, those with the Fargate launch type) only support Application Load Balancers and Network Load Balancers. Classic Load Balancers are not supported. Also, when you create any target groups for these services, you must choose <code>ip</code> as the target type, not <code>instance</code>, because tasks that use the <code>awsvpc</code> network mode are associated with an elastic network interface, not an Amazon EC2 instance.</p>
    #[serde(rename = "loadBalancers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<LoadBalancer>>,
    /// <p>The network configuration for the service. This parameter is required for task definitions that use the <code>awsvpc</code> network mode to receive their own elastic network interface, and it is not supported for other network modes. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-networking.html">Task Networking</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "networkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// <p>An array of placement constraint objects to use for tasks in your service. You can specify a maximum of 10 constraints per task (this limit includes constraints in the task definition and those specified at runtime). </p>
    #[serde(rename = "placementConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,
    /// <p>The placement strategy objects to use for tasks in your service. You can specify a maximum of five strategy rules per service.</p>
    #[serde(rename = "placementStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_strategy: Option<Vec<PlacementStrategy>>,
    /// <p>The platform version on which your tasks in the service are running. A platform version is only specified for tasks using the Fargate launch type. If one is not specified, the <code>LATEST</code> platform version is used by default. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html">AWS Fargate Platform Versions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "platformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>Specifies whether to propagate the tags from the task definition or the service to the tasks in the service. If no value is specified, the tags are not propagated. Tags can only be propagated to the tasks within the service during service creation. To add tags to a task after service creation, use the <a>TagResource</a> API action.</p>
    #[serde(rename = "propagateTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
    /// <p>The name or full Amazon Resource Name (ARN) of the IAM role that allows Amazon ECS to make calls to your load balancer on your behalf. This parameter is only permitted if you are using a load balancer with your service and your task definition does not use the <code>awsvpc</code> network mode. If you specify the <code>role</code> parameter, you must also specify a load balancer object with the <code>loadBalancers</code> parameter.</p> <important> <p>If your account has already created the Amazon ECS service-linked role, that role is used by default for your service unless you specify a role here. The service-linked role is required if your task definition uses the <code>awsvpc</code> network mode, in which case you should not specify a role here. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/using-service-linked-roles.html">Using Service-Linked Roles for Amazon ECS</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </important> <p>If your specified role has a path other than <code>/</code>, then you must either specify the full role ARN (this is recommended) or prefix the role name with the path. For example, if a role with the name <code>bar</code> has a path of <code>/foo/</code> then you would specify <code>/foo/bar</code> as the role name. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-friendly-names">Friendly Names and Paths</a> in the <i>IAM User Guide</i>.</p>
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p><p>The scheduling strategy to use for the service. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs_services.html">Services</a>.</p> <p>There are two service scheduler strategies available:</p> <ul> <li> <p> <code>REPLICA</code>-The replica scheduling strategy places and maintains the desired number of tasks across your cluster. By default, the service scheduler spreads tasks across Availability Zones. You can use task placement strategies and constraints to customize task placement decisions. This scheduler strategy is required if using the <code>CODE<em>DEPLOY</code> deployment controller.</p> </li> <li> <p> <code>DAEMON</code>-The daemon scheduling strategy deploys exactly one task on each active container instance that meets all of the task placement constraints that you specify in your cluster. When you are using this strategy, there is no need to specify a desired number of tasks, a task placement strategy, or use Service Auto Scaling policies.</p> <note> <p>Tasks using the Fargate launch type or the <code>CODE</em>DEPLOY</code> deploymenet controller do not support the <code>DAEMON</code> scheduling strategy.</p> </note> </li> </ul></p>
    #[serde(rename = "schedulingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_strategy: Option<String>,
    /// <p>The name of your service. Up to 255 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed. Service names must be unique within a cluster, but you can have similarly named services in multiple clusters within a Region or across multiple Regions.</p>
    #[serde(rename = "serviceName")]
    pub service_name: String,
    /// <p><p>The details of the service discovery registries to assign to this service. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-discovery.html">Service Discovery</a>.</p> <note> <p>Service discovery is supported for Fargate tasks if you are using platform version v1.1.0 or later. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html">AWS Fargate Platform Versions</a>.</p> </note></p>
    #[serde(rename = "serviceRegistries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_registries: Option<Vec<ServiceRegistry>>,
    /// <p>The metadata that you apply to the service to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define. When a service is deleted, the tags are deleted as well. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or full ARN of the task definition to run in your service. If a <code>revision</code> is not specified, the latest <code>ACTIVE</code> revision is used.</p>
    #[serde(rename = "taskDefinition")]
    pub task_definition: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateServiceResponse {
    /// <p>The full description of your service following the create call.</p> <p>If a service is using the <code>ECS</code> deployment controller, the <code>deploymentController</code> and <code>taskSets</code> parameters will not be returned.</p> <p>If the service is using the <code>CODE_DEPLOY</code> deployment controller, the <code>deploymentController</code>, <code>taskSets</code> and <code>deployments</code> parameters will be returned, however the <code>deployments</code> parameter will be an empty list.</p>
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAccountSettingRequest {
    /// <p>The resource name for which to disable the new format. If <code>serviceLongArnFormat</code> is specified, the ARN for your Amazon ECS services is affected. If <code>taskLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS tasks is affected. If <code>containerInstanceLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS container instances is affected.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The ARN of the principal, which can be an IAM user, IAM role, or the root user. If you specify the root user, it modifies the ARN and resource ID format for all IAM users, IAM roles, and the root user of the account unless an IAM user or role explicitly overrides these settings for themselves. If this field is omitted, the setting are changed only for the authenticated user.</p>
    #[serde(rename = "principalArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteAccountSettingResponse {
    /// <p>The account setting for the specified principal ARN.</p>
    #[serde(rename = "setting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting: Option<Setting>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAttributesRequest {
    /// <p>The attributes to delete from your resource. You can specify up to 10 attributes per request. For custom attributes, specify the attribute name and target ID, but do not specify the value. If you specify the target ID using the short form, you must also specify the target type.</p>
    #[serde(rename = "attributes")]
    pub attributes: Vec<Attribute>,
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that contains the resource to delete attributes. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteAttributesResponse {
    /// <p>A list of attribute objects that were successfully deleted from your resource.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteClusterRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster to delete.</p>
    #[serde(rename = "cluster")]
    pub cluster: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteClusterResponse {
    /// <p>The full description of the deleted cluster.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteServiceRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service to delete. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>If <code>true</code>, allows you to delete a service even if it has not been scaled down to zero tasks. It is only necessary to use this if the service is using the <code>REPLICA</code> scheduling strategy.</p>
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// <p>The name of the service to delete.</p>
    #[serde(rename = "service")]
    pub service: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteServiceResponse {
    /// <p>The full description of the deleted service.</p>
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

/// <p>The details of an Amazon ECS service deployment. This is used when a service uses the <code>CODE_DEPLOY</code> deployment controller type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Deployment {
    /// <p>The Unix timestamp for when the service deployment was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The most recent desired count of tasks that was specified for the service to deploy or maintain.</p>
    #[serde(rename = "desiredCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<i64>,
    /// <p>The ID of the deployment.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The launch type the tasks in the service are using. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_types.html">Amazon ECS Launch Types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "launchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    /// <p>The VPC subnet and security group configuration for tasks that receive their own elastic network interface by using the <code>awsvpc</code> networking mode.</p>
    #[serde(rename = "networkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// <p>The number of tasks in the deployment that are in the <code>PENDING</code> status.</p>
    #[serde(rename = "pendingCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<i64>,
    /// <p>The platform version on which your tasks in the service are running. A platform version is only specified for tasks using the Fargate launch type. If one is not specified, the <code>LATEST</code> platform version is used by default. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html">AWS Fargate Platform Versions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "platformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>The number of tasks in the deployment that are in the <code>RUNNING</code> status.</p>
    #[serde(rename = "runningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i64>,
    /// <p><p>The status of the deployment. The following describes each state:</p> <dl> <dt>PRIMARY</dt> <dd> <p>The most recent deployment of a service.</p> </dd> <dt>ACTIVE</dt> <dd> <p>A service deployment that still has running tasks, but are in the process of being replaced with a new <code>PRIMARY</code> deployment.</p> </dd> <dt>INACTIVE</dt> <dd> <p>A deployment that has been completely replaced.</p> </dd> </dl></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The most recent task definition that was specified for the tasks in the service to use.</p>
    #[serde(rename = "taskDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<String>,
    /// <p>The Unix timestamp for when the service deployment was last updated.</p>
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p>Optional deployment parameters that control how many tasks run during the deployment and the ordering of stopping and starting tasks.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeploymentConfiguration {
    /// <p>If a service is using the rolling update (<code>ECS</code>) deployment type, the <b>maximum percent</b> parameter represents an upper limit on the number of tasks in a service that are allowed in the <code>RUNNING</code> or <code>PENDING</code> state during a deployment, as a percentage of the desired number of tasks (rounded down to the nearest integer), and while any container instances are in the <code>DRAINING</code> state if the service contains tasks using the EC2 launch type. This parameter enables you to define the deployment batch size. For example, if your service has a desired number of four tasks and a maximum percent value of 200%, the scheduler may start four new tasks before stopping the four older tasks (provided that the cluster resources required to do this are available). The default value for maximum percent is 200%.</p> <p>If a service is using the blue/green (<code>CODE_DEPLOY</code>) deployment type and tasks that use the EC2 launch type, the <b>maximum percent</b> value is set to the default value and is used to define the upper limit on the number of the tasks in the service that remain in the <code>RUNNING</code> state while the container instances are in the <code>DRAINING</code> state. If the tasks in the service use the Fargate launch type, the maximum percent value is not used, although it is returned when describing your service.</p>
    #[serde(rename = "maximumPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_percent: Option<i64>,
    /// <p>If a service is using the rolling update (<code>ECS</code>) deployment type, the <b>minimum healthy percent</b> represents a lower limit on the number of tasks in a service that must remain in the <code>RUNNING</code> state during a deployment, as a percentage of the desired number of tasks (rounded up to the nearest integer), and while any container instances are in the <code>DRAINING</code> state if the service contains tasks using the EC2 launch type. This parameter enables you to deploy without using additional cluster capacity. For example, if your service has a desired number of four tasks and a minimum healthy percent of 50%, the scheduler may stop two existing tasks to free up cluster capacity before starting two new tasks. Tasks for services that <i>do not</i> use a load balancer are considered healthy if they are in the <code>RUNNING</code> state; tasks for services that <i>do</i> use a load balancer are considered healthy if they are in the <code>RUNNING</code> state and they are reported as healthy by the load balancer. The default value for minimum healthy percent is 100%.</p> <p>If a service is using the blue/green (<code>CODE_DEPLOY</code>) deployment type and tasks that use the EC2 launch type, the <b>minimum healthy percent</b> value is set to the default value and is used to define the lower limit on the number of the tasks in the service that remain in the <code>RUNNING</code> state while the container instances are in the <code>DRAINING</code> state. If the tasks in the service use the Fargate launch type, the minimum healthy percent value is not used, although it is returned when describing your service.</p>
    #[serde(rename = "minimumHealthyPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_healthy_percent: Option<i64>,
}

/// <p>The deployment controller to use for the service. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-types.html">Amazon ECS Deployment Types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeploymentController {
    /// <p><p>The deployment controller type to use.</p> <p>There are two deployment controller types available:</p> <dl> <dt>ECS</dt> <dd> <p>The rolling update (<code>ECS</code>) deployment type involves replacing the current running version of the container with the latest version. The number of containers Amazon ECS adds or removes from the service during a rolling update is controlled by adjusting the minimum and maximum number of healthy tasks allowed during a service deployment, as specified in the <a>DeploymentConfiguration</a>.</p> </dd> <dt>CODE<em>DEPLOY</dt> <dd> <p>The blue/green (<code>CODE</em>DEPLOY</code>) deployment type uses the blue/green deployment model powered by AWS CodeDeploy, which allows you to verify a new deployment of a service before sending production traffic to it. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-types.html">Amazon ECS Deployment Types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </dd> </dl></p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeregisterContainerInstanceRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the container instance to deregister. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>The container instance ID or full ARN of the container instance to deregister. The ARN contains the <code>arn:aws:ecs</code> namespace, followed by the Region of the container instance, the AWS account ID of the container instance owner, the <code>container-instance</code> namespace, and then the container instance ID. For example, <code>arn:aws:ecs:<i>region</i>:<i>aws_account_id</i>:container-instance/<i>container_instance_ID</i> </code>.</p>
    #[serde(rename = "containerInstance")]
    pub container_instance: String,
    /// <p>Forces the deregistration of the container instance. If you have tasks running on the container instance when you deregister it with the <code>force</code> option, these tasks remain running until you terminate the instance or the tasks stop through some other means, but they are orphaned (no longer monitored or accounted for by Amazon ECS). If an orphaned task on your container instance is part of an Amazon ECS service, then the service scheduler starts another copy of that task, on a different container instance if possible. </p> <p>Any containers in orphaned service tasks that are registered with a Classic Load Balancer or an Application Load Balancer target group are deregistered. They begin connection draining according to the settings on the load balancer or target group.</p>
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeregisterContainerInstanceResponse {
    /// <p>The container instance that was deregistered.</p>
    #[serde(rename = "containerInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance: Option<ContainerInstance>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeregisterTaskDefinitionRequest {
    /// <p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or full Amazon Resource Name (ARN) of the task definition to deregister. You must specify a <code>revision</code>.</p>
    #[serde(rename = "taskDefinition")]
    pub task_definition: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeregisterTaskDefinitionResponse {
    /// <p>The full description of the deregistered task.</p>
    #[serde(rename = "taskDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<TaskDefinition>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeClustersRequest {
    /// <p>A list of up to 100 cluster names or full cluster Amazon Resource Name (ARN) entries. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "clusters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<String>>,
    /// <p><p>Additional information about your clusters to be separated by launch type, including:</p> <ul> <li> <p>runningEC2TasksCount</p> </li> <li> <p>runningFargateTasksCount</p> </li> <li> <p>pendingEC2TasksCount</p> </li> <li> <p>pendingFargateTasksCount</p> </li> <li> <p>activeEC2ServiceCount</p> </li> <li> <p>activeFargateServiceCount</p> </li> <li> <p>drainingEC2ServiceCount</p> </li> <li> <p>drainingFargateServiceCount</p> </li> </ul></p>
    #[serde(rename = "include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeClustersResponse {
    /// <p>The list of clusters.</p>
    #[serde(rename = "clusters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<Vec<Cluster>>,
    /// <p>Any failures associated with the call.</p>
    #[serde(rename = "failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeContainerInstancesRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the container instances to describe. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>A list of up to 100 container instance IDs or full Amazon Resource Name (ARN) entries.</p>
    #[serde(rename = "containerInstances")]
    pub container_instances: Vec<String>,
    /// <p>Specifies whether you want to see the resource tags for the container instance. If <code>TAGS</code> is specified, the tags are included in the response. If this field is omitted, tags are not included in the response.</p>
    #[serde(rename = "include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeContainerInstancesResponse {
    /// <p>The list of container instances.</p>
    #[serde(rename = "containerInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instances: Option<Vec<ContainerInstance>>,
    /// <p>Any failures associated with the call.</p>
    #[serde(rename = "failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeServicesRequest {
    /// <p>The short name or full Amazon Resource Name (ARN)the cluster that hosts the service to describe. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>Specifies whether you want to see the resource tags for the service. If <code>TAGS</code> is specified, the tags are included in the response. If this field is omitted, tags are not included in the response.</p>
    #[serde(rename = "include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    /// <p>A list of services to describe. You may specify up to 10 services to describe in a single operation.</p>
    #[serde(rename = "services")]
    pub services: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeServicesResponse {
    /// <p>Any failures associated with the call.</p>
    #[serde(rename = "failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    /// <p>The list of services described.</p>
    #[serde(rename = "services")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<Service>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeTaskDefinitionRequest {
    /// <p>Specifies whether to see the resource tags for the task definition. If <code>TAGS</code> is specified, the tags are included in the response. If this field is omitted, tags are not included in the response.</p>
    #[serde(rename = "include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    /// <p>The <code>family</code> for the latest <code>ACTIVE</code> revision, <code>family</code> and <code>revision</code> (<code>family:revision</code>) for a specific revision in the family, or full Amazon Resource Name (ARN) of the task definition to describe.</p>
    #[serde(rename = "taskDefinition")]
    pub task_definition: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeTaskDefinitionResponse {
    /// <p>The metadata that is applied to the task definition to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The full task definition description.</p>
    #[serde(rename = "taskDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<TaskDefinition>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeTasksRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the task to describe. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>Specifies whether you want to see the resource tags for the task. If <code>TAGS</code> is specified, the tags are included in the response. If this field is omitted, tags are not included in the response.</p>
    #[serde(rename = "include")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    /// <p>A list of up to 100 task IDs or full ARN entries.</p>
    #[serde(rename = "tasks")]
    pub tasks: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeTasksResponse {
    /// <p>Any failures associated with the call.</p>
    #[serde(rename = "failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    /// <p>The list of tasks.</p>
    #[serde(rename = "tasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<Task>>,
}

/// <p>An object representing a container instance host device.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Device {
    /// <p>The path inside the container at which to expose the host device.</p>
    #[serde(rename = "containerPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<String>,
    /// <p>The path for the device on the host container instance.</p>
    #[serde(rename = "hostPath")]
    pub host_path: String,
    /// <p>The explicit permissions to provide to the container for the device. By default, the container has permissions for <code>read</code>, <code>write</code>, and <code>mknod</code> for the device.</p>
    #[serde(rename = "permissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DiscoverPollEndpointRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster to which the container instance belongs.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>The container instance ID or full ARN of the container instance. The ARN contains the <code>arn:aws:ecs</code> namespace, followed by the Region of the container instance, the AWS account ID of the container instance owner, the <code>container-instance</code> namespace, and then the container instance ID. For example, <code>arn:aws:ecs:<i>region</i>:<i>aws_account_id</i>:container-instance/<i>container_instance_ID</i> </code>.</p>
    #[serde(rename = "containerInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DiscoverPollEndpointResponse {
    /// <p>The endpoint for the Amazon ECS agent to poll.</p>
    #[serde(rename = "endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// <p>The telemetry endpoint for the Amazon ECS agent.</p>
    #[serde(rename = "telemetryEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telemetry_endpoint: Option<String>,
}

/// <p>This parameter is specified when you are using Docker volumes. Docker volumes are only supported when you are using the EC2 launch type. Windows containers only support the use of the <code>local</code> driver. To use bind mounts, specify a <code>host</code> instead.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DockerVolumeConfiguration {
    /// <p><p>If this value is <code>true</code>, the Docker volume is created if it does not already exist.</p> <note> <p>This field is only used if the <code>scope</code> is <code>shared</code>.</p> </note></p>
    #[serde(rename = "autoprovision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoprovision: Option<bool>,
    /// <p>The Docker volume driver to use. The driver value must match the driver name provided by Docker because it is used for task placement. If the driver was installed using the Docker plugin CLI, use <code>docker plugin ls</code> to retrieve the driver name from your container instance. If the driver was installed using another method, use Docker plugin discovery to retrieve the driver name. For more information, see <a href="https://docs.docker.com/engine/extend/plugin_api/#plugin-discovery">Docker plugin discovery</a>. This parameter maps to <code>Driver</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/VolumeCreate">Create a volume</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>xxdriver</code> option to <a href="https://docs.docker.com/engine/reference/commandline/volume_create/"> <code>docker volume create</code> </a>.</p>
    #[serde(rename = "driver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver: Option<String>,
    /// <p>A map of Docker driver-specific options passed through. This parameter maps to <code>DriverOpts</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/VolumeCreate">Create a volume</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>xxopt</code> option to <a href="https://docs.docker.com/engine/reference/commandline/volume_create/"> <code>docker volume create</code> </a>.</p>
    #[serde(rename = "driverOpts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub driver_opts: Option<::std::collections::HashMap<String, String>>,
    /// <p>Custom metadata to add to your Docker volume. This parameter maps to <code>Labels</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/VolumeCreate">Create a volume</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>xxlabel</code> option to <a href="https://docs.docker.com/engine/reference/commandline/volume_create/"> <code>docker volume create</code> </a>.</p>
    #[serde(rename = "labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<::std::collections::HashMap<String, String>>,
    /// <p>The scope for the Docker volume that determines its lifecycle. Docker volumes that are scoped to a <code>task</code> are automatically provisioned when the task starts and destroyed when the task stops. Docker volumes that are scoped as <code>shared</code> persist after the task stops.</p>
    #[serde(rename = "scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

/// <p>A failed resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Failure {
    /// <p>The Amazon Resource Name (ARN) of the failed resource.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The reason for the failure.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// <p><p>An object representing a container health check. Health check parameters that are specified in a container definition override any Docker health checks that exist in the container image (such as those specified in a parent image or from the image&#39;s Dockerfile).</p> <p>The following are notes about container health check support:</p> <ul> <li> <p>Container health checks require version 1.17.0 or greater of the Amazon ECS container agent. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-update.html">Updating the Amazon ECS Container Agent</a>.</p> </li> <li> <p>Container health checks are supported for Fargate tasks if you are using platform version 1.1.0 or greater. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html">AWS Fargate Platform Versions</a>.</p> </li> <li> <p>Container health checks are not supported for tasks that are part of a service that is configured to use a Classic Load Balancer.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthCheck {
    /// <p>A string array representing the command that the container runs to determine if it is healthy. The string array must start with <code>CMD</code> to execute the command arguments directly, or <code>CMD-SHELL</code> to run the command with the container's default shell. For example:</p> <p> <code>[ "CMD-SHELL", "curl -f http://localhost/ || exit 1" ]</code> </p> <p>An exit code of 0 indicates success, and non-zero exit code indicates failure. For more information, see <code>HealthCheck</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a>.</p>
    #[serde(rename = "command")]
    pub command: Vec<String>,
    /// <p>The time period in seconds between each health check execution. You may specify between 5 and 300 seconds. The default value is 30 seconds.</p>
    #[serde(rename = "interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// <p>The number of times to retry a failed health check before the container is considered unhealthy. You may specify between 1 and 10 retries. The default value is 3.</p>
    #[serde(rename = "retries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,
    /// <p><p>The optional grace period within which to provide containers time to bootstrap before failed health checks count towards the maximum number of retries. You may specify between 0 and 300 seconds. The <code>startPeriod</code> is disabled by default.</p> <note> <p>If a health check succeeds within the <code>startPeriod</code>, then the container is considered healthy and any subsequent failures count toward the maximum number of retries.</p> </note></p>
    #[serde(rename = "startPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_period: Option<i64>,
    /// <p>The time period in seconds to wait for a health check to succeed before it is considered a failure. You may specify between 2 and 60 seconds. The default value is 5.</p>
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
}

/// <p>Hostnames and IP address entries that are added to the <code>/etc/hosts</code> file of a container via the <code>extraHosts</code> parameter of its <a>ContainerDefinition</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HostEntry {
    /// <p>The hostname to use in the <code>/etc/hosts</code> entry.</p>
    #[serde(rename = "hostname")]
    pub hostname: String,
    /// <p>The IP address to use in the <code>/etc/hosts</code> entry.</p>
    #[serde(rename = "ipAddress")]
    pub ip_address: String,
}

/// <p>Details on a container instance bind mount host volume.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HostVolumeProperties {
    /// <p>When the <code>host</code> parameter is used, specify a <code>sourcePath</code> to declare the path on the host container instance that is presented to the container. If this parameter is empty, then the Docker daemon has assigned a host path for you. If the <code>host</code> parameter contains a <code>sourcePath</code> file location, then the data volume persists at the specified location on the host container instance until you delete it manually. If the <code>sourcePath</code> value does not exist on the host container instance, the Docker daemon creates it. If the location does exist, the contents of the source path folder are exported.</p> <p>If you are using the Fargate launch type, the <code>sourcePath</code> parameter is not supported.</p>
    #[serde(rename = "sourcePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
}

/// <p>The Linux capabilities for the container that are added to or dropped from the default configuration provided by Docker. For more information on the default capabilities and the non-default available capabilities, see <a href="https://docs.docker.com/engine/reference/run/#runtime-privilege-and-linux-capabilities">Runtime privilege and Linux capabilities</a> in the <i>Docker run reference</i>. For more detailed information on these Linux capabilities, see the <a href="http://man7.org/linux/man-pages/man7/capabilities.7.html">capabilities(7)</a> Linux manual page.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KernelCapabilities {
    /// <p>The Linux capabilities for the container that have been added to the default configuration provided by Docker. This parameter maps to <code>CapAdd</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--cap-add</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <note> <p>If you are using tasks that use the Fargate launch type, the <code>add</code> parameter is not supported.</p> </note> <p>Valid values: <code>"ALL" | "AUDIT_CONTROL" | "AUDIT_WRITE" | "BLOCK_SUSPEND" | "CHOWN" | "DAC_OVERRIDE" | "DAC_READ_SEARCH" | "FOWNER" | "FSETID" | "IPC_LOCK" | "IPC_OWNER" | "KILL" | "LEASE" | "LINUX_IMMUTABLE" | "MAC_ADMIN" | "MAC_OVERRIDE" | "MKNOD" | "NET_ADMIN" | "NET_BIND_SERVICE" | "NET_BROADCAST" | "NET_RAW" | "SETFCAP" | "SETGID" | "SETPCAP" | "SETUID" | "SYS_ADMIN" | "SYS_BOOT" | "SYS_CHROOT" | "SYS_MODULE" | "SYS_NICE" | "SYS_PACCT" | "SYS_PTRACE" | "SYS_RAWIO" | "SYS_RESOURCE" | "SYS_TIME" | "SYS_TTY_CONFIG" | "SYSLOG" | "WAKE_ALARM"</code> </p>
    #[serde(rename = "add")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<String>>,
    /// <p>The Linux capabilities for the container that have been removed from the default configuration provided by Docker. This parameter maps to <code>CapDrop</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--cap-drop</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <p>Valid values: <code>"ALL" | "AUDIT_CONTROL" | "AUDIT_WRITE" | "BLOCK_SUSPEND" | "CHOWN" | "DAC_OVERRIDE" | "DAC_READ_SEARCH" | "FOWNER" | "FSETID" | "IPC_LOCK" | "IPC_OWNER" | "KILL" | "LEASE" | "LINUX_IMMUTABLE" | "MAC_ADMIN" | "MAC_OVERRIDE" | "MKNOD" | "NET_ADMIN" | "NET_BIND_SERVICE" | "NET_BROADCAST" | "NET_RAW" | "SETFCAP" | "SETGID" | "SETPCAP" | "SETUID" | "SYS_ADMIN" | "SYS_BOOT" | "SYS_CHROOT" | "SYS_MODULE" | "SYS_NICE" | "SYS_PACCT" | "SYS_PTRACE" | "SYS_RAWIO" | "SYS_RESOURCE" | "SYS_TIME" | "SYS_TTY_CONFIG" | "SYSLOG" | "WAKE_ALARM"</code> </p>
    #[serde(rename = "drop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop: Option<Vec<String>>,
}

/// <p>A key-value pair object.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyValuePair {
    /// <p>The name of the key-value pair. For environment variables, this is the name of the environment variable.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The value of the key-value pair. For environment variables, this is the value of the environment variable.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Linux-specific options that are applied to the container, such as Linux <a>KernelCapabilities</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinuxParameters {
    /// <p><p>The Linux capabilities for the container that are added to or dropped from the default configuration provided by Docker.</p> <note> <p>If you are using tasks that use the Fargate launch type, <code>capabilities</code> is supported but the <code>add</code> parameter is not supported.</p> </note></p>
    #[serde(rename = "capabilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<KernelCapabilities>,
    /// <p><p>Any host devices to expose to the container. This parameter maps to <code>Devices</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--device</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <note> <p>If you are using tasks that use the Fargate launch type, the <code>devices</code> parameter is not supported.</p> </note></p>
    #[serde(rename = "devices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
    /// <p>Run an <code>init</code> process inside the container that forwards signals and reaps processes. This parameter maps to the <code>--init</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>. This parameter requires version 1.25 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: <code>sudo docker version --format '{{.Server.APIVersion}}'</code> </p>
    #[serde(rename = "initProcessEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_process_enabled: Option<bool>,
    /// <p><p>The value for the size (in MiB) of the <code>/dev/shm</code> volume. This parameter maps to the <code>--shm-size</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <note> <p>If you are using tasks that use the Fargate launch type, the <code>sharedMemorySize</code> parameter is not supported.</p> </note></p>
    #[serde(rename = "sharedMemorySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_memory_size: Option<i64>,
    /// <p><p>The container path, mount options, and size (in MiB) of the tmpfs mount. This parameter maps to the <code>--tmpfs</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <note> <p>If you are using tasks that use the Fargate launch type, the <code>tmpfs</code> parameter is not supported.</p> </note></p>
    #[serde(rename = "tmpfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<Vec<Tmpfs>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAccountSettingsRequest {
    /// <p>Specifies whether to return the effective settings. If <code>true</code>, the account settings for the root user or the default setting for the <code>principalArn</code>. If <code>false</code>, the account settings for the <code>principalArn</code> are returned if they are set. Otherwise, no account settings are returned.</p>
    #[serde(rename = "effectiveSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_settings: Option<bool>,
    /// <p>The maximum number of account setting results returned by <code>ListAccountSettings</code> in paginated output. When this parameter is used, <code>ListAccountSettings</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListAccountSettings</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 10. If this parameter is not used, then <code>ListAccountSettings</code> returns up to 10 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The resource name you want to list the account settings for.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListAccountSettings</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARN of the principal, which can be an IAM user, IAM role, or the root user. If this field is omitted, the account settings are listed only for the authenticated user.</p>
    #[serde(rename = "principalArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_arn: Option<String>,
    /// <p>The value of the account settings with which to filter results. You must also specify an account setting name to use this parameter.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListAccountSettingsResponse {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListAccountSettings</code> request. When the results of a <code>ListAccountSettings</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The account settings for the resource.</p>
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<Setting>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAttributesRequest {
    /// <p>The name of the attribute with which to filter the results. </p>
    #[serde(rename = "attributeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    /// <p>The value of the attribute with which to filter results. You must also specify an attribute name to use this parameter.</p>
    #[serde(rename = "attributeValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value: Option<String>,
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster to list attributes. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>The maximum number of cluster results returned by <code>ListAttributes</code> in paginated output. When this parameter is used, <code>ListAttributes</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListAttributes</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListAttributes</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListAttributes</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The type of the target with which to list attributes.</p>
    #[serde(rename = "targetType")]
    pub target_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListAttributesResponse {
    /// <p>A list of attribute objects that meet the criteria of the request.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>ListAttributes</code> request. When the results of a <code>ListAttributes</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListClustersRequest {
    /// <p>The maximum number of cluster results returned by <code>ListClusters</code> in paginated output. When this parameter is used, <code>ListClusters</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListClusters</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListClusters</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListClusters</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListClustersResponse {
    /// <p>The list of full Amazon Resource Name (ARN) entries for each cluster associated with your account.</p>
    #[serde(rename = "clusterArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arns: Option<Vec<String>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>ListClusters</code> request. When the results of a <code>ListClusters</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListContainerInstancesRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the container instances to list. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>You can filter the results of a <code>ListContainerInstances</code> operation with cluster query language statements. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html">Cluster Query Language</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// <p>The maximum number of container instance results returned by <code>ListContainerInstances</code> in paginated output. When this parameter is used, <code>ListContainerInstances</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListContainerInstances</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListContainerInstances</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListContainerInstances</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Filters the container instances by status. For example, if you specify the <code>DRAINING</code> status, the results include only container instances that have been set to <code>DRAINING</code> using <a>UpdateContainerInstancesState</a>. If you do not specify this parameter, the default is to include container instances set to <code>ACTIVE</code> and <code>DRAINING</code>.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListContainerInstancesResponse {
    /// <p>The list of container instances with full ARN entries for each container instance associated with the specified cluster.</p>
    #[serde(rename = "containerInstanceArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_arns: Option<Vec<String>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>ListContainerInstances</code> request. When the results of a <code>ListContainerInstances</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListServicesRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the services to list. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>The launch type for the services to list.</p>
    #[serde(rename = "launchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    /// <p>The maximum number of service results returned by <code>ListServices</code> in paginated output. When this parameter is used, <code>ListServices</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListServices</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListServices</code> returns up to 10 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListServices</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The scheduling strategy for services to list.</p>
    #[serde(rename = "schedulingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_strategy: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListServicesResponse {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListServices</code> request. When the results of a <code>ListServices</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of full ARN entries for each service associated with the specified cluster.</p>
    #[serde(rename = "serviceArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arns: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) that identifies the resource for which to list the tags. Currently, the supported resources are Amazon ECS tasks, services, task definitions, clusters, and container instances.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tags for the resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTaskDefinitionFamiliesRequest {
    /// <p>The <code>familyPrefix</code> is a string that is used to filter the results of <code>ListTaskDefinitionFamilies</code>. If you specify a <code>familyPrefix</code>, only task definition family names that begin with the <code>familyPrefix</code> string are returned.</p>
    #[serde(rename = "familyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_prefix: Option<String>,
    /// <p>The maximum number of task definition family results returned by <code>ListTaskDefinitionFamilies</code> in paginated output. When this parameter is used, <code>ListTaskDefinitions</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListTaskDefinitionFamilies</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListTaskDefinitionFamilies</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListTaskDefinitionFamilies</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The task definition family status with which to filter the <code>ListTaskDefinitionFamilies</code> results. By default, both <code>ACTIVE</code> and <code>INACTIVE</code> task definition families are listed. If this parameter is set to <code>ACTIVE</code>, only task definition families that have an <code>ACTIVE</code> task definition revision are returned. If this parameter is set to <code>INACTIVE</code>, only task definition families that do not have any <code>ACTIVE</code> task definition revisions are returned. If you paginate the resulting output, be sure to keep the <code>status</code> value constant in each subsequent request.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListTaskDefinitionFamiliesResponse {
    /// <p>The list of task definition family names that match the <code>ListTaskDefinitionFamilies</code> request.</p>
    #[serde(rename = "families")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub families: Option<Vec<String>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>ListTaskDefinitionFamilies</code> request. When the results of a <code>ListTaskDefinitionFamilies</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTaskDefinitionsRequest {
    /// <p>The full family name with which to filter the <code>ListTaskDefinitions</code> results. Specifying a <code>familyPrefix</code> limits the listed task definitions to task definition revisions that belong to that family.</p>
    #[serde(rename = "familyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_prefix: Option<String>,
    /// <p>The maximum number of task definition results returned by <code>ListTaskDefinitions</code> in paginated output. When this parameter is used, <code>ListTaskDefinitions</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListTaskDefinitions</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListTaskDefinitions</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListTaskDefinitions</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The order in which to sort the results. Valid values are <code>ASC</code> and <code>DESC</code>. By default (<code>ASC</code>), task definitions are listed lexicographically by family name and in ascending numerical order by revision so that the newest task definitions in a family are listed last. Setting this parameter to <code>DESC</code> reverses the sort order on family name and revision so that the newest task definitions in a family are listed first.</p>
    #[serde(rename = "sort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// <p>The task definition status with which to filter the <code>ListTaskDefinitions</code> results. By default, only <code>ACTIVE</code> task definitions are listed. By setting this parameter to <code>INACTIVE</code>, you can view task definitions that are <code>INACTIVE</code> as long as an active task or service still references them. If you paginate the resulting output, be sure to keep the <code>status</code> value constant in each subsequent request.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListTaskDefinitionsResponse {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListTaskDefinitions</code> request. When the results of a <code>ListTaskDefinitions</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of task definition Amazon Resource Name (ARN) entries for the <code>ListTaskDefinitions</code> request.</p>
    #[serde(rename = "taskDefinitionArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition_arns: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTasksRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the tasks to list. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>The container instance ID or full ARN of the container instance with which to filter the <code>ListTasks</code> results. Specifying a <code>containerInstance</code> limits the results to tasks that belong to that container instance.</p>
    #[serde(rename = "containerInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance: Option<String>,
    /// <p><p>The task desired status with which to filter the <code>ListTasks</code> results. Specifying a <code>desiredStatus</code> of <code>STOPPED</code> limits the results to tasks that Amazon ECS has set the desired status to <code>STOPPED</code>. This can be useful for debugging tasks that are not starting properly or have died or finished. The default status filter is <code>RUNNING</code>, which shows tasks that Amazon ECS has set the desired status to <code>RUNNING</code>.</p> <note> <p>Although you can filter results based on a desired status of <code>PENDING</code>, this does not return any results. Amazon ECS never sets the desired status of a task to that value (only a task&#39;s <code>lastStatus</code> may have a value of <code>PENDING</code>).</p> </note></p>
    #[serde(rename = "desiredStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_status: Option<String>,
    /// <p>The name of the family with which to filter the <code>ListTasks</code> results. Specifying a <code>family</code> limits the results to tasks that belong to that family.</p>
    #[serde(rename = "family")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// <p>The launch type for services to list.</p>
    #[serde(rename = "launchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    /// <p>The maximum number of task results returned by <code>ListTasks</code> in paginated output. When this parameter is used, <code>ListTasks</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListTasks</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter is not used, then <code>ListTasks</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p><p>The <code>nextToken</code> value returned from a previous paginated <code>ListTasks</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value.</p> <note> <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p> </note></p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The name of the service with which to filter the <code>ListTasks</code> results. Specifying a <code>serviceName</code> limits the results to tasks that belong to that service.</p>
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// <p>The <code>startedBy</code> value with which to filter the task results. Specifying a <code>startedBy</code> value limits the results to tasks that were started with that value.</p>
    #[serde(rename = "startedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_by: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListTasksResponse {
    /// <p>The <code>nextToken</code> value to include in a future <code>ListTasks</code> request. When the results of a <code>ListTasks</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of task ARN entries for the <code>ListTasks</code> request.</p>
    #[serde(rename = "taskArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arns: Option<Vec<String>>,
}

/// <p>Details on a load balancer that is used with a service.</p> <p>If the service is using the <code>ECS</code> deployment controller, you are limited to one load balancer or target group.</p> <p>If the service is using the <code>CODE_DEPLOY</code> deployment controller, the service is required to use either an Application Load Balancer or Network Load Balancer. When you are creating an AWS CodeDeploy deployment group, you specify two target groups (referred to as a <code>targetGroupPair</code>). Each target group binds to a separate task set in the deployment. The load balancer can also have up to two listeners, a required listener for production traffic and an optional listener that allows you to test new revisions of the service before routing production traffic to it.</p> <p>Services with tasks that use the <code>awsvpc</code> network mode (for example, those with the Fargate launch type) only support Application Load Balancers and Network Load Balancers. Classic Load Balancers are not supported. Also, when you create any target groups for these services, you must choose <code>ip</code> as the target type, not <code>instance</code>. Tasks that use the <code>awsvpc</code> network mode are associated with an elastic network interface, not an Amazon EC2 instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadBalancer {
    /// <p>The name of the container (as it appears in a container definition) to associate with the load balancer.</p>
    #[serde(rename = "containerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    /// <p>The port on the container to associate with the load balancer. This port must correspond to a <code>containerPort</code> in the service's task definition. Your container instances must allow ingress traffic on the <code>hostPort</code> of the port mapping.</p>
    #[serde(rename = "containerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i64>,
    /// <p>The name of a load balancer.</p>
    #[serde(rename = "loadBalancerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancer_name: Option<String>,
    /// <p><p>The full Amazon Resource Name (ARN) of the Elastic Load Balancing target group or groups associated with a service. For services using the <code>ECS</code> deployment controller, you are limited to one target group. For services using the <code>CODE_DEPLOY</code> deployment controller, you are required to define two target groups for the load balancer.</p> <important> <p>If your service&#39;s task definition uses the <code>awsvpc</code> network mode (which is required for the Fargate launch type), you must choose <code>ip</code> as the target type, not <code>instance</code>, because tasks that use the <code>awsvpc</code> network mode are associated with an elastic network interface, not an Amazon EC2 instance.</p> </important></p>
    #[serde(rename = "targetGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_group_arn: Option<String>,
}

/// <p>Log configuration options to send to a custom log driver for the container.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LogConfiguration {
    /// <p>The log driver to use for the container. The valid values listed for this parameter are log drivers that the Amazon ECS container agent can communicate with by default. If you are using the Fargate launch type, the only supported value is <code>awslogs</code>. For more information about using the <code>awslogs</code> driver, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/using_awslogs.html">Using the awslogs Log Driver</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <note> <p>If you have a custom driver that is not listed above that you would like to work with the Amazon ECS container agent, you can fork the Amazon ECS container agent project that is <a href="https://github.com/aws/amazon-ecs-agent">available on GitHub</a> and customize it to work with that driver. We encourage you to submit pull requests for changes that you would like to have included. However, Amazon Web Services does not currently support running modified copies of this software.</p> </note> <p>This parameter requires version 1.18 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: <code>sudo docker version --format '{{.Server.APIVersion}}'</code> </p>
    #[serde(rename = "logDriver")]
    pub log_driver: String,
    /// <p>The configuration options to send to the log driver. This parameter requires version 1.19 of the Docker Remote API or greater on your container instance. To check the Docker Remote API version on your container instance, log in to your container instance and run the following command: <code>sudo docker version --format '{{.Server.APIVersion}}'</code> </p>
    #[serde(rename = "options")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Details on a volume mount point that is used in a container definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MountPoint {
    /// <p>The path on the container to mount the host volume at.</p>
    #[serde(rename = "containerPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<String>,
    /// <p>If this value is <code>true</code>, the container has read-only access to the volume. If this value is <code>false</code>, then the container can write to the volume. The default value is <code>false</code>.</p>
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// <p>The name of the volume to mount. Must be a volume name referenced in the <code>name</code> parameter of task definition <code>volume</code>.</p>
    #[serde(rename = "sourceVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_volume: Option<String>,
}

/// <p>Details on the network bindings between a container and its host container instance. After a task reaches the <code>RUNNING</code> status, manual and automatic host and container port assignments are visible in the <code>networkBindings</code> section of <a>DescribeTasks</a> API responses.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkBinding {
    /// <p>The IP address that the container is bound to on the container instance.</p>
    #[serde(rename = "bindIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_ip: Option<String>,
    /// <p>The port number on the container that is used with the network binding.</p>
    #[serde(rename = "containerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i64>,
    /// <p>The port number on the host that is used with the network binding.</p>
    #[serde(rename = "hostPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_port: Option<i64>,
    /// <p>The protocol used for the network binding.</p>
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

/// <p>An object representing the network configuration for a task or service.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkConfiguration {
    /// <p><p>The VPC subnets and security groups associated with a task.</p> <note> <p>All specified subnets and security groups must be from the same VPC.</p> </note></p>
    #[serde(rename = "awsvpcConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awsvpc_configuration: Option<AwsVpcConfiguration>,
}

/// <p>An object representing the elastic network interface for tasks that use the <code>awsvpc</code> network mode.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct NetworkInterface {
    /// <p>The attachment ID for the network interface.</p>
    #[serde(rename = "attachmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    /// <p>The private IPv6 address for the network interface.</p>
    #[serde(rename = "ipv6Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_6_address: Option<String>,
    /// <p>The private IPv4 address for the network interface.</p>
    #[serde(rename = "privateIpv4Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ipv_4_address: Option<String>,
}

/// <p>An object representing a constraint on task placement. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-placement-constraints.html">Task Placement Constraints</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlacementConstraint {
    /// <p>A cluster query language expression to apply to the constraint. You cannot specify an expression if the constraint type is <code>distinctInstance</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html">Cluster Query Language</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "expression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// <p>The type of constraint. Use <code>distinctInstance</code> to ensure that each task in a particular group is running on a different container instance. Use <code>memberOf</code> to restrict the selection to a group of valid candidates. The value <code>distinctInstance</code> is not supported in task definitions.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The task placement strategy for a task or service. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-placement-strategies.html">Task Placement Strategies</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlacementStrategy {
    /// <p>The field to apply the placement strategy against. For the <code>spread</code> placement strategy, valid values are <code>instanceId</code> (or <code>host</code>, which has the same effect), or any platform or custom attribute that is applied to a container instance, such as <code>attribute:ecs.availability-zone</code>. For the <code>binpack</code> placement strategy, valid values are <code>cpu</code> and <code>memory</code>. For the <code>random</code> placement strategy, this field is not used.</p>
    #[serde(rename = "field")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// <p>The type of placement strategy. The <code>random</code> placement strategy randomly places tasks on available candidates. The <code>spread</code> placement strategy spreads placement across available candidates evenly based on the <code>field</code> parameter. The <code>binpack</code> strategy places tasks on available candidates that have the least available amount of the resource that is specified with the <code>field</code> parameter. For example, if you binpack on memory, a task is placed on the instance with the least amount of remaining memory (but still enough to run the task).</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The devices that are available on the container instance. The only supported device type is a GPU.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PlatformDevice {
    /// <p>The ID for the GPU(s) on the container instance. The available GPU IDs can also be obtained on the container instance in the <code>/var/lib/ecs/gpu/nvidia_gpu_info.json</code> file.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The type of device that is available on the container instance. The only supported value is <code>GPU</code>.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Port mappings allow containers to access ports on the host container instance to send or receive traffic. Port mappings are specified as part of the container definition.</p> <p>If you are using containers in a task with the <code>awsvpc</code> or <code>host</code> network mode, exposed ports should be specified using <code>containerPort</code>. The <code>hostPort</code> can be left blank or it must be the same value as the <code>containerPort</code>.</p> <p>After a task reaches the <code>RUNNING</code> status, manual and automatic host and container port assignments are visible in the <code>networkBindings</code> section of <a>DescribeTasks</a> API responses.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PortMapping {
    /// <p>The port number on the container that is bound to the user-specified or automatically assigned host port.</p> <p>If you are using containers in a task with the <code>awsvpc</code> or <code>host</code> network mode, exposed ports should be specified using <code>containerPort</code>.</p> <p>If you are using containers in a task with the <code>bridge</code> network mode and you specify a container port and not a host port, your container automatically receives a host port in the ephemeral port range. For more information, see <code>hostPort</code>. Port mappings that are automatically assigned in this way do not count toward the 100 reserved ports limit of a container instance.</p>
    #[serde(rename = "containerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i64>,
    /// <p>The port number on the container instance to reserve for your container.</p> <p>If you are using containers in a task with the <code>awsvpc</code> or <code>host</code> network mode, the <code>hostPort</code> can either be left blank or set to the same value as the <code>containerPort</code>.</p> <p>If you are using containers in a task with the <code>bridge</code> network mode, you can specify a non-reserved host port for your container port mapping, or you can omit the <code>hostPort</code> (or set it to <code>0</code>) while specifying a <code>containerPort</code> and your container automatically receives a port in the ephemeral port range for your container instance operating system and Docker version.</p> <p>The default ephemeral port range for Docker version 1.6.0 and later is listed on the instance under <code>/proc/sys/net/ipv4/ip_local_port_range</code>. If this kernel parameter is unavailable, the default ephemeral port range from 49153 through 65535 is used. Do not attempt to specify a host port in the ephemeral port range as these are reserved for automatic assignment. In general, ports below 32768 are outside of the ephemeral port range.</p> <note> <p>The default ephemeral port range from 49153 through 65535 is always used for Docker versions before 1.6.0.</p> </note> <p>The default reserved ports are 22 for SSH, the Docker ports 2375 and 2376, and the Amazon ECS container agent ports 51678-51680. Any host port that was previously specified in a running task is also reserved while the task is running (after a task stops, the host port is released). The current reserved ports are displayed in the <code>remainingResources</code> of <a>DescribeContainerInstances</a> output. A container instance can have up to 100 reserved ports at a time, including the default reserved ports. Automatically assigned ports don't count toward the 100 reserved ports limit.</p>
    #[serde(rename = "hostPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_port: Option<i64>,
    /// <p>The protocol used for the port mapping. Valid values are <code>tcp</code> and <code>udp</code>. The default is <code>tcp</code>.</p>
    #[serde(rename = "protocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutAccountSettingDefaultRequest {
    /// <p>The resource type to enable the new format for. If <code>serviceLongArnFormat</code> is specified, the ARN for your Amazon ECS services is affected. If <code>taskLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS tasks are affected. If <code>containerInstanceLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS container instances are affected.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The account setting value for the specified principal ARN. Accepted values are <code>enabled</code> and <code>disabled</code>.</p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutAccountSettingDefaultResponse {
    #[serde(rename = "setting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting: Option<Setting>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutAccountSettingRequest {
    /// <p>The resource name for which to enable the new format. If <code>serviceLongArnFormat</code> is specified, the ARN for your Amazon ECS services is affected. If <code>taskLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS tasks is affected. If <code>containerInstanceLongArnFormat</code> is specified, the ARN and resource ID for your Amazon ECS container instances is affected.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The ARN of the principal, which can be an IAM user, IAM role, or the root user. If you specify the root user, it modifies the ARN and resource ID format for all IAM users, IAM roles, and the root user of the account unless an IAM user or role explicitly overrides these settings for themselves. If this field is omitted, the settings are changed only for the authenticated user.</p>
    #[serde(rename = "principalArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_arn: Option<String>,
    /// <p>The account setting value for the specified principal ARN. Accepted values are <code>enabled</code> and <code>disabled</code>.</p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutAccountSettingResponse {
    /// <p>The current account setting for a resource.</p>
    #[serde(rename = "setting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting: Option<Setting>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutAttributesRequest {
    /// <p>The attributes to apply to your resource. You can specify up to 10 custom attributes per resource. You can specify up to 10 attributes in a single call.</p>
    #[serde(rename = "attributes")]
    pub attributes: Vec<Attribute>,
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that contains the resource to apply attributes. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutAttributesResponse {
    /// <p>The attributes applied to your resource.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterContainerInstanceRequest {
    /// <p>The container instance attributes that this container instance supports.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster with which to register your container instance. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>The ARN of the container instance (if it was previously registered).</p>
    #[serde(rename = "containerInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_arn: Option<String>,
    /// <p>The instance identity document for the EC2 instance to register. This document can be found by running the following command from the instance: <code>curl http://169.254.169.254/latest/dynamic/instance-identity/document/</code> </p>
    #[serde(rename = "instanceIdentityDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identity_document: Option<String>,
    /// <p>The instance identity document signature for the EC2 instance to register. This signature can be found by running the following command from the instance: <code>curl http://169.254.169.254/latest/dynamic/instance-identity/signature/</code> </p>
    #[serde(rename = "instanceIdentityDocumentSignature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_identity_document_signature: Option<String>,
    /// <p>The devices that are available on the container instance. The only supported device type is a GPU.</p>
    #[serde(rename = "platformDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_devices: Option<Vec<PlatformDevice>>,
    /// <p>The metadata that you apply to the container instance to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The resources available on the instance.</p>
    #[serde(rename = "totalResources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_resources: Option<Vec<Resource>>,
    /// <p>The version information for the Amazon ECS container agent and Docker daemon running on the container instance.</p>
    #[serde(rename = "versionInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_info: Option<VersionInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RegisterContainerInstanceResponse {
    /// <p>The container instance that was registered.</p>
    #[serde(rename = "containerInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance: Option<ContainerInstance>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterTaskDefinitionRequest {
    /// <p>A list of container definitions in JSON format that describe the different containers that make up your task.</p>
    #[serde(rename = "containerDefinitions")]
    pub container_definitions: Vec<ContainerDefinition>,
    /// <p><p>The number of CPU units used by the task. It can be expressed as an integer using CPU units, for example <code>1024</code>, or as a string using vCPUs, for example <code>1 vCPU</code> or <code>1 vcpu</code>, in a task definition. String values are converted to an integer indicating the CPU units when the task definition is registered.</p> <note> <p>Task-level CPU and memory parameters are ignored for Windows containers. We recommend specifying container-level resources for Windows containers.</p> </note> <p>If you are using the EC2 launch type, this field is optional. Supported values are between <code>128</code> CPU units (<code>0.125</code> vCPUs) and <code>10240</code> CPU units (<code>10</code> vCPUs).</p> <p>If you are using the Fargate launch type, this field is required and you must use one of the following values, which determines your range of supported values for the <code>memory</code> parameter:</p> <ul> <li> <p>256 (.25 vCPU) - Available <code>memory</code> values: 512 (0.5 GB), 1024 (1 GB), 2048 (2 GB)</p> </li> <li> <p>512 (.5 vCPU) - Available <code>memory</code> values: 1024 (1 GB), 2048 (2 GB), 3072 (3 GB), 4096 (4 GB)</p> </li> <li> <p>1024 (1 vCPU) - Available <code>memory</code> values: 2048 (2 GB), 3072 (3 GB), 4096 (4 GB), 5120 (5 GB), 6144 (6 GB), 7168 (7 GB), 8192 (8 GB)</p> </li> <li> <p>2048 (2 vCPU) - Available <code>memory</code> values: Between 4096 (4 GB) and 16384 (16 GB) in increments of 1024 (1 GB)</p> </li> <li> <p>4096 (4 vCPU) - Available <code>memory</code> values: Between 8192 (8 GB) and 30720 (30 GB) in increments of 1024 (1 GB)</p> </li> </ul></p>
    #[serde(rename = "cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the task execution role that the Amazon ECS container agent and the Docker daemon can assume.</p>
    #[serde(rename = "executionRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    /// <p>You must specify a <code>family</code> for a task definition, which allows you to track multiple versions of the same task definition. The <code>family</code> is used as a name for your task definition. Up to 255 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed.</p>
    #[serde(rename = "family")]
    pub family: String,
    /// <p><p>The IPC resource namespace to use for the containers in the task. The valid values are <code>host</code>, <code>task</code>, or <code>none</code>. If <code>host</code> is specified, then all containers within the tasks that specified the <code>host</code> IPC mode on the same container instance share the same IPC resources with the host Amazon EC2 instance. If <code>task</code> is specified, all containers within the specified task share the same IPC resources. If <code>none</code> is specified, then IPC resources within the containers of a task are private and not shared with other containers in a task or on the container instance. If no value is specified, then the IPC resource namespace sharing depends on the Docker daemon setting on the container instance. For more information, see <a href="https://docs.docker.com/engine/reference/run/#ipc-settings---ipc">IPC settings</a> in the <i>Docker run reference</i>.</p> <p>If the <code>host</code> IPC mode is used, be aware that there is a heightened risk of undesired IPC namespace expose. For more information, see <a href="https://docs.docker.com/engine/security/security/">Docker security</a>.</p> <p>If you are setting namespaced kernel parameters using <code>systemControls</code> for the containers in the task, the following will apply to your IPC resource namespace. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_definition_parameters.html">System Controls</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <ul> <li> <p>For tasks that use the <code>host</code> IPC mode, IPC namespace related <code>systemControls</code> are not supported.</p> </li> <li> <p>For tasks that use the <code>task</code> IPC mode, IPC namespace related <code>systemControls</code> will apply to all containers within a task.</p> </li> </ul> <note> <p>This parameter is not supported for Windows containers or tasks using the Fargate launch type.</p> </note></p>
    #[serde(rename = "ipcMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipc_mode: Option<String>,
    /// <p><p>The amount of memory (in MiB) used by the task. It can be expressed as an integer using MiB, for example <code>1024</code>, or as a string using GB, for example <code>1GB</code> or <code>1 GB</code>, in a task definition. String values are converted to an integer indicating the MiB when the task definition is registered.</p> <note> <p>Task-level CPU and memory parameters are ignored for Windows containers. We recommend specifying container-level resources for Windows containers.</p> </note> <p>If using the EC2 launch type, this field is optional.</p> <p>If using the Fargate launch type, this field is required and you must use one of the following values, which determines your range of supported values for the <code>cpu</code> parameter:</p> <ul> <li> <p>512 (0.5 GB), 1024 (1 GB), 2048 (2 GB) - Available <code>cpu</code> values: 256 (.25 vCPU)</p> </li> <li> <p>1024 (1 GB), 2048 (2 GB), 3072 (3 GB), 4096 (4 GB) - Available <code>cpu</code> values: 512 (.5 vCPU)</p> </li> <li> <p>2048 (2 GB), 3072 (3 GB), 4096 (4 GB), 5120 (5 GB), 6144 (6 GB), 7168 (7 GB), 8192 (8 GB) - Available <code>cpu</code> values: 1024 (1 vCPU)</p> </li> <li> <p>Between 4096 (4 GB) and 16384 (16 GB) in increments of 1024 (1 GB) - Available <code>cpu</code> values: 2048 (2 vCPU)</p> </li> <li> <p>Between 8192 (8 GB) and 30720 (30 GB) in increments of 1024 (1 GB) - Available <code>cpu</code> values: 4096 (4 vCPU)</p> </li> </ul></p>
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    /// <p>The Docker networking mode to use for the containers in the task. The valid values are <code>none</code>, <code>bridge</code>, <code>awsvpc</code>, and <code>host</code>. The default Docker network mode is <code>bridge</code>. If you are using the Fargate launch type, the <code>awsvpc</code> network mode is required. If you are using the EC2 launch type, any network mode can be used. If the network mode is set to <code>none</code>, you cannot specify port mappings in your container definitions, and the tasks containers do not have external connectivity. The <code>host</code> and <code>awsvpc</code> network modes offer the highest networking performance for containers because they use the EC2 network stack instead of the virtualized network stack provided by the <code>bridge</code> mode.</p> <p>With the <code>host</code> and <code>awsvpc</code> network modes, exposed container ports are mapped directly to the corresponding host port (for the <code>host</code> network mode) or the attached elastic network interface port (for the <code>awsvpc</code> network mode), so you cannot take advantage of dynamic host port mappings. </p> <p>If the network mode is <code>awsvpc</code>, the task is allocated an elastic network interface, and you must specify a <a>NetworkConfiguration</a> value when you create a service or run a task with the task definition. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-networking.html">Task Networking</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <note> <p>Currently, only Amazon ECS-optimized AMIs, other Amazon Linux variants with the <code>ecs-init</code> package, or AWS Fargate infrastructure support the <code>awsvpc</code> network mode. </p> </note> <p>If the network mode is <code>host</code>, you cannot run multiple instantiations of the same task on a single container instance when port mappings are used.</p> <p>Docker for Windows uses different network modes than Docker for Linux. When you register a task definition with Windows containers, you must not specify a network mode. If you use the console to register a task definition with Windows containers, you must choose the <code>&lt;default&gt;</code> network mode object. </p> <p>For more information, see <a href="https://docs.docker.com/engine/reference/run/#network-settings">Network settings</a> in the <i>Docker run reference</i>.</p>
    #[serde(rename = "networkMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
    /// <p><p>The process namespace to use for the containers in the task. The valid values are <code>host</code> or <code>task</code>. If <code>host</code> is specified, then all containers within the tasks that specified the <code>host</code> PID mode on the same container instance share the same IPC resources with the host Amazon EC2 instance. If <code>task</code> is specified, all containers within the specified task share the same process namespace. If no value is specified, the default is a private namespace. For more information, see <a href="https://docs.docker.com/engine/reference/run/#pid-settings---pid">PID settings</a> in the <i>Docker run reference</i>.</p> <p>If the <code>host</code> PID mode is used, be aware that there is a heightened risk of undesired process namespace expose. For more information, see <a href="https://docs.docker.com/engine/security/security/">Docker security</a>.</p> <note> <p>This parameter is not supported for Windows containers or tasks using the Fargate launch type.</p> </note></p>
    #[serde(rename = "pidMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid_mode: Option<String>,
    /// <p>An array of placement constraint objects to use for the task. You can specify a maximum of 10 constraints per task (this limit includes constraints in the task definition and those specified at runtime).</p>
    #[serde(rename = "placementConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<TaskDefinitionPlacementConstraint>>,
    /// <p>The launch type required by the task. If no value is specified, it defaults to <code>EC2</code>.</p>
    #[serde(rename = "requiresCompatibilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_compatibilities: Option<Vec<String>>,
    /// <p>The metadata that you apply to the task definition to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The short name or full Amazon Resource Name (ARN) of the IAM role that containers in this task can assume. All containers in this task are granted the permissions that are specified in this role. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-iam-roles.html">IAM Roles for Tasks</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "taskRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<String>,
    /// <p>A list of volume definitions in JSON format that containers in your task may use.</p>
    #[serde(rename = "volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RegisterTaskDefinitionResponse {
    /// <p>The list of tags associated with the task definition.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The full description of the registered task definition.</p>
    #[serde(rename = "taskDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<TaskDefinition>,
}

/// <p>The repository credentials for private registry authentication.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RepositoryCredentials {
    /// <p><p>The Amazon Resource Name (ARN) of the secret containing the private repository credentials.</p> <note> <p>When you are using the Amazon ECS API, AWS CLI, or AWS SDK, if the secret exists in the same Region as the task that you are launching then you can use either the full ARN or the name of the secret. When you are using the AWS Management Console, you must specify the full ARN of the secret.</p> </note></p>
    #[serde(rename = "credentialsParameter")]
    pub credentials_parameter: String,
}

/// <p>Describes the resources available for a container instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    /// <p>When the <code>doubleValue</code> type is set, the value of the resource must be a double precision floating-point type.</p>
    #[serde(rename = "doubleValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_value: Option<f64>,
    /// <p>When the <code>integerValue</code> type is set, the value of the resource must be an integer.</p>
    #[serde(rename = "integerValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integer_value: Option<i64>,
    /// <p>When the <code>longValue</code> type is set, the value of the resource must be an extended precision floating-point type.</p>
    #[serde(rename = "longValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub long_value: Option<i64>,
    /// <p>The name of the resource, such as <code>CPU</code>, <code>MEMORY</code>, <code>PORTS</code>, <code>PORTS_UDP</code>, or a user-defined resource.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>When the <code>stringSetValue</code> type is set, the value of the resource must be a string type.</p>
    #[serde(rename = "stringSetValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_set_value: Option<Vec<String>>,
    /// <p>The type of the resource, such as <code>INTEGER</code>, <code>DOUBLE</code>, <code>LONG</code>, or <code>STRINGSET</code>.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The type and amount of a resource to assign to a container. The only supported resource is a GPU. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-gpu.html">Working with GPUs on Amazon ECS</a> in the <i>Amazon Elastic Container Service Developer Guide</i> </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceRequirement {
    /// <p>The type of resource to assign to a container. The only supported value is <code>GPU</code>.</p>
    #[serde(rename = "type")]
    pub type_: String,
    /// <p>The number of physical <code>GPUs</code> the Amazon ECS container agent will reserve for the container. The number of GPUs reserved for all containers in a task should not exceed the number of available GPUs on the container instance the task is launched on.</p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RunTaskRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster on which to run your task. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>The number of instantiations of the specified task to place on your cluster. You can specify up to 10 tasks per call.</p>
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>Specifies whether to enable Amazon ECS managed tags for the task. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-using-tags.html">Tagging Your Amazon ECS Resources</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "enableECSManagedTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ecs_managed_tags: Option<bool>,
    /// <p>The name of the task group to associate with the task. The default value is the family name of the task definition (for example, family:my-family-name).</p>
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// <p>The launch type on which to run your task. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_types.html">Amazon ECS Launch Types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "launchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    /// <p>The network configuration for the task. This parameter is required for task definitions that use the <code>awsvpc</code> network mode to receive their own elastic network interface, and it is not supported for other network modes. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-networking.html">Task Networking</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "networkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// <p><p>A list of container overrides in JSON format that specify the name of a container in the specified task definition and the overrides it should receive. You can override the default command for a container (that is specified in the task definition or Docker image) with a <code>command</code> override. You can also override existing environment variables (that are specified in the task definition or Docker image) on a container or add new environment variables to it with an <code>environment</code> override.</p> <note> <p>A total of 8192 characters are allowed for overrides. This limit includes the JSON formatting characters of the override structure.</p> </note></p>
    #[serde(rename = "overrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<TaskOverride>,
    /// <p>An array of placement constraint objects to use for the task. You can specify up to 10 constraints per task (including constraints in the task definition and those specified at runtime).</p>
    #[serde(rename = "placementConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,
    /// <p>The placement strategy objects to use for the task. You can specify a maximum of five strategy rules per task.</p>
    #[serde(rename = "placementStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_strategy: Option<Vec<PlacementStrategy>>,
    /// <p>The platform version the task should run. A platform version is only specified for tasks using the Fargate launch type. If one is not specified, the <code>LATEST</code> platform version is used by default. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html">AWS Fargate Platform Versions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "platformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p><p>Specifies whether to propagate the tags from the task definition to the task. If no value is specified, the tags are not propagated. Tags can only be propagated to the task during task creation. To add tags to a task after task creation, use the <a>TagResource</a> API action.</p> <note> <p>An error will be received if you specify the <code>SERVICE</code> option when running a task.</p> </note></p>
    #[serde(rename = "propagateTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
    /// <p>An optional tag specified when a task is started. For example, if you automatically trigger a task to run a batch process job, you could apply a unique identifier for that job to your task with the <code>startedBy</code> parameter. You can then identify which tasks belong to that job by filtering the results of a <a>ListTasks</a> call with the <code>startedBy</code> value. Up to 36 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed.</p> <p>If a task is started by an Amazon ECS service, then the <code>startedBy</code> parameter contains the deployment ID of the service that starts it.</p>
    #[serde(rename = "startedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_by: Option<String>,
    /// <p>The metadata that you apply to the task to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or full ARN of the task definition to run. If a <code>revision</code> is not specified, the latest <code>ACTIVE</code> revision is used.</p>
    #[serde(rename = "taskDefinition")]
    pub task_definition: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RunTaskResponse {
    /// <p>Any failures associated with the call.</p>
    #[serde(rename = "failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    /// <p>A full description of the tasks that were run. The tasks that were successfully placed on your cluster are described here.</p>
    #[serde(rename = "tasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<Task>>,
}

/// <p>A floating-point percentage of the desired number of tasks to place and keep running in the service. This is used when a service uses the <code>CODE_DEPLOY</code> deployment controller type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Scale {
    /// <p>The unit of measure for the scale value.</p>
    #[serde(rename = "unit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// <p>The value, specified as a percent total of a service's <code>desiredCount</code>, to scale the task set.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

/// <p>An object representing the secret to expose to your container. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/specifying-sensitive-data.html">Specifying Sensitive Data</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Secret {
    /// <p>The value to set as the environment variable on the container.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p><p>The secret to expose to the container. If your task is using the EC2 launch type, then supported values are either the full ARN of the AWS Secrets Manager secret or the full ARN of the parameter in the AWS Systems Manager Parameter Store. If your task is using the Fargate launch type, then the only supported value is the full ARN of the parameter in the AWS Systems Manager Parameter Store.</p> <note> <p>If the AWS Systems Manager Parameter Store parameter exists in the same Region as the task you are launching, then you can use either the full ARN or name of the parameter. If the parameter exists in a different Region, then the full ARN must be specified.</p> </note></p>
    #[serde(rename = "valueFrom")]
    pub value_from: String,
}

/// <p>Details on a service within a cluster</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Service {
    /// <p>The Amazon Resource Name (ARN) of the cluster that hosts the service.</p>
    #[serde(rename = "clusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <p>The Unix timestamp for when the service was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The principal that created the service.</p>
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// <p>Optional deployment parameters that control how many tasks run during the deployment and the ordering of stopping and starting tasks.</p>
    #[serde(rename = "deploymentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_configuration: Option<DeploymentConfiguration>,
    /// <p>The deployment controller type the service is using.</p>
    #[serde(rename = "deploymentController")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_controller: Option<DeploymentController>,
    /// <p>The current state of deployments for the service.</p>
    #[serde(rename = "deployments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployments: Option<Vec<Deployment>>,
    /// <p>The desired number of instantiations of the task definition to keep running on the service. This value is specified when the service is created with <a>CreateService</a>, and it can be modified with <a>UpdateService</a>.</p>
    #[serde(rename = "desiredCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<i64>,
    /// <p>Specifies whether to enable Amazon ECS managed tags for the tasks in the service. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-using-tags.html">Tagging Your Amazon ECS Resources</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "enableECSManagedTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ecs_managed_tags: Option<bool>,
    /// <p>The event stream for your service. A maximum of 100 of the latest events are displayed.</p>
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<ServiceEvent>>,
    /// <p>The period of time, in seconds, that the Amazon ECS service scheduler ignores unhealthy Elastic Load Balancing target health checks after a task has first started.</p>
    #[serde(rename = "healthCheckGracePeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_grace_period_seconds: Option<i64>,
    /// <p>The launch type on which your service is running. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_types.html">Amazon ECS Launch Types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "launchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    /// <p>A list of Elastic Load Balancing load balancer objects, containing the load balancer name, the container name (as it appears in a container definition), and the container port to access from the load balancer.</p> <p>Services with tasks that use the <code>awsvpc</code> network mode (for example, those with the Fargate launch type) only support Application Load Balancers and Network Load Balancers. Classic Load Balancers are not supported. Also, when you create any target groups for these services, you must choose <code>ip</code> as the target type, not <code>instance</code>. Tasks that use the <code>awsvpc</code> network mode are associated with an elastic network interface, not an Amazon EC2 instance.</p>
    #[serde(rename = "loadBalancers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<LoadBalancer>>,
    /// <p>The VPC subnet and security group configuration for tasks that receive their own elastic network interface by using the <code>awsvpc</code> networking mode.</p>
    #[serde(rename = "networkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// <p>The number of tasks in the cluster that are in the <code>PENDING</code> state.</p>
    #[serde(rename = "pendingCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<i64>,
    /// <p>The placement constraints for the tasks in the service.</p>
    #[serde(rename = "placementConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,
    /// <p>The placement strategy that determines how tasks for the service are placed.</p>
    #[serde(rename = "placementStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_strategy: Option<Vec<PlacementStrategy>>,
    /// <p>The platform version on which your tasks in the service are running. A platform version is only specified for tasks using the Fargate launch type. If one is not specified, the <code>LATEST</code> platform version is used by default. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html">AWS Fargate Platform Versions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "platformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>Specifies whether to propagate the tags from the task definition or the service to the task. If no value is specified, the tags are not propagated.</p>
    #[serde(rename = "propagateTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
    /// <p>The ARN of the IAM role associated with the service that allows the Amazon ECS container agent to register container instances with an Elastic Load Balancing load balancer.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The number of tasks in the cluster that are in the <code>RUNNING</code> state.</p>
    #[serde(rename = "runningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i64>,
    /// <p><p>The scheduling strategy to use for the service. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs_services.html">Services</a>.</p> <p>There are two service scheduler strategies available:</p> <ul> <li> <p> <code>REPLICA</code>-The replica scheduling strategy places and maintains the desired number of tasks across your cluster. By default, the service scheduler spreads tasks across Availability Zones. You can use task placement strategies and constraints to customize task placement decisions.</p> </li> <li> <p> <code>DAEMON</code>-The daemon scheduling strategy deploys exactly one task on each container instance in your cluster. When you are using this strategy, do not specify a desired number of tasks or any task placement strategies.</p> <note> <p>Fargate tasks do not support the <code>DAEMON</code> scheduling strategy.</p> </note> </li> </ul></p>
    #[serde(rename = "schedulingStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_strategy: Option<String>,
    /// <p>The ARN that identifies the service. The ARN contains the <code>arn:aws:ecs</code> namespace, followed by the Region of the service, the AWS account ID of the service owner, the <code>service</code> namespace, and then the service name. For example, <code>arn:aws:ecs:<i>region</i>:<i>012345678910</i>:service/<i>my-service</i> </code>.</p>
    #[serde(rename = "serviceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_arn: Option<String>,
    /// <p>The name of your service. Up to 255 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed. Service names must be unique within a cluster, but you can have similarly named services in multiple clusters within a Region or across multiple Regions.</p>
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "serviceRegistries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_registries: Option<Vec<ServiceRegistry>>,
    /// <p>The status of the service. The valid values are <code>ACTIVE</code>, <code>DRAINING</code>, or <code>INACTIVE</code>.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The metadata that you apply to the service to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The task definition to use for tasks in the service. This value is specified when the service is created with <a>CreateService</a>, and it can be modified with <a>UpdateService</a>.</p>
    #[serde(rename = "taskDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<String>,
    /// <p>Information about a set of Amazon ECS tasks in an AWS CodeDeploy deployment. An Amazon ECS task set includes details such as the desired number of tasks, how many tasks are running, and whether the task set serves production traffic.</p>
    #[serde(rename = "taskSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_sets: Option<Vec<TaskSet>>,
}

/// <p>Details on an event associated with a service.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ServiceEvent {
    /// <p>The Unix timestamp for when the event was triggered.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The ID string of the event.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The event message.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>Details of the service registry.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceRegistry {
    /// <p>The container name value, already specified in the task definition, to be used for your service discovery service. If the task definition that your service task specifies uses the <code>bridge</code> or <code>host</code> network mode, you must specify a <code>containerName</code> and <code>containerPort</code> combination from the task definition. If the task definition that your service task specifies uses the <code>awsvpc</code> network mode and a type SRV DNS record is used, you must specify either a <code>containerName</code> and <code>containerPort</code> combination or a <code>port</code> value, but not both.</p>
    #[serde(rename = "containerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    /// <p>The port value, already specified in the task definition, to be used for your service discovery service. If the task definition your service task specifies uses the <code>bridge</code> or <code>host</code> network mode, you must specify a <code>containerName</code> and <code>containerPort</code> combination from the task definition. If the task definition your service task specifies uses the <code>awsvpc</code> network mode and a type SRV DNS record is used, you must specify either a <code>containerName</code> and <code>containerPort</code> combination or a <code>port</code> value, but not both.</p>
    #[serde(rename = "containerPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_port: Option<i64>,
    /// <p>The port value used if your service discovery service specified an SRV record. This field may be used if both the <code>awsvpc</code> network mode and SRV records are used.</p>
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the service registry. The currently supported service registry is Amazon Route 53 Auto Naming. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_autonaming_Service.html">Service</a>.</p>
    #[serde(rename = "registryArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_arn: Option<String>,
}

/// <p>The current account setting for a resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Setting {
    /// <p>The account resource name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The ARN of the principal, which can be an IAM user, IAM role, or the root user. If this field is omitted, the authenticated user is assumed.</p>
    #[serde(rename = "principalArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_arn: Option<String>,
    /// <p>The current account setting for the resource name. If <code>enabled</code>, then the resource will receive the new Amazon Resource Name (ARN) and resource identifier (ID) format. If <code>disabled</code>, then the resource will receive the old Amazon Resource Name (ARN) and resource identifier (ID) format.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartTaskRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster on which to start your task. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>The container instance IDs or full ARN entries for the container instances on which you would like to place your task. You can specify up to 10 container instances.</p>
    #[serde(rename = "containerInstances")]
    pub container_instances: Vec<String>,
    /// <p>Specifies whether to enable Amazon ECS managed tags for the task. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-using-tags.html">Tagging Your Amazon ECS Resources</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "enableECSManagedTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ecs_managed_tags: Option<bool>,
    /// <p>The name of the task group to associate with the task. The default value is the family name of the task definition (for example, family:my-family-name).</p>
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// <p>The VPC subnet and security group configuration for tasks that receive their own elastic network interface by using the <code>awsvpc</code> networking mode.</p>
    #[serde(rename = "networkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// <p><p>A list of container overrides in JSON format that specify the name of a container in the specified task definition and the overrides it should receive. You can override the default command for a container (that is specified in the task definition or Docker image) with a <code>command</code> override. You can also override existing environment variables (that are specified in the task definition or Docker image) on a container or add new environment variables to it with an <code>environment</code> override.</p> <note> <p>A total of 8192 characters are allowed for overrides. This limit includes the JSON formatting characters of the override structure.</p> </note></p>
    #[serde(rename = "overrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<TaskOverride>,
    /// <p>Specifies whether to propagate the tags from the task definition or the service to the task. If no value is specified, the tags are not propagated.</p>
    #[serde(rename = "propagateTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
    /// <p>An optional tag specified when a task is started. For example, if you automatically trigger a task to run a batch process job, you could apply a unique identifier for that job to your task with the <code>startedBy</code> parameter. You can then identify which tasks belong to that job by filtering the results of a <a>ListTasks</a> call with the <code>startedBy</code> value. Up to 36 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed.</p> <p>If a task is started by an Amazon ECS service, then the <code>startedBy</code> parameter contains the deployment ID of the service that starts it.</p>
    #[serde(rename = "startedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_by: Option<String>,
    /// <p>The metadata that you apply to the task to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or full ARN of the task definition to start. If a <code>revision</code> is not specified, the latest <code>ACTIVE</code> revision is used.</p>
    #[serde(rename = "taskDefinition")]
    pub task_definition: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartTaskResponse {
    /// <p>Any failures associated with the call.</p>
    #[serde(rename = "failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
    /// <p>A full description of the tasks that were started. Each task that was successfully placed on your container instances is described.</p>
    #[serde(rename = "tasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<Task>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopTaskRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the task to stop. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>An optional message specified when a task is stopped. For example, if you are using a custom scheduler, you can use this parameter to specify the reason for stopping the task here, and the message appears in subsequent <a>DescribeTasks</a> API operations on this task. Up to 255 characters are allowed in this message.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The task ID or full ARN entry of the task to stop.</p>
    #[serde(rename = "task")]
    pub task: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StopTaskResponse {
    /// <p>The task that was stopped.</p>
    #[serde(rename = "task")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<Task>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SubmitContainerStateChangeRequest {
    /// <p>The short name or full ARN of the cluster that hosts the container.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>The name of the container.</p>
    #[serde(rename = "containerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    /// <p>The exit code returned for the state change request.</p>
    #[serde(rename = "exitCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i64>,
    /// <p>The network bindings of the container.</p>
    #[serde(rename = "networkBindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_bindings: Option<Vec<NetworkBinding>>,
    /// <p>The reason for the state change request.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The status of the state change request.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The task ID or full Amazon Resource Name (ARN) of the task that hosts the container.</p>
    #[serde(rename = "task")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SubmitContainerStateChangeResponse {
    /// <p>Acknowledgement of the state change.</p>
    #[serde(rename = "acknowledgment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledgment: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SubmitTaskStateChangeRequest {
    /// <p>Any attachments associated with the state change request.</p>
    #[serde(rename = "attachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<AttachmentStateChange>>,
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the task.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>Any containers associated with the state change request.</p>
    #[serde(rename = "containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<ContainerStateChange>>,
    /// <p>The Unix timestamp for when the task execution stopped.</p>
    #[serde(rename = "executionStoppedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_stopped_at: Option<f64>,
    /// <p>The Unix timestamp for when the container image pull began.</p>
    #[serde(rename = "pullStartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_started_at: Option<f64>,
    /// <p>The Unix timestamp for when the container image pull completed.</p>
    #[serde(rename = "pullStoppedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_stopped_at: Option<f64>,
    /// <p>The reason for the state change request.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The status of the state change request.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The task ID or full ARN of the task in the state change request.</p>
    #[serde(rename = "task")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SubmitTaskStateChangeResponse {
    /// <p>Acknowledgement of the state change.</p>
    #[serde(rename = "acknowledgment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledgment: Option<String>,
}

/// <p><p>A list of namespaced kernel parameters to set in the container. This parameter maps to <code>Sysctls</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--sysctl</code> option to <a href="https://docs.docker.com/engine/reference/run/">docker run</a>.</p> <p>It is not recommended that you specify network-related <code>systemControls</code> parameters for multiple containers in a single task that also uses either the <code>awsvpc</code> or <code>host</code> network mode for the following reasons:</p> <ul> <li> <p>For tasks that use the <code>awsvpc</code> network mode, if you set <code>systemControls</code> for any container, it applies to all containers in the task. If you set different <code>systemControls</code> for multiple containers in a single task, the container that is started last determines which <code>systemControls</code> take effect.</p> </li> <li> <p>For tasks that use the <code>host</code> network mode, the <code>systemControls</code> parameter applies to the container instance&#39;s kernel parameter as well as that of all containers of any tasks running on that container instance.</p> </li> </ul></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemControl {
    /// <p>The namespaced kernel parameter for which to set a <code>value</code>.</p>
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// <p>The value for the namespaced kernel parameter specified in <code>namespace</code>.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The metadata that you apply to a resource to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>One part of a key-value pair that make up a tag. A <code>key</code> is a general label that acts like a category for more specific tag values.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The optional part of a key-value pair that make up a tag. A <code>value</code> acts as a descriptor within a tag category (key).</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource to which to add tags. Currently, the supported resources are Amazon ECS tasks, services, task definitions, clusters, and container instances.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tags to add to the resource. A tag is an array of key-value pairs. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Details on a task in a cluster.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Task {
    /// <p>The Elastic Network Adapter associated with the task if the task uses the <code>awsvpc</code> network mode.</p>
    #[serde(rename = "attachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    /// <p>The ARN of the cluster that hosts the task.</p>
    #[serde(rename = "clusterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_arn: Option<String>,
    /// <p>The connectivity status of a task.</p>
    #[serde(rename = "connectivity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity: Option<String>,
    /// <p>The Unix timestamp for when the task last went into <code>CONNECTED</code> status.</p>
    #[serde(rename = "connectivityAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_at: Option<f64>,
    /// <p>The ARN of the container instances that host the task.</p>
    #[serde(rename = "containerInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_arn: Option<String>,
    /// <p>The containers associated with the task.</p>
    #[serde(rename = "containers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<Container>>,
    /// <p><p>The number of CPU units used by the task as expressed in a task definition. It can be expressed as an integer using CPU units, for example <code>1024</code>. It can also be expressed as a string using vCPUs, for example <code>1 vCPU</code> or <code>1 vcpu</code>. String values are converted to an integer indicating the CPU units when the task definition is registered.</p> <p>If you are using the EC2 launch type, this field is optional. Supported values are between <code>128</code> CPU units (<code>0.125</code> vCPUs) and <code>10240</code> CPU units (<code>10</code> vCPUs).</p> <p>If you are using the Fargate launch type, this field is required and you must use one of the following values, which determines your range of supported values for the <code>memory</code> parameter:</p> <ul> <li> <p>256 (.25 vCPU) - Available <code>memory</code> values: 512 (0.5 GB), 1024 (1 GB), 2048 (2 GB)</p> </li> <li> <p>512 (.5 vCPU) - Available <code>memory</code> values: 1024 (1 GB), 2048 (2 GB), 3072 (3 GB), 4096 (4 GB)</p> </li> <li> <p>1024 (1 vCPU) - Available <code>memory</code> values: 2048 (2 GB), 3072 (3 GB), 4096 (4 GB), 5120 (5 GB), 6144 (6 GB), 7168 (7 GB), 8192 (8 GB)</p> </li> <li> <p>2048 (2 vCPU) - Available <code>memory</code> values: Between 4096 (4 GB) and 16384 (16 GB) in increments of 1024 (1 GB)</p> </li> <li> <p>4096 (4 vCPU) - Available <code>memory</code> values: Between 8192 (8 GB) and 30720 (30 GB) in increments of 1024 (1 GB)</p> </li> </ul></p>
    #[serde(rename = "cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    /// <p>The Unix timestamp for when the task was created (the task entered the <code>PENDING</code> state).</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The desired status of the task. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_life_cycle.html">Task Lifecycle</a>.</p>
    #[serde(rename = "desiredStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_status: Option<String>,
    /// <p>The Unix timestamp for when the task execution stopped.</p>
    #[serde(rename = "executionStoppedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_stopped_at: Option<f64>,
    /// <p>The name of the task group associated with the task.</p>
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// <p><p>The health status for the task, which is determined by the health of the essential containers in the task. If all essential containers in the task are reporting as <code>HEALTHY</code>, then the task status also reports as <code>HEALTHY</code>. If any essential containers in the task are reporting as <code>UNHEALTHY</code> or <code>UNKNOWN</code>, then the task status also reports as <code>UNHEALTHY</code> or <code>UNKNOWN</code>, accordingly.</p> <note> <p>The Amazon ECS container agent does not monitor or report on Docker health checks that are embedded in a container image (such as those specified in a parent image or from the image&#39;s Dockerfile) and not specified in the container definition. Health check parameters that are specified in a container definition override any Docker health checks that exist in the container image.</p> </note></p>
    #[serde(rename = "healthStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
    /// <p>The last known status of the task. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_life_cycle.html">Task Lifecycle</a>.</p>
    #[serde(rename = "lastStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_status: Option<String>,
    /// <p>The launch type on which your task is running. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_types.html">Amazon ECS Launch Types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "launchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    /// <p><p>The amount of memory (in MiB) used by the task as expressed in a task definition. It can be expressed as an integer using MiB, for example <code>1024</code>. It can also be expressed as a string using GB, for example <code>1GB</code> or <code>1 GB</code>. String values are converted to an integer indicating the MiB when the task definition is registered.</p> <p>If you are using the EC2 launch type, this field is optional.</p> <p>If you are using the Fargate launch type, this field is required and you must use one of the following values, which determines your range of supported values for the <code>cpu</code> parameter:</p> <ul> <li> <p>512 (0.5 GB), 1024 (1 GB), 2048 (2 GB) - Available <code>cpu</code> values: 256 (.25 vCPU)</p> </li> <li> <p>1024 (1 GB), 2048 (2 GB), 3072 (3 GB), 4096 (4 GB) - Available <code>cpu</code> values: 512 (.5 vCPU)</p> </li> <li> <p>2048 (2 GB), 3072 (3 GB), 4096 (4 GB), 5120 (5 GB), 6144 (6 GB), 7168 (7 GB), 8192 (8 GB) - Available <code>cpu</code> values: 1024 (1 vCPU)</p> </li> <li> <p>Between 4096 (4 GB) and 16384 (16 GB) in increments of 1024 (1 GB) - Available <code>cpu</code> values: 2048 (2 vCPU)</p> </li> <li> <p>Between 8192 (8 GB) and 30720 (30 GB) in increments of 1024 (1 GB) - Available <code>cpu</code> values: 4096 (4 vCPU)</p> </li> </ul></p>
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    /// <p>One or more container overrides.</p>
    #[serde(rename = "overrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<TaskOverride>,
    /// <p>The platform version on which your task is running. A platform version is only specified for tasks using the Fargate launch type. If one is not specified, the <code>LATEST</code> platform version is used by default. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html">AWS Fargate Platform Versions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "platformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>The Unix timestamp for when the container image pull began.</p>
    #[serde(rename = "pullStartedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_started_at: Option<f64>,
    /// <p>The Unix timestamp for when the container image pull completed.</p>
    #[serde(rename = "pullStoppedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_stopped_at: Option<f64>,
    /// <p>The Unix timestamp for when the task started (the task transitioned from the <code>PENDING</code> state to the <code>RUNNING</code> state).</p>
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    /// <p>The tag specified when a task is started. If the task is started by an Amazon ECS service, then the <code>startedBy</code> parameter contains the deployment ID of the service that starts it.</p>
    #[serde(rename = "startedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_by: Option<String>,
    /// <p>The stop code indicating why a task was stopped. The <code>stoppedReason</code> may contain additional details.</p>
    #[serde(rename = "stopCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_code: Option<String>,
    /// <p>The Unix timestamp for when the task was stopped (the task transitioned from the <code>RUNNING</code> state to the <code>STOPPED</code> state).</p>
    #[serde(rename = "stoppedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<f64>,
    /// <p>The reason that the task was stopped.</p>
    #[serde(rename = "stoppedReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_reason: Option<String>,
    /// <p>The Unix timestamp for when the task stops (transitions from the <code>RUNNING</code> state to <code>STOPPED</code>).</p>
    #[serde(rename = "stoppingAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopping_at: Option<f64>,
    /// <p>The metadata that you apply to the task to help you categorize and organize them. Each tag consists of a key and an optional value, both of which you define. Tag keys can have a maximum character length of 128 characters, and tag values can have a maximum length of 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The Amazon Resource Name (ARN) of the task.</p>
    #[serde(rename = "taskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    /// <p>The ARN of the task definition that creates the task.</p>
    #[serde(rename = "taskDefinitionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition_arn: Option<String>,
    /// <p>The version counter for the task. Every time a task experiences a change that triggers a CloudWatch event, the version counter is incremented. If you are replicating your Amazon ECS task state with CloudWatch Events, you can compare the version of a task reported by the Amazon ECS API actionss with the version reported in CloudWatch Events for the task (inside the <code>detail</code> object) to verify that the version in your event stream is current.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Details of a task definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TaskDefinition {
    /// <p>The launch type to use with your task. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_types.html">Amazon ECS Launch Types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "compatibilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibilities: Option<Vec<String>>,
    /// <p>A list of container definitions in JSON format that describe the different containers that make up your task. For more information about container definition parameters and defaults, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_defintions.html">Amazon ECS Task Definitions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "containerDefinitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_definitions: Option<Vec<ContainerDefinition>>,
    /// <p><p>The number of <code>cpu</code> units used by the task. If you are using the EC2 launch type, this field is optional and any value can be used. If you are using the Fargate launch type, this field is required and you must use one of the following values, which determines your range of valid values for the <code>memory</code> parameter:</p> <ul> <li> <p>256 (.25 vCPU) - Available <code>memory</code> values: 512 (0.5 GB), 1024 (1 GB), 2048 (2 GB)</p> </li> <li> <p>512 (.5 vCPU) - Available <code>memory</code> values: 1024 (1 GB), 2048 (2 GB), 3072 (3 GB), 4096 (4 GB)</p> </li> <li> <p>1024 (1 vCPU) - Available <code>memory</code> values: 2048 (2 GB), 3072 (3 GB), 4096 (4 GB), 5120 (5 GB), 6144 (6 GB), 7168 (7 GB), 8192 (8 GB)</p> </li> <li> <p>2048 (2 vCPU) - Available <code>memory</code> values: Between 4096 (4 GB) and 16384 (16 GB) in increments of 1024 (1 GB)</p> </li> <li> <p>4096 (4 vCPU) - Available <code>memory</code> values: Between 8192 (8 GB) and 30720 (30 GB) in increments of 1024 (1 GB)</p> </li> </ul></p>
    #[serde(rename = "cpu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the task execution role that the Amazon ECS container agent and the Docker daemon can assume.</p>
    #[serde(rename = "executionRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    /// <p>The family of your task definition, used as the definition name.</p>
    #[serde(rename = "family")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    /// <p><p>The IPC resource namespace to use for the containers in the task. The valid values are <code>host</code>, <code>task</code>, or <code>none</code>. If <code>host</code> is specified, then all containers within the tasks that specified the <code>host</code> IPC mode on the same container instance share the same IPC resources with the host Amazon EC2 instance. If <code>task</code> is specified, all containers within the specified task share the same IPC resources. If <code>none</code> is specified, then IPC resources within the containers of a task are private and not shared with other containers in a task or on the container instance. If no value is specified, then the IPC resource namespace sharing depends on the Docker daemon setting on the container instance. For more information, see <a href="https://docs.docker.com/engine/reference/run/#ipc-settings---ipc">IPC settings</a> in the <i>Docker run reference</i>.</p> <p>If the <code>host</code> IPC mode is used, be aware that there is a heightened risk of undesired IPC namespace expose. For more information, see <a href="https://docs.docker.com/engine/security/security/">Docker security</a>.</p> <p>If you are setting namespaced kernel parameters using <code>systemControls</code> for the containers in the task, the following will apply to your IPC resource namespace. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_definition_parameters.html">System Controls</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <ul> <li> <p>For tasks that use the <code>host</code> IPC mode, IPC namespace related <code>systemControls</code> are not supported.</p> </li> <li> <p>For tasks that use the <code>task</code> IPC mode, IPC namespace related <code>systemControls</code> will apply to all containers within a task.</p> </li> </ul> <note> <p>This parameter is not supported for Windows containers or tasks using the Fargate launch type.</p> </note></p>
    #[serde(rename = "ipcMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipc_mode: Option<String>,
    /// <p><p>The amount (in MiB) of memory used by the task. If using the EC2 launch type, this field is optional and any value can be used. If using the Fargate launch type, this field is required and you must use one of the following values, which determines your range of valid values for the <code>cpu</code> parameter:</p> <ul> <li> <p>512 (0.5 GB), 1024 (1 GB), 2048 (2 GB) - Available <code>cpu</code> values: 256 (.25 vCPU)</p> </li> <li> <p>1024 (1 GB), 2048 (2 GB), 3072 (3 GB), 4096 (4 GB) - Available <code>cpu</code> values: 512 (.5 vCPU)</p> </li> <li> <p>2048 (2 GB), 3072 (3 GB), 4096 (4 GB), 5120 (5 GB), 6144 (6 GB), 7168 (7 GB), 8192 (8 GB) - Available <code>cpu</code> values: 1024 (1 vCPU)</p> </li> <li> <p>Between 4096 (4 GB) and 16384 (16 GB) in increments of 1024 (1 GB) - Available <code>cpu</code> values: 2048 (2 vCPU)</p> </li> <li> <p>Between 8192 (8 GB) and 30720 (30 GB) in increments of 1024 (1 GB) - Available <code>cpu</code> values: 4096 (4 vCPU)</p> </li> </ul></p>
    #[serde(rename = "memory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
    /// <p>The Docker networking mode to use for the containers in the task. The valid values are <code>none</code>, <code>bridge</code>, <code>awsvpc</code>, and <code>host</code>. The default Docker network mode is <code>bridge</code>. If you are using the Fargate launch type, the <code>awsvpc</code> network mode is required. If you are using the EC2 launch type, any network mode can be used. If the network mode is set to <code>none</code>, you cannot specify port mappings in your container definitions, and the tasks containers do not have external connectivity. The <code>host</code> and <code>awsvpc</code> network modes offer the highest networking performance for containers because they use the EC2 network stack instead of the virtualized network stack provided by the <code>bridge</code> mode.</p> <p>With the <code>host</code> and <code>awsvpc</code> network modes, exposed container ports are mapped directly to the corresponding host port (for the <code>host</code> network mode) or the attached elastic network interface port (for the <code>awsvpc</code> network mode), so you cannot take advantage of dynamic host port mappings. </p> <p>If the network mode is <code>awsvpc</code>, the task is allocated an elastic network interface, and you must specify a <a>NetworkConfiguration</a> value when you create a service or run a task with the task definition. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-networking.html">Task Networking</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <note> <p>Currently, only Amazon ECS-optimized AMIs, other Amazon Linux variants with the <code>ecs-init</code> package, or AWS Fargate infrastructure support the <code>awsvpc</code> network mode. </p> </note> <p>If the network mode is <code>host</code>, you cannot run multiple instantiations of the same task on a single container instance when port mappings are used.</p> <p>Docker for Windows uses different network modes than Docker for Linux. When you register a task definition with Windows containers, you must not specify a network mode. If you use the console to register a task definition with Windows containers, you must choose the <code>&lt;default&gt;</code> network mode object. </p> <p>For more information, see <a href="https://docs.docker.com/engine/reference/run/#network-settings">Network settings</a> in the <i>Docker run reference</i>.</p>
    #[serde(rename = "networkMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_mode: Option<String>,
    /// <p><p>The process namespace to use for the containers in the task. The valid values are <code>host</code> or <code>task</code>. If <code>host</code> is specified, then all containers within the tasks that specified the <code>host</code> PID mode on the same container instance share the same IPC resources with the host Amazon EC2 instance. If <code>task</code> is specified, all containers within the specified task share the same process namespace. If no value is specified, the default is a private namespace. For more information, see <a href="https://docs.docker.com/engine/reference/run/#pid-settings---pid">PID settings</a> in the <i>Docker run reference</i>.</p> <p>If the <code>host</code> PID mode is used, be aware that there is a heightened risk of undesired process namespace expose. For more information, see <a href="https://docs.docker.com/engine/security/security/">Docker security</a>.</p> <note> <p>This parameter is not supported for Windows containers or tasks using the Fargate launch type.</p> </note></p>
    #[serde(rename = "pidMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid_mode: Option<String>,
    /// <p>An array of placement constraint objects to use for tasks. This field is not valid if you are using the Fargate launch type for your task.</p>
    #[serde(rename = "placementConstraints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<TaskDefinitionPlacementConstraint>>,
    /// <p>The container instance attributes required by your task. This field is not valid if you are using the Fargate launch type for your task.</p>
    #[serde(rename = "requiresAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_attributes: Option<Vec<Attribute>>,
    /// <p>The launch type that the task is using.</p>
    #[serde(rename = "requiresCompatibilities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_compatibilities: Option<Vec<String>>,
    /// <p>The revision of the task in a particular family. The revision is a version number of a task definition in a family. When you register a task definition for the first time, the revision is <code>1</code>. Each time that you register a new revision of a task definition in the same family, the revision value always increases by one, even if you have deregistered previous revisions in this family.</p>
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    /// <p>The status of the task definition.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The full Amazon Resource Name (ARN) of the task definition.</p>
    #[serde(rename = "taskDefinitionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition_arn: Option<String>,
    /// <p>The ARN of the IAM role that containers in this task can assume. All containers in this task are granted the permissions that are specified in this role.</p> <p>IAM roles for tasks on Windows require that the <code>-EnableTaskIAMRole</code> option is set when you launch the Amazon ECS-optimized Windows AMI. Your containers must also run some configuration code in order to take advantage of the feature. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/windows_task_IAM_roles.html">Windows IAM Roles for Tasks</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "taskRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<String>,
    /// <p>The list of volumes in a task.</p> <p>If you are using the Fargate launch type, the <code>host</code> and <code>sourcePath</code> parameters are not supported.</p> <p>For more information about volume definition parameters and defaults, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_definitions.html">Amazon ECS Task Definitions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "volumes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

/// <p>An object representing a constraint on task placement in the task definition.</p> <p>If you are using the Fargate launch type, task placement constraints are not supported.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-placement-constraints.html">Task Placement Constraints</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskDefinitionPlacementConstraint {
    /// <p>A cluster query language expression to apply to the constraint. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html">Cluster Query Language</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "expression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    /// <p>The type of constraint. The <code>DistinctInstance</code> constraint ensures that each task in a particular group is running on a different container instance. The <code>MemberOf</code> constraint restricts selection to be from a group of valid candidates.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>The overrides associated with a task.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskOverride {
    /// <p>One or more container overrides sent to a task.</p>
    #[serde(rename = "containerOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_overrides: Option<Vec<ContainerOverride>>,
    /// <p>The Amazon Resource Name (ARN) of the task execution role that the Amazon ECS container agent and the Docker daemon can assume.</p>
    #[serde(rename = "executionRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role that containers in this task can assume. All containers in this task are granted the permissions that are specified in this role.</p>
    #[serde(rename = "taskRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<String>,
}

/// <p>Information about a set of Amazon ECS tasks in an AWS CodeDeploy deployment. An Amazon ECS task set includes details such as the desired number of tasks, how many tasks are running, and whether the task set serves production traffic.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TaskSet {
    /// <p>The computed desired count for the task set. This is calculated by multiplying the service's <code>desiredCount</code> by the task set's <code>scale</code> percentage.</p>
    #[serde(rename = "computedDesiredCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub computed_desired_count: Option<i64>,
    /// <p>The Unix timestamp for when the task set was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The deployment ID of the AWS CodeDeploy deployment.</p>
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// <p>The ID of the task set.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The launch type the tasks in the task set are using. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/launch_types.html">Amazon ECS Launch Types</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "launchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    /// <p>Details on a load balancer that is used with a task set.</p>
    #[serde(rename = "loadBalancers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_balancers: Option<Vec<LoadBalancer>>,
    /// <p>The network configuration for the task set.</p>
    #[serde(rename = "networkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// <p>The number of tasks in the task set that are in the <code>PENDING</code> status during a deployment. A task in the <code>PENDING</code> state is preparing to enter the <code>RUNNING</code> state. A task set enters the <code>PENDING</code> status when it launches for the first time, or when it is restarted after being in the <code>STOPPED</code> state.</p>
    #[serde(rename = "pendingCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_count: Option<i64>,
    /// <p>The platform version on which the tasks in the task set are running. A platform version is only specified for tasks using the Fargate launch type. If one is not specified, the <code>LATEST</code> platform version is used by default. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html">AWS Fargate Platform Versions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "platformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>The number of tasks in the task set that are in the <code>RUNNING</code> status during a deployment. A task in the <code>RUNNING</code> state is running and ready for use.</p>
    #[serde(rename = "runningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running_count: Option<i64>,
    /// <p>A floating-point percentage of the desired number of tasks to place and keep running in the service.</p>
    #[serde(rename = "scale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<Scale>,
    /// <p>The stability status, which indicates whether the task set has reached a steady state. If the following conditions are met, the task set will be in <code>STEADY_STATE</code>:</p> <ul> <li> <p>The task <code>runningCount</code> is equal to the <code>computedDesiredCount</code>.</p> </li> <li> <p>The <code>pendingCount</code> is <code>0</code>.</p> </li> <li> <p>There are no tasks running on container instances in the <code>DRAINING</code> status.</p> </li> <li> <p>All tasks are reporting a healthy status from the load balancers, service discovery, and container health checks.</p> </li> </ul> <p>If any of those conditions are not met, the stability status returns <code>STABILIZING</code>.</p>
    #[serde(rename = "stabilityStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stability_status: Option<String>,
    /// <p>The Unix timestamp for when the task set stability status was retrieved.</p>
    #[serde(rename = "stabilityStatusAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stability_status_at: Option<f64>,
    /// <p>The tag specified when a task set is started. If the task is started by an AWS CodeDeploy deployment, then the <code>startedBy</code> parameter is <code>CODE_DEPLOY</code>.</p>
    #[serde(rename = "startedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_by: Option<String>,
    /// <p><p>The status of the task set. The following describes each state:</p> <dl> <dt>PRIMARY</dt> <dd> <p>The task set is serving production traffic.</p> </dd> <dt>ACTIVE</dt> <dd> <p>The task set is not serving production traffic.</p> </dd> <dt>DRAINING</dt> <dd> <p>The tasks in the task set are being stopped and their corresponding targets are being deregistered from their target group.</p> </dd> </dl></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The task definition the task set is using.</p>
    #[serde(rename = "taskDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the task set.</p>
    #[serde(rename = "taskSetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_set_arn: Option<String>,
    /// <p>The Unix timestamp for when the task set was last updated.</p>
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

/// <p>The container path, mount options, and size of the tmpfs mount.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tmpfs {
    /// <p>The absolute file path where the tmpfs volume is to be mounted.</p>
    #[serde(rename = "containerPath")]
    pub container_path: String,
    /// <p>The list of tmpfs volume mount options.</p> <p>Valid values: <code>"defaults" | "ro" | "rw" | "suid" | "nosuid" | "dev" | "nodev" | "exec" | "noexec" | "sync" | "async" | "dirsync" | "remount" | "mand" | "nomand" | "atime" | "noatime" | "diratime" | "nodiratime" | "bind" | "rbind" | "unbindable" | "runbindable" | "private" | "rprivate" | "shared" | "rshared" | "slave" | "rslave" | "relatime" | "norelatime" | "strictatime" | "nostrictatime" | "mode" | "uid" | "gid" | "nr_inodes" | "nr_blocks" | "mpol"</code> </p>
    #[serde(rename = "mountOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<Vec<String>>,
    /// <p>The size (in MiB) of the tmpfs volume.</p>
    #[serde(rename = "size")]
    pub size: i64,
}

/// <p>The <code>ulimit</code> settings to pass to the container.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ulimit {
    /// <p>The hard limit for the ulimit type.</p>
    #[serde(rename = "hardLimit")]
    pub hard_limit: i64,
    /// <p>The <code>type</code> of the <code>ulimit</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The soft limit for the ulimit type.</p>
    #[serde(rename = "softLimit")]
    pub soft_limit: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource from which to delete tags. Currently, the supported resources are Amazon ECS tasks, services, task definitions, clusters, and container instances.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The keys of the tags to be removed.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateContainerAgentRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that your container instance is running on. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>The container instance ID or full ARN entries for the container instance on which you would like to update the Amazon ECS container agent.</p>
    #[serde(rename = "containerInstance")]
    pub container_instance: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateContainerAgentResponse {
    /// <p>The container instance for which the container agent was updated.</p>
    #[serde(rename = "containerInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance: Option<ContainerInstance>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateContainerInstancesStateRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the container instance to update. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>A list of container instance IDs or full ARN entries.</p>
    #[serde(rename = "containerInstances")]
    pub container_instances: Vec<String>,
    /// <p>The container instance state with which to update the container instance.</p>
    #[serde(rename = "status")]
    pub status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateContainerInstancesStateResponse {
    /// <p>The list of container instances.</p>
    #[serde(rename = "containerInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instances: Option<Vec<ContainerInstance>>,
    /// <p>Any failures associated with the call.</p>
    #[serde(rename = "failures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<Failure>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateServiceRequest {
    /// <p>The short name or full Amazon Resource Name (ARN) of the cluster that your service is running on. If you do not specify a cluster, the default cluster is assumed.</p>
    #[serde(rename = "cluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    /// <p>Optional deployment parameters that control how many tasks run during the deployment and the ordering of stopping and starting tasks.</p>
    #[serde(rename = "deploymentConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_configuration: Option<DeploymentConfiguration>,
    /// <p>The number of instantiations of the task to place and keep running in your service.</p>
    #[serde(rename = "desiredCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_count: Option<i64>,
    /// <p>Whether to force a new deployment of the service. Deployments are not forced by default. You can use this option to trigger a new deployment with no service definition changes. For example, you can update a service's tasks to use a newer Docker image with the same image/tag combination (<code>my_image:latest</code>) or to roll Fargate tasks onto a newer platform version.</p>
    #[serde(rename = "forceNewDeployment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_new_deployment: Option<bool>,
    /// <p>The period of time, in seconds, that the Amazon ECS service scheduler should ignore unhealthy Elastic Load Balancing target health checks after a task has first started. This is only valid if your service is configured to use a load balancer. If your service's tasks take a while to start and respond to Elastic Load Balancing health checks, you can specify a health check grace period of up to 1,800 seconds. During that time, the ECS service scheduler ignores the Elastic Load Balancing health check status. This grace period can prevent the ECS service scheduler from marking tasks as unhealthy and stopping them before they have time to come up.</p>
    #[serde(rename = "healthCheckGracePeriodSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_check_grace_period_seconds: Option<i64>,
    /// <p><p>The network configuration for the service. This parameter is required for task definitions that use the <code>awsvpc</code> network mode to receive their own elastic network interface, and it is not supported for other network modes. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-networking.html">Task Networking</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <note> <p>Updating a service to add a subnet to a list of existing subnets does not trigger a service deployment. For example, if your network configuration change is to keep the existing subnets and simply add another subnet to the network configuration, this does not trigger a new service deployment.</p> </note></p>
    #[serde(rename = "networkConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    /// <p>The platform version on which your tasks in the service are running. A platform version is only specified for tasks using the Fargate launch type. If one is not specified, the <code>LATEST</code> platform version is used by default. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/platform_versions.html">AWS Fargate Platform Versions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    #[serde(rename = "platformVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    /// <p>The name of the service to update.</p>
    #[serde(rename = "service")]
    pub service: String,
    /// <p>The <code>family</code> and <code>revision</code> (<code>family:revision</code>) or full ARN of the task definition to run in your service. If a <code>revision</code> is not specified, the latest <code>ACTIVE</code> revision is used. If you modify the task definition with <code>UpdateService</code>, Amazon ECS spawns a task with the new version of the task definition and then stops an old task after the new version is running.</p>
    #[serde(rename = "taskDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_definition: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateServiceResponse {
    /// <p>The full description of your service following the update call.</p>
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Service>,
}

/// <p>The Docker and Amazon ECS container agent version information about a container instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VersionInfo {
    /// <p>The Git commit hash for the Amazon ECS container agent build on the <a href="https://github.com/aws/amazon-ecs-agent/commits/master">amazon-ecs-agent </a> GitHub repository.</p>
    #[serde(rename = "agentHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_hash: Option<String>,
    /// <p>The version number of the Amazon ECS container agent.</p>
    #[serde(rename = "agentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    /// <p>The Docker version running on the container instance.</p>
    #[serde(rename = "dockerVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_version: Option<String>,
}

/// <p>A data volume used in a task definition. For tasks that use a Docker volume, specify a <code>DockerVolumeConfiguration</code>. For tasks that use a bind mount host volume, specify a <code>host</code> and optional <code>sourcePath</code>. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/using_data_volumes.html">Using Data Volumes in Tasks</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Volume {
    /// <p>This parameter is specified when you are using Docker volumes. Docker volumes are only supported when you are using the EC2 launch type. Windows containers only support the use of the <code>local</code> driver. To use bind mounts, specify a <code>host</code> instead.</p>
    #[serde(rename = "dockerVolumeConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docker_volume_configuration: Option<DockerVolumeConfiguration>,
    /// <p>This parameter is specified when you are using bind mount host volumes. Bind mount host volumes are supported when you are using either the EC2 or Fargate launch types. The contents of the <code>host</code> parameter determine whether your bind mount host volume persists on the host container instance and where it is stored. If the <code>host</code> parameter is empty, then the Docker daemon assigns a host path for your data volume. However, the data is not guaranteed to persist after the containers associated with it stop running.</p> <p>Windows containers can mount whole directories on the same drive as <code>$env:ProgramData</code>. Windows containers cannot mount directories on a different drive, and mount point cannot be across drives. For example, you can mount <code>C:\my\path:C:\my\path</code> and <code>D:\:D:\</code>, but not <code>D:\my\path:C:\my\path</code> or <code>D:\:C:\my\path</code>.</p>
    #[serde(rename = "host")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<HostVolumeProperties>,
    /// <p>The name of the volume. Up to 255 letters (uppercase and lowercase), numbers, hyphens, and underscores are allowed. This name is referenced in the <code>sourceVolume</code> parameter of container definition <code>mountPoints</code>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Details on a data volume from another container in the same task definition.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VolumeFrom {
    /// <p>If this value is <code>true</code>, the container has read-only access to the volume. If this value is <code>false</code>, then the container can write to the volume. The default value is <code>false</code>.</p>
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// <p>The name of another container within the same task definition from which to mount volumes.</p>
    #[serde(rename = "sourceContainer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_container: Option<String>,
}

/// Errors returned by CreateCluster
#[derive(Debug, PartialEq)]
pub enum CreateClusterError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl CreateClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(CreateClusterError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateClusterError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(CreateClusterError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateClusterError {
    fn description(&self) -> &str {
        match *self {
            CreateClusterError::Client(ref cause) => cause,
            CreateClusterError::InvalidParameter(ref cause) => cause,
            CreateClusterError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateService
#[derive(Debug, PartialEq)]
pub enum CreateServiceError {
    /// <p>You do not have authorization to perform the requested action.</p>
    AccessDenied(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region-specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified platform version does not satisfy the task definition's required capabilities.</p>
    PlatformTaskDefinitionIncompatibility(String),
    /// <p>The specified platform version does not exist.</p>
    PlatformUnknown(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// <p>The specified task is not supported in this Region.</p>
    UnsupportedFeature(String),
}

impl CreateServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateServiceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateServiceError::AccessDenied(err.msg))
                }
                "ClientException" => {
                    return RusotoError::Service(CreateServiceError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(CreateServiceError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateServiceError::InvalidParameter(err.msg))
                }
                "PlatformTaskDefinitionIncompatibilityException" => {
                    return RusotoError::Service(
                        CreateServiceError::PlatformTaskDefinitionIncompatibility(err.msg),
                    )
                }
                "PlatformUnknownException" => {
                    return RusotoError::Service(CreateServiceError::PlatformUnknown(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(CreateServiceError::Server(err.msg))
                }
                "UnsupportedFeatureException" => {
                    return RusotoError::Service(CreateServiceError::UnsupportedFeature(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateServiceError {
    fn description(&self) -> &str {
        match *self {
            CreateServiceError::AccessDenied(ref cause) => cause,
            CreateServiceError::Client(ref cause) => cause,
            CreateServiceError::ClusterNotFound(ref cause) => cause,
            CreateServiceError::InvalidParameter(ref cause) => cause,
            CreateServiceError::PlatformTaskDefinitionIncompatibility(ref cause) => cause,
            CreateServiceError::PlatformUnknown(ref cause) => cause,
            CreateServiceError::Server(ref cause) => cause,
            CreateServiceError::UnsupportedFeature(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAccountSetting
#[derive(Debug, PartialEq)]
pub enum DeleteAccountSettingError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DeleteAccountSettingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAccountSettingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteAccountSettingError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteAccountSettingError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(DeleteAccountSettingError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteAccountSettingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAccountSettingError {
    fn description(&self) -> &str {
        match *self {
            DeleteAccountSettingError::Client(ref cause) => cause,
            DeleteAccountSettingError::InvalidParameter(ref cause) => cause,
            DeleteAccountSettingError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAttributes
#[derive(Debug, PartialEq)]
pub enum DeleteAttributesError {
    /// <p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region-specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified target could not be found. You can view your available container instances with <a>ListContainerInstances</a>. Amazon ECS container instances are cluster-specific and Region-specific.</p>
    TargetNotFound(String),
}

impl DeleteAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAttributesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClusterNotFoundException" => {
                    return RusotoError::Service(DeleteAttributesError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteAttributesError::InvalidParameter(err.msg))
                }
                "TargetNotFoundException" => {
                    return RusotoError::Service(DeleteAttributesError::TargetNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAttributesError {
    fn description(&self) -> &str {
        match *self {
            DeleteAttributesError::ClusterNotFound(ref cause) => cause,
            DeleteAttributesError::InvalidParameter(ref cause) => cause,
            DeleteAttributesError::TargetNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCluster
#[derive(Debug, PartialEq)]
pub enum DeleteClusterError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>You cannot delete a cluster that has registered container instances. First, deregister the container instances before you can delete the cluster. For more information, see <a>DeregisterContainerInstance</a>.</p>
    ClusterContainsContainerInstances(String),
    /// <p>You cannot delete a cluster that contains services. First, update the service to reduce its desired task count to 0 and then delete the service. For more information, see <a>UpdateService</a> and <a>DeleteService</a>.</p>
    ClusterContainsServices(String),
    /// <p>You cannot delete a cluster that has active tasks.</p>
    ClusterContainsTasks(String),
    /// <p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region-specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DeleteClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteClusterError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteClusterError::Client(err.msg))
                }
                "ClusterContainsContainerInstancesException" => {
                    return RusotoError::Service(
                        DeleteClusterError::ClusterContainsContainerInstances(err.msg),
                    )
                }
                "ClusterContainsServicesException" => {
                    return RusotoError::Service(DeleteClusterError::ClusterContainsServices(
                        err.msg,
                    ))
                }
                "ClusterContainsTasksException" => {
                    return RusotoError::Service(DeleteClusterError::ClusterContainsTasks(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(DeleteClusterError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteClusterError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DeleteClusterError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteClusterError {
    fn description(&self) -> &str {
        match *self {
            DeleteClusterError::Client(ref cause) => cause,
            DeleteClusterError::ClusterContainsContainerInstances(ref cause) => cause,
            DeleteClusterError::ClusterContainsServices(ref cause) => cause,
            DeleteClusterError::ClusterContainsTasks(ref cause) => cause,
            DeleteClusterError::ClusterNotFound(ref cause) => cause,
            DeleteClusterError::InvalidParameter(ref cause) => cause,
            DeleteClusterError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteService
#[derive(Debug, PartialEq)]
pub enum DeleteServiceError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region-specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// <p>The specified service could not be found. You can view your available services with <a>ListServices</a>. Amazon ECS services are cluster-specific and Region-specific.</p>
    ServiceNotFound(String),
}

impl DeleteServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteServiceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeleteServiceError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(DeleteServiceError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteServiceError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DeleteServiceError::Server(err.msg))
                }
                "ServiceNotFoundException" => {
                    return RusotoError::Service(DeleteServiceError::ServiceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteServiceError {
    fn description(&self) -> &str {
        match *self {
            DeleteServiceError::Client(ref cause) => cause,
            DeleteServiceError::ClusterNotFound(ref cause) => cause,
            DeleteServiceError::InvalidParameter(ref cause) => cause,
            DeleteServiceError::Server(ref cause) => cause,
            DeleteServiceError::ServiceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterContainerInstance
#[derive(Debug, PartialEq)]
pub enum DeregisterContainerInstanceError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region-specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DeregisterContainerInstanceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeregisterContainerInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeregisterContainerInstanceError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(DeregisterContainerInstanceError::ClusterNotFound(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DeregisterContainerInstanceError::InvalidParameter(err.msg),
                    )
                }
                "ServerException" => {
                    return RusotoError::Service(DeregisterContainerInstanceError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeregisterContainerInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterContainerInstanceError {
    fn description(&self) -> &str {
        match *self {
            DeregisterContainerInstanceError::Client(ref cause) => cause,
            DeregisterContainerInstanceError::ClusterNotFound(ref cause) => cause,
            DeregisterContainerInstanceError::InvalidParameter(ref cause) => cause,
            DeregisterContainerInstanceError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by DeregisterTaskDefinition
#[derive(Debug, PartialEq)]
pub enum DeregisterTaskDefinitionError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DeregisterTaskDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeregisterTaskDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DeregisterTaskDefinitionError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeregisterTaskDefinitionError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(DeregisterTaskDefinitionError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeregisterTaskDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeregisterTaskDefinitionError {
    fn description(&self) -> &str {
        match *self {
            DeregisterTaskDefinitionError::Client(ref cause) => cause,
            DeregisterTaskDefinitionError::InvalidParameter(ref cause) => cause,
            DeregisterTaskDefinitionError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeClusters
#[derive(Debug, PartialEq)]
pub enum DescribeClustersError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DescribeClustersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeClustersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeClustersError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeClustersError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeClustersError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeClustersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeClustersError {
    fn description(&self) -> &str {
        match *self {
            DescribeClustersError::Client(ref cause) => cause,
            DescribeClustersError::InvalidParameter(ref cause) => cause,
            DescribeClustersError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeContainerInstances
#[derive(Debug, PartialEq)]
pub enum DescribeContainerInstancesError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region-specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DescribeContainerInstancesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeContainerInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeContainerInstancesError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(DescribeContainerInstancesError::ClusterNotFound(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeContainerInstancesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeContainerInstancesError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeContainerInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeContainerInstancesError {
    fn description(&self) -> &str {
        match *self {
            DescribeContainerInstancesError::Client(ref cause) => cause,
            DescribeContainerInstancesError::ClusterNotFound(ref cause) => cause,
            DescribeContainerInstancesError::InvalidParameter(ref cause) => cause,
            DescribeContainerInstancesError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeServices
#[derive(Debug, PartialEq)]
pub enum DescribeServicesError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region-specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DescribeServicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeServicesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeServicesError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(DescribeServicesError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeServicesError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeServicesError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeServicesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeServicesError {
    fn description(&self) -> &str {
        match *self {
            DescribeServicesError::Client(ref cause) => cause,
            DescribeServicesError::ClusterNotFound(ref cause) => cause,
            DescribeServicesError::InvalidParameter(ref cause) => cause,
            DescribeServicesError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTaskDefinition
#[derive(Debug, PartialEq)]
pub enum DescribeTaskDefinitionError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DescribeTaskDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTaskDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeTaskDefinitionError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeTaskDefinitionError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeTaskDefinitionError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeTaskDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTaskDefinitionError {
    fn description(&self) -> &str {
        match *self {
            DescribeTaskDefinitionError::Client(ref cause) => cause,
            DescribeTaskDefinitionError::InvalidParameter(ref cause) => cause,
            DescribeTaskDefinitionError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTasks
#[derive(Debug, PartialEq)]
pub enum DescribeTasksError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region-specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DescribeTasksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTasksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DescribeTasksError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(DescribeTasksError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeTasksError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DescribeTasksError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeTasksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTasksError {
    fn description(&self) -> &str {
        match *self {
            DescribeTasksError::Client(ref cause) => cause,
            DescribeTasksError::ClusterNotFound(ref cause) => cause,
            DescribeTasksError::InvalidParameter(ref cause) => cause,
            DescribeTasksError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by DiscoverPollEndpoint
#[derive(Debug, PartialEq)]
pub enum DiscoverPollEndpointError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl DiscoverPollEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DiscoverPollEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(DiscoverPollEndpointError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(DiscoverPollEndpointError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DiscoverPollEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DiscoverPollEndpointError {
    fn description(&self) -> &str {
        match *self {
            DiscoverPollEndpointError::Client(ref cause) => cause,
            DiscoverPollEndpointError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAccountSettings
#[derive(Debug, PartialEq)]
pub enum ListAccountSettingsError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl ListAccountSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAccountSettingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListAccountSettingsError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListAccountSettingsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(ListAccountSettingsError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAccountSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAccountSettingsError {
    fn description(&self) -> &str {
        match *self {
            ListAccountSettingsError::Client(ref cause) => cause,
            ListAccountSettingsError::InvalidParameter(ref cause) => cause,
            ListAccountSettingsError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAttributes
#[derive(Debug, PartialEq)]
pub enum ListAttributesError {
    /// <p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region-specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
}

impl ListAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAttributesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClusterNotFoundException" => {
                    return RusotoError::Service(ListAttributesError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListAttributesError::InvalidParameter(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAttributesError {
    fn description(&self) -> &str {
        match *self {
            ListAttributesError::ClusterNotFound(ref cause) => cause,
            ListAttributesError::InvalidParameter(ref cause) => cause,
        }
    }
}
/// Errors returned by ListClusters
#[derive(Debug, PartialEq)]
pub enum ListClustersError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl ListClustersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListClustersError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListClustersError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListClustersError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(ListClustersError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListClustersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListClustersError {
    fn description(&self) -> &str {
        match *self {
            ListClustersError::Client(ref cause) => cause,
            ListClustersError::InvalidParameter(ref cause) => cause,
            ListClustersError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by ListContainerInstances
#[derive(Debug, PartialEq)]
pub enum ListContainerInstancesError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region-specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl ListContainerInstancesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListContainerInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListContainerInstancesError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(ListContainerInstancesError::ClusterNotFound(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListContainerInstancesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(ListContainerInstancesError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListContainerInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListContainerInstancesError {
    fn description(&self) -> &str {
        match *self {
            ListContainerInstancesError::Client(ref cause) => cause,
            ListContainerInstancesError::ClusterNotFound(ref cause) => cause,
            ListContainerInstancesError::InvalidParameter(ref cause) => cause,
            ListContainerInstancesError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by ListServices
#[derive(Debug, PartialEq)]
pub enum ListServicesError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region-specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl ListServicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListServicesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListServicesError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(ListServicesError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListServicesError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(ListServicesError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListServicesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListServicesError {
    fn description(&self) -> &str {
        match *self {
            ListServicesError::Client(ref cause) => cause,
            ListServicesError::ClusterNotFound(ref cause) => cause,
            ListServicesError::InvalidParameter(ref cause) => cause,
            ListServicesError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region-specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListTagsForResourceError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(ListTagsForResourceError::Server(err.msg))
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
            ListTagsForResourceError::Client(ref cause) => cause,
            ListTagsForResourceError::ClusterNotFound(ref cause) => cause,
            ListTagsForResourceError::InvalidParameter(ref cause) => cause,
            ListTagsForResourceError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTaskDefinitionFamilies
#[derive(Debug, PartialEq)]
pub enum ListTaskDefinitionFamiliesError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl ListTaskDefinitionFamiliesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListTaskDefinitionFamiliesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListTaskDefinitionFamiliesError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListTaskDefinitionFamiliesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(ListTaskDefinitionFamiliesError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTaskDefinitionFamiliesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTaskDefinitionFamiliesError {
    fn description(&self) -> &str {
        match *self {
            ListTaskDefinitionFamiliesError::Client(ref cause) => cause,
            ListTaskDefinitionFamiliesError::InvalidParameter(ref cause) => cause,
            ListTaskDefinitionFamiliesError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTaskDefinitions
#[derive(Debug, PartialEq)]
pub enum ListTaskDefinitionsError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl ListTaskDefinitionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTaskDefinitionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(ListTaskDefinitionsError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListTaskDefinitionsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(ListTaskDefinitionsError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTaskDefinitionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTaskDefinitionsError {
    fn description(&self) -> &str {
        match *self {
            ListTaskDefinitionsError::Client(ref cause) => cause,
            ListTaskDefinitionsError::InvalidParameter(ref cause) => cause,
            ListTaskDefinitionsError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTasks
#[derive(Debug, PartialEq)]
pub enum ListTasksError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region-specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// <p>The specified service could not be found. You can view your available services with <a>ListServices</a>. Amazon ECS services are cluster-specific and Region-specific.</p>
    ServiceNotFound(String),
}

impl ListTasksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTasksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => return RusotoError::Service(ListTasksError::Client(err.msg)),
                "ClusterNotFoundException" => {
                    return RusotoError::Service(ListTasksError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListTasksError::InvalidParameter(err.msg))
                }
                "ServerException" => return RusotoError::Service(ListTasksError::Server(err.msg)),
                "ServiceNotFoundException" => {
                    return RusotoError::Service(ListTasksError::ServiceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTasksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTasksError {
    fn description(&self) -> &str {
        match *self {
            ListTasksError::Client(ref cause) => cause,
            ListTasksError::ClusterNotFound(ref cause) => cause,
            ListTasksError::InvalidParameter(ref cause) => cause,
            ListTasksError::Server(ref cause) => cause,
            ListTasksError::ServiceNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by PutAccountSetting
#[derive(Debug, PartialEq)]
pub enum PutAccountSettingError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl PutAccountSettingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutAccountSettingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(PutAccountSettingError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(PutAccountSettingError::InvalidParameter(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(PutAccountSettingError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutAccountSettingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutAccountSettingError {
    fn description(&self) -> &str {
        match *self {
            PutAccountSettingError::Client(ref cause) => cause,
            PutAccountSettingError::InvalidParameter(ref cause) => cause,
            PutAccountSettingError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by PutAccountSettingDefault
#[derive(Debug, PartialEq)]
pub enum PutAccountSettingDefaultError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl PutAccountSettingDefaultError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutAccountSettingDefaultError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(PutAccountSettingDefaultError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(PutAccountSettingDefaultError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(PutAccountSettingDefaultError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutAccountSettingDefaultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutAccountSettingDefaultError {
    fn description(&self) -> &str {
        match *self {
            PutAccountSettingDefaultError::Client(ref cause) => cause,
            PutAccountSettingDefaultError::InvalidParameter(ref cause) => cause,
            PutAccountSettingDefaultError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by PutAttributes
#[derive(Debug, PartialEq)]
pub enum PutAttributesError {
    /// <p>You can apply up to 10 custom attributes per resource. You can view the attributes of a resource with <a>ListAttributes</a>. You can remove existing attributes on a resource with <a>DeleteAttributes</a>.</p>
    AttributeLimitExceeded(String),
    /// <p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region-specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified target could not be found. You can view your available container instances with <a>ListContainerInstances</a>. Amazon ECS container instances are cluster-specific and Region-specific.</p>
    TargetNotFound(String),
}

impl PutAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutAttributesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AttributeLimitExceededException" => {
                    return RusotoError::Service(PutAttributesError::AttributeLimitExceeded(
                        err.msg,
                    ))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(PutAttributesError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(PutAttributesError::InvalidParameter(err.msg))
                }
                "TargetNotFoundException" => {
                    return RusotoError::Service(PutAttributesError::TargetNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutAttributesError {
    fn description(&self) -> &str {
        match *self {
            PutAttributesError::AttributeLimitExceeded(ref cause) => cause,
            PutAttributesError::ClusterNotFound(ref cause) => cause,
            PutAttributesError::InvalidParameter(ref cause) => cause,
            PutAttributesError::TargetNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterContainerInstance
#[derive(Debug, PartialEq)]
pub enum RegisterContainerInstanceError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl RegisterContainerInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterContainerInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(RegisterContainerInstanceError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RegisterContainerInstanceError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(RegisterContainerInstanceError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RegisterContainerInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterContainerInstanceError {
    fn description(&self) -> &str {
        match *self {
            RegisterContainerInstanceError::Client(ref cause) => cause,
            RegisterContainerInstanceError::InvalidParameter(ref cause) => cause,
            RegisterContainerInstanceError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterTaskDefinition
#[derive(Debug, PartialEq)]
pub enum RegisterTaskDefinitionError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl RegisterTaskDefinitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterTaskDefinitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(RegisterTaskDefinitionError::Client(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RegisterTaskDefinitionError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(RegisterTaskDefinitionError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RegisterTaskDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterTaskDefinitionError {
    fn description(&self) -> &str {
        match *self {
            RegisterTaskDefinitionError::Client(ref cause) => cause,
            RegisterTaskDefinitionError::InvalidParameter(ref cause) => cause,
            RegisterTaskDefinitionError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by RunTask
#[derive(Debug, PartialEq)]
pub enum RunTaskError {
    /// <p>You do not have authorization to perform the requested action.</p>
    AccessDenied(String),
    /// <p>Your AWS account has been blocked. For more information, contact <a href="http://aws.amazon.com/contact-us/">AWS Support</a>.</p>
    Blocked(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region-specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified platform version does not satisfy the task definition's required capabilities.</p>
    PlatformTaskDefinitionIncompatibility(String),
    /// <p>The specified platform version does not exist.</p>
    PlatformUnknown(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// <p>The specified task is not supported in this Region.</p>
    UnsupportedFeature(String),
}

impl RunTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RunTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(RunTaskError::AccessDenied(err.msg))
                }
                "BlockedException" => return RusotoError::Service(RunTaskError::Blocked(err.msg)),
                "ClientException" => return RusotoError::Service(RunTaskError::Client(err.msg)),
                "ClusterNotFoundException" => {
                    return RusotoError::Service(RunTaskError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RunTaskError::InvalidParameter(err.msg))
                }
                "PlatformTaskDefinitionIncompatibilityException" => {
                    return RusotoError::Service(
                        RunTaskError::PlatformTaskDefinitionIncompatibility(err.msg),
                    )
                }
                "PlatformUnknownException" => {
                    return RusotoError::Service(RunTaskError::PlatformUnknown(err.msg))
                }
                "ServerException" => return RusotoError::Service(RunTaskError::Server(err.msg)),
                "UnsupportedFeatureException" => {
                    return RusotoError::Service(RunTaskError::UnsupportedFeature(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RunTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RunTaskError {
    fn description(&self) -> &str {
        match *self {
            RunTaskError::AccessDenied(ref cause) => cause,
            RunTaskError::Blocked(ref cause) => cause,
            RunTaskError::Client(ref cause) => cause,
            RunTaskError::ClusterNotFound(ref cause) => cause,
            RunTaskError::InvalidParameter(ref cause) => cause,
            RunTaskError::PlatformTaskDefinitionIncompatibility(ref cause) => cause,
            RunTaskError::PlatformUnknown(ref cause) => cause,
            RunTaskError::Server(ref cause) => cause,
            RunTaskError::UnsupportedFeature(ref cause) => cause,
        }
    }
}
/// Errors returned by StartTask
#[derive(Debug, PartialEq)]
pub enum StartTaskError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region-specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl StartTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => return RusotoError::Service(StartTaskError::Client(err.msg)),
                "ClusterNotFoundException" => {
                    return RusotoError::Service(StartTaskError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartTaskError::InvalidParameter(err.msg))
                }
                "ServerException" => return RusotoError::Service(StartTaskError::Server(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartTaskError {
    fn description(&self) -> &str {
        match *self {
            StartTaskError::Client(ref cause) => cause,
            StartTaskError::ClusterNotFound(ref cause) => cause,
            StartTaskError::InvalidParameter(ref cause) => cause,
            StartTaskError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by StopTask
#[derive(Debug, PartialEq)]
pub enum StopTaskError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region-specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl StopTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => return RusotoError::Service(StopTaskError::Client(err.msg)),
                "ClusterNotFoundException" => {
                    return RusotoError::Service(StopTaskError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StopTaskError::InvalidParameter(err.msg))
                }
                "ServerException" => return RusotoError::Service(StopTaskError::Server(err.msg)),
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopTaskError {
    fn description(&self) -> &str {
        match *self {
            StopTaskError::Client(ref cause) => cause,
            StopTaskError::ClusterNotFound(ref cause) => cause,
            StopTaskError::InvalidParameter(ref cause) => cause,
            StopTaskError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by SubmitContainerStateChange
#[derive(Debug, PartialEq)]
pub enum SubmitContainerStateChangeError {
    /// <p>You do not have authorization to perform the requested action.</p>
    AccessDenied(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl SubmitContainerStateChangeError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<SubmitContainerStateChangeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(SubmitContainerStateChangeError::AccessDenied(
                        err.msg,
                    ))
                }
                "ClientException" => {
                    return RusotoError::Service(SubmitContainerStateChangeError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(SubmitContainerStateChangeError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SubmitContainerStateChangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SubmitContainerStateChangeError {
    fn description(&self) -> &str {
        match *self {
            SubmitContainerStateChangeError::AccessDenied(ref cause) => cause,
            SubmitContainerStateChangeError::Client(ref cause) => cause,
            SubmitContainerStateChangeError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by SubmitTaskStateChange
#[derive(Debug, PartialEq)]
pub enum SubmitTaskStateChangeError {
    /// <p>You do not have authorization to perform the requested action.</p>
    AccessDenied(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl SubmitTaskStateChangeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SubmitTaskStateChangeError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(SubmitTaskStateChangeError::AccessDenied(err.msg))
                }
                "ClientException" => {
                    return RusotoError::Service(SubmitTaskStateChangeError::Client(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(SubmitTaskStateChangeError::Server(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SubmitTaskStateChangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SubmitTaskStateChangeError {
    fn description(&self) -> &str {
        match *self {
            SubmitTaskStateChangeError::AccessDenied(ref cause) => cause,
            SubmitTaskStateChangeError::Client(ref cause) => cause,
            SubmitTaskStateChangeError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region-specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(TagResourceError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(TagResourceError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(TagResourceError::Server(err.msg))
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
            TagResourceError::Client(ref cause) => cause,
            TagResourceError::ClusterNotFound(ref cause) => cause,
            TagResourceError::InvalidParameter(ref cause) => cause,
            TagResourceError::ResourceNotFound(ref cause) => cause,
            TagResourceError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region-specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UntagResourceError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UntagResourceError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(UntagResourceError::Server(err.msg))
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
            UntagResourceError::Client(ref cause) => cause,
            UntagResourceError::ClusterNotFound(ref cause) => cause,
            UntagResourceError::InvalidParameter(ref cause) => cause,
            UntagResourceError::ResourceNotFound(ref cause) => cause,
            UntagResourceError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateContainerAgent
#[derive(Debug, PartialEq)]
pub enum UpdateContainerAgentError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region-specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>Amazon ECS is unable to determine the current version of the Amazon ECS container agent on the container instance and does not have enough information to proceed with an update. This could be because the agent running on the container instance is an older or custom version that does not use our version information.</p>
    MissingVersion(String),
    /// <p>There is no update available for this Amazon ECS container agent. This could be because the agent is already running the latest version, or it is so old that there is no update path to the current version.</p>
    NoUpdateAvailable(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// <p>There is already a current Amazon ECS container agent update in progress on the specified container instance. If the container agent becomes disconnected while it is in a transitional stage, such as <code>PENDING</code> or <code>STAGING</code>, the update process can get stuck in that state. However, when the agent reconnects, it resumes where it stopped previously.</p>
    UpdateInProgress(String),
}

impl UpdateContainerAgentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateContainerAgentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateContainerAgentError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(UpdateContainerAgentError::ClusterNotFound(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateContainerAgentError::InvalidParameter(
                        err.msg,
                    ))
                }
                "MissingVersionException" => {
                    return RusotoError::Service(UpdateContainerAgentError::MissingVersion(err.msg))
                }
                "NoUpdateAvailableException" => {
                    return RusotoError::Service(UpdateContainerAgentError::NoUpdateAvailable(
                        err.msg,
                    ))
                }
                "ServerException" => {
                    return RusotoError::Service(UpdateContainerAgentError::Server(err.msg))
                }
                "UpdateInProgressException" => {
                    return RusotoError::Service(UpdateContainerAgentError::UpdateInProgress(
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
impl fmt::Display for UpdateContainerAgentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateContainerAgentError {
    fn description(&self) -> &str {
        match *self {
            UpdateContainerAgentError::Client(ref cause) => cause,
            UpdateContainerAgentError::ClusterNotFound(ref cause) => cause,
            UpdateContainerAgentError::InvalidParameter(ref cause) => cause,
            UpdateContainerAgentError::MissingVersion(ref cause) => cause,
            UpdateContainerAgentError::NoUpdateAvailable(ref cause) => cause,
            UpdateContainerAgentError::Server(ref cause) => cause,
            UpdateContainerAgentError::UpdateInProgress(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateContainerInstancesState
#[derive(Debug, PartialEq)]
pub enum UpdateContainerInstancesStateError {
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region-specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
}

impl UpdateContainerInstancesStateError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateContainerInstancesStateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ClientException" => {
                    return RusotoError::Service(UpdateContainerInstancesStateError::Client(
                        err.msg,
                    ))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(
                        UpdateContainerInstancesStateError::ClusterNotFound(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        UpdateContainerInstancesStateError::InvalidParameter(err.msg),
                    )
                }
                "ServerException" => {
                    return RusotoError::Service(UpdateContainerInstancesStateError::Server(
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
impl fmt::Display for UpdateContainerInstancesStateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateContainerInstancesStateError {
    fn description(&self) -> &str {
        match *self {
            UpdateContainerInstancesStateError::Client(ref cause) => cause,
            UpdateContainerInstancesStateError::ClusterNotFound(ref cause) => cause,
            UpdateContainerInstancesStateError::InvalidParameter(ref cause) => cause,
            UpdateContainerInstancesStateError::Server(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateService
#[derive(Debug, PartialEq)]
pub enum UpdateServiceError {
    /// <p>You do not have authorization to perform the requested action.</p>
    AccessDenied(String),
    /// <p>These errors are usually caused by a client action, such as using an action or resource on behalf of a user that doesn't have permissions to use the action or resource, or specifying an identifier that is not valid.</p>
    Client(String),
    /// <p>The specified cluster could not be found. You can view your available clusters with <a>ListClusters</a>. Amazon ECS clusters are Region-specific.</p>
    ClusterNotFound(String),
    /// <p>The specified parameter is invalid. Review the available parameters for the API request.</p>
    InvalidParameter(String),
    /// <p>The specified platform version does not satisfy the task definition's required capabilities.</p>
    PlatformTaskDefinitionIncompatibility(String),
    /// <p>The specified platform version does not exist.</p>
    PlatformUnknown(String),
    /// <p>These errors are usually caused by a server issue.</p>
    Server(String),
    /// <p>The specified service is not active. You can't update a service that is inactive. If you have previously deleted a service, you can re-create it with <a>CreateService</a>.</p>
    ServiceNotActive(String),
    /// <p>The specified service could not be found. You can view your available services with <a>ListServices</a>. Amazon ECS services are cluster-specific and Region-specific.</p>
    ServiceNotFound(String),
}

impl UpdateServiceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateServiceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateServiceError::AccessDenied(err.msg))
                }
                "ClientException" => {
                    return RusotoError::Service(UpdateServiceError::Client(err.msg))
                }
                "ClusterNotFoundException" => {
                    return RusotoError::Service(UpdateServiceError::ClusterNotFound(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateServiceError::InvalidParameter(err.msg))
                }
                "PlatformTaskDefinitionIncompatibilityException" => {
                    return RusotoError::Service(
                        UpdateServiceError::PlatformTaskDefinitionIncompatibility(err.msg),
                    )
                }
                "PlatformUnknownException" => {
                    return RusotoError::Service(UpdateServiceError::PlatformUnknown(err.msg))
                }
                "ServerException" => {
                    return RusotoError::Service(UpdateServiceError::Server(err.msg))
                }
                "ServiceNotActiveException" => {
                    return RusotoError::Service(UpdateServiceError::ServiceNotActive(err.msg))
                }
                "ServiceNotFoundException" => {
                    return RusotoError::Service(UpdateServiceError::ServiceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateServiceError {
    fn description(&self) -> &str {
        match *self {
            UpdateServiceError::AccessDenied(ref cause) => cause,
            UpdateServiceError::Client(ref cause) => cause,
            UpdateServiceError::ClusterNotFound(ref cause) => cause,
            UpdateServiceError::InvalidParameter(ref cause) => cause,
            UpdateServiceError::PlatformTaskDefinitionIncompatibility(ref cause) => cause,
            UpdateServiceError::PlatformUnknown(ref cause) => cause,
            UpdateServiceError::Server(ref cause) => cause,
            UpdateServiceError::ServiceNotActive(ref cause) => cause,
            UpdateServiceError::ServiceNotFound(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon ECS API. Amazon ECS clients implement this trait.
pub trait Ecs {
    /// <p><p>Creates a new Amazon ECS cluster. By default, your account receives a <code>default</code> cluster when you launch your first container instance. However, you can create your own cluster with a unique name with the <code>CreateCluster</code> action.</p> <note> <p>When you call the <a>CreateCluster</a> API operation, Amazon ECS attempts to create the service-linked role for your account so that required resources in other AWS services can be managed on your behalf. However, if the IAM user that makes the call does not have permissions to create the service-linked role, it is not created. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/using-service-linked-roles.html">Using Service-Linked Roles for Amazon ECS</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </note></p>
    fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> RusotoFuture<CreateClusterResponse, CreateClusterError>;

    /// <p><p>Runs and maintains a desired number of tasks from a specified task definition. If the number of tasks running in a service drops below <code>desiredCount</code>, Amazon ECS spawns another copy of the task in the specified cluster. To update an existing service, see <a>UpdateService</a>.</p> <p>In addition to maintaining the desired count of tasks in your service, you can optionally run your service behind a load balancer. The load balancer distributes traffic across the tasks that are associated with the service. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-load-balancing.html">Service Load Balancing</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>You can optionally specify a deployment configuration for your service. The deployment is triggered by changing properties, such as the task definition or the desired count of a service, with an <a>UpdateService</a> operation.</p> <p>If a service is using the <code>ECS</code> deployment controller, the <b>minimum healthy percent</b> represents a lower limit on the number of tasks in a service that must remain in the <code>RUNNING</code> state during a deployment, as a percentage of the desired number of tasks (rounded up to the nearest integer), and while any container instances are in the <code>DRAINING</code> state if the service contains tasks using the EC2 launch type. This parameter enables you to deploy without using additional cluster capacity. For example, if your service has a desired number of four tasks and a minimum healthy percent of 50%, the scheduler may stop two existing tasks to free up cluster capacity before starting two new tasks. Tasks for services that <i>do not</i> use a load balancer are considered healthy if they are in the <code>RUNNING</code> state; tasks for services that <i>do</i> use a load balancer are considered healthy if they are in the <code>RUNNING</code> state and they are reported as healthy by the load balancer. The default value for minimum healthy percent is 100%.</p> <p>If a service is using the <code>ECS</code> deployment controller, the <b>maximum percent</b> parameter represents an upper limit on the number of tasks in a service that are allowed in the <code>RUNNING</code> or <code>PENDING</code> state during a deployment, as a percentage of the desired number of tasks (rounded down to the nearest integer), and while any container instances are in the <code>DRAINING</code> state if the service contains tasks using the EC2 launch type. This parameter enables you to define the deployment batch size. For example, if your service has a desired number of four tasks and a maximum percent value of 200%, the scheduler may start four new tasks before stopping the four older tasks (provided that the cluster resources required to do this are available). The default value for maximum percent is 200%.</p> <p>If a service is using the <code>CODE_DEPLOY</code> deployment controller and tasks that use the EC2 launch type, the <b>minimum healthy percent</b> and <b>maximum percent</b> values are only used to define the lower and upper limit on the number of the tasks in the service that remain in the <code>RUNNING</code> state while the container instances are in the <code>DRAINING</code> state. If the tasks in the service use the Fargate launch type, the minimum healthy percent and maximum percent values are not used, although they are currently visible when describing your service.</p> <p>Tasks for services that <i>do not</i> use a load balancer are considered healthy if they are in the <code>RUNNING</code> state. Tasks for services that <i>do</i> use a load balancer are considered healthy if they are in the <code>RUNNING</code> state and the container instance they are hosted on is reported as healthy by the load balancer. The default value for a replica service for <code>minimumHealthyPercent</code> is 100%. The default value for a daemon service for <code>minimumHealthyPercent</code> is 0%.</p> <p>When the service scheduler launches new tasks, it determines task placement in your cluster using the following logic:</p> <ul> <li> <p>Determine which of the container instances in your cluster can support your service&#39;s task definition (for example, they have the required CPU, memory, ports, and container instance attributes).</p> </li> <li> <p>By default, the service scheduler attempts to balance tasks across Availability Zones in this manner (although you can choose a different placement strategy) with the <code>placementStrategy</code> parameter):</p> <ul> <li> <p>Sort the valid container instances, giving priority to instances that have the fewest number of running tasks for this service in their respective Availability Zone. For example, if zone A has one running service task and zones B and C each have zero, valid container instances in either zone B or C are considered optimal for placement.</p> </li> <li> <p>Place the new service task on a valid container instance in an optimal Availability Zone (based on the previous steps), favoring container instances with the fewest number of running tasks for this service.</p> </li> </ul> </li> </ul></p>
    fn create_service(
        &self,
        input: CreateServiceRequest,
    ) -> RusotoFuture<CreateServiceResponse, CreateServiceError>;

    /// <p>Modifies the ARN and resource ID format of a resource for a specified IAM user, IAM role, or the root user for an account. You can specify whether the new ARN and resource ID format are disabled for new resources that are created.</p>
    fn delete_account_setting(
        &self,
        input: DeleteAccountSettingRequest,
    ) -> RusotoFuture<DeleteAccountSettingResponse, DeleteAccountSettingError>;

    /// <p>Deletes one or more custom attributes from an Amazon ECS resource.</p>
    fn delete_attributes(
        &self,
        input: DeleteAttributesRequest,
    ) -> RusotoFuture<DeleteAttributesResponse, DeleteAttributesError>;

    /// <p>Deletes the specified cluster. You must deregister all container instances from this cluster before you may delete it. You can list the container instances in a cluster with <a>ListContainerInstances</a> and deregister them with <a>DeregisterContainerInstance</a>.</p>
    fn delete_cluster(
        &self,
        input: DeleteClusterRequest,
    ) -> RusotoFuture<DeleteClusterResponse, DeleteClusterError>;

    /// <p><p>Deletes a specified service within a cluster. You can delete a service if you have no running tasks in it and the desired task count is zero. If the service is actively maintaining tasks, you cannot delete it, and you must update the service to a desired task count of zero. For more information, see <a>UpdateService</a>.</p> <note> <p>When you delete a service, if there are still running tasks that require cleanup, the service status moves from <code>ACTIVE</code> to <code>DRAINING</code>, and the service is no longer visible in the console or in the <a>ListServices</a> API operation. After the tasks have stopped, then the service status moves from <code>DRAINING</code> to <code>INACTIVE</code>. Services in the <code>DRAINING</code> or <code>INACTIVE</code> status can still be viewed with the <a>DescribeServices</a> API operation. However, in the future, <code>INACTIVE</code> services may be cleaned up and purged from Amazon ECS record keeping, and <a>DescribeServices</a> calls on those services return a <code>ServiceNotFoundException</code> error.</p> </note> <important> <p>If you attempt to create a new service with the same name as an existing service in either <code>ACTIVE</code> or <code>DRAINING</code> status, you receive an error.</p> </important></p>
    fn delete_service(
        &self,
        input: DeleteServiceRequest,
    ) -> RusotoFuture<DeleteServiceResponse, DeleteServiceError>;

    /// <p><p>Deregisters an Amazon ECS container instance from the specified cluster. This instance is no longer available to run tasks.</p> <p>If you intend to use the container instance for some other purpose after deregistration, you should stop all of the tasks running on the container instance before deregistration. That prevents any orphaned tasks from consuming resources.</p> <p>Deregistering a container instance removes the instance from a cluster, but it does not terminate the EC2 instance. If you are finished using the instance, be sure to terminate it in the Amazon EC2 console to stop billing.</p> <note> <p>If you terminate a running container instance, Amazon ECS automatically deregisters the instance from your cluster (stopped container instances or instances with disconnected agents are not automatically deregistered when terminated).</p> </note></p>
    fn deregister_container_instance(
        &self,
        input: DeregisterContainerInstanceRequest,
    ) -> RusotoFuture<DeregisterContainerInstanceResponse, DeregisterContainerInstanceError>;

    /// <p><p>Deregisters the specified task definition by family and revision. Upon deregistration, the task definition is marked as <code>INACTIVE</code>. Existing tasks and services that reference an <code>INACTIVE</code> task definition continue to run without disruption. Existing services that reference an <code>INACTIVE</code> task definition can still scale up or down by modifying the service&#39;s desired count.</p> <p>You cannot use an <code>INACTIVE</code> task definition to run new tasks or create new services, and you cannot update an existing service to reference an <code>INACTIVE</code> task definition. However, there may be up to a 10-minute window following deregistration where these restrictions have not yet taken effect.</p> <note> <p>At this time, <code>INACTIVE</code> task definitions remain discoverable in your account indefinitely. However, this behavior is subject to change in the future, so you should not rely on <code>INACTIVE</code> task definitions persisting beyond the lifecycle of any associated tasks and services.</p> </note></p>
    fn deregister_task_definition(
        &self,
        input: DeregisterTaskDefinitionRequest,
    ) -> RusotoFuture<DeregisterTaskDefinitionResponse, DeregisterTaskDefinitionError>;

    /// <p>Describes one or more of your clusters.</p>
    fn describe_clusters(
        &self,
        input: DescribeClustersRequest,
    ) -> RusotoFuture<DescribeClustersResponse, DescribeClustersError>;

    /// <p>Describes Amazon Elastic Container Service container instances. Returns metadata about registered and remaining resources on each container instance requested.</p>
    fn describe_container_instances(
        &self,
        input: DescribeContainerInstancesRequest,
    ) -> RusotoFuture<DescribeContainerInstancesResponse, DescribeContainerInstancesError>;

    /// <p>Describes the specified services running in your cluster.</p>
    fn describe_services(
        &self,
        input: DescribeServicesRequest,
    ) -> RusotoFuture<DescribeServicesResponse, DescribeServicesError>;

    /// <p><p>Describes a task definition. You can specify a <code>family</code> and <code>revision</code> to find information about a specific task definition, or you can simply specify the family to find the latest <code>ACTIVE</code> revision in that family.</p> <note> <p>You can only describe <code>INACTIVE</code> task definitions while an active task or service references them.</p> </note></p>
    fn describe_task_definition(
        &self,
        input: DescribeTaskDefinitionRequest,
    ) -> RusotoFuture<DescribeTaskDefinitionResponse, DescribeTaskDefinitionError>;

    /// <p>Describes a specified task or tasks.</p>
    fn describe_tasks(
        &self,
        input: DescribeTasksRequest,
    ) -> RusotoFuture<DescribeTasksResponse, DescribeTasksError>;

    /// <p><note> <p>This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent.</p> </note> <p>Returns an endpoint for the Amazon ECS agent to poll for updates.</p></p>
    fn discover_poll_endpoint(
        &self,
        input: DiscoverPollEndpointRequest,
    ) -> RusotoFuture<DiscoverPollEndpointResponse, DiscoverPollEndpointError>;

    /// <p>Lists the account settings for an Amazon ECS resource for a specified principal.</p>
    fn list_account_settings(
        &self,
        input: ListAccountSettingsRequest,
    ) -> RusotoFuture<ListAccountSettingsResponse, ListAccountSettingsError>;

    /// <p>Lists the attributes for Amazon ECS resources within a specified target type and cluster. When you specify a target type and cluster, <code>ListAttributes</code> returns a list of attribute objects, one for each attribute on each resource. You can filter the list of results to a single attribute name to only return results that have that name. You can also filter the results by attribute name and value, for example, to see which container instances in a cluster are running a Linux AMI (<code>ecs.os-type=linux</code>). </p>
    fn list_attributes(
        &self,
        input: ListAttributesRequest,
    ) -> RusotoFuture<ListAttributesResponse, ListAttributesError>;

    /// <p>Returns a list of existing clusters.</p>
    fn list_clusters(
        &self,
        input: ListClustersRequest,
    ) -> RusotoFuture<ListClustersResponse, ListClustersError>;

    /// <p>Returns a list of container instances in a specified cluster. You can filter the results of a <code>ListContainerInstances</code> operation with cluster query language statements inside the <code>filter</code> parameter. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html">Cluster Query Language</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    fn list_container_instances(
        &self,
        input: ListContainerInstancesRequest,
    ) -> RusotoFuture<ListContainerInstancesResponse, ListContainerInstancesError>;

    /// <p>Lists the services that are running in a specified cluster.</p>
    fn list_services(
        &self,
        input: ListServicesRequest,
    ) -> RusotoFuture<ListServicesResponse, ListServicesError>;

    /// <p>List the tags for an Amazon ECS resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError>;

    /// <p>Returns a list of task definition families that are registered to your account (which may include task definition families that no longer have any <code>ACTIVE</code> task definition revisions).</p> <p>You can filter out task definition families that do not contain any <code>ACTIVE</code> task definition revisions by setting the <code>status</code> parameter to <code>ACTIVE</code>. You can also filter the results with the <code>familyPrefix</code> parameter.</p>
    fn list_task_definition_families(
        &self,
        input: ListTaskDefinitionFamiliesRequest,
    ) -> RusotoFuture<ListTaskDefinitionFamiliesResponse, ListTaskDefinitionFamiliesError>;

    /// <p>Returns a list of task definitions that are registered to your account. You can filter the results by family name with the <code>familyPrefix</code> parameter or by status with the <code>status</code> parameter.</p>
    fn list_task_definitions(
        &self,
        input: ListTaskDefinitionsRequest,
    ) -> RusotoFuture<ListTaskDefinitionsResponse, ListTaskDefinitionsError>;

    /// <p>Returns a list of tasks for a specified cluster. You can filter the results by family name, by a particular container instance, or by the desired status of the task with the <code>family</code>, <code>containerInstance</code>, and <code>desiredStatus</code> parameters.</p> <p>Recently stopped tasks might appear in the returned results. Currently, stopped tasks appear in the returned results for at least one hour. </p>
    fn list_tasks(
        &self,
        input: ListTasksRequest,
    ) -> RusotoFuture<ListTasksResponse, ListTasksError>;

    /// <p>Modifies the ARN and resource ID format of a resource type for a specified IAM user, IAM role, or the root user for an account. If the account setting for the root user is changed, it sets the default setting for all of the IAM users and roles for which no individual account setting has been set. The opt-in and opt-out account setting can be set for each Amazon ECS resource separately. The ARN and resource ID format of a resource will be defined by the opt-in status of the IAM user or role that created the resource. Enabling this setting is required to use new Amazon ECS features such as resource tagging. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-resource-ids.html">Amazon Resource Names (ARNs) and IDs</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    fn put_account_setting(
        &self,
        input: PutAccountSettingRequest,
    ) -> RusotoFuture<PutAccountSettingResponse, PutAccountSettingError>;

    /// <p>Modifies the ARN and resource ID format of a resource type for all IAM users on an account for which no individual account setting has been set. Enabling this setting is required to use new Amazon ECS features such as resource tagging.</p>
    fn put_account_setting_default(
        &self,
        input: PutAccountSettingDefaultRequest,
    ) -> RusotoFuture<PutAccountSettingDefaultResponse, PutAccountSettingDefaultError>;

    /// <p>Create or update an attribute on an Amazon ECS resource. If the attribute does not exist, it is created. If the attribute exists, its value is replaced with the specified value. To delete an attribute, use <a>DeleteAttributes</a>. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-placement-constraints.html#attributes">Attributes</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    fn put_attributes(
        &self,
        input: PutAttributesRequest,
    ) -> RusotoFuture<PutAttributesResponse, PutAttributesError>;

    /// <p><note> <p>This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent.</p> </note> <p>Registers an EC2 instance into the specified cluster. This instance becomes available to place containers on.</p></p>
    fn register_container_instance(
        &self,
        input: RegisterContainerInstanceRequest,
    ) -> RusotoFuture<RegisterContainerInstanceResponse, RegisterContainerInstanceError>;

    /// <p>Registers a new task definition from the supplied <code>family</code> and <code>containerDefinitions</code>. Optionally, you can add data volumes to your containers with the <code>volumes</code> parameter. For more information about task definition parameters and defaults, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_defintions.html">Amazon ECS Task Definitions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>You can specify an IAM role for your task with the <code>taskRoleArn</code> parameter. When you specify an IAM role for a task, its containers can then use the latest versions of the AWS CLI or SDKs to make API requests to the AWS services that are specified in the IAM policy associated with the role. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-iam-roles.html">IAM Roles for Tasks</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>You can specify a Docker networking mode for the containers in your task definition with the <code>networkMode</code> parameter. The available network modes correspond to those described in <a href="https://docs.docker.com/engine/reference/run/#/network-settings">Network settings</a> in the Docker run reference. If you specify the <code>awsvpc</code> network mode, the task is allocated an elastic network interface, and you must specify a <a>NetworkConfiguration</a> when you create a service or run a task with the task definition. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-networking.html">Task Networking</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    fn register_task_definition(
        &self,
        input: RegisterTaskDefinitionRequest,
    ) -> RusotoFuture<RegisterTaskDefinitionResponse, RegisterTaskDefinitionError>;

    /// <p><p>Starts a new task using the specified task definition.</p> <p>You can allow Amazon ECS to place tasks for you, or you can customize how Amazon ECS places tasks using placement constraints and placement strategies. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/scheduling_tasks.html">Scheduling Tasks</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>Alternatively, you can use <a>StartTask</a> to use your own scheduler or place tasks manually on specific container instances.</p> <p>The Amazon ECS API follows an eventual consistency model, due to the distributed nature of the system supporting the API. This means that the result of an API command you run that affects your Amazon ECS resources might not be immediately visible to all subsequent commands you run. Keep this in mind when you carry out an API command that immediately follows a previous API command.</p> <p>To manage eventual consistency, you can do the following:</p> <ul> <li> <p>Confirm the state of the resource before you run a command to modify it. Run the DescribeTasks command using an exponential backoff algorithm to ensure that you allow enough time for the previous command to propagate through the system. To do this, run the DescribeTasks command repeatedly, starting with a couple of seconds of wait time and increasing gradually up to five minutes of wait time.</p> </li> <li> <p>Add wait time between subsequent commands, even if the DescribeTasks command returns an accurate response. Apply an exponential backoff algorithm starting with a couple of seconds of wait time, and increase gradually up to about five minutes of wait time.</p> </li> </ul></p>
    fn run_task(&self, input: RunTaskRequest) -> RusotoFuture<RunTaskResponse, RunTaskError>;

    /// <p>Starts a new task from the specified task definition on the specified container instance or instances.</p> <p>Alternatively, you can use <a>RunTask</a> to place tasks for you. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/scheduling_tasks.html">Scheduling Tasks</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    fn start_task(
        &self,
        input: StartTaskRequest,
    ) -> RusotoFuture<StartTaskResponse, StartTaskError>;

    /// <p><p>Stops a running task. Any tags associated with the task will be deleted.</p> <p>When <a>StopTask</a> is called on a task, the equivalent of <code>docker stop</code> is issued to the containers running in the task. This results in a <code>SIGTERM</code> value and a default 30-second timeout, after which the <code>SIGKILL</code> value is sent and the containers are forcibly stopped. If the container handles the <code>SIGTERM</code> value gracefully and exits within 30 seconds from receiving it, no <code>SIGKILL</code> value is sent.</p> <note> <p>The default 30-second timeout can be configured on the Amazon ECS container agent with the <code>ECS<em>CONTAINER</em>STOP_TIMEOUT</code> variable. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-config.html">Amazon ECS Container Agent Configuration</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </note></p>
    fn stop_task(&self, input: StopTaskRequest) -> RusotoFuture<StopTaskResponse, StopTaskError>;

    /// <p><note> <p>This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent.</p> </note> <p>Sent to acknowledge that a container changed states.</p></p>
    fn submit_container_state_change(
        &self,
        input: SubmitContainerStateChangeRequest,
    ) -> RusotoFuture<SubmitContainerStateChangeResponse, SubmitContainerStateChangeError>;

    /// <p><note> <p>This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent.</p> </note> <p>Sent to acknowledge that a task changed states.</p></p>
    fn submit_task_state_change(
        &self,
        input: SubmitTaskStateChangeRequest,
    ) -> RusotoFuture<SubmitTaskStateChangeResponse, SubmitTaskStateChangeError>;

    /// <p>Associates the specified tags to a resource with the specified <code>resourceArn</code>. If existing tags on a resource are not specified in the request parameters, they are not changed. When a resource is deleted, the tags associated with that resource are deleted as well.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError>;

    /// <p>Deletes specified tags from a resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError>;

    /// <p>Updates the Amazon ECS container agent on a specified container instance. Updating the Amazon ECS container agent does not interrupt running tasks or services on the container instance. The process for updating the agent differs depending on whether your container instance was launched with the Amazon ECS-optimized AMI or another operating system.</p> <p> <code>UpdateContainerAgent</code> requires the Amazon ECS-optimized AMI or Amazon Linux with the <code>ecs-init</code> service installed and running. For help updating the Amazon ECS container agent on other operating systems, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-update.html#manually_update_agent">Manually Updating the Amazon ECS Container Agent</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    fn update_container_agent(
        &self,
        input: UpdateContainerAgentRequest,
    ) -> RusotoFuture<UpdateContainerAgentResponse, UpdateContainerAgentError>;

    /// <p>Modifies the status of an Amazon ECS container instance.</p> <p>You can change the status of a container instance to <code>DRAINING</code> to manually remove an instance from a cluster, for example to perform system updates, update the Docker daemon, or scale down the cluster size. </p> <p>When you set a container instance to <code>DRAINING</code>, Amazon ECS prevents new tasks from being scheduled for placement on the container instance and replacement service tasks are started on other container instances in the cluster if the resources are available. Service tasks on the container instance that are in the <code>PENDING</code> state are stopped immediately.</p> <p>Service tasks on the container instance that are in the <code>RUNNING</code> state are stopped and replaced according to the service's deployment configuration parameters, <code>minimumHealthyPercent</code> and <code>maximumPercent</code>. You can change the deployment configuration of your service using <a>UpdateService</a>.</p> <ul> <li> <p>If <code>minimumHealthyPercent</code> is below 100%, the scheduler can ignore <code>desiredCount</code> temporarily during task replacement. For example, <code>desiredCount</code> is four tasks, a minimum of 50% allows the scheduler to stop two existing tasks before starting two new tasks. If the minimum is 100%, the service scheduler can't remove existing tasks until the replacement tasks are considered healthy. Tasks for services that do not use a load balancer are considered healthy if they are in the <code>RUNNING</code> state. Tasks for services that use a load balancer are considered healthy if they are in the <code>RUNNING</code> state and the container instance they are hosted on is reported as healthy by the load balancer.</p> </li> <li> <p>The <code>maximumPercent</code> parameter represents an upper limit on the number of running tasks during task replacement, which enables you to define the replacement batch size. For example, if <code>desiredCount</code> is four tasks, a maximum of 200% starts four new tasks before stopping the four tasks to be drained, provided that the cluster resources required to do this are available. If the maximum is 100%, then replacement tasks can't start until the draining tasks have stopped.</p> </li> </ul> <p>Any <code>PENDING</code> or <code>RUNNING</code> tasks that do not belong to a service are not affected. You must wait for them to finish or stop them manually.</p> <p>A container instance has completed draining when it has no more <code>RUNNING</code> tasks. You can verify this using <a>ListTasks</a>.</p> <p>When you set a container instance to <code>ACTIVE</code>, the Amazon ECS scheduler can begin scheduling tasks on the instance again.</p>
    fn update_container_instances_state(
        &self,
        input: UpdateContainerInstancesStateRequest,
    ) -> RusotoFuture<UpdateContainerInstancesStateResponse, UpdateContainerInstancesStateError>;

    /// <p><p>Modifies the parameters of a service.</p> <p>For services using the rolling update (<code>ECS</code>) deployment controller, the desired count, deployment configuration, network configuration, or task definition used can be updated.</p> <p>For services using the blue/green (<code>CODE<em>DEPLOY</code>) deployment controller, only the desired count, deployment configuration, and health check grace period can be updated using this API. If the network configuration, platform version, or task definition need to be updated, a new AWS CodeDeploy deployment should be created. For more information, see &lt;a href=&quot;https://docs.aws.amazon.com/codedeploy/latest/APIReference/API</em>CreateDeployment.html&quot;&gt;CreateDeployment</a> in the <i>AWS CodeDeploy API Reference</i>.</p> <p>You can add to or subtract from the number of instantiations of a task definition in a service by specifying the cluster that the service is running in and a new <code>desiredCount</code> parameter.</p> <p>If you have updated the Docker image of your application, you can create a new task definition with that image and deploy it to your service. The service scheduler uses the minimum healthy percent and maximum percent parameters (in the service&#39;s deployment configuration) to determine the deployment strategy.</p> <note> <p>If your updated Docker image uses the same tag as what is in the existing task definition for your service (for example, <code>my_image:latest</code>), you do not need to create a new revision of your task definition. You can update the service using the <code>forceNewDeployment</code> option. The new tasks launched by the deployment pull the current image/tag combination from your repository when they start.</p> </note> <p>You can also update the deployment configuration of a service. When a deployment is triggered by updating the task definition of a service, the service scheduler uses the deployment configuration parameters, <code>minimumHealthyPercent</code> and <code>maximumPercent</code>, to determine the deployment strategy.</p> <ul> <li> <p>If <code>minimumHealthyPercent</code> is below 100%, the scheduler can ignore <code>desiredCount</code> temporarily during a deployment. For example, if <code>desiredCount</code> is four tasks, a minimum of 50% allows the scheduler to stop two existing tasks before starting two new tasks. Tasks for services that do not use a load balancer are considered healthy if they are in the <code>RUNNING</code> state. Tasks for services that use a load balancer are considered healthy if they are in the <code>RUNNING</code> state and the container instance they are hosted on is reported as healthy by the load balancer.</p> </li> <li> <p>The <code>maximumPercent</code> parameter represents an upper limit on the number of running tasks during a deployment, which enables you to define the deployment batch size. For example, if <code>desiredCount</code> is four tasks, a maximum of 200% starts four new tasks before stopping the four older tasks (provided that the cluster resources required to do this are available).</p> </li> </ul> <p>When <a>UpdateService</a> stops a task during a deployment, the equivalent of <code>docker stop</code> is issued to the containers running in the task. This results in a <code>SIGTERM</code> and a 30-second timeout, after which <code>SIGKILL</code> is sent and the containers are forcibly stopped. If the container handles the <code>SIGTERM</code> gracefully and exits within 30 seconds from receiving it, no <code>SIGKILL</code> is sent.</p> <p>When the service scheduler launches new tasks, it determines task placement in your cluster with the following logic:</p> <ul> <li> <p>Determine which of the container instances in your cluster can support your service&#39;s task definition (for example, they have the required CPU, memory, ports, and container instance attributes).</p> </li> <li> <p>By default, the service scheduler attempts to balance tasks across Availability Zones in this manner (although you can choose a different placement strategy):</p> <ul> <li> <p>Sort the valid container instances by the fewest number of running tasks for this service in the same Availability Zone as the instance. For example, if zone A has one running service task and zones B and C each have zero, valid container instances in either zone B or C are considered optimal for placement.</p> </li> <li> <p>Place the new service task on a valid container instance in an optimal Availability Zone (based on the previous steps), favoring container instances with the fewest number of running tasks for this service.</p> </li> </ul> </li> </ul> <p>When the service scheduler stops running tasks, it attempts to maintain balance across the Availability Zones in your cluster using the following logic: </p> <ul> <li> <p>Sort the container instances by the largest number of running tasks for this service in the same Availability Zone as the instance. For example, if zone A has one running service task and zones B and C each have two, container instances in either zone B or C are considered optimal for termination.</p> </li> <li> <p>Stop the task on a container instance in an optimal Availability Zone (based on the previous steps), favoring container instances with the largest number of running tasks for this service.</p> </li> </ul></p>
    fn update_service(
        &self,
        input: UpdateServiceRequest,
    ) -> RusotoFuture<UpdateServiceResponse, UpdateServiceError>;
}
/// A client for the Amazon ECS API.
#[derive(Clone)]
pub struct EcsClient {
    client: Client,
    region: region::Region,
}

impl EcsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> EcsClient {
        EcsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> EcsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        EcsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }
}

impl Ecs for EcsClient {
    /// <p><p>Creates a new Amazon ECS cluster. By default, your account receives a <code>default</code> cluster when you launch your first container instance. However, you can create your own cluster with a unique name with the <code>CreateCluster</code> action.</p> <note> <p>When you call the <a>CreateCluster</a> API operation, Amazon ECS attempts to create the service-linked role for your account so that required resources in other AWS services can be managed on your behalf. However, if the IAM user that makes the call does not have permissions to create the service-linked role, it is not created. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/using-service-linked-roles.html">Using Service-Linked Roles for Amazon ECS</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </note></p>
    fn create_cluster(
        &self,
        input: CreateClusterRequest,
    ) -> RusotoFuture<CreateClusterResponse, CreateClusterError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.CreateCluster",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateClusterResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateClusterError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Runs and maintains a desired number of tasks from a specified task definition. If the number of tasks running in a service drops below <code>desiredCount</code>, Amazon ECS spawns another copy of the task in the specified cluster. To update an existing service, see <a>UpdateService</a>.</p> <p>In addition to maintaining the desired count of tasks in your service, you can optionally run your service behind a load balancer. The load balancer distributes traffic across the tasks that are associated with the service. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-load-balancing.html">Service Load Balancing</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>You can optionally specify a deployment configuration for your service. The deployment is triggered by changing properties, such as the task definition or the desired count of a service, with an <a>UpdateService</a> operation.</p> <p>If a service is using the <code>ECS</code> deployment controller, the <b>minimum healthy percent</b> represents a lower limit on the number of tasks in a service that must remain in the <code>RUNNING</code> state during a deployment, as a percentage of the desired number of tasks (rounded up to the nearest integer), and while any container instances are in the <code>DRAINING</code> state if the service contains tasks using the EC2 launch type. This parameter enables you to deploy without using additional cluster capacity. For example, if your service has a desired number of four tasks and a minimum healthy percent of 50%, the scheduler may stop two existing tasks to free up cluster capacity before starting two new tasks. Tasks for services that <i>do not</i> use a load balancer are considered healthy if they are in the <code>RUNNING</code> state; tasks for services that <i>do</i> use a load balancer are considered healthy if they are in the <code>RUNNING</code> state and they are reported as healthy by the load balancer. The default value for minimum healthy percent is 100%.</p> <p>If a service is using the <code>ECS</code> deployment controller, the <b>maximum percent</b> parameter represents an upper limit on the number of tasks in a service that are allowed in the <code>RUNNING</code> or <code>PENDING</code> state during a deployment, as a percentage of the desired number of tasks (rounded down to the nearest integer), and while any container instances are in the <code>DRAINING</code> state if the service contains tasks using the EC2 launch type. This parameter enables you to define the deployment batch size. For example, if your service has a desired number of four tasks and a maximum percent value of 200%, the scheduler may start four new tasks before stopping the four older tasks (provided that the cluster resources required to do this are available). The default value for maximum percent is 200%.</p> <p>If a service is using the <code>CODE_DEPLOY</code> deployment controller and tasks that use the EC2 launch type, the <b>minimum healthy percent</b> and <b>maximum percent</b> values are only used to define the lower and upper limit on the number of the tasks in the service that remain in the <code>RUNNING</code> state while the container instances are in the <code>DRAINING</code> state. If the tasks in the service use the Fargate launch type, the minimum healthy percent and maximum percent values are not used, although they are currently visible when describing your service.</p> <p>Tasks for services that <i>do not</i> use a load balancer are considered healthy if they are in the <code>RUNNING</code> state. Tasks for services that <i>do</i> use a load balancer are considered healthy if they are in the <code>RUNNING</code> state and the container instance they are hosted on is reported as healthy by the load balancer. The default value for a replica service for <code>minimumHealthyPercent</code> is 100%. The default value for a daemon service for <code>minimumHealthyPercent</code> is 0%.</p> <p>When the service scheduler launches new tasks, it determines task placement in your cluster using the following logic:</p> <ul> <li> <p>Determine which of the container instances in your cluster can support your service&#39;s task definition (for example, they have the required CPU, memory, ports, and container instance attributes).</p> </li> <li> <p>By default, the service scheduler attempts to balance tasks across Availability Zones in this manner (although you can choose a different placement strategy) with the <code>placementStrategy</code> parameter):</p> <ul> <li> <p>Sort the valid container instances, giving priority to instances that have the fewest number of running tasks for this service in their respective Availability Zone. For example, if zone A has one running service task and zones B and C each have zero, valid container instances in either zone B or C are considered optimal for placement.</p> </li> <li> <p>Place the new service task on a valid container instance in an optimal Availability Zone (based on the previous steps), favoring container instances with the fewest number of running tasks for this service.</p> </li> </ul> </li> </ul></p>
    fn create_service(
        &self,
        input: CreateServiceRequest,
    ) -> RusotoFuture<CreateServiceResponse, CreateServiceError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.CreateService",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateServiceResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateServiceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Modifies the ARN and resource ID format of a resource for a specified IAM user, IAM role, or the root user for an account. You can specify whether the new ARN and resource ID format are disabled for new resources that are created.</p>
    fn delete_account_setting(
        &self,
        input: DeleteAccountSettingRequest,
    ) -> RusotoFuture<DeleteAccountSettingResponse, DeleteAccountSettingError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DeleteAccountSetting",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteAccountSettingResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteAccountSettingError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes one or more custom attributes from an Amazon ECS resource.</p>
    fn delete_attributes(
        &self,
        input: DeleteAttributesRequest,
    ) -> RusotoFuture<DeleteAttributesResponse, DeleteAttributesError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DeleteAttributes",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteAttributesResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteAttributesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified cluster. You must deregister all container instances from this cluster before you may delete it. You can list the container instances in a cluster with <a>ListContainerInstances</a> and deregister them with <a>DeregisterContainerInstance</a>.</p>
    fn delete_cluster(
        &self,
        input: DeleteClusterRequest,
    ) -> RusotoFuture<DeleteClusterResponse, DeleteClusterError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DeleteCluster",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteClusterResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteClusterError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Deletes a specified service within a cluster. You can delete a service if you have no running tasks in it and the desired task count is zero. If the service is actively maintaining tasks, you cannot delete it, and you must update the service to a desired task count of zero. For more information, see <a>UpdateService</a>.</p> <note> <p>When you delete a service, if there are still running tasks that require cleanup, the service status moves from <code>ACTIVE</code> to <code>DRAINING</code>, and the service is no longer visible in the console or in the <a>ListServices</a> API operation. After the tasks have stopped, then the service status moves from <code>DRAINING</code> to <code>INACTIVE</code>. Services in the <code>DRAINING</code> or <code>INACTIVE</code> status can still be viewed with the <a>DescribeServices</a> API operation. However, in the future, <code>INACTIVE</code> services may be cleaned up and purged from Amazon ECS record keeping, and <a>DescribeServices</a> calls on those services return a <code>ServiceNotFoundException</code> error.</p> </note> <important> <p>If you attempt to create a new service with the same name as an existing service in either <code>ACTIVE</code> or <code>DRAINING</code> status, you receive an error.</p> </important></p>
    fn delete_service(
        &self,
        input: DeleteServiceRequest,
    ) -> RusotoFuture<DeleteServiceResponse, DeleteServiceError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DeleteService",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteServiceResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteServiceError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Deregisters an Amazon ECS container instance from the specified cluster. This instance is no longer available to run tasks.</p> <p>If you intend to use the container instance for some other purpose after deregistration, you should stop all of the tasks running on the container instance before deregistration. That prevents any orphaned tasks from consuming resources.</p> <p>Deregistering a container instance removes the instance from a cluster, but it does not terminate the EC2 instance. If you are finished using the instance, be sure to terminate it in the Amazon EC2 console to stop billing.</p> <note> <p>If you terminate a running container instance, Amazon ECS automatically deregisters the instance from your cluster (stopped container instances or instances with disconnected agents are not automatically deregistered when terminated).</p> </note></p>
    fn deregister_container_instance(
        &self,
        input: DeregisterContainerInstanceRequest,
    ) -> RusotoFuture<DeregisterContainerInstanceResponse, DeregisterContainerInstanceError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DeregisterContainerInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeregisterContainerInstanceResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeregisterContainerInstanceError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Deregisters the specified task definition by family and revision. Upon deregistration, the task definition is marked as <code>INACTIVE</code>. Existing tasks and services that reference an <code>INACTIVE</code> task definition continue to run without disruption. Existing services that reference an <code>INACTIVE</code> task definition can still scale up or down by modifying the service&#39;s desired count.</p> <p>You cannot use an <code>INACTIVE</code> task definition to run new tasks or create new services, and you cannot update an existing service to reference an <code>INACTIVE</code> task definition. However, there may be up to a 10-minute window following deregistration where these restrictions have not yet taken effect.</p> <note> <p>At this time, <code>INACTIVE</code> task definitions remain discoverable in your account indefinitely. However, this behavior is subject to change in the future, so you should not rely on <code>INACTIVE</code> task definitions persisting beyond the lifecycle of any associated tasks and services.</p> </note></p>
    fn deregister_task_definition(
        &self,
        input: DeregisterTaskDefinitionRequest,
    ) -> RusotoFuture<DeregisterTaskDefinitionResponse, DeregisterTaskDefinitionError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DeregisterTaskDefinition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeregisterTaskDefinitionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeregisterTaskDefinitionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Describes one or more of your clusters.</p>
    fn describe_clusters(
        &self,
        input: DescribeClustersRequest,
    ) -> RusotoFuture<DescribeClustersResponse, DescribeClustersError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DescribeClusters",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeClustersResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeClustersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes Amazon Elastic Container Service container instances. Returns metadata about registered and remaining resources on each container instance requested.</p>
    fn describe_container_instances(
        &self,
        input: DescribeContainerInstancesRequest,
    ) -> RusotoFuture<DescribeContainerInstancesResponse, DescribeContainerInstancesError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DescribeContainerInstances",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeContainerInstancesResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeContainerInstancesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Describes the specified services running in your cluster.</p>
    fn describe_services(
        &self,
        input: DescribeServicesRequest,
    ) -> RusotoFuture<DescribeServicesResponse, DescribeServicesError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DescribeServices",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeServicesResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeServicesError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Describes a task definition. You can specify a <code>family</code> and <code>revision</code> to find information about a specific task definition, or you can simply specify the family to find the latest <code>ACTIVE</code> revision in that family.</p> <note> <p>You can only describe <code>INACTIVE</code> task definitions while an active task or service references them.</p> </note></p>
    fn describe_task_definition(
        &self,
        input: DescribeTaskDefinitionRequest,
    ) -> RusotoFuture<DescribeTaskDefinitionResponse, DescribeTaskDefinitionError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DescribeTaskDefinition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeTaskDefinitionResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeTaskDefinitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Describes a specified task or tasks.</p>
    fn describe_tasks(
        &self,
        input: DescribeTasksRequest,
    ) -> RusotoFuture<DescribeTasksResponse, DescribeTasksError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DescribeTasks",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeTasksResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeTasksError::from_response(response))),
                )
            }
        })
    }

    /// <p><note> <p>This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent.</p> </note> <p>Returns an endpoint for the Amazon ECS agent to poll for updates.</p></p>
    fn discover_poll_endpoint(
        &self,
        input: DiscoverPollEndpointRequest,
    ) -> RusotoFuture<DiscoverPollEndpointResponse, DiscoverPollEndpointError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.DiscoverPollEndpoint",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DiscoverPollEndpointResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DiscoverPollEndpointError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists the account settings for an Amazon ECS resource for a specified principal.</p>
    fn list_account_settings(
        &self,
        input: ListAccountSettingsRequest,
    ) -> RusotoFuture<ListAccountSettingsResponse, ListAccountSettingsError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.ListAccountSettings",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListAccountSettingsResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListAccountSettingsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists the attributes for Amazon ECS resources within a specified target type and cluster. When you specify a target type and cluster, <code>ListAttributes</code> returns a list of attribute objects, one for each attribute on each resource. You can filter the list of results to a single attribute name to only return results that have that name. You can also filter the results by attribute name and value, for example, to see which container instances in a cluster are running a Linux AMI (<code>ecs.os-type=linux</code>). </p>
    fn list_attributes(
        &self,
        input: ListAttributesRequest,
    ) -> RusotoFuture<ListAttributesResponse, ListAttributesError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.ListAttributes",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListAttributesResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListAttributesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of existing clusters.</p>
    fn list_clusters(
        &self,
        input: ListClustersRequest,
    ) -> RusotoFuture<ListClustersResponse, ListClustersError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.ListClusters",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListClustersResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListClustersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of container instances in a specified cluster. You can filter the results of a <code>ListContainerInstances</code> operation with cluster query language statements inside the <code>filter</code> parameter. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/cluster-query-language.html">Cluster Query Language</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    fn list_container_instances(
        &self,
        input: ListContainerInstancesRequest,
    ) -> RusotoFuture<ListContainerInstancesResponse, ListContainerInstancesError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.ListContainerInstances",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListContainerInstancesResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListContainerInstancesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists the services that are running in a specified cluster.</p>
    fn list_services(
        &self,
        input: ListServicesRequest,
    ) -> RusotoFuture<ListServicesResponse, ListServicesError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.ListServices",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListServicesResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListServicesError::from_response(response))),
                )
            }
        })
    }

    /// <p>List the tags for an Amazon ECS resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.ListTagsForResource",
        );
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

    /// <p>Returns a list of task definition families that are registered to your account (which may include task definition families that no longer have any <code>ACTIVE</code> task definition revisions).</p> <p>You can filter out task definition families that do not contain any <code>ACTIVE</code> task definition revisions by setting the <code>status</code> parameter to <code>ACTIVE</code>. You can also filter the results with the <code>familyPrefix</code> parameter.</p>
    fn list_task_definition_families(
        &self,
        input: ListTaskDefinitionFamiliesRequest,
    ) -> RusotoFuture<ListTaskDefinitionFamiliesResponse, ListTaskDefinitionFamiliesError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.ListTaskDefinitionFamilies",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTaskDefinitionFamiliesResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListTaskDefinitionFamiliesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns a list of task definitions that are registered to your account. You can filter the results by family name with the <code>familyPrefix</code> parameter or by status with the <code>status</code> parameter.</p>
    fn list_task_definitions(
        &self,
        input: ListTaskDefinitionsRequest,
    ) -> RusotoFuture<ListTaskDefinitionsResponse, ListTaskDefinitionsError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.ListTaskDefinitions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTaskDefinitionsResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListTaskDefinitionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns a list of tasks for a specified cluster. You can filter the results by family name, by a particular container instance, or by the desired status of the task with the <code>family</code>, <code>containerInstance</code>, and <code>desiredStatus</code> parameters.</p> <p>Recently stopped tasks might appear in the returned results. Currently, stopped tasks appear in the returned results for at least one hour. </p>
    fn list_tasks(
        &self,
        input: ListTasksRequest,
    ) -> RusotoFuture<ListTasksResponse, ListTasksError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.ListTasks",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTasksResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListTasksError::from_response(response))),
                )
            }
        })
    }

    /// <p>Modifies the ARN and resource ID format of a resource type for a specified IAM user, IAM role, or the root user for an account. If the account setting for the root user is changed, it sets the default setting for all of the IAM users and roles for which no individual account setting has been set. The opt-in and opt-out account setting can be set for each Amazon ECS resource separately. The ARN and resource ID format of a resource will be defined by the opt-in status of the IAM user or role that created the resource. Enabling this setting is required to use new Amazon ECS features such as resource tagging. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-resource-ids.html">Amazon Resource Names (ARNs) and IDs</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    fn put_account_setting(
        &self,
        input: PutAccountSettingRequest,
    ) -> RusotoFuture<PutAccountSettingResponse, PutAccountSettingError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.PutAccountSetting",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutAccountSettingResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutAccountSettingError::from_response(response))),
                )
            }
        })
    }

    /// <p>Modifies the ARN and resource ID format of a resource type for all IAM users on an account for which no individual account setting has been set. Enabling this setting is required to use new Amazon ECS features such as resource tagging.</p>
    fn put_account_setting_default(
        &self,
        input: PutAccountSettingDefaultRequest,
    ) -> RusotoFuture<PutAccountSettingDefaultResponse, PutAccountSettingDefaultError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.PutAccountSettingDefault",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutAccountSettingDefaultResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutAccountSettingDefaultError::from_response(response))
                }))
            }
        })
    }

    /// <p>Create or update an attribute on an Amazon ECS resource. If the attribute does not exist, it is created. If the attribute exists, its value is replaced with the specified value. To delete an attribute, use <a>DeleteAttributes</a>. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-placement-constraints.html#attributes">Attributes</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    fn put_attributes(
        &self,
        input: PutAttributesRequest,
    ) -> RusotoFuture<PutAttributesResponse, PutAttributesError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.PutAttributes",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutAttributesResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutAttributesError::from_response(response))),
                )
            }
        })
    }

    /// <p><note> <p>This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent.</p> </note> <p>Registers an EC2 instance into the specified cluster. This instance becomes available to place containers on.</p></p>
    fn register_container_instance(
        &self,
        input: RegisterContainerInstanceRequest,
    ) -> RusotoFuture<RegisterContainerInstanceResponse, RegisterContainerInstanceError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.RegisterContainerInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RegisterContainerInstanceResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RegisterContainerInstanceError::from_response(response))
                }))
            }
        })
    }

    /// <p>Registers a new task definition from the supplied <code>family</code> and <code>containerDefinitions</code>. Optionally, you can add data volumes to your containers with the <code>volumes</code> parameter. For more information about task definition parameters and defaults, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task_defintions.html">Amazon ECS Task Definitions</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>You can specify an IAM role for your task with the <code>taskRoleArn</code> parameter. When you specify an IAM role for a task, its containers can then use the latest versions of the AWS CLI or SDKs to make API requests to the AWS services that are specified in the IAM policy associated with the role. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-iam-roles.html">IAM Roles for Tasks</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>You can specify a Docker networking mode for the containers in your task definition with the <code>networkMode</code> parameter. The available network modes correspond to those described in <a href="https://docs.docker.com/engine/reference/run/#/network-settings">Network settings</a> in the Docker run reference. If you specify the <code>awsvpc</code> network mode, the task is allocated an elastic network interface, and you must specify a <a>NetworkConfiguration</a> when you create a service or run a task with the task definition. For more information, see <a href="http://docs.aws.amazon.com/AmazonECS/latest/developerguide/task-networking.html">Task Networking</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    fn register_task_definition(
        &self,
        input: RegisterTaskDefinitionRequest,
    ) -> RusotoFuture<RegisterTaskDefinitionResponse, RegisterTaskDefinitionError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.RegisterTaskDefinition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RegisterTaskDefinitionResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(RegisterTaskDefinitionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p><p>Starts a new task using the specified task definition.</p> <p>You can allow Amazon ECS to place tasks for you, or you can customize how Amazon ECS places tasks using placement constraints and placement strategies. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/scheduling_tasks.html">Scheduling Tasks</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <p>Alternatively, you can use <a>StartTask</a> to use your own scheduler or place tasks manually on specific container instances.</p> <p>The Amazon ECS API follows an eventual consistency model, due to the distributed nature of the system supporting the API. This means that the result of an API command you run that affects your Amazon ECS resources might not be immediately visible to all subsequent commands you run. Keep this in mind when you carry out an API command that immediately follows a previous API command.</p> <p>To manage eventual consistency, you can do the following:</p> <ul> <li> <p>Confirm the state of the resource before you run a command to modify it. Run the DescribeTasks command using an exponential backoff algorithm to ensure that you allow enough time for the previous command to propagate through the system. To do this, run the DescribeTasks command repeatedly, starting with a couple of seconds of wait time and increasing gradually up to five minutes of wait time.</p> </li> <li> <p>Add wait time between subsequent commands, even if the DescribeTasks command returns an accurate response. Apply an exponential backoff algorithm starting with a couple of seconds of wait time, and increase gradually up to about five minutes of wait time.</p> </li> </ul></p>
    fn run_task(&self, input: RunTaskRequest) -> RusotoFuture<RunTaskResponse, RunTaskError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonEC2ContainerServiceV20141113.RunTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<RunTaskResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RunTaskError::from_response(response))),
                )
            }
        })
    }

    /// <p>Starts a new task from the specified task definition on the specified container instance or instances.</p> <p>Alternatively, you can use <a>RunTask</a> to place tasks for you. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/scheduling_tasks.html">Scheduling Tasks</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    fn start_task(
        &self,
        input: StartTaskRequest,
    ) -> RusotoFuture<StartTaskResponse, StartTaskError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.StartTask",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartTaskResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StartTaskError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Stops a running task. Any tags associated with the task will be deleted.</p> <p>When <a>StopTask</a> is called on a task, the equivalent of <code>docker stop</code> is issued to the containers running in the task. This results in a <code>SIGTERM</code> value and a default 30-second timeout, after which the <code>SIGKILL</code> value is sent and the containers are forcibly stopped. If the container handles the <code>SIGTERM</code> value gracefully and exits within 30 seconds from receiving it, no <code>SIGKILL</code> value is sent.</p> <note> <p>The default 30-second timeout can be configured on the Amazon ECS container agent with the <code>ECS<em>CONTAINER</em>STOP_TIMEOUT</code> variable. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-config.html">Amazon ECS Container Agent Configuration</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </note></p>
    fn stop_task(&self, input: StopTaskRequest) -> RusotoFuture<StopTaskResponse, StopTaskError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.StopTask",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StopTaskResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StopTaskError::from_response(response))),
                )
            }
        })
    }

    /// <p><note> <p>This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent.</p> </note> <p>Sent to acknowledge that a container changed states.</p></p>
    fn submit_container_state_change(
        &self,
        input: SubmitContainerStateChangeRequest,
    ) -> RusotoFuture<SubmitContainerStateChangeResponse, SubmitContainerStateChangeError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.SubmitContainerStateChange",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<SubmitContainerStateChangeResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(SubmitContainerStateChangeError::from_response(response))
                }))
            }
        })
    }

    /// <p><note> <p>This action is only used by the Amazon ECS agent, and it is not intended for use outside of the agent.</p> </note> <p>Sent to acknowledge that a task changed states.</p></p>
    fn submit_task_state_change(
        &self,
        input: SubmitTaskStateChangeRequest,
    ) -> RusotoFuture<SubmitTaskStateChangeResponse, SubmitTaskStateChangeError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.SubmitTaskStateChange",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<SubmitTaskStateChangeResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(SubmitTaskStateChangeError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Associates the specified tags to a resource with the specified <code>resourceArn</code>. If existing tags on a resource are not specified in the request parameters, they are not changed. When a resource is deleted, the tags associated with that resource are deleted as well.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.TagResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<TagResourceResponse, _>()
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

    /// <p>Deletes specified tags from a resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.UntagResource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UntagResourceResponse, _>()
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

    /// <p>Updates the Amazon ECS container agent on a specified container instance. Updating the Amazon ECS container agent does not interrupt running tasks or services on the container instance. The process for updating the agent differs depending on whether your container instance was launched with the Amazon ECS-optimized AMI or another operating system.</p> <p> <code>UpdateContainerAgent</code> requires the Amazon ECS-optimized AMI or Amazon Linux with the <code>ecs-init</code> service installed and running. For help updating the Amazon ECS container agent on other operating systems, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-update.html#manually_update_agent">Manually Updating the Amazon ECS Container Agent</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    fn update_container_agent(
        &self,
        input: UpdateContainerAgentRequest,
    ) -> RusotoFuture<UpdateContainerAgentResponse, UpdateContainerAgentError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.UpdateContainerAgent",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateContainerAgentResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateContainerAgentError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Modifies the status of an Amazon ECS container instance.</p> <p>You can change the status of a container instance to <code>DRAINING</code> to manually remove an instance from a cluster, for example to perform system updates, update the Docker daemon, or scale down the cluster size. </p> <p>When you set a container instance to <code>DRAINING</code>, Amazon ECS prevents new tasks from being scheduled for placement on the container instance and replacement service tasks are started on other container instances in the cluster if the resources are available. Service tasks on the container instance that are in the <code>PENDING</code> state are stopped immediately.</p> <p>Service tasks on the container instance that are in the <code>RUNNING</code> state are stopped and replaced according to the service's deployment configuration parameters, <code>minimumHealthyPercent</code> and <code>maximumPercent</code>. You can change the deployment configuration of your service using <a>UpdateService</a>.</p> <ul> <li> <p>If <code>minimumHealthyPercent</code> is below 100%, the scheduler can ignore <code>desiredCount</code> temporarily during task replacement. For example, <code>desiredCount</code> is four tasks, a minimum of 50% allows the scheduler to stop two existing tasks before starting two new tasks. If the minimum is 100%, the service scheduler can't remove existing tasks until the replacement tasks are considered healthy. Tasks for services that do not use a load balancer are considered healthy if they are in the <code>RUNNING</code> state. Tasks for services that use a load balancer are considered healthy if they are in the <code>RUNNING</code> state and the container instance they are hosted on is reported as healthy by the load balancer.</p> </li> <li> <p>The <code>maximumPercent</code> parameter represents an upper limit on the number of running tasks during task replacement, which enables you to define the replacement batch size. For example, if <code>desiredCount</code> is four tasks, a maximum of 200% starts four new tasks before stopping the four tasks to be drained, provided that the cluster resources required to do this are available. If the maximum is 100%, then replacement tasks can't start until the draining tasks have stopped.</p> </li> </ul> <p>Any <code>PENDING</code> or <code>RUNNING</code> tasks that do not belong to a service are not affected. You must wait for them to finish or stop them manually.</p> <p>A container instance has completed draining when it has no more <code>RUNNING</code> tasks. You can verify this using <a>ListTasks</a>.</p> <p>When you set a container instance to <code>ACTIVE</code>, the Amazon ECS scheduler can begin scheduling tasks on the instance again.</p>
    fn update_container_instances_state(
        &self,
        input: UpdateContainerInstancesStateRequest,
    ) -> RusotoFuture<UpdateContainerInstancesStateResponse, UpdateContainerInstancesStateError>
    {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.UpdateContainerInstancesState",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateContainerInstancesStateResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateContainerInstancesStateError::from_response(response))
                }))
            }
        })
    }

    /// <p><p>Modifies the parameters of a service.</p> <p>For services using the rolling update (<code>ECS</code>) deployment controller, the desired count, deployment configuration, network configuration, or task definition used can be updated.</p> <p>For services using the blue/green (<code>CODE<em>DEPLOY</code>) deployment controller, only the desired count, deployment configuration, and health check grace period can be updated using this API. If the network configuration, platform version, or task definition need to be updated, a new AWS CodeDeploy deployment should be created. For more information, see &lt;a href=&quot;https://docs.aws.amazon.com/codedeploy/latest/APIReference/API</em>CreateDeployment.html&quot;&gt;CreateDeployment</a> in the <i>AWS CodeDeploy API Reference</i>.</p> <p>You can add to or subtract from the number of instantiations of a task definition in a service by specifying the cluster that the service is running in and a new <code>desiredCount</code> parameter.</p> <p>If you have updated the Docker image of your application, you can create a new task definition with that image and deploy it to your service. The service scheduler uses the minimum healthy percent and maximum percent parameters (in the service&#39;s deployment configuration) to determine the deployment strategy.</p> <note> <p>If your updated Docker image uses the same tag as what is in the existing task definition for your service (for example, <code>my_image:latest</code>), you do not need to create a new revision of your task definition. You can update the service using the <code>forceNewDeployment</code> option. The new tasks launched by the deployment pull the current image/tag combination from your repository when they start.</p> </note> <p>You can also update the deployment configuration of a service. When a deployment is triggered by updating the task definition of a service, the service scheduler uses the deployment configuration parameters, <code>minimumHealthyPercent</code> and <code>maximumPercent</code>, to determine the deployment strategy.</p> <ul> <li> <p>If <code>minimumHealthyPercent</code> is below 100%, the scheduler can ignore <code>desiredCount</code> temporarily during a deployment. For example, if <code>desiredCount</code> is four tasks, a minimum of 50% allows the scheduler to stop two existing tasks before starting two new tasks. Tasks for services that do not use a load balancer are considered healthy if they are in the <code>RUNNING</code> state. Tasks for services that use a load balancer are considered healthy if they are in the <code>RUNNING</code> state and the container instance they are hosted on is reported as healthy by the load balancer.</p> </li> <li> <p>The <code>maximumPercent</code> parameter represents an upper limit on the number of running tasks during a deployment, which enables you to define the deployment batch size. For example, if <code>desiredCount</code> is four tasks, a maximum of 200% starts four new tasks before stopping the four older tasks (provided that the cluster resources required to do this are available).</p> </li> </ul> <p>When <a>UpdateService</a> stops a task during a deployment, the equivalent of <code>docker stop</code> is issued to the containers running in the task. This results in a <code>SIGTERM</code> and a 30-second timeout, after which <code>SIGKILL</code> is sent and the containers are forcibly stopped. If the container handles the <code>SIGTERM</code> gracefully and exits within 30 seconds from receiving it, no <code>SIGKILL</code> is sent.</p> <p>When the service scheduler launches new tasks, it determines task placement in your cluster with the following logic:</p> <ul> <li> <p>Determine which of the container instances in your cluster can support your service&#39;s task definition (for example, they have the required CPU, memory, ports, and container instance attributes).</p> </li> <li> <p>By default, the service scheduler attempts to balance tasks across Availability Zones in this manner (although you can choose a different placement strategy):</p> <ul> <li> <p>Sort the valid container instances by the fewest number of running tasks for this service in the same Availability Zone as the instance. For example, if zone A has one running service task and zones B and C each have zero, valid container instances in either zone B or C are considered optimal for placement.</p> </li> <li> <p>Place the new service task on a valid container instance in an optimal Availability Zone (based on the previous steps), favoring container instances with the fewest number of running tasks for this service.</p> </li> </ul> </li> </ul> <p>When the service scheduler stops running tasks, it attempts to maintain balance across the Availability Zones in your cluster using the following logic: </p> <ul> <li> <p>Sort the container instances by the largest number of running tasks for this service in the same Availability Zone as the instance. For example, if zone A has one running service task and zones B and C each have two, container instances in either zone B or C are considered optimal for termination.</p> </li> <li> <p>Stop the task on a container instance in an optimal Availability Zone (based on the previous steps), favoring container instances with the largest number of running tasks for this service.</p> </li> </ul></p>
    fn update_service(
        &self,
        input: UpdateServiceRequest,
    ) -> RusotoFuture<UpdateServiceResponse, UpdateServiceError> {
        let mut request = SignedRequest::new("POST", "ecs", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AmazonEC2ContainerServiceV20141113.UpdateService",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateServiceResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateServiceError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
