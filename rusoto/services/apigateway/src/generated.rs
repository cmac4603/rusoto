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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
/// <p>Access log settings, including the access log format and access log destination ARN.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AccessLogSettings {
    /// <p>The ARN of the CloudWatch Logs log group to receive access logs.</p>
    #[serde(rename = "destinationArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_arn: Option<String>,
    /// <p>A single line format of the access logs of data, as specified by selected <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-mapping-template-reference.html#context-variable-reference">$context variables</a>. The format must include at least <code>$context.requestId</code>.</p>
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// <p><p>Represents an AWS account that is associated with API Gateway.</p> <div class="remarks"> <p>To view the account info, call <code>GET</code> on this resource.</p> <h4>Error Codes</h4> <p>The following exception may be thrown when the request fails.</p> <ul> <li>UnauthorizedException</li> <li>NotFoundException</li> <li>TooManyRequestsException</li> </ul> <p>For detailed error code information, including the corresponding HTTP Status Codes, see <a href="https://docs.aws.amazon.com/apigateway/api-reference/handling-errors/#api-error-codes">API Gateway Error Codes</a></p> <h4>Example: Get the information about an account.</h4> <h5>Request</h5> <pre><code>GET /account HTTP/1.1 Content-Type: application/json Host: apigateway.us-east-1.amazonaws.com X-Amz-Date: 20160531T184618Z Authorization: AWS4-HMAC-SHA256 Credential={access<em>key</em>ID}/us-east-1/apigateway/aws4<em>request, SignedHeaders=content-type;host;x-amz-date, Signature={sig4</em>hash} </code></pre> <h5>Response</h5> <p>The successful response returns a <code>200 OK</code> status code and a payload similar to the following:</p> <pre><code>{ &quot;_links&quot;: { &quot;curies&quot;: { &quot;href&quot;: &quot;https://docs.aws.amazon.com/apigateway/latest/developerguide/account-apigateway-{rel}.html&quot;, &quot;name&quot;: &quot;account&quot;, &quot;templated&quot;: true }, &quot;self&quot;: { &quot;href&quot;: &quot;/account&quot; }, &quot;account:update&quot;: { &quot;href&quot;: &quot;/account&quot; } }, &quot;cloudwatchRoleArn&quot;: &quot;arn:aws:iam::123456789012:role/apigAwsProxyRole&quot;, &quot;throttleSettings&quot;: { &quot;rateLimit&quot;: 500, &quot;burstLimit&quot;: 1000 } } </code></pre> <p>In addition to making the REST API call directly, you can use the AWS CLI and an AWS SDK to access this resource.</p> </div> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-limits.html">API Gateway Limits</a> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/welcome.html">Developer Guide</a>, <a href="https://docs.aws.amazon.com/cli/latest/reference/apigateway/get-account.html">AWS CLI</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Account {
    /// <p>The version of the API keys used for the account.</p>
    #[serde(rename = "apiKeyVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_version: Option<String>,
    /// <p>The ARN of an Amazon CloudWatch role for the current <a>Account</a>. </p>
    #[serde(rename = "cloudwatchRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch_role_arn: Option<String>,
    /// <p>A list of features supported for the account. When usage plans are enabled, the features list will include an entry of <code>"UsagePlans"</code>.</p>
    #[serde(rename = "features")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
    /// <p>Specifies the API request limits configured for the current <a>Account</a>.</p>
    #[serde(rename = "throttleSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle_settings: Option<ThrottleSettings>,
}

/// <p><p>A resource that can be distributed to callers for executing <a>Method</a> resources that require an API key. API keys can be mapped to any <a>Stage</a> on any <a>RestApi</a>, which indicates that the callers with the API key can make requests to that stage.</p> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-api-keys.html">Use API Keys</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ApiKey {
    /// <p>The timestamp when the API Key was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>An AWS Marketplace customer identifier , when integrating with the AWS SaaS Marketplace.</p>
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// <p>The description of the API Key.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Specifies whether the API Key can be used by callers.</p>
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The identifier of the API Key.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The timestamp when the API Key was last updated.</p>
    #[serde(rename = "lastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>The name of the API Key.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of <a>Stage</a> resources that are associated with the <a>ApiKey</a> resource.</p>
    #[serde(rename = "stageKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_keys: Option<Vec<String>>,
    /// <p>The collection of tags. Each tag element is associated with a given resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The value of the API Key.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The identifier of an <a>ApiKey</a> used in a <a>UsagePlan</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ApiKeyIds {
    /// <p>A list of all the <a>ApiKey</a> identifiers.</p>
    #[serde(rename = "ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// <p>A list of warning messages.</p>
    #[serde(rename = "warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// <p><p>Represents a collection of API keys as represented by an <a>ApiKeys</a> resource.</p> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-api-keys.html">Use API Keys</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ApiKeys {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ApiKey>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>A list of warning messages logged during the import of API keys when the <code>failOnWarnings</code> option is set to true.</p>
    #[serde(rename = "warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// <p>API stage name of the associated API stage in a usage plan.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiStage {
    /// <p>API Id of the associated API stage in a usage plan.</p>
    #[serde(rename = "apiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    /// <p>API stage name of the associated API stage in a usage plan.</p>
    #[serde(rename = "stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    /// <p>Map containing method level throttling information for API stage in a usage plan.</p>
    #[serde(rename = "throttle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle: Option<::std::collections::HashMap<String, ThrottleSettings>>,
}

/// <p><p>Represents an authorization layer for methods. If enabled on a method, API Gateway will activate the authorizer when a client calls the method.</p> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/use-custom-authorizer.html">Enable custom authorization</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Authorizer {
    /// <p>Optional customer-defined field, used in OpenAPI imports and exports without functional impact.</p>
    #[serde(rename = "authType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// <p>Specifies the required credentials as an IAM role for API Gateway to invoke the authorizer. To specify an IAM role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To use resource-based permissions on the Lambda function, specify null.</p>
    #[serde(rename = "authorizerCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials: Option<String>,
    /// <p>The TTL in seconds of cached authorizer results. If it equals 0, authorization caching is disabled. If it is greater than 0, API Gateway will cache authorizer responses. If this field is not set, the default value is 300. The maximum value is 3600, or 1 hour.</p>
    #[serde(rename = "authorizerResultTtlInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i64>,
    /// <p>Specifies the authorizer's Uniform Resource Identifier (URI). For <code>TOKEN</code> or <code>REQUEST</code> authorizers, this must be a well-formed Lambda function URI, for example, <code>arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:{account_id}:function:{lambda_function_name}/invocations</code>. In general, the URI has this form <code>arn:aws:apigateway:{region}:lambda:path/{service_api}</code>, where <code>{region}</code> is the same as the region hosting the Lambda function, <code>path</code> indicates that the remaining substring in the URI should be treated as the path to the resource, including the initial <code>/</code>. For Lambda functions, this is usually of the form <code>/2015-03-31/functions/[FunctionARN]/invocations</code>.</p>
    #[serde(rename = "authorizerUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    /// <p>The identifier for the authorizer resource.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The identity source for which authorization is requested. <ul><li>For a <code>TOKEN</code> or <code>COGNITO_USER_POOLS</code> authorizer, this is required and specifies the request header mapping expression for the custom header holding the authorization token submitted by the client. For example, if the token header name is <code>Auth</code>, the header mapping expression is <code>method.request.header.Auth</code>.</li><li>For the <code>REQUEST</code> authorizer, this is required when authorization caching is enabled. The value is a comma-separated string of one or more mapping expressions of the specified request parameters. For example, if an <code>Auth</code> header, a <code>Name</code> query string parameter are defined as identity sources, this value is <code>method.request.header.Auth, method.request.querystring.Name</code>. These parameters will be used to derive the authorization caching key and to perform runtime validation of the <code>REQUEST</code> authorizer by verifying all of the identity-related request parameters are present, not null and non-empty. Only when this is true does the authorizer invoke the authorizer Lambda function, otherwise, it returns a 401 Unauthorized response without calling the Lambda function. The valid value is a string of comma-separated mapping expressions of the specified request parameters. When the authorization caching is not enabled, this property is optional.</li></ul></p>
    #[serde(rename = "identitySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<String>,
    /// <p>A validation expression for the incoming identity token. For <code>TOKEN</code> authorizers, this value is a regular expression. API Gateway will match the <code>aud</code> field of the incoming token from the client against the specified regular expression. It will invoke the authorizer's Lambda function when there is a match. Otherwise, it will return a 401 Unauthorized response without calling the Lambda function. The validation expression does not apply to the <code>REQUEST</code> authorizer.</p>
    #[serde(rename = "identityValidationExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    /// <p>[Required] The name of the authorizer.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A list of the Amazon Cognito user pool ARNs for the <code>COGNITO_USER_POOLS</code> authorizer. Each element is of this format: <code>arn:aws:cognito-idp:{region}:{account_id}:userpool/{user_pool_id}</code>. For a <code>TOKEN</code> or <code>REQUEST</code> authorizer, this is not defined. </p>
    #[serde(rename = "providerARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_ar_ns: Option<Vec<String>>,
    /// <p>The authorizer type. Valid values are <code>TOKEN</code> for a Lambda function using a single authorization token submitted in a custom header, <code>REQUEST</code> for a Lambda function using incoming request parameters, and <code>COGNITO_USER_POOLS</code> for using an Amazon Cognito user pool.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p><p>Represents a collection of <a>Authorizer</a> resources.</p> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/use-custom-authorizer.html">Enable custom authorization</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Authorizers {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Authorizer>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p><p>Represents the base path that callers of the API must provide as part of the URL after the domain name.</p> <div class="remarks">A custom domain name plus a <code>BasePathMapping</code> specification identifies a deployed <a>RestApi</a> in a given stage of the owner <a>Account</a>.</div> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-custom-domains.html">Use Custom Domain Names</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BasePathMapping {
    /// <p>The base path name that callers of the API must provide as part of the URL after the domain name.</p>
    #[serde(rename = "basePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_path: Option<String>,
    /// <p>The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_api_id: Option<String>,
    /// <p>The name of the associated stage.</p>
    #[serde(rename = "stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

/// <p><p>Represents a collection of <a>BasePathMapping</a> resources.</p> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-custom-domains.html">Use Custom Domain Names</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BasePathMappings {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<BasePathMapping>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>Configuration settings of a canary deployment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanarySettings {
    /// <p>The ID of the canary deployment.</p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The percent (0-100) of traffic diverted to a canary deployment.</p>
    #[serde(rename = "percentTraffic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_traffic: Option<f64>,
    /// <p>Stage variables overridden for a canary release deployment, including new stage variables introduced in the canary. These stage variables are represented as a string-to-string map between stage variable names and their values.</p>
    #[serde(rename = "stageVariableOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variable_overrides: Option<::std::collections::HashMap<String, String>>,
    /// <p>A Boolean flag to indicate whether the canary deployment uses the stage cache or not.</p>
    #[serde(rename = "useStageCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stage_cache: Option<bool>,
}

/// <p><p>Represents a client certificate used to configure client-side SSL authentication while sending requests to the integration endpoint.</p> <div class="remarks">Client certificates are used to authenticate an API by the backend server. To authenticate an API client (or user), use IAM roles and policies, a custom <a>Authorizer</a> or an Amazon Cognito user pool.</div> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/getting-started-client-side-ssl-authentication.html">Use Client-Side Certificate</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ClientCertificate {
    /// <p>The identifier of the client certificate.</p>
    #[serde(rename = "clientCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    /// <p>The timestamp when the client certificate was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The description of the client certificate.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The timestamp when the client certificate will expire.</p>
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<f64>,
    /// <p>The PEM-encoded public key of the client certificate, which can be used to configure certificate authentication in the integration endpoint .</p>
    #[serde(rename = "pemEncodedCertificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pem_encoded_certificate: Option<String>,
    /// <p>The collection of tags. Each tag element is associated with a given resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p><p>Represents a collection of <a>ClientCertificate</a> resources.</p> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/getting-started-client-side-ssl-authentication.html">Use Client-Side Certificate</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ClientCertificates {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ClientCertificate>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>Request to create an <a>ApiKey</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateApiKeyRequest {
    /// <p>An AWS Marketplace customer identifier , when integrating with the AWS SaaS Marketplace.</p>
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// <p>The description of the <a>ApiKey</a>.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Specifies whether the <a>ApiKey</a> can be used by callers.</p>
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Specifies whether (<code>true</code>) or not (<code>false</code>) the key identifier is distinct from the created API key value.</p>
    #[serde(rename = "generateDistinctId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_distinct_id: Option<bool>,
    /// <p>The name of the <a>ApiKey</a>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>DEPRECATED FOR USAGE PLANS - Specifies stages associated with the API key.</p>
    #[serde(rename = "stageKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_keys: Option<Vec<StageKey>>,
    /// <p>The key-value map of strings. The valid character set is [a-zA-Z+-=._:/]. The tag key can be up to 128 characters and must not start with <code>aws:</code>. The tag value can be up to 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Specifies a value of the API key.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Request to add a new <a>Authorizer</a> to an existing <a>RestApi</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAuthorizerRequest {
    /// <p>Optional customer-defined field, used in OpenAPI imports and exports without functional impact.</p>
    #[serde(rename = "authType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// <p>Specifies the required credentials as an IAM role for API Gateway to invoke the authorizer. To specify an IAM role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To use resource-based permissions on the Lambda function, specify null.</p>
    #[serde(rename = "authorizerCredentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_credentials: Option<String>,
    /// <p>The TTL in seconds of cached authorizer results. If it equals 0, authorization caching is disabled. If it is greater than 0, API Gateway will cache authorizer responses. If this field is not set, the default value is 300. The maximum value is 3600, or 1 hour.</p>
    #[serde(rename = "authorizerResultTtlInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i64>,
    /// <p>Specifies the authorizer's Uniform Resource Identifier (URI). For <code>TOKEN</code> or <code>REQUEST</code> authorizers, this must be a well-formed Lambda function URI, for example, <code>arn:aws:apigateway:us-west-2:lambda:path/2015-03-31/functions/arn:aws:lambda:us-west-2:{account_id}:function:{lambda_function_name}/invocations</code>. In general, the URI has this form <code>arn:aws:apigateway:{region}:lambda:path/{service_api}</code>, where <code>{region}</code> is the same as the region hosting the Lambda function, <code>path</code> indicates that the remaining substring in the URI should be treated as the path to the resource, including the initial <code>/</code>. For Lambda functions, this is usually of the form <code>/2015-03-31/functions/[FunctionARN]/invocations</code>.</p>
    #[serde(rename = "authorizerUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_uri: Option<String>,
    /// <p>The identity source for which authorization is requested. <ul><li>For a <code>TOKEN</code> or <code>COGNITO_USER_POOLS</code> authorizer, this is required and specifies the request header mapping expression for the custom header holding the authorization token submitted by the client. For example, if the token header name is <code>Auth</code>, the header mapping expression is <code>method.request.header.Auth</code>.</li><li>For the <code>REQUEST</code> authorizer, this is required when authorization caching is enabled. The value is a comma-separated string of one or more mapping expressions of the specified request parameters. For example, if an <code>Auth</code> header, a <code>Name</code> query string parameter are defined as identity sources, this value is <code>method.request.header.Auth, method.request.querystring.Name</code>. These parameters will be used to derive the authorization caching key and to perform runtime validation of the <code>REQUEST</code> authorizer by verifying all of the identity-related request parameters are present, not null and non-empty. Only when this is true does the authorizer invoke the authorizer Lambda function, otherwise, it returns a 401 Unauthorized response without calling the Lambda function. The valid value is a string of comma-separated mapping expressions of the specified request parameters. When the authorization caching is not enabled, this property is optional.</li></ul></p>
    #[serde(rename = "identitySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_source: Option<String>,
    /// <p>A validation expression for the incoming identity token. For <code>TOKEN</code> authorizers, this value is a regular expression. API Gateway will match the <code>aud</code> field of the incoming token from the client against the specified regular expression. It will invoke the authorizer's Lambda function when there is a match. Otherwise, it will return a 401 Unauthorized response without calling the Lambda function. The validation expression does not apply to the <code>REQUEST</code> authorizer.</p>
    #[serde(rename = "identityValidationExpression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
    /// <p>[Required] The name of the authorizer.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A list of the Amazon Cognito user pool ARNs for the <code>COGNITO_USER_POOLS</code> authorizer. Each element is of this format: <code>arn:aws:cognito-idp:{region}:{account_id}:userpool/{user_pool_id}</code>. For a <code>TOKEN</code> or <code>REQUEST</code> authorizer, this is not defined. </p>
    #[serde(rename = "providerARNs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_ar_ns: Option<Vec<String>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The authorizer type. Valid values are <code>TOKEN</code> for a Lambda function using a single authorization token submitted in a custom header, <code>REQUEST</code> for a Lambda function using incoming request parameters, and <code>COGNITO_USER_POOLS</code> for using an Amazon Cognito user pool.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p>Requests API Gateway to create a new <a>BasePathMapping</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateBasePathMappingRequest {
    /// <p>The base path name that callers of the API must provide as part of the URL after the domain name. This value must be unique for all of the mappings across a single API. Leave this blank if you do not want callers to specify a base path name after the domain name.</p>
    #[serde(rename = "basePath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_path: Option<String>,
    /// <p>[Required] The domain name of the <a>BasePathMapping</a> resource to create.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>The name of the API's stage that you want to use for this mapping. Leave this blank if you do not want callers to explicitly specify the stage name after any base path name.</p>
    #[serde(rename = "stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
}

/// <p>Requests API Gateway to create a <a>Deployment</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDeploymentRequest {
    /// <p>Enables a cache cluster for the <a>Stage</a> resource specified in the input.</p>
    #[serde(rename = "cacheClusterEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_enabled: Option<bool>,
    /// <p>Specifies the cache cluster size for the <a>Stage</a> resource specified in the input, if a cache cluster is enabled.</p>
    #[serde(rename = "cacheClusterSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_size: Option<String>,
    /// <p>The input configuration for the canary deployment when the deployment is a canary release deployment. </p>
    #[serde(rename = "canarySettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_settings: Option<DeploymentCanarySettings>,
    /// <p>The description for the <a>Deployment</a> resource to create.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>The description of the <a>Stage</a> resource for the <a>Deployment</a> resource to create.</p>
    #[serde(rename = "stageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_description: Option<String>,
    /// <p>The name of the <a>Stage</a> resource for the <a>Deployment</a> resource to create.</p>
    #[serde(rename = "stageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    /// <p>Specifies whether active tracing with X-ray is enabled for the <a>Stage</a>.</p>
    #[serde(rename = "tracingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_enabled: Option<bool>,
    /// <p>A map that defines the stage variables for the <a>Stage</a> resource that is associated with the new deployment. Variable names can have alphanumeric and underscore characters, and the values must match <code>[A-Za-z0-9-._~:/?#&amp;=,]+</code>.</p>
    #[serde(rename = "variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Creates a new documentation part of a given API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDocumentationPartRequest {
    /// <p>[Required] The location of the targeted API entity of the to-be-created documentation part.</p>
    #[serde(rename = "location")]
    pub location: DocumentationPartLocation,
    /// <p>[Required] The new documentation content map of the targeted API entity. Enclosed key-value pairs are API-specific, but only OpenAPI-compliant key-value pairs can be exported and, hence, published.</p>
    #[serde(rename = "properties")]
    pub properties: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Creates a new documentation version of a given API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDocumentationVersionRequest {
    /// <p>A description about the new documentation snapshot.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>[Required] The version identifier of the new snapshot.</p>
    #[serde(rename = "documentationVersion")]
    pub documentation_version: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>The stage name to be associated with the new documentation snapshot.</p>
    #[serde(rename = "stageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
}

/// <p>A request to create a new domain name.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDomainNameRequest {
    /// <p>The reference to an AWS-managed certificate that will be used by edge-optimized endpoint for this domain name. AWS Certificate Manager is the only supported source.</p>
    #[serde(rename = "certificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>[Deprecated] The body of the server certificate that will be used by edge-optimized endpoint for this domain name provided by your certificate authority.</p>
    #[serde(rename = "certificateBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_body: Option<String>,
    /// <p>[Deprecated] The intermediate certificates and optionally the root certificate, one after the other without any blank lines, used by an edge-optimized endpoint for this domain name. If you include the root certificate, your certificate chain must start with intermediate certificates and end with the root certificate. Use the intermediate certificates that were provided by your certificate authority. Do not include any intermediaries that are not in the chain of trust path.</p>
    #[serde(rename = "certificateChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_chain: Option<String>,
    /// <p>The user-friendly name of the certificate that will be used by edge-optimized endpoint for this domain name.</p>
    #[serde(rename = "certificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_name: Option<String>,
    /// <p>[Deprecated] Your edge-optimized endpoint's domain name certificate's private key.</p>
    #[serde(rename = "certificatePrivateKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_private_key: Option<String>,
    /// <p>[Required] The name of the <a>DomainName</a> resource.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// <p>The endpoint configuration of this <a>DomainName</a> showing the endpoint types of the domain name. </p>
    #[serde(rename = "endpointConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<EndpointConfiguration>,
    /// <p>The reference to an AWS-managed certificate that will be used by regional endpoint for this domain name. AWS Certificate Manager is the only supported source.</p>
    #[serde(rename = "regionalCertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regional_certificate_arn: Option<String>,
    /// <p>The user-friendly name of the certificate that will be used by regional endpoint for this domain name.</p>
    #[serde(rename = "regionalCertificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regional_certificate_name: Option<String>,
    /// <p>The key-value map of strings. The valid character set is [a-zA-Z+-=._:/]. The tag key can be up to 128 characters and must not start with <code>aws:</code>. The tag value can be up to 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Request to add a new <a>Model</a> to an existing <a>RestApi</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateModelRequest {
    /// <p>[Required] The content-type for the model.</p>
    #[serde(rename = "contentType")]
    pub content_type: String,
    /// <p>The description of the model.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>[Required] The name of the model. Must be alphanumeric.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>[Required] The <a>RestApi</a> identifier under which the <a>Model</a> will be created.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>The schema for the model. For <code>application/json</code> models, this should be <a href="https://tools.ietf.org/html/draft-zyp-json-schema-04" target="_blank">JSON schema draft 4</a> model.</p>
    #[serde(rename = "schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

/// <p>Creates a <a>RequestValidator</a> of a given <a>RestApi</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateRequestValidatorRequest {
    /// <p>The name of the to-be-created <a>RequestValidator</a>.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>A Boolean flag to indicate whether to validate request body according to the configured model schema for the method (<code>true</code>) or not (<code>false</code>).</p>
    #[serde(rename = "validateRequestBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_request_body: Option<bool>,
    /// <p>A Boolean flag to indicate whether to validate request parameters, <code>true</code>, or not <code>false</code>.</p>
    #[serde(rename = "validateRequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_request_parameters: Option<bool>,
}

/// <p>Requests API Gateway to create a <a>Resource</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateResourceRequest {
    /// <p>[Required] The parent resource's identifier.</p>
    #[serde(rename = "parentId")]
    pub parent_id: String,
    /// <p>The last path segment for this resource.</p>
    #[serde(rename = "pathPart")]
    pub path_part: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>The POST Request to add a new <a>RestApi</a> resource to your collection.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateRestApiRequest {
    /// <p>The source of the API key for metering requests according to a usage plan. Valid values are: <ul><li><code>HEADER</code> to read the API key from the <code>X-API-Key</code> header of a request. </li><li><code>AUTHORIZER</code> to read the API key from the <code>UsageIdentifierKey</code> from a custom authorizer.</li></ul> </p>
    #[serde(rename = "apiKeySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_source: Option<String>,
    /// <p>The list of binary media types supported by the <a>RestApi</a>. By default, the <a>RestApi</a> supports only UTF-8-encoded text payloads.</p>
    #[serde(rename = "binaryMediaTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_media_types: Option<Vec<String>>,
    /// <p>The ID of the <a>RestApi</a> that you want to clone from.</p>
    #[serde(rename = "cloneFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_from: Option<String>,
    /// <p>The description of the <a>RestApi</a>.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The endpoint configuration of this <a>RestApi</a> showing the endpoint types of the API. </p>
    #[serde(rename = "endpointConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<EndpointConfiguration>,
    /// <p>A nullable integer that is used to enable compression (with non-negative between 0 and 10485760 (10M) bytes, inclusive) or disable compression (with a null value) on an API. When compression is enabled, compression or decompression is not applied on the payload if the payload size is smaller than this value. Setting it to zero allows compression for any payload size.</p>
    #[serde(rename = "minimumCompressionSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_compression_size: Option<i64>,
    /// <p>[Required] The name of the <a>RestApi</a>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A stringified JSON policy document that applies to this RestApi regardless of the caller and <a>Method</a> configuration.</p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>The key-value map of strings. The valid character set is [a-zA-Z+-=._:/]. The tag key can be up to 128 characters and must not start with <code>aws:</code>. The tag value can be up to 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>A version identifier for the API.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Requests API Gateway to create a <a>Stage</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateStageRequest {
    /// <p>Whether cache clustering is enabled for the stage.</p>
    #[serde(rename = "cacheClusterEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_enabled: Option<bool>,
    /// <p>The stage's cache cluster size.</p>
    #[serde(rename = "cacheClusterSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_size: Option<String>,
    /// <p>The canary deployment settings of this stage.</p>
    #[serde(rename = "canarySettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_settings: Option<CanarySettings>,
    /// <p>[Required] The identifier of the <a>Deployment</a> resource for the <a>Stage</a> resource.</p>
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
    /// <p>The description of the <a>Stage</a> resource.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The version of the associated API documentation.</p>
    #[serde(rename = "documentationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_version: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The name for the <a>Stage</a> resource.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
    /// <p>The key-value map of strings. The valid character set is [a-zA-Z+-=._:/]. The tag key can be up to 128 characters and must not start with <code>aws:</code>. The tag value can be up to 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Specifies whether active tracing with X-ray is enabled for the <a>Stage</a>.</p>
    #[serde(rename = "tracingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_enabled: Option<bool>,
    /// <p>A map that defines the stage variables for the new <a>Stage</a> resource. Variable names can have alphanumeric and underscore characters, and the values must match <code>[A-Za-z0-9-._~:/?#&amp;=,]+</code>.</p>
    #[serde(rename = "variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The POST request to create a usage plan key for adding an existing API key to a usage plan.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateUsagePlanKeyRequest {
    /// <p>[Required] The identifier of a <a>UsagePlanKey</a> resource for a plan customer.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>[Required] The type of a <a>UsagePlanKey</a> resource for a plan customer.</p>
    #[serde(rename = "keyType")]
    pub key_type: String,
    /// <p>[Required] The Id of the <a>UsagePlan</a> resource representing the usage plan containing the to-be-created <a>UsagePlanKey</a> resource representing a plan customer.</p>
    #[serde(rename = "usagePlanId")]
    pub usage_plan_id: String,
}

/// <p>The POST request to create a usage plan with the name, description, throttle limits and quota limits, as well as the associated API stages, specified in the payload.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateUsagePlanRequest {
    /// <p>The associated API stages of the usage plan.</p>
    #[serde(rename = "apiStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_stages: Option<Vec<ApiStage>>,
    /// <p>The description of the usage plan.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>[Required] The name of the usage plan.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The quota of the usage plan.</p>
    #[serde(rename = "quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<QuotaSettings>,
    /// <p>The key-value map of strings. The valid character set is [a-zA-Z+-=._:/]. The tag key can be up to 128 characters and must not start with <code>aws:</code>. The tag value can be up to 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The throttling limits of the usage plan.</p>
    #[serde(rename = "throttle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle: Option<ThrottleSettings>,
}

/// <p>Creates a VPC link, under the caller's account in a selected region, in an asynchronous operation that typically takes 2-4 minutes to complete and become operational. The caller must have permissions to create and update VPC Endpoint services.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateVpcLinkRequest {
    /// <p>The description of the VPC link.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>[Required] The name used to label and identify the VPC link.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The key-value map of strings. The valid character set is [a-zA-Z+-=._:/]. The tag key can be up to 128 characters and must not start with <code>aws:</code>. The tag value can be up to 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>[Required] The ARNs of network load balancers of the VPC targeted by the VPC link. The network load balancers must be owned by the same AWS account of the API owner.</p>
    #[serde(rename = "targetArns")]
    pub target_arns: Vec<String>,
}

/// <p>A request to delete the <a>ApiKey</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApiKeyRequest {
    /// <p>[Required] The identifier of the <a>ApiKey</a> resource to be deleted.</p>
    #[serde(rename = "apiKey")]
    pub api_key: String,
}

/// <p>Request to delete an existing <a>Authorizer</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAuthorizerRequest {
    /// <p>[Required] The identifier of the <a>Authorizer</a> resource.</p>
    #[serde(rename = "authorizerId")]
    pub authorizer_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>A request to delete the <a>BasePathMapping</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteBasePathMappingRequest {
    /// <p>[Required] The base path name of the <a>BasePathMapping</a> resource to delete.</p>
    #[serde(rename = "basePath")]
    pub base_path: String,
    /// <p>[Required] The domain name of the <a>BasePathMapping</a> resource to delete.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

/// <p>A request to delete the <a>ClientCertificate</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteClientCertificateRequest {
    /// <p>[Required] The identifier of the <a>ClientCertificate</a> resource to be deleted.</p>
    #[serde(rename = "clientCertificateId")]
    pub client_certificate_id: String,
}

/// <p>Requests API Gateway to delete a <a>Deployment</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDeploymentRequest {
    /// <p>[Required] The identifier of the <a>Deployment</a> resource to delete.</p>
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Deletes an existing documentation part of an API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDocumentationPartRequest {
    /// <p>[Required] The identifier of the to-be-deleted documentation part.</p>
    #[serde(rename = "documentationPartId")]
    pub documentation_part_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Deletes an existing documentation version of an API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDocumentationVersionRequest {
    /// <p>[Required] The version identifier of a to-be-deleted documentation snapshot.</p>
    #[serde(rename = "documentationVersion")]
    pub documentation_version: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>A request to delete the <a>DomainName</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDomainNameRequest {
    /// <p>[Required] The name of the <a>DomainName</a> resource to be deleted.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

/// <p>Clears any customization of a <a>GatewayResponse</a> of a specified response type on the given <a>RestApi</a> and resets it with the default settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteGatewayResponseRequest {
    /// <p>[Required] <p>The response type of the associated <a>GatewayResponse</a>. Valid values are <ul><li>ACCESS_DENIED</li><li>API_CONFIGURATION_ERROR</li><li>AUTHORIZER_FAILURE</li><li> AUTHORIZER_CONFIGURATION_ERROR</li><li>BAD_REQUEST_PARAMETERS</li><li>BAD_REQUEST_BODY</li><li>DEFAULT_4XX</li><li>DEFAULT_5XX</li><li>EXPIRED_TOKEN</li><li>INVALID_SIGNATURE</li><li>INTEGRATION_FAILURE</li><li>INTEGRATION_TIMEOUT</li><li>INVALID_API_KEY</li><li>MISSING_AUTHENTICATION_TOKEN</li><li> QUOTA_EXCEEDED</li><li>REQUEST_TOO_LARGE</li><li>RESOURCE_NOT_FOUND</li><li>THROTTLED</li><li>UNAUTHORIZED</li><li>UNSUPPORTED_MEDIA_TYPE</li></ul> </p></p>
    #[serde(rename = "responseType")]
    pub response_type: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Represents a delete integration request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteIntegrationRequest {
    /// <p>[Required] Specifies a delete integration request's HTTP method.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>[Required] Specifies a delete integration request's resource identifier.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Represents a delete integration response request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteIntegrationResponseRequest {
    /// <p>[Required] Specifies a delete integration response request's HTTP method.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>[Required] Specifies a delete integration response request's resource identifier.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] Specifies a delete integration response request's status code.</p>
    #[serde(rename = "statusCode")]
    pub status_code: String,
}

/// <p>Request to delete an existing <a>Method</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteMethodRequest {
    /// <p>[Required] The HTTP verb of the <a>Method</a> resource.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>[Required] The <a>Resource</a> identifier for the <a>Method</a> resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>A request to delete an existing <a>MethodResponse</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteMethodResponseRequest {
    /// <p>[Required] The HTTP verb of the <a>Method</a> resource.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>[Required] The <a>Resource</a> identifier for the <a>MethodResponse</a> resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The status code identifier for the <a>MethodResponse</a> resource.</p>
    #[serde(rename = "statusCode")]
    pub status_code: String,
}

/// <p>Request to delete an existing model in an existing <a>RestApi</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteModelRequest {
    /// <p>[Required] The name of the model to delete.</p>
    #[serde(rename = "modelName")]
    pub model_name: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Deletes a specified <a>RequestValidator</a> of a given <a>RestApi</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRequestValidatorRequest {
    /// <p>[Required] The identifier of the <a>RequestValidator</a> to be deleted.</p>
    #[serde(rename = "requestValidatorId")]
    pub request_validator_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to delete a <a>Resource</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteResourceRequest {
    /// <p>[Required] The identifier of the <a>Resource</a> resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to delete the specified API from your collection.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteRestApiRequest {
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Requests API Gateway to delete a <a>Stage</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteStageRequest {
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The name of the <a>Stage</a> resource to delete.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
}

/// <p>The DELETE request to delete a usage plan key and remove the underlying API key from the associated usage plan.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteUsagePlanKeyRequest {
    /// <p>[Required] The Id of the <a>UsagePlanKey</a> resource to be deleted.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>[Required] The Id of the <a>UsagePlan</a> resource representing the usage plan containing the to-be-deleted <a>UsagePlanKey</a> resource representing a plan customer.</p>
    #[serde(rename = "usagePlanId")]
    pub usage_plan_id: String,
}

/// <p>The DELETE request to delete a usage plan of a given plan Id.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteUsagePlanRequest {
    /// <p>[Required] The Id of the to-be-deleted usage plan.</p>
    #[serde(rename = "usagePlanId")]
    pub usage_plan_id: String,
}

/// <p>Deletes an existing <a>VpcLink</a> of a specified identifier.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteVpcLinkRequest {
    /// <p>[Required] The identifier of the <a>VpcLink</a>. It is used in an <a>Integration</a> to reference this <a>VpcLink</a>.</p>
    #[serde(rename = "vpcLinkId")]
    pub vpc_link_id: String,
}

/// <p><p>An immutable representation of a <a>RestApi</a> resource that can be called by users using <a>Stages</a>. A deployment must be associated with a <a>Stage</a> for it to be callable over the Internet.</p> <div class="remarks">To create a deployment, call <code>POST</code> on the <a>Deployments</a> resource of a <a>RestApi</a>. To view, update, or delete a deployment, call <code>GET</code>, <code>PATCH</code>, or <code>DELETE</code> on the specified deployment resource (<code>/restapis/{restapi<em>id}/deployments/{deployment</em>id}</code>).</div> <div class="seeAlso"><a>RestApi</a>, <a>Deployments</a>, <a>Stage</a>, <a href="https://docs.aws.amazon.com/cli/latest/reference/apigateway/get-deployment.html">AWS CLI</a>, <a href="https://aws.amazon.com/tools/">AWS SDKs</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Deployment {
    /// <p>A summary of the <a>RestApi</a> at the date and time that the deployment resource was created.</p>
    #[serde(rename = "apiSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_summary: Option<
        ::std::collections::HashMap<String, ::std::collections::HashMap<String, MethodSnapshot>>,
    >,
    /// <p>The date and time that the deployment resource was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The description for the deployment resource.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier for the deployment resource.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>The input configuration for a canary deployment.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeploymentCanarySettings {
    /// <p>The percentage (0.0-100.0) of traffic routed to the canary deployment.</p>
    #[serde(rename = "percentTraffic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_traffic: Option<f64>,
    /// <p>A stage variable overrides used for the canary release deployment. They can override existing stage variables or add new stage variables for the canary release deployment. These stage variables are represented as a string-to-string map between stage variable names and their values.</p>
    #[serde(rename = "stageVariableOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variable_overrides: Option<::std::collections::HashMap<String, String>>,
    /// <p>A Boolean flag to indicate whether the canary release deployment uses the stage cache or not.</p>
    #[serde(rename = "useStageCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_stage_cache: Option<bool>,
}

/// <p><p>Represents a collection resource that contains zero or more references to your existing deployments, and links that guide you on how to interact with your collection. The collection offers a paginated view of the contained deployments.</p> <div class="remarks">To create a new deployment of a <a>RestApi</a>, make a <code>POST</code> request against this resource. To view, update, or delete an existing deployment, make a <code>GET</code>, <code>PATCH</code>, or <code>DELETE</code> request, respectively, on a specified <a>Deployment</a> resource.</div> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-deploy-api.html">Deploying an API</a>, <a href="https://docs.aws.amazon.com/cli/latest/reference/apigateway/get-deployment.html">AWS CLI</a>, <a href="https://aws.amazon.com/tools/">AWS SDKs</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Deployments {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Deployment>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p><p>A documentation part for a targeted API entity.</p> <div class="remarks"> <p>A documentation part consists of a content map (<code>properties</code>) and a target (<code>location</code>). The target specifies an API entity to which the documentation content applies. The supported API entity types are <code>API</code>, <code>AUTHORIZER</code>, <code>MODEL</code>, <code>RESOURCE</code>, <code>METHOD</code>, <code>PATH<em>PARAMETER</code>, <code>QUERY</em>PARAMETER</code>, <code>REQUEST<em>HEADER</code>, <code>REQUEST</em>BODY</code>, <code>RESPONSE</code>, <code>RESPONSE<em>HEADER</code>, and <code>RESPONSE</em>BODY</code>. Valid <code>location</code> fields depend on the API entity type. All valid fields are not required.</p> <p>The content map is a JSON string of API-specific key-value pairs. Although an API can use any shape for the content map, only the OpenAPI-compliant documentation fields will be injected into the associated API entity definition in the exported OpenAPI definition file.</p></div> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-documenting-api.html">Documenting an API</a>, <a>DocumentationParts</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DocumentationPart {
    /// <p>The <a>DocumentationPart</a> identifier, generated by API Gateway when the <code>DocumentationPart</code> is created.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The location of the API entity to which the documentation applies. Valid fields depend on the targeted API entity type. All the valid location fields are not required. If not explicitly specified, a valid location field is treated as a wildcard and associated documentation content may be inherited by matching entities, unless overridden.</p>
    #[serde(rename = "location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<DocumentationPartLocation>,
    /// <p>A content map of API-specific key-value pairs describing the targeted API entity. The map must be encoded as a JSON string, e.g., <code>"{ \"description\": \"The API does ...\" }"</code>. Only OpenAPI-compliant documentation-related fields from the <literal>properties</literal> map are exported and, hence, published as part of the API entity definitions, while the original documentation parts are exported in a OpenAPI extension of <code>x-amazon-apigateway-documentation</code>.</p>
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<String>,
}

/// <p><p>A collection of the imported <a>DocumentationPart</a> identifiers.</p> <div class="remarks">This is used to return the result when documentation parts in an external (e.g., OpenAPI) file are imported into API Gateway</div> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-documenting-api.html">Documenting an API</a>, <a href="https://docs.aws.amazon.com/apigateway/api-reference/link-relation/documentationpart-import/">documentationpart:import</a>, <a>DocumentationPart</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DocumentationPartIds {
    /// <p>A list of the returned documentation part identifiers.</p>
    #[serde(rename = "ids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    /// <p>A list of warning messages reported during import of documentation parts.</p>
    #[serde(rename = "warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// <p>Specifies the target API entity to which the documentation applies.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentationPartLocation {
    /// <p>The HTTP verb of a method. It is a valid field for the API entity types of <code>METHOD</code>, <code>PATH_PARAMETER</code>, <code>QUERY_PARAMETER</code>, <code>REQUEST_HEADER</code>, <code>REQUEST_BODY</code>, <code>RESPONSE</code>, <code>RESPONSE_HEADER</code>, and <code>RESPONSE_BODY</code>. The default value is <code>*</code> for any method. When an applicable child entity inherits the content of an entity of the same type with more general specifications of the other <code>location</code> attributes, the child entity's <code>method</code> attribute must match that of the parent entity exactly.</p>
    #[serde(rename = "method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// <p>The name of the targeted API entity. It is a valid and required field for the API entity types of <code>AUTHORIZER</code>, <code>MODEL</code>, <code>PATH_PARAMETER</code>, <code>QUERY_PARAMETER</code>, <code>REQUEST_HEADER</code>, <code>REQUEST_BODY</code> and <code>RESPONSE_HEADER</code>. It is an invalid field for any other entity type.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The URL path of the target. It is a valid field for the API entity types of <code>RESOURCE</code>, <code>METHOD</code>, <code>PATH_PARAMETER</code>, <code>QUERY_PARAMETER</code>, <code>REQUEST_HEADER</code>, <code>REQUEST_BODY</code>, <code>RESPONSE</code>, <code>RESPONSE_HEADER</code>, and <code>RESPONSE_BODY</code>. The default value is <code>/</code> for the root resource. When an applicable child entity inherits the content of another entity of the same type with more general specifications of the other <code>location</code> attributes, the child entity's <code>path</code> attribute must match that of the parent entity as a prefix.</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The HTTP status code of a response. It is a valid field for the API entity types of <code>RESPONSE</code>, <code>RESPONSE_HEADER</code>, and <code>RESPONSE_BODY</code>. The default value is <code>*</code> for any status code. When an applicable child entity inherits the content of an entity of the same type with more general specifications of the other <code>location</code> attributes, the child entity's <code>statusCode</code> attribute must match that of the parent entity exactly.</p>
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    /// <p>[Required] The type of API entity to which the documentation content applies. Valid values are <code>API</code>, <code>AUTHORIZER</code>, <code>MODEL</code>, <code>RESOURCE</code>, <code>METHOD</code>, <code>PATH_PARAMETER</code>, <code>QUERY_PARAMETER</code>, <code>REQUEST_HEADER</code>, <code>REQUEST_BODY</code>, <code>RESPONSE</code>, <code>RESPONSE_HEADER</code>, and <code>RESPONSE_BODY</code>. Content inheritance does not apply to any entity of the <code>API</code>, <code>AUTHORIZER</code>, <code>METHOD</code>, <code>MODEL</code>, <code>REQUEST_BODY</code>, or <code>RESOURCE</code> type.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

/// <p><p>The collection of documentation parts of an API.</p> <div class="remarks"/> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-documenting-api.html">Documenting an API</a>, <a>DocumentationPart</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DocumentationParts {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DocumentationPart>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p><p>A snapshot of the documentation of an API.</p> <div class="remarks"><p>Publishing API documentation involves creating a documentation version associated with an API stage and exporting the versioned documentation to an external (e.g., OpenAPI) file.</p></div> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-documenting-api.html">Documenting an API</a>, <a>DocumentationPart</a>, <a>DocumentationVersions</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DocumentationVersion {
    /// <p>The date when the API documentation snapshot is created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The description of the API documentation snapshot.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The version identifier of the API documentation snapshot.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p><p>The collection of documentation snapshots of an API. </p> <div class="remarks"><p>Use the <a>DocumentationVersions</a> to manage documentation snapshots associated with various API stages.</p></div> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-documenting-api.html">Documenting an API</a>, <a>DocumentationPart</a>, <a>DocumentationVersion</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DocumentationVersions {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DocumentationVersion>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p><p>Represents a custom domain name as a user-friendly host name of an API (<a>RestApi</a>).</p> <div class="Remarks"> <p>When you deploy an API, API Gateway creates a default host name for the API. This default API host name is of the <code>{restapi-id}.execute-api.{region}.amazonaws.com</code> format. With the default host name, you can access the API&#39;s root resource with the URL of <code>https://{restapi-id}.execute-api.{region}.amazonaws.com/{stage}/</code>. When you set up a custom domain name of <code>apis.example.com</code> for this API, you can then access the same resource using the URL of the <code>https://apis.examples.com/myApi</code>, where <code>myApi</code> is the base path mapping (<a>BasePathMapping</a>) of your API under the custom domain name. </p> </div> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-custom-domains.html">Set a Custom Host Name for an API</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DomainName {
    /// <p>The reference to an AWS-managed certificate that will be used by edge-optimized endpoint for this domain name. AWS Certificate Manager is the only supported source.</p>
    #[serde(rename = "certificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The name of the certificate that will be used by edge-optimized endpoint for this domain name.</p>
    #[serde(rename = "certificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_name: Option<String>,
    /// <p>The timestamp when the certificate that was used by edge-optimized endpoint for this domain name was uploaded.</p>
    #[serde(rename = "certificateUploadDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_upload_date: Option<f64>,
    /// <p>The domain name of the Amazon CloudFront distribution associated with this custom domain name for an edge-optimized endpoint. You set up this association when adding a DNS record pointing the custom domain name to this distribution name. For more information about CloudFront distributions, see the <a href="https://aws.amazon.com/documentation/cloudfront/" target="_blank">Amazon CloudFront documentation</a>.</p>
    #[serde(rename = "distributionDomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_domain_name: Option<String>,
    /// <p>The region-agnostic Amazon Route 53 Hosted Zone ID of the edge-optimized endpoint. The valid value is <code>Z2FDTNDATAQYW2</code> for all the regions. For more information, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-regional-api-custom-domain-create.html">Set up a Regional Custom Domain Name</a> and <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html#apigateway_region">AWS Regions and Endpoints for API Gateway</a>. </p>
    #[serde(rename = "distributionHostedZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution_hosted_zone_id: Option<String>,
    /// <p>The custom domain name as an API host name, for example, <code>my-api.example.com</code>.</p>
    #[serde(rename = "domainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    /// <p>The endpoint configuration of this <a>DomainName</a> showing the endpoint types of the domain name. </p>
    #[serde(rename = "endpointConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<EndpointConfiguration>,
    /// <p>The reference to an AWS-managed certificate that will be used for validating the regional domain name. AWS Certificate Manager is the only supported source.</p>
    #[serde(rename = "regionalCertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regional_certificate_arn: Option<String>,
    /// <p>The name of the certificate that will be used for validating the regional domain name.</p>
    #[serde(rename = "regionalCertificateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regional_certificate_name: Option<String>,
    /// <p>The domain name associated with the regional endpoint for this custom domain name. You set up this association by adding a DNS record that points the custom domain name to this regional domain name. The regional domain name is returned by API Gateway when you create a regional endpoint.</p>
    #[serde(rename = "regionalDomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regional_domain_name: Option<String>,
    /// <p>The region-specific Amazon Route 53 Hosted Zone ID of the regional endpoint. For more information, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/apigateway-regional-api-custom-domain-create.html">Set up a Regional Custom Domain Name</a> and <a href="https://docs.aws.amazon.com/general/latest/gr/rande.html#apigateway_region">AWS Regions and Endpoints for API Gateway</a>. </p>
    #[serde(rename = "regionalHostedZoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regional_hosted_zone_id: Option<String>,
    /// <p>The collection of tags. Each tag element is associated with a given resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p><p>Represents a collection of <a>DomainName</a> resources.</p> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-custom-domains.html">Use Client-Side Certificate</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DomainNames {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DomainName>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>The endpoint configuration to indicate the types of endpoints an API (<a>RestApi</a>) or its custom domain name (<a>DomainName</a>) has. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndpointConfiguration {
    /// <p>A list of endpoint types of an API (<a>RestApi</a>) or its custom domain name (<a>DomainName</a>). For an edge-optimized API and its custom domain name, the endpoint type is <code>"EDGE"</code>. For a regional API and its custom domain name, the endpoint type is <code>REGIONAL</code>. For a private API, the endpoint type is <code>PRIVATE</code>.</p>
    #[serde(rename = "types")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

/// <p>The binary blob response to <a>GetExport</a>, which contains the generated SDK.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ExportResponse {
    /// <p>The binary blob response to <a>GetExport</a>, which contains the export.</p>
    pub body: Option<bytes::Bytes>,
    /// <p>The content-disposition header value in the HTTP response.</p>
    pub content_disposition: Option<String>,
    /// <p>The content-type header value in the HTTP response. This will correspond to a valid 'accept' type in the request.</p>
    pub content_type: Option<String>,
}

/// <p>Request to flush authorizer cache entries on a specified stage.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct FlushStageAuthorizersCacheRequest {
    /// <p>The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>The name of the stage to flush.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
}

/// <p>Requests API Gateway to flush a stage's cache.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct FlushStageCacheRequest {
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The name of the stage to flush its cache.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
}

/// <p><p>A gateway response of a given response type and status code, with optional response parameters and mapping templates.</p> <div class="remarks"> For more information about valid gateway response types, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/supported-gateway-response-types.html">Gateway Response Types Supported by API Gateway</a> <div class="example"> <h4>Example: Get a Gateway Response of a given response type</h4> <h5>Request</h5> <p>This example shows how to get a gateway response of the <code>MISSING<em>AUTHENTICATION</em>TOKEN</code> type.</p> <pre><code>GET /restapis/o81lxisefl/gatewayresponses/MISSING<em>AUTHENTICATION</em>TOKEN HTTP/1.1 Host: beta-apigateway.us-east-1.amazonaws.com Content-Type: application/json X-Amz-Date: 20170503T202516Z Authorization: AWS4-HMAC-SHA256 Credential={access-key-id}/20170503/us-east-1/apigateway/aws4<em>request, SignedHeaders=content-type;host;x-amz-date, Signature=1b52460e3159c1a26cff29093855d50ea141c1c5b937528fecaf60f51129697a Cache-Control: no-cache Postman-Token: 3b2a1ce9-c848-2e26-2e2f-9c2caefbed45 </code></pre> <p>The response type is specified as a URL path.</p> <h5>Response</h5> <p>The successful operation returns the <code>200 OK</code> status code and a payload similar to the following:</p> <pre><code>{ &quot;</em>links&quot;: { &quot;curies&quot;: { &quot;href&quot;: &quot;http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-gatewayresponse-{rel}.html&quot;, &quot;name&quot;: &quot;gatewayresponse&quot;, &quot;templated&quot;: true }, &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/MISSING<em>AUTHENTICATION</em>TOKEN&quot; }, &quot;gatewayresponse:delete&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/MISSING<em>AUTHENTICATION</em>TOKEN&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response<em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/MISSING</em>AUTHENTICATION<em>TOKEN&quot; } }, &quot;defaultResponse&quot;: false, &quot;responseParameters&quot;: { &quot;gatewayresponse.header.x-request-path&quot;: &quot;method.request.path.petId&quot;, &quot;gatewayresponse.header.Access-Control-Allow-Origin&quot;: &quot;&apos;a.b.c&apos;&quot;, &quot;gatewayresponse.header.x-request-query&quot;: &quot;method.request.querystring.q&quot;, &quot;gatewayresponse.header.x-request-header&quot;: &quot;method.request.header.Accept&quot; }, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{\n &quot;message&quot;: $context.error.messageString,\n &quot;type&quot;: &quot;$context.error.responseType&quot;,\n &quot;stage&quot;: &quot;$context.stage&quot;,\n &quot;resourcePath&quot;: &quot;$context.resourcePath&quot;,\n &quot;stageVariables.a&quot;: &quot;$stageVariables.a&quot;,\n &quot;statusCode&quot;: &quot;&apos;404&apos;&quot;\n}&quot; }, &quot;responseType&quot;: &quot;MISSING</em>AUTHENTICATION_TOKEN&quot;, &quot;statusCode&quot;: &quot;404&quot; }</code></pre> <p></p> </div> </div> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/customize-gateway-responses.html">Customize Gateway Responses</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GatewayResponse {
    /// <p>A Boolean flag to indicate whether this <a>GatewayResponse</a> is the default gateway response (<code>true</code>) or not (<code>false</code>). A default gateway response is one generated by API Gateway without any customization by an API developer. </p>
    #[serde(rename = "defaultResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_response: Option<bool>,
    /// <p>Response parameters (paths, query strings and headers) of the <a>GatewayResponse</a> as a string-to-string map of key-value pairs.</p>
    #[serde(rename = "responseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Response templates of the <a>GatewayResponse</a> as a string-to-string map of key-value pairs.</p>
    #[serde(rename = "responseTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>The response type of the associated <a>GatewayResponse</a>. Valid values are <ul><li>ACCESS_DENIED</li><li>API_CONFIGURATION_ERROR</li><li>AUTHORIZER_FAILURE</li><li> AUTHORIZER_CONFIGURATION_ERROR</li><li>BAD_REQUEST_PARAMETERS</li><li>BAD_REQUEST_BODY</li><li>DEFAULT_4XX</li><li>DEFAULT_5XX</li><li>EXPIRED_TOKEN</li><li>INVALID_SIGNATURE</li><li>INTEGRATION_FAILURE</li><li>INTEGRATION_TIMEOUT</li><li>INVALID_API_KEY</li><li>MISSING_AUTHENTICATION_TOKEN</li><li> QUOTA_EXCEEDED</li><li>REQUEST_TOO_LARGE</li><li>RESOURCE_NOT_FOUND</li><li>THROTTLED</li><li>UNAUTHORIZED</li><li>UNSUPPORTED_MEDIA_TYPE</li></ul> </p>
    #[serde(rename = "responseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_type: Option<String>,
    /// <p>The HTTP status code for this <a>GatewayResponse</a>.</p>
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

/// <p><p>The collection of the <a>GatewayResponse</a> instances of a <a>RestApi</a> as a <code>responseType</code>-to-<a>GatewayResponse</a> object map of key-value pairs. As such, pagination is not supported for querying this collection.</p> <div class="remarks"> For more information about valid gateway response types, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/supported-gateway-response-types.html">Gateway Response Types Supported by API Gateway</a> <div class="example"> <h4>Example: Get the collection of gateway responses of an API</h4> <h5>Request</h5> <p>This example request shows how to retrieve the <a>GatewayResponses</a> collection from an API.</p> <pre><code>GET /restapis/o81lxisefl/gatewayresponses HTTP/1.1 Host: beta-apigateway.us-east-1.amazonaws.com Content-Type: application/json X-Amz-Date: 20170503T220604Z Authorization: AWS4-HMAC-SHA256 Credential={access-key-id}/20170503/us-east-1/apigateway/aws4<em>request, SignedHeaders=content-type;host;x-amz-date, Signature=59b42fe54a76a5de8adf2c67baa6d39206f8e9ad49a1d77ccc6a5da3103a398a Cache-Control: no-cache Postman-Token: 5637af27-dc29-fc5c-9dfe-0645d52cb515 </code></pre> <p></p> <h5>Response</h5> <p>The successful operation returns the <code>200 OK</code> status code and a payload similar to the following:</p> <pre><code>{ &quot;</em>links&quot;: { &quot;curies&quot;: { &quot;href&quot;: &quot;http://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-gatewayresponse-{rel}.html&quot;, &quot;name&quot;: &quot;gatewayresponse&quot;, &quot;templated&quot;: true }, &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses&quot; }, &quot;first&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses&quot; }, &quot;gatewayresponse:by-type&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response<em>type}&quot;, &quot;templated&quot;: true }, &quot;item&quot;: [ { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INTEGRATION</em>FAILURE&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/RESOURCE<em>NOT</em>FOUND&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/REQUEST<em>TOO</em>LARGE&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/THROTTLED&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/UNSUPPORTED<em>MEDIA</em>TYPE&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/AUTHORIZER<em>CONFIGURATION</em>ERROR&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/DEFAULT<em>5XX&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/DEFAULT</em>4XX&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/BAD<em>REQUEST</em>PARAMETERS&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/BAD<em>REQUEST</em>BODY&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/EXPIRED<em>TOKEN&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/ACCESS</em>DENIED&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INVALID<em>API</em>KEY&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/UNAUTHORIZED&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/API<em>CONFIGURATION</em>ERROR&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/QUOTA<em>EXCEEDED&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INTEGRATION</em>TIMEOUT&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/MISSING<em>AUTHENTICATION</em>TOKEN&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INVALID<em>SIGNATURE&quot; }, { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/AUTHORIZER</em>FAILURE&quot; } ] }, &quot;<em>embedded&quot;: { &quot;item&quot;: [ { &quot;</em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INTEGRATION<em>FAILURE&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INTEGRATION<em>FAILURE&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;INTEGRATION</em>FAILURE&quot;, &quot;statusCode&quot;: &quot;504&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/RESOURCE</em>NOT<em>FOUND&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/RESOURCE<em>NOT</em>FOUND&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;RESOURCE<em>NOT</em>FOUND&quot;, &quot;statusCode&quot;: &quot;404&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/REQUEST</em>TOO<em>LARGE&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/REQUEST<em>TOO</em>LARGE&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;REQUEST<em>TOO</em>LARGE&quot;, &quot;statusCode&quot;: &quot;413&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/THROTTLED&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/THROTTLED&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;THROTTLED&quot;, &quot;statusCode&quot;: &quot;429&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/UNSUPPORTED</em>MEDIA<em>TYPE&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/UNSUPPORTED<em>MEDIA</em>TYPE&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;UNSUPPORTED<em>MEDIA</em>TYPE&quot;, &quot;statusCode&quot;: &quot;415&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/AUTHORIZER</em>CONFIGURATION<em>ERROR&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/AUTHORIZER<em>CONFIGURATION</em>ERROR&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;AUTHORIZER<em>CONFIGURATION</em>ERROR&quot;, &quot;statusCode&quot;: &quot;500&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/DEFAULT</em>5XX&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response<em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/DEFAULT</em>5XX&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;DEFAULT<em>5XX&quot; }, { &quot;</em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/DEFAULT<em>4XX&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/DEFAULT<em>4XX&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;DEFAULT</em>4XX&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/BAD</em>REQUEST<em>PARAMETERS&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/BAD<em>REQUEST</em>PARAMETERS&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;BAD<em>REQUEST</em>PARAMETERS&quot;, &quot;statusCode&quot;: &quot;400&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/BAD</em>REQUEST<em>BODY&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/BAD<em>REQUEST</em>BODY&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;BAD<em>REQUEST</em>BODY&quot;, &quot;statusCode&quot;: &quot;400&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/EXPIRED</em>TOKEN&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response<em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/EXPIRED</em>TOKEN&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;EXPIRED<em>TOKEN&quot;, &quot;statusCode&quot;: &quot;403&quot; }, { &quot;</em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/ACCESS<em>DENIED&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/ACCESS<em>DENIED&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;ACCESS</em>DENIED&quot;, &quot;statusCode&quot;: &quot;403&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INVALID</em>API<em>KEY&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INVALID<em>API</em>KEY&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;INVALID<em>API</em>KEY&quot;, &quot;statusCode&quot;: &quot;403&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/UNAUTHORIZED&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/UNAUTHORIZED&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;UNAUTHORIZED&quot;, &quot;statusCode&quot;: &quot;401&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/API</em>CONFIGURATION<em>ERROR&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/API<em>CONFIGURATION</em>ERROR&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;API<em>CONFIGURATION</em>ERROR&quot;, &quot;statusCode&quot;: &quot;500&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/QUOTA</em>EXCEEDED&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response<em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/QUOTA</em>EXCEEDED&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;QUOTA<em>EXCEEDED&quot;, &quot;statusCode&quot;: &quot;429&quot; }, { &quot;</em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INTEGRATION<em>TIMEOUT&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INTEGRATION<em>TIMEOUT&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;INTEGRATION</em>TIMEOUT&quot;, &quot;statusCode&quot;: &quot;504&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/MISSING</em>AUTHENTICATION<em>TOKEN&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/MISSING<em>AUTHENTICATION</em>TOKEN&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;MISSING<em>AUTHENTICATION</em>TOKEN&quot;, &quot;statusCode&quot;: &quot;403&quot; }, { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INVALID</em>SIGNATURE&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response<em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/INVALID</em>SIGNATURE&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;INVALID<em>SIGNATURE&quot;, &quot;statusCode&quot;: &quot;403&quot; }, { &quot;</em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/AUTHORIZER<em>FAILURE&quot; }, &quot;gatewayresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/{response</em>type}&quot;, &quot;templated&quot;: true }, &quot;gatewayresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/o81lxisefl/gatewayresponses/AUTHORIZER<em>FAILURE&quot; } }, &quot;defaultResponse&quot;: true, &quot;responseParameters&quot;: {}, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;{&quot;message&quot;:$context.error.messageString}&quot; }, &quot;responseType&quot;: &quot;AUTHORIZER</em>FAILURE&quot;, &quot;statusCode&quot;: &quot;500&quot; } ] } }</code></pre> <p></p> </div> </div> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/customize-gateway-responses.html">Customize Gateway Responses</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GatewayResponses {
    /// <p>Returns the entire collection, because of no pagination support.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<GatewayResponse>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>A request to generate a <a>ClientCertificate</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GenerateClientCertificateRequest {
    /// <p>The description of the <a>ClientCertificate</a>.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The key-value map of strings. The valid character set is [a-zA-Z+-=._:/]. The tag key can be up to 128 characters and must not start with <code>aws:</code>. The tag value can be up to 256 characters.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Requests API Gateway to get information about the current <a>Account</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAccountRequest {}

/// <p>A request to get information about the current <a>ApiKey</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetApiKeyRequest {
    /// <p>[Required] The identifier of the <a>ApiKey</a> resource.</p>
    #[serde(rename = "apiKey")]
    pub api_key: String,
    /// <p>A boolean flag to specify whether (<code>true</code>) or not (<code>false</code>) the result contains the key value.</p>
    #[serde(rename = "includeValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_value: Option<bool>,
}

/// <p>A request to get information about the current <a>ApiKeys</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetApiKeysRequest {
    /// <p>The identifier of a customer in AWS Marketplace or an external system, such as a developer portal.</p>
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<String>,
    /// <p>A boolean flag to specify whether (<code>true</code>) or not (<code>false</code>) the result contains key values.</p>
    #[serde(rename = "includeValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_values: Option<bool>,
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The name of queried API keys.</p>
    #[serde(rename = "nameQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_query: Option<String>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>Request to describe an existing <a>Authorizer</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAuthorizerRequest {
    /// <p>[Required] The identifier of the <a>Authorizer</a> resource.</p>
    #[serde(rename = "authorizerId")]
    pub authorizer_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to describe an existing <a>Authorizers</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAuthorizersRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to describe a <a>BasePathMapping</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBasePathMappingRequest {
    /// <p>[Required] The base path name that callers of the API must provide as part of the URL after the domain name. This value must be unique for all of the mappings across a single API. Leave this blank if you do not want callers to specify any base path name after the domain name.</p>
    #[serde(rename = "basePath")]
    pub base_path: String,
    /// <p>[Required] The domain name of the <a>BasePathMapping</a> resource to be described.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

/// <p>A request to get information about a collection of <a>BasePathMapping</a> resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBasePathMappingsRequest {
    /// <p>[Required] The domain name of a <a>BasePathMapping</a> resource.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>A request to get information about the current <a>ClientCertificate</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetClientCertificateRequest {
    /// <p>[Required] The identifier of the <a>ClientCertificate</a> resource to be described.</p>
    #[serde(rename = "clientCertificateId")]
    pub client_certificate_id: String,
}

/// <p>A request to get information about a collection of <a>ClientCertificate</a> resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetClientCertificatesRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>Requests API Gateway to get information about a <a>Deployment</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeploymentRequest {
    /// <p>[Required] The identifier of the <a>Deployment</a> resource to get information about.</p>
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
    /// <p>A query parameter to retrieve the specified embedded resources of the returned <a>Deployment</a> resource in the response. In a REST API call, this <code>embed</code> parameter value is a list of comma-separated strings, as in <code>GET /restapis/{restapi_id}/deployments/{deployment_id}?embed=var1,var2</code>. The SDK and other platform-dependent libraries might use a different format for the list. Currently, this request supports only retrieval of the embedded API summary this way. Hence, the parameter value must be a single-valued list containing only the <code>"apisummary"</code> string. For example, <code>GET /restapis/{restapi_id}/deployments/{deployment_id}?embed=apisummary</code>.</p>
    #[serde(rename = "embed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<Vec<String>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Requests API Gateway to get information about a <a>Deployments</a> collection.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDeploymentsRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Gets a specified documentation part of a given API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDocumentationPartRequest {
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "documentationPartId")]
    pub documentation_part_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Gets the documentation parts of an API. The result may be filtered by the type, name, or path of API entities (targets).</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDocumentationPartsRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The status of the API documentation parts to retrieve. Valid values are <code>DOCUMENTED</code> for retrieving <a>DocumentationPart</a> resources with content and <code>UNDOCUMENTED</code> for <a>DocumentationPart</a> resources without content.</p>
    #[serde(rename = "locationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_status: Option<String>,
    /// <p>The name of API entities of the to-be-retrieved documentation parts.</p>
    #[serde(rename = "nameQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_query: Option<String>,
    /// <p>The path of API entities of the to-be-retrieved documentation parts.</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>The type of API entities of the to-be-retrieved documentation parts. </p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Gets a documentation snapshot of an API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDocumentationVersionRequest {
    /// <p>[Required] The version identifier of the to-be-retrieved documentation snapshot.</p>
    #[serde(rename = "documentationVersion")]
    pub documentation_version: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Gets the documentation versions of an API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDocumentationVersionsRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to get the name of a <a>DomainName</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDomainNameRequest {
    /// <p>[Required] The name of the <a>DomainName</a> resource.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
}

/// <p>Request to describe a collection of <a>DomainName</a> resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDomainNamesRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>Request a new export of a <a>RestApi</a> for a particular <a>Stage</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetExportRequest {
    /// <p>The content-type of the export, for example <code>application/json</code>. Currently <code>application/json</code> and <code>application/yaml</code> are supported for <code>exportType</code> of<code>oas30</code> and <code>swagger</code>. This should be specified in the <code>Accept</code> header for direct API requests.</p>
    #[serde(rename = "accepts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepts: Option<String>,
    /// <p>[Required] The type of export. Acceptable values are 'oas30' for OpenAPI 3.0.x and 'swagger' for Swagger/OpenAPI 2.0.</p>
    #[serde(rename = "exportType")]
    pub export_type: String,
    /// <p>A key-value map of query string parameters that specify properties of the export, depending on the requested <code>exportType</code>. For <code>exportType</code> <code>oas30</code> and <code>swagger</code>, any combination of the following parameters are supported: <code>extensions='integrations'</code> or <code>extensions='apigateway'</code> will export the API with x-amazon-apigateway-integration extensions. <code>extensions='authorizers'</code> will export the API with x-amazon-apigateway-authorizer extensions. <code>postman</code> will export the API with Postman extensions, allowing for import to the Postman tool</p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The name of the <a>Stage</a> that will be exported.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
}

/// <p>Gets a <a>GatewayResponse</a> of a specified response type on the given <a>RestApi</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGatewayResponseRequest {
    /// <p>[Required] <p>The response type of the associated <a>GatewayResponse</a>. Valid values are <ul><li>ACCESS_DENIED</li><li>API_CONFIGURATION_ERROR</li><li>AUTHORIZER_FAILURE</li><li> AUTHORIZER_CONFIGURATION_ERROR</li><li>BAD_REQUEST_PARAMETERS</li><li>BAD_REQUEST_BODY</li><li>DEFAULT_4XX</li><li>DEFAULT_5XX</li><li>EXPIRED_TOKEN</li><li>INVALID_SIGNATURE</li><li>INTEGRATION_FAILURE</li><li>INTEGRATION_TIMEOUT</li><li>INVALID_API_KEY</li><li>MISSING_AUTHENTICATION_TOKEN</li><li> QUOTA_EXCEEDED</li><li>REQUEST_TOO_LARGE</li><li>RESOURCE_NOT_FOUND</li><li>THROTTLED</li><li>UNAUTHORIZED</li><li>UNSUPPORTED_MEDIA_TYPE</li></ul> </p></p>
    #[serde(rename = "responseType")]
    pub response_type: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Gets the <a>GatewayResponses</a> collection on the given <a>RestApi</a>. If an API developer has not added any definitions for gateway responses, the result will be the API Gateway-generated default <a>GatewayResponses</a> collection for the supported response types.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetGatewayResponsesRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500. The <a>GatewayResponses</a> collection does not support pagination and the limit does not apply here.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set. The <a>GatewayResponse</a> collection does not support pagination and the position does not apply here.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Represents a request to get the integration configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetIntegrationRequest {
    /// <p>[Required] Specifies a get integration request's HTTP method.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>[Required] Specifies a get integration request's resource identifier</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Represents a get integration response request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetIntegrationResponseRequest {
    /// <p>[Required] Specifies a get integration response request's HTTP method.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>[Required] Specifies a get integration response request's resource identifier.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] Specifies a get integration response request's status code.</p>
    #[serde(rename = "statusCode")]
    pub status_code: String,
}

/// <p>Request to describe an existing <a>Method</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMethodRequest {
    /// <p>[Required] Specifies the method request's HTTP method type.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>[Required] The <a>Resource</a> identifier for the <a>Method</a> resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to describe a <a>MethodResponse</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetMethodResponseRequest {
    /// <p>[Required] The HTTP verb of the <a>Method</a> resource.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>[Required] The <a>Resource</a> identifier for the <a>MethodResponse</a> resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The status code for the <a>MethodResponse</a> resource.</p>
    #[serde(rename = "statusCode")]
    pub status_code: String,
}

/// <p>Request to list information about a model in an existing <a>RestApi</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetModelRequest {
    /// <p>A query parameter of a Boolean value to resolve (<code>true</code>) all external model references and returns a flattened model schema or not (<code>false</code>) The default is <code>false</code>.</p>
    #[serde(rename = "flatten")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flatten: Option<bool>,
    /// <p>[Required] The name of the model as an identifier.</p>
    #[serde(rename = "modelName")]
    pub model_name: String,
    /// <p>[Required] The <a>RestApi</a> identifier under which the <a>Model</a> exists.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to generate a sample mapping template used to transform the payload.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetModelTemplateRequest {
    /// <p>[Required] The name of the model for which to generate a template.</p>
    #[serde(rename = "modelName")]
    pub model_name: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to list existing <a>Models</a> defined for a <a>RestApi</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetModelsRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Gets a <a>RequestValidator</a> of a given <a>RestApi</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRequestValidatorRequest {
    /// <p>[Required] The identifier of the <a>RequestValidator</a> to be retrieved.</p>
    #[serde(rename = "requestValidatorId")]
    pub request_validator_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Gets the <a>RequestValidators</a> collection of a given <a>RestApi</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRequestValidatorsRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to list information about a resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetResourceRequest {
    /// <p>A query parameter to retrieve the specified resources embedded in the returned <a>Resource</a> representation in the response. This <code>embed</code> parameter value is a list of comma-separated strings. Currently, the request supports only retrieval of the embedded <a>Method</a> resources this way. The query parameter value must be a single-valued list and contain the <code>"methods"</code> string. For example, <code>GET /restapis/{restapi_id}/resources/{resource_id}?embed=methods</code>.</p>
    #[serde(rename = "embed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<Vec<String>>,
    /// <p>[Required] The identifier for the <a>Resource</a> resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to list information about a collection of resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetResourcesRequest {
    /// <p>A query parameter used to retrieve the specified resources embedded in the returned <a>Resources</a> resource in the response. This <code>embed</code> parameter value is a list of comma-separated strings. Currently, the request supports only retrieval of the embedded <a>Method</a> resources this way. The query parameter value must be a single-valued list and contain the <code>"methods"</code> string. For example, <code>GET /restapis/{restapi_id}/resources?embed=methods</code>.</p>
    #[serde(rename = "embed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embed: Option<Vec<String>>,
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>The GET request to list an existing <a>RestApi</a> defined for your collection. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRestApiRequest {
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>The GET request to list existing <a>RestApis</a> defined for your collection.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetRestApisRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>Request a new generated client SDK for a <a>RestApi</a> and <a>Stage</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSdkRequest {
    /// <p>A string-to-string key-value map of query parameters <code>sdkType</code>-dependent properties of the SDK. For <code>sdkType</code> of <code>objectivec</code> or <code>swift</code>, a parameter named <code>classPrefix</code> is required. For <code>sdkType</code> of <code>android</code>, parameters named <code>groupId</code>, <code>artifactId</code>, <code>artifactVersion</code>, and <code>invokerPackage</code> are required. For <code>sdkType</code> of <code>java</code>, parameters named <code>serviceName</code> and <code>javaPackageName</code> are required. </p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The language for the generated SDK. Currently <code>java</code>, <code>javascript</code>, <code>android</code>, <code>objectivec</code> (for iOS), <code>swift</code> (for iOS), and <code>ruby</code> are supported.</p>
    #[serde(rename = "sdkType")]
    pub sdk_type: String,
    /// <p>[Required] The name of the <a>Stage</a> that the SDK will use.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
}

/// <p>Get an <a>SdkType</a> instance.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSdkTypeRequest {
    /// <p>[Required] The identifier of the queried <a>SdkType</a> instance.</p>
    #[serde(rename = "id")]
    pub id: String,
}

/// <p>Get the <a>SdkTypes</a> collection.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSdkTypesRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>Requests API Gateway to get information about a <a>Stage</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetStageRequest {
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The name of the <a>Stage</a> resource to get information about.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
}

/// <p>Requests API Gateway to get information about one or more <a>Stage</a> resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetStagesRequest {
    /// <p>The stages' deployment identifiers.</p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Gets the <a>Tags</a> collection for a given resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTagsRequest {
    /// <p>(Not currently supported) The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>(Not currently supported) The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>[Required] The ARN of a resource that can be tagged. The resource ARN must be URL-encoded. At present, <a>Stage</a> is the only taggable resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

/// <p>The GET request to get a usage plan key of a given key identifier.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUsagePlanKeyRequest {
    /// <p>[Required] The key Id of the to-be-retrieved <a>UsagePlanKey</a> resource representing a plan customer.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>[Required] The Id of the <a>UsagePlan</a> resource representing the usage plan containing the to-be-retrieved <a>UsagePlanKey</a> resource representing a plan customer.</p>
    #[serde(rename = "usagePlanId")]
    pub usage_plan_id: String,
}

/// <p>The GET request to get all the usage plan keys representing the API keys added to a specified usage plan.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUsagePlanKeysRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>A query parameter specifying the name of the to-be-returned usage plan keys.</p>
    #[serde(rename = "nameQuery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_query: Option<String>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>[Required] The Id of the <a>UsagePlan</a> resource representing the usage plan containing the to-be-retrieved <a>UsagePlanKey</a> resource representing a plan customer.</p>
    #[serde(rename = "usagePlanId")]
    pub usage_plan_id: String,
}

/// <p>The GET request to get a usage plan of a given plan identifier.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUsagePlanRequest {
    /// <p>[Required] The identifier of the <a>UsagePlan</a> resource to be retrieved.</p>
    #[serde(rename = "usagePlanId")]
    pub usage_plan_id: String,
}

/// <p>The GET request to get all the usage plans of the caller's account.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUsagePlansRequest {
    /// <p>The identifier of the API key associated with the usage plans.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>The GET request to get the usage data of a usage plan in a specified time interval.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetUsageRequest {
    /// <p>[Required] The ending date (e.g., 2016-12-31) of the usage data.</p>
    #[serde(rename = "endDate")]
    pub end_date: String,
    /// <p>The Id of the API key associated with the resultant usage data.</p>
    #[serde(rename = "keyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>[Required] The starting date (e.g., 2016-01-01) of the usage data.</p>
    #[serde(rename = "startDate")]
    pub start_date: String,
    /// <p>[Required] The Id of the usage plan associated with the usage data.</p>
    #[serde(rename = "usagePlanId")]
    pub usage_plan_id: String,
}

/// <p>Gets a specified VPC link under the caller's account in a region.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetVpcLinkRequest {
    /// <p>[Required] The identifier of the <a>VpcLink</a>. It is used in an <a>Integration</a> to reference this <a>VpcLink</a>.</p>
    #[serde(rename = "vpcLinkId")]
    pub vpc_link_id: String,
}

/// <p>Gets the <a>VpcLinks</a> collection under the caller's account in a selected region.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetVpcLinksRequest {
    /// <p>The maximum number of returned results per page. The default value is 25 and the maximum value is 500.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The current pagination position in the paged result set.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>The POST request to import API keys from an external source, such as a CSV-formatted file.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportApiKeysRequest {
    /// <p>The payload of the POST request to import API keys. For the payload format, see <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-key-file-format.html">API Key File Format</a>.</p>
    #[serde(rename = "body")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub body: bytes::Bytes,
    /// <p>A query parameter to indicate whether to rollback <a>ApiKey</a> importation (<code>true</code>) or not (<code>false</code>) when error is encountered.</p>
    #[serde(rename = "failOnWarnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_warnings: Option<bool>,
    /// <p>A query parameter to specify the input format to imported API keys. Currently, only the <code>csv</code> format is supported.</p>
    #[serde(rename = "format")]
    pub format: String,
}

/// <p>Import documentation parts from an external (e.g., OpenAPI) definition file. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportDocumentationPartsRequest {
    /// <p>[Required] Raw byte array representing the to-be-imported documentation parts. To import from an OpenAPI file, this is a JSON object.</p>
    #[serde(rename = "body")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub body: bytes::Bytes,
    /// <p>A query parameter to specify whether to rollback the documentation importation (<code>true</code>) or not (<code>false</code>) when a warning is encountered. The default value is <code>false</code>.</p>
    #[serde(rename = "failOnWarnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_warnings: Option<bool>,
    /// <p>A query parameter to indicate whether to overwrite (<code>OVERWRITE</code>) any existing <a>DocumentationParts</a> definition or to merge (<code>MERGE</code>) the new definition into the existing one. The default value is <code>MERGE</code>.</p>
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>A POST request to import an API to API Gateway using an input of an API definition file.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportRestApiRequest {
    /// <p>[Required] The POST request body containing external API definitions. Currently, only OpenAPI definition JSON/YAML files are supported. The maximum size of the API definition file is 2MB.</p>
    #[serde(rename = "body")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub body: bytes::Bytes,
    /// <p>A query parameter to indicate whether to rollback the API creation (<code>true</code>) or not (<code>false</code>) when a warning is encountered. The default value is <code>false</code>.</p>
    #[serde(rename = "failOnWarnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_warnings: Option<bool>,
    /// <p><p>A key-value map of context-specific query string parameters specifying the behavior of different API importing operations. The following shows operation-specific parameters and their supported values.</p> <p> To exclude <a>DocumentationParts</a> from the import, set <code>parameters</code> as <code>ignore=documentation</code>.</p> <p> To configure the endpoint type, set <code>parameters</code> as <code>endpointConfigurationTypes=EDGE</code>, <code>endpointConfigurationTypes=REGIONAL</code>, or <code>endpointConfigurationTypes=PRIVATE</code>. The default endpoint type is <code>EDGE</code>.</p> <p> To handle imported <code>basePath</code>, set <code>parameters</code> as <code>basePath=ignore</code>, <code>basePath=prepend</code> or <code>basePath=split</code>.</p> <p>For example, the AWS CLI command to exclude documentation from the imported API is:</p> <pre><code>aws apigateway import-rest-api --parameters ignore=documentation --body &#39;file:///path/to/imported-api-body.json&#39;</code></pre> <p>The AWS CLI command to set the regional endpoint on the imported API is:</p> <pre><code>aws apigateway import-rest-api --parameters endpointConfigurationTypes=REGIONAL --body &#39;file:///path/to/imported-api-body.json&#39;</code></pre></p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
}

/// <p><p>Represents an HTTP, HTTP<em>PROXY, AWS, AWS</em>PROXY, or Mock integration.</p> <div class="remarks">In the API Gateway console, the built-in Lambda integration is an AWS integration.</div> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-create-api.html">Creating an API</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Integration {
    /// <p>Specifies the integration's cache key parameters.</p>
    #[serde(rename = "cacheKeyParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_key_parameters: Option<Vec<String>>,
    /// <p>Specifies the integration's cache namespace.</p>
    #[serde(rename = "cacheNamespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_namespace: Option<String>,
    /// <p>The (<a href="https://docs.aws.amazon.com/apigateway/api-reference/resource/vpc-link/#id"><code>id</code></a>) of the <a>VpcLink</a> used for the integration when <code>connectionType=VPC_LINK</code> and undefined, otherwise.</p>
    #[serde(rename = "connectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The type of the network connection to the integration endpoint. The valid value is <code>INTERNET</code> for connections through the public routable internet or <code>VPC_LINK</code> for private connections between API Gateway and a network load balancer in a VPC. The default value is <code>INTERNET</code>.</p>
    #[serde(rename = "connectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// <p>Specifies how to handle request payload content type conversions. Supported values are <code>CONVERT_TO_BINARY</code> and <code>CONVERT_TO_TEXT</code>, with the following behaviors:</p> <ul> <li><p><code>CONVERT_TO_BINARY</code>: Converts a request payload from a Base64-encoded string to the corresponding binary blob.</p></li> <li><p><code>CONVERT_TO_TEXT</code>: Converts a request payload from a binary blob to a Base64-encoded string.</p></li> </ul> <p>If this property is not defined, the request payload will be passed through from the method request to integration request without modification, provided that the <code>passthroughBehaviors</code> is configured to support payload pass-through.</p>
    #[serde(rename = "contentHandling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling: Option<String>,
    /// <p>Specifies the credentials required for the integration, if any. For AWS integrations, three options are available. To specify an IAM Role for API Gateway to assume, use the role's Amazon Resource Name (ARN). To require that the caller's identity be passed through from the request, specify the string <code>arn:aws:iam::\*:user/\*</code>. To use resource-based permissions on supported AWS services, specify null.</p>
    #[serde(rename = "credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
    /// <p>Specifies the integration's HTTP method type.</p>
    #[serde(rename = "httpMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    /// <p><p>Specifies the integration&#39;s responses.</p> <div class="remarks"> <p/> <h4>Example: Get integration responses of a method</h4> <h5>Request</h5> <p/> <pre><code>GET /restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200 HTTP/1.1 Content-Type: application/json Host: apigateway.us-east-1.amazonaws.com X-Amz-Date: 20160607T191449Z Authorization: AWS4-HMAC-SHA256 Credential={access<em>key</em>ID}/20160607/us-east-1/apigateway/aws4<em>request, SignedHeaders=content-type;host;x-amz-date, Signature={sig4</em>hash} </code></pre> <h5>Response</h5> <p>The successful response returns <code>200 OK</code> status and a payload as follows:</p> <pre><code>{ &quot;_links&quot;: { &quot;curies&quot;: { &quot;href&quot;: &quot;https://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-integration-response-{rel}.html&quot;, &quot;name&quot;: &quot;integrationresponse&quot;, &quot;templated&quot;: true }, &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;integrationresponse:delete&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200&quot; }, &quot;integrationresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200&quot; } }, &quot;responseParameters&quot;: { &quot;method.response.header.Content-Type&quot;: &quot;&#39;application/xml&#39;&quot; }, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;$util.urlDecode(&quot;%3CkinesisStreams%3E#foreach($stream in $input.path(&#39;$.StreamNames&#39;))%3Cstream%3E%3Cname%3E$stream%3C/name%3E%3C/stream%3E#end%3C/kinesisStreams%3E&quot;)\n&quot; }, &quot;statusCode&quot;: &quot;200&quot; }</code></pre> <p/> </div> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-create-api.html">Creating an API</a> </div></p>
    #[serde(rename = "integrationResponses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_responses: Option<::std::collections::HashMap<String, IntegrationResponse>>,
    /// <div> <p> Specifies how the method request body of an unmapped content type will be passed through the integration request to the back end without transformation. A content type is unmapped if no mapping template is defined in the integration or the content type does not match any of the mapped content types, as specified in <code>requestTemplates</code>. The valid value is one of the following: </p> <ul> <li> <code>WHEN_NO_MATCH</code>: passes the method request body through the integration request to the back end without transformation when the method request content type does not match any content type associated with the mapping templates defined in the integration request. </li> <li> <code>WHEN_NO_TEMPLATES</code>: passes the method request body through the integration request to the back end without transformation when no mapping template is defined in the integration request. If a template is defined when this option is selected, the method request of an unmapped content-type will be rejected with an HTTP <code>415 Unsupported Media Type</code> response. </li> <li> <code>NEVER</code>: rejects the method request with an HTTP <code>415 Unsupported Media Type</code> response when either the method request content type does not match any content type associated with the mapping templates defined in the integration request or no mapping template is defined in the integration request. </li> </ul> </div>
    #[serde(rename = "passthroughBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    /// <p>A key-value map specifying request parameters that are passed from the method request to the back end. The key is an integration request parameter name and the associated value is a method request parameter value or static value that must be enclosed within single quotes and pre-encoded as required by the back end. The method request parameter value must match the pattern of <code>method.request.{location}.{name}</code>, where <code>location</code> is <code>querystring</code>, <code>path</code>, or <code>header</code> and <code>name</code> must be a valid and unique method request parameter name.</p>
    #[serde(rename = "requestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Represents a map of Velocity templates that are applied on the request payload based on the value of the Content-Type header sent by the client. The content type value is the key in this map, and the template (as a String) is the value.</p>
    #[serde(rename = "requestTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>Custom timeout between 50 and 29,000 milliseconds. The default value is 29,000 milliseconds or 29 seconds.</p>
    #[serde(rename = "timeoutInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i64>,
    /// <p>Specifies an API method integration type. The valid value is one of the following:</p> <ul> <li><code>AWS</code>: for integrating the API method request with an AWS service action, including the Lambda function-invoking action. With the Lambda function-invoking action, this is referred to as the Lambda custom integration. With any other AWS service action, this is known as AWS integration.</li> <li><code>AWS_PROXY</code>: for integrating the API method request with the Lambda function-invoking action with the client request passed through as-is. This integration is also referred to as the Lambda proxy integration.</li> <li><code>HTTP</code>: for integrating the API method request with an HTTP endpoint, including a private HTTP endpoint within a VPC. This integration is also referred to as the HTTP custom integration.</li> <li><code>HTTP_PROXY</code>: for integrating the API method request with an HTTP endpoint, including a private HTTP endpoint within a VPC, with the client request passed through as-is. This is also referred to as the HTTP proxy integration.</li> <li><code>MOCK</code>: for integrating the API method request with API Gateway as a "loop-back" endpoint without invoking any backend.</li> </ul> <p>For the HTTP and HTTP proxy integrations, each integration can specify a protocol (<code>http/https</code>), port and path. Standard 80 and 443 ports are supported as well as custom ports above 1024. An HTTP or HTTP proxy integration with a <code>connectionType</code> of <code>VPC_LINK</code> is referred to as a private integration and uses a <a>VpcLink</a> to connect API Gateway to a network load balancer of a VPC.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p><p>Specifies Uniform Resource Identifier (URI) of the integration endpoint.</p> <ul> <li><p> For <code>HTTP</code> or <code>HTTP<em>PROXY</code> integrations, the URI must be a fully formed, encoded HTTP(S) URL according to the &lt;a target=&quot;</em>blank&quot; href=&quot;https://en.wikipedia.org/wiki/Uniform<em>Resource</em>Identifier&quot;&gt;RFC-3986 specification</a>, for either standard integration, where <code>connectionType</code> is not <code>VPC<em>LINK</code>, or private integration, where <code>connectionType</code> is <code>VPC</em>LINK</code>. For a private HTTP integration, the URI is not used for routing. </p> </li> <li><p> For <code>AWS</code> or <code>AWS<em>PROXY</code> integrations, the URI is of the form <code>arn:aws:apigateway:{region}:{subdomain.service|service}:path|action/{service</em>api}</code>. Here, <code>{Region}</code> is the API Gateway region (e.g., <code>us-east-1</code>); <code>{service}</code> is the name of the integrated AWS service (e.g., <code>s3</code>); and <code>{subdomain}</code> is a designated subdomain supported by certain AWS service for fast host-name lookup. <code>action</code> can be used for an AWS service action-based API, using an <code>Action={name}&amp;{p1}={v1}&amp;p2={v2}...</code> query string. The ensuing <code>{service<em>api}</code> refers to a supported action <code>{name}</code> plus any required input parameters. Alternatively, <code>path</code> can be used for an AWS service path-based API. The ensuing <code>service</em>api</code> refers to the path to an AWS service resource, including the region of the integrated AWS service, if applicable. For example, for integration with the S3 API of <code><a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTObjectGET.html">GetObject</a></code>, the <code>uri</code> can be either <code>arn:aws:apigateway:us-west-2:s3:action/GetObject&amp;Bucket={bucket}&amp;Key={key}</code> or <code>arn:aws:apigateway:us-west-2:s3:path/{bucket}/{key}</code></p> </li></ul></p>
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// <p><p>Represents an integration response. The status code must map to an existing <a>MethodResponse</a>, and parameters and templates can be used to transform the back-end response.</p> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-create-api.html">Creating an API</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct IntegrationResponse {
    /// <p>Specifies how to handle response payload content type conversions. Supported values are <code>CONVERT_TO_BINARY</code> and <code>CONVERT_TO_TEXT</code>, with the following behaviors:</p> <ul> <li><p><code>CONVERT_TO_BINARY</code>: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p></li> <li><p><code>CONVERT_TO_TEXT</code>: Converts a response payload from a binary blob to a Base64-encoded string.</p></li> </ul> <p>If this property is not defined, the response payload will be passed through from the integration response to the method response without modification.</p>
    #[serde(rename = "contentHandling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling: Option<String>,
    /// <p>A key-value map specifying response parameters that are passed to the method response from the back end. The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of <code>method.response.header.{name}</code>, where <code>name</code> is a valid and unique header name. The mapped non-static value must match the pattern of <code>integration.response.header.{name}</code> or <code>integration.response.body.{JSON-expression}</code>, where <code>name</code> is a valid and unique response header name and <code>JSON-expression</code> is a valid JSON expression without the <code>$</code> prefix.</p>
    #[serde(rename = "responseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Specifies the templates used to transform the integration response body. Response templates are represented as a key/value map, with a content-type as the key and a template as the value.</p>
    #[serde(rename = "responseTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>Specifies the regular expression (regex) pattern used to choose an integration response based on the response from the back end. For example, if the success response returns nothing and the error response returns some string, you could use the <code>.+</code> regex to match error response. However, make sure that the error response does not contain any newline (<code>\n</code>) character in such cases. If the back end is an AWS Lambda function, the AWS Lambda function error header is matched. For all other HTTP and AWS back ends, the HTTP status code is matched.</p>
    #[serde(rename = "selectionPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_pattern: Option<String>,
    /// <p>Specifies the status code that is used to map the integration response to an existing <a>MethodResponse</a>.</p>
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

/// <p><p> Represents a client-facing interface by which the client calls the API to access back-end resources. A <b>Method</b> resource is integrated with an <a>Integration</a> resource. Both consist of a request and one or more responses. The method request takes the client input that is passed to the back end through the integration request. A method response returns the output from the back end to the client through an integration response. A method request is embodied in a <b>Method</b> resource, whereas an integration request is embodied in an <a>Integration</a> resource. On the other hand, a method response is represented by a <a>MethodResponse</a> resource, whereas an integration response is represented by an <a>IntegrationResponse</a> resource. </p> <div class="remarks"> <p/> <h4>Example: Retrive the GET method on a specified resource</h4> <h5>Request</h5> <p>The following example request retrieves the information about the GET method on an API resource (<code>3kzxbg5sa2</code>) of an API (<code>fugvjdxtri</code>). </p> <pre><code>GET /restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET HTTP/1.1 Content-Type: application/json Host: apigateway.us-east-1.amazonaws.com X-Amz-Date: 20160603T210259Z Authorization: AWS4-HMAC-SHA256 Credential={access<em>key</em>ID}/20160603/us-east-1/apigateway/aws4<em>request, SignedHeaders=content-type;host;x-amz-date, Signature={sig4</em>hash}</code></pre> <h5>Response</h5> <p>The successful response returns a <code>200 OK</code> status code and a payload similar to the following:</p> <pre><code>{ &quot;<em>links&quot;: { &quot;curies&quot;: [ { &quot;href&quot;: &quot;https://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-integration-{rel}.html&quot;, &quot;name&quot;: &quot;integration&quot;, &quot;templated&quot;: true }, { &quot;href&quot;: &quot;https://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-integration-response-{rel}.html&quot;, &quot;name&quot;: &quot;integrationresponse&quot;, &quot;templated&quot;: true }, { &quot;href&quot;: &quot;https://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-method-{rel}.html&quot;, &quot;name&quot;: &quot;method&quot;, &quot;templated&quot;: true }, { &quot;href&quot;: &quot;https://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-method-response-{rel}.html&quot;, &quot;name&quot;: &quot;methodresponse&quot;, &quot;templated&quot;: true } ], &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET&quot;, &quot;name&quot;: &quot;GET&quot;, &quot;title&quot;: &quot;GET&quot; }, &quot;integration:put&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration&quot; }, &quot;method:delete&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET&quot; }, &quot;method:integration&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration&quot; }, &quot;method:responses&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200&quot;, &quot;name&quot;: &quot;200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;method:update&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET&quot; }, &quot;methodresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/{status</em>code}&quot;, &quot;templated&quot;: true } }, &quot;apiKeyRequired&quot;: true, &quot;authorizationType&quot;: &quot;NONE&quot;, &quot;httpMethod&quot;: &quot;GET&quot;, &quot;<em>embedded&quot;: { &quot;method:integration&quot;: { &quot;</em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration&quot; }, &quot;integration:delete&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration&quot; }, &quot;integration:responses&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200&quot;, &quot;name&quot;: &quot;200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;integration:update&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration&quot; }, &quot;integrationresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/{status<em>code}&quot;, &quot;templated&quot;: true } }, &quot;cacheKeyParameters&quot;: [], &quot;cacheNamespace&quot;: &quot;3kzxbg5sa2&quot;, &quot;credentials&quot;: &quot;arn:aws:iam::123456789012:role/apigAwsProxyRole&quot;, &quot;httpMethod&quot;: &quot;POST&quot;, &quot;passthroughBehavior&quot;: &quot;WHEN</em>NO<em>MATCH&quot;, &quot;requestParameters&quot;: { &quot;integration.request.header.Content-Type&quot;: &quot;&#39;application/x-amz-json-1.1&#39;&quot; }, &quot;requestTemplates&quot;: { &quot;application/json&quot;: &quot;{\n}&quot; }, &quot;type&quot;: &quot;AWS&quot;, &quot;uri&quot;: &quot;arn:aws:apigateway:us-east-1:kinesis:action/ListStreams&quot;, &quot;</em>embedded&quot;: { &quot;integration:responses&quot;: { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200&quot;, &quot;name&quot;: &quot;200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;integrationresponse:delete&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200&quot; }, &quot;integrationresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200&quot; } }, &quot;responseParameters&quot;: { &quot;method.response.header.Content-Type&quot;: &quot;&#39;application/xml&#39;&quot; }, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;$util.urlDecode(&quot;%3CkinesisStreams%3E%23foreach(%24stream%20in%20%24input.path(%27%24.StreamNames%27))%3Cstream%3E%3Cname%3E%24stream%3C%2Fname%3E%3C%2Fstream%3E%23end%3C%2FkinesisStreams%3E&quot;)&quot; }, &quot;statusCode&quot;: &quot;200&quot; } } }, &quot;method:responses&quot;: { &quot;</em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200&quot;, &quot;name&quot;: &quot;200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;methodresponse:delete&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200&quot; }, &quot;methodresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200&quot; } }, &quot;responseModels&quot;: { &quot;application/json&quot;: &quot;Empty&quot; }, &quot;responseParameters&quot;: { &quot;method.response.header.Content-Type&quot;: false }, &quot;statusCode&quot;: &quot;200&quot; } } }</code></pre> <p>In the example above, the response template for the <code>200 OK</code> response maps the JSON output from the <code>ListStreams</code> action in the back end to an XML output. The mapping template is URL-encoded as <code>%3CkinesisStreams%3E%23foreach(%24stream%20in%20%24input.path(%27%24.StreamNames%27))%3Cstream%3E%3Cname%3E%24stream%3C%2Fname%3E%3C%2Fstream%3E%23end%3C%2FkinesisStreams%3E</code> and the output is decoded using the <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-mapping-template-reference.html#util-templat-reference">$util.urlDecode()</a> helper function.</p> </div> <div class="seeAlso"> <a>MethodResponse</a>, <a>Integration</a>, <a>IntegrationResponse</a>, <a>Resource</a>, <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-method-settings.html">Set up an API&#39;s method</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Method {
    /// <p>A boolean flag specifying whether a valid <a>ApiKey</a> is required to invoke this method.</p>
    #[serde(rename = "apiKeyRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    /// <p>A list of authorization scopes configured on the method. The scopes are used with a <code>COGNITO_USER_POOLS</code> authorizer to authorize the method invocation. The authorization works by matching the method scopes against the scopes parsed from the access token in the incoming request. The method invocation is authorized if any method scopes matches a claimed scope in the access token. Otherwise, the invocation is not authorized. When the method scope is configured, the client must provide an access token instead of an identity token for authorization purposes.</p>
    #[serde(rename = "authorizationScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,
    /// <p>The method's authorization type. Valid values are <code>NONE</code> for open access, <code>AWS_IAM</code> for using AWS IAM permissions, <code>CUSTOM</code> for using a custom authorizer, or <code>COGNITO_USER_POOLS</code> for using a Cognito user pool.</p>
    #[serde(rename = "authorizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    /// <p>The identifier of an <a>Authorizer</a> to use on this method. The <code>authorizationType</code> must be <code>CUSTOM</code>.</p>
    #[serde(rename = "authorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>The method's HTTP verb.</p>
    #[serde(rename = "httpMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    /// <p><p>Gets the method&#39;s integration responsible for passing the client-submitted request to the back end and performing necessary transformations to make the request compliant with the back end.</p> <div class="remarks"> <p/> <h4>Example: </h4> <h5>Request</h5> <p/> <pre><code>GET /restapis/uojnr9hd57/resources/0cjtch/methods/GET/integration HTTP/1.1 Content-Type: application/json Host: apigateway.us-east-1.amazonaws.com Content-Length: 117 X-Amz-Date: 20160613T213210Z Authorization: AWS4-HMAC-SHA256 Credential={access<em>key</em>ID}/20160613/us-east-1/apigateway/aws4<em>request, SignedHeaders=content-type;host;x-amz-date, Signature={sig4</em>hash}</code></pre> <h5>Response</h5> <p>The successful response returns a <code>200 OK</code> status code and a payload similar to the following:</p> <pre><code>{ &quot;<em>links&quot;: { &quot;curies&quot;: [ { &quot;href&quot;: &quot;https://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-integration-{rel}.html&quot;, &quot;name&quot;: &quot;integration&quot;, &quot;templated&quot;: true }, { &quot;href&quot;: &quot;https://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-integration-response-{rel}.html&quot;, &quot;name&quot;: &quot;integrationresponse&quot;, &quot;templated&quot;: true } ], &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/uojnr9hd57/resources/0cjtch/methods/GET/integration&quot; }, &quot;integration:delete&quot;: { &quot;href&quot;: &quot;/restapis/uojnr9hd57/resources/0cjtch/methods/GET/integration&quot; }, &quot;integration:responses&quot;: { &quot;href&quot;: &quot;/restapis/uojnr9hd57/resources/0cjtch/methods/GET/integration/responses/200&quot;, &quot;name&quot;: &quot;200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;integration:update&quot;: { &quot;href&quot;: &quot;/restapis/uojnr9hd57/resources/0cjtch/methods/GET/integration&quot; }, &quot;integrationresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/uojnr9hd57/resources/0cjtch/methods/GET/integration/responses/{status</em>code}&quot;, &quot;templated&quot;: true } }, &quot;cacheKeyParameters&quot;: [], &quot;cacheNamespace&quot;: &quot;0cjtch&quot;, &quot;credentials&quot;: &quot;arn:aws:iam::123456789012:role/apigAwsProxyRole&quot;, &quot;httpMethod&quot;: &quot;POST&quot;, &quot;passthroughBehavior&quot;: &quot;WHEN<em>NO</em>MATCH&quot;, &quot;requestTemplates&quot;: { &quot;application/json&quot;: &quot;{\n &quot;a&quot;: &quot;$input.params(&#39;operand1&#39;)&quot;,\n &quot;b&quot;: &quot;$input.params(&#39;operand2&#39;)&quot;, \n &quot;op&quot;: &quot;$input.params(&#39;operator&#39;)&quot; \n}&quot; }, &quot;type&quot;: &quot;AWS&quot;, &quot;uri&quot;: &quot;arn:aws:apigateway:us-west-2:lambda:path//2015-03-31/functions/arn:aws:lambda:us-west-2:123456789012:function:Calc/invocations&quot;, &quot;<em>embedded&quot;: { &quot;integration:responses&quot;: { &quot;</em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/uojnr9hd57/resources/0cjtch/methods/GET/integration/responses/200&quot;, &quot;name&quot;: &quot;200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;integrationresponse:delete&quot;: { &quot;href&quot;: &quot;/restapis/uojnr9hd57/resources/0cjtch/methods/GET/integration/responses/200&quot; }, &quot;integrationresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/uojnr9hd57/resources/0cjtch/methods/GET/integration/responses/200&quot; } }, &quot;responseParameters&quot;: { &quot;method.response.header.operator&quot;: &quot;integration.response.body.op&quot;, &quot;method.response.header.operand<em>2&quot;: &quot;integration.response.body.b&quot;, &quot;method.response.header.operand</em>1&quot;: &quot;integration.response.body.a&quot; }, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;#set($res = $input.path(&#39;$&#39;))\n{\n &quot;result&quot;: &quot;$res.a, $res.b, $res.op =&gt; $res.c&quot;,\n &quot;a&quot; : &quot;$res.a&quot;,\n &quot;b&quot; : &quot;$res.b&quot;,\n &quot;op&quot; : &quot;$res.op&quot;,\n &quot;c&quot; : &quot;$res.c&quot;\n}&quot; }, &quot;selectionPattern&quot;: &quot;&quot;, &quot;statusCode&quot;: &quot;200&quot; } } }</code></pre> <p/> </div> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/cli/latest/reference/apigateway/get-integration.html">AWS CLI</a> </div></p>
    #[serde(rename = "methodIntegration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_integration: Option<Integration>,
    /// <p><p>Gets a method response associated with a given HTTP status code. </p> <div class="remarks"> <p>The collection of method responses are encapsulated in a key-value map, where the key is a response&#39;s HTTP status code and the value is a <a>MethodResponse</a> resource that specifies the response returned to the caller from the back end through the integration response.</p> <h4>Example: Get a 200 OK response of a GET method</h4> <h5>Request</h5> <p/> <pre><code>GET /restapis/uojnr9hd57/resources/0cjtch/methods/GET/responses/200 HTTP/1.1 Content-Type: application/json Host: apigateway.us-east-1.amazonaws.com Content-Length: 117 X-Amz-Date: 20160613T215008Z Authorization: AWS4-HMAC-SHA256 Credential={access<em>key</em>ID}/20160613/us-east-1/apigateway/aws4<em>request, SignedHeaders=content-type;host;x-amz-date, Signature={sig4</em>hash}</code></pre> <h5>Response</h5> <p>The successful response returns a <code>200 OK</code> status code and a payload similar to the following:</p> <pre><code>{ &quot;<em>links&quot;: { &quot;curies&quot;: { &quot;href&quot;: &quot;https://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-method-response-{rel}.html&quot;, &quot;name&quot;: &quot;methodresponse&quot;, &quot;templated&quot;: true }, &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/uojnr9hd57/resources/0cjtch/methods/GET/responses/200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;methodresponse:delete&quot;: { &quot;href&quot;: &quot;/restapis/uojnr9hd57/resources/0cjtch/methods/GET/responses/200&quot; }, &quot;methodresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/uojnr9hd57/resources/0cjtch/methods/GET/responses/200&quot; } }, &quot;responseModels&quot;: { &quot;application/json&quot;: &quot;Empty&quot; }, &quot;responseParameters&quot;: { &quot;method.response.header.operator&quot;: false, &quot;method.response.header.operand</em>2&quot;: false, &quot;method.response.header.operand_1&quot;: false }, &quot;statusCode&quot;: &quot;200&quot; }</code></pre> <p/> </div> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/cli/latest/reference/apigateway/get-method-response.html">AWS CLI</a> </div></p>
    #[serde(rename = "methodResponses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_responses: Option<::std::collections::HashMap<String, MethodResponse>>,
    /// <p>A human-friendly operation identifier for the method. For example, you can assign the <code>operationName</code> of <code>ListPets</code> for the <code>GET /pets</code> method in <a href="https://petstore-demo-endpoint.execute-api.com/petstore/pets">PetStore</a> example.</p>
    #[serde(rename = "operationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    /// <p>A key-value map specifying data schemas, represented by <a>Model</a> resources, (as the mapped value) of the request payloads of given content types (as the mapping key).</p>
    #[serde(rename = "requestModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>A key-value map defining required or optional method request parameters that can be accepted by API Gateway. A key is a method request parameter name matching the pattern of <code>method.request.{location}.{name}</code>, where <code>location</code> is <code>querystring</code>, <code>path</code>, or <code>header</code> and <code>name</code> is a valid and unique parameter name. The value associated with the key is a Boolean flag indicating whether the parameter is required (<code>true</code>) or optional (<code>false</code>). The method request parameter names defined here are available in <a>Integration</a> to be mapped to integration request parameters or templates.</p>
    #[serde(rename = "requestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, bool>>,
    /// <p>The identifier of a <a>RequestValidator</a> for request validation.</p>
    #[serde(rename = "requestValidatorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_validator_id: Option<String>,
}

/// <p><p>Represents a method response of a given HTTP status code returned to the client. The method response is passed from the back end through the associated integration response that can be transformed using a mapping template. </p> <div class="remarks"> <p/> <h4>Example: A <b>MethodResponse</b> instance of an API</h4> <h5>Request</h5> <p>The example request retrieves a <b>MethodResponse</b> of the 200 status code.</p> <pre><code>GET /restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200 HTTP/1.1 Content-Type: application/json Host: apigateway.us-east-1.amazonaws.com X-Amz-Date: 20160603T222952Z Authorization: AWS4-HMAC-SHA256 Credential={access<em>key</em>ID}/20160603/us-east-1/apigateway/aws4<em>request, SignedHeaders=content-type;host;x-amz-date, Signature={sig4</em>hash}</code></pre> <h5>Response</h5> <p>The successful response returns <code>200 OK</code> status and a payload as follows:</p> <pre><code>{ &quot;_links&quot;: { &quot;curies&quot;: { &quot;href&quot;: &quot;https://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-method-response-{rel}.html&quot;, &quot;name&quot;: &quot;methodresponse&quot;, &quot;templated&quot;: true }, &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;methodresponse:delete&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200&quot; }, &quot;methodresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200&quot; } }, &quot;responseModels&quot;: { &quot;application/json&quot;: &quot;Empty&quot; }, &quot;responseParameters&quot;: { &quot;method.response.header.Content-Type&quot;: false }, &quot;statusCode&quot;: &quot;200&quot; }</code></pre> <p/> </div> <div class="seeAlso"> <a>Method</a>, <a>IntegrationResponse</a>, <a>Integration</a> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-create-api.html">Creating an API</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct MethodResponse {
    /// <p>Specifies the <a>Model</a> resources used for the response's content-type. Response models are represented as a key/value map, with a content-type as the key and a <a>Model</a> name as the value.</p>
    #[serde(rename = "responseModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>A key-value map specifying required or optional response parameters that API Gateway can send back to the caller. A key defines a method response header and the value specifies whether the associated method response header is required or not. The expression of the key must match the pattern <code>method.response.header.{name}</code>, where <code>name</code> is a valid and unique header name. API Gateway passes certain integration response data to the method response headers specified here according to the mapping you prescribe in the API's <a>IntegrationResponse</a>. The integration response data that can be mapped include an integration response header expressed in <code>integration.response.header.{name}</code>, a static value enclosed within a pair of single quotes (e.g., <code>'application/json'</code>), or a JSON expression from the back-end response payload in the form of <code>integration.response.body.{JSON-expression}</code>, where <code>JSON-expression</code> is a valid JSON expression without the <code>$</code> prefix.)</p>
    #[serde(rename = "responseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, bool>>,
    /// <p>The method response's status code.</p>
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

/// <p>Specifies the method setting properties.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct MethodSetting {
    /// <p>Specifies whether the cached responses are encrypted. The PATCH path for this setting is <code>/{method_setting_key}/caching/dataEncrypted</code>, and the value is a Boolean.</p>
    #[serde(rename = "cacheDataEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_data_encrypted: Option<bool>,
    /// <p>Specifies the time to live (TTL), in seconds, for cached responses. The higher the TTL, the longer the response will be cached. The PATCH path for this setting is <code>/{method_setting_key}/caching/ttlInSeconds</code>, and the value is an integer.</p>
    #[serde(rename = "cacheTtlInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_ttl_in_seconds: Option<i64>,
    /// <p>Specifies whether responses should be cached and returned for requests. A cache cluster must be enabled on the stage for responses to be cached. The PATCH path for this setting is <code>/{method_setting_key}/caching/enabled</code>, and the value is a Boolean.</p>
    #[serde(rename = "cachingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caching_enabled: Option<bool>,
    /// <p>Specifies whether data trace logging is enabled for this method, which affects the log entries pushed to Amazon CloudWatch Logs. The PATCH path for this setting is <code>/{method_setting_key}/logging/dataTrace</code>, and the value is a Boolean.</p>
    #[serde(rename = "dataTraceEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_trace_enabled: Option<bool>,
    /// <p>Specifies the logging level for this method, which affects the log entries pushed to Amazon CloudWatch Logs. The PATCH path for this setting is <code>/{method_setting_key}/logging/loglevel</code>, and the available levels are <code>OFF</code>, <code>ERROR</code>, and <code>INFO</code>.</p>
    #[serde(rename = "loggingLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_level: Option<String>,
    /// <p>Specifies whether Amazon CloudWatch metrics are enabled for this method. The PATCH path for this setting is <code>/{method_setting_key}/metrics/enabled</code>, and the value is a Boolean.</p>
    #[serde(rename = "metricsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_enabled: Option<bool>,
    /// <p>Specifies whether authorization is required for a cache invalidation request. The PATCH path for this setting is <code>/{method_setting_key}/caching/requireAuthorizationForCacheControl</code>, and the value is a Boolean.</p>
    #[serde(rename = "requireAuthorizationForCacheControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_authorization_for_cache_control: Option<bool>,
    /// <p>Specifies the throttling burst limit. The PATCH path for this setting is <code>/{method_setting_key}/throttling/burstLimit</code>, and the value is an integer.</p>
    #[serde(rename = "throttlingBurstLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_burst_limit: Option<i64>,
    /// <p>Specifies the throttling rate limit. The PATCH path for this setting is <code>/{method_setting_key}/throttling/rateLimit</code>, and the value is a double.</p>
    #[serde(rename = "throttlingRateLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttling_rate_limit: Option<f64>,
    /// <p>Specifies how to handle unauthorized requests for cache invalidation. The PATCH path for this setting is <code>/{method_setting_key}/caching/unauthorizedCacheControlHeaderStrategy</code>, and the available values are <code>FAIL_WITH_403</code>, <code>SUCCEED_WITH_RESPONSE_HEADER</code>, <code>SUCCEED_WITHOUT_RESPONSE_HEADER</code>.</p>
    #[serde(rename = "unauthorizedCacheControlHeaderStrategy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unauthorized_cache_control_header_strategy: Option<String>,
}

/// <p>Represents a summary of a <a>Method</a> resource, given a particular date and time.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct MethodSnapshot {
    /// <p>Specifies whether the method requires a valid <a>ApiKey</a>.</p>
    #[serde(rename = "apiKeyRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    /// <p>The method's authorization type. Valid values are <code>NONE</code> for open access, <code>AWS_IAM</code> for using AWS IAM permissions, <code>CUSTOM</code> for using a custom authorizer, or <code>COGNITO_USER_POOLS</code> for using a Cognito user pool.</p>
    #[serde(rename = "authorizationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
}

/// <p><p>Represents the data structure of a method&#39;s request or response payload.</p> <div class="remarks"> <p>A request model defines the data structure of the client-supplied request payload. A response model defines the data structure of the response payload returned by the back end. Although not required, models are useful for mapping payloads between the front end and back end.</p> <p>A model is used for generating an API&#39;s SDK, validating the input request body, and creating a skeletal mapping template.</p> </div> <div class="seeAlso"> <a>Method</a>, <a>MethodResponse</a>, <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/models-mappings.html">Models and Mappings</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Model {
    /// <p>The content-type for the model.</p>
    #[serde(rename = "contentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// <p>The description of the model.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier for the model resource.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the model. Must be an alphanumeric string.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The schema for the model. For <code>application/json</code> models, this should be <a href="https://tools.ietf.org/html/draft-zyp-json-schema-04" target="_blank">JSON schema draft 4</a> model. Do not include "\*/" characters in the description of any properties because such "\*/" characters may be interpreted as the closing marker for comments in some languages, such as Java or JavaScript, causing the installation of your API's SDK generated by API Gateway to fail.</p>
    #[serde(rename = "schema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

/// <p><p>Represents a collection of <a>Model</a> resources.</p> <div class="seeAlso"> <a>Method</a>, <a>MethodResponse</a>, <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/models-mappings.html">Models and Mappings</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Models {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Model>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>A single patch operation to apply to the specified resource. Please refer to http://tools.ietf.org/html/rfc6902#section-4 for an explanation of how each operation is used.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PatchOperation {
    /// <p>The <code>copy</code> update operation's source as identified by a <code>JSON-Pointer</code> value referencing the location within the targeted resource to copy the value from. For example, to promote a canary deployment, you copy the canary deployment ID to the affiliated deployment ID by calling a PATCH request on a <a>Stage</a> resource with <code>"op":"copy"</code>, <code>"from":"/canarySettings/deploymentId"</code> and <code>"path":"/deploymentId"</code>.</p>
    #[serde(rename = "from")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// <p> An update operation to be performed with this PATCH request. The valid value can be <code>add</code>, <code>remove</code>, <code>replace</code> or <code>copy</code>. Not all valid operations are supported for a given resource. Support of the operations depends on specific operational contexts. Attempts to apply an unsupported operation on a resource will return an error message.</p>
    #[serde(rename = "op")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    /// <p>The <code>op</code> operation's target, as identified by a <a href="https://tools.ietf.org/html/draft-ietf-appsawg-json-pointer-08">JSON Pointer</a> value that references a location within the targeted resource. For example, if the target resource has an updateable property of <code>{"name":"value"}</code>, the path for this property is <code>/name</code>. If the <code>name</code> property value is a JSON object (e.g., <code>{"name": {"child/name": "child-value"}}</code>), the path for the <code>child/name</code> property will be <code>/name/child~1name</code>. Any slash ("/") character appearing in path names must be escaped with "~1", as shown in the example above. Each <code>op</code> operation can have only one <code>path</code> associated with it.</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The new target value of the update operation. It is applicable for the <code>add</code> or <code>replace</code> operation. When using AWS CLI to update a property of a JSON value, enclose the JSON object with a pair of single quotes in a Linux shell, e.g., '{"a": ...}'. In a Windows shell, see <a href="https://docs.aws.amazon.com/cli/latest/userguide/cli-using-param.html#cli-using-param-json">Using JSON for Parameters</a>.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Creates a customization of a <a>GatewayResponse</a> of a specified response type and status code on the given <a>RestApi</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutGatewayResponseRequest {
    /// <p><p>Response parameters (paths, query strings and headers) of the <a>GatewayResponse</a> as a string-to-string map of key-value pairs.</p></p>
    #[serde(rename = "responseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p><p>Response templates of the <a>GatewayResponse</a> as a string-to-string map of key-value pairs.</p></p>
    #[serde(rename = "responseTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>[Required] <p>The response type of the associated <a>GatewayResponse</a>. Valid values are <ul><li>ACCESS_DENIED</li><li>API_CONFIGURATION_ERROR</li><li>AUTHORIZER_FAILURE</li><li> AUTHORIZER_CONFIGURATION_ERROR</li><li>BAD_REQUEST_PARAMETERS</li><li>BAD_REQUEST_BODY</li><li>DEFAULT_4XX</li><li>DEFAULT_5XX</li><li>EXPIRED_TOKEN</li><li>INVALID_SIGNATURE</li><li>INTEGRATION_FAILURE</li><li>INTEGRATION_TIMEOUT</li><li>INVALID_API_KEY</li><li>MISSING_AUTHENTICATION_TOKEN</li><li> QUOTA_EXCEEDED</li><li>REQUEST_TOO_LARGE</li><li>RESOURCE_NOT_FOUND</li><li>THROTTLED</li><li>UNAUTHORIZED</li><li>UNSUPPORTED_MEDIA_TYPE</li></ul> </p></p>
    #[serde(rename = "responseType")]
    pub response_type: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>The HTTP status code of the <a>GatewayResponse</a>.</p>
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

/// <p>Sets up a method's integration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutIntegrationRequest {
    /// <p>Specifies a put integration input's cache key parameters.</p>
    #[serde(rename = "cacheKeyParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_key_parameters: Option<Vec<String>>,
    /// <p>Specifies a put integration input's cache namespace.</p>
    #[serde(rename = "cacheNamespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_namespace: Option<String>,
    /// <p>The (<a href="https://docs.aws.amazon.com/apigateway/api-reference/resource/vpc-link/#id"><code>id</code></a>) of the <a>VpcLink</a> used for the integration when <code>connectionType=VPC_LINK</code> and undefined, otherwise.</p>
    #[serde(rename = "connectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// <p>The type of the network connection to the integration endpoint. The valid value is <code>INTERNET</code> for connections through the public routable internet or <code>VPC_LINK</code> for private connections between API Gateway and a network load balancer in a VPC. The default value is <code>INTERNET</code>.</p>
    #[serde(rename = "connectionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_type: Option<String>,
    /// <p>Specifies how to handle request payload content type conversions. Supported values are <code>CONVERT_TO_BINARY</code> and <code>CONVERT_TO_TEXT</code>, with the following behaviors:</p> <ul> <li><p><code>CONVERT_TO_BINARY</code>: Converts a request payload from a Base64-encoded string to the corresponding binary blob.</p></li> <li><p><code>CONVERT_TO_TEXT</code>: Converts a request payload from a binary blob to a Base64-encoded string.</p></li> </ul> <p>If this property is not defined, the request payload will be passed through from the method request to integration request without modification, provided that the <code>passthroughBehaviors</code> is configured to support payload pass-through.</p>
    #[serde(rename = "contentHandling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling: Option<String>,
    /// <p>Specifies whether credentials are required for a put integration.</p>
    #[serde(rename = "credentials")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
    /// <p>[Required] Specifies a put integration request's HTTP method.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>Specifies a put integration HTTP method. When the integration type is HTTP or AWS, this field is required.</p>
    #[serde(rename = "integrationHttpMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_http_method: Option<String>,
    /// <p><p>Specifies the pass-through behavior for incoming requests based on the Content-Type header in the request, and the available mapping templates specified as the <code>requestTemplates</code> property on the Integration resource. There are three valid values: <code>WHEN<em>NO</em>MATCH</code>, <code>WHEN<em>NO</em>TEMPLATES</code>, and <code>NEVER</code>. </p> <ul> <li><p><code>WHEN<em>NO</em>MATCH</code> passes the request body for unmapped content types through to the integration back end without transformation.</p></li> <li><p><code>NEVER</code> rejects unmapped content types with an HTTP 415 &#39;Unsupported Media Type&#39; response.</p></li> <li><p><code>WHEN<em>NO</em>TEMPLATES</code> allows pass-through when the integration has NO content types mapped to templates. However if there is at least one content type defined, unmapped content types will be rejected with the same 415 response.</p></li> </ul></p>
    #[serde(rename = "passthroughBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_behavior: Option<String>,
    /// <p>A key-value map specifying request parameters that are passed from the method request to the back end. The key is an integration request parameter name and the associated value is a method request parameter value or static value that must be enclosed within single quotes and pre-encoded as required by the back end. The method request parameter value must match the pattern of <code>method.request.{location}.{name}</code>, where <code>location</code> is <code>querystring</code>, <code>path</code>, or <code>header</code> and <code>name</code> must be a valid and unique method request parameter name.</p>
    #[serde(rename = "requestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Represents a map of Velocity templates that are applied on the request payload based on the value of the Content-Type header sent by the client. The content type value is the key in this map, and the template (as a String) is the value.</p>
    #[serde(rename = "requestTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>[Required] Specifies a put integration request's resource ID.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>Custom timeout between 50 and 29,000 milliseconds. The default value is 29,000 milliseconds or 29 seconds.</p>
    #[serde(rename = "timeoutInMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_in_millis: Option<i64>,
    /// <p>[Required] Specifies a put integration input's type.</p>
    #[serde(rename = "type")]
    pub type_: String,
    /// <p><p>Specifies Uniform Resource Identifier (URI) of the integration endpoint.</p> <ul> <li><p> For <code>HTTP</code> or <code>HTTP<em>PROXY</code> integrations, the URI must be a fully formed, encoded HTTP(S) URL according to the &lt;a target=&quot;</em>blank&quot; href=&quot;https://en.wikipedia.org/wiki/Uniform<em>Resource</em>Identifier&quot;&gt;RFC-3986 specification</a>, for either standard integration, where <code>connectionType</code> is not <code>VPC<em>LINK</code>, or private integration, where <code>connectionType</code> is <code>VPC</em>LINK</code>. For a private HTTP integration, the URI is not used for routing. </p> </li> <li><p> For <code>AWS</code> or <code>AWS<em>PROXY</code> integrations, the URI is of the form <code>arn:aws:apigateway:{region}:{subdomain.service|service}:path|action/{service</em>api}</code>. Here, <code>{Region}</code> is the API Gateway region (e.g., <code>us-east-1</code>); <code>{service}</code> is the name of the integrated AWS service (e.g., <code>s3</code>); and <code>{subdomain}</code> is a designated subdomain supported by certain AWS service for fast host-name lookup. <code>action</code> can be used for an AWS service action-based API, using an <code>Action={name}&amp;{p1}={v1}&amp;p2={v2}...</code> query string. The ensuing <code>{service<em>api}</code> refers to a supported action <code>{name}</code> plus any required input parameters. Alternatively, <code>path</code> can be used for an AWS service path-based API. The ensuing <code>service</em>api</code> refers to the path to an AWS service resource, including the region of the integrated AWS service, if applicable. For example, for integration with the S3 API of <code><a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTObjectGET.html">GetObject</a></code>, the <code>uri</code> can be either <code>arn:aws:apigateway:us-west-2:s3:action/GetObject&amp;Bucket={bucket}&amp;Key={key}</code> or <code>arn:aws:apigateway:us-west-2:s3:path/{bucket}/{key}</code></p> </li></ul></p>
    #[serde(rename = "uri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// <p>Represents a put integration response request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutIntegrationResponseRequest {
    /// <p>Specifies how to handle response payload content type conversions. Supported values are <code>CONVERT_TO_BINARY</code> and <code>CONVERT_TO_TEXT</code>, with the following behaviors:</p> <ul> <li><p><code>CONVERT_TO_BINARY</code>: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p></li> <li><p><code>CONVERT_TO_TEXT</code>: Converts a response payload from a binary blob to a Base64-encoded string.</p></li> </ul> <p>If this property is not defined, the response payload will be passed through from the integration response to the method response without modification.</p>
    #[serde(rename = "contentHandling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_handling: Option<String>,
    /// <p>[Required] Specifies a put integration response request's HTTP method.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>[Required] Specifies a put integration response request's resource identifier.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>A key-value map specifying response parameters that are passed to the method response from the back end. The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of <code>method.response.header.{name}</code>, where <code>name</code> is a valid and unique header name. The mapped non-static value must match the pattern of <code>integration.response.header.{name}</code> or <code>integration.response.body.{JSON-expression}</code>, where <code>name</code> must be a valid and unique response header name and <code>JSON-expression</code> a valid JSON expression without the <code>$</code> prefix.</p>
    #[serde(rename = "responseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>Specifies a put integration response's templates.</p>
    #[serde(rename = "responseTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_templates: Option<::std::collections::HashMap<String, String>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>Specifies the selection pattern of a put integration response.</p>
    #[serde(rename = "selectionPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_pattern: Option<String>,
    /// <p>[Required] Specifies the status code that is used to map the integration response to an existing <a>MethodResponse</a>.</p>
    #[serde(rename = "statusCode")]
    pub status_code: String,
}

/// <p>Request to add a method to an existing <a>Resource</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutMethodRequest {
    /// <p>Specifies whether the method required a valid <a>ApiKey</a>.</p>
    #[serde(rename = "apiKeyRequired")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_required: Option<bool>,
    /// <p>A list of authorization scopes configured on the method. The scopes are used with a <code>COGNITO_USER_POOLS</code> authorizer to authorize the method invocation. The authorization works by matching the method scopes against the scopes parsed from the access token in the incoming request. The method invocation is authorized if any method scopes matches a claimed scope in the access token. Otherwise, the invocation is not authorized. When the method scope is configured, the client must provide an access token instead of an identity token for authorization purposes.</p>
    #[serde(rename = "authorizationScopes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_scopes: Option<Vec<String>>,
    /// <p>[Required] The method's authorization type. Valid values are <code>NONE</code> for open access, <code>AWS_IAM</code> for using AWS IAM permissions, <code>CUSTOM</code> for using a custom authorizer, or <code>COGNITO_USER_POOLS</code> for using a Cognito user pool.</p>
    #[serde(rename = "authorizationType")]
    pub authorization_type: String,
    /// <p>Specifies the identifier of an <a>Authorizer</a> to use on this Method, if the type is CUSTOM or COGNITO_USER_POOLS. The authorizer identifier is generated by API Gateway when you created the authorizer.</p>
    #[serde(rename = "authorizerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_id: Option<String>,
    /// <p>[Required] Specifies the method request's HTTP method type.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>A human-friendly operation identifier for the method. For example, you can assign the <code>operationName</code> of <code>ListPets</code> for the <code>GET /pets</code> method in <a href="https://petstore-demo-endpoint.execute-api.com/petstore/pets">PetStore</a> example.</p>
    #[serde(rename = "operationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    /// <p>Specifies the <a>Model</a> resources used for the request's content type. Request models are represented as a key/value map, with a content type as the key and a <a>Model</a> name as the value.</p>
    #[serde(rename = "requestModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>A key-value map defining required or optional method request parameters that can be accepted by API Gateway. A key defines a method request parameter name matching the pattern of <code>method.request.{location}.{name}</code>, where <code>location</code> is <code>querystring</code>, <code>path</code>, or <code>header</code> and <code>name</code> is a valid and unique parameter name. The value associated with the key is a Boolean flag indicating whether the parameter is required (<code>true</code>) or optional (<code>false</code>). The method request parameter names defined here are available in <a>Integration</a> to be mapped to integration request parameters or body-mapping templates.</p>
    #[serde(rename = "requestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_parameters: Option<::std::collections::HashMap<String, bool>>,
    /// <p>The identifier of a <a>RequestValidator</a> for validating the method request.</p>
    #[serde(rename = "requestValidatorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_validator_id: Option<String>,
    /// <p>[Required] The <a>Resource</a> identifier for the new <a>Method</a> resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to add a <a>MethodResponse</a> to an existing <a>Method</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutMethodResponseRequest {
    /// <p>[Required] The HTTP verb of the <a>Method</a> resource.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>[Required] The <a>Resource</a> identifier for the <a>Method</a> resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>Specifies the <a>Model</a> resources used for the response's content type. Response models are represented as a key/value map, with a content type as the key and a <a>Model</a> name as the value.</p>
    #[serde(rename = "responseModels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_models: Option<::std::collections::HashMap<String, String>>,
    /// <p>A key-value map specifying required or optional response parameters that API Gateway can send back to the caller. A key defines a method response header name and the associated value is a Boolean flag indicating whether the method response parameter is required or not. The method response header names must match the pattern of <code>method.response.header.{name}</code>, where <code>name</code> is a valid and unique header name. The response parameter names defined here are available in the integration response to be mapped from an integration response header expressed in <code>integration.response.header.{name}</code>, a static value enclosed within a pair of single quotes (e.g., <code>'application/json'</code>), or a JSON expression from the back-end response payload in the form of <code>integration.response.body.{JSON-expression}</code>, where <code>JSON-expression</code> is a valid JSON expression without the <code>$</code> prefix.)</p>
    #[serde(rename = "responseParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_parameters: Option<::std::collections::HashMap<String, bool>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The method response's status code.</p>
    #[serde(rename = "statusCode")]
    pub status_code: String,
}

/// <p>A PUT request to update an existing API, with external API definitions specified as the request body.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutRestApiRequest {
    /// <p>[Required] The PUT request body containing external API definitions. Currently, only OpenAPI definition JSON/YAML files are supported. The maximum size of the API definition file is 2MB.</p>
    #[serde(rename = "body")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub body: bytes::Bytes,
    /// <p>A query parameter to indicate whether to rollback the API update (<code>true</code>) or not (<code>false</code>) when a warning is encountered. The default value is <code>false</code>.</p>
    #[serde(rename = "failOnWarnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_on_warnings: Option<bool>,
    /// <p>The <code>mode</code> query parameter to specify the update mode. Valid values are "merge" and "overwrite". By default, the update mode is "merge".</p>
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// <p>Custom header parameters as part of the request. For example, to exclude <a>DocumentationParts</a> from an imported API, set <code>ignore=documentation</code> as a <code>parameters</code> value, as in the AWS CLI command of <code>aws apigateway import-rest-api --parameters ignore=documentation --body 'file:///path/to/imported-api-body.json'</code>.</p>
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<::std::collections::HashMap<String, String>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Quotas configured for a usage plan.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuotaSettings {
    /// <p>The maximum number of requests that can be made in a given time period.</p>
    #[serde(rename = "limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// <p>The number of requests subtracted from the given limit in the initial time period.</p>
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// <p>The time period in which the limit applies. Valid values are "DAY", "WEEK" or "MONTH".</p>
    #[serde(rename = "period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}

/// <p><p>A set of validation rules for incoming <a>Method</a> requests.</p> <div class="remarks"> <p>In OpenAPI, a <a>RequestValidator</a> of an API is defined by the <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions.html#api-gateway-swagger-extensions-request-validators.requestValidator.html">x-amazon-apigateway-request-validators.requestValidator</a> object. It the referenced using the <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions.html#api-gateway-swagger-extensions-request-validator">x-amazon-apigateway-request-validator</a> property.</p> </div> <div class="seeAlso"><a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-method-request-validation.html">Enable Basic Request Validation in API Gateway</a></div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RequestValidator {
    /// <p>The identifier of this <a>RequestValidator</a>.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of this <a>RequestValidator</a></p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A Boolean flag to indicate whether to validate a request body according to the configured <a>Model</a> schema.</p>
    #[serde(rename = "validateRequestBody")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_request_body: Option<bool>,
    /// <p>A Boolean flag to indicate whether to validate request parameters (<code>true</code>) or not (<code>false</code>).</p>
    #[serde(rename = "validateRequestParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_request_parameters: Option<bool>,
}

/// <p><p>A collection of <a>RequestValidator</a> resources of a given <a>RestApi</a>.</p> <div class="remarks"> <p>In OpenAPI, the <a>RequestValidators</a> of an API is defined by the <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-swagger-extensions.html#api-gateway-swagger-extensions-request-validators.html">x-amazon-apigateway-request-validators</a> extension.</p> </div> <div class="seeAlso"><a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-method-request-validation.html">Enable Basic Request Validation in API Gateway</a></div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RequestValidators {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<RequestValidator>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p><p>Represents an API resource.</p> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-create-api.html">Create an API</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Resource {
    /// <p>The resource's identifier.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The parent resource's identifier.</p>
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// <p>The full path for this resource.</p>
    #[serde(rename = "path")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// <p>The last path segment for this resource.</p>
    #[serde(rename = "pathPart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_part: Option<String>,
    /// <p><p>Gets an API resource&#39;s method of a given HTTP verb.</p> <div class="remarks"> <p>The resource methods are a map of methods indexed by methods&#39; HTTP verbs enabled on the resource. This method map is included in the <code>200 OK</code> response of the <code>GET /restapis/{restapi<em>id}/resources/{resource</em>id}</code> or <code>GET /restapis/{restapi<em>id}/resources/{resource</em>id}?embed=methods</code> request.</p> <h4>Example: Get the GET method of an API resource</h4> <h5>Request</h5> <pre><code>GET /restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET HTTP/1.1 Content-Type: application/json Host: apigateway.us-east-1.amazonaws.com X-Amz-Date: 20170223T031827Z Authorization: AWS4-HMAC-SHA256 Credential={access<em>key</em>ID}/20170223/us-east-1/apigateway/aws4<em>request, SignedHeaders=content-type;host;x-amz-date, Signature={sig4</em>hash}</code></pre> <h5>Response</h5> <pre><code>{ &quot;<em>links&quot;: { &quot;curies&quot;: [ { &quot;href&quot;: &quot;https://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-integration-{rel}.html&quot;, &quot;name&quot;: &quot;integration&quot;, &quot;templated&quot;: true }, { &quot;href&quot;: &quot;https://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-integration-response-{rel}.html&quot;, &quot;name&quot;: &quot;integrationresponse&quot;, &quot;templated&quot;: true }, { &quot;href&quot;: &quot;https://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-method-{rel}.html&quot;, &quot;name&quot;: &quot;method&quot;, &quot;templated&quot;: true }, { &quot;href&quot;: &quot;https://docs.aws.amazon.com/apigateway/latest/developerguide/restapi-method-response-{rel}.html&quot;, &quot;name&quot;: &quot;methodresponse&quot;, &quot;templated&quot;: true } ], &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET&quot;, &quot;name&quot;: &quot;GET&quot;, &quot;title&quot;: &quot;GET&quot; }, &quot;integration:put&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration&quot; }, &quot;method:delete&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET&quot; }, &quot;method:integration&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration&quot; }, &quot;method:responses&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200&quot;, &quot;name&quot;: &quot;200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;method:update&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET&quot; }, &quot;methodresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/{status</em>code}&quot;, &quot;templated&quot;: true } }, &quot;apiKeyRequired&quot;: false, &quot;authorizationType&quot;: &quot;NONE&quot;, &quot;httpMethod&quot;: &quot;GET&quot;, &quot;<em>embedded&quot;: { &quot;method:integration&quot;: { &quot;</em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration&quot; }, &quot;integration:delete&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration&quot; }, &quot;integration:responses&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200&quot;, &quot;name&quot;: &quot;200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;integration:update&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration&quot; }, &quot;integrationresponse:put&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/{status<em>code}&quot;, &quot;templated&quot;: true } }, &quot;cacheKeyParameters&quot;: [], &quot;cacheNamespace&quot;: &quot;3kzxbg5sa2&quot;, &quot;credentials&quot;: &quot;arn:aws:iam::123456789012:role/apigAwsProxyRole&quot;, &quot;httpMethod&quot;: &quot;POST&quot;, &quot;passthroughBehavior&quot;: &quot;WHEN</em>NO<em>MATCH&quot;, &quot;requestParameters&quot;: { &quot;integration.request.header.Content-Type&quot;: &quot;&#39;application/x-amz-json-1.1&#39;&quot; }, &quot;requestTemplates&quot;: { &quot;application/json&quot;: &quot;{\n}&quot; }, &quot;type&quot;: &quot;AWS&quot;, &quot;uri&quot;: &quot;arn:aws:apigateway:us-east-1:kinesis:action/ListStreams&quot;, &quot;</em>embedded&quot;: { &quot;integration:responses&quot;: { &quot;<em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200&quot;, &quot;name&quot;: &quot;200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;integrationresponse:delete&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200&quot; }, &quot;integrationresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/integration/responses/200&quot; } }, &quot;responseParameters&quot;: { &quot;method.response.header.Content-Type&quot;: &quot;&#39;application/xml&#39;&quot; }, &quot;responseTemplates&quot;: { &quot;application/json&quot;: &quot;$util.urlDecode(&quot;%3CkinesisStreams%3E#foreach($stream in $input.path(&#39;$.StreamNames&#39;))%3Cstream%3E%3Cname%3E$stream%3C/name%3E%3C/stream%3E#end%3C/kinesisStreams%3E&quot;)\n&quot; }, &quot;statusCode&quot;: &quot;200&quot; } } }, &quot;method:responses&quot;: { &quot;</em>links&quot;: { &quot;self&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200&quot;, &quot;name&quot;: &quot;200&quot;, &quot;title&quot;: &quot;200&quot; }, &quot;methodresponse:delete&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200&quot; }, &quot;methodresponse:update&quot;: { &quot;href&quot;: &quot;/restapis/fugvjdxtri/resources/3kzxbg5sa2/methods/GET/responses/200&quot; } }, &quot;responseModels&quot;: { &quot;application/json&quot;: &quot;Empty&quot; }, &quot;responseParameters&quot;: { &quot;method.response.header.Content-Type&quot;: false }, &quot;statusCode&quot;: &quot;200&quot; } } }</code></pre> <p>If the <code>OPTIONS</code> is enabled on the resource, you can follow the example here to get that method. Just replace the <code>GET</code> of the last path segment in the request URL with <code>OPTIONS</code>.</p> </div> <div class="seeAlso"> </div></p>
    #[serde(rename = "resourceMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_methods: Option<::std::collections::HashMap<String, Method>>,
}

/// <p><p>Represents a collection of <a>Resource</a> resources.</p> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-create-api.html">Create an API</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Resources {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Resource>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p><p>Represents a REST API.</p> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-create-api.html">Create an API</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RestApi {
    /// <p>The source of the API key for metering requests according to a usage plan. Valid values are: <ul><li><code>HEADER</code> to read the API key from the <code>X-API-Key</code> header of a request. </li><li><code>AUTHORIZER</code> to read the API key from the <code>UsageIdentifierKey</code> from a custom authorizer.</li></ul> </p>
    #[serde(rename = "apiKeySource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_source: Option<String>,
    /// <p>The list of binary media types supported by the <a>RestApi</a>. By default, the <a>RestApi</a> supports only UTF-8-encoded text payloads.</p>
    #[serde(rename = "binaryMediaTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_media_types: Option<Vec<String>>,
    /// <p>The timestamp when the API was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The API's description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The endpoint configuration of this <a>RestApi</a> showing the endpoint types of the API. </p>
    #[serde(rename = "endpointConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<EndpointConfiguration>,
    /// <p>The API's identifier. This identifier is unique across all of your APIs in API Gateway.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>A nullable integer that is used to enable compression (with non-negative between 0 and 10485760 (10M) bytes, inclusive) or disable compression (with a null value) on an API. When compression is enabled, compression or decompression is not applied on the payload if the payload size is smaller than this value. Setting it to zero allows compression for any payload size.</p>
    #[serde(rename = "minimumCompressionSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_compression_size: Option<i64>,
    /// <p>The API's name.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A stringified JSON policy document that applies to this RestApi regardless of the caller and <a>Method</a> configuration.</p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>The collection of tags. Each tag element is associated with a given resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>A version identifier for the API.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The warning messages reported when <code>failonwarnings</code> is turned on during API import.</p>
    #[serde(rename = "warnings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

/// <p><p>Contains references to your APIs and links that guide you in how to interact with your collection. A collection offers a paginated view of your APIs.</p> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-create-api.html">Create an API</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RestApis {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<RestApi>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p>A configuration property of an SDK type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SdkConfigurationProperty {
    /// <p>The default value of an <a>SdkType</a> configuration property.</p>
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// <p>The description of an <a>SdkType</a> configuration property.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The user-friendly name of an <a>SdkType</a> configuration property.</p>
    #[serde(rename = "friendlyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// <p>The name of a an <a>SdkType</a> configuration property.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A boolean flag of an <a>SdkType</a> configuration property to indicate if the associated SDK configuration property is required (<code>true</code>) or not (<code>false</code>).</p>
    #[serde(rename = "required")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

/// <p>The binary blob response to <a>GetSdk</a>, which contains the generated SDK.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SdkResponse {
    /// <p>The binary blob response to <a>GetSdk</a>, which contains the generated SDK.</p>
    pub body: Option<bytes::Bytes>,
    /// <p>The content-disposition header value in the HTTP response.</p>
    pub content_disposition: Option<String>,
    /// <p>The content-type header value in the HTTP response.</p>
    pub content_type: Option<String>,
}

/// <p>A type of SDK that API Gateway can generate.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SdkType {
    /// <p>A list of configuration properties of an <a>SdkType</a>.</p>
    #[serde(rename = "configurationProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_properties: Option<Vec<SdkConfigurationProperty>>,
    /// <p>The description of an <a>SdkType</a>.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The user-friendly name of an <a>SdkType</a> instance.</p>
    #[serde(rename = "friendlyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// <p>The identifier of an <a>SdkType</a> instance.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>The collection of <a>SdkType</a> instances.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SdkTypes {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<SdkType>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p><p>Represents a unique identifier for a version of a deployed <a>RestApi</a> that is callable by users.</p> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-deploy-api.html">Deploy an API</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Stage {
    /// <p>Settings for logging access in this stage.</p>
    #[serde(rename = "accessLogSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_log_settings: Option<AccessLogSettings>,
    /// <p>Specifies whether a cache cluster is enabled for the stage.</p>
    #[serde(rename = "cacheClusterEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_enabled: Option<bool>,
    /// <p>The size of the cache cluster for the stage, if enabled.</p>
    #[serde(rename = "cacheClusterSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_size: Option<String>,
    /// <p>The status of the cache cluster for the stage, if enabled.</p>
    #[serde(rename = "cacheClusterStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_cluster_status: Option<String>,
    /// <p>Settings for the canary deployment in this stage.</p>
    #[serde(rename = "canarySettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canary_settings: Option<CanarySettings>,
    /// <p>The identifier of a client certificate for an API stage.</p>
    #[serde(rename = "clientCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    /// <p>The timestamp when the stage was created.</p>
    #[serde(rename = "createdDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    /// <p>The identifier of the <a>Deployment</a> that the stage points to.</p>
    #[serde(rename = "deploymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_id: Option<String>,
    /// <p>The stage's description.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The version of the associated API documentation.</p>
    #[serde(rename = "documentationVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documentation_version: Option<String>,
    /// <p>The timestamp when the stage last updated.</p>
    #[serde(rename = "lastUpdatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_date: Option<f64>,
    /// <p>A map that defines the method settings for a <a>Stage</a> resource. Keys (designated as <code>/{method_setting_key</code> below) are method paths defined as <code>{resource_path}/{http_method}</code> for an individual method override, or <code>/\*/\*</code> for overriding all methods in the stage. </p>
    #[serde(rename = "methodSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_settings: Option<::std::collections::HashMap<String, MethodSetting>>,
    /// <p>The name of the stage is the first path segment in the Uniform Resource Identifier (URI) of a call to API Gateway.</p>
    #[serde(rename = "stageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    /// <p>The collection of tags. Each tag element is associated with a given resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Specifies whether active tracing with X-ray is enabled for the <a>Stage</a>.</p>
    #[serde(rename = "tracingEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_enabled: Option<bool>,
    /// <p>A map that defines the stage variables for a <a>Stage</a> resource. Variable names can have alphanumeric and underscore characters, and the values must match <code>[A-Za-z0-9-._~:/?#&amp;=,]+</code>.</p>
    #[serde(rename = "variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, String>>,
    /// <p>The ARN of the WebAcl associated with the <a>Stage</a>.</p>
    #[serde(rename = "webAclArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_acl_arn: Option<String>,
}

/// <p>A reference to a unique stage identified in the format <code>{restApiId}/{stage}</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StageKey {
    /// <p>The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_api_id: Option<String>,
    /// <p>The stage name associated with the stage key.</p>
    #[serde(rename = "stageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
}

/// <p><p>A list of <a>Stage</a> resources that are associated with the <a>ApiKey</a> resource.</p> <div class="seeAlso"><a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/stages.html">Deploying API in Stages</a></div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Stages {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Vec<Stage>>,
}

/// <p>Adds or updates a tag on a given resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>[Required] The ARN of a resource that can be tagged. The resource ARN must be URL-encoded. At present, <a>Stage</a> is the only taggable resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>[Required] The key-value map of strings. The valid character set is [a-zA-Z+-=._:/]. The tag key can be up to 128 characters and must not start with <code>aws:</code>. The tag value can be up to 256 characters.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

/// <p>The collection of tags. Each tag element is associated with a given resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Tags {
    /// <p>The collection of tags. Each tag element is associated with a given resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p><p>Represents a mapping template used to transform a payload.</p> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/models-mappings.html#models-mappings-mappings">Mapping Templates</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Template {
    /// <p>The Apache <a href="https://velocity.apache.org/engine/devel/vtl-reference-guide.html" target="_blank">Velocity Template Language (VTL)</a> template content used for the template resource.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Make a request to simulate the execution of an <a>Authorizer</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TestInvokeAuthorizerRequest {
    /// <p>[Optional] A key-value map of additional context variables.</p>
    #[serde(rename = "additionalContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_context: Option<::std::collections::HashMap<String, String>>,
    /// <p>[Required] Specifies a test invoke authorizer request's <a>Authorizer</a> ID.</p>
    #[serde(rename = "authorizerId")]
    pub authorizer_id: String,
    /// <p>[Optional] The simulated request body of an incoming invocation request.</p>
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>[Required] A key-value map of headers to simulate an incoming invocation request. This is where the incoming authorization token, or identity source, should be specified.</p>
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<::std::collections::HashMap<String, String>>,
    /// <p>[Optional] The headers as a map from string to list of values to simulate an incoming invocation request. This is where the incoming authorization token, or identity source, may be specified.</p>
    #[serde(rename = "multiValueHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_value_headers: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>[Optional] The URI path, including query string, of the simulated invocation request. Use this to specify path parameters and query string parameters.</p>
    #[serde(rename = "pathWithQueryString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_with_query_string: Option<String>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>A key-value map of stage variables to simulate an invocation on a deployed <a>Stage</a>.</p>
    #[serde(rename = "stageVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Represents the response of the test invoke request for a custom <a>Authorizer</a></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TestInvokeAuthorizerResponse {
    #[serde(rename = "authorization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The <a href="https://openid.net/specs/openid-connect-core-1_0.html#StandardClaims">open identity claims</a>, with any supported custom attributes, returned from the Cognito Your User Pool configured for the API.</p>
    #[serde(rename = "claims")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims: Option<::std::collections::HashMap<String, String>>,
    /// <p>The HTTP status code that the client would have received. Value is 0 if the authorizer succeeded.</p>
    #[serde(rename = "clientStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_status: Option<i64>,
    /// <p>The execution latency of the test authorizer request.</p>
    #[serde(rename = "latency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<i64>,
    /// <p>The API Gateway execution log for the test authorizer request.</p>
    #[serde(rename = "log")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<String>,
    /// <p>The JSON policy document returned by the <a>Authorizer</a></p>
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>The principal identity returned by the <a>Authorizer</a></p>
    #[serde(rename = "principalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_id: Option<String>,
}

/// <p>Make a request to simulate the execution of a <a>Method</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TestInvokeMethodRequest {
    /// <p>The simulated request body of an incoming invocation request.</p>
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>A <a>ClientCertificate</a> identifier to use in the test invocation. API Gateway will use the certificate when making the HTTPS request to the defined back-end endpoint.</p>
    #[serde(rename = "clientCertificateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_certificate_id: Option<String>,
    /// <p>A key-value map of headers to simulate an incoming invocation request.</p>
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<::std::collections::HashMap<String, String>>,
    /// <p>[Required] Specifies a test invoke method request's HTTP method.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>The headers as a map from string to list of values to simulate an incoming invocation request.</p>
    #[serde(rename = "multiValueHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_value_headers: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The URI path, including query string, of the simulated invocation request. Use this to specify path parameters and query string parameters.</p>
    #[serde(rename = "pathWithQueryString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_with_query_string: Option<String>,
    /// <p>[Required] Specifies a test invoke method request's resource ID.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>A key-value map of stage variables to simulate an invocation on a deployed <a>Stage</a>.</p>
    #[serde(rename = "stageVariables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_variables: Option<::std::collections::HashMap<String, String>>,
}

/// <p><p>Represents the response of the test invoke request in the HTTP method.</p> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/how-to-test-method.html#how-to-test-method-console">Test API using the API Gateway console</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TestInvokeMethodResponse {
    /// <p>The body of the HTTP response.</p>
    #[serde(rename = "body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// <p>The headers of the HTTP response.</p>
    #[serde(rename = "headers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<::std::collections::HashMap<String, String>>,
    /// <p>The execution latency of the test invoke request.</p>
    #[serde(rename = "latency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency: Option<i64>,
    /// <p>The API Gateway execution log for the test invoke request.</p>
    #[serde(rename = "log")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<String>,
    /// <p>The headers of the HTTP response as a map from string to list of values.</p>
    #[serde(rename = "multiValueHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_value_headers: Option<::std::collections::HashMap<String, Vec<String>>>,
    /// <p>The HTTP status code.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

/// <p> The API request rate limits.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThrottleSettings {
    /// <p>The API request burst limit, the maximum rate limit over a time ranging from one to a few seconds, depending upon whether the underlying token bucket is at its full capacity.</p>
    #[serde(rename = "burstLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burst_limit: Option<i64>,
    /// <p>The API request steady-state rate limit.</p>
    #[serde(rename = "rateLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit: Option<f64>,
}

/// <p>Removes a tag from a given resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>[Required] The ARN of a resource that can be tagged. The resource ARN must be URL-encoded. At present, <a>Stage</a> is the only taggable resource.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>[Required] The Tag keys to delete.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>Requests API Gateway to change information about the current <a>Account</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAccountRequest {
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
}

/// <p>A request to change information about an <a>ApiKey</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateApiKeyRequest {
    /// <p>[Required] The identifier of the <a>ApiKey</a> resource to be updated.</p>
    #[serde(rename = "apiKey")]
    pub api_key: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
}

/// <p>Request to update an existing <a>Authorizer</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAuthorizerRequest {
    /// <p>[Required] The identifier of the <a>Authorizer</a> resource.</p>
    #[serde(rename = "authorizerId")]
    pub authorizer_id: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>A request to change information about the <a>BasePathMapping</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateBasePathMappingRequest {
    /// <p>[Required] The base path of the <a>BasePathMapping</a> resource to change.</p>
    #[serde(rename = "basePath")]
    pub base_path: String,
    /// <p>[Required] The domain name of the <a>BasePathMapping</a> resource to change.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
}

/// <p>A request to change information about an <a>ClientCertificate</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateClientCertificateRequest {
    /// <p>[Required] The identifier of the <a>ClientCertificate</a> resource to be updated.</p>
    #[serde(rename = "clientCertificateId")]
    pub client_certificate_id: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
}

/// <p>Requests API Gateway to change information about a <a>Deployment</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDeploymentRequest {
    /// <p>The replacement identifier for the <a>Deployment</a> resource to change information about.</p>
    #[serde(rename = "deploymentId")]
    pub deployment_id: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Updates an existing documentation part of a given API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDocumentationPartRequest {
    /// <p>[Required] The identifier of the to-be-updated documentation part.</p>
    #[serde(rename = "documentationPartId")]
    pub documentation_part_id: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Updates an existing documentation version of an API.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDocumentationVersionRequest {
    /// <p>[Required] The version identifier of the to-be-updated documentation version.</p>
    #[serde(rename = "documentationVersion")]
    pub documentation_version: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>..</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>A request to change information about the <a>DomainName</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDomainNameRequest {
    /// <p>[Required] The name of the <a>DomainName</a> resource to be changed.</p>
    #[serde(rename = "domainName")]
    pub domain_name: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
}

/// <p>Updates a <a>GatewayResponse</a> of a specified response type on the given <a>RestApi</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateGatewayResponseRequest {
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] <p>The response type of the associated <a>GatewayResponse</a>. Valid values are <ul><li>ACCESS_DENIED</li><li>API_CONFIGURATION_ERROR</li><li>AUTHORIZER_FAILURE</li><li> AUTHORIZER_CONFIGURATION_ERROR</li><li>BAD_REQUEST_PARAMETERS</li><li>BAD_REQUEST_BODY</li><li>DEFAULT_4XX</li><li>DEFAULT_5XX</li><li>EXPIRED_TOKEN</li><li>INVALID_SIGNATURE</li><li>INTEGRATION_FAILURE</li><li>INTEGRATION_TIMEOUT</li><li>INVALID_API_KEY</li><li>MISSING_AUTHENTICATION_TOKEN</li><li> QUOTA_EXCEEDED</li><li>REQUEST_TOO_LARGE</li><li>RESOURCE_NOT_FOUND</li><li>THROTTLED</li><li>UNAUTHORIZED</li><li>UNSUPPORTED_MEDIA_TYPE</li></ul> </p></p>
    #[serde(rename = "responseType")]
    pub response_type: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Represents an update integration request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateIntegrationRequest {
    /// <p>[Required] Represents an update integration request's HTTP method.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] Represents an update integration request's resource identifier.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Represents an update integration response request.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateIntegrationResponseRequest {
    /// <p>[Required] Specifies an update integration response request's HTTP method.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] Specifies an update integration response request's resource identifier.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] Specifies an update integration response request's status code.</p>
    #[serde(rename = "statusCode")]
    pub status_code: String,
}

/// <p>Request to update an existing <a>Method</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateMethodRequest {
    /// <p>[Required] The HTTP verb of the <a>Method</a> resource.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The <a>Resource</a> identifier for the <a>Method</a> resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>A request to update an existing <a>MethodResponse</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateMethodResponseRequest {
    /// <p>[Required] The HTTP verb of the <a>Method</a> resource.</p>
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The <a>Resource</a> identifier for the <a>MethodResponse</a> resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The status code for the <a>MethodResponse</a> resource.</p>
    #[serde(rename = "statusCode")]
    pub status_code: String,
}

/// <p>Request to update an existing model in an existing <a>RestApi</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateModelRequest {
    /// <p>[Required] The name of the model to update.</p>
    #[serde(rename = "modelName")]
    pub model_name: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Updates a <a>RequestValidator</a> of a given <a>RestApi</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateRequestValidatorRequest {
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The identifier of <a>RequestValidator</a> to be updated.</p>
    #[serde(rename = "requestValidatorId")]
    pub request_validator_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to change information about a <a>Resource</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateResourceRequest {
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The identifier of the <a>Resource</a> resource.</p>
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Request to update an existing <a>RestApi</a> resource in your collection.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateRestApiRequest {
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
}

/// <p>Requests API Gateway to change information about a <a>Stage</a> resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateStageRequest {
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The string identifier of the associated <a>RestApi</a>.</p>
    #[serde(rename = "restApiId")]
    pub rest_api_id: String,
    /// <p>[Required] The name of the <a>Stage</a> resource to change information about.</p>
    #[serde(rename = "stageName")]
    pub stage_name: String,
}

/// <p>The PATCH request to update a usage plan of a given plan Id.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUsagePlanRequest {
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The Id of the to-be-updated usage plan.</p>
    #[serde(rename = "usagePlanId")]
    pub usage_plan_id: String,
}

/// <p>The PATCH request to grant a temporary extension to the remaining quota of a usage plan associated with a specified API key.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateUsageRequest {
    /// <p>[Required] The identifier of the API key associated with the usage plan in which a temporary extension is granted to the remaining quota.</p>
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The Id of the usage plan associated with the usage data.</p>
    #[serde(rename = "usagePlanId")]
    pub usage_plan_id: String,
}

/// <p>Updates an existing <a>VpcLink</a> of a specified identifier.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateVpcLinkRequest {
    /// <p>A list of update operations to be applied to the specified resource and in the order specified in this list.</p>
    #[serde(rename = "patchOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_operations: Option<Vec<PatchOperation>>,
    /// <p>[Required] The identifier of the <a>VpcLink</a>. It is used in an <a>Integration</a> to reference this <a>VpcLink</a>.</p>
    #[serde(rename = "vpcLinkId")]
    pub vpc_link_id: String,
}

/// <p><p>Represents the usage data of a usage plan.</p> <div class="remarks"/> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-api-usage-plans.html">Create and Use Usage Plans</a>, <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-create-usage-plans-with-console.html#api-gateway-usage-plan-manage-usage">Manage Usage in a Usage Plan</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Usage {
    /// <p>The ending date of the usage data.</p>
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// <p>The usage data, as daily logs of used and remaining quotas, over the specified time interval indexed over the API keys in a usage plan. For example, <code>{..., "values" : { "{api_key}" : [ [0, 100], [10, 90], [100, 10]]}</code>, where <code>{api_key}</code> stands for an API key value and the daily log entry is of the format <code>[used quota, remaining quota]</code>.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<::std::collections::HashMap<String, Vec<Vec<i64>>>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>The starting date of the usage data.</p>
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// <p>The plan Id associated with this usage data.</p>
    #[serde(rename = "usagePlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_plan_id: Option<String>,
}

/// <p><p>Represents a usage plan than can specify who can assess associated API stages with specified request limits and quotas.</p> <div class="remarks"> <p>In a usage plan, you associate an API by specifying the API&#39;s Id and a stage name of the specified API. You add plan customers by adding API keys to the plan. </p> </div> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-api-usage-plans.html">Create and Use Usage Plans</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UsagePlan {
    /// <p>The associated API stages of a usage plan.</p>
    #[serde(rename = "apiStages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_stages: Option<Vec<ApiStage>>,
    /// <p>The description of a usage plan.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of a <a>UsagePlan</a> resource.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of a usage plan.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The AWS Markeplace product identifier to associate with the usage plan as a SaaS product on AWS Marketplace.</p>
    #[serde(rename = "productCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    /// <p>The maximum number of permitted requests per a given unit time interval.</p>
    #[serde(rename = "quota")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<QuotaSettings>,
    /// <p>The collection of tags. Each tag element is associated with a given resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The request throttle limits of a usage plan.</p>
    #[serde(rename = "throttle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throttle: Option<ThrottleSettings>,
}

/// <p><p>Represents a usage plan key to identify a plan customer.</p> <div class="remarks"> <p>To associate an API stage with a selected API key in a usage plan, you must create a UsagePlanKey resource to represent the selected <a>ApiKey</a>.</p> </div>&quot; <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-api-usage-plans.html">Create and Use Usage Plans</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UsagePlanKey {
    /// <p>The Id of a usage plan key.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of a usage plan key.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The type of a usage plan key. Currently, the valid key type is <code>API_KEY</code>.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The value of a usage plan key.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p><p>Represents the collection of usage plan keys added to usage plans for the associated API keys and, possibly, other types of keys.</p> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-api-usage-plans.html">Create and Use Usage Plans</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UsagePlanKeys {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<UsagePlanKey>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p><p>Represents a collection of usage plans for an AWS account.</p> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/api-gateway-api-usage-plans.html">Create and Use Usage Plans</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UsagePlans {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<UsagePlan>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// <p><p>A API Gateway VPC link for a <a>RestApi</a> to access resources in an Amazon Virtual Private Cloud (VPC).</p> <div class="remarks"> <p><p>To enable access to a resource in an Amazon Virtual Private Cloud through Amazon API Gateway, you, as an API developer, create a <a>VpcLink</a> resource targeted for one or more network load balancers of the VPC and then integrate an API method with a private integration that uses the <a>VpcLink</a>. The private integration has an integration type of <code>HTTP</code> or <code>HTTP<em>PROXY</code> and has a connection type of <code>VPC</em>LINK</code>. The integration uses the <code>connectionId</code> property to identify the <a>VpcLink</a> used.</p> </p> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct VpcLink {
    /// <p>The description of the VPC link.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the <a>VpcLink</a>. It is used in an <a>Integration</a> to reference this <a>VpcLink</a>.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name used to label and identify the VPC link.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The status of the VPC link. The valid values are <code>AVAILABLE</code>, <code>PENDING</code>, <code>DELETING</code>, or <code>FAILED</code>. Deploying an API will wait if the status is <code>PENDING</code> and will fail if the status is <code>DELETING</code>. </p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A description about the VPC link status.</p>
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>The collection of tags. Each tag element is associated with a given resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The ARNs of network load balancers of the VPC targeted by the VPC link. The network load balancers must be owned by the same AWS account of the API owner.</p>
    #[serde(rename = "targetArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arns: Option<Vec<String>>,
}

/// <p><p>The collection of VPC links under the caller&#39;s account in a region.</p> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/getting-started-with-private-integration.html">Getting Started with Private Integrations</a>, <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/set-up-private-integration.html">Set up Private Integrations</a> </div></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct VpcLinks {
    /// <p>The current page of elements from this collection.</p>
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<VpcLink>>,
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// Errors returned by CreateApiKey
#[derive(Debug, PartialEq)]
pub enum CreateApiKeyError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl CreateApiKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateApiKeyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateApiKeyError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateApiKeyError::Conflict(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateApiKeyError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateApiKeyError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateApiKeyError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateApiKeyError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateApiKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateApiKeyError {
    fn description(&self) -> &str {
        match *self {
            CreateApiKeyError::BadRequest(ref cause) => cause,
            CreateApiKeyError::Conflict(ref cause) => cause,
            CreateApiKeyError::LimitExceeded(ref cause) => cause,
            CreateApiKeyError::NotFound(ref cause) => cause,
            CreateApiKeyError::TooManyRequests(ref cause) => cause,
            CreateApiKeyError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAuthorizer
#[derive(Debug, PartialEq)]
pub enum CreateAuthorizerError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl CreateAuthorizerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAuthorizerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateAuthorizerError::BadRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateAuthorizerError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateAuthorizerError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateAuthorizerError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateAuthorizerError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateAuthorizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAuthorizerError {
    fn description(&self) -> &str {
        match *self {
            CreateAuthorizerError::BadRequest(ref cause) => cause,
            CreateAuthorizerError::LimitExceeded(ref cause) => cause,
            CreateAuthorizerError::NotFound(ref cause) => cause,
            CreateAuthorizerError::TooManyRequests(ref cause) => cause,
            CreateAuthorizerError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateBasePathMapping
#[derive(Debug, PartialEq)]
pub enum CreateBasePathMappingError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl CreateBasePathMappingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateBasePathMappingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateBasePathMappingError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateBasePathMappingError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateBasePathMappingError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateBasePathMappingError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateBasePathMappingError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateBasePathMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateBasePathMappingError {
    fn description(&self) -> &str {
        match *self {
            CreateBasePathMappingError::BadRequest(ref cause) => cause,
            CreateBasePathMappingError::Conflict(ref cause) => cause,
            CreateBasePathMappingError::NotFound(ref cause) => cause,
            CreateBasePathMappingError::TooManyRequests(ref cause) => cause,
            CreateBasePathMappingError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDeployment
#[derive(Debug, PartialEq)]
pub enum CreateDeploymentError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The requested service is not available. For details see the accompanying error message. Retry after the specified time period.</p>
    ServiceUnavailable(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl CreateDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDeploymentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateDeploymentError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateDeploymentError::Conflict(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateDeploymentError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateDeploymentError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateDeploymentError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateDeploymentError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateDeploymentError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateDeploymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDeploymentError {
    fn description(&self) -> &str {
        match *self {
            CreateDeploymentError::BadRequest(ref cause) => cause,
            CreateDeploymentError::Conflict(ref cause) => cause,
            CreateDeploymentError::LimitExceeded(ref cause) => cause,
            CreateDeploymentError::NotFound(ref cause) => cause,
            CreateDeploymentError::ServiceUnavailable(ref cause) => cause,
            CreateDeploymentError::TooManyRequests(ref cause) => cause,
            CreateDeploymentError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDocumentationPart
#[derive(Debug, PartialEq)]
pub enum CreateDocumentationPartError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl CreateDocumentationPartError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDocumentationPartError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateDocumentationPartError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateDocumentationPartError::Conflict(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateDocumentationPartError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateDocumentationPartError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateDocumentationPartError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateDocumentationPartError::Unauthorized(
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
impl fmt::Display for CreateDocumentationPartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDocumentationPartError {
    fn description(&self) -> &str {
        match *self {
            CreateDocumentationPartError::BadRequest(ref cause) => cause,
            CreateDocumentationPartError::Conflict(ref cause) => cause,
            CreateDocumentationPartError::LimitExceeded(ref cause) => cause,
            CreateDocumentationPartError::NotFound(ref cause) => cause,
            CreateDocumentationPartError::TooManyRequests(ref cause) => cause,
            CreateDocumentationPartError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDocumentationVersion
#[derive(Debug, PartialEq)]
pub enum CreateDocumentationVersionError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl CreateDocumentationVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateDocumentationVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateDocumentationVersionError::BadRequest(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateDocumentationVersionError::Conflict(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateDocumentationVersionError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateDocumentationVersionError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateDocumentationVersionError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateDocumentationVersionError::Unauthorized(
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
impl fmt::Display for CreateDocumentationVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDocumentationVersionError {
    fn description(&self) -> &str {
        match *self {
            CreateDocumentationVersionError::BadRequest(ref cause) => cause,
            CreateDocumentationVersionError::Conflict(ref cause) => cause,
            CreateDocumentationVersionError::LimitExceeded(ref cause) => cause,
            CreateDocumentationVersionError::NotFound(ref cause) => cause,
            CreateDocumentationVersionError::TooManyRequests(ref cause) => cause,
            CreateDocumentationVersionError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDomainName
#[derive(Debug, PartialEq)]
pub enum CreateDomainNameError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl CreateDomainNameError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDomainNameError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateDomainNameError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateDomainNameError::Conflict(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateDomainNameError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateDomainNameError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateDomainNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDomainNameError {
    fn description(&self) -> &str {
        match *self {
            CreateDomainNameError::BadRequest(ref cause) => cause,
            CreateDomainNameError::Conflict(ref cause) => cause,
            CreateDomainNameError::TooManyRequests(ref cause) => cause,
            CreateDomainNameError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateModel
#[derive(Debug, PartialEq)]
pub enum CreateModelError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl CreateModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateModelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateModelError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateModelError::Conflict(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateModelError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateModelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateModelError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateModelError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateModelError {
    fn description(&self) -> &str {
        match *self {
            CreateModelError::BadRequest(ref cause) => cause,
            CreateModelError::Conflict(ref cause) => cause,
            CreateModelError::LimitExceeded(ref cause) => cause,
            CreateModelError::NotFound(ref cause) => cause,
            CreateModelError::TooManyRequests(ref cause) => cause,
            CreateModelError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateRequestValidator
#[derive(Debug, PartialEq)]
pub enum CreateRequestValidatorError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl CreateRequestValidatorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRequestValidatorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateRequestValidatorError::BadRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateRequestValidatorError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateRequestValidatorError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateRequestValidatorError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateRequestValidatorError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateRequestValidatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRequestValidatorError {
    fn description(&self) -> &str {
        match *self {
            CreateRequestValidatorError::BadRequest(ref cause) => cause,
            CreateRequestValidatorError::LimitExceeded(ref cause) => cause,
            CreateRequestValidatorError::NotFound(ref cause) => cause,
            CreateRequestValidatorError::TooManyRequests(ref cause) => cause,
            CreateRequestValidatorError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateResource
#[derive(Debug, PartialEq)]
pub enum CreateResourceError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl CreateResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateResourceError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateResourceError::Conflict(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateResourceError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateResourceError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateResourceError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateResourceError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateResourceError {
    fn description(&self) -> &str {
        match *self {
            CreateResourceError::BadRequest(ref cause) => cause,
            CreateResourceError::Conflict(ref cause) => cause,
            CreateResourceError::LimitExceeded(ref cause) => cause,
            CreateResourceError::NotFound(ref cause) => cause,
            CreateResourceError::TooManyRequests(ref cause) => cause,
            CreateResourceError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateRestApi
#[derive(Debug, PartialEq)]
pub enum CreateRestApiError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl CreateRestApiError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateRestApiError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateRestApiError::BadRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateRestApiError::LimitExceeded(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateRestApiError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateRestApiError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateRestApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateRestApiError {
    fn description(&self) -> &str {
        match *self {
            CreateRestApiError::BadRequest(ref cause) => cause,
            CreateRestApiError::LimitExceeded(ref cause) => cause,
            CreateRestApiError::TooManyRequests(ref cause) => cause,
            CreateRestApiError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateStage
#[derive(Debug, PartialEq)]
pub enum CreateStageError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl CreateStageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateStageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateStageError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateStageError::Conflict(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateStageError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateStageError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateStageError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateStageError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateStageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateStageError {
    fn description(&self) -> &str {
        match *self {
            CreateStageError::BadRequest(ref cause) => cause,
            CreateStageError::Conflict(ref cause) => cause,
            CreateStageError::LimitExceeded(ref cause) => cause,
            CreateStageError::NotFound(ref cause) => cause,
            CreateStageError::TooManyRequests(ref cause) => cause,
            CreateStageError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateUsagePlan
#[derive(Debug, PartialEq)]
pub enum CreateUsagePlanError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl CreateUsagePlanError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUsagePlanError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateUsagePlanError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateUsagePlanError::Conflict(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateUsagePlanError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateUsagePlanError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateUsagePlanError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateUsagePlanError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateUsagePlanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateUsagePlanError {
    fn description(&self) -> &str {
        match *self {
            CreateUsagePlanError::BadRequest(ref cause) => cause,
            CreateUsagePlanError::Conflict(ref cause) => cause,
            CreateUsagePlanError::LimitExceeded(ref cause) => cause,
            CreateUsagePlanError::NotFound(ref cause) => cause,
            CreateUsagePlanError::TooManyRequests(ref cause) => cause,
            CreateUsagePlanError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateUsagePlanKey
#[derive(Debug, PartialEq)]
pub enum CreateUsagePlanKeyError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl CreateUsagePlanKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUsagePlanKeyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateUsagePlanKeyError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateUsagePlanKeyError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateUsagePlanKeyError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateUsagePlanKeyError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateUsagePlanKeyError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateUsagePlanKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateUsagePlanKeyError {
    fn description(&self) -> &str {
        match *self {
            CreateUsagePlanKeyError::BadRequest(ref cause) => cause,
            CreateUsagePlanKeyError::Conflict(ref cause) => cause,
            CreateUsagePlanKeyError::NotFound(ref cause) => cause,
            CreateUsagePlanKeyError::TooManyRequests(ref cause) => cause,
            CreateUsagePlanKeyError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateVpcLink
#[derive(Debug, PartialEq)]
pub enum CreateVpcLinkError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl CreateVpcLinkError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateVpcLinkError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateVpcLinkError::BadRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateVpcLinkError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(CreateVpcLinkError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateVpcLinkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateVpcLinkError {
    fn description(&self) -> &str {
        match *self {
            CreateVpcLinkError::BadRequest(ref cause) => cause,
            CreateVpcLinkError::TooManyRequests(ref cause) => cause,
            CreateVpcLinkError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApiKey
#[derive(Debug, PartialEq)]
pub enum DeleteApiKeyError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl DeleteApiKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteApiKeyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(DeleteApiKeyError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteApiKeyError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteApiKeyError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteApiKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApiKeyError {
    fn description(&self) -> &str {
        match *self {
            DeleteApiKeyError::NotFound(ref cause) => cause,
            DeleteApiKeyError::TooManyRequests(ref cause) => cause,
            DeleteApiKeyError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAuthorizer
#[derive(Debug, PartialEq)]
pub enum DeleteAuthorizerError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl DeleteAuthorizerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAuthorizerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteAuthorizerError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteAuthorizerError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteAuthorizerError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteAuthorizerError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteAuthorizerError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteAuthorizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAuthorizerError {
    fn description(&self) -> &str {
        match *self {
            DeleteAuthorizerError::BadRequest(ref cause) => cause,
            DeleteAuthorizerError::Conflict(ref cause) => cause,
            DeleteAuthorizerError::NotFound(ref cause) => cause,
            DeleteAuthorizerError::TooManyRequests(ref cause) => cause,
            DeleteAuthorizerError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteBasePathMapping
#[derive(Debug, PartialEq)]
pub enum DeleteBasePathMappingError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl DeleteBasePathMappingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteBasePathMappingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteBasePathMappingError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteBasePathMappingError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteBasePathMappingError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteBasePathMappingError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteBasePathMappingError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteBasePathMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteBasePathMappingError {
    fn description(&self) -> &str {
        match *self {
            DeleteBasePathMappingError::BadRequest(ref cause) => cause,
            DeleteBasePathMappingError::Conflict(ref cause) => cause,
            DeleteBasePathMappingError::NotFound(ref cause) => cause,
            DeleteBasePathMappingError::TooManyRequests(ref cause) => cause,
            DeleteBasePathMappingError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteClientCertificate
#[derive(Debug, PartialEq)]
pub enum DeleteClientCertificateError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl DeleteClientCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteClientCertificateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteClientCertificateError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteClientCertificateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteClientCertificateError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteClientCertificateError::Unauthorized(
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
impl fmt::Display for DeleteClientCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteClientCertificateError {
    fn description(&self) -> &str {
        match *self {
            DeleteClientCertificateError::BadRequest(ref cause) => cause,
            DeleteClientCertificateError::NotFound(ref cause) => cause,
            DeleteClientCertificateError::TooManyRequests(ref cause) => cause,
            DeleteClientCertificateError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDeployment
#[derive(Debug, PartialEq)]
pub enum DeleteDeploymentError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl DeleteDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDeploymentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteDeploymentError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteDeploymentError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteDeploymentError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteDeploymentError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteDeploymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDeploymentError {
    fn description(&self) -> &str {
        match *self {
            DeleteDeploymentError::BadRequest(ref cause) => cause,
            DeleteDeploymentError::NotFound(ref cause) => cause,
            DeleteDeploymentError::TooManyRequests(ref cause) => cause,
            DeleteDeploymentError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDocumentationPart
#[derive(Debug, PartialEq)]
pub enum DeleteDocumentationPartError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl DeleteDocumentationPartError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDocumentationPartError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteDocumentationPartError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteDocumentationPartError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteDocumentationPartError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteDocumentationPartError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteDocumentationPartError::Unauthorized(
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
impl fmt::Display for DeleteDocumentationPartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDocumentationPartError {
    fn description(&self) -> &str {
        match *self {
            DeleteDocumentationPartError::BadRequest(ref cause) => cause,
            DeleteDocumentationPartError::Conflict(ref cause) => cause,
            DeleteDocumentationPartError::NotFound(ref cause) => cause,
            DeleteDocumentationPartError::TooManyRequests(ref cause) => cause,
            DeleteDocumentationPartError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDocumentationVersion
#[derive(Debug, PartialEq)]
pub enum DeleteDocumentationVersionError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl DeleteDocumentationVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteDocumentationVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteDocumentationVersionError::BadRequest(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteDocumentationVersionError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteDocumentationVersionError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteDocumentationVersionError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteDocumentationVersionError::Unauthorized(
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
impl fmt::Display for DeleteDocumentationVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDocumentationVersionError {
    fn description(&self) -> &str {
        match *self {
            DeleteDocumentationVersionError::BadRequest(ref cause) => cause,
            DeleteDocumentationVersionError::Conflict(ref cause) => cause,
            DeleteDocumentationVersionError::NotFound(ref cause) => cause,
            DeleteDocumentationVersionError::TooManyRequests(ref cause) => cause,
            DeleteDocumentationVersionError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDomainName
#[derive(Debug, PartialEq)]
pub enum DeleteDomainNameError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl DeleteDomainNameError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDomainNameError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(DeleteDomainNameError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteDomainNameError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteDomainNameError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteDomainNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDomainNameError {
    fn description(&self) -> &str {
        match *self {
            DeleteDomainNameError::NotFound(ref cause) => cause,
            DeleteDomainNameError::TooManyRequests(ref cause) => cause,
            DeleteDomainNameError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteGatewayResponse
#[derive(Debug, PartialEq)]
pub enum DeleteGatewayResponseError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl DeleteGatewayResponseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteGatewayResponseError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteGatewayResponseError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteGatewayResponseError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteGatewayResponseError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteGatewayResponseError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteGatewayResponseError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteGatewayResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteGatewayResponseError {
    fn description(&self) -> &str {
        match *self {
            DeleteGatewayResponseError::BadRequest(ref cause) => cause,
            DeleteGatewayResponseError::Conflict(ref cause) => cause,
            DeleteGatewayResponseError::NotFound(ref cause) => cause,
            DeleteGatewayResponseError::TooManyRequests(ref cause) => cause,
            DeleteGatewayResponseError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteIntegration
#[derive(Debug, PartialEq)]
pub enum DeleteIntegrationError {
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl DeleteIntegrationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteIntegrationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(DeleteIntegrationError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteIntegrationError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteIntegrationError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteIntegrationError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteIntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteIntegrationError {
    fn description(&self) -> &str {
        match *self {
            DeleteIntegrationError::Conflict(ref cause) => cause,
            DeleteIntegrationError::NotFound(ref cause) => cause,
            DeleteIntegrationError::TooManyRequests(ref cause) => cause,
            DeleteIntegrationError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteIntegrationResponse
#[derive(Debug, PartialEq)]
pub enum DeleteIntegrationResponseError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl DeleteIntegrationResponseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteIntegrationResponseError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteIntegrationResponseError::BadRequest(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteIntegrationResponseError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteIntegrationResponseError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteIntegrationResponseError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteIntegrationResponseError::Unauthorized(
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
impl fmt::Display for DeleteIntegrationResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteIntegrationResponseError {
    fn description(&self) -> &str {
        match *self {
            DeleteIntegrationResponseError::BadRequest(ref cause) => cause,
            DeleteIntegrationResponseError::Conflict(ref cause) => cause,
            DeleteIntegrationResponseError::NotFound(ref cause) => cause,
            DeleteIntegrationResponseError::TooManyRequests(ref cause) => cause,
            DeleteIntegrationResponseError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteMethod
#[derive(Debug, PartialEq)]
pub enum DeleteMethodError {
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl DeleteMethodError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMethodError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConflictException" => {
                    return RusotoError::Service(DeleteMethodError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteMethodError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteMethodError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteMethodError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteMethodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteMethodError {
    fn description(&self) -> &str {
        match *self {
            DeleteMethodError::Conflict(ref cause) => cause,
            DeleteMethodError::NotFound(ref cause) => cause,
            DeleteMethodError::TooManyRequests(ref cause) => cause,
            DeleteMethodError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteMethodResponse
#[derive(Debug, PartialEq)]
pub enum DeleteMethodResponseError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl DeleteMethodResponseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMethodResponseError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteMethodResponseError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteMethodResponseError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteMethodResponseError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteMethodResponseError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteMethodResponseError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteMethodResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteMethodResponseError {
    fn description(&self) -> &str {
        match *self {
            DeleteMethodResponseError::BadRequest(ref cause) => cause,
            DeleteMethodResponseError::Conflict(ref cause) => cause,
            DeleteMethodResponseError::NotFound(ref cause) => cause,
            DeleteMethodResponseError::TooManyRequests(ref cause) => cause,
            DeleteMethodResponseError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteModel
#[derive(Debug, PartialEq)]
pub enum DeleteModelError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl DeleteModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteModelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteModelError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteModelError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteModelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteModelError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteModelError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteModelError {
    fn description(&self) -> &str {
        match *self {
            DeleteModelError::BadRequest(ref cause) => cause,
            DeleteModelError::Conflict(ref cause) => cause,
            DeleteModelError::NotFound(ref cause) => cause,
            DeleteModelError::TooManyRequests(ref cause) => cause,
            DeleteModelError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRequestValidator
#[derive(Debug, PartialEq)]
pub enum DeleteRequestValidatorError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl DeleteRequestValidatorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRequestValidatorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteRequestValidatorError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteRequestValidatorError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteRequestValidatorError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteRequestValidatorError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteRequestValidatorError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteRequestValidatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRequestValidatorError {
    fn description(&self) -> &str {
        match *self {
            DeleteRequestValidatorError::BadRequest(ref cause) => cause,
            DeleteRequestValidatorError::Conflict(ref cause) => cause,
            DeleteRequestValidatorError::NotFound(ref cause) => cause,
            DeleteRequestValidatorError::TooManyRequests(ref cause) => cause,
            DeleteRequestValidatorError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteResource
#[derive(Debug, PartialEq)]
pub enum DeleteResourceError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl DeleteResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteResourceError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteResourceError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteResourceError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteResourceError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteResourceError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteResourceError {
    fn description(&self) -> &str {
        match *self {
            DeleteResourceError::BadRequest(ref cause) => cause,
            DeleteResourceError::Conflict(ref cause) => cause,
            DeleteResourceError::NotFound(ref cause) => cause,
            DeleteResourceError::TooManyRequests(ref cause) => cause,
            DeleteResourceError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteRestApi
#[derive(Debug, PartialEq)]
pub enum DeleteRestApiError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl DeleteRestApiError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteRestApiError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteRestApiError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteRestApiError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteRestApiError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteRestApiError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteRestApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteRestApiError {
    fn description(&self) -> &str {
        match *self {
            DeleteRestApiError::BadRequest(ref cause) => cause,
            DeleteRestApiError::NotFound(ref cause) => cause,
            DeleteRestApiError::TooManyRequests(ref cause) => cause,
            DeleteRestApiError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteStage
#[derive(Debug, PartialEq)]
pub enum DeleteStageError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl DeleteStageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteStageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteStageError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteStageError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteStageError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteStageError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteStageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteStageError {
    fn description(&self) -> &str {
        match *self {
            DeleteStageError::BadRequest(ref cause) => cause,
            DeleteStageError::NotFound(ref cause) => cause,
            DeleteStageError::TooManyRequests(ref cause) => cause,
            DeleteStageError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteUsagePlan
#[derive(Debug, PartialEq)]
pub enum DeleteUsagePlanError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl DeleteUsagePlanError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUsagePlanError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteUsagePlanError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteUsagePlanError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteUsagePlanError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteUsagePlanError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteUsagePlanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUsagePlanError {
    fn description(&self) -> &str {
        match *self {
            DeleteUsagePlanError::BadRequest(ref cause) => cause,
            DeleteUsagePlanError::NotFound(ref cause) => cause,
            DeleteUsagePlanError::TooManyRequests(ref cause) => cause,
            DeleteUsagePlanError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteUsagePlanKey
#[derive(Debug, PartialEq)]
pub enum DeleteUsagePlanKeyError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl DeleteUsagePlanKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUsagePlanKeyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteUsagePlanKeyError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteUsagePlanKeyError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteUsagePlanKeyError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteUsagePlanKeyError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteUsagePlanKeyError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteUsagePlanKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteUsagePlanKeyError {
    fn description(&self) -> &str {
        match *self {
            DeleteUsagePlanKeyError::BadRequest(ref cause) => cause,
            DeleteUsagePlanKeyError::Conflict(ref cause) => cause,
            DeleteUsagePlanKeyError::NotFound(ref cause) => cause,
            DeleteUsagePlanKeyError::TooManyRequests(ref cause) => cause,
            DeleteUsagePlanKeyError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteVpcLink
#[derive(Debug, PartialEq)]
pub enum DeleteVpcLinkError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl DeleteVpcLinkError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVpcLinkError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteVpcLinkError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteVpcLinkError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteVpcLinkError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(DeleteVpcLinkError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteVpcLinkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteVpcLinkError {
    fn description(&self) -> &str {
        match *self {
            DeleteVpcLinkError::BadRequest(ref cause) => cause,
            DeleteVpcLinkError::NotFound(ref cause) => cause,
            DeleteVpcLinkError::TooManyRequests(ref cause) => cause,
            DeleteVpcLinkError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by FlushStageAuthorizersCache
#[derive(Debug, PartialEq)]
pub enum FlushStageAuthorizersCacheError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl FlushStageAuthorizersCacheError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<FlushStageAuthorizersCacheError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(FlushStageAuthorizersCacheError::BadRequest(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(FlushStageAuthorizersCacheError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(FlushStageAuthorizersCacheError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(FlushStageAuthorizersCacheError::Unauthorized(
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
impl fmt::Display for FlushStageAuthorizersCacheError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for FlushStageAuthorizersCacheError {
    fn description(&self) -> &str {
        match *self {
            FlushStageAuthorizersCacheError::BadRequest(ref cause) => cause,
            FlushStageAuthorizersCacheError::NotFound(ref cause) => cause,
            FlushStageAuthorizersCacheError::TooManyRequests(ref cause) => cause,
            FlushStageAuthorizersCacheError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by FlushStageCache
#[derive(Debug, PartialEq)]
pub enum FlushStageCacheError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl FlushStageCacheError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<FlushStageCacheError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(FlushStageCacheError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(FlushStageCacheError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(FlushStageCacheError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(FlushStageCacheError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for FlushStageCacheError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for FlushStageCacheError {
    fn description(&self) -> &str {
        match *self {
            FlushStageCacheError::BadRequest(ref cause) => cause,
            FlushStageCacheError::NotFound(ref cause) => cause,
            FlushStageCacheError::TooManyRequests(ref cause) => cause,
            FlushStageCacheError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GenerateClientCertificate
#[derive(Debug, PartialEq)]
pub enum GenerateClientCertificateError {
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GenerateClientCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GenerateClientCertificateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "LimitExceededException" => {
                    return RusotoError::Service(GenerateClientCertificateError::LimitExceeded(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GenerateClientCertificateError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GenerateClientCertificateError::Unauthorized(
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
impl fmt::Display for GenerateClientCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GenerateClientCertificateError {
    fn description(&self) -> &str {
        match *self {
            GenerateClientCertificateError::LimitExceeded(ref cause) => cause,
            GenerateClientCertificateError::TooManyRequests(ref cause) => cause,
            GenerateClientCertificateError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAccount
#[derive(Debug, PartialEq)]
pub enum GetAccountError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetAccountError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetAccountError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetAccountError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAccountError {
    fn description(&self) -> &str {
        match *self {
            GetAccountError::NotFound(ref cause) => cause,
            GetAccountError::TooManyRequests(ref cause) => cause,
            GetAccountError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApiKey
#[derive(Debug, PartialEq)]
pub enum GetApiKeyError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetApiKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetApiKeyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetApiKeyError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetApiKeyError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetApiKeyError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetApiKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApiKeyError {
    fn description(&self) -> &str {
        match *self {
            GetApiKeyError::NotFound(ref cause) => cause,
            GetApiKeyError::TooManyRequests(ref cause) => cause,
            GetApiKeyError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetApiKeys
#[derive(Debug, PartialEq)]
pub enum GetApiKeysError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetApiKeysError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetApiKeysError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetApiKeysError::BadRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetApiKeysError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetApiKeysError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetApiKeysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetApiKeysError {
    fn description(&self) -> &str {
        match *self {
            GetApiKeysError::BadRequest(ref cause) => cause,
            GetApiKeysError::TooManyRequests(ref cause) => cause,
            GetApiKeysError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAuthorizer
#[derive(Debug, PartialEq)]
pub enum GetAuthorizerError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetAuthorizerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAuthorizerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetAuthorizerError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetAuthorizerError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetAuthorizerError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAuthorizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAuthorizerError {
    fn description(&self) -> &str {
        match *self {
            GetAuthorizerError::NotFound(ref cause) => cause,
            GetAuthorizerError::TooManyRequests(ref cause) => cause,
            GetAuthorizerError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAuthorizers
#[derive(Debug, PartialEq)]
pub enum GetAuthorizersError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetAuthorizersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAuthorizersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetAuthorizersError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetAuthorizersError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetAuthorizersError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetAuthorizersError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAuthorizersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAuthorizersError {
    fn description(&self) -> &str {
        match *self {
            GetAuthorizersError::BadRequest(ref cause) => cause,
            GetAuthorizersError::NotFound(ref cause) => cause,
            GetAuthorizersError::TooManyRequests(ref cause) => cause,
            GetAuthorizersError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBasePathMapping
#[derive(Debug, PartialEq)]
pub enum GetBasePathMappingError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetBasePathMappingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBasePathMappingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetBasePathMappingError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetBasePathMappingError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetBasePathMappingError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetBasePathMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBasePathMappingError {
    fn description(&self) -> &str {
        match *self {
            GetBasePathMappingError::NotFound(ref cause) => cause,
            GetBasePathMappingError::TooManyRequests(ref cause) => cause,
            GetBasePathMappingError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBasePathMappings
#[derive(Debug, PartialEq)]
pub enum GetBasePathMappingsError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetBasePathMappingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBasePathMappingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetBasePathMappingsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetBasePathMappingsError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetBasePathMappingsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetBasePathMappingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBasePathMappingsError {
    fn description(&self) -> &str {
        match *self {
            GetBasePathMappingsError::NotFound(ref cause) => cause,
            GetBasePathMappingsError::TooManyRequests(ref cause) => cause,
            GetBasePathMappingsError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetClientCertificate
#[derive(Debug, PartialEq)]
pub enum GetClientCertificateError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetClientCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetClientCertificateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetClientCertificateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetClientCertificateError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetClientCertificateError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetClientCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetClientCertificateError {
    fn description(&self) -> &str {
        match *self {
            GetClientCertificateError::NotFound(ref cause) => cause,
            GetClientCertificateError::TooManyRequests(ref cause) => cause,
            GetClientCertificateError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetClientCertificates
#[derive(Debug, PartialEq)]
pub enum GetClientCertificatesError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetClientCertificatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetClientCertificatesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetClientCertificatesError::BadRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetClientCertificatesError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetClientCertificatesError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetClientCertificatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetClientCertificatesError {
    fn description(&self) -> &str {
        match *self {
            GetClientCertificatesError::BadRequest(ref cause) => cause,
            GetClientCertificatesError::TooManyRequests(ref cause) => cause,
            GetClientCertificatesError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDeployment
#[derive(Debug, PartialEq)]
pub enum GetDeploymentError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The requested service is not available. For details see the accompanying error message. Retry after the specified time period.</p>
    ServiceUnavailable(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDeploymentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetDeploymentError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetDeploymentError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetDeploymentError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetDeploymentError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDeploymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDeploymentError {
    fn description(&self) -> &str {
        match *self {
            GetDeploymentError::NotFound(ref cause) => cause,
            GetDeploymentError::ServiceUnavailable(ref cause) => cause,
            GetDeploymentError::TooManyRequests(ref cause) => cause,
            GetDeploymentError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDeployments
#[derive(Debug, PartialEq)]
pub enum GetDeploymentsError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested service is not available. For details see the accompanying error message. Retry after the specified time period.</p>
    ServiceUnavailable(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetDeploymentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDeploymentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDeploymentsError::BadRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetDeploymentsError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetDeploymentsError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetDeploymentsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDeploymentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDeploymentsError {
    fn description(&self) -> &str {
        match *self {
            GetDeploymentsError::BadRequest(ref cause) => cause,
            GetDeploymentsError::ServiceUnavailable(ref cause) => cause,
            GetDeploymentsError::TooManyRequests(ref cause) => cause,
            GetDeploymentsError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDocumentationPart
#[derive(Debug, PartialEq)]
pub enum GetDocumentationPartError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetDocumentationPartError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDocumentationPartError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetDocumentationPartError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetDocumentationPartError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetDocumentationPartError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDocumentationPartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDocumentationPartError {
    fn description(&self) -> &str {
        match *self {
            GetDocumentationPartError::NotFound(ref cause) => cause,
            GetDocumentationPartError::TooManyRequests(ref cause) => cause,
            GetDocumentationPartError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDocumentationParts
#[derive(Debug, PartialEq)]
pub enum GetDocumentationPartsError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetDocumentationPartsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDocumentationPartsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDocumentationPartsError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDocumentationPartsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetDocumentationPartsError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetDocumentationPartsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDocumentationPartsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDocumentationPartsError {
    fn description(&self) -> &str {
        match *self {
            GetDocumentationPartsError::BadRequest(ref cause) => cause,
            GetDocumentationPartsError::NotFound(ref cause) => cause,
            GetDocumentationPartsError::TooManyRequests(ref cause) => cause,
            GetDocumentationPartsError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDocumentationVersion
#[derive(Debug, PartialEq)]
pub enum GetDocumentationVersionError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetDocumentationVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDocumentationVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetDocumentationVersionError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetDocumentationVersionError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetDocumentationVersionError::Unauthorized(
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
impl fmt::Display for GetDocumentationVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDocumentationVersionError {
    fn description(&self) -> &str {
        match *self {
            GetDocumentationVersionError::NotFound(ref cause) => cause,
            GetDocumentationVersionError::TooManyRequests(ref cause) => cause,
            GetDocumentationVersionError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDocumentationVersions
#[derive(Debug, PartialEq)]
pub enum GetDocumentationVersionsError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetDocumentationVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDocumentationVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDocumentationVersionsError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetDocumentationVersionsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetDocumentationVersionsError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetDocumentationVersionsError::Unauthorized(
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
impl fmt::Display for GetDocumentationVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDocumentationVersionsError {
    fn description(&self) -> &str {
        match *self {
            GetDocumentationVersionsError::BadRequest(ref cause) => cause,
            GetDocumentationVersionsError::NotFound(ref cause) => cause,
            GetDocumentationVersionsError::TooManyRequests(ref cause) => cause,
            GetDocumentationVersionsError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDomainName
#[derive(Debug, PartialEq)]
pub enum GetDomainNameError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The requested service is not available. For details see the accompanying error message. Retry after the specified time period.</p>
    ServiceUnavailable(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetDomainNameError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDomainNameError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetDomainNameError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetDomainNameError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetDomainNameError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetDomainNameError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDomainNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDomainNameError {
    fn description(&self) -> &str {
        match *self {
            GetDomainNameError::NotFound(ref cause) => cause,
            GetDomainNameError::ServiceUnavailable(ref cause) => cause,
            GetDomainNameError::TooManyRequests(ref cause) => cause,
            GetDomainNameError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDomainNames
#[derive(Debug, PartialEq)]
pub enum GetDomainNamesError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetDomainNamesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDomainNamesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetDomainNamesError::BadRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetDomainNamesError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetDomainNamesError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDomainNamesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDomainNamesError {
    fn description(&self) -> &str {
        match *self {
            GetDomainNamesError::BadRequest(ref cause) => cause,
            GetDomainNamesError::TooManyRequests(ref cause) => cause,
            GetDomainNamesError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetExport
#[derive(Debug, PartialEq)]
pub enum GetExportError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetExportError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetExportError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetExportError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(GetExportError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetExportError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetExportError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetExportError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetExportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetExportError {
    fn description(&self) -> &str {
        match *self {
            GetExportError::BadRequest(ref cause) => cause,
            GetExportError::Conflict(ref cause) => cause,
            GetExportError::NotFound(ref cause) => cause,
            GetExportError::TooManyRequests(ref cause) => cause,
            GetExportError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetGatewayResponse
#[derive(Debug, PartialEq)]
pub enum GetGatewayResponseError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetGatewayResponseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetGatewayResponseError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetGatewayResponseError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetGatewayResponseError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetGatewayResponseError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetGatewayResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGatewayResponseError {
    fn description(&self) -> &str {
        match *self {
            GetGatewayResponseError::NotFound(ref cause) => cause,
            GetGatewayResponseError::TooManyRequests(ref cause) => cause,
            GetGatewayResponseError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetGatewayResponses
#[derive(Debug, PartialEq)]
pub enum GetGatewayResponsesError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetGatewayResponsesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetGatewayResponsesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetGatewayResponsesError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetGatewayResponsesError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetGatewayResponsesError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetGatewayResponsesError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetGatewayResponsesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetGatewayResponsesError {
    fn description(&self) -> &str {
        match *self {
            GetGatewayResponsesError::BadRequest(ref cause) => cause,
            GetGatewayResponsesError::NotFound(ref cause) => cause,
            GetGatewayResponsesError::TooManyRequests(ref cause) => cause,
            GetGatewayResponsesError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetIntegration
#[derive(Debug, PartialEq)]
pub enum GetIntegrationError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetIntegrationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetIntegrationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetIntegrationError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetIntegrationError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetIntegrationError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetIntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetIntegrationError {
    fn description(&self) -> &str {
        match *self {
            GetIntegrationError::NotFound(ref cause) => cause,
            GetIntegrationError::TooManyRequests(ref cause) => cause,
            GetIntegrationError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetIntegrationResponse
#[derive(Debug, PartialEq)]
pub enum GetIntegrationResponseError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetIntegrationResponseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetIntegrationResponseError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetIntegrationResponseError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetIntegrationResponseError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetIntegrationResponseError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetIntegrationResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetIntegrationResponseError {
    fn description(&self) -> &str {
        match *self {
            GetIntegrationResponseError::NotFound(ref cause) => cause,
            GetIntegrationResponseError::TooManyRequests(ref cause) => cause,
            GetIntegrationResponseError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMethod
#[derive(Debug, PartialEq)]
pub enum GetMethodError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetMethodError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMethodError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetMethodError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetMethodError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetMethodError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetMethodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMethodError {
    fn description(&self) -> &str {
        match *self {
            GetMethodError::NotFound(ref cause) => cause,
            GetMethodError::TooManyRequests(ref cause) => cause,
            GetMethodError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetMethodResponse
#[derive(Debug, PartialEq)]
pub enum GetMethodResponseError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetMethodResponseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetMethodResponseError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetMethodResponseError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetMethodResponseError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetMethodResponseError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetMethodResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetMethodResponseError {
    fn description(&self) -> &str {
        match *self {
            GetMethodResponseError::NotFound(ref cause) => cause,
            GetMethodResponseError::TooManyRequests(ref cause) => cause,
            GetMethodResponseError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetModel
#[derive(Debug, PartialEq)]
pub enum GetModelError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetModelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetModelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetModelError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetModelError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetModelError {
    fn description(&self) -> &str {
        match *self {
            GetModelError::NotFound(ref cause) => cause,
            GetModelError::TooManyRequests(ref cause) => cause,
            GetModelError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetModelTemplate
#[derive(Debug, PartialEq)]
pub enum GetModelTemplateError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetModelTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetModelTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetModelTemplateError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetModelTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetModelTemplateError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetModelTemplateError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetModelTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetModelTemplateError {
    fn description(&self) -> &str {
        match *self {
            GetModelTemplateError::BadRequest(ref cause) => cause,
            GetModelTemplateError::NotFound(ref cause) => cause,
            GetModelTemplateError::TooManyRequests(ref cause) => cause,
            GetModelTemplateError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetModels
#[derive(Debug, PartialEq)]
pub enum GetModelsError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetModelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetModelsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetModelsError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetModelsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetModelsError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetModelsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetModelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetModelsError {
    fn description(&self) -> &str {
        match *self {
            GetModelsError::BadRequest(ref cause) => cause,
            GetModelsError::NotFound(ref cause) => cause,
            GetModelsError::TooManyRequests(ref cause) => cause,
            GetModelsError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRequestValidator
#[derive(Debug, PartialEq)]
pub enum GetRequestValidatorError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetRequestValidatorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRequestValidatorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetRequestValidatorError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetRequestValidatorError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetRequestValidatorError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRequestValidatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRequestValidatorError {
    fn description(&self) -> &str {
        match *self {
            GetRequestValidatorError::NotFound(ref cause) => cause,
            GetRequestValidatorError::TooManyRequests(ref cause) => cause,
            GetRequestValidatorError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRequestValidators
#[derive(Debug, PartialEq)]
pub enum GetRequestValidatorsError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetRequestValidatorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRequestValidatorsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetRequestValidatorsError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetRequestValidatorsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetRequestValidatorsError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetRequestValidatorsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRequestValidatorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRequestValidatorsError {
    fn description(&self) -> &str {
        match *self {
            GetRequestValidatorsError::BadRequest(ref cause) => cause,
            GetRequestValidatorsError::NotFound(ref cause) => cause,
            GetRequestValidatorsError::TooManyRequests(ref cause) => cause,
            GetRequestValidatorsError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetResource
#[derive(Debug, PartialEq)]
pub enum GetResourceError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetResourceError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetResourceError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetResourceError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetResourceError {
    fn description(&self) -> &str {
        match *self {
            GetResourceError::NotFound(ref cause) => cause,
            GetResourceError::TooManyRequests(ref cause) => cause,
            GetResourceError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetResources
#[derive(Debug, PartialEq)]
pub enum GetResourcesError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetResourcesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetResourcesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetResourcesError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetResourcesError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetResourcesError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetResourcesError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetResourcesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetResourcesError {
    fn description(&self) -> &str {
        match *self {
            GetResourcesError::BadRequest(ref cause) => cause,
            GetResourcesError::NotFound(ref cause) => cause,
            GetResourcesError::TooManyRequests(ref cause) => cause,
            GetResourcesError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRestApi
#[derive(Debug, PartialEq)]
pub enum GetRestApiError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetRestApiError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRestApiError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetRestApiError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetRestApiError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetRestApiError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRestApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRestApiError {
    fn description(&self) -> &str {
        match *self {
            GetRestApiError::NotFound(ref cause) => cause,
            GetRestApiError::TooManyRequests(ref cause) => cause,
            GetRestApiError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetRestApis
#[derive(Debug, PartialEq)]
pub enum GetRestApisError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetRestApisError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRestApisError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetRestApisError::BadRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetRestApisError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetRestApisError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetRestApisError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetRestApisError {
    fn description(&self) -> &str {
        match *self {
            GetRestApisError::BadRequest(ref cause) => cause,
            GetRestApisError::TooManyRequests(ref cause) => cause,
            GetRestApisError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSdk
#[derive(Debug, PartialEq)]
pub enum GetSdkError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetSdkError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSdkError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetSdkError::BadRequest(err.msg))
                }
                "ConflictException" => return RusotoError::Service(GetSdkError::Conflict(err.msg)),
                "NotFoundException" => return RusotoError::Service(GetSdkError::NotFound(err.msg)),
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetSdkError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetSdkError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetSdkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSdkError {
    fn description(&self) -> &str {
        match *self {
            GetSdkError::BadRequest(ref cause) => cause,
            GetSdkError::Conflict(ref cause) => cause,
            GetSdkError::NotFound(ref cause) => cause,
            GetSdkError::TooManyRequests(ref cause) => cause,
            GetSdkError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSdkType
#[derive(Debug, PartialEq)]
pub enum GetSdkTypeError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetSdkTypeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSdkTypeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetSdkTypeError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetSdkTypeError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetSdkTypeError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetSdkTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSdkTypeError {
    fn description(&self) -> &str {
        match *self {
            GetSdkTypeError::NotFound(ref cause) => cause,
            GetSdkTypeError::TooManyRequests(ref cause) => cause,
            GetSdkTypeError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSdkTypes
#[derive(Debug, PartialEq)]
pub enum GetSdkTypesError {
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetSdkTypesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSdkTypesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetSdkTypesError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetSdkTypesError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetSdkTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSdkTypesError {
    fn description(&self) -> &str {
        match *self {
            GetSdkTypesError::TooManyRequests(ref cause) => cause,
            GetSdkTypesError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetStage
#[derive(Debug, PartialEq)]
pub enum GetStageError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetStageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetStageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetStageError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetStageError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetStageError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetStageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetStageError {
    fn description(&self) -> &str {
        match *self {
            GetStageError::NotFound(ref cause) => cause,
            GetStageError::TooManyRequests(ref cause) => cause,
            GetStageError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetStages
#[derive(Debug, PartialEq)]
pub enum GetStagesError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetStagesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetStagesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetStagesError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetStagesError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetStagesError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetStagesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetStagesError {
    fn description(&self) -> &str {
        match *self {
            GetStagesError::NotFound(ref cause) => cause,
            GetStagesError::TooManyRequests(ref cause) => cause,
            GetStagesError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTags
#[derive(Debug, PartialEq)]
pub enum GetTagsError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTagsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetTagsError::BadRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(GetTagsError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetTagsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetTagsError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetTagsError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTagsError {
    fn description(&self) -> &str {
        match *self {
            GetTagsError::BadRequest(ref cause) => cause,
            GetTagsError::LimitExceeded(ref cause) => cause,
            GetTagsError::NotFound(ref cause) => cause,
            GetTagsError::TooManyRequests(ref cause) => cause,
            GetTagsError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUsage
#[derive(Debug, PartialEq)]
pub enum GetUsageError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetUsageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetUsageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetUsageError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetUsageError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetUsageError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetUsageError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetUsageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUsageError {
    fn description(&self) -> &str {
        match *self {
            GetUsageError::BadRequest(ref cause) => cause,
            GetUsageError::NotFound(ref cause) => cause,
            GetUsageError::TooManyRequests(ref cause) => cause,
            GetUsageError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUsagePlan
#[derive(Debug, PartialEq)]
pub enum GetUsagePlanError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetUsagePlanError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetUsagePlanError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetUsagePlanError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetUsagePlanError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetUsagePlanError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetUsagePlanError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetUsagePlanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUsagePlanError {
    fn description(&self) -> &str {
        match *self {
            GetUsagePlanError::BadRequest(ref cause) => cause,
            GetUsagePlanError::NotFound(ref cause) => cause,
            GetUsagePlanError::TooManyRequests(ref cause) => cause,
            GetUsagePlanError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUsagePlanKey
#[derive(Debug, PartialEq)]
pub enum GetUsagePlanKeyError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetUsagePlanKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetUsagePlanKeyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetUsagePlanKeyError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetUsagePlanKeyError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetUsagePlanKeyError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetUsagePlanKeyError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetUsagePlanKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUsagePlanKeyError {
    fn description(&self) -> &str {
        match *self {
            GetUsagePlanKeyError::BadRequest(ref cause) => cause,
            GetUsagePlanKeyError::NotFound(ref cause) => cause,
            GetUsagePlanKeyError::TooManyRequests(ref cause) => cause,
            GetUsagePlanKeyError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUsagePlanKeys
#[derive(Debug, PartialEq)]
pub enum GetUsagePlanKeysError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetUsagePlanKeysError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetUsagePlanKeysError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetUsagePlanKeysError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetUsagePlanKeysError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetUsagePlanKeysError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetUsagePlanKeysError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetUsagePlanKeysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUsagePlanKeysError {
    fn description(&self) -> &str {
        match *self {
            GetUsagePlanKeysError::BadRequest(ref cause) => cause,
            GetUsagePlanKeysError::NotFound(ref cause) => cause,
            GetUsagePlanKeysError::TooManyRequests(ref cause) => cause,
            GetUsagePlanKeysError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetUsagePlans
#[derive(Debug, PartialEq)]
pub enum GetUsagePlansError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetUsagePlansError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetUsagePlansError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetUsagePlansError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(GetUsagePlansError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetUsagePlansError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetUsagePlansError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetUsagePlansError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetUsagePlansError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetUsagePlansError {
    fn description(&self) -> &str {
        match *self {
            GetUsagePlansError::BadRequest(ref cause) => cause,
            GetUsagePlansError::Conflict(ref cause) => cause,
            GetUsagePlansError::NotFound(ref cause) => cause,
            GetUsagePlansError::TooManyRequests(ref cause) => cause,
            GetUsagePlansError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetVpcLink
#[derive(Debug, PartialEq)]
pub enum GetVpcLinkError {
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetVpcLinkError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetVpcLinkError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "NotFoundException" => {
                    return RusotoError::Service(GetVpcLinkError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetVpcLinkError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetVpcLinkError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetVpcLinkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetVpcLinkError {
    fn description(&self) -> &str {
        match *self {
            GetVpcLinkError::NotFound(ref cause) => cause,
            GetVpcLinkError::TooManyRequests(ref cause) => cause,
            GetVpcLinkError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by GetVpcLinks
#[derive(Debug, PartialEq)]
pub enum GetVpcLinksError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl GetVpcLinksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetVpcLinksError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetVpcLinksError::BadRequest(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetVpcLinksError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(GetVpcLinksError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetVpcLinksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetVpcLinksError {
    fn description(&self) -> &str {
        match *self {
            GetVpcLinksError::BadRequest(ref cause) => cause,
            GetVpcLinksError::TooManyRequests(ref cause) => cause,
            GetVpcLinksError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by ImportApiKeys
#[derive(Debug, PartialEq)]
pub enum ImportApiKeysError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl ImportApiKeysError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportApiKeysError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ImportApiKeysError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(ImportApiKeysError::Conflict(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ImportApiKeysError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ImportApiKeysError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ImportApiKeysError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ImportApiKeysError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ImportApiKeysError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ImportApiKeysError {
    fn description(&self) -> &str {
        match *self {
            ImportApiKeysError::BadRequest(ref cause) => cause,
            ImportApiKeysError::Conflict(ref cause) => cause,
            ImportApiKeysError::LimitExceeded(ref cause) => cause,
            ImportApiKeysError::NotFound(ref cause) => cause,
            ImportApiKeysError::TooManyRequests(ref cause) => cause,
            ImportApiKeysError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by ImportDocumentationParts
#[derive(Debug, PartialEq)]
pub enum ImportDocumentationPartsError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl ImportDocumentationPartsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportDocumentationPartsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ImportDocumentationPartsError::BadRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ImportDocumentationPartsError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ImportDocumentationPartsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ImportDocumentationPartsError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ImportDocumentationPartsError::Unauthorized(
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
impl fmt::Display for ImportDocumentationPartsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ImportDocumentationPartsError {
    fn description(&self) -> &str {
        match *self {
            ImportDocumentationPartsError::BadRequest(ref cause) => cause,
            ImportDocumentationPartsError::LimitExceeded(ref cause) => cause,
            ImportDocumentationPartsError::NotFound(ref cause) => cause,
            ImportDocumentationPartsError::TooManyRequests(ref cause) => cause,
            ImportDocumentationPartsError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by ImportRestApi
#[derive(Debug, PartialEq)]
pub enum ImportRestApiError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl ImportRestApiError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportRestApiError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ImportRestApiError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(ImportRestApiError::Conflict(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ImportRestApiError::LimitExceeded(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ImportRestApiError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(ImportRestApiError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ImportRestApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ImportRestApiError {
    fn description(&self) -> &str {
        match *self {
            ImportRestApiError::BadRequest(ref cause) => cause,
            ImportRestApiError::Conflict(ref cause) => cause,
            ImportRestApiError::LimitExceeded(ref cause) => cause,
            ImportRestApiError::TooManyRequests(ref cause) => cause,
            ImportRestApiError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by PutGatewayResponse
#[derive(Debug, PartialEq)]
pub enum PutGatewayResponseError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl PutGatewayResponseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutGatewayResponseError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutGatewayResponseError::BadRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutGatewayResponseError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutGatewayResponseError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PutGatewayResponseError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(PutGatewayResponseError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutGatewayResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutGatewayResponseError {
    fn description(&self) -> &str {
        match *self {
            PutGatewayResponseError::BadRequest(ref cause) => cause,
            PutGatewayResponseError::LimitExceeded(ref cause) => cause,
            PutGatewayResponseError::NotFound(ref cause) => cause,
            PutGatewayResponseError::TooManyRequests(ref cause) => cause,
            PutGatewayResponseError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by PutIntegration
#[derive(Debug, PartialEq)]
pub enum PutIntegrationError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl PutIntegrationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutIntegrationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutIntegrationError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(PutIntegrationError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutIntegrationError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PutIntegrationError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(PutIntegrationError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutIntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutIntegrationError {
    fn description(&self) -> &str {
        match *self {
            PutIntegrationError::BadRequest(ref cause) => cause,
            PutIntegrationError::Conflict(ref cause) => cause,
            PutIntegrationError::NotFound(ref cause) => cause,
            PutIntegrationError::TooManyRequests(ref cause) => cause,
            PutIntegrationError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by PutIntegrationResponse
#[derive(Debug, PartialEq)]
pub enum PutIntegrationResponseError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl PutIntegrationResponseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutIntegrationResponseError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutIntegrationResponseError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(PutIntegrationResponseError::Conflict(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutIntegrationResponseError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutIntegrationResponseError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PutIntegrationResponseError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(PutIntegrationResponseError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutIntegrationResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutIntegrationResponseError {
    fn description(&self) -> &str {
        match *self {
            PutIntegrationResponseError::BadRequest(ref cause) => cause,
            PutIntegrationResponseError::Conflict(ref cause) => cause,
            PutIntegrationResponseError::LimitExceeded(ref cause) => cause,
            PutIntegrationResponseError::NotFound(ref cause) => cause,
            PutIntegrationResponseError::TooManyRequests(ref cause) => cause,
            PutIntegrationResponseError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by PutMethod
#[derive(Debug, PartialEq)]
pub enum PutMethodError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl PutMethodError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutMethodError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutMethodError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(PutMethodError::Conflict(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutMethodError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutMethodError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PutMethodError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(PutMethodError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutMethodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutMethodError {
    fn description(&self) -> &str {
        match *self {
            PutMethodError::BadRequest(ref cause) => cause,
            PutMethodError::Conflict(ref cause) => cause,
            PutMethodError::LimitExceeded(ref cause) => cause,
            PutMethodError::NotFound(ref cause) => cause,
            PutMethodError::TooManyRequests(ref cause) => cause,
            PutMethodError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by PutMethodResponse
#[derive(Debug, PartialEq)]
pub enum PutMethodResponseError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl PutMethodResponseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutMethodResponseError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutMethodResponseError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(PutMethodResponseError::Conflict(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutMethodResponseError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutMethodResponseError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PutMethodResponseError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(PutMethodResponseError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutMethodResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutMethodResponseError {
    fn description(&self) -> &str {
        match *self {
            PutMethodResponseError::BadRequest(ref cause) => cause,
            PutMethodResponseError::Conflict(ref cause) => cause,
            PutMethodResponseError::LimitExceeded(ref cause) => cause,
            PutMethodResponseError::NotFound(ref cause) => cause,
            PutMethodResponseError::TooManyRequests(ref cause) => cause,
            PutMethodResponseError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by PutRestApi
#[derive(Debug, PartialEq)]
pub enum PutRestApiError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl PutRestApiError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutRestApiError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(PutRestApiError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(PutRestApiError::Conflict(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(PutRestApiError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PutRestApiError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PutRestApiError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(PutRestApiError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutRestApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutRestApiError {
    fn description(&self) -> &str {
        match *self {
            PutRestApiError::BadRequest(ref cause) => cause,
            PutRestApiError::Conflict(ref cause) => cause,
            PutRestApiError::LimitExceeded(ref cause) => cause,
            PutRestApiError::NotFound(ref cause) => cause,
            PutRestApiError::TooManyRequests(ref cause) => cause,
            PutRestApiError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TagResourceError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(TagResourceError::Conflict(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(TagResourceError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TagResourceError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(TagResourceError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(TagResourceError::Unauthorized(err.msg))
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
            TagResourceError::BadRequest(ref cause) => cause,
            TagResourceError::Conflict(ref cause) => cause,
            TagResourceError::LimitExceeded(ref cause) => cause,
            TagResourceError::NotFound(ref cause) => cause,
            TagResourceError::TooManyRequests(ref cause) => cause,
            TagResourceError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by TestInvokeAuthorizer
#[derive(Debug, PartialEq)]
pub enum TestInvokeAuthorizerError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl TestInvokeAuthorizerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TestInvokeAuthorizerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TestInvokeAuthorizerError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TestInvokeAuthorizerError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(TestInvokeAuthorizerError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(TestInvokeAuthorizerError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for TestInvokeAuthorizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TestInvokeAuthorizerError {
    fn description(&self) -> &str {
        match *self {
            TestInvokeAuthorizerError::BadRequest(ref cause) => cause,
            TestInvokeAuthorizerError::NotFound(ref cause) => cause,
            TestInvokeAuthorizerError::TooManyRequests(ref cause) => cause,
            TestInvokeAuthorizerError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by TestInvokeMethod
#[derive(Debug, PartialEq)]
pub enum TestInvokeMethodError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl TestInvokeMethodError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TestInvokeMethodError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TestInvokeMethodError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TestInvokeMethodError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(TestInvokeMethodError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(TestInvokeMethodError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for TestInvokeMethodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TestInvokeMethodError {
    fn description(&self) -> &str {
        match *self {
            TestInvokeMethodError::BadRequest(ref cause) => cause,
            TestInvokeMethodError::NotFound(ref cause) => cause,
            TestInvokeMethodError::TooManyRequests(ref cause) => cause,
            TestInvokeMethodError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UntagResourceError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UntagResourceError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UntagResourceError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UntagResourceError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UntagResourceError::Unauthorized(err.msg))
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
            UntagResourceError::BadRequest(ref cause) => cause,
            UntagResourceError::Conflict(ref cause) => cause,
            UntagResourceError::NotFound(ref cause) => cause,
            UntagResourceError::TooManyRequests(ref cause) => cause,
            UntagResourceError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAccount
#[derive(Debug, PartialEq)]
pub enum UpdateAccountError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UpdateAccountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAccountError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateAccountError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateAccountError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateAccountError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateAccountError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAccountError {
    fn description(&self) -> &str {
        match *self {
            UpdateAccountError::BadRequest(ref cause) => cause,
            UpdateAccountError::NotFound(ref cause) => cause,
            UpdateAccountError::TooManyRequests(ref cause) => cause,
            UpdateAccountError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApiKey
#[derive(Debug, PartialEq)]
pub enum UpdateApiKeyError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UpdateApiKeyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateApiKeyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateApiKeyError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateApiKeyError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateApiKeyError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateApiKeyError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateApiKeyError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateApiKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApiKeyError {
    fn description(&self) -> &str {
        match *self {
            UpdateApiKeyError::BadRequest(ref cause) => cause,
            UpdateApiKeyError::Conflict(ref cause) => cause,
            UpdateApiKeyError::NotFound(ref cause) => cause,
            UpdateApiKeyError::TooManyRequests(ref cause) => cause,
            UpdateApiKeyError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAuthorizer
#[derive(Debug, PartialEq)]
pub enum UpdateAuthorizerError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UpdateAuthorizerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAuthorizerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateAuthorizerError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateAuthorizerError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateAuthorizerError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateAuthorizerError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateAuthorizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAuthorizerError {
    fn description(&self) -> &str {
        match *self {
            UpdateAuthorizerError::BadRequest(ref cause) => cause,
            UpdateAuthorizerError::NotFound(ref cause) => cause,
            UpdateAuthorizerError::TooManyRequests(ref cause) => cause,
            UpdateAuthorizerError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateBasePathMapping
#[derive(Debug, PartialEq)]
pub enum UpdateBasePathMappingError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UpdateBasePathMappingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateBasePathMappingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateBasePathMappingError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateBasePathMappingError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateBasePathMappingError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateBasePathMappingError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateBasePathMappingError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateBasePathMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateBasePathMappingError {
    fn description(&self) -> &str {
        match *self {
            UpdateBasePathMappingError::BadRequest(ref cause) => cause,
            UpdateBasePathMappingError::Conflict(ref cause) => cause,
            UpdateBasePathMappingError::NotFound(ref cause) => cause,
            UpdateBasePathMappingError::TooManyRequests(ref cause) => cause,
            UpdateBasePathMappingError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateClientCertificate
#[derive(Debug, PartialEq)]
pub enum UpdateClientCertificateError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UpdateClientCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateClientCertificateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateClientCertificateError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateClientCertificateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateClientCertificateError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateClientCertificateError::Unauthorized(
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
impl fmt::Display for UpdateClientCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateClientCertificateError {
    fn description(&self) -> &str {
        match *self {
            UpdateClientCertificateError::BadRequest(ref cause) => cause,
            UpdateClientCertificateError::NotFound(ref cause) => cause,
            UpdateClientCertificateError::TooManyRequests(ref cause) => cause,
            UpdateClientCertificateError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDeployment
#[derive(Debug, PartialEq)]
pub enum UpdateDeploymentError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The requested service is not available. For details see the accompanying error message. Retry after the specified time period.</p>
    ServiceUnavailable(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UpdateDeploymentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDeploymentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateDeploymentError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateDeploymentError::NotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateDeploymentError::ServiceUnavailable(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateDeploymentError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateDeploymentError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateDeploymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDeploymentError {
    fn description(&self) -> &str {
        match *self {
            UpdateDeploymentError::BadRequest(ref cause) => cause,
            UpdateDeploymentError::NotFound(ref cause) => cause,
            UpdateDeploymentError::ServiceUnavailable(ref cause) => cause,
            UpdateDeploymentError::TooManyRequests(ref cause) => cause,
            UpdateDeploymentError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDocumentationPart
#[derive(Debug, PartialEq)]
pub enum UpdateDocumentationPartError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UpdateDocumentationPartError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDocumentationPartError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateDocumentationPartError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateDocumentationPartError::Conflict(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateDocumentationPartError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateDocumentationPartError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateDocumentationPartError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateDocumentationPartError::Unauthorized(
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
impl fmt::Display for UpdateDocumentationPartError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDocumentationPartError {
    fn description(&self) -> &str {
        match *self {
            UpdateDocumentationPartError::BadRequest(ref cause) => cause,
            UpdateDocumentationPartError::Conflict(ref cause) => cause,
            UpdateDocumentationPartError::LimitExceeded(ref cause) => cause,
            UpdateDocumentationPartError::NotFound(ref cause) => cause,
            UpdateDocumentationPartError::TooManyRequests(ref cause) => cause,
            UpdateDocumentationPartError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDocumentationVersion
#[derive(Debug, PartialEq)]
pub enum UpdateDocumentationVersionError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UpdateDocumentationVersionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateDocumentationVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateDocumentationVersionError::BadRequest(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateDocumentationVersionError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateDocumentationVersionError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateDocumentationVersionError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateDocumentationVersionError::Unauthorized(
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
impl fmt::Display for UpdateDocumentationVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDocumentationVersionError {
    fn description(&self) -> &str {
        match *self {
            UpdateDocumentationVersionError::BadRequest(ref cause) => cause,
            UpdateDocumentationVersionError::Conflict(ref cause) => cause,
            UpdateDocumentationVersionError::NotFound(ref cause) => cause,
            UpdateDocumentationVersionError::TooManyRequests(ref cause) => cause,
            UpdateDocumentationVersionError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDomainName
#[derive(Debug, PartialEq)]
pub enum UpdateDomainNameError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UpdateDomainNameError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDomainNameError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateDomainNameError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateDomainNameError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateDomainNameError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateDomainNameError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateDomainNameError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateDomainNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDomainNameError {
    fn description(&self) -> &str {
        match *self {
            UpdateDomainNameError::BadRequest(ref cause) => cause,
            UpdateDomainNameError::Conflict(ref cause) => cause,
            UpdateDomainNameError::NotFound(ref cause) => cause,
            UpdateDomainNameError::TooManyRequests(ref cause) => cause,
            UpdateDomainNameError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateGatewayResponse
#[derive(Debug, PartialEq)]
pub enum UpdateGatewayResponseError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UpdateGatewayResponseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateGatewayResponseError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateGatewayResponseError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateGatewayResponseError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateGatewayResponseError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateGatewayResponseError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateGatewayResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateGatewayResponseError {
    fn description(&self) -> &str {
        match *self {
            UpdateGatewayResponseError::BadRequest(ref cause) => cause,
            UpdateGatewayResponseError::NotFound(ref cause) => cause,
            UpdateGatewayResponseError::TooManyRequests(ref cause) => cause,
            UpdateGatewayResponseError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateIntegration
#[derive(Debug, PartialEq)]
pub enum UpdateIntegrationError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UpdateIntegrationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateIntegrationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateIntegrationError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateIntegrationError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateIntegrationError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateIntegrationError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateIntegrationError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateIntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateIntegrationError {
    fn description(&self) -> &str {
        match *self {
            UpdateIntegrationError::BadRequest(ref cause) => cause,
            UpdateIntegrationError::Conflict(ref cause) => cause,
            UpdateIntegrationError::NotFound(ref cause) => cause,
            UpdateIntegrationError::TooManyRequests(ref cause) => cause,
            UpdateIntegrationError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateIntegrationResponse
#[derive(Debug, PartialEq)]
pub enum UpdateIntegrationResponseError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UpdateIntegrationResponseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateIntegrationResponseError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateIntegrationResponseError::BadRequest(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateIntegrationResponseError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateIntegrationResponseError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateIntegrationResponseError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateIntegrationResponseError::Unauthorized(
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
impl fmt::Display for UpdateIntegrationResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateIntegrationResponseError {
    fn description(&self) -> &str {
        match *self {
            UpdateIntegrationResponseError::BadRequest(ref cause) => cause,
            UpdateIntegrationResponseError::Conflict(ref cause) => cause,
            UpdateIntegrationResponseError::NotFound(ref cause) => cause,
            UpdateIntegrationResponseError::TooManyRequests(ref cause) => cause,
            UpdateIntegrationResponseError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateMethod
#[derive(Debug, PartialEq)]
pub enum UpdateMethodError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UpdateMethodError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateMethodError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateMethodError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateMethodError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateMethodError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateMethodError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateMethodError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateMethodError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateMethodError {
    fn description(&self) -> &str {
        match *self {
            UpdateMethodError::BadRequest(ref cause) => cause,
            UpdateMethodError::Conflict(ref cause) => cause,
            UpdateMethodError::NotFound(ref cause) => cause,
            UpdateMethodError::TooManyRequests(ref cause) => cause,
            UpdateMethodError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateMethodResponse
#[derive(Debug, PartialEq)]
pub enum UpdateMethodResponseError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The request exceeded the rate limit. Retry after the specified time period.</p>
    LimitExceeded(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UpdateMethodResponseError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateMethodResponseError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateMethodResponseError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateMethodResponseError::Conflict(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateMethodResponseError::LimitExceeded(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateMethodResponseError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateMethodResponseError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateMethodResponseError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateMethodResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateMethodResponseError {
    fn description(&self) -> &str {
        match *self {
            UpdateMethodResponseError::BadRequest(ref cause) => cause,
            UpdateMethodResponseError::Conflict(ref cause) => cause,
            UpdateMethodResponseError::LimitExceeded(ref cause) => cause,
            UpdateMethodResponseError::NotFound(ref cause) => cause,
            UpdateMethodResponseError::TooManyRequests(ref cause) => cause,
            UpdateMethodResponseError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateModel
#[derive(Debug, PartialEq)]
pub enum UpdateModelError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UpdateModelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateModelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateModelError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateModelError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateModelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateModelError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateModelError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateModelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateModelError {
    fn description(&self) -> &str {
        match *self {
            UpdateModelError::BadRequest(ref cause) => cause,
            UpdateModelError::Conflict(ref cause) => cause,
            UpdateModelError::NotFound(ref cause) => cause,
            UpdateModelError::TooManyRequests(ref cause) => cause,
            UpdateModelError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateRequestValidator
#[derive(Debug, PartialEq)]
pub enum UpdateRequestValidatorError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UpdateRequestValidatorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRequestValidatorError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateRequestValidatorError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateRequestValidatorError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateRequestValidatorError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateRequestValidatorError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateRequestValidatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRequestValidatorError {
    fn description(&self) -> &str {
        match *self {
            UpdateRequestValidatorError::BadRequest(ref cause) => cause,
            UpdateRequestValidatorError::NotFound(ref cause) => cause,
            UpdateRequestValidatorError::TooManyRequests(ref cause) => cause,
            UpdateRequestValidatorError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateResource
#[derive(Debug, PartialEq)]
pub enum UpdateResourceError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UpdateResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateResourceError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateResourceError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateResourceError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateResourceError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateResourceError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateResourceError {
    fn description(&self) -> &str {
        match *self {
            UpdateResourceError::BadRequest(ref cause) => cause,
            UpdateResourceError::Conflict(ref cause) => cause,
            UpdateResourceError::NotFound(ref cause) => cause,
            UpdateResourceError::TooManyRequests(ref cause) => cause,
            UpdateResourceError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateRestApi
#[derive(Debug, PartialEq)]
pub enum UpdateRestApiError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UpdateRestApiError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRestApiError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateRestApiError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateRestApiError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateRestApiError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateRestApiError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateRestApiError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateRestApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRestApiError {
    fn description(&self) -> &str {
        match *self {
            UpdateRestApiError::BadRequest(ref cause) => cause,
            UpdateRestApiError::Conflict(ref cause) => cause,
            UpdateRestApiError::NotFound(ref cause) => cause,
            UpdateRestApiError::TooManyRequests(ref cause) => cause,
            UpdateRestApiError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateStage
#[derive(Debug, PartialEq)]
pub enum UpdateStageError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UpdateStageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateStageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateStageError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateStageError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateStageError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateStageError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateStageError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateStageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateStageError {
    fn description(&self) -> &str {
        match *self {
            UpdateStageError::BadRequest(ref cause) => cause,
            UpdateStageError::Conflict(ref cause) => cause,
            UpdateStageError::NotFound(ref cause) => cause,
            UpdateStageError::TooManyRequests(ref cause) => cause,
            UpdateStageError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUsage
#[derive(Debug, PartialEq)]
pub enum UpdateUsageError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UpdateUsageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUsageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateUsageError::BadRequest(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateUsageError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateUsageError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateUsageError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateUsageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUsageError {
    fn description(&self) -> &str {
        match *self {
            UpdateUsageError::BadRequest(ref cause) => cause,
            UpdateUsageError::NotFound(ref cause) => cause,
            UpdateUsageError::TooManyRequests(ref cause) => cause,
            UpdateUsageError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateUsagePlan
#[derive(Debug, PartialEq)]
pub enum UpdateUsagePlanError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UpdateUsagePlanError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateUsagePlanError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateUsagePlanError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateUsagePlanError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateUsagePlanError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateUsagePlanError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateUsagePlanError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateUsagePlanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateUsagePlanError {
    fn description(&self) -> &str {
        match *self {
            UpdateUsagePlanError::BadRequest(ref cause) => cause,
            UpdateUsagePlanError::Conflict(ref cause) => cause,
            UpdateUsagePlanError::NotFound(ref cause) => cause,
            UpdateUsagePlanError::TooManyRequests(ref cause) => cause,
            UpdateUsagePlanError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateVpcLink
#[derive(Debug, PartialEq)]
pub enum UpdateVpcLinkError {
    /// <p>The submitted request is not valid, for example, the input is incomplete or incorrect. See the accompanying error message for details.</p>
    BadRequest(String),
    /// <p>The request configuration has conflicts. For details, see the accompanying error message.</p>
    Conflict(String),
    /// <p>The requested resource is not found. Make sure that the request URI is correct.</p>
    NotFound(String),
    /// <p>The request has reached its throttling limit. Retry after the specified time period.</p>
    TooManyRequests(String),
    /// <p>The request is denied because the caller has insufficient permissions.</p>
    Unauthorized(String),
}

impl UpdateVpcLinkError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateVpcLinkError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateVpcLinkError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateVpcLinkError::Conflict(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateVpcLinkError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateVpcLinkError::TooManyRequests(err.msg))
                }
                "UnauthorizedException" => {
                    return RusotoError::Service(UpdateVpcLinkError::Unauthorized(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateVpcLinkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateVpcLinkError {
    fn description(&self) -> &str {
        match *self {
            UpdateVpcLinkError::BadRequest(ref cause) => cause,
            UpdateVpcLinkError::Conflict(ref cause) => cause,
            UpdateVpcLinkError::NotFound(ref cause) => cause,
            UpdateVpcLinkError::TooManyRequests(ref cause) => cause,
            UpdateVpcLinkError::Unauthorized(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon API Gateway API. Amazon API Gateway clients implement this trait.
pub trait ApiGateway {
    /// <p><p>Create an <a>ApiKey</a> resource. </p> <div class="seeAlso"><a href="https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-api-key.html">AWS CLI</a></div></p>
    fn create_api_key(&self, input: CreateApiKeyRequest)
        -> RusotoFuture<ApiKey, CreateApiKeyError>;

    /// <p><p>Adds a new <a>Authorizer</a> resource to an existing <a>RestApi</a> resource.</p> <div class="seeAlso"><a href="https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-authorizer.html">AWS CLI</a></div></p>
    fn create_authorizer(
        &self,
        input: CreateAuthorizerRequest,
    ) -> RusotoFuture<Authorizer, CreateAuthorizerError>;

    /// <p>Creates a new <a>BasePathMapping</a> resource.</p>
    fn create_base_path_mapping(
        &self,
        input: CreateBasePathMappingRequest,
    ) -> RusotoFuture<BasePathMapping, CreateBasePathMappingError>;

    /// <p>Creates a <a>Deployment</a> resource, which makes a specified <a>RestApi</a> callable over the internet.</p>
    fn create_deployment(
        &self,
        input: CreateDeploymentRequest,
    ) -> RusotoFuture<Deployment, CreateDeploymentError>;

    fn create_documentation_part(
        &self,
        input: CreateDocumentationPartRequest,
    ) -> RusotoFuture<DocumentationPart, CreateDocumentationPartError>;

    fn create_documentation_version(
        &self,
        input: CreateDocumentationVersionRequest,
    ) -> RusotoFuture<DocumentationVersion, CreateDocumentationVersionError>;

    /// <p>Creates a new domain name.</p>
    fn create_domain_name(
        &self,
        input: CreateDomainNameRequest,
    ) -> RusotoFuture<DomainName, CreateDomainNameError>;

    /// <p>Adds a new <a>Model</a> resource to an existing <a>RestApi</a> resource.</p>
    fn create_model(&self, input: CreateModelRequest) -> RusotoFuture<Model, CreateModelError>;

    /// <p>Creates a <a>ReqeustValidator</a> of a given <a>RestApi</a>.</p>
    fn create_request_validator(
        &self,
        input: CreateRequestValidatorRequest,
    ) -> RusotoFuture<RequestValidator, CreateRequestValidatorError>;

    /// <p>Creates a <a>Resource</a> resource.</p>
    fn create_resource(
        &self,
        input: CreateResourceRequest,
    ) -> RusotoFuture<Resource, CreateResourceError>;

    /// <p>Creates a new <a>RestApi</a> resource.</p>
    fn create_rest_api(
        &self,
        input: CreateRestApiRequest,
    ) -> RusotoFuture<RestApi, CreateRestApiError>;

    /// <p>Creates a new <a>Stage</a> resource that references a pre-existing <a>Deployment</a> for the API. </p>
    fn create_stage(&self, input: CreateStageRequest) -> RusotoFuture<Stage, CreateStageError>;

    /// <p>Creates a usage plan with the throttle and quota limits, as well as the associated API stages, specified in the payload. </p>
    fn create_usage_plan(
        &self,
        input: CreateUsagePlanRequest,
    ) -> RusotoFuture<UsagePlan, CreateUsagePlanError>;

    /// <p>Creates a usage plan key for adding an existing API key to a usage plan.</p>
    fn create_usage_plan_key(
        &self,
        input: CreateUsagePlanKeyRequest,
    ) -> RusotoFuture<UsagePlanKey, CreateUsagePlanKeyError>;

    /// <p>Creates a VPC link, under the caller's account in a selected region, in an asynchronous operation that typically takes 2-4 minutes to complete and become operational. The caller must have permissions to create and update VPC Endpoint services.</p>
    fn create_vpc_link(
        &self,
        input: CreateVpcLinkRequest,
    ) -> RusotoFuture<VpcLink, CreateVpcLinkError>;

    /// <p>Deletes the <a>ApiKey</a> resource.</p>
    fn delete_api_key(&self, input: DeleteApiKeyRequest) -> RusotoFuture<(), DeleteApiKeyError>;

    /// <p><p>Deletes an existing <a>Authorizer</a> resource.</p> <div class="seeAlso"><a href="https://docs.aws.amazon.com/cli/latest/reference/apigateway/delete-authorizer.html">AWS CLI</a></div></p>
    fn delete_authorizer(
        &self,
        input: DeleteAuthorizerRequest,
    ) -> RusotoFuture<(), DeleteAuthorizerError>;

    /// <p>Deletes the <a>BasePathMapping</a> resource.</p>
    fn delete_base_path_mapping(
        &self,
        input: DeleteBasePathMappingRequest,
    ) -> RusotoFuture<(), DeleteBasePathMappingError>;

    /// <p>Deletes the <a>ClientCertificate</a> resource.</p>
    fn delete_client_certificate(
        &self,
        input: DeleteClientCertificateRequest,
    ) -> RusotoFuture<(), DeleteClientCertificateError>;

    /// <p>Deletes a <a>Deployment</a> resource. Deleting a deployment will only succeed if there are no <a>Stage</a> resources associated with it.</p>
    fn delete_deployment(
        &self,
        input: DeleteDeploymentRequest,
    ) -> RusotoFuture<(), DeleteDeploymentError>;

    fn delete_documentation_part(
        &self,
        input: DeleteDocumentationPartRequest,
    ) -> RusotoFuture<(), DeleteDocumentationPartError>;

    fn delete_documentation_version(
        &self,
        input: DeleteDocumentationVersionRequest,
    ) -> RusotoFuture<(), DeleteDocumentationVersionError>;

    /// <p>Deletes the <a>DomainName</a> resource.</p>
    fn delete_domain_name(
        &self,
        input: DeleteDomainNameRequest,
    ) -> RusotoFuture<(), DeleteDomainNameError>;

    /// <p>Clears any customization of a <a>GatewayResponse</a> of a specified response type on the given <a>RestApi</a> and resets it with the default settings.</p>
    fn delete_gateway_response(
        &self,
        input: DeleteGatewayResponseRequest,
    ) -> RusotoFuture<(), DeleteGatewayResponseError>;

    /// <p>Represents a delete integration.</p>
    fn delete_integration(
        &self,
        input: DeleteIntegrationRequest,
    ) -> RusotoFuture<(), DeleteIntegrationError>;

    /// <p>Represents a delete integration response.</p>
    fn delete_integration_response(
        &self,
        input: DeleteIntegrationResponseRequest,
    ) -> RusotoFuture<(), DeleteIntegrationResponseError>;

    /// <p>Deletes an existing <a>Method</a> resource.</p>
    fn delete_method(&self, input: DeleteMethodRequest) -> RusotoFuture<(), DeleteMethodError>;

    /// <p>Deletes an existing <a>MethodResponse</a> resource.</p>
    fn delete_method_response(
        &self,
        input: DeleteMethodResponseRequest,
    ) -> RusotoFuture<(), DeleteMethodResponseError>;

    /// <p>Deletes a model.</p>
    fn delete_model(&self, input: DeleteModelRequest) -> RusotoFuture<(), DeleteModelError>;

    /// <p>Deletes a <a>RequestValidator</a> of a given <a>RestApi</a>.</p>
    fn delete_request_validator(
        &self,
        input: DeleteRequestValidatorRequest,
    ) -> RusotoFuture<(), DeleteRequestValidatorError>;

    /// <p>Deletes a <a>Resource</a> resource.</p>
    fn delete_resource(
        &self,
        input: DeleteResourceRequest,
    ) -> RusotoFuture<(), DeleteResourceError>;

    /// <p>Deletes the specified API.</p>
    fn delete_rest_api(&self, input: DeleteRestApiRequest) -> RusotoFuture<(), DeleteRestApiError>;

    /// <p>Deletes a <a>Stage</a> resource.</p>
    fn delete_stage(&self, input: DeleteStageRequest) -> RusotoFuture<(), DeleteStageError>;

    /// <p>Deletes a usage plan of a given plan Id.</p>
    fn delete_usage_plan(
        &self,
        input: DeleteUsagePlanRequest,
    ) -> RusotoFuture<(), DeleteUsagePlanError>;

    /// <p>Deletes a usage plan key and remove the underlying API key from the associated usage plan.</p>
    fn delete_usage_plan_key(
        &self,
        input: DeleteUsagePlanKeyRequest,
    ) -> RusotoFuture<(), DeleteUsagePlanKeyError>;

    /// <p>Deletes an existing <a>VpcLink</a> of a specified identifier.</p>
    fn delete_vpc_link(&self, input: DeleteVpcLinkRequest) -> RusotoFuture<(), DeleteVpcLinkError>;

    /// <p>Flushes all authorizer cache entries on a stage.</p>
    fn flush_stage_authorizers_cache(
        &self,
        input: FlushStageAuthorizersCacheRequest,
    ) -> RusotoFuture<(), FlushStageAuthorizersCacheError>;

    /// <p>Flushes a stage's cache.</p>
    fn flush_stage_cache(
        &self,
        input: FlushStageCacheRequest,
    ) -> RusotoFuture<(), FlushStageCacheError>;

    /// <p>Generates a <a>ClientCertificate</a> resource.</p>
    fn generate_client_certificate(
        &self,
        input: GenerateClientCertificateRequest,
    ) -> RusotoFuture<ClientCertificate, GenerateClientCertificateError>;

    /// <p>Gets information about the current <a>Account</a> resource.</p>
    fn get_account(&self) -> RusotoFuture<Account, GetAccountError>;

    /// <p>Gets information about the current <a>ApiKey</a> resource.</p>
    fn get_api_key(&self, input: GetApiKeyRequest) -> RusotoFuture<ApiKey, GetApiKeyError>;

    /// <p>Gets information about the current <a>ApiKeys</a> resource.</p>
    fn get_api_keys(&self, input: GetApiKeysRequest) -> RusotoFuture<ApiKeys, GetApiKeysError>;

    /// <p><p>Describe an existing <a>Authorizer</a> resource.</p> <div class="seeAlso"><a href="https://docs.aws.amazon.com/cli/latest/reference/apigateway/get-authorizer.html">AWS CLI</a></div></p>
    fn get_authorizer(
        &self,
        input: GetAuthorizerRequest,
    ) -> RusotoFuture<Authorizer, GetAuthorizerError>;

    /// <p><p>Describe an existing <a>Authorizers</a> resource.</p> <div class="seeAlso"><a href="https://docs.aws.amazon.com/cli/latest/reference/apigateway/get-authorizers.html">AWS CLI</a></div></p>
    fn get_authorizers(
        &self,
        input: GetAuthorizersRequest,
    ) -> RusotoFuture<Authorizers, GetAuthorizersError>;

    /// <p>Describe a <a>BasePathMapping</a> resource.</p>
    fn get_base_path_mapping(
        &self,
        input: GetBasePathMappingRequest,
    ) -> RusotoFuture<BasePathMapping, GetBasePathMappingError>;

    /// <p>Represents a collection of <a>BasePathMapping</a> resources.</p>
    fn get_base_path_mappings(
        &self,
        input: GetBasePathMappingsRequest,
    ) -> RusotoFuture<BasePathMappings, GetBasePathMappingsError>;

    /// <p>Gets information about the current <a>ClientCertificate</a> resource.</p>
    fn get_client_certificate(
        &self,
        input: GetClientCertificateRequest,
    ) -> RusotoFuture<ClientCertificate, GetClientCertificateError>;

    /// <p>Gets a collection of <a>ClientCertificate</a> resources.</p>
    fn get_client_certificates(
        &self,
        input: GetClientCertificatesRequest,
    ) -> RusotoFuture<ClientCertificates, GetClientCertificatesError>;

    /// <p>Gets information about a <a>Deployment</a> resource.</p>
    fn get_deployment(
        &self,
        input: GetDeploymentRequest,
    ) -> RusotoFuture<Deployment, GetDeploymentError>;

    /// <p>Gets information about a <a>Deployments</a> collection.</p>
    fn get_deployments(
        &self,
        input: GetDeploymentsRequest,
    ) -> RusotoFuture<Deployments, GetDeploymentsError>;

    fn get_documentation_part(
        &self,
        input: GetDocumentationPartRequest,
    ) -> RusotoFuture<DocumentationPart, GetDocumentationPartError>;

    fn get_documentation_parts(
        &self,
        input: GetDocumentationPartsRequest,
    ) -> RusotoFuture<DocumentationParts, GetDocumentationPartsError>;

    fn get_documentation_version(
        &self,
        input: GetDocumentationVersionRequest,
    ) -> RusotoFuture<DocumentationVersion, GetDocumentationVersionError>;

    fn get_documentation_versions(
        &self,
        input: GetDocumentationVersionsRequest,
    ) -> RusotoFuture<DocumentationVersions, GetDocumentationVersionsError>;

    /// <p>Represents a domain name that is contained in a simpler, more intuitive URL that can be called.</p>
    fn get_domain_name(
        &self,
        input: GetDomainNameRequest,
    ) -> RusotoFuture<DomainName, GetDomainNameError>;

    /// <p>Represents a collection of <a>DomainName</a> resources.</p>
    fn get_domain_names(
        &self,
        input: GetDomainNamesRequest,
    ) -> RusotoFuture<DomainNames, GetDomainNamesError>;

    /// <p>Exports a deployed version of a <a>RestApi</a> in a specified format.</p>
    fn get_export(&self, input: GetExportRequest) -> RusotoFuture<ExportResponse, GetExportError>;

    /// <p>Gets a <a>GatewayResponse</a> of a specified response type on the given <a>RestApi</a>.</p>
    fn get_gateway_response(
        &self,
        input: GetGatewayResponseRequest,
    ) -> RusotoFuture<GatewayResponse, GetGatewayResponseError>;

    /// <p>Gets the <a>GatewayResponses</a> collection on the given <a>RestApi</a>. If an API developer has not added any definitions for gateway responses, the result will be the API Gateway-generated default <a>GatewayResponses</a> collection for the supported response types.</p>
    fn get_gateway_responses(
        &self,
        input: GetGatewayResponsesRequest,
    ) -> RusotoFuture<GatewayResponses, GetGatewayResponsesError>;

    /// <p>Get the integration settings.</p>
    fn get_integration(
        &self,
        input: GetIntegrationRequest,
    ) -> RusotoFuture<Integration, GetIntegrationError>;

    /// <p>Represents a get integration response.</p>
    fn get_integration_response(
        &self,
        input: GetIntegrationResponseRequest,
    ) -> RusotoFuture<IntegrationResponse, GetIntegrationResponseError>;

    /// <p>Describe an existing <a>Method</a> resource.</p>
    fn get_method(&self, input: GetMethodRequest) -> RusotoFuture<Method, GetMethodError>;

    /// <p>Describes a <a>MethodResponse</a> resource.</p>
    fn get_method_response(
        &self,
        input: GetMethodResponseRequest,
    ) -> RusotoFuture<MethodResponse, GetMethodResponseError>;

    /// <p>Describes an existing model defined for a <a>RestApi</a> resource.</p>
    fn get_model(&self, input: GetModelRequest) -> RusotoFuture<Model, GetModelError>;

    /// <p>Generates a sample mapping template that can be used to transform a payload into the structure of a model.</p>
    fn get_model_template(
        &self,
        input: GetModelTemplateRequest,
    ) -> RusotoFuture<Template, GetModelTemplateError>;

    /// <p>Describes existing <a>Models</a> defined for a <a>RestApi</a> resource.</p>
    fn get_models(&self, input: GetModelsRequest) -> RusotoFuture<Models, GetModelsError>;

    /// <p>Gets a <a>RequestValidator</a> of a given <a>RestApi</a>.</p>
    fn get_request_validator(
        &self,
        input: GetRequestValidatorRequest,
    ) -> RusotoFuture<RequestValidator, GetRequestValidatorError>;

    /// <p>Gets the <a>RequestValidators</a> collection of a given <a>RestApi</a>.</p>
    fn get_request_validators(
        &self,
        input: GetRequestValidatorsRequest,
    ) -> RusotoFuture<RequestValidators, GetRequestValidatorsError>;

    /// <p>Lists information about a resource.</p>
    fn get_resource(&self, input: GetResourceRequest) -> RusotoFuture<Resource, GetResourceError>;

    /// <p>Lists information about a collection of <a>Resource</a> resources.</p>
    fn get_resources(
        &self,
        input: GetResourcesRequest,
    ) -> RusotoFuture<Resources, GetResourcesError>;

    /// <p>Lists the <a>RestApi</a> resource in the collection.</p>
    fn get_rest_api(&self, input: GetRestApiRequest) -> RusotoFuture<RestApi, GetRestApiError>;

    /// <p>Lists the <a>RestApis</a> resources for your collection.</p>
    fn get_rest_apis(&self, input: GetRestApisRequest) -> RusotoFuture<RestApis, GetRestApisError>;

    /// <p>Generates a client SDK for a <a>RestApi</a> and <a>Stage</a>.</p>
    fn get_sdk(&self, input: GetSdkRequest) -> RusotoFuture<SdkResponse, GetSdkError>;

    fn get_sdk_type(&self, input: GetSdkTypeRequest) -> RusotoFuture<SdkType, GetSdkTypeError>;

    fn get_sdk_types(&self, input: GetSdkTypesRequest) -> RusotoFuture<SdkTypes, GetSdkTypesError>;

    /// <p>Gets information about a <a>Stage</a> resource.</p>
    fn get_stage(&self, input: GetStageRequest) -> RusotoFuture<Stage, GetStageError>;

    /// <p>Gets information about one or more <a>Stage</a> resources.</p>
    fn get_stages(&self, input: GetStagesRequest) -> RusotoFuture<Stages, GetStagesError>;

    /// <p>Gets the <a>Tags</a> collection for a given resource.</p>
    fn get_tags(&self, input: GetTagsRequest) -> RusotoFuture<Tags, GetTagsError>;

    /// <p>Gets the usage data of a usage plan in a specified time interval.</p>
    fn get_usage(&self, input: GetUsageRequest) -> RusotoFuture<Usage, GetUsageError>;

    /// <p>Gets a usage plan of a given plan identifier.</p>
    fn get_usage_plan(
        &self,
        input: GetUsagePlanRequest,
    ) -> RusotoFuture<UsagePlan, GetUsagePlanError>;

    /// <p>Gets a usage plan key of a given key identifier.</p>
    fn get_usage_plan_key(
        &self,
        input: GetUsagePlanKeyRequest,
    ) -> RusotoFuture<UsagePlanKey, GetUsagePlanKeyError>;

    /// <p>Gets all the usage plan keys representing the API keys added to a specified usage plan.</p>
    fn get_usage_plan_keys(
        &self,
        input: GetUsagePlanKeysRequest,
    ) -> RusotoFuture<UsagePlanKeys, GetUsagePlanKeysError>;

    /// <p>Gets all the usage plans of the caller's account.</p>
    fn get_usage_plans(
        &self,
        input: GetUsagePlansRequest,
    ) -> RusotoFuture<UsagePlans, GetUsagePlansError>;

    /// <p>Gets a specified VPC link under the caller's account in a region.</p>
    fn get_vpc_link(&self, input: GetVpcLinkRequest) -> RusotoFuture<VpcLink, GetVpcLinkError>;

    /// <p>Gets the <a>VpcLinks</a> collection under the caller's account in a selected region.</p>
    fn get_vpc_links(&self, input: GetVpcLinksRequest) -> RusotoFuture<VpcLinks, GetVpcLinksError>;

    /// <p>Import API keys from an external source, such as a CSV-formatted file.</p>
    fn import_api_keys(
        &self,
        input: ImportApiKeysRequest,
    ) -> RusotoFuture<ApiKeyIds, ImportApiKeysError>;

    fn import_documentation_parts(
        &self,
        input: ImportDocumentationPartsRequest,
    ) -> RusotoFuture<DocumentationPartIds, ImportDocumentationPartsError>;

    /// <p>A feature of the API Gateway control service for creating a new API from an external API definition file.</p>
    fn import_rest_api(
        &self,
        input: ImportRestApiRequest,
    ) -> RusotoFuture<RestApi, ImportRestApiError>;

    /// <p>Creates a customization of a <a>GatewayResponse</a> of a specified response type and status code on the given <a>RestApi</a>.</p>
    fn put_gateway_response(
        &self,
        input: PutGatewayResponseRequest,
    ) -> RusotoFuture<GatewayResponse, PutGatewayResponseError>;

    /// <p>Sets up a method's integration.</p>
    fn put_integration(
        &self,
        input: PutIntegrationRequest,
    ) -> RusotoFuture<Integration, PutIntegrationError>;

    /// <p>Represents a put integration.</p>
    fn put_integration_response(
        &self,
        input: PutIntegrationResponseRequest,
    ) -> RusotoFuture<IntegrationResponse, PutIntegrationResponseError>;

    /// <p>Add a method to an existing <a>Resource</a> resource.</p>
    fn put_method(&self, input: PutMethodRequest) -> RusotoFuture<Method, PutMethodError>;

    /// <p>Adds a <a>MethodResponse</a> to an existing <a>Method</a> resource.</p>
    fn put_method_response(
        &self,
        input: PutMethodResponseRequest,
    ) -> RusotoFuture<MethodResponse, PutMethodResponseError>;

    /// <p>A feature of the API Gateway control service for updating an existing API with an input of external API definitions. The update can take the form of merging the supplied definition into the existing API or overwriting the existing API.</p>
    fn put_rest_api(&self, input: PutRestApiRequest) -> RusotoFuture<RestApi, PutRestApiError>;

    /// <p>Adds or updates a tag on a given resource.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError>;

    /// <p><p>Simulate the execution of an <a>Authorizer</a> in your <a>RestApi</a> with headers, parameters, and an incoming request body.</p> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/use-custom-authorizer.html">Enable custom authorizers</a> </div></p>
    fn test_invoke_authorizer(
        &self,
        input: TestInvokeAuthorizerRequest,
    ) -> RusotoFuture<TestInvokeAuthorizerResponse, TestInvokeAuthorizerError>;

    /// <p>Simulate the execution of a <a>Method</a> in your <a>RestApi</a> with headers, parameters, and an incoming request body.</p>
    fn test_invoke_method(
        &self,
        input: TestInvokeMethodRequest,
    ) -> RusotoFuture<TestInvokeMethodResponse, TestInvokeMethodError>;

    /// <p>Removes a tag from a given resource.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError>;

    /// <p>Changes information about the current <a>Account</a> resource.</p>
    fn update_account(
        &self,
        input: UpdateAccountRequest,
    ) -> RusotoFuture<Account, UpdateAccountError>;

    /// <p>Changes information about an <a>ApiKey</a> resource.</p>
    fn update_api_key(&self, input: UpdateApiKeyRequest)
        -> RusotoFuture<ApiKey, UpdateApiKeyError>;

    /// <p><p>Updates an existing <a>Authorizer</a> resource.</p> <div class="seeAlso"><a href="https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-authorizer.html">AWS CLI</a></div></p>
    fn update_authorizer(
        &self,
        input: UpdateAuthorizerRequest,
    ) -> RusotoFuture<Authorizer, UpdateAuthorizerError>;

    /// <p>Changes information about the <a>BasePathMapping</a> resource.</p>
    fn update_base_path_mapping(
        &self,
        input: UpdateBasePathMappingRequest,
    ) -> RusotoFuture<BasePathMapping, UpdateBasePathMappingError>;

    /// <p>Changes information about an <a>ClientCertificate</a> resource.</p>
    fn update_client_certificate(
        &self,
        input: UpdateClientCertificateRequest,
    ) -> RusotoFuture<ClientCertificate, UpdateClientCertificateError>;

    /// <p>Changes information about a <a>Deployment</a> resource.</p>
    fn update_deployment(
        &self,
        input: UpdateDeploymentRequest,
    ) -> RusotoFuture<Deployment, UpdateDeploymentError>;

    fn update_documentation_part(
        &self,
        input: UpdateDocumentationPartRequest,
    ) -> RusotoFuture<DocumentationPart, UpdateDocumentationPartError>;

    fn update_documentation_version(
        &self,
        input: UpdateDocumentationVersionRequest,
    ) -> RusotoFuture<DocumentationVersion, UpdateDocumentationVersionError>;

    /// <p>Changes information about the <a>DomainName</a> resource.</p>
    fn update_domain_name(
        &self,
        input: UpdateDomainNameRequest,
    ) -> RusotoFuture<DomainName, UpdateDomainNameError>;

    /// <p>Updates a <a>GatewayResponse</a> of a specified response type on the given <a>RestApi</a>.</p>
    fn update_gateway_response(
        &self,
        input: UpdateGatewayResponseRequest,
    ) -> RusotoFuture<GatewayResponse, UpdateGatewayResponseError>;

    /// <p>Represents an update integration.</p>
    fn update_integration(
        &self,
        input: UpdateIntegrationRequest,
    ) -> RusotoFuture<Integration, UpdateIntegrationError>;

    /// <p>Represents an update integration response.</p>
    fn update_integration_response(
        &self,
        input: UpdateIntegrationResponseRequest,
    ) -> RusotoFuture<IntegrationResponse, UpdateIntegrationResponseError>;

    /// <p>Updates an existing <a>Method</a> resource.</p>
    fn update_method(&self, input: UpdateMethodRequest) -> RusotoFuture<Method, UpdateMethodError>;

    /// <p>Updates an existing <a>MethodResponse</a> resource.</p>
    fn update_method_response(
        &self,
        input: UpdateMethodResponseRequest,
    ) -> RusotoFuture<MethodResponse, UpdateMethodResponseError>;

    /// <p>Changes information about a model.</p>
    fn update_model(&self, input: UpdateModelRequest) -> RusotoFuture<Model, UpdateModelError>;

    /// <p>Updates a <a>RequestValidator</a> of a given <a>RestApi</a>.</p>
    fn update_request_validator(
        &self,
        input: UpdateRequestValidatorRequest,
    ) -> RusotoFuture<RequestValidator, UpdateRequestValidatorError>;

    /// <p>Changes information about a <a>Resource</a> resource.</p>
    fn update_resource(
        &self,
        input: UpdateResourceRequest,
    ) -> RusotoFuture<Resource, UpdateResourceError>;

    /// <p>Changes information about the specified API.</p>
    fn update_rest_api(
        &self,
        input: UpdateRestApiRequest,
    ) -> RusotoFuture<RestApi, UpdateRestApiError>;

    /// <p>Changes information about a <a>Stage</a> resource.</p>
    fn update_stage(&self, input: UpdateStageRequest) -> RusotoFuture<Stage, UpdateStageError>;

    /// <p>Grants a temporary extension to the remaining quota of a usage plan associated with a specified API key.</p>
    fn update_usage(&self, input: UpdateUsageRequest) -> RusotoFuture<Usage, UpdateUsageError>;

    /// <p>Updates a usage plan of a given plan Id.</p>
    fn update_usage_plan(
        &self,
        input: UpdateUsagePlanRequest,
    ) -> RusotoFuture<UsagePlan, UpdateUsagePlanError>;

    /// <p>Updates an existing <a>VpcLink</a> of a specified identifier.</p>
    fn update_vpc_link(
        &self,
        input: UpdateVpcLinkRequest,
    ) -> RusotoFuture<VpcLink, UpdateVpcLinkError>;
}
/// A client for the Amazon API Gateway API.
#[derive(Clone)]
pub struct ApiGatewayClient {
    client: Client,
    region: region::Region,
}

impl ApiGatewayClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ApiGatewayClient {
        ApiGatewayClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ApiGatewayClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        ApiGatewayClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl ApiGateway for ApiGatewayClient {
    /// <p><p>Create an <a>ApiKey</a> resource. </p> <div class="seeAlso"><a href="https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-api-key.html">AWS CLI</a></div></p>
    fn create_api_key(
        &self,
        input: CreateApiKeyRequest,
    ) -> RusotoFuture<ApiKey, CreateApiKeyError> {
        let request_uri = "/apikeys";

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<ApiKey, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateApiKeyError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Adds a new <a>Authorizer</a> resource to an existing <a>RestApi</a> resource.</p> <div class="seeAlso"><a href="https://docs.aws.amazon.com/cli/latest/reference/apigateway/create-authorizer.html">AWS CLI</a></div></p>
    fn create_authorizer(
        &self,
        input: CreateAuthorizerRequest,
    ) -> RusotoFuture<Authorizer, CreateAuthorizerError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/authorizers",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<Authorizer, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateAuthorizerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new <a>BasePathMapping</a> resource.</p>
    fn create_base_path_mapping(
        &self,
        input: CreateBasePathMappingRequest,
    ) -> RusotoFuture<BasePathMapping, CreateBasePathMappingError> {
        let request_uri = format!(
            "/domainnames/{domain_name}/basepathmappings",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<BasePathMapping, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateBasePathMappingError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a <a>Deployment</a> resource, which makes a specified <a>RestApi</a> callable over the internet.</p>
    fn create_deployment(
        &self,
        input: CreateDeploymentRequest,
    ) -> RusotoFuture<Deployment, CreateDeploymentError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/deployments",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<Deployment, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDeploymentError::from_response(response))),
                )
            }
        })
    }

    fn create_documentation_part(
        &self,
        input: CreateDocumentationPartRequest,
    ) -> RusotoFuture<DocumentationPart, CreateDocumentationPartError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/documentation/parts",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DocumentationPart, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateDocumentationPartError::from_response(response))
                }))
            }
        })
    }

    fn create_documentation_version(
        &self,
        input: CreateDocumentationVersionRequest,
    ) -> RusotoFuture<DocumentationVersion, CreateDocumentationVersionError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/documentation/versions",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DocumentationVersion, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateDocumentationVersionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a new domain name.</p>
    fn create_domain_name(
        &self,
        input: CreateDomainNameRequest,
    ) -> RusotoFuture<DomainName, CreateDomainNameError> {
        let request_uri = "/domainnames";

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DomainName, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDomainNameError::from_response(response))),
                )
            }
        })
    }

    /// <p>Adds a new <a>Model</a> resource to an existing <a>RestApi</a> resource.</p>
    fn create_model(&self, input: CreateModelRequest) -> RusotoFuture<Model, CreateModelError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/models",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<Model, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateModelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a <a>ReqeustValidator</a> of a given <a>RestApi</a>.</p>
    fn create_request_validator(
        &self,
        input: CreateRequestValidatorRequest,
    ) -> RusotoFuture<RequestValidator, CreateRequestValidatorError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/requestvalidators",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<RequestValidator, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateRequestValidatorError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a <a>Resource</a> resource.</p>
    fn create_resource(
        &self,
        input: CreateResourceRequest,
    ) -> RusotoFuture<Resource, CreateResourceError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{parent_id}",
            parent_id = input.parent_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<Resource, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new <a>RestApi</a> resource.</p>
    fn create_rest_api(
        &self,
        input: CreateRestApiRequest,
    ) -> RusotoFuture<RestApi, CreateRestApiError> {
        let request_uri = "/restapis";

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<RestApi, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateRestApiError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a new <a>Stage</a> resource that references a pre-existing <a>Deployment</a> for the API. </p>
    fn create_stage(&self, input: CreateStageRequest) -> RusotoFuture<Stage, CreateStageError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/stages",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<Stage, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateStageError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a usage plan with the throttle and quota limits, as well as the associated API stages, specified in the payload. </p>
    fn create_usage_plan(
        &self,
        input: CreateUsagePlanRequest,
    ) -> RusotoFuture<UsagePlan, CreateUsagePlanError> {
        let request_uri = "/usageplans";

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UsagePlan, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateUsagePlanError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a usage plan key for adding an existing API key to a usage plan.</p>
    fn create_usage_plan_key(
        &self,
        input: CreateUsagePlanKeyRequest,
    ) -> RusotoFuture<UsagePlanKey, CreateUsagePlanKeyError> {
        let request_uri = format!(
            "/usageplans/{usageplan_id}/keys",
            usageplan_id = input.usage_plan_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UsagePlanKey, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateUsagePlanKeyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a VPC link, under the caller's account in a selected region, in an asynchronous operation that typically takes 2-4 minutes to complete and become operational. The caller must have permissions to create and update VPC Endpoint services.</p>
    fn create_vpc_link(
        &self,
        input: CreateVpcLinkRequest,
    ) -> RusotoFuture<VpcLink, CreateVpcLinkError> {
        let request_uri = "/vpclinks";

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<VpcLink, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateVpcLinkError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the <a>ApiKey</a> resource.</p>
    fn delete_api_key(&self, input: DeleteApiKeyRequest) -> RusotoFuture<(), DeleteApiKeyError> {
        let request_uri = format!("/apikeys/{api_key}", api_key = input.api_key);

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteApiKeyError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Deletes an existing <a>Authorizer</a> resource.</p> <div class="seeAlso"><a href="https://docs.aws.amazon.com/cli/latest/reference/apigateway/delete-authorizer.html">AWS CLI</a></div></p>
    fn delete_authorizer(
        &self,
        input: DeleteAuthorizerRequest,
    ) -> RusotoFuture<(), DeleteAuthorizerError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/authorizers/{authorizer_id}",
            authorizer_id = input.authorizer_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteAuthorizerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the <a>BasePathMapping</a> resource.</p>
    fn delete_base_path_mapping(
        &self,
        input: DeleteBasePathMappingRequest,
    ) -> RusotoFuture<(), DeleteBasePathMappingError> {
        let request_uri = format!(
            "/domainnames/{domain_name}/basepathmappings/{base_path}",
            base_path = input.base_path,
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteBasePathMappingError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes the <a>ClientCertificate</a> resource.</p>
    fn delete_client_certificate(
        &self,
        input: DeleteClientCertificateRequest,
    ) -> RusotoFuture<(), DeleteClientCertificateError> {
        let request_uri = format!(
            "/clientcertificates/{clientcertificate_id}",
            clientcertificate_id = input.client_certificate_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteClientCertificateError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes a <a>Deployment</a> resource. Deleting a deployment will only succeed if there are no <a>Stage</a> resources associated with it.</p>
    fn delete_deployment(
        &self,
        input: DeleteDeploymentRequest,
    ) -> RusotoFuture<(), DeleteDeploymentError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/deployments/{deployment_id}",
            deployment_id = input.deployment_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDeploymentError::from_response(response))),
                )
            }
        })
    }

    fn delete_documentation_part(
        &self,
        input: DeleteDocumentationPartRequest,
    ) -> RusotoFuture<(), DeleteDocumentationPartError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/documentation/parts/{part_id}",
            part_id = input.documentation_part_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDocumentationPartError::from_response(response))
                }))
            }
        })
    }

    fn delete_documentation_version(
        &self,
        input: DeleteDocumentationVersionRequest,
    ) -> RusotoFuture<(), DeleteDocumentationVersionError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/documentation/versions/{doc_version}",
            doc_version = input.documentation_version,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDocumentationVersionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes the <a>DomainName</a> resource.</p>
    fn delete_domain_name(
        &self,
        input: DeleteDomainNameRequest,
    ) -> RusotoFuture<(), DeleteDomainNameError> {
        let request_uri = format!(
            "/domainnames/{domain_name}",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDomainNameError::from_response(response))),
                )
            }
        })
    }

    /// <p>Clears any customization of a <a>GatewayResponse</a> of a specified response type on the given <a>RestApi</a> and resets it with the default settings.</p>
    fn delete_gateway_response(
        &self,
        input: DeleteGatewayResponseRequest,
    ) -> RusotoFuture<(), DeleteGatewayResponseError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/gatewayresponses/{response_type}",
            response_type = input.response_type,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteGatewayResponseError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Represents a delete integration.</p>
    fn delete_integration(
        &self,
        input: DeleteIntegrationRequest,
    ) -> RusotoFuture<(), DeleteIntegrationError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/integration",
            http_method = input.http_method,
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteIntegrationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Represents a delete integration response.</p>
    fn delete_integration_response(
        &self,
        input: DeleteIntegrationResponseRequest,
    ) -> RusotoFuture<(), DeleteIntegrationResponseError> {
        let request_uri = format!("/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/integration/responses/{status_code}", http_method = input.http_method, resource_id = input.resource_id, restapi_id = input.rest_api_id, status_code = input.status_code);

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteIntegrationResponseError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes an existing <a>Method</a> resource.</p>
    fn delete_method(&self, input: DeleteMethodRequest) -> RusotoFuture<(), DeleteMethodError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}",
            http_method = input.http_method,
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteMethodError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an existing <a>MethodResponse</a> resource.</p>
    fn delete_method_response(
        &self,
        input: DeleteMethodResponseRequest,
    ) -> RusotoFuture<(), DeleteMethodResponseError> {
        let request_uri = format!("/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/responses/{status_code}", http_method = input.http_method, resource_id = input.resource_id, restapi_id = input.rest_api_id, status_code = input.status_code);

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteMethodResponseError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a model.</p>
    fn delete_model(&self, input: DeleteModelRequest) -> RusotoFuture<(), DeleteModelError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/models/{model_name}",
            model_name = input.model_name,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteModelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a <a>RequestValidator</a> of a given <a>RestApi</a>.</p>
    fn delete_request_validator(
        &self,
        input: DeleteRequestValidatorRequest,
    ) -> RusotoFuture<(), DeleteRequestValidatorError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/requestvalidators/{requestvalidator_id}",
            requestvalidator_id = input.request_validator_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteRequestValidatorError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes a <a>Resource</a> resource.</p>
    fn delete_resource(
        &self,
        input: DeleteResourceRequest,
    ) -> RusotoFuture<(), DeleteResourceError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}",
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified API.</p>
    fn delete_rest_api(&self, input: DeleteRestApiRequest) -> RusotoFuture<(), DeleteRestApiError> {
        let request_uri = format!("/restapis/{restapi_id}", restapi_id = input.rest_api_id);

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteRestApiError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a <a>Stage</a> resource.</p>
    fn delete_stage(&self, input: DeleteStageRequest) -> RusotoFuture<(), DeleteStageError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/stages/{stage_name}",
            restapi_id = input.rest_api_id,
            stage_name = input.stage_name
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteStageError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a usage plan of a given plan Id.</p>
    fn delete_usage_plan(
        &self,
        input: DeleteUsagePlanRequest,
    ) -> RusotoFuture<(), DeleteUsagePlanError> {
        let request_uri = format!(
            "/usageplans/{usageplan_id}",
            usageplan_id = input.usage_plan_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteUsagePlanError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a usage plan key and remove the underlying API key from the associated usage plan.</p>
    fn delete_usage_plan_key(
        &self,
        input: DeleteUsagePlanKeyRequest,
    ) -> RusotoFuture<(), DeleteUsagePlanKeyError> {
        let request_uri = format!(
            "/usageplans/{usageplan_id}/keys/{key_id}",
            key_id = input.key_id,
            usageplan_id = input.usage_plan_id
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteUsagePlanKeyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an existing <a>VpcLink</a> of a specified identifier.</p>
    fn delete_vpc_link(&self, input: DeleteVpcLinkRequest) -> RusotoFuture<(), DeleteVpcLinkError> {
        let request_uri = format!("/vpclinks/{vpclink_id}", vpclink_id = input.vpc_link_id);

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteVpcLinkError::from_response(response))),
                )
            }
        })
    }

    /// <p>Flushes all authorizer cache entries on a stage.</p>
    fn flush_stage_authorizers_cache(
        &self,
        input: FlushStageAuthorizersCacheRequest,
    ) -> RusotoFuture<(), FlushStageAuthorizersCacheError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/stages/{stage_name}/cache/authorizers",
            restapi_id = input.rest_api_id,
            stage_name = input.stage_name
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(FlushStageAuthorizersCacheError::from_response(response))
                }))
            }
        })
    }

    /// <p>Flushes a stage's cache.</p>
    fn flush_stage_cache(
        &self,
        input: FlushStageCacheRequest,
    ) -> RusotoFuture<(), FlushStageCacheError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/stages/{stage_name}/cache/data",
            restapi_id = input.rest_api_id,
            stage_name = input.stage_name
        );

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(FlushStageCacheError::from_response(response))),
                )
            }
        })
    }

    /// <p>Generates a <a>ClientCertificate</a> resource.</p>
    fn generate_client_certificate(
        &self,
        input: GenerateClientCertificateRequest,
    ) -> RusotoFuture<ClientCertificate, GenerateClientCertificateError> {
        let request_uri = "/clientcertificates";

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ClientCertificate, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GenerateClientCertificateError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets information about the current <a>Account</a> resource.</p>
    fn get_account(&self) -> RusotoFuture<Account, GetAccountError> {
        let request_uri = "/account";

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<Account, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAccountError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets information about the current <a>ApiKey</a> resource.</p>
    fn get_api_key(&self, input: GetApiKeyRequest) -> RusotoFuture<ApiKey, GetApiKeyError> {
        let request_uri = format!("/apikeys/{api_key}", api_key = input.api_key);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.include_value {
            params.put("includeValue", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<ApiKey, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetApiKeyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets information about the current <a>ApiKeys</a> resource.</p>
    fn get_api_keys(&self, input: GetApiKeysRequest) -> RusotoFuture<ApiKeys, GetApiKeysError> {
        let request_uri = "/apikeys";

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.customer_id {
            params.put("customerId", x);
        }
        if let Some(ref x) = input.include_values {
            params.put("includeValues", x);
        }
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.name_query {
            params.put("name", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<ApiKeys, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetApiKeysError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Describe an existing <a>Authorizer</a> resource.</p> <div class="seeAlso"><a href="https://docs.aws.amazon.com/cli/latest/reference/apigateway/get-authorizer.html">AWS CLI</a></div></p>
    fn get_authorizer(
        &self,
        input: GetAuthorizerRequest,
    ) -> RusotoFuture<Authorizer, GetAuthorizerError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/authorizers/{authorizer_id}",
            authorizer_id = input.authorizer_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<Authorizer, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAuthorizerError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Describe an existing <a>Authorizers</a> resource.</p> <div class="seeAlso"><a href="https://docs.aws.amazon.com/cli/latest/reference/apigateway/get-authorizers.html">AWS CLI</a></div></p>
    fn get_authorizers(
        &self,
        input: GetAuthorizersRequest,
    ) -> RusotoFuture<Authorizers, GetAuthorizersError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/authorizers",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<Authorizers, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAuthorizersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describe a <a>BasePathMapping</a> resource.</p>
    fn get_base_path_mapping(
        &self,
        input: GetBasePathMappingRequest,
    ) -> RusotoFuture<BasePathMapping, GetBasePathMappingError> {
        let request_uri = format!(
            "/domainnames/{domain_name}/basepathmappings/{base_path}",
            base_path = input.base_path,
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<BasePathMapping, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetBasePathMappingError::from_response(response))),
                )
            }
        })
    }

    /// <p>Represents a collection of <a>BasePathMapping</a> resources.</p>
    fn get_base_path_mappings(
        &self,
        input: GetBasePathMappingsRequest,
    ) -> RusotoFuture<BasePathMappings, GetBasePathMappingsError> {
        let request_uri = format!(
            "/domainnames/{domain_name}/basepathmappings",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<BasePathMappings, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetBasePathMappingsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets information about the current <a>ClientCertificate</a> resource.</p>
    fn get_client_certificate(
        &self,
        input: GetClientCertificateRequest,
    ) -> RusotoFuture<ClientCertificate, GetClientCertificateError> {
        let request_uri = format!(
            "/clientcertificates/{clientcertificate_id}",
            clientcertificate_id = input.client_certificate_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ClientCertificate, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetClientCertificateError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets a collection of <a>ClientCertificate</a> resources.</p>
    fn get_client_certificates(
        &self,
        input: GetClientCertificatesRequest,
    ) -> RusotoFuture<ClientCertificates, GetClientCertificatesError> {
        let request_uri = "/clientcertificates";

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ClientCertificates, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetClientCertificatesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets information about a <a>Deployment</a> resource.</p>
    fn get_deployment(
        &self,
        input: GetDeploymentRequest,
    ) -> RusotoFuture<Deployment, GetDeploymentError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/deployments/{deployment_id}",
            deployment_id = input.deployment_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.embed {
            for item in x.iter() {
                params.put("embed", item);
            }
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<Deployment, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDeploymentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets information about a <a>Deployments</a> collection.</p>
    fn get_deployments(
        &self,
        input: GetDeploymentsRequest,
    ) -> RusotoFuture<Deployments, GetDeploymentsError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/deployments",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<Deployments, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDeploymentsError::from_response(response))),
                )
            }
        })
    }

    fn get_documentation_part(
        &self,
        input: GetDocumentationPartRequest,
    ) -> RusotoFuture<DocumentationPart, GetDocumentationPartError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/documentation/parts/{part_id}",
            part_id = input.documentation_part_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DocumentationPart, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetDocumentationPartError::from_response(response))
                    }),
                )
            }
        })
    }

    fn get_documentation_parts(
        &self,
        input: GetDocumentationPartsRequest,
    ) -> RusotoFuture<DocumentationParts, GetDocumentationPartsError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/documentation/parts",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.location_status {
            params.put("locationStatus", x);
        }
        if let Some(ref x) = input.name_query {
            params.put("name", x);
        }
        if let Some(ref x) = input.path {
            params.put("path", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        if let Some(ref x) = input.type_ {
            params.put("type", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DocumentationParts, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetDocumentationPartsError::from_response(response))
                    }),
                )
            }
        })
    }

    fn get_documentation_version(
        &self,
        input: GetDocumentationVersionRequest,
    ) -> RusotoFuture<DocumentationVersion, GetDocumentationVersionError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/documentation/versions/{doc_version}",
            doc_version = input.documentation_version,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DocumentationVersion, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDocumentationVersionError::from_response(response))
                }))
            }
        })
    }

    fn get_documentation_versions(
        &self,
        input: GetDocumentationVersionsRequest,
    ) -> RusotoFuture<DocumentationVersions, GetDocumentationVersionsError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/documentation/versions",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DocumentationVersions, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetDocumentationVersionsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Represents a domain name that is contained in a simpler, more intuitive URL that can be called.</p>
    fn get_domain_name(
        &self,
        input: GetDomainNameRequest,
    ) -> RusotoFuture<DomainName, GetDomainNameError> {
        let request_uri = format!(
            "/domainnames/{domain_name}",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DomainName, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDomainNameError::from_response(response))),
                )
            }
        })
    }

    /// <p>Represents a collection of <a>DomainName</a> resources.</p>
    fn get_domain_names(
        &self,
        input: GetDomainNamesRequest,
    ) -> RusotoFuture<DomainNames, GetDomainNamesError> {
        let request_uri = "/domainnames";

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DomainNames, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDomainNamesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Exports a deployed version of a <a>RestApi</a> in a specified format.</p>
    fn get_export(&self, input: GetExportRequest) -> RusotoFuture<ExportResponse, GetExportError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/stages/{stage_name}/exports/{export_type}",
            export_type = input.export_type,
            restapi_id = input.rest_api_id,
            stage_name = input.stage_name
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        if let Some(ref accepts) = input.accepts {
            request.add_header("Accept", &accepts.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.parameters {
            for (key, val) in x.iter() {
                params.put(key, val);
            }
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let mut result = ExportResponse::default();
                    result.body = Some(response.body);

                    if let Some(content_disposition) = response.headers.get("Content-Disposition") {
                        let value = content_disposition.to_owned();
                        result.content_disposition = Some(value)
                    };
                    if let Some(content_type) = response.headers.get("Content-Type") {
                        let value = content_type.to_owned();
                        result.content_type = Some(value)
                    };

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetExportError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a <a>GatewayResponse</a> of a specified response type on the given <a>RestApi</a>.</p>
    fn get_gateway_response(
        &self,
        input: GetGatewayResponseRequest,
    ) -> RusotoFuture<GatewayResponse, GetGatewayResponseError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/gatewayresponses/{response_type}",
            response_type = input.response_type,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GatewayResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetGatewayResponseError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the <a>GatewayResponses</a> collection on the given <a>RestApi</a>. If an API developer has not added any definitions for gateway responses, the result will be the API Gateway-generated default <a>GatewayResponses</a> collection for the supported response types.</p>
    fn get_gateway_responses(
        &self,
        input: GetGatewayResponsesRequest,
    ) -> RusotoFuture<GatewayResponses, GetGatewayResponsesError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/gatewayresponses",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GatewayResponses, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetGatewayResponsesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Get the integration settings.</p>
    fn get_integration(
        &self,
        input: GetIntegrationRequest,
    ) -> RusotoFuture<Integration, GetIntegrationError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/integration",
            http_method = input.http_method,
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<Integration, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetIntegrationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Represents a get integration response.</p>
    fn get_integration_response(
        &self,
        input: GetIntegrationResponseRequest,
    ) -> RusotoFuture<IntegrationResponse, GetIntegrationResponseError> {
        let request_uri = format!("/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/integration/responses/{status_code}", http_method = input.http_method, resource_id = input.resource_id, restapi_id = input.rest_api_id, status_code = input.status_code);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<IntegrationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetIntegrationResponseError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Describe an existing <a>Method</a> resource.</p>
    fn get_method(&self, input: GetMethodRequest) -> RusotoFuture<Method, GetMethodError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}",
            http_method = input.http_method,
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<Method, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetMethodError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes a <a>MethodResponse</a> resource.</p>
    fn get_method_response(
        &self,
        input: GetMethodResponseRequest,
    ) -> RusotoFuture<MethodResponse, GetMethodResponseError> {
        let request_uri = format!("/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/responses/{status_code}", http_method = input.http_method, resource_id = input.resource_id, restapi_id = input.rest_api_id, status_code = input.status_code);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<MethodResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetMethodResponseError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes an existing model defined for a <a>RestApi</a> resource.</p>
    fn get_model(&self, input: GetModelRequest) -> RusotoFuture<Model, GetModelError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/models/{model_name}",
            model_name = input.model_name,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.flatten {
            params.put("flatten", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<Model, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetModelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Generates a sample mapping template that can be used to transform a payload into the structure of a model.</p>
    fn get_model_template(
        &self,
        input: GetModelTemplateRequest,
    ) -> RusotoFuture<Template, GetModelTemplateError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/models/{model_name}/default_template",
            model_name = input.model_name,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<Template, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetModelTemplateError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes existing <a>Models</a> defined for a <a>RestApi</a> resource.</p>
    fn get_models(&self, input: GetModelsRequest) -> RusotoFuture<Models, GetModelsError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/models",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<Models, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetModelsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a <a>RequestValidator</a> of a given <a>RestApi</a>.</p>
    fn get_request_validator(
        &self,
        input: GetRequestValidatorRequest,
    ) -> RusotoFuture<RequestValidator, GetRequestValidatorError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/requestvalidators/{requestvalidator_id}",
            requestvalidator_id = input.request_validator_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<RequestValidator, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetRequestValidatorError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets the <a>RequestValidators</a> collection of a given <a>RestApi</a>.</p>
    fn get_request_validators(
        &self,
        input: GetRequestValidatorsRequest,
    ) -> RusotoFuture<RequestValidators, GetRequestValidatorsError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/requestvalidators",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<RequestValidators, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetRequestValidatorsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists information about a resource.</p>
    fn get_resource(&self, input: GetResourceRequest) -> RusotoFuture<Resource, GetResourceError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}",
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.embed {
            for item in x.iter() {
                params.put("embed", item);
            }
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<Resource, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists information about a collection of <a>Resource</a> resources.</p>
    fn get_resources(
        &self,
        input: GetResourcesRequest,
    ) -> RusotoFuture<Resources, GetResourcesError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.embed {
            for item in x.iter() {
                params.put("embed", item);
            }
        }
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<Resources, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetResourcesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the <a>RestApi</a> resource in the collection.</p>
    fn get_rest_api(&self, input: GetRestApiRequest) -> RusotoFuture<RestApi, GetRestApiError> {
        let request_uri = format!("/restapis/{restapi_id}", restapi_id = input.rest_api_id);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<RestApi, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetRestApiError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the <a>RestApis</a> resources for your collection.</p>
    fn get_rest_apis(&self, input: GetRestApisRequest) -> RusotoFuture<RestApis, GetRestApisError> {
        let request_uri = "/restapis";

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<RestApis, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetRestApisError::from_response(response))),
                )
            }
        })
    }

    /// <p>Generates a client SDK for a <a>RestApi</a> and <a>Stage</a>.</p>
    fn get_sdk(&self, input: GetSdkRequest) -> RusotoFuture<SdkResponse, GetSdkError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/stages/{stage_name}/sdks/{sdk_type}",
            restapi_id = input.rest_api_id,
            sdk_type = input.sdk_type,
            stage_name = input.stage_name
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.parameters {
            for (key, val) in x.iter() {
                params.put(key, val);
            }
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let mut result = SdkResponse::default();
                    result.body = Some(response.body);

                    if let Some(content_disposition) = response.headers.get("Content-Disposition") {
                        let value = content_disposition.to_owned();
                        result.content_disposition = Some(value)
                    };
                    if let Some(content_type) = response.headers.get("Content-Type") {
                        let value = content_type.to_owned();
                        result.content_type = Some(value)
                    };

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetSdkError::from_response(response))),
                )
            }
        })
    }

    fn get_sdk_type(&self, input: GetSdkTypeRequest) -> RusotoFuture<SdkType, GetSdkTypeError> {
        let request_uri = format!("/sdktypes/{sdktype_id}", sdktype_id = input.id);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<SdkType, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetSdkTypeError::from_response(response))),
                )
            }
        })
    }

    fn get_sdk_types(&self, input: GetSdkTypesRequest) -> RusotoFuture<SdkTypes, GetSdkTypesError> {
        let request_uri = "/sdktypes";

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<SdkTypes, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetSdkTypesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets information about a <a>Stage</a> resource.</p>
    fn get_stage(&self, input: GetStageRequest) -> RusotoFuture<Stage, GetStageError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/stages/{stage_name}",
            restapi_id = input.rest_api_id,
            stage_name = input.stage_name
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<Stage, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetStageError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets information about one or more <a>Stage</a> resources.</p>
    fn get_stages(&self, input: GetStagesRequest) -> RusotoFuture<Stages, GetStagesError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/stages",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.deployment_id {
            params.put("deploymentId", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<Stages, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetStagesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the <a>Tags</a> collection for a given resource.</p>
    fn get_tags(&self, input: GetTagsRequest) -> RusotoFuture<Tags, GetTagsError> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<Tags, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetTagsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the usage data of a usage plan in a specified time interval.</p>
    fn get_usage(&self, input: GetUsageRequest) -> RusotoFuture<Usage, GetUsageError> {
        let request_uri = format!(
            "/usageplans/{usageplan_id}/usage",
            usageplan_id = input.usage_plan_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("endDate", &input.end_date);
        if let Some(ref x) = input.key_id {
            params.put("keyId", x);
        }
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        params.put("startDate", &input.start_date);
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<Usage, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetUsageError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a usage plan of a given plan identifier.</p>
    fn get_usage_plan(
        &self,
        input: GetUsagePlanRequest,
    ) -> RusotoFuture<UsagePlan, GetUsagePlanError> {
        let request_uri = format!(
            "/usageplans/{usageplan_id}",
            usageplan_id = input.usage_plan_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UsagePlan, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetUsagePlanError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a usage plan key of a given key identifier.</p>
    fn get_usage_plan_key(
        &self,
        input: GetUsagePlanKeyRequest,
    ) -> RusotoFuture<UsagePlanKey, GetUsagePlanKeyError> {
        let request_uri = format!(
            "/usageplans/{usageplan_id}/keys/{key_id}",
            key_id = input.key_id,
            usageplan_id = input.usage_plan_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UsagePlanKey, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetUsagePlanKeyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets all the usage plan keys representing the API keys added to a specified usage plan.</p>
    fn get_usage_plan_keys(
        &self,
        input: GetUsagePlanKeysRequest,
    ) -> RusotoFuture<UsagePlanKeys, GetUsagePlanKeysError> {
        let request_uri = format!(
            "/usageplans/{usageplan_id}/keys",
            usageplan_id = input.usage_plan_id
        );

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.name_query {
            params.put("name", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UsagePlanKeys, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetUsagePlanKeysError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets all the usage plans of the caller's account.</p>
    fn get_usage_plans(
        &self,
        input: GetUsagePlansRequest,
    ) -> RusotoFuture<UsagePlans, GetUsagePlansError> {
        let request_uri = "/usageplans";

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.key_id {
            params.put("keyId", x);
        }
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UsagePlans, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetUsagePlansError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a specified VPC link under the caller's account in a region.</p>
    fn get_vpc_link(&self, input: GetVpcLinkRequest) -> RusotoFuture<VpcLink, GetVpcLinkError> {
        let request_uri = format!("/vpclinks/{vpclink_id}", vpclink_id = input.vpc_link_id);

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<VpcLink, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetVpcLinkError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the <a>VpcLinks</a> collection under the caller's account in a selected region.</p>
    fn get_vpc_links(&self, input: GetVpcLinksRequest) -> RusotoFuture<VpcLinks, GetVpcLinksError> {
        let request_uri = "/vpclinks";

        let mut request = SignedRequest::new("GET", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.limit {
            params.put("limit", x);
        }
        if let Some(ref x) = input.position {
            params.put("position", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<VpcLinks, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetVpcLinksError::from_response(response))),
                )
            }
        })
    }

    /// <p>Import API keys from an external source, such as a CSV-formatted file.</p>
    fn import_api_keys(
        &self,
        input: ImportApiKeysRequest,
    ) -> RusotoFuture<ApiKeyIds, ImportApiKeysError> {
        let request_uri = "/apikeys";

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(input.body.to_owned());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.fail_on_warnings {
            params.put("failonwarnings", x);
        }
        params.put("format", &input.format);
        params.put("mode", "import");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ApiKeyIds, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ImportApiKeysError::from_response(response))),
                )
            }
        })
    }

    fn import_documentation_parts(
        &self,
        input: ImportDocumentationPartsRequest,
    ) -> RusotoFuture<DocumentationPartIds, ImportDocumentationPartsError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/documentation/parts",
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PUT", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(input.body.to_owned());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.fail_on_warnings {
            params.put("failonwarnings", x);
        }
        if let Some(ref x) = input.mode {
            params.put("mode", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DocumentationPartIds, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ImportDocumentationPartsError::from_response(response))
                }))
            }
        })
    }

    /// <p>A feature of the API Gateway control service for creating a new API from an external API definition file.</p>
    fn import_rest_api(
        &self,
        input: ImportRestApiRequest,
    ) -> RusotoFuture<RestApi, ImportRestApiError> {
        let request_uri = "/restapis";

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(input.body.to_owned());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.fail_on_warnings {
            params.put("failonwarnings", x);
        }
        if let Some(ref x) = input.parameters {
            for (key, val) in x.iter() {
                params.put(key, val);
            }
        }
        params.put("mode", "import");
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<RestApi, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ImportRestApiError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a customization of a <a>GatewayResponse</a> of a specified response type and status code on the given <a>RestApi</a>.</p>
    fn put_gateway_response(
        &self,
        input: PutGatewayResponseRequest,
    ) -> RusotoFuture<GatewayResponse, PutGatewayResponseError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/gatewayresponses/{response_type}",
            response_type = input.response_type,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PUT", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GatewayResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutGatewayResponseError::from_response(response))),
                )
            }
        })
    }

    /// <p>Sets up a method's integration.</p>
    fn put_integration(
        &self,
        input: PutIntegrationRequest,
    ) -> RusotoFuture<Integration, PutIntegrationError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/integration",
            http_method = input.http_method,
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PUT", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<Integration, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutIntegrationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Represents a put integration.</p>
    fn put_integration_response(
        &self,
        input: PutIntegrationResponseRequest,
    ) -> RusotoFuture<IntegrationResponse, PutIntegrationResponseError> {
        let request_uri = format!("/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/integration/responses/{status_code}", http_method = input.http_method, resource_id = input.resource_id, restapi_id = input.rest_api_id, status_code = input.status_code);

        let mut request = SignedRequest::new("PUT", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<IntegrationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutIntegrationResponseError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Add a method to an existing <a>Resource</a> resource.</p>
    fn put_method(&self, input: PutMethodRequest) -> RusotoFuture<Method, PutMethodError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}",
            http_method = input.http_method,
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PUT", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<Method, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutMethodError::from_response(response))),
                )
            }
        })
    }

    /// <p>Adds a <a>MethodResponse</a> to an existing <a>Method</a> resource.</p>
    fn put_method_response(
        &self,
        input: PutMethodResponseRequest,
    ) -> RusotoFuture<MethodResponse, PutMethodResponseError> {
        let request_uri = format!("/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/responses/{status_code}", http_method = input.http_method, resource_id = input.resource_id, restapi_id = input.rest_api_id, status_code = input.status_code);

        let mut request = SignedRequest::new("PUT", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<MethodResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutMethodResponseError::from_response(response))),
                )
            }
        })
    }

    /// <p>A feature of the API Gateway control service for updating an existing API with an input of external API definitions. The update can take the form of merging the supplied definition into the existing API or overwriting the existing API.</p>
    fn put_rest_api(&self, input: PutRestApiRequest) -> RusotoFuture<RestApi, PutRestApiError> {
        let request_uri = format!("/restapis/{restapi_id}", restapi_id = input.rest_api_id);

        let mut request = SignedRequest::new("PUT", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(input.body.to_owned());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.fail_on_warnings {
            params.put("failonwarnings", x);
        }
        if let Some(ref x) = input.mode {
            params.put("mode", x);
        }
        if let Some(ref x) = input.parameters {
            for (key, val) in x.iter() {
                params.put(key, val);
            }
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<RestApi, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutRestApiError::from_response(response))),
                )
            }
        })
    }

    /// <p>Adds or updates a tag on a given resource.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("PUT", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
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

    /// <p><p>Simulate the execution of an <a>Authorizer</a> in your <a>RestApi</a> with headers, parameters, and an incoming request body.</p> <div class="seeAlso"> <a href="https://docs.aws.amazon.com/apigateway/latest/developerguide/use-custom-authorizer.html">Enable custom authorizers</a> </div></p>
    fn test_invoke_authorizer(
        &self,
        input: TestInvokeAuthorizerRequest,
    ) -> RusotoFuture<TestInvokeAuthorizerResponse, TestInvokeAuthorizerError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/authorizers/{authorizer_id}",
            authorizer_id = input.authorizer_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<TestInvokeAuthorizerResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(TestInvokeAuthorizerError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Simulate the execution of a <a>Method</a> in your <a>RestApi</a> with headers, parameters, and an incoming request body.</p>
    fn test_invoke_method(
        &self,
        input: TestInvokeMethodRequest,
    ) -> RusotoFuture<TestInvokeMethodResponse, TestInvokeMethodError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}",
            http_method = input.http_method,
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("POST", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<TestInvokeMethodResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TestInvokeMethodError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes a tag from a given resource.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
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

    /// <p>Changes information about the current <a>Account</a> resource.</p>
    fn update_account(
        &self,
        input: UpdateAccountRequest,
    ) -> RusotoFuture<Account, UpdateAccountError> {
        let request_uri = "/account";

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<Account, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateAccountError::from_response(response))),
                )
            }
        })
    }

    /// <p>Changes information about an <a>ApiKey</a> resource.</p>
    fn update_api_key(
        &self,
        input: UpdateApiKeyRequest,
    ) -> RusotoFuture<ApiKey, UpdateApiKeyError> {
        let request_uri = format!("/apikeys/{api_key}", api_key = input.api_key);

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<ApiKey, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateApiKeyError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Updates an existing <a>Authorizer</a> resource.</p> <div class="seeAlso"><a href="https://docs.aws.amazon.com/cli/latest/reference/apigateway/update-authorizer.html">AWS CLI</a></div></p>
    fn update_authorizer(
        &self,
        input: UpdateAuthorizerRequest,
    ) -> RusotoFuture<Authorizer, UpdateAuthorizerError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/authorizers/{authorizer_id}",
            authorizer_id = input.authorizer_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<Authorizer, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateAuthorizerError::from_response(response))),
                )
            }
        })
    }

    /// <p>Changes information about the <a>BasePathMapping</a> resource.</p>
    fn update_base_path_mapping(
        &self,
        input: UpdateBasePathMappingRequest,
    ) -> RusotoFuture<BasePathMapping, UpdateBasePathMappingError> {
        let request_uri = format!(
            "/domainnames/{domain_name}/basepathmappings/{base_path}",
            base_path = input.base_path,
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<BasePathMapping, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateBasePathMappingError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Changes information about an <a>ClientCertificate</a> resource.</p>
    fn update_client_certificate(
        &self,
        input: UpdateClientCertificateRequest,
    ) -> RusotoFuture<ClientCertificate, UpdateClientCertificateError> {
        let request_uri = format!(
            "/clientcertificates/{clientcertificate_id}",
            clientcertificate_id = input.client_certificate_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ClientCertificate, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateClientCertificateError::from_response(response))
                }))
            }
        })
    }

    /// <p>Changes information about a <a>Deployment</a> resource.</p>
    fn update_deployment(
        &self,
        input: UpdateDeploymentRequest,
    ) -> RusotoFuture<Deployment, UpdateDeploymentError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/deployments/{deployment_id}",
            deployment_id = input.deployment_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<Deployment, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateDeploymentError::from_response(response))),
                )
            }
        })
    }

    fn update_documentation_part(
        &self,
        input: UpdateDocumentationPartRequest,
    ) -> RusotoFuture<DocumentationPart, UpdateDocumentationPartError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/documentation/parts/{part_id}",
            part_id = input.documentation_part_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DocumentationPart, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDocumentationPartError::from_response(response))
                }))
            }
        })
    }

    fn update_documentation_version(
        &self,
        input: UpdateDocumentationVersionRequest,
    ) -> RusotoFuture<DocumentationVersion, UpdateDocumentationVersionError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/documentation/versions/{doc_version}",
            doc_version = input.documentation_version,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DocumentationVersion, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDocumentationVersionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Changes information about the <a>DomainName</a> resource.</p>
    fn update_domain_name(
        &self,
        input: UpdateDomainNameRequest,
    ) -> RusotoFuture<DomainName, UpdateDomainNameError> {
        let request_uri = format!(
            "/domainnames/{domain_name}",
            domain_name = input.domain_name
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DomainName, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateDomainNameError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a <a>GatewayResponse</a> of a specified response type on the given <a>RestApi</a>.</p>
    fn update_gateway_response(
        &self,
        input: UpdateGatewayResponseRequest,
    ) -> RusotoFuture<GatewayResponse, UpdateGatewayResponseError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/gatewayresponses/{response_type}",
            response_type = input.response_type,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GatewayResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateGatewayResponseError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Represents an update integration.</p>
    fn update_integration(
        &self,
        input: UpdateIntegrationRequest,
    ) -> RusotoFuture<Integration, UpdateIntegrationError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/integration",
            http_method = input.http_method,
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<Integration, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateIntegrationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Represents an update integration response.</p>
    fn update_integration_response(
        &self,
        input: UpdateIntegrationResponseRequest,
    ) -> RusotoFuture<IntegrationResponse, UpdateIntegrationResponseError> {
        let request_uri = format!("/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/integration/responses/{status_code}", http_method = input.http_method, resource_id = input.resource_id, restapi_id = input.rest_api_id, status_code = input.status_code);

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<IntegrationResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateIntegrationResponseError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates an existing <a>Method</a> resource.</p>
    fn update_method(&self, input: UpdateMethodRequest) -> RusotoFuture<Method, UpdateMethodError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}",
            http_method = input.http_method,
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<Method, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateMethodError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates an existing <a>MethodResponse</a> resource.</p>
    fn update_method_response(
        &self,
        input: UpdateMethodResponseRequest,
    ) -> RusotoFuture<MethodResponse, UpdateMethodResponseError> {
        let request_uri = format!("/restapis/{restapi_id}/resources/{resource_id}/methods/{http_method}/responses/{status_code}", http_method = input.http_method, resource_id = input.resource_id, restapi_id = input.rest_api_id, status_code = input.status_code);

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<MethodResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateMethodResponseError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Changes information about a model.</p>
    fn update_model(&self, input: UpdateModelRequest) -> RusotoFuture<Model, UpdateModelError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/models/{model_name}",
            model_name = input.model_name,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<Model, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateModelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a <a>RequestValidator</a> of a given <a>RestApi</a>.</p>
    fn update_request_validator(
        &self,
        input: UpdateRequestValidatorRequest,
    ) -> RusotoFuture<RequestValidator, UpdateRequestValidatorError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/requestvalidators/{requestvalidator_id}",
            requestvalidator_id = input.request_validator_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<RequestValidator, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateRequestValidatorError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Changes information about a <a>Resource</a> resource.</p>
    fn update_resource(
        &self,
        input: UpdateResourceRequest,
    ) -> RusotoFuture<Resource, UpdateResourceError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/resources/{resource_id}",
            resource_id = input.resource_id,
            restapi_id = input.rest_api_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<Resource, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Changes information about the specified API.</p>
    fn update_rest_api(
        &self,
        input: UpdateRestApiRequest,
    ) -> RusotoFuture<RestApi, UpdateRestApiError> {
        let request_uri = format!("/restapis/{restapi_id}", restapi_id = input.rest_api_id);

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<RestApi, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateRestApiError::from_response(response))),
                )
            }
        })
    }

    /// <p>Changes information about a <a>Stage</a> resource.</p>
    fn update_stage(&self, input: UpdateStageRequest) -> RusotoFuture<Stage, UpdateStageError> {
        let request_uri = format!(
            "/restapis/{restapi_id}/stages/{stage_name}",
            restapi_id = input.rest_api_id,
            stage_name = input.stage_name
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<Stage, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateStageError::from_response(response))),
                )
            }
        })
    }

    /// <p>Grants a temporary extension to the remaining quota of a usage plan associated with a specified API key.</p>
    fn update_usage(&self, input: UpdateUsageRequest) -> RusotoFuture<Usage, UpdateUsageError> {
        let request_uri = format!(
            "/usageplans/{usageplan_id}/keys/{key_id}/usage",
            key_id = input.key_id,
            usageplan_id = input.usage_plan_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<Usage, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateUsageError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates a usage plan of a given plan Id.</p>
    fn update_usage_plan(
        &self,
        input: UpdateUsagePlanRequest,
    ) -> RusotoFuture<UsagePlan, UpdateUsagePlanError> {
        let request_uri = format!(
            "/usageplans/{usageplan_id}",
            usageplan_id = input.usage_plan_id
        );

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UsagePlan, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateUsagePlanError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates an existing <a>VpcLink</a> of a specified identifier.</p>
    fn update_vpc_link(
        &self,
        input: UpdateVpcLinkRequest,
    ) -> RusotoFuture<VpcLink, UpdateVpcLinkError> {
        let request_uri = format!("/vpclinks/{vpclink_id}", vpclink_id = input.vpc_link_id);

        let mut request = SignedRequest::new("PATCH", "apigateway", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result =
                        proto::json::ResponsePayload::new(&response).deserialize::<VpcLink, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateVpcLinkError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
