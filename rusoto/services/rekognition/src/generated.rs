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
/// <p>Structure containing the estimated age range, in years, for a face.</p> <p>Amazon Rekognition estimates an age range for faces detected in the input image. Estimated age ranges can overlap. A face of a 5-year-old might have an estimated range of 4-6, while the face of a 6-year-old might have an estimated range of 4-8.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AgeRange {
    /// <p>The highest estimated age.</p>
    #[serde(rename = "High")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high: Option<i64>,
    /// <p>The lowest estimated age.</p>
    #[serde(rename = "Low")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low: Option<i64>,
}

/// <p>Indicates whether or not the face has a beard, and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Beard {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Boolean value that indicates whether the face has beard or not.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

/// <p><p>Identifies the bounding box around the label, face, or text. The <code>left</code> (x-coordinate) and <code>top</code> (y-coordinate) are coordinates representing the top and left sides of the bounding box. Note that the upper-left corner of the image is the origin (0,0). </p> <p>The <code>top</code> and <code>left</code> values returned are ratios of the overall image size. For example, if the input image is 700x200 pixels, and the top-left coordinate of the bounding box is 350x50 pixels, the API returns a <code>left</code> value of 0.5 (350/700) and a <code>top</code> value of 0.25 (50/200).</p> <p>The <code>width</code> and <code>height</code> values represent the dimensions of the bounding box as a ratio of the overall image dimension. For example, if the input image is 700x200 pixels, and the bounding box width is 70 pixels, the width returned is 0.1. </p> <note> <p> The bounding box coordinates can have negative values. For example, if Amazon Rekognition is able to detect a face that is at the image edge and is only partially visible, the service can return coordinates that are outside the image bounds and, depending on the image edge, you might get negative values or values greater than 1 for the <code>left</code> or <code>top</code> values. </p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BoundingBox {
    /// <p>Height of the bounding box as a ratio of the overall image height.</p>
    #[serde(rename = "Height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f32>,
    /// <p>Left coordinate of the bounding box as a ratio of overall image width.</p>
    #[serde(rename = "Left")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left: Option<f32>,
    /// <p>Top coordinate of the bounding box as a ratio of overall image height.</p>
    #[serde(rename = "Top")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top: Option<f32>,
    /// <p>Width of the bounding box as a ratio of the overall image width.</p>
    #[serde(rename = "Width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f32>,
}

/// <p>Provides information about a celebrity recognized by the <a>RecognizeCelebrities</a> operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Celebrity {
    /// <p>Provides information about the celebrity's face, such as its location on the image.</p>
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<ComparedFace>,
    /// <p>A unique identifier for the celebrity. </p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The confidence, in percentage, that Amazon Rekognition has that the recognized face is the celebrity.</p>
    #[serde(rename = "MatchConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_confidence: Option<f32>,
    /// <p>The name of the celebrity.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An array of URLs pointing to additional information about the celebrity. If there is no additional information about the celebrity, this list is empty.</p>
    #[serde(rename = "Urls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

/// <p>Information about a recognized celebrity.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CelebrityDetail {
    /// <p>Bounding box around the body of a celebrity.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>The confidence, in percentage, that Amazon Rekognition has that the recognized face is the celebrity. </p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Face details for the recognized celebrity.</p>
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<FaceDetail>,
    /// <p>The unique identifier for the celebrity. </p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the celebrity.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An array of URLs pointing to additional celebrity information. </p>
    #[serde(rename = "Urls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

/// <p>Information about a detected celebrity and the time the celebrity was detected in a stored video. For more information, see GetCelebrityRecognition in the Amazon Rekognition Developer Guide.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CelebrityRecognition {
    /// <p>Information about a recognized celebrity.</p>
    #[serde(rename = "Celebrity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub celebrity: Option<CelebrityDetail>,
    /// <p>The time, in milliseconds from the start of the video, that the celebrity was recognized.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

/// <p>Provides information about a face in a target image that matches the source image face analyzed by <code>CompareFaces</code>. The <code>Face</code> property contains the bounding box of the face in the target image. The <code>Similarity</code> property is the confidence that the source image face matches the face in the bounding box.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CompareFacesMatch {
    /// <p>Provides face metadata (bounding box and confidence that the bounding box actually contains a face).</p>
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<ComparedFace>,
    /// <p>Level of confidence that the faces match.</p>
    #[serde(rename = "Similarity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CompareFacesRequest {
    /// <p>The minimum level of confidence in the face matches that a match must meet to be included in the <code>FaceMatches</code> array.</p>
    #[serde(rename = "SimilarityThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity_threshold: Option<f32>,
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p> <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    #[serde(rename = "SourceImage")]
    pub source_image: Image,
    /// <p>The target image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p> <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    #[serde(rename = "TargetImage")]
    pub target_image: Image,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CompareFacesResponse {
    /// <p>An array of faces in the target image that match the source image face. Each <code>CompareFacesMatch</code> object provides the bounding box, the confidence level that the bounding box contains a face, and the similarity score for the face in the bounding box and the face in the source image.</p>
    #[serde(rename = "FaceMatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_matches: Option<Vec<CompareFacesMatch>>,
    /// <p>The face in the source image that was used for comparison.</p>
    #[serde(rename = "SourceImageFace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_image_face: Option<ComparedSourceImageFace>,
    /// <p>The value of <code>SourceImageOrientationCorrection</code> is always null.</p> <p>If the input image is in .jpeg format, it might contain exchangeable image file format (Exif) metadata that includes the image's orientation. Amazon Rekognition uses this orientation information to perform image correction. The bounding box coordinates are translated to represent object locations after the orientation information in the Exif metadata is used to correct the image orientation. Images in .png format don't contain Exif metadata.</p> <p>Amazon Rekognition doesn’t perform image correction for images in .png format and .jpeg images without orientation information in the image Exif metadata. The bounding box coordinates aren't translated and represent the object locations before the image is rotated. </p>
    #[serde(rename = "SourceImageOrientationCorrection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_image_orientation_correction: Option<String>,
    /// <p>The value of <code>TargetImageOrientationCorrection</code> is always null.</p> <p>If the input image is in .jpeg format, it might contain exchangeable image file format (Exif) metadata that includes the image's orientation. Amazon Rekognition uses this orientation information to perform image correction. The bounding box coordinates are translated to represent object locations after the orientation information in the Exif metadata is used to correct the image orientation. Images in .png format don't contain Exif metadata.</p> <p>Amazon Rekognition doesn’t perform image correction for images in .png format and .jpeg images without orientation information in the image Exif metadata. The bounding box coordinates aren't translated and represent the object locations before the image is rotated. </p>
    #[serde(rename = "TargetImageOrientationCorrection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_image_orientation_correction: Option<String>,
    /// <p>An array of faces in the target image that did not match the source image face.</p>
    #[serde(rename = "UnmatchedFaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmatched_faces: Option<Vec<ComparedFace>>,
}

/// <p>Provides face metadata for target image faces that are analyzed by <code>CompareFaces</code> and <code>RecognizeCelebrities</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ComparedFace {
    /// <p>Bounding box of the face.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>Level of confidence that what the bounding box contains is a face.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>An array of facial landmarks.</p>
    #[serde(rename = "Landmarks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landmarks: Option<Vec<Landmark>>,
    /// <p>Indicates the pose of the face as determined by its pitch, roll, and yaw.</p>
    #[serde(rename = "Pose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose: Option<Pose>,
    /// <p>Identifies face image brightness and sharpness. </p>
    #[serde(rename = "Quality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageQuality>,
}

/// <p>Type that describes the face Amazon Rekognition chose to compare with the faces in the target. This contains a bounding box for the selected face and confidence level that the bounding box contains a face. Note that Amazon Rekognition selects the largest face in the source image for this comparison. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ComparedSourceImageFace {
    /// <p>Bounding box of the face.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>Confidence level that the selected bounding box contains a face.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
}

/// <p>Information about a moderation label detection in a stored video.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ContentModerationDetection {
    /// <p>The moderation label detected by in the stored video.</p>
    #[serde(rename = "ModerationLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_label: Option<ModerationLabel>,
    /// <p>Time, in milliseconds from the beginning of the video, that the moderation label was detected.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateCollectionRequest {
    /// <p>ID for the collection that you are creating.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateCollectionResponse {
    /// <p>Amazon Resource Name (ARN) of the collection. You can use this to manage permissions on your resources. </p>
    #[serde(rename = "CollectionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_arn: Option<String>,
    /// <p>Version number of the face detection model associated with the collection you are creating.</p>
    #[serde(rename = "FaceModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    /// <p>HTTP status code indicating the result of the operation.</p>
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateStreamProcessorRequest {
    /// <p>Kinesis video stream stream that provides the source streaming video. If you are using the AWS CLI, the parameter name is <code>StreamProcessorInput</code>.</p>
    #[serde(rename = "Input")]
    pub input: StreamProcessorInput,
    /// <p>An identifier you assign to the stream processor. You can use <code>Name</code> to manage the stream processor. For example, you can get the current status of the stream processor by calling <a>DescribeStreamProcessor</a>. <code>Name</code> is idempotent. </p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Kinesis data stream stream to which Amazon Rekognition Video puts the analysis results. If you are using the AWS CLI, the parameter name is <code>StreamProcessorOutput</code>.</p>
    #[serde(rename = "Output")]
    pub output: StreamProcessorOutput,
    /// <p>ARN of the IAM role that allows access to the stream processor.</p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>Face recognition input parameters to be used by the stream processor. Includes the collection to use for face recognition and the face attributes to detect.</p>
    #[serde(rename = "Settings")]
    pub settings: StreamProcessorSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateStreamProcessorResponse {
    /// <p>ARN for the newly create stream processor.</p>
    #[serde(rename = "StreamProcessorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_processor_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteCollectionRequest {
    /// <p>ID of the collection to delete.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteCollectionResponse {
    /// <p>HTTP status code that indicates the result of the operation.</p>
    #[serde(rename = "StatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteFacesRequest {
    /// <p>Collection from which to remove the specific faces.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
    /// <p>An array of face IDs to delete.</p>
    #[serde(rename = "FaceIds")]
    pub face_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteFacesResponse {
    /// <p>An array of strings (face IDs) of the faces that were deleted.</p>
    #[serde(rename = "DeletedFaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_faces: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteStreamProcessorRequest {
    /// <p>The name of the stream processor you want to delete.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteStreamProcessorResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeCollectionRequest {
    /// <p>The ID of the collection to describe.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeCollectionResponse {
    /// <p>The Amazon Resource Name (ARN) of the collection.</p>
    #[serde(rename = "CollectionARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_arn: Option<String>,
    /// <p>The number of milliseconds since the Unix epoch time until the creation of the collection. The Unix epoch time is 00:00:00 Coordinated Universal Time (UTC), Thursday, 1 January 1970.</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<f64>,
    /// <p>The number of faces that are indexed into the collection. To index faces into a collection, use <a>IndexFaces</a>.</p>
    #[serde(rename = "FaceCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_count: Option<i64>,
    /// <p>The version of the face model that's used by the collection for face detection.</p> <p>For more information, see Model Versioning in the Amazon Rekognition Developer Guide.</p>
    #[serde(rename = "FaceModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeStreamProcessorRequest {
    /// <p>Name of the stream processor for which you want information.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeStreamProcessorResponse {
    /// <p>Date and time the stream processor was created</p>
    #[serde(rename = "CreationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<f64>,
    /// <p>Kinesis video stream that provides the source streaming video.</p>
    #[serde(rename = "Input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<StreamProcessorInput>,
    /// <p>The time, in Unix format, the stream processor was last updated. For example, when the stream processor moves from a running state to a failed state, or when the user starts or stops the stream processor.</p>
    #[serde(rename = "LastUpdateTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_timestamp: Option<f64>,
    /// <p>Name of the stream processor. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Kinesis data stream to which Amazon Rekognition Video puts the analysis results.</p>
    #[serde(rename = "Output")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<StreamProcessorOutput>,
    /// <p>ARN of the IAM role that allows access to the stream processor.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>Face recognition input parameters that are being used by the stream processor. Includes the collection to use for face recognition and the face attributes to detect.</p>
    #[serde(rename = "Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<StreamProcessorSettings>,
    /// <p>Current status of the stream processor.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Detailed status message about the stream processor.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>ARN of the stream processor.</p>
    #[serde(rename = "StreamProcessorArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_processor_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetectFacesRequest {
    /// <p>An array of facial attributes you want to be returned. This can be the default list of attributes or all attributes. If you don't specify a value for <code>Attributes</code> or if you specify <code>["DEFAULT"]</code>, the API returns the following subset of facial attributes: <code>BoundingBox</code>, <code>Confidence</code>, <code>Pose</code>, <code>Quality</code>, and <code>Landmarks</code>. If you provide <code>["ALL"]</code>, all facial attributes are returned, but the operation takes longer to complete.</p> <p>If you provide both, <code>["ALL", "DEFAULT"]</code>, the service uses a logical AND operator to determine which attributes to return (in this case, all attributes). </p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p> <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    #[serde(rename = "Image")]
    pub image: Image,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DetectFacesResponse {
    /// <p>Details of each face found in the image. </p>
    #[serde(rename = "FaceDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_details: Option<Vec<FaceDetail>>,
    /// <p>The value of <code>OrientationCorrection</code> is always null.</p> <p>If the input image is in .jpeg format, it might contain exchangeable image file format (Exif) metadata that includes the image's orientation. Amazon Rekognition uses this orientation information to perform image correction. The bounding box coordinates are translated to represent object locations after the orientation information in the Exif metadata is used to correct the image orientation. Images in .png format don't contain Exif metadata.</p> <p>Amazon Rekognition doesn’t perform image correction for images in .png format and .jpeg images without orientation information in the image Exif metadata. The bounding box coordinates aren't translated and represent the object locations before the image is rotated. </p>
    #[serde(rename = "OrientationCorrection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation_correction: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetectLabelsRequest {
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes is not supported. Images stored in an S3 Bucket do not need to be base64-encoded.</p> <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    #[serde(rename = "Image")]
    pub image: Image,
    /// <p>Maximum number of labels you want the service to return in the response. The service returns the specified number of highest confidence labels. </p>
    #[serde(rename = "MaxLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_labels: Option<i64>,
    /// <p>Specifies the minimum confidence level for the labels to return. Amazon Rekognition doesn't return any labels with confidence lower than this specified value.</p> <p>If <code>MinConfidence</code> is not specified, the operation returns labels with a confidence values greater than or equal to 55 percent.</p>
    #[serde(rename = "MinConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DetectLabelsResponse {
    /// <p>Version number of the label detection model that was used to detect labels.</p>
    #[serde(rename = "LabelModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_model_version: Option<String>,
    /// <p>An array of labels for the real-world objects detected. </p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<Label>>,
    /// <p>The value of <code>OrientationCorrection</code> is always null.</p> <p>If the input image is in .jpeg format, it might contain exchangeable image file format (Exif) metadata that includes the image's orientation. Amazon Rekognition uses this orientation information to perform image correction. The bounding box coordinates are translated to represent object locations after the orientation information in the Exif metadata is used to correct the image orientation. Images in .png format don't contain Exif metadata.</p> <p>Amazon Rekognition doesn’t perform image correction for images in .png format and .jpeg images without orientation information in the image Exif metadata. The bounding box coordinates aren't translated and represent the object locations before the image is rotated. </p>
    #[serde(rename = "OrientationCorrection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation_correction: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetectModerationLabelsRequest {
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p> <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    #[serde(rename = "Image")]
    pub image: Image,
    /// <p>Specifies the minimum confidence level for the labels to return. Amazon Rekognition doesn't return any labels with a confidence level lower than this specified value.</p> <p>If you don't specify <code>MinConfidence</code>, the operation returns labels with confidence values greater than or equal to 50 percent.</p>
    #[serde(rename = "MinConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DetectModerationLabelsResponse {
    /// <p>Array of detected Moderation labels and the time, in millseconds from the start of the video, they were detected.</p>
    #[serde(rename = "ModerationLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_labels: Option<Vec<ModerationLabel>>,
    /// <p>Version number of the moderation detection model that was used to detect unsafe content.</p>
    #[serde(rename = "ModerationModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_model_version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DetectTextRequest {
    /// <p>The input image as base64-encoded bytes or an Amazon S3 object. If you use the AWS CLI to call Amazon Rekognition operations, you can't pass image bytes. </p> <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    #[serde(rename = "Image")]
    pub image: Image,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DetectTextResponse {
    /// <p>An array of text that was detected in the input image.</p>
    #[serde(rename = "TextDetections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_detections: Option<Vec<TextDetection>>,
}

/// <p>The emotions detected on the face, and the confidence level in the determination. For example, HAPPY, SAD, and ANGRY.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Emotion {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Type of emotion detected.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Indicates whether or not the eyes on the face are open, and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EyeOpen {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Boolean value that indicates whether the eyes on the face are open.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

/// <p>Indicates whether or not the face is wearing eye glasses, and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Eyeglasses {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Boolean value that indicates whether the face is wearing eye glasses or not.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

/// <p>Describes the face properties such as the bounding box, face ID, image ID of the input image, and external image ID that you assigned. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Face {
    /// <p>Bounding box of the face.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>Confidence level that the bounding box contains a face (and not a different object such as a tree).</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Identifier that you assign to all the faces in the input image.</p>
    #[serde(rename = "ExternalImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_image_id: Option<String>,
    /// <p>Unique identifier that Amazon Rekognition assigns to the face.</p>
    #[serde(rename = "FaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_id: Option<String>,
    /// <p>Unique identifier that Amazon Rekognition assigns to the input image.</p>
    #[serde(rename = "ImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
}

/// <p>Structure containing attributes of the face that the algorithm detected.</p> <p>A <code>FaceDetail</code> object contains either the default facial attributes or all facial attributes. The default attributes are <code>BoundingBox</code>, <code>Confidence</code>, <code>Landmarks</code>, <code>Pose</code>, and <code>Quality</code>.</p> <p> <a>GetFaceDetection</a> is the only Amazon Rekognition Video stored video operation that can return a <code>FaceDetail</code> object with all attributes. To specify which attributes to return, use the <code>FaceAttributes</code> input parameter for <a>StartFaceDetection</a>. The following Amazon Rekognition Video operations return only the default attributes. The corresponding Start operations don't have a <code>FaceAttributes</code> input parameter.</p> <ul> <li> <p>GetCelebrityRecognition</p> </li> <li> <p>GetPersonTracking</p> </li> <li> <p>GetFaceSearch</p> </li> </ul> <p>The Amazon Rekognition Image <a>DetectFaces</a> and <a>IndexFaces</a> operations can return all facial attributes. To specify which attributes to return, use the <code>Attributes</code> input parameter for <code>DetectFaces</code>. For <code>IndexFaces</code>, use the <code>DetectAttributes</code> input parameter.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FaceDetail {
    /// <p>The estimated age range, in years, for the face. Low represents the lowest estimated age and High represents the highest estimated age.</p>
    #[serde(rename = "AgeRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_range: Option<AgeRange>,
    /// <p>Indicates whether or not the face has a beard, and the confidence level in the determination.</p>
    #[serde(rename = "Beard")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beard: Option<Beard>,
    /// <p>Bounding box of the face. Default attribute.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>Confidence level that the bounding box contains a face (and not a different object such as a tree). Default attribute.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>The emotions detected on the face, and the confidence level in the determination. For example, HAPPY, SAD, and ANGRY. </p>
    #[serde(rename = "Emotions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emotions: Option<Vec<Emotion>>,
    /// <p>Indicates whether or not the face is wearing eye glasses, and the confidence level in the determination.</p>
    #[serde(rename = "Eyeglasses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eyeglasses: Option<Eyeglasses>,
    /// <p>Indicates whether or not the eyes on the face are open, and the confidence level in the determination.</p>
    #[serde(rename = "EyesOpen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eyes_open: Option<EyeOpen>,
    /// <p>Gender of the face and the confidence level in the determination.</p>
    #[serde(rename = "Gender")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    /// <p>Indicates the location of landmarks on the face. Default attribute.</p>
    #[serde(rename = "Landmarks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landmarks: Option<Vec<Landmark>>,
    /// <p>Indicates whether or not the mouth on the face is open, and the confidence level in the determination.</p>
    #[serde(rename = "MouthOpen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouth_open: Option<MouthOpen>,
    /// <p>Indicates whether or not the face has a mustache, and the confidence level in the determination.</p>
    #[serde(rename = "Mustache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mustache: Option<Mustache>,
    /// <p>Indicates the pose of the face as determined by its pitch, roll, and yaw. Default attribute.</p>
    #[serde(rename = "Pose")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pose: Option<Pose>,
    /// <p>Identifies image brightness and sharpness. Default attribute.</p>
    #[serde(rename = "Quality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<ImageQuality>,
    /// <p>Indicates whether or not the face is smiling, and the confidence level in the determination.</p>
    #[serde(rename = "Smile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smile: Option<Smile>,
    /// <p>Indicates whether or not the face is wearing sunglasses, and the confidence level in the determination.</p>
    #[serde(rename = "Sunglasses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunglasses: Option<Sunglasses>,
}

/// <p>Information about a face detected in a video analysis request and the time the face was detected in the video. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FaceDetection {
    /// <p>The face properties for the detected face.</p>
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<FaceDetail>,
    /// <p>Time, in milliseconds from the start of the video, that the face was detected.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

/// <p>Provides face metadata. In addition, it also provides the confidence in the match of this face with the input face.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FaceMatch {
    /// <p>Describes the face properties such as the bounding box, face ID, image ID of the source image, and external image ID that you assigned.</p>
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<Face>,
    /// <p>Confidence in the match of this face with the input face.</p>
    #[serde(rename = "Similarity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub similarity: Option<f32>,
}

/// <p>Object containing both the face metadata (stored in the backend database), and facial attributes that are detected but aren't stored in the database.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FaceRecord {
    /// <p>Describes the face properties such as the bounding box, face ID, image ID of the input image, and external image ID that you assigned. </p>
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<Face>,
    /// <p>Structure containing attributes of the face that the algorithm detected.</p>
    #[serde(rename = "FaceDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_detail: Option<FaceDetail>,
}

/// <p>Input face recognition parameters for an Amazon Rekognition stream processor. <code>FaceRecognitionSettings</code> is a request parameter for <a>CreateStreamProcessor</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FaceSearchSettings {
    /// <p>The ID of a collection that contains faces that you want to search for.</p>
    #[serde(rename = "CollectionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_id: Option<String>,
    /// <p>Minimum face match confidence score that must be met to return a result for a recognized face. Default is 70. 0 is the lowest confidence. 100 is the highest confidence.</p>
    #[serde(rename = "FaceMatchThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_match_threshold: Option<f32>,
}

/// <p>Gender of the face and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Gender {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Gender of the face.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Information about where the text detected by <a>DetectText</a> is located on an image.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Geometry {
    /// <p>An axis-aligned coarse representation of the detected text's location on the image.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>Within the bounding box, a fine-grained polygon around the detected text.</p>
    #[serde(rename = "Polygon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon: Option<Vec<Point>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCelebrityInfoRequest {
    /// <p>The ID for the celebrity. You get the celebrity ID from a call to the <a>RecognizeCelebrities</a> operation, which recognizes celebrities in an image. </p>
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCelebrityInfoResponse {
    /// <p>The name of the celebrity.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An array of URLs pointing to additional celebrity information. </p>
    #[serde(rename = "Urls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urls: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCelebrityRecognitionRequest {
    /// <p>Job identifier for the required celebrity recognition analysis. You can get the job identifer from a call to <code>StartCelebrityRecognition</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>Maximum number of results to return per paginated call. The largest value you can specify is 1000. If you specify a value greater than 1000, a maximum of 1000 results is returned. The default value is 1000.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there is more recognized celebrities to retrieve), Amazon Rekognition Video returns a pagination token in the response. You can use this pagination token to retrieve the next set of celebrities. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Sort to use for celebrities returned in <code>Celebrities</code> field. Specify <code>ID</code> to sort by the celebrity identifier, specify <code>TIMESTAMP</code> to sort by the time the celebrity was recognized.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCelebrityRecognitionResponse {
    /// <p>Array of celebrities recognized in the video.</p>
    #[serde(rename = "Celebrities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub celebrities: Option<Vec<CelebrityRecognition>>,
    /// <p>The current status of the celebrity recognition job.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of celebrities.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition Video operation.</p>
    #[serde(rename = "VideoMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetContentModerationRequest {
    /// <p>The identifier for the content moderation job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetContentModeration</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>Maximum number of results to return per paginated call. The largest value you can specify is 1000. If you specify a value greater than 1000, a maximum of 1000 results is returned. The default value is 1000.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there is more data to retrieve), Amazon Rekognition returns a pagination token in the response. You can use this pagination token to retrieve the next set of content moderation labels.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Sort to use for elements in the <code>ModerationLabelDetections</code> array. Use <code>TIMESTAMP</code> to sort array elements by the time labels are detected. Use <code>NAME</code> to alphabetically group elements for a label together. Within each label group, the array element are sorted by detection confidence. The default sort is by <code>TIMESTAMP</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetContentModerationResponse {
    /// <p>The current status of the content moderation job.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>The detected moderation labels and the time(s) they were detected.</p>
    #[serde(rename = "ModerationLabels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_labels: Option<Vec<ContentModerationDetection>>,
    /// <p>Version number of the moderation detection model that was used to detect unsafe content.</p>
    #[serde(rename = "ModerationModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation_model_version: Option<String>,
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of moderation labels. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Information about a video that Amazon Rekognition analyzed. <code>Videometadata</code> is returned in every page of paginated responses from <code>GetContentModeration</code>. </p>
    #[serde(rename = "VideoMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetFaceDetectionRequest {
    /// <p>Unique identifier for the face detection job. The <code>JobId</code> is returned from <code>StartFaceDetection</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>Maximum number of results to return per paginated call. The largest value you can specify is 1000. If you specify a value greater than 1000, a maximum of 1000 results is returned. The default value is 1000.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there are more faces to retrieve), Amazon Rekognition Video returns a pagination token in the response. You can use this pagination token to retrieve the next set of faces.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetFaceDetectionResponse {
    /// <p>An array of faces detected in the video. Each element contains a detected face's details and the time, in milliseconds from the start of the video, the face was detected. </p>
    #[serde(rename = "Faces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faces: Option<Vec<FaceDetection>>,
    /// <p>The current status of the face detection job.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>If the response is truncated, Amazon Rekognition returns this token that you can use in the subsequent request to retrieve the next set of faces. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition video operation.</p>
    #[serde(rename = "VideoMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetFaceSearchRequest {
    /// <p>The job identifer for the search request. You get the job identifier from an initial call to <code>StartFaceSearch</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>Maximum number of results to return per paginated call. The largest value you can specify is 1000. If you specify a value greater than 1000, a maximum of 1000 results is returned. The default value is 1000.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there is more search results to retrieve), Amazon Rekognition Video returns a pagination token in the response. You can use this pagination token to retrieve the next set of search results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Sort to use for grouping faces in the response. Use <code>TIMESTAMP</code> to group faces by the time that they are recognized. Use <code>INDEX</code> to sort by recognized faces. </p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetFaceSearchResponse {
    /// <p>The current status of the face search job.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of search results. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of persons, <a>PersonMatch</a>, in the video whose face(s) match the face(s) in an Amazon Rekognition collection. It also includes time information for when persons are matched in the video. You specify the input collection in an initial call to <code>StartFaceSearch</code>. Each <code>Persons</code> element includes a time the person was matched, face match details (<code>FaceMatches</code>) for matching faces in the collection, and person information (<code>Person</code>) for the matched person. </p>
    #[serde(rename = "Persons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persons: Option<Vec<PersonMatch>>,
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Information about a video that Amazon Rekognition analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition Video operation. </p>
    #[serde(rename = "VideoMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetLabelDetectionRequest {
    /// <p>Job identifier for the label detection operation for which you want results returned. You get the job identifer from an initial call to <code>StartlabelDetection</code>.</p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>Maximum number of results to return per paginated call. The largest value you can specify is 1000. If you specify a value greater than 1000, a maximum of 1000 results is returned. The default value is 1000.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there are more labels to retrieve), Amazon Rekognition Video returns a pagination token in the response. You can use this pagination token to retrieve the next set of labels. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Sort to use for elements in the <code>Labels</code> array. Use <code>TIMESTAMP</code> to sort array elements by the time labels are detected. Use <code>NAME</code> to alphabetically group elements for a label together. Within each label group, the array element are sorted by detection confidence. The default sort is by <code>TIMESTAMP</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetLabelDetectionResponse {
    /// <p>The current status of the label detection job.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>Version number of the label detection model that was used to detect labels.</p>
    #[serde(rename = "LabelModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label_model_version: Option<String>,
    /// <p>An array of labels detected in the video. Each element contains the detected label and the time, in milliseconds from the start of the video, that the label was detected. </p>
    #[serde(rename = "Labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<LabelDetection>>,
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of labels.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition video operation.</p>
    #[serde(rename = "VideoMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPersonTrackingRequest {
    /// <p>The identifier for a job that tracks persons in a video. You get the <code>JobId</code> from a call to <code>StartPersonTracking</code>. </p>
    #[serde(rename = "JobId")]
    pub job_id: String,
    /// <p>Maximum number of results to return per paginated call. The largest value you can specify is 1000. If you specify a value greater than 1000, a maximum of 1000 results is returned. The default value is 1000.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there are more persons to retrieve), Amazon Rekognition Video returns a pagination token in the response. You can use this pagination token to retrieve the next set of persons. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Sort to use for elements in the <code>Persons</code> array. Use <code>TIMESTAMP</code> to sort array elements by the time persons are detected. Use <code>INDEX</code> to sort by the tracked persons. If you sort by <code>INDEX</code>, the array elements for each person are sorted by detection confidence. The default sort is by <code>TIMESTAMP</code>.</p>
    #[serde(rename = "SortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetPersonTrackingResponse {
    /// <p>The current status of the person tracking job.</p>
    #[serde(rename = "JobStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of persons. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>An array of the persons detected in the video and the time(s) their path was tracked throughout the video. An array element will exist for each time a person's path is tracked. </p>
    #[serde(rename = "Persons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persons: Option<Vec<PersonDetection>>,
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    #[serde(rename = "StatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition Video operation.</p>
    #[serde(rename = "VideoMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

/// <p>Provides the input image either as bytes or an S3 object.</p> <p>You pass image bytes to an Amazon Rekognition API operation by using the <code>Bytes</code> property. For example, you would use the <code>Bytes</code> property to pass an image loaded from a local file system. Image bytes passed by using the <code>Bytes</code> property must be base64-encoded. Your code may not need to encode image bytes if you are using an AWS SDK to call Amazon Rekognition API operations. </p> <p>For more information, see Analyzing an Image Loaded from a Local File System in the Amazon Rekognition Developer Guide.</p> <p> You pass images stored in an S3 bucket to an Amazon Rekognition API operation by using the <code>S3Object</code> property. Images stored in an S3 bucket do not need to be base64-encoded.</p> <p>The region for the S3 bucket containing the S3 object must match the region you use for Amazon Rekognition operations.</p> <p>If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes using the Bytes property is not supported. You must first upload the image to an Amazon S3 bucket and then call the operation using the S3Object property.</p> <p>For Amazon Rekognition to process an S3 object, the user must have permission to access the S3 object. For more information, see Resource Based Policies in the Amazon Rekognition Developer Guide. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Image {
    /// <p>Blob of image bytes up to 5 MBs.</p>
    #[serde(rename = "Bytes")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<bytes::Bytes>,
    /// <p>Identifies an S3 object as the image source.</p>
    #[serde(rename = "S3Object")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3Object>,
}

/// <p>Identifies face image brightness and sharpness. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ImageQuality {
    /// <p>Value representing brightness of the face. The service returns a value between 0 and 100 (inclusive). A higher value indicates a brighter face image.</p>
    #[serde(rename = "Brightness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<f32>,
    /// <p>Value representing sharpness of the face. The service returns a value between 0 and 100 (inclusive). A higher value indicates a sharper face image.</p>
    #[serde(rename = "Sharpness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharpness: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct IndexFacesRequest {
    /// <p>The ID of an existing collection to which you want to add the faces that are detected in the input images.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
    /// <p>An array of facial attributes that you want to be returned. This can be the default list of attributes or all attributes. If you don't specify a value for <code>Attributes</code> or if you specify <code>["DEFAULT"]</code>, the API returns the following subset of facial attributes: <code>BoundingBox</code>, <code>Confidence</code>, <code>Pose</code>, <code>Quality</code>, and <code>Landmarks</code>. If you provide <code>["ALL"]</code>, all facial attributes are returned, but the operation takes longer to complete.</p> <p>If you provide both, <code>["ALL", "DEFAULT"]</code>, the service uses a logical AND operator to determine which attributes to return (in this case, all attributes). </p>
    #[serde(rename = "DetectionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detection_attributes: Option<Vec<String>>,
    /// <p>The ID you want to assign to all the faces detected in the image.</p>
    #[serde(rename = "ExternalImageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_image_id: Option<String>,
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes isn't supported. </p> <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    #[serde(rename = "Image")]
    pub image: Image,
    /// <p>The maximum number of faces to index. The value of <code>MaxFaces</code> must be greater than or equal to 1. <code>IndexFaces</code> returns no more than 100 detected faces in an image, even if you specify a larger value for <code>MaxFaces</code>.</p> <p>If <code>IndexFaces</code> detects more faces than the value of <code>MaxFaces</code>, the faces with the lowest quality are filtered out first. If there are still more faces than the value of <code>MaxFaces</code>, the faces with the smallest bounding boxes are filtered out (up to the number that's needed to satisfy the value of <code>MaxFaces</code>). Information about the unindexed faces is available in the <code>UnindexedFaces</code> array. </p> <p>The faces that are returned by <code>IndexFaces</code> are sorted by the largest face bounding box size to the smallest size, in descending order.</p> <p> <code>MaxFaces</code> can be used with a collection associated with any version of the face model.</p>
    #[serde(rename = "MaxFaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_faces: Option<i64>,
    /// <p>A filter that specifies how much filtering is done to identify faces that are detected with low quality. Filtered faces aren't indexed. If you specify <code>AUTO</code>, filtering prioritizes the identification of faces that don’t meet the required quality bar chosen by Amazon Rekognition. The quality bar is based on a variety of common use cases. Low-quality detections can occur for a number of reasons. Some examples are an object that's misidentified as a face, a face that's too blurry, or a face with a pose that's too extreme to use. If you specify <code>NONE</code>, no filtering is performed. The default value is AUTO.</p> <p>To use quality filtering, the collection you are using must be associated with version 3 of the face model.</p>
    #[serde(rename = "QualityFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_filter: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct IndexFacesResponse {
    /// <p>The version number of the face detection model that's associated with the input collection (<code>CollectionId</code>).</p>
    #[serde(rename = "FaceModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    /// <p>An array of faces detected and added to the collection. For more information, see Searching Faces in a Collection in the Amazon Rekognition Developer Guide. </p>
    #[serde(rename = "FaceRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_records: Option<Vec<FaceRecord>>,
    /// <p>If your collection is associated with a face detection model that's later than version 3.0, the value of <code>OrientationCorrection</code> is always null and no orientation information is returned.</p> <p>If your collection is associated with a face detection model that's version 3.0 or earlier, the following applies:</p> <ul> <li> <p>If the input image is in .jpeg format, it might contain exchangeable image file format (Exif) metadata that includes the image's orientation. Amazon Rekognition uses this orientation information to perform image correction - the bounding box coordinates are translated to represent object locations after the orientation information in the Exif metadata is used to correct the image orientation. Images in .png format don't contain Exif metadata. The value of <code>OrientationCorrection</code> is null.</p> </li> <li> <p>If the image doesn't contain orientation information in its Exif metadata, Amazon Rekognition returns an estimated orientation (ROTATE_0, ROTATE_90, ROTATE_180, ROTATE_270). Amazon Rekognition doesn’t perform image correction for images. The bounding box coordinates aren't translated and represent the object locations before the image is rotated.</p> </li> </ul> <p>Bounding box information is returned in the <code>FaceRecords</code> array. You can get the version of the face detection model by calling <a>DescribeCollection</a>. </p>
    #[serde(rename = "OrientationCorrection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation_correction: Option<String>,
    /// <p>An array of faces that were detected in the image but weren't indexed. They weren't indexed because the quality filter identified them as low quality, or the <code>MaxFaces</code> request parameter filtered them out. To use the quality filter, you specify the <code>QualityFilter</code> request parameter.</p>
    #[serde(rename = "UnindexedFaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unindexed_faces: Option<Vec<UnindexedFace>>,
}

/// <p>An instance of a label returned by Amazon Rekognition Image (<a>DetectLabels</a>) or by Amazon Rekognition Video (<a>GetLabelDetection</a>).</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Instance {
    /// <p>The position of the label instance on the image.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>The confidence that Amazon Rekognition has in the accuracy of the bounding box.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
}

/// <p>The Kinesis data stream Amazon Rekognition to which the analysis results of a Amazon Rekognition stream processor are streamed. For more information, see CreateStreamProcessor in the Amazon Rekognition Developer Guide.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KinesisDataStream {
    /// <p>ARN of the output Amazon Kinesis Data Streams stream.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

/// <p>Kinesis video stream stream that provides the source streaming video for a Amazon Rekognition Video stream processor. For more information, see CreateStreamProcessor in the Amazon Rekognition Developer Guide.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KinesisVideoStream {
    /// <p>ARN of the Kinesis video stream stream that streams the source video.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

/// <p>Structure containing details about the detected label, including the name, detected instances, parent labels, and level of confidence.</p> <p> </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Label {
    /// <p>Level of confidence.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>If <code>Label</code> represents an object, <code>Instances</code> contains the bounding boxes for each instance of the detected object. Bounding boxes are returned for common object labels such as people, cars, furniture, apparel or pets.</p>
    #[serde(rename = "Instances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<Instance>>,
    /// <p>The name (label) of the object or scene.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The parent labels for a label. The response includes all ancestor labels.</p>
    #[serde(rename = "Parents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<Parent>>,
}

/// <p>Information about a label detected in a video analysis request and the time the label was detected in the video. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LabelDetection {
    /// <p>Details about the detected label.</p>
    #[serde(rename = "Label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    /// <p>Time, in milliseconds from the start of the video, that the label was detected.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

/// <p>Indicates the location of the landmark on the face.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Landmark {
    /// <p>Type of landmark.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The x-coordinate from the top left of the landmark expressed as the ratio of the width of the image. For example, if the image is 700 x 200 and the x-coordinate of the landmark is at 350 pixels, this value is 0.5. </p>
    #[serde(rename = "X")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f32>,
    /// <p>The y-coordinate from the top left of the landmark expressed as the ratio of the height of the image. For example, if the image is 700 x 200 and the y-coordinate of the landmark is at 100 pixels, this value is 0.5.</p>
    #[serde(rename = "Y")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListCollectionsRequest {
    /// <p>Maximum number of collection IDs to return. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Pagination token from the previous response.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListCollectionsResponse {
    /// <p>An array of collection IDs.</p>
    #[serde(rename = "CollectionIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_ids: Option<Vec<String>>,
    /// <p>Version numbers of the face detection models associated with the collections in the array <code>CollectionIds</code>. For example, the value of <code>FaceModelVersions[2]</code> is the version number for the face detection model used by the collection in <code>CollectionId[2]</code>.</p>
    #[serde(rename = "FaceModelVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_versions: Option<Vec<String>>,
    /// <p>If the result is truncated, the response provides a <code>NextToken</code> that you can use in the subsequent request to fetch the next set of collection IDs.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListFacesRequest {
    /// <p>ID of the collection from which to list the faces.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
    /// <p>Maximum number of faces to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there is more data to retrieve), Amazon Rekognition returns a pagination token in the response. You can use this pagination token to retrieve the next set of faces.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListFacesResponse {
    /// <p>Version number of the face detection model associated with the input collection (<code>CollectionId</code>).</p>
    #[serde(rename = "FaceModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    /// <p>An array of <code>Face</code> objects. </p>
    #[serde(rename = "Faces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faces: Option<Vec<Face>>,
    /// <p>If the response is truncated, Amazon Rekognition returns this token that you can use in the subsequent request to retrieve the next set of faces.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListStreamProcessorsRequest {
    /// <p>Maximum number of stream processors you want Amazon Rekognition Video to return in the response. The default is 1000. </p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>If the previous response was incomplete (because there are more stream processors to retrieve), Amazon Rekognition Video returns a pagination token in the response. You can use this pagination token to retrieve the next set of stream processors. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListStreamProcessorsResponse {
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of stream processors. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of stream processors that you have created.</p>
    #[serde(rename = "StreamProcessors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_processors: Option<Vec<StreamProcessor>>,
}

/// <p>Provides information about a single type of moderated content found in an image or video. Each type of moderated content has a label within a hierarchical taxonomy. For more information, see Detecting Unsafe Content in the Amazon Rekognition Developer Guide.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ModerationLabel {
    /// <p>Specifies the confidence that Amazon Rekognition has that the label has been correctly identified.</p> <p>If you don't specify the <code>MinConfidence</code> parameter in the call to <code>DetectModerationLabels</code>, the operation returns labels with a confidence value greater than or equal to 50 percent.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>The label name for the type of content detected in the image.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The name for the parent label. Labels at the top level of the hierarchy have the parent label <code>""</code>.</p>
    #[serde(rename = "ParentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_name: Option<String>,
}

/// <p>Indicates whether or not the mouth on the face is open, and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct MouthOpen {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Boolean value that indicates whether the mouth on the face is open or not.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

/// <p>Indicates whether or not the face has a mustache, and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Mustache {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Boolean value that indicates whether the face has mustache or not.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

/// <p>The Amazon Simple Notification Service topic to which Amazon Rekognition publishes the completion status of a video analysis operation. For more information, see <a>api-video</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct NotificationChannel {
    /// <p>The ARN of an IAM role that gives Amazon Rekognition publishing permissions to the Amazon SNS topic. </p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The Amazon SNS topic to which Amazon Rekognition to posts the completion status.</p>
    #[serde(rename = "SNSTopicArn")]
    pub sns_topic_arn: String,
}

/// <p>A parent label for a label. A label can have 0, 1, or more parents. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Parent {
    /// <p>The name of the parent label.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Details about a person detected in a video analysis request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PersonDetail {
    /// <p>Bounding box around the detected person.</p>
    #[serde(rename = "BoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bounding_box: Option<BoundingBox>,
    /// <p>Face details for the detected person.</p>
    #[serde(rename = "Face")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face: Option<FaceDetail>,
    /// <p>Identifier for the person detected person within a video. Use to keep track of the person throughout the video. The identifier is not stored by Amazon Rekognition.</p>
    #[serde(rename = "Index")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
}

/// <p>Details and path tracking information for a single time a person's path is tracked in a video. Amazon Rekognition operations that track people's paths return an array of <code>PersonDetection</code> objects with elements for each time a person's path is tracked in a video. </p> <p>For more information, see GetPersonTracking in the Amazon Rekognition Developer Guide. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PersonDetection {
    /// <p>Details about a person whose path was tracked in a video.</p>
    #[serde(rename = "Person")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<PersonDetail>,
    /// <p>The time, in milliseconds from the start of the video, that the person's path was tracked.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

/// <p>Information about a person whose face matches a face(s) in an Amazon Rekognition collection. Includes information about the faces in the Amazon Rekognition collection (<a>FaceMatch</a>), information about the person (<a>PersonDetail</a>), and the time stamp for when the person was detected in a video. An array of <code>PersonMatch</code> objects is returned by <a>GetFaceSearch</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PersonMatch {
    /// <p>Information about the faces in the input collection that match the face of a person in the video.</p>
    #[serde(rename = "FaceMatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_matches: Option<Vec<FaceMatch>>,
    /// <p>Information about the matched person.</p>
    #[serde(rename = "Person")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<PersonDetail>,
    /// <p>The time, in milliseconds from the beginning of the video, that the person was matched in the video.</p>
    #[serde(rename = "Timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

/// <p>The X and Y coordinates of a point on an image. The X and Y values returned are ratios of the overall image size. For example, if the input image is 700x200 and the operation returns X=0.5 and Y=0.25, then the point is at the (350,50) pixel coordinate on the image.</p> <p>An array of <code>Point</code> objects, <code>Polygon</code>, is returned by <a>DetectText</a>. <code>Polygon</code> represents a fine-grained polygon around detected text. For more information, see Geometry in the Amazon Rekognition Developer Guide. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Point {
    /// <p>The value of the X coordinate for a point on a <code>Polygon</code>.</p>
    #[serde(rename = "X")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f32>,
    /// <p>The value of the Y coordinate for a point on a <code>Polygon</code>.</p>
    #[serde(rename = "Y")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f32>,
}

/// <p>Indicates the pose of the face as determined by its pitch, roll, and yaw.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Pose {
    /// <p>Value representing the face rotation on the pitch axis.</p>
    #[serde(rename = "Pitch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch: Option<f32>,
    /// <p>Value representing the face rotation on the roll axis.</p>
    #[serde(rename = "Roll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roll: Option<f32>,
    /// <p>Value representing the face rotation on the yaw axis.</p>
    #[serde(rename = "Yaw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yaw: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RecognizeCelebritiesRequest {
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p> <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    #[serde(rename = "Image")]
    pub image: Image,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RecognizeCelebritiesResponse {
    /// <p>Details about each celebrity found in the image. Amazon Rekognition can detect a maximum of 15 celebrities in an image.</p>
    #[serde(rename = "CelebrityFaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub celebrity_faces: Option<Vec<Celebrity>>,
    /// <p><p>The orientation of the input image (counterclockwise direction). If your application displays the image, you can use this value to correct the orientation. The bounding box coordinates returned in <code>CelebrityFaces</code> and <code>UnrecognizedFaces</code> represent face locations before the image orientation is corrected. </p> <note> <p>If the input image is in .jpeg format, it might contain exchangeable image (Exif) metadata that includes the image&#39;s orientation. If so, and the Exif metadata for the input image populates the orientation field, the value of <code>OrientationCorrection</code> is null. The <code>CelebrityFaces</code> and <code>UnrecognizedFaces</code> bounding box coordinates represent face locations after Exif metadata is used to correct the image orientation. Images in .png format don&#39;t contain Exif metadata. </p> </note></p>
    #[serde(rename = "OrientationCorrection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation_correction: Option<String>,
    /// <p>Details about each unrecognized face in the image.</p>
    #[serde(rename = "UnrecognizedFaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unrecognized_faces: Option<Vec<ComparedFace>>,
}

/// <p>Provides the S3 bucket name and object name.</p> <p>The region for the S3 bucket containing the S3 object must match the region you use for Amazon Rekognition operations.</p> <p>For Amazon Rekognition to process an S3 object, the user must have permission to access the S3 object. For more information, see Resource-Based Policies in the Amazon Rekognition Developer Guide. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct S3Object {
    /// <p>Name of the S3 bucket.</p>
    #[serde(rename = "Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// <p>S3 object key name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>If the bucket is versioning enabled, you can specify the object version. </p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SearchFacesByImageRequest {
    /// <p>ID of the collection to search.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
    /// <p>(Optional) Specifies the minimum confidence in the face match to return. For example, don't return any matches where confidence in matches is less than 70%.</p>
    #[serde(rename = "FaceMatchThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_match_threshold: Option<f32>,
    /// <p>The input image as base64-encoded bytes or an S3 object. If you use the AWS CLI to call Amazon Rekognition operations, passing base64-encoded image bytes is not supported. </p> <p>If you are using an AWS SDK to call Amazon Rekognition, you might not need to base64-encode image bytes passed using the <code>Bytes</code> field. For more information, see Images in the Amazon Rekognition developer guide.</p>
    #[serde(rename = "Image")]
    pub image: Image,
    /// <p>Maximum number of faces to return. The operation returns the maximum number of faces with the highest confidence in the match.</p>
    #[serde(rename = "MaxFaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_faces: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SearchFacesByImageResponse {
    /// <p>An array of faces that match the input face, along with the confidence in the match.</p>
    #[serde(rename = "FaceMatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_matches: Option<Vec<FaceMatch>>,
    /// <p>Version number of the face detection model associated with the input collection (<code>CollectionId</code>).</p>
    #[serde(rename = "FaceModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    /// <p>The bounding box around the face in the input image that Amazon Rekognition used for the search.</p>
    #[serde(rename = "SearchedFaceBoundingBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searched_face_bounding_box: Option<BoundingBox>,
    /// <p>The level of confidence that the <code>searchedFaceBoundingBox</code>, contains a face.</p>
    #[serde(rename = "SearchedFaceConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searched_face_confidence: Option<f32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SearchFacesRequest {
    /// <p>ID of the collection the face belongs to.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
    /// <p>ID of a face to find matches for in the collection.</p>
    #[serde(rename = "FaceId")]
    pub face_id: String,
    /// <p>Optional value specifying the minimum confidence in the face match to return. For example, don't return any matches where confidence in matches is less than 70%.</p>
    #[serde(rename = "FaceMatchThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_match_threshold: Option<f32>,
    /// <p>Maximum number of faces to return. The operation returns the maximum number of faces with the highest confidence in the match.</p>
    #[serde(rename = "MaxFaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_faces: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SearchFacesResponse {
    /// <p>An array of faces that matched the input face, along with the confidence in the match.</p>
    #[serde(rename = "FaceMatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_matches: Option<Vec<FaceMatch>>,
    /// <p>Version number of the face detection model associated with the input collection (<code>CollectionId</code>).</p>
    #[serde(rename = "FaceModelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_model_version: Option<String>,
    /// <p>ID of the face that was searched for matches in a collection.</p>
    #[serde(rename = "SearchedFaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searched_face_id: Option<String>,
}

/// <p>Indicates whether or not the face is smiling, and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Smile {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Boolean value that indicates whether the face is smiling or not.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartCelebrityRecognitionRequest {
    /// <p>Idempotent token used to identify the start request. If you use the same token with multiple <code>StartCelebrityRecognition</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidently started more than once. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>Unique identifier you specify to identify the job in the completion status published to the Amazon Simple Notification Service topic. </p>
    #[serde(rename = "JobTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    /// <p>The Amazon SNS topic ARN that you want Amazon Rekognition Video to publish the completion status of the celebrity recognition analysis to.</p>
    #[serde(rename = "NotificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    /// <p>The video in which you want to recognize celebrities. The video must be stored in an Amazon S3 bucket.</p>
    #[serde(rename = "Video")]
    pub video: Video,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartCelebrityRecognitionResponse {
    /// <p>The identifier for the celebrity recognition analysis job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetCelebrityRecognition</code>.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartContentModerationRequest {
    /// <p>Idempotent token used to identify the start request. If you use the same token with multiple <code>StartContentModeration</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidently started more than once. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>Unique identifier you specify to identify the job in the completion status published to the Amazon Simple Notification Service topic. </p>
    #[serde(rename = "JobTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    /// <p>Specifies the minimum confidence that Amazon Rekognition must have in order to return a moderated content label. Confidence represents how certain Amazon Rekognition is that the moderated content is correctly identified. 0 is the lowest confidence. 100 is the highest confidence. Amazon Rekognition doesn't return any moderated content labels with a confidence level lower than this specified value. If you don't specify <code>MinConfidence</code>, <code>GetContentModeration</code> returns labels with confidence values greater than or equal to 50 percent.</p>
    #[serde(rename = "MinConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
    /// <p>The Amazon SNS topic ARN that you want Amazon Rekognition Video to publish the completion status of the content moderation analysis to.</p>
    #[serde(rename = "NotificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    /// <p>The video in which you want to moderate content. The video must be stored in an Amazon S3 bucket.</p>
    #[serde(rename = "Video")]
    pub video: Video,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartContentModerationResponse {
    /// <p>The identifier for the content moderation analysis job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetContentModeration</code>.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartFaceDetectionRequest {
    /// <p>Idempotent token used to identify the start request. If you use the same token with multiple <code>StartFaceDetection</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidently started more than once. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>The face attributes you want returned.</p> <p> <code>DEFAULT</code> - The following subset of facial attributes are returned: BoundingBox, Confidence, Pose, Quality and Landmarks. </p> <p> <code>ALL</code> - All facial attributes are returned.</p>
    #[serde(rename = "FaceAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_attributes: Option<String>,
    /// <p>Unique identifier you specify to identify the job in the completion status published to the Amazon Simple Notification Service topic. </p>
    #[serde(rename = "JobTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    /// <p>The ARN of the Amazon SNS topic to which you want Amazon Rekognition Video to publish the completion status of the face detection operation.</p>
    #[serde(rename = "NotificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    /// <p>The video in which you want to detect faces. The video must be stored in an Amazon S3 bucket.</p>
    #[serde(rename = "Video")]
    pub video: Video,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartFaceDetectionResponse {
    /// <p>The identifier for the face detection job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetFaceDetection</code>.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartFaceSearchRequest {
    /// <p>Idempotent token used to identify the start request. If you use the same token with multiple <code>StartFaceSearch</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidently started more than once. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>ID of the collection that contains the faces you want to search for.</p>
    #[serde(rename = "CollectionId")]
    pub collection_id: String,
    /// <p>The minimum confidence in the person match to return. For example, don't return any matches where confidence in matches is less than 70%. </p>
    #[serde(rename = "FaceMatchThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_match_threshold: Option<f32>,
    /// <p>Unique identifier you specify to identify the job in the completion status published to the Amazon Simple Notification Service topic. </p>
    #[serde(rename = "JobTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    /// <p>The ARN of the Amazon SNS topic to which you want Amazon Rekognition Video to publish the completion status of the search. </p>
    #[serde(rename = "NotificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    /// <p>The video you want to search. The video must be stored in an Amazon S3 bucket. </p>
    #[serde(rename = "Video")]
    pub video: Video,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartFaceSearchResponse {
    /// <p>The identifier for the search job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetFaceSearch</code>. </p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartLabelDetectionRequest {
    /// <p>Idempotent token used to identify the start request. If you use the same token with multiple <code>StartLabelDetection</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidently started more than once. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>Unique identifier you specify to identify the job in the completion status published to the Amazon Simple Notification Service topic. </p>
    #[serde(rename = "JobTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    /// <p>Specifies the minimum confidence that Amazon Rekognition Video must have in order to return a detected label. Confidence represents how certain Amazon Rekognition is that a label is correctly identified.0 is the lowest confidence. 100 is the highest confidence. Amazon Rekognition Video doesn't return any labels with a confidence level lower than this specified value.</p> <p>If you don't specify <code>MinConfidence</code>, the operation returns labels with confidence values greater than or equal to 50 percent.</p>
    #[serde(rename = "MinConfidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
    /// <p>The Amazon SNS topic ARN you want Amazon Rekognition Video to publish the completion status of the label detection operation to. </p>
    #[serde(rename = "NotificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    /// <p>The video in which you want to detect labels. The video must be stored in an Amazon S3 bucket.</p>
    #[serde(rename = "Video")]
    pub video: Video,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartLabelDetectionResponse {
    /// <p>The identifier for the label detection job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetLabelDetection</code>. </p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartPersonTrackingRequest {
    /// <p>Idempotent token used to identify the start request. If you use the same token with multiple <code>StartPersonTracking</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidently started more than once. </p>
    #[serde(rename = "ClientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>Unique identifier you specify to identify the job in the completion status published to the Amazon Simple Notification Service topic. </p>
    #[serde(rename = "JobTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tag: Option<String>,
    /// <p>The Amazon SNS topic ARN you want Amazon Rekognition Video to publish the completion status of the people detection operation to.</p>
    #[serde(rename = "NotificationChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_channel: Option<NotificationChannel>,
    /// <p>The video in which you want to detect people. The video must be stored in an Amazon S3 bucket.</p>
    #[serde(rename = "Video")]
    pub video: Video,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartPersonTrackingResponse {
    /// <p>The identifier for the person detection job. Use <code>JobId</code> to identify the job in a subsequent call to <code>GetPersonTracking</code>.</p>
    #[serde(rename = "JobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartStreamProcessorRequest {
    /// <p>The name of the stream processor to start processing.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartStreamProcessorResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopStreamProcessorRequest {
    /// <p>The name of a stream processor created by <a>CreateStreamProcessor</a>.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StopStreamProcessorResponse {}

/// <p>An object that recognizes faces in a streaming video. An Amazon Rekognition stream processor is created by a call to <a>CreateStreamProcessor</a>. The request parameters for <code>CreateStreamProcessor</code> describe the Kinesis video stream source for the streaming video, face recognition parameters, and where to stream the analysis resullts. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StreamProcessor {
    /// <p>Name of the Amazon Rekognition stream processor. </p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Current status of the Amazon Rekognition stream processor.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Information about the source streaming video. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamProcessorInput {
    /// <p>The Kinesis video stream input stream for the source streaming video.</p>
    #[serde(rename = "KinesisVideoStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_video_stream: Option<KinesisVideoStream>,
}

/// <p>Information about the Amazon Kinesis Data Streams stream to which a Amazon Rekognition Video stream processor streams the results of a video analysis. For more information, see CreateStreamProcessor in the Amazon Rekognition Developer Guide.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamProcessorOutput {
    /// <p>The Amazon Kinesis Data Streams stream to which the Amazon Rekognition stream processor streams the analysis results.</p>
    #[serde(rename = "KinesisDataStream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_data_stream: Option<KinesisDataStream>,
}

/// <p>Input parameters used to recognize faces in a streaming video analyzed by a Amazon Rekognition stream processor.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamProcessorSettings {
    /// <p>Face search settings to use on a streaming video. </p>
    #[serde(rename = "FaceSearch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_search: Option<FaceSearchSettings>,
}

/// <p>Indicates whether or not the face is wearing sunglasses, and the confidence level in the determination.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Sunglasses {
    /// <p>Level of confidence in the determination.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>Boolean value that indicates whether the face is wearing sunglasses or not.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<bool>,
}

/// <p>Information about a word or line of text detected by <a>DetectText</a>.</p> <p>The <code>DetectedText</code> field contains the text that Amazon Rekognition detected in the image. </p> <p>Every word and line has an identifier (<code>Id</code>). Each word belongs to a line and has a parent identifier (<code>ParentId</code>) that identifies the line of text in which the word appears. The word <code>Id</code> is also an index for the word within a line of words. </p> <p>For more information, see Detecting Text in the Amazon Rekognition Developer Guide.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TextDetection {
    /// <p>The confidence that Amazon Rekognition has in the accuracy of the detected text and the accuracy of the geometry points around the detected text.</p>
    #[serde(rename = "Confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
    /// <p>The word or line of text recognized by Amazon Rekognition. </p>
    #[serde(rename = "DetectedText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detected_text: Option<String>,
    /// <p>The location of the detected text on the image. Includes an axis aligned coarse bounding box surrounding the text and a finer grain polygon for more accurate spatial information.</p>
    #[serde(rename = "Geometry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geometry: Option<Geometry>,
    /// <p>The identifier for the detected text. The identifier is only unique for a single call to <code>DetectText</code>. </p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// <p>The Parent identifier for the detected text identified by the value of <code>ID</code>. If the type of detected text is <code>LINE</code>, the value of <code>ParentId</code> is <code>Null</code>. </p>
    #[serde(rename = "ParentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i64>,
    /// <p>The type of text that was detected.</p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>A face that <a>IndexFaces</a> detected, but didn't index. Use the <code>Reasons</code> response attribute to determine why a face wasn't indexed.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UnindexedFace {
    /// <p>The structure that contains attributes of a face that <code>IndexFaces</code>detected, but didn't index. </p>
    #[serde(rename = "FaceDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_detail: Option<FaceDetail>,
    /// <p><p>An array of reasons that specify why a face wasn&#39;t indexed. </p> <ul> <li> <p>EXTREME<em>POSE - The face is at a pose that can&#39;t be detected. For example, the head is turned too far away from the camera.</p> </li> <li> <p>EXCEEDS</em>MAX<em>FACES - The number of faces detected is already higher than that specified by the <code>MaxFaces</code> input parameter for <code>IndexFaces</code>.</p> </li> <li> <p>LOW</em>BRIGHTNESS - The image is too dark.</p> </li> <li> <p>LOW<em>SHARPNESS - The image is too blurry.</p> </li> <li> <p>LOW</em>CONFIDENCE - The face was detected with a low confidence.</p> </li> <li> <p>SMALL<em>BOUNDING</em>BOX - The bounding box around the face is too small.</p> </li> </ul></p>
    #[serde(rename = "Reasons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasons: Option<Vec<String>>,
}

/// <p>Video file stored in an Amazon S3 bucket. Amazon Rekognition video start operations such as <a>StartLabelDetection</a> use <code>Video</code> to specify a video for analysis. The supported file formats are .mp4, .mov and .avi.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Video {
    /// <p>The Amazon S3 bucket name and file name for the video.</p>
    #[serde(rename = "S3Object")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object: Option<S3Object>,
}

/// <p>Information about a video that Amazon Rekognition analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition video operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct VideoMetadata {
    /// <p>Type of compression used in the analyzed video. </p>
    #[serde(rename = "Codec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    /// <p>Length of the video in milliseconds.</p>
    #[serde(rename = "DurationMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_millis: Option<i64>,
    /// <p>Format of the analyzed video. Possible values are MP4, MOV and AVI. </p>
    #[serde(rename = "Format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p>Vertical pixel dimension of the video.</p>
    #[serde(rename = "FrameHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_height: Option<i64>,
    /// <p>Number of frames per second in the video.</p>
    #[serde(rename = "FrameRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_rate: Option<f32>,
    /// <p>Horizontal pixel dimension of the video.</p>
    #[serde(rename = "FrameWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_width: Option<i64>,
}

/// Errors returned by CompareFaces
#[derive(Debug, PartialEq)]
pub enum CompareFacesError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CompareFacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CompareFacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CompareFacesError::AccessDenied(err.msg))
                }
                "ImageTooLargeException" => {
                    return RusotoError::Service(CompareFacesError::ImageTooLarge(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(CompareFacesError::InternalServerError(err.msg))
                }
                "InvalidImageFormatException" => {
                    return RusotoError::Service(CompareFacesError::InvalidImageFormat(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CompareFacesError::InvalidParameter(err.msg))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(CompareFacesError::InvalidS3Object(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(CompareFacesError::ProvisionedThroughputExceeded(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CompareFacesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CompareFacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CompareFacesError {
    fn description(&self) -> &str {
        match *self {
            CompareFacesError::AccessDenied(ref cause) => cause,
            CompareFacesError::ImageTooLarge(ref cause) => cause,
            CompareFacesError::InternalServerError(ref cause) => cause,
            CompareFacesError::InvalidImageFormat(ref cause) => cause,
            CompareFacesError::InvalidParameter(ref cause) => cause,
            CompareFacesError::InvalidS3Object(ref cause) => cause,
            CompareFacesError::ProvisionedThroughputExceeded(ref cause) => cause,
            CompareFacesError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCollection
#[derive(Debug, PartialEq)]
pub enum CreateCollectionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>A collection with the specified ID already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CreateCollectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCollectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateCollectionError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(CreateCollectionError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateCollectionError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        CreateCollectionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateCollectionError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateCollectionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateCollectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCollectionError {
    fn description(&self) -> &str {
        match *self {
            CreateCollectionError::AccessDenied(ref cause) => cause,
            CreateCollectionError::InternalServerError(ref cause) => cause,
            CreateCollectionError::InvalidParameter(ref cause) => cause,
            CreateCollectionError::ProvisionedThroughputExceeded(ref cause) => cause,
            CreateCollectionError::ResourceAlreadyExists(ref cause) => cause,
            CreateCollectionError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateStreamProcessor
#[derive(Debug, PartialEq)]
pub enum CreateStreamProcessorError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p><p/></p>
    ResourceInUse(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl CreateStreamProcessorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateStreamProcessorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateStreamProcessorError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(CreateStreamProcessorError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateStreamProcessorError::InvalidParameter(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateStreamProcessorError::LimitExceeded(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        CreateStreamProcessorError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateStreamProcessorError::ResourceInUse(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateStreamProcessorError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateStreamProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateStreamProcessorError {
    fn description(&self) -> &str {
        match *self {
            CreateStreamProcessorError::AccessDenied(ref cause) => cause,
            CreateStreamProcessorError::InternalServerError(ref cause) => cause,
            CreateStreamProcessorError::InvalidParameter(ref cause) => cause,
            CreateStreamProcessorError::LimitExceeded(ref cause) => cause,
            CreateStreamProcessorError::ProvisionedThroughputExceeded(ref cause) => cause,
            CreateStreamProcessorError::ResourceInUse(ref cause) => cause,
            CreateStreamProcessorError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCollection
#[derive(Debug, PartialEq)]
pub enum DeleteCollectionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DeleteCollectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCollectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteCollectionError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DeleteCollectionError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteCollectionError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        DeleteCollectionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteCollectionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteCollectionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteCollectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCollectionError {
    fn description(&self) -> &str {
        match *self {
            DeleteCollectionError::AccessDenied(ref cause) => cause,
            DeleteCollectionError::InternalServerError(ref cause) => cause,
            DeleteCollectionError::InvalidParameter(ref cause) => cause,
            DeleteCollectionError::ProvisionedThroughputExceeded(ref cause) => cause,
            DeleteCollectionError::ResourceNotFound(ref cause) => cause,
            DeleteCollectionError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteFaces
#[derive(Debug, PartialEq)]
pub enum DeleteFacesError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DeleteFacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteFacesError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DeleteFacesError::InternalServerError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteFacesError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(DeleteFacesError::ProvisionedThroughputExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteFacesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteFacesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteFacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteFacesError {
    fn description(&self) -> &str {
        match *self {
            DeleteFacesError::AccessDenied(ref cause) => cause,
            DeleteFacesError::InternalServerError(ref cause) => cause,
            DeleteFacesError::InvalidParameter(ref cause) => cause,
            DeleteFacesError::ProvisionedThroughputExceeded(ref cause) => cause,
            DeleteFacesError::ResourceNotFound(ref cause) => cause,
            DeleteFacesError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteStreamProcessor
#[derive(Debug, PartialEq)]
pub enum DeleteStreamProcessorError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p><p/></p>
    ResourceInUse(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DeleteStreamProcessorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteStreamProcessorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteStreamProcessorError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DeleteStreamProcessorError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteStreamProcessorError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        DeleteStreamProcessorError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteStreamProcessorError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteStreamProcessorError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteStreamProcessorError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteStreamProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteStreamProcessorError {
    fn description(&self) -> &str {
        match *self {
            DeleteStreamProcessorError::AccessDenied(ref cause) => cause,
            DeleteStreamProcessorError::InternalServerError(ref cause) => cause,
            DeleteStreamProcessorError::InvalidParameter(ref cause) => cause,
            DeleteStreamProcessorError::ProvisionedThroughputExceeded(ref cause) => cause,
            DeleteStreamProcessorError::ResourceInUse(ref cause) => cause,
            DeleteStreamProcessorError::ResourceNotFound(ref cause) => cause,
            DeleteStreamProcessorError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCollection
#[derive(Debug, PartialEq)]
pub enum DescribeCollectionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DescribeCollectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCollectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeCollectionError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DescribeCollectionError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeCollectionError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        DescribeCollectionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeCollectionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeCollectionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeCollectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCollectionError {
    fn description(&self) -> &str {
        match *self {
            DescribeCollectionError::AccessDenied(ref cause) => cause,
            DescribeCollectionError::InternalServerError(ref cause) => cause,
            DescribeCollectionError::InvalidParameter(ref cause) => cause,
            DescribeCollectionError::ProvisionedThroughputExceeded(ref cause) => cause,
            DescribeCollectionError::ResourceNotFound(ref cause) => cause,
            DescribeCollectionError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeStreamProcessor
#[derive(Debug, PartialEq)]
pub enum DescribeStreamProcessorError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DescribeStreamProcessorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeStreamProcessorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DescribeStreamProcessorError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DescribeStreamProcessorError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeStreamProcessorError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        DescribeStreamProcessorError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeStreamProcessorError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeStreamProcessorError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeStreamProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeStreamProcessorError {
    fn description(&self) -> &str {
        match *self {
            DescribeStreamProcessorError::AccessDenied(ref cause) => cause,
            DescribeStreamProcessorError::InternalServerError(ref cause) => cause,
            DescribeStreamProcessorError::InvalidParameter(ref cause) => cause,
            DescribeStreamProcessorError::ProvisionedThroughputExceeded(ref cause) => cause,
            DescribeStreamProcessorError::ResourceNotFound(ref cause) => cause,
            DescribeStreamProcessorError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DetectFaces
#[derive(Debug, PartialEq)]
pub enum DetectFacesError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DetectFacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectFacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DetectFacesError::AccessDenied(err.msg))
                }
                "ImageTooLargeException" => {
                    return RusotoError::Service(DetectFacesError::ImageTooLarge(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DetectFacesError::InternalServerError(err.msg))
                }
                "InvalidImageFormatException" => {
                    return RusotoError::Service(DetectFacesError::InvalidImageFormat(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DetectFacesError::InvalidParameter(err.msg))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(DetectFacesError::InvalidS3Object(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(DetectFacesError::ProvisionedThroughputExceeded(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DetectFacesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetectFacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetectFacesError {
    fn description(&self) -> &str {
        match *self {
            DetectFacesError::AccessDenied(ref cause) => cause,
            DetectFacesError::ImageTooLarge(ref cause) => cause,
            DetectFacesError::InternalServerError(ref cause) => cause,
            DetectFacesError::InvalidImageFormat(ref cause) => cause,
            DetectFacesError::InvalidParameter(ref cause) => cause,
            DetectFacesError::InvalidS3Object(ref cause) => cause,
            DetectFacesError::ProvisionedThroughputExceeded(ref cause) => cause,
            DetectFacesError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DetectLabels
#[derive(Debug, PartialEq)]
pub enum DetectLabelsError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DetectLabelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectLabelsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DetectLabelsError::AccessDenied(err.msg))
                }
                "ImageTooLargeException" => {
                    return RusotoError::Service(DetectLabelsError::ImageTooLarge(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DetectLabelsError::InternalServerError(err.msg))
                }
                "InvalidImageFormatException" => {
                    return RusotoError::Service(DetectLabelsError::InvalidImageFormat(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DetectLabelsError::InvalidParameter(err.msg))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(DetectLabelsError::InvalidS3Object(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(DetectLabelsError::ProvisionedThroughputExceeded(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DetectLabelsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetectLabelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetectLabelsError {
    fn description(&self) -> &str {
        match *self {
            DetectLabelsError::AccessDenied(ref cause) => cause,
            DetectLabelsError::ImageTooLarge(ref cause) => cause,
            DetectLabelsError::InternalServerError(ref cause) => cause,
            DetectLabelsError::InvalidImageFormat(ref cause) => cause,
            DetectLabelsError::InvalidParameter(ref cause) => cause,
            DetectLabelsError::InvalidS3Object(ref cause) => cause,
            DetectLabelsError::ProvisionedThroughputExceeded(ref cause) => cause,
            DetectLabelsError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DetectModerationLabels
#[derive(Debug, PartialEq)]
pub enum DetectModerationLabelsError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DetectModerationLabelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectModerationLabelsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DetectModerationLabelsError::AccessDenied(err.msg))
                }
                "ImageTooLargeException" => {
                    return RusotoError::Service(DetectModerationLabelsError::ImageTooLarge(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DetectModerationLabelsError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidImageFormatException" => {
                    return RusotoError::Service(DetectModerationLabelsError::InvalidImageFormat(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DetectModerationLabelsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(DetectModerationLabelsError::InvalidS3Object(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        DetectModerationLabelsError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DetectModerationLabelsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetectModerationLabelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetectModerationLabelsError {
    fn description(&self) -> &str {
        match *self {
            DetectModerationLabelsError::AccessDenied(ref cause) => cause,
            DetectModerationLabelsError::ImageTooLarge(ref cause) => cause,
            DetectModerationLabelsError::InternalServerError(ref cause) => cause,
            DetectModerationLabelsError::InvalidImageFormat(ref cause) => cause,
            DetectModerationLabelsError::InvalidParameter(ref cause) => cause,
            DetectModerationLabelsError::InvalidS3Object(ref cause) => cause,
            DetectModerationLabelsError::ProvisionedThroughputExceeded(ref cause) => cause,
            DetectModerationLabelsError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DetectText
#[derive(Debug, PartialEq)]
pub enum DetectTextError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl DetectTextError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DetectTextError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DetectTextError::AccessDenied(err.msg))
                }
                "ImageTooLargeException" => {
                    return RusotoError::Service(DetectTextError::ImageTooLarge(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(DetectTextError::InternalServerError(err.msg))
                }
                "InvalidImageFormatException" => {
                    return RusotoError::Service(DetectTextError::InvalidImageFormat(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DetectTextError::InvalidParameter(err.msg))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(DetectTextError::InvalidS3Object(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(DetectTextError::ProvisionedThroughputExceeded(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DetectTextError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DetectTextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DetectTextError {
    fn description(&self) -> &str {
        match *self {
            DetectTextError::AccessDenied(ref cause) => cause,
            DetectTextError::ImageTooLarge(ref cause) => cause,
            DetectTextError::InternalServerError(ref cause) => cause,
            DetectTextError::InvalidImageFormat(ref cause) => cause,
            DetectTextError::InvalidParameter(ref cause) => cause,
            DetectTextError::InvalidS3Object(ref cause) => cause,
            DetectTextError::ProvisionedThroughputExceeded(ref cause) => cause,
            DetectTextError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCelebrityInfo
#[derive(Debug, PartialEq)]
pub enum GetCelebrityInfoError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl GetCelebrityInfoError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCelebrityInfoError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetCelebrityInfoError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetCelebrityInfoError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetCelebrityInfoError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        GetCelebrityInfoError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetCelebrityInfoError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetCelebrityInfoError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetCelebrityInfoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCelebrityInfoError {
    fn description(&self) -> &str {
        match *self {
            GetCelebrityInfoError::AccessDenied(ref cause) => cause,
            GetCelebrityInfoError::InternalServerError(ref cause) => cause,
            GetCelebrityInfoError::InvalidParameter(ref cause) => cause,
            GetCelebrityInfoError::ProvisionedThroughputExceeded(ref cause) => cause,
            GetCelebrityInfoError::ResourceNotFound(ref cause) => cause,
            GetCelebrityInfoError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCelebrityRecognition
#[derive(Debug, PartialEq)]
pub enum GetCelebrityRecognitionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl GetCelebrityRecognitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCelebrityRecognitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetCelebrityRecognitionError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetCelebrityRecognitionError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(
                        GetCelebrityRecognitionError::InvalidPaginationToken(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetCelebrityRecognitionError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        GetCelebrityRecognitionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetCelebrityRecognitionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetCelebrityRecognitionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetCelebrityRecognitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCelebrityRecognitionError {
    fn description(&self) -> &str {
        match *self {
            GetCelebrityRecognitionError::AccessDenied(ref cause) => cause,
            GetCelebrityRecognitionError::InternalServerError(ref cause) => cause,
            GetCelebrityRecognitionError::InvalidPaginationToken(ref cause) => cause,
            GetCelebrityRecognitionError::InvalidParameter(ref cause) => cause,
            GetCelebrityRecognitionError::ProvisionedThroughputExceeded(ref cause) => cause,
            GetCelebrityRecognitionError::ResourceNotFound(ref cause) => cause,
            GetCelebrityRecognitionError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by GetContentModeration
#[derive(Debug, PartialEq)]
pub enum GetContentModerationError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl GetContentModerationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetContentModerationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetContentModerationError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetContentModerationError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(GetContentModerationError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetContentModerationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        GetContentModerationError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetContentModerationError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetContentModerationError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetContentModerationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetContentModerationError {
    fn description(&self) -> &str {
        match *self {
            GetContentModerationError::AccessDenied(ref cause) => cause,
            GetContentModerationError::InternalServerError(ref cause) => cause,
            GetContentModerationError::InvalidPaginationToken(ref cause) => cause,
            GetContentModerationError::InvalidParameter(ref cause) => cause,
            GetContentModerationError::ProvisionedThroughputExceeded(ref cause) => cause,
            GetContentModerationError::ResourceNotFound(ref cause) => cause,
            GetContentModerationError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by GetFaceDetection
#[derive(Debug, PartialEq)]
pub enum GetFaceDetectionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl GetFaceDetectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFaceDetectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetFaceDetectionError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetFaceDetectionError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(GetFaceDetectionError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetFaceDetectionError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        GetFaceDetectionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetFaceDetectionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetFaceDetectionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetFaceDetectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFaceDetectionError {
    fn description(&self) -> &str {
        match *self {
            GetFaceDetectionError::AccessDenied(ref cause) => cause,
            GetFaceDetectionError::InternalServerError(ref cause) => cause,
            GetFaceDetectionError::InvalidPaginationToken(ref cause) => cause,
            GetFaceDetectionError::InvalidParameter(ref cause) => cause,
            GetFaceDetectionError::ProvisionedThroughputExceeded(ref cause) => cause,
            GetFaceDetectionError::ResourceNotFound(ref cause) => cause,
            GetFaceDetectionError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by GetFaceSearch
#[derive(Debug, PartialEq)]
pub enum GetFaceSearchError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl GetFaceSearchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFaceSearchError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetFaceSearchError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetFaceSearchError::InternalServerError(err.msg))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(GetFaceSearchError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetFaceSearchError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(GetFaceSearchError::ProvisionedThroughputExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetFaceSearchError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetFaceSearchError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetFaceSearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFaceSearchError {
    fn description(&self) -> &str {
        match *self {
            GetFaceSearchError::AccessDenied(ref cause) => cause,
            GetFaceSearchError::InternalServerError(ref cause) => cause,
            GetFaceSearchError::InvalidPaginationToken(ref cause) => cause,
            GetFaceSearchError::InvalidParameter(ref cause) => cause,
            GetFaceSearchError::ProvisionedThroughputExceeded(ref cause) => cause,
            GetFaceSearchError::ResourceNotFound(ref cause) => cause,
            GetFaceSearchError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by GetLabelDetection
#[derive(Debug, PartialEq)]
pub enum GetLabelDetectionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl GetLabelDetectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLabelDetectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetLabelDetectionError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetLabelDetectionError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(GetLabelDetectionError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetLabelDetectionError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        GetLabelDetectionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetLabelDetectionError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetLabelDetectionError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetLabelDetectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLabelDetectionError {
    fn description(&self) -> &str {
        match *self {
            GetLabelDetectionError::AccessDenied(ref cause) => cause,
            GetLabelDetectionError::InternalServerError(ref cause) => cause,
            GetLabelDetectionError::InvalidPaginationToken(ref cause) => cause,
            GetLabelDetectionError::InvalidParameter(ref cause) => cause,
            GetLabelDetectionError::ProvisionedThroughputExceeded(ref cause) => cause,
            GetLabelDetectionError::ResourceNotFound(ref cause) => cause,
            GetLabelDetectionError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPersonTracking
#[derive(Debug, PartialEq)]
pub enum GetPersonTrackingError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl GetPersonTrackingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPersonTrackingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetPersonTrackingError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(GetPersonTrackingError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(GetPersonTrackingError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetPersonTrackingError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        GetPersonTrackingError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetPersonTrackingError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetPersonTrackingError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetPersonTrackingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPersonTrackingError {
    fn description(&self) -> &str {
        match *self {
            GetPersonTrackingError::AccessDenied(ref cause) => cause,
            GetPersonTrackingError::InternalServerError(ref cause) => cause,
            GetPersonTrackingError::InvalidPaginationToken(ref cause) => cause,
            GetPersonTrackingError::InvalidParameter(ref cause) => cause,
            GetPersonTrackingError::ProvisionedThroughputExceeded(ref cause) => cause,
            GetPersonTrackingError::ResourceNotFound(ref cause) => cause,
            GetPersonTrackingError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by IndexFaces
#[derive(Debug, PartialEq)]
pub enum IndexFacesError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl IndexFacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<IndexFacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(IndexFacesError::AccessDenied(err.msg))
                }
                "ImageTooLargeException" => {
                    return RusotoError::Service(IndexFacesError::ImageTooLarge(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(IndexFacesError::InternalServerError(err.msg))
                }
                "InvalidImageFormatException" => {
                    return RusotoError::Service(IndexFacesError::InvalidImageFormat(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(IndexFacesError::InvalidParameter(err.msg))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(IndexFacesError::InvalidS3Object(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(IndexFacesError::ProvisionedThroughputExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(IndexFacesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(IndexFacesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for IndexFacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for IndexFacesError {
    fn description(&self) -> &str {
        match *self {
            IndexFacesError::AccessDenied(ref cause) => cause,
            IndexFacesError::ImageTooLarge(ref cause) => cause,
            IndexFacesError::InternalServerError(ref cause) => cause,
            IndexFacesError::InvalidImageFormat(ref cause) => cause,
            IndexFacesError::InvalidParameter(ref cause) => cause,
            IndexFacesError::InvalidS3Object(ref cause) => cause,
            IndexFacesError::ProvisionedThroughputExceeded(ref cause) => cause,
            IndexFacesError::ResourceNotFound(ref cause) => cause,
            IndexFacesError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by ListCollections
#[derive(Debug, PartialEq)]
pub enum ListCollectionsError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl ListCollectionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListCollectionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListCollectionsError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(ListCollectionsError::InternalServerError(err.msg))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(ListCollectionsError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListCollectionsError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        ListCollectionsError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListCollectionsError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListCollectionsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListCollectionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListCollectionsError {
    fn description(&self) -> &str {
        match *self {
            ListCollectionsError::AccessDenied(ref cause) => cause,
            ListCollectionsError::InternalServerError(ref cause) => cause,
            ListCollectionsError::InvalidPaginationToken(ref cause) => cause,
            ListCollectionsError::InvalidParameter(ref cause) => cause,
            ListCollectionsError::ProvisionedThroughputExceeded(ref cause) => cause,
            ListCollectionsError::ResourceNotFound(ref cause) => cause,
            ListCollectionsError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by ListFaces
#[derive(Debug, PartialEq)]
pub enum ListFacesError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl ListFacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListFacesError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(ListFacesError::InternalServerError(err.msg))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(ListFacesError::InvalidPaginationToken(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListFacesError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(ListFacesError::ProvisionedThroughputExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListFacesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListFacesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListFacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListFacesError {
    fn description(&self) -> &str {
        match *self {
            ListFacesError::AccessDenied(ref cause) => cause,
            ListFacesError::InternalServerError(ref cause) => cause,
            ListFacesError::InvalidPaginationToken(ref cause) => cause,
            ListFacesError::InvalidParameter(ref cause) => cause,
            ListFacesError::ProvisionedThroughputExceeded(ref cause) => cause,
            ListFacesError::ResourceNotFound(ref cause) => cause,
            ListFacesError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by ListStreamProcessors
#[derive(Debug, PartialEq)]
pub enum ListStreamProcessorsError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Pagination token in the request is not valid.</p>
    InvalidPaginationToken(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl ListStreamProcessorsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListStreamProcessorsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListStreamProcessorsError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(ListStreamProcessorsError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidPaginationTokenException" => {
                    return RusotoError::Service(ListStreamProcessorsError::InvalidPaginationToken(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListStreamProcessorsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        ListStreamProcessorsError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListStreamProcessorsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListStreamProcessorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListStreamProcessorsError {
    fn description(&self) -> &str {
        match *self {
            ListStreamProcessorsError::AccessDenied(ref cause) => cause,
            ListStreamProcessorsError::InternalServerError(ref cause) => cause,
            ListStreamProcessorsError::InvalidPaginationToken(ref cause) => cause,
            ListStreamProcessorsError::InvalidParameter(ref cause) => cause,
            ListStreamProcessorsError::ProvisionedThroughputExceeded(ref cause) => cause,
            ListStreamProcessorsError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by RecognizeCelebrities
#[derive(Debug, PartialEq)]
pub enum RecognizeCelebritiesError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl RecognizeCelebritiesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RecognizeCelebritiesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(RecognizeCelebritiesError::AccessDenied(err.msg))
                }
                "ImageTooLargeException" => {
                    return RusotoError::Service(RecognizeCelebritiesError::ImageTooLarge(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(RecognizeCelebritiesError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidImageFormatException" => {
                    return RusotoError::Service(RecognizeCelebritiesError::InvalidImageFormat(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RecognizeCelebritiesError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(RecognizeCelebritiesError::InvalidS3Object(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        RecognizeCelebritiesError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(RecognizeCelebritiesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RecognizeCelebritiesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RecognizeCelebritiesError {
    fn description(&self) -> &str {
        match *self {
            RecognizeCelebritiesError::AccessDenied(ref cause) => cause,
            RecognizeCelebritiesError::ImageTooLarge(ref cause) => cause,
            RecognizeCelebritiesError::InternalServerError(ref cause) => cause,
            RecognizeCelebritiesError::InvalidImageFormat(ref cause) => cause,
            RecognizeCelebritiesError::InvalidParameter(ref cause) => cause,
            RecognizeCelebritiesError::InvalidS3Object(ref cause) => cause,
            RecognizeCelebritiesError::ProvisionedThroughputExceeded(ref cause) => cause,
            RecognizeCelebritiesError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by SearchFaces
#[derive(Debug, PartialEq)]
pub enum SearchFacesError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl SearchFacesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchFacesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(SearchFacesError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(SearchFacesError::InternalServerError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(SearchFacesError::InvalidParameter(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(SearchFacesError::ProvisionedThroughputExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SearchFacesError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SearchFacesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SearchFacesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchFacesError {
    fn description(&self) -> &str {
        match *self {
            SearchFacesError::AccessDenied(ref cause) => cause,
            SearchFacesError::InternalServerError(ref cause) => cause,
            SearchFacesError::InvalidParameter(ref cause) => cause,
            SearchFacesError::ProvisionedThroughputExceeded(ref cause) => cause,
            SearchFacesError::ResourceNotFound(ref cause) => cause,
            SearchFacesError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by SearchFacesByImage
#[derive(Debug, PartialEq)]
pub enum SearchFacesByImageError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>The input image size exceeds the allowed limit. For more information, see Limits in Amazon Rekognition in the Amazon Rekognition Developer Guide. </p>
    ImageTooLarge(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>The provided image format is not supported. </p>
    InvalidImageFormat(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl SearchFacesByImageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SearchFacesByImageError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(SearchFacesByImageError::AccessDenied(err.msg))
                }
                "ImageTooLargeException" => {
                    return RusotoError::Service(SearchFacesByImageError::ImageTooLarge(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(SearchFacesByImageError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidImageFormatException" => {
                    return RusotoError::Service(SearchFacesByImageError::InvalidImageFormat(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(SearchFacesByImageError::InvalidParameter(err.msg))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(SearchFacesByImageError::InvalidS3Object(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        SearchFacesByImageError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SearchFacesByImageError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SearchFacesByImageError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SearchFacesByImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SearchFacesByImageError {
    fn description(&self) -> &str {
        match *self {
            SearchFacesByImageError::AccessDenied(ref cause) => cause,
            SearchFacesByImageError::ImageTooLarge(ref cause) => cause,
            SearchFacesByImageError::InternalServerError(ref cause) => cause,
            SearchFacesByImageError::InvalidImageFormat(ref cause) => cause,
            SearchFacesByImageError::InvalidParameter(ref cause) => cause,
            SearchFacesByImageError::InvalidS3Object(ref cause) => cause,
            SearchFacesByImageError::ProvisionedThroughputExceeded(ref cause) => cause,
            SearchFacesByImageError::ResourceNotFound(ref cause) => cause,
            SearchFacesByImageError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by StartCelebrityRecognition
#[derive(Debug, PartialEq)]
pub enum StartCelebrityRecognitionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>A <code>ClientRequestToken</code> input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// <p>The file size or duration of the supplied media is too large. The maximum file size is 8GB. The maximum duration is 2 hours. </p>
    VideoTooLarge(String),
}

impl StartCelebrityRecognitionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartCelebrityRecognitionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartCelebrityRecognitionError::AccessDenied(
                        err.msg,
                    ))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        StartCelebrityRecognitionError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        StartCelebrityRecognitionError::InternalServerError(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartCelebrityRecognitionError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(StartCelebrityRecognitionError::InvalidS3Object(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartCelebrityRecognitionError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        StartCelebrityRecognitionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartCelebrityRecognitionError::Throttling(
                        err.msg,
                    ))
                }
                "VideoTooLargeException" => {
                    return RusotoError::Service(StartCelebrityRecognitionError::VideoTooLarge(
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
impl fmt::Display for StartCelebrityRecognitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartCelebrityRecognitionError {
    fn description(&self) -> &str {
        match *self {
            StartCelebrityRecognitionError::AccessDenied(ref cause) => cause,
            StartCelebrityRecognitionError::IdempotentParameterMismatch(ref cause) => cause,
            StartCelebrityRecognitionError::InternalServerError(ref cause) => cause,
            StartCelebrityRecognitionError::InvalidParameter(ref cause) => cause,
            StartCelebrityRecognitionError::InvalidS3Object(ref cause) => cause,
            StartCelebrityRecognitionError::LimitExceeded(ref cause) => cause,
            StartCelebrityRecognitionError::ProvisionedThroughputExceeded(ref cause) => cause,
            StartCelebrityRecognitionError::Throttling(ref cause) => cause,
            StartCelebrityRecognitionError::VideoTooLarge(ref cause) => cause,
        }
    }
}
/// Errors returned by StartContentModeration
#[derive(Debug, PartialEq)]
pub enum StartContentModerationError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>A <code>ClientRequestToken</code> input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// <p>The file size or duration of the supplied media is too large. The maximum file size is 8GB. The maximum duration is 2 hours. </p>
    VideoTooLarge(String),
}

impl StartContentModerationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartContentModerationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartContentModerationError::AccessDenied(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        StartContentModerationError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(StartContentModerationError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartContentModerationError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(StartContentModerationError::InvalidS3Object(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartContentModerationError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        StartContentModerationError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartContentModerationError::Throttling(err.msg))
                }
                "VideoTooLargeException" => {
                    return RusotoError::Service(StartContentModerationError::VideoTooLarge(
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
impl fmt::Display for StartContentModerationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartContentModerationError {
    fn description(&self) -> &str {
        match *self {
            StartContentModerationError::AccessDenied(ref cause) => cause,
            StartContentModerationError::IdempotentParameterMismatch(ref cause) => cause,
            StartContentModerationError::InternalServerError(ref cause) => cause,
            StartContentModerationError::InvalidParameter(ref cause) => cause,
            StartContentModerationError::InvalidS3Object(ref cause) => cause,
            StartContentModerationError::LimitExceeded(ref cause) => cause,
            StartContentModerationError::ProvisionedThroughputExceeded(ref cause) => cause,
            StartContentModerationError::Throttling(ref cause) => cause,
            StartContentModerationError::VideoTooLarge(ref cause) => cause,
        }
    }
}
/// Errors returned by StartFaceDetection
#[derive(Debug, PartialEq)]
pub enum StartFaceDetectionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>A <code>ClientRequestToken</code> input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// <p>The file size or duration of the supplied media is too large. The maximum file size is 8GB. The maximum duration is 2 hours. </p>
    VideoTooLarge(String),
}

impl StartFaceDetectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartFaceDetectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartFaceDetectionError::AccessDenied(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        StartFaceDetectionError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(StartFaceDetectionError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartFaceDetectionError::InvalidParameter(err.msg))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(StartFaceDetectionError::InvalidS3Object(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartFaceDetectionError::LimitExceeded(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        StartFaceDetectionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartFaceDetectionError::Throttling(err.msg))
                }
                "VideoTooLargeException" => {
                    return RusotoError::Service(StartFaceDetectionError::VideoTooLarge(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartFaceDetectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartFaceDetectionError {
    fn description(&self) -> &str {
        match *self {
            StartFaceDetectionError::AccessDenied(ref cause) => cause,
            StartFaceDetectionError::IdempotentParameterMismatch(ref cause) => cause,
            StartFaceDetectionError::InternalServerError(ref cause) => cause,
            StartFaceDetectionError::InvalidParameter(ref cause) => cause,
            StartFaceDetectionError::InvalidS3Object(ref cause) => cause,
            StartFaceDetectionError::LimitExceeded(ref cause) => cause,
            StartFaceDetectionError::ProvisionedThroughputExceeded(ref cause) => cause,
            StartFaceDetectionError::Throttling(ref cause) => cause,
            StartFaceDetectionError::VideoTooLarge(ref cause) => cause,
        }
    }
}
/// Errors returned by StartFaceSearch
#[derive(Debug, PartialEq)]
pub enum StartFaceSearchError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>A <code>ClientRequestToken</code> input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// <p>The file size or duration of the supplied media is too large. The maximum file size is 8GB. The maximum duration is 2 hours. </p>
    VideoTooLarge(String),
}

impl StartFaceSearchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartFaceSearchError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartFaceSearchError::AccessDenied(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(StartFaceSearchError::IdempotentParameterMismatch(
                        err.msg,
                    ))
                }
                "InternalServerError" => {
                    return RusotoError::Service(StartFaceSearchError::InternalServerError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartFaceSearchError::InvalidParameter(err.msg))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(StartFaceSearchError::InvalidS3Object(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartFaceSearchError::LimitExceeded(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        StartFaceSearchError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartFaceSearchError::ResourceNotFound(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartFaceSearchError::Throttling(err.msg))
                }
                "VideoTooLargeException" => {
                    return RusotoError::Service(StartFaceSearchError::VideoTooLarge(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartFaceSearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartFaceSearchError {
    fn description(&self) -> &str {
        match *self {
            StartFaceSearchError::AccessDenied(ref cause) => cause,
            StartFaceSearchError::IdempotentParameterMismatch(ref cause) => cause,
            StartFaceSearchError::InternalServerError(ref cause) => cause,
            StartFaceSearchError::InvalidParameter(ref cause) => cause,
            StartFaceSearchError::InvalidS3Object(ref cause) => cause,
            StartFaceSearchError::LimitExceeded(ref cause) => cause,
            StartFaceSearchError::ProvisionedThroughputExceeded(ref cause) => cause,
            StartFaceSearchError::ResourceNotFound(ref cause) => cause,
            StartFaceSearchError::Throttling(ref cause) => cause,
            StartFaceSearchError::VideoTooLarge(ref cause) => cause,
        }
    }
}
/// Errors returned by StartLabelDetection
#[derive(Debug, PartialEq)]
pub enum StartLabelDetectionError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>A <code>ClientRequestToken</code> input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// <p>The file size or duration of the supplied media is too large. The maximum file size is 8GB. The maximum duration is 2 hours. </p>
    VideoTooLarge(String),
}

impl StartLabelDetectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartLabelDetectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartLabelDetectionError::AccessDenied(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        StartLabelDetectionError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(StartLabelDetectionError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartLabelDetectionError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(StartLabelDetectionError::InvalidS3Object(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartLabelDetectionError::LimitExceeded(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        StartLabelDetectionError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartLabelDetectionError::Throttling(err.msg))
                }
                "VideoTooLargeException" => {
                    return RusotoError::Service(StartLabelDetectionError::VideoTooLarge(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartLabelDetectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartLabelDetectionError {
    fn description(&self) -> &str {
        match *self {
            StartLabelDetectionError::AccessDenied(ref cause) => cause,
            StartLabelDetectionError::IdempotentParameterMismatch(ref cause) => cause,
            StartLabelDetectionError::InternalServerError(ref cause) => cause,
            StartLabelDetectionError::InvalidParameter(ref cause) => cause,
            StartLabelDetectionError::InvalidS3Object(ref cause) => cause,
            StartLabelDetectionError::LimitExceeded(ref cause) => cause,
            StartLabelDetectionError::ProvisionedThroughputExceeded(ref cause) => cause,
            StartLabelDetectionError::Throttling(ref cause) => cause,
            StartLabelDetectionError::VideoTooLarge(ref cause) => cause,
        }
    }
}
/// Errors returned by StartPersonTracking
#[derive(Debug, PartialEq)]
pub enum StartPersonTrackingError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>A <code>ClientRequestToken</code> input parameter was reused with an operation, but at least one of the other input parameters is different from the previous call to the operation.</p>
    IdempotentParameterMismatch(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>Amazon Rekognition is unable to access the S3 object specified in the request.</p>
    InvalidS3Object(String),
    /// <p>An Amazon Rekognition service limit was exceeded. For example, if you start too many Amazon Rekognition Video jobs concurrently, calls to start operations (<code>StartLabelDetection</code>, for example) will raise a <code>LimitExceededException</code> exception (HTTP status code: 400) until the number of concurrently running jobs is below the Amazon Rekognition service limit. </p>
    LimitExceeded(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
    /// <p>The file size or duration of the supplied media is too large. The maximum file size is 8GB. The maximum duration is 2 hours. </p>
    VideoTooLarge(String),
}

impl StartPersonTrackingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartPersonTrackingError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartPersonTrackingError::AccessDenied(err.msg))
                }
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        StartPersonTrackingError::IdempotentParameterMismatch(err.msg),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(StartPersonTrackingError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartPersonTrackingError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidS3ObjectException" => {
                    return RusotoError::Service(StartPersonTrackingError::InvalidS3Object(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartPersonTrackingError::LimitExceeded(err.msg))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        StartPersonTrackingError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartPersonTrackingError::Throttling(err.msg))
                }
                "VideoTooLargeException" => {
                    return RusotoError::Service(StartPersonTrackingError::VideoTooLarge(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartPersonTrackingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartPersonTrackingError {
    fn description(&self) -> &str {
        match *self {
            StartPersonTrackingError::AccessDenied(ref cause) => cause,
            StartPersonTrackingError::IdempotentParameterMismatch(ref cause) => cause,
            StartPersonTrackingError::InternalServerError(ref cause) => cause,
            StartPersonTrackingError::InvalidParameter(ref cause) => cause,
            StartPersonTrackingError::InvalidS3Object(ref cause) => cause,
            StartPersonTrackingError::LimitExceeded(ref cause) => cause,
            StartPersonTrackingError::ProvisionedThroughputExceeded(ref cause) => cause,
            StartPersonTrackingError::Throttling(ref cause) => cause,
            StartPersonTrackingError::VideoTooLarge(ref cause) => cause,
        }
    }
}
/// Errors returned by StartStreamProcessor
#[derive(Debug, PartialEq)]
pub enum StartStreamProcessorError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p><p/></p>
    ResourceInUse(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl StartStreamProcessorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartStreamProcessorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartStreamProcessorError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(StartStreamProcessorError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartStreamProcessorError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        StartStreamProcessorError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(StartStreamProcessorError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartStreamProcessorError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartStreamProcessorError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartStreamProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartStreamProcessorError {
    fn description(&self) -> &str {
        match *self {
            StartStreamProcessorError::AccessDenied(ref cause) => cause,
            StartStreamProcessorError::InternalServerError(ref cause) => cause,
            StartStreamProcessorError::InvalidParameter(ref cause) => cause,
            StartStreamProcessorError::ProvisionedThroughputExceeded(ref cause) => cause,
            StartStreamProcessorError::ResourceInUse(ref cause) => cause,
            StartStreamProcessorError::ResourceNotFound(ref cause) => cause,
            StartStreamProcessorError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by StopStreamProcessor
#[derive(Debug, PartialEq)]
pub enum StopStreamProcessorError {
    /// <p>You are not authorized to perform the action.</p>
    AccessDenied(String),
    /// <p>Amazon Rekognition experienced a service issue. Try your call again.</p>
    InternalServerError(String),
    /// <p>Input parameter violated a constraint. Validate your parameter before calling the API operation again.</p>
    InvalidParameter(String),
    /// <p>The number of requests exceeded your throughput limit. If you want to increase this limit, contact Amazon Rekognition.</p>
    ProvisionedThroughputExceeded(String),
    /// <p><p/></p>
    ResourceInUse(String),
    /// <p>The collection specified in the request cannot be found.</p>
    ResourceNotFound(String),
    /// <p>Amazon Rekognition is temporarily unable to process the request. Try your call again.</p>
    Throttling(String),
}

impl StopStreamProcessorError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopStreamProcessorError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StopStreamProcessorError::AccessDenied(err.msg))
                }
                "InternalServerError" => {
                    return RusotoError::Service(StopStreamProcessorError::InternalServerError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StopStreamProcessorError::InvalidParameter(
                        err.msg,
                    ))
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        StopStreamProcessorError::ProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(StopStreamProcessorError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopStreamProcessorError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StopStreamProcessorError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopStreamProcessorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopStreamProcessorError {
    fn description(&self) -> &str {
        match *self {
            StopStreamProcessorError::AccessDenied(ref cause) => cause,
            StopStreamProcessorError::InternalServerError(ref cause) => cause,
            StopStreamProcessorError::InvalidParameter(ref cause) => cause,
            StopStreamProcessorError::ProvisionedThroughputExceeded(ref cause) => cause,
            StopStreamProcessorError::ResourceInUse(ref cause) => cause,
            StopStreamProcessorError::ResourceNotFound(ref cause) => cause,
            StopStreamProcessorError::Throttling(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Rekognition API. Amazon Rekognition clients implement this trait.
pub trait Rekognition {
    /// <p>Compares a face in the <i>source</i> input image with each of the 100 largest faces detected in the <i>target</i> input image. </p> <note> <p> If the source image contains multiple faces, the service detects the largest face and compares it with each face detected in the target image. </p> </note> <p>You pass the input and target images either as base64-encoded image bytes or as references to images in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes isn't supported. The image must be formatted as a PNG or JPEG file. </p> <p>In response, the operation returns an array of face matches ordered by similarity score in descending order. For each face match, the response provides a bounding box of the face, facial landmarks, pose details (pitch, role, and yaw), quality (brightness and sharpness), and confidence value (indicating the level of confidence that the bounding box contains a face). The response also provides a similarity score, which indicates how closely the faces match. </p> <note> <p>By default, only faces with a similarity score of greater than or equal to 80% are returned in the response. You can change this value by specifying the <code>SimilarityThreshold</code> parameter.</p> </note> <p> <code>CompareFaces</code> also returns an array of faces that don't match the source image. For each face, it returns a bounding box, confidence value, landmarks, pose details, and quality. The response also returns information about the face in the source image, including the bounding box of the face and confidence value.</p> <p>If the image doesn't contain Exif metadata, <code>CompareFaces</code> returns orientation information for the source and target images. Use these values to display the images with the correct image orientation.</p> <p>If no faces are detected in the source or target images, <code>CompareFaces</code> returns an <code>InvalidParameterException</code> error. </p> <note> <p> This is a stateless API operation. That is, data returned by this operation doesn't persist.</p> </note> <p>For an example, see Comparing Faces in Images in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:CompareFaces</code> action.</p>
    fn compare_faces(
        &self,
        input: CompareFacesRequest,
    ) -> RusotoFuture<CompareFacesResponse, CompareFacesError>;

    /// <p>Creates a collection in an AWS Region. You can add faces to the collection using the <a>IndexFaces</a> operation. </p> <p>For example, you might create collections, one for each of your application users. A user can then index faces using the <code>IndexFaces</code> operation and persist results in a specific collection. Then, a user can search the collection for faces in the user-specific container. </p> <p>When you create a collection, it is associated with the latest version of the face model version.</p> <note> <p>Collection names are case-sensitive.</p> </note> <p>This operation requires permissions to perform the <code>rekognition:CreateCollection</code> action.</p>
    fn create_collection(
        &self,
        input: CreateCollectionRequest,
    ) -> RusotoFuture<CreateCollectionResponse, CreateCollectionError>;

    /// <p>Creates an Amazon Rekognition stream processor that you can use to detect and recognize faces in a streaming video.</p> <p>Amazon Rekognition Video is a consumer of live video from Amazon Kinesis Video Streams. Amazon Rekognition Video sends analysis results to Amazon Kinesis Data Streams.</p> <p>You provide as input a Kinesis video stream (<code>Input</code>) and a Kinesis data stream (<code>Output</code>) stream. You also specify the face recognition criteria in <code>Settings</code>. For example, the collection containing faces that you want to recognize. Use <code>Name</code> to assign an identifier for the stream processor. You use <code>Name</code> to manage the stream processor. For example, you can start processing the source video by calling <a>StartStreamProcessor</a> with the <code>Name</code> field. </p> <p>After you have finished analyzing a streaming video, use <a>StopStreamProcessor</a> to stop processing. You can delete the stream processor by calling <a>DeleteStreamProcessor</a>.</p>
    fn create_stream_processor(
        &self,
        input: CreateStreamProcessorRequest,
    ) -> RusotoFuture<CreateStreamProcessorResponse, CreateStreamProcessorError>;

    /// <p>Deletes the specified collection. Note that this operation removes all faces in the collection. For an example, see <a>delete-collection-procedure</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:DeleteCollection</code> action.</p>
    fn delete_collection(
        &self,
        input: DeleteCollectionRequest,
    ) -> RusotoFuture<DeleteCollectionResponse, DeleteCollectionError>;

    /// <p>Deletes faces from a collection. You specify a collection ID and an array of face IDs to remove from the collection.</p> <p>This operation requires permissions to perform the <code>rekognition:DeleteFaces</code> action.</p>
    fn delete_faces(
        &self,
        input: DeleteFacesRequest,
    ) -> RusotoFuture<DeleteFacesResponse, DeleteFacesError>;

    /// <p>Deletes the stream processor identified by <code>Name</code>. You assign the value for <code>Name</code> when you create the stream processor with <a>CreateStreamProcessor</a>. You might not be able to use the same name for a stream processor for a few seconds after calling <code>DeleteStreamProcessor</code>.</p>
    fn delete_stream_processor(
        &self,
        input: DeleteStreamProcessorRequest,
    ) -> RusotoFuture<DeleteStreamProcessorResponse, DeleteStreamProcessorError>;

    /// <p>Describes the specified collection. You can use <code>DescribeCollection</code> to get information, such as the number of faces indexed into a collection and the version of the model used by the collection for face detection.</p> <p>For more information, see Describing a Collection in the Amazon Rekognition Developer Guide.</p>
    fn describe_collection(
        &self,
        input: DescribeCollectionRequest,
    ) -> RusotoFuture<DescribeCollectionResponse, DescribeCollectionError>;

    /// <p>Provides information about a stream processor created by <a>CreateStreamProcessor</a>. You can get information about the input and output streams, the input parameters for the face recognition being performed, and the current status of the stream processor.</p>
    fn describe_stream_processor(
        &self,
        input: DescribeStreamProcessorRequest,
    ) -> RusotoFuture<DescribeStreamProcessorResponse, DescribeStreamProcessorError>;

    /// <p>Detects faces within an image that is provided as input.</p> <p> <code>DetectFaces</code> detects the 100 largest faces in the image. For each face detected, the operation returns face details. These details include a bounding box of the face, a confidence value (that the bounding box contains a face), and a fixed set of attributes such as facial landmarks (for example, coordinates of eye and mouth), gender, presence of beard, sunglasses, and so on. </p> <p>The face-detection algorithm is most effective on frontal faces. For non-frontal or obscured faces, the algorithm might not detect the faces or might detect faces with lower confidence. </p> <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <note> <p>This is a stateless API operation. That is, the operation does not persist any data.</p> </note> <p>This operation requires permissions to perform the <code>rekognition:DetectFaces</code> action. </p>
    fn detect_faces(
        &self,
        input: DetectFacesRequest,
    ) -> RusotoFuture<DetectFacesResponse, DetectFacesError>;

    /// <p>Detects instances of real-world entities within an image (JPEG or PNG) provided as input. This includes objects like flower, tree, and table; events like wedding, graduation, and birthday party; and concepts like landscape, evening, and nature. </p> <p>For an example, see Analyzing Images Stored in an Amazon S3 Bucket in the Amazon Rekognition Developer Guide.</p> <note> <p> <code>DetectLabels</code> does not support the detection of activities. However, activity detection is supported for label detection in videos. For more information, see StartLabelDetection in the Amazon Rekognition Developer Guide.</p> </note> <p>You pass the input image as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p> For each object, scene, and concept the API returns one or more labels. Each label provides the object name, and the level of confidence that the image contains the object. For example, suppose the input image has a lighthouse, the sea, and a rock. The response includes all three labels, one for each object. </p> <p> <code>{Name: lighthouse, Confidence: 98.4629}</code> </p> <p> <code>{Name: rock,Confidence: 79.2097}</code> </p> <p> <code> {Name: sea,Confidence: 75.061}</code> </p> <p>In the preceding example, the operation returns one label for each of the three objects. The operation can also return multiple labels for the same object in the image. For example, if the input image shows a flower (for example, a tulip), the operation might return the following three labels. </p> <p> <code>{Name: flower,Confidence: 99.0562}</code> </p> <p> <code>{Name: plant,Confidence: 99.0562}</code> </p> <p> <code>{Name: tulip,Confidence: 99.0562}</code> </p> <p>In this example, the detection algorithm more precisely identifies the flower as a tulip.</p> <p>In response, the API returns an array of labels. In addition, the response also includes the orientation correction. Optionally, you can specify <code>MinConfidence</code> to control the confidence threshold for the labels returned. The default is 55%. You can also add the <code>MaxLabels</code> parameter to limit the number of labels returned. </p> <note> <p>If the object detected is a person, the operation doesn't provide the same facial details that the <a>DetectFaces</a> operation provides.</p> </note> <p> <code>DetectLabels</code> returns bounding boxes for instances of common object labels in an array of <a>Instance</a> objects. An <code>Instance</code> object contains a <a>BoundingBox</a> object, for the location of the label on the image. It also includes the confidence by which the bounding box was detected.</p> <p> <code>DetectLabels</code> also returns a hierarchical taxonomy of detected labels. For example, a detected car might be assigned the label <i>car</i>. The label <i>car</i> has two parent labels: <i>Vehicle</i> (its parent) and <i>Transportation</i> (its grandparent). The response returns the entire list of ancestors for a label. Each ancestor is a unique label in the response. In the previous example, <i>Car</i>, <i>Vehicle</i>, and <i>Transportation</i> are returned as unique labels in the response. </p> <p>This is a stateless API operation. That is, the operation does not persist any data.</p> <p>This operation requires permissions to perform the <code>rekognition:DetectLabels</code> action. </p>
    fn detect_labels(
        &self,
        input: DetectLabelsRequest,
    ) -> RusotoFuture<DetectLabelsResponse, DetectLabelsError>;

    /// <p>Detects explicit or suggestive adult content in a specified JPEG or PNG format image. Use <code>DetectModerationLabels</code> to moderate images depending on your requirements. For example, you might want to filter images that contain nudity, but not images containing suggestive content.</p> <p>To filter images, use the labels returned by <code>DetectModerationLabels</code> to determine which types of content are appropriate.</p> <p>For information about moderation labels, see Detecting Unsafe Content in the Amazon Rekognition Developer Guide.</p> <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p>
    fn detect_moderation_labels(
        &self,
        input: DetectModerationLabelsRequest,
    ) -> RusotoFuture<DetectModerationLabelsResponse, DetectModerationLabelsError>;

    /// <p>Detects text in the input image and converts it into machine-readable text.</p> <p>Pass the input image as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, you must pass it as a reference to an image in an Amazon S3 bucket. For the AWS CLI, passing image bytes is not supported. The image must be either a .png or .jpeg formatted file. </p> <p>The <code>DetectText</code> operation returns text in an array of <a>TextDetection</a> elements, <code>TextDetections</code>. Each <code>TextDetection</code> element provides information about a single word or line of text that was detected in the image. </p> <p>A word is one or more ISO basic latin script characters that are not separated by spaces. <code>DetectText</code> can detect up to 50 words in an image.</p> <p>A line is a string of equally spaced words. A line isn't necessarily a complete sentence. For example, a driver's license number is detected as a line. A line ends when there is no aligned text after it. Also, a line ends when there is a large gap between words, relative to the length of the words. This means, depending on the gap between words, Amazon Rekognition may detect multiple lines in text aligned in the same direction. Periods don't represent the end of a line. If a sentence spans multiple lines, the <code>DetectText</code> operation returns multiple lines.</p> <p>To determine whether a <code>TextDetection</code> element is a line of text or a word, use the <code>TextDetection</code> object <code>Type</code> field. </p> <p>To be detected, text must be within +/- 90 degrees orientation of the horizontal axis.</p> <p>For more information, see DetectText in the Amazon Rekognition Developer Guide.</p>
    fn detect_text(
        &self,
        input: DetectTextRequest,
    ) -> RusotoFuture<DetectTextResponse, DetectTextError>;

    /// <p>Gets the name and additional information about a celebrity based on his or her Amazon Rekognition ID. The additional information is returned as an array of URLs. If there is no additional information about the celebrity, this list is empty.</p> <p>For more information, see Recognizing Celebrities in an Image in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:GetCelebrityInfo</code> action. </p>
    fn get_celebrity_info(
        &self,
        input: GetCelebrityInfoRequest,
    ) -> RusotoFuture<GetCelebrityInfoResponse, GetCelebrityInfoError>;

    /// <p>Gets the celebrity recognition results for a Amazon Rekognition Video analysis started by <a>StartCelebrityRecognition</a>.</p> <p>Celebrity recognition in a video is an asynchronous operation. Analysis is started by a call to <a>StartCelebrityRecognition</a> which returns a job identifier (<code>JobId</code>). When the celebrity recognition operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartCelebrityRecognition</code>. To get the results of the celebrity recognition analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetCelebrityDetection</code> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartCelebrityDetection</code>. </p> <p>For more information, see Working With Stored Videos in the Amazon Rekognition Developer Guide.</p> <p> <code>GetCelebrityRecognition</code> returns detected celebrities and the time(s) they are detected in an array (<code>Celebrities</code>) of <a>CelebrityRecognition</a> objects. Each <code>CelebrityRecognition</code> contains information about the celebrity in a <a>CelebrityDetail</a> object and the time, <code>Timestamp</code>, the celebrity was detected. </p> <note> <p> <code>GetCelebrityRecognition</code> only returns the default facial attributes (<code>BoundingBox</code>, <code>Confidence</code>, <code>Landmarks</code>, <code>Pose</code>, and <code>Quality</code>). The other facial attributes listed in the <code>Face</code> object of the following response syntax are not returned. For more information, see FaceDetail in the Amazon Rekognition Developer Guide. </p> </note> <p>By default, the <code>Celebrities</code> array is sorted by time (milliseconds from the start of the video). You can also sort the array by celebrity by specifying the value <code>ID</code> in the <code>SortBy</code> input parameter.</p> <p>The <code>CelebrityDetail</code> object includes the celebrity identifer and additional information urls. If you don't store the additional information urls, you can get them later by calling <a>GetCelebrityInfo</a> with the celebrity identifer.</p> <p>No information is returned for faces not recognized as celebrities.</p> <p>Use MaxResults parameter to limit the number of labels returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetCelebrityDetection</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetCelebrityRecognition</code>.</p>
    fn get_celebrity_recognition(
        &self,
        input: GetCelebrityRecognitionRequest,
    ) -> RusotoFuture<GetCelebrityRecognitionResponse, GetCelebrityRecognitionError>;

    /// <p>Gets the content moderation analysis results for a Amazon Rekognition Video analysis started by <a>StartContentModeration</a>.</p> <p>Content moderation analysis of a video is an asynchronous operation. You start analysis by calling <a>StartContentModeration</a> which returns a job identifier (<code>JobId</code>). When analysis finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartContentModeration</code>. To get the results of the content moderation analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetContentModeration</code> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartContentModeration</code>. </p> <p>For more information, see Working with Stored Videos in the Amazon Rekognition Devlopers Guide.</p> <p> <code>GetContentModeration</code> returns detected content moderation labels, and the time they are detected, in an array, <code>ModerationLabels</code>, of <a>ContentModerationDetection</a> objects. </p> <p>By default, the moderated labels are returned sorted by time, in milliseconds from the start of the video. You can also sort them by moderated label by specifying <code>NAME</code> for the <code>SortBy</code> input parameter. </p> <p>Since video analysis can return a large number of results, use the <code>MaxResults</code> parameter to limit the number of labels returned in a single call to <code>GetContentModeration</code>. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetContentModeration</code> and populate the <code>NextToken</code> request parameter with the value of <code>NextToken</code> returned from the previous call to <code>GetContentModeration</code>.</p> <p>For more information, see Detecting Unsafe Content in the Amazon Rekognition Developer Guide.</p>
    fn get_content_moderation(
        &self,
        input: GetContentModerationRequest,
    ) -> RusotoFuture<GetContentModerationResponse, GetContentModerationError>;

    /// <p>Gets face detection results for a Amazon Rekognition Video analysis started by <a>StartFaceDetection</a>.</p> <p>Face detection with Amazon Rekognition Video is an asynchronous operation. You start face detection by calling <a>StartFaceDetection</a> which returns a job identifier (<code>JobId</code>). When the face detection operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartFaceDetection</code>. To get the results of the face detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetFaceDetection</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceDetection</code>.</p> <p> <code>GetFaceDetection</code> returns an array of detected faces (<code>Faces</code>) sorted by the time the faces were detected. </p> <p>Use MaxResults parameter to limit the number of labels returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetFaceDetection</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetFaceDetection</code>.</p>
    fn get_face_detection(
        &self,
        input: GetFaceDetectionRequest,
    ) -> RusotoFuture<GetFaceDetectionResponse, GetFaceDetectionError>;

    /// <p>Gets the face search results for Amazon Rekognition Video face search started by <a>StartFaceSearch</a>. The search returns faces in a collection that match the faces of persons detected in a video. It also includes the time(s) that faces are matched in the video.</p> <p>Face search in a video is an asynchronous operation. You start face search by calling to <a>StartFaceSearch</a> which returns a job identifier (<code>JobId</code>). When the search operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartFaceSearch</code>. To get the search results, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetFaceSearch</code> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceSearch</code>.</p> <p>For more information, see Searching Faces in a Collection in the Amazon Rekognition Developer Guide.</p> <p>The search results are retured in an array, <code>Persons</code>, of <a>PersonMatch</a> objects. Each<code>PersonMatch</code> element contains details about the matching faces in the input collection, person information (facial attributes, bounding boxes, and person identifer) for the matched person, and the time the person was matched in the video.</p> <note> <p> <code>GetFaceSearch</code> only returns the default facial attributes (<code>BoundingBox</code>, <code>Confidence</code>, <code>Landmarks</code>, <code>Pose</code>, and <code>Quality</code>). The other facial attributes listed in the <code>Face</code> object of the following response syntax are not returned. For more information, see FaceDetail in the Amazon Rekognition Developer Guide. </p> </note> <p>By default, the <code>Persons</code> array is sorted by the time, in milliseconds from the start of the video, persons are matched. You can also sort by persons by specifying <code>INDEX</code> for the <code>SORTBY</code> input parameter.</p>
    fn get_face_search(
        &self,
        input: GetFaceSearchRequest,
    ) -> RusotoFuture<GetFaceSearchResponse, GetFaceSearchError>;

    /// <p>Gets the label detection results of a Amazon Rekognition Video analysis started by <a>StartLabelDetection</a>. </p> <p>The label detection operation is started by a call to <a>StartLabelDetection</a> which returns a job identifier (<code>JobId</code>). When the label detection operation finishes, Amazon Rekognition publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartlabelDetection</code>. To get the results of the label detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetLabelDetection</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartLabelDetection</code>.</p> <p> <code>GetLabelDetection</code> returns an array of detected labels (<code>Labels</code>) sorted by the time the labels were detected. You can also sort by the label name by specifying <code>NAME</code> for the <code>SortBy</code> input parameter.</p> <p>The labels returned include the label name, the percentage confidence in the accuracy of the detected label, and the time the label was detected in the video.</p> <p>The returned labels also include bounding box information for common objects, a hierarchical taxonomy of detected labels, and the version of the label model used for detection.</p> <p>Use MaxResults parameter to limit the number of labels returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetlabelDetection</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetLabelDetection</code>.</p>
    fn get_label_detection(
        &self,
        input: GetLabelDetectionRequest,
    ) -> RusotoFuture<GetLabelDetectionResponse, GetLabelDetectionError>;

    /// <p>Gets the path tracking results of a Amazon Rekognition Video analysis started by <a>StartPersonTracking</a>.</p> <p>The person path tracking operation is started by a call to <code>StartPersonTracking</code> which returns a job identifier (<code>JobId</code>). When the operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartPersonTracking</code>.</p> <p>To get the results of the person path tracking operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetPersonTracking</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartPersonTracking</code>.</p> <p> <code>GetPersonTracking</code> returns an array, <code>Persons</code>, of tracked persons and the time(s) their paths were tracked in the video. </p> <note> <p> <code>GetPersonTracking</code> only returns the default facial attributes (<code>BoundingBox</code>, <code>Confidence</code>, <code>Landmarks</code>, <code>Pose</code>, and <code>Quality</code>). The other facial attributes listed in the <code>Face</code> object of the following response syntax are not returned. </p> <p>For more information, see FaceDetail in the Amazon Rekognition Developer Guide.</p> </note> <p>By default, the array is sorted by the time(s) a person's path is tracked in the video. You can sort by tracked persons by specifying <code>INDEX</code> for the <code>SortBy</code> input parameter.</p> <p>Use the <code>MaxResults</code> parameter to limit the number of items returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetPersonTracking</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetPersonTracking</code>.</p>
    fn get_person_tracking(
        &self,
        input: GetPersonTrackingRequest,
    ) -> RusotoFuture<GetPersonTrackingResponse, GetPersonTrackingError>;

    /// <p>Detects faces in the input image and adds them to the specified collection. </p> <p>Amazon Rekognition doesn't save the actual faces that are detected. Instead, the underlying detection algorithm first detects the faces in the input image. For each face, the algorithm extracts facial features into a feature vector, and stores it in the backend database. Amazon Rekognition uses feature vectors when it performs face match and search operations using the <a>SearchFaces</a> and <a>SearchFacesByImage</a> operations.</p> <p>For more information, see Adding Faces to a Collection in the Amazon Rekognition Developer Guide.</p> <p>To get the number of faces in a collection, call <a>DescribeCollection</a>. </p> <p>If you're using version 1.0 of the face detection model, <code>IndexFaces</code> indexes the 15 largest faces in the input image. Later versions of the face detection model index the 100 largest faces in the input image. </p> <p>If you're using version 4 or later of the face model, image orientation information is not returned in the <code>OrientationCorrection</code> field. </p> <p>To determine which version of the model you're using, call <a>DescribeCollection</a> and supply the collection ID. You can also get the model version from the value of <code>FaceModelVersion</code> in the response from <code>IndexFaces</code> </p> <p>For more information, see Model Versioning in the Amazon Rekognition Developer Guide.</p> <p>If you provide the optional <code>ExternalImageID</code> for the input image you provided, Amazon Rekognition associates this ID with all faces that it detects. When you call the <a>ListFaces</a> operation, the response returns the external ID. You can use this external image ID to create a client-side index to associate the faces with each image. You can then use the index to find all faces in an image.</p> <p>You can specify the maximum number of faces to index with the <code>MaxFaces</code> input parameter. This is useful when you want to index the largest faces in an image and don't want to index smaller faces, such as those belonging to people standing in the background.</p> <p>The <code>QualityFilter</code> input parameter allows you to filter out detected faces that don’t meet the required quality bar chosen by Amazon Rekognition. The quality bar is based on a variety of common use cases. By default, <code>IndexFaces</code> filters detected faces. You can also explicitly filter detected faces by specifying <code>AUTO</code> for the value of <code>QualityFilter</code>. If you do not want to filter detected faces, specify <code>NONE</code>. </p> <note> <p>To use quality filtering, you need a collection associated with version 3 of the face model. To get the version of the face model associated with a collection, call <a>DescribeCollection</a>. </p> </note> <p>Information about faces detected in an image, but not indexed, is returned in an array of <a>UnindexedFace</a> objects, <code>UnindexedFaces</code>. Faces aren't indexed for reasons such as:</p> <ul> <li> <p>The number of faces detected exceeds the value of the <code>MaxFaces</code> request parameter.</p> </li> <li> <p>The face is too small compared to the image dimensions.</p> </li> <li> <p>The face is too blurry.</p> </li> <li> <p>The image is too dark.</p> </li> <li> <p>The face has an extreme pose.</p> </li> </ul> <p>In response, the <code>IndexFaces</code> operation returns an array of metadata for all detected faces, <code>FaceRecords</code>. This includes: </p> <ul> <li> <p>The bounding box, <code>BoundingBox</code>, of the detected face. </p> </li> <li> <p>A confidence value, <code>Confidence</code>, which indicates the confidence that the bounding box contains a face.</p> </li> <li> <p>A face ID, <code>FaceId</code>, assigned by the service for each face that's detected and stored.</p> </li> <li> <p>An image ID, <code>ImageId</code>, assigned by the service for the input image.</p> </li> </ul> <p>If you request all facial attributes (by using the <code>detectionAttributes</code> parameter), Amazon Rekognition returns detailed facial attributes, such as facial landmarks (for example, location of eye and mouth) and other facial attributes like gender. If you provide the same image, specify the same collection, and use the same external ID in the <code>IndexFaces</code> operation, Amazon Rekognition doesn't save duplicate face metadata.</p> <p/> <p>The input image is passed either as base64-encoded image bytes, or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes isn't supported. The image must be formatted as a PNG or JPEG file. </p> <p>This operation requires permissions to perform the <code>rekognition:IndexFaces</code> action.</p>
    fn index_faces(
        &self,
        input: IndexFacesRequest,
    ) -> RusotoFuture<IndexFacesResponse, IndexFacesError>;

    /// <p>Returns list of collection IDs in your account. If the result is truncated, the response also provides a <code>NextToken</code> that you can use in the subsequent request to fetch the next set of collection IDs.</p> <p>For an example, see Listing Collections in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:ListCollections</code> action.</p>
    fn list_collections(
        &self,
        input: ListCollectionsRequest,
    ) -> RusotoFuture<ListCollectionsResponse, ListCollectionsError>;

    /// <p>Returns metadata for faces in the specified collection. This metadata includes information such as the bounding box coordinates, the confidence (that the bounding box contains a face), and face ID. For an example, see Listing Faces in a Collection in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:ListFaces</code> action.</p>
    fn list_faces(
        &self,
        input: ListFacesRequest,
    ) -> RusotoFuture<ListFacesResponse, ListFacesError>;

    /// <p>Gets a list of stream processors that you have created with <a>CreateStreamProcessor</a>. </p>
    fn list_stream_processors(
        &self,
        input: ListStreamProcessorsRequest,
    ) -> RusotoFuture<ListStreamProcessorsResponse, ListStreamProcessorsError>;

    /// <p>Returns an array of celebrities recognized in the input image. For more information, see Recognizing Celebrities in the Amazon Rekognition Developer Guide. </p> <p> <code>RecognizeCelebrities</code> returns the 100 largest faces in the image. It lists recognized celebrities in the <code>CelebrityFaces</code> array and unrecognized faces in the <code>UnrecognizedFaces</code> array. <code>RecognizeCelebrities</code> doesn't return celebrities whose faces aren't among the largest 100 faces in the image.</p> <p>For each celebrity recognized, <code>RecognizeCelebrities</code> returns a <code>Celebrity</code> object. The <code>Celebrity</code> object contains the celebrity name, ID, URL links to additional information, match confidence, and a <code>ComparedFace</code> object that you can use to locate the celebrity's face on the image.</p> <p>Amazon Rekognition doesn't retain information about which images a celebrity has been recognized in. Your application must store this information and use the <code>Celebrity</code> ID property as a unique identifier for the celebrity. If you don't store the celebrity name or additional information URLs returned by <code>RecognizeCelebrities</code>, you will need the ID to identify the celebrity in a call to the <a>GetCelebrityInfo</a> operation.</p> <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p>For an example, see Recognizing Celebrities in an Image in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:RecognizeCelebrities</code> operation.</p>
    fn recognize_celebrities(
        &self,
        input: RecognizeCelebritiesRequest,
    ) -> RusotoFuture<RecognizeCelebritiesResponse, RecognizeCelebritiesError>;

    /// <p>For a given input face ID, searches for matching faces in the collection the face belongs to. You get a face ID when you add a face to the collection using the <a>IndexFaces</a> operation. The operation compares the features of the input face with faces in the specified collection. </p> <note> <p>You can also search faces without indexing faces by using the <code>SearchFacesByImage</code> operation.</p> </note> <p> The operation response returns an array of faces that match, ordered by similarity score with the highest similarity first. More specifically, it is an array of metadata for each face match that is found. Along with the metadata, the response also includes a <code>confidence</code> value for each face match, indicating the confidence that the specific face matches the input face. </p> <p>For an example, see Searching for a Face Using Its Face ID in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:SearchFaces</code> action.</p>
    fn search_faces(
        &self,
        input: SearchFacesRequest,
    ) -> RusotoFuture<SearchFacesResponse, SearchFacesError>;

    /// <p>For a given input image, first detects the largest face in the image, and then searches the specified collection for matching faces. The operation compares the features of the input face with faces in the specified collection. </p> <note> <p>To search for all faces in an input image, you might first call the <a>IndexFaces</a> operation, and then use the face IDs returned in subsequent calls to the <a>SearchFaces</a> operation. </p> <p> You can also call the <code>DetectFaces</code> operation and use the bounding boxes in the response to make face crops, which then you can pass in to the <code>SearchFacesByImage</code> operation. </p> </note> <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p> The response returns an array of faces that match, ordered by similarity score with the highest similarity first. More specifically, it is an array of metadata for each face match found. Along with the metadata, the response also includes a <code>similarity</code> indicating how similar the face is to the input face. In the response, the operation also returns the bounding box (and a confidence level that the bounding box contains a face) of the face that Amazon Rekognition used for the input image. </p> <p>For an example, Searching for a Face Using an Image in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:SearchFacesByImage</code> action.</p>
    fn search_faces_by_image(
        &self,
        input: SearchFacesByImageRequest,
    ) -> RusotoFuture<SearchFacesByImageResponse, SearchFacesByImageError>;

    /// <p>Starts asynchronous recognition of celebrities in a stored video.</p> <p>Amazon Rekognition Video can detect celebrities in a video must be stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartCelebrityRecognition</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the analysis. When celebrity recognition analysis is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. To get the results of the celebrity recognition analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetCelebrityRecognition</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartCelebrityRecognition</code>. </p> <p>For more information, see Recognizing Celebrities in the Amazon Rekognition Developer Guide.</p>
    fn start_celebrity_recognition(
        &self,
        input: StartCelebrityRecognitionRequest,
    ) -> RusotoFuture<StartCelebrityRecognitionResponse, StartCelebrityRecognitionError>;

    /// <p> Starts asynchronous detection of explicit or suggestive adult content in a stored video.</p> <p>Amazon Rekognition Video can moderate content in a video stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartContentModeration</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the analysis. When content moderation analysis is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>.</p> <p>To get the results of the content moderation analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetContentModeration</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartContentModeration</code>. </p> <p>For more information, see Detecting Unsafe Content in the Amazon Rekognition Developer Guide.</p>
    fn start_content_moderation(
        &self,
        input: StartContentModerationRequest,
    ) -> RusotoFuture<StartContentModerationResponse, StartContentModerationError>;

    /// <p>Starts asynchronous detection of faces in a stored video.</p> <p>Amazon Rekognition Video can detect faces in a video stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartFaceDetection</code> returns a job identifier (<code>JobId</code>) that you use to get the results of the operation. When face detection is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. To get the results of the face detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetFaceDetection</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceDetection</code>.</p> <p>For more information, see Detecting Faces in a Stored Video in the Amazon Rekognition Developer Guide.</p>
    fn start_face_detection(
        &self,
        input: StartFaceDetectionRequest,
    ) -> RusotoFuture<StartFaceDetectionResponse, StartFaceDetectionError>;

    /// <p>Starts the asynchronous search for faces in a collection that match the faces of persons detected in a stored video.</p> <p>The video must be stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartFaceSearch</code> returns a job identifier (<code>JobId</code>) which you use to get the search results once the search has completed. When searching is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. To get the search results, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetFaceSearch</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceSearch</code>. For more information, see <a>procedure-person-search-videos</a>.</p>
    fn start_face_search(
        &self,
        input: StartFaceSearchRequest,
    ) -> RusotoFuture<StartFaceSearchResponse, StartFaceSearchError>;

    /// <p><p>Starts asynchronous detection of labels in a stored video.</p> <p>Amazon Rekognition Video can detect labels in a video. Labels are instances of real-world entities. This includes objects like flower, tree, and table; events like wedding, graduation, and birthday party; concepts like landscape, evening, and nature; and activities like a person getting out of a car or a person skiing.</p> <p>The video must be stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartLabelDetection</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the operation. When label detection is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>.</p> <p>To get the results of the label detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetLabelDetection</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartLabelDetection</code>.</p> <p/></p>
    fn start_label_detection(
        &self,
        input: StartLabelDetectionRequest,
    ) -> RusotoFuture<StartLabelDetectionResponse, StartLabelDetectionError>;

    /// <p>Starts the asynchronous tracking of a person's path in a stored video.</p> <p>Amazon Rekognition Video can track the path of people in a video stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartPersonTracking</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the operation. When label detection is finished, Amazon Rekognition publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. </p> <p>To get the results of the person detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetPersonTracking</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartPersonTracking</code>.</p>
    fn start_person_tracking(
        &self,
        input: StartPersonTrackingRequest,
    ) -> RusotoFuture<StartPersonTrackingResponse, StartPersonTrackingError>;

    /// <p>Starts processing a stream processor. You create a stream processor by calling <a>CreateStreamProcessor</a>. To tell <code>StartStreamProcessor</code> which stream processor to start, use the value of the <code>Name</code> field specified in the call to <code>CreateStreamProcessor</code>.</p>
    fn start_stream_processor(
        &self,
        input: StartStreamProcessorRequest,
    ) -> RusotoFuture<StartStreamProcessorResponse, StartStreamProcessorError>;

    /// <p>Stops a running stream processor that was created by <a>CreateStreamProcessor</a>.</p>
    fn stop_stream_processor(
        &self,
        input: StopStreamProcessorRequest,
    ) -> RusotoFuture<StopStreamProcessorResponse, StopStreamProcessorError>;
}
/// A client for the Amazon Rekognition API.
#[derive(Clone)]
pub struct RekognitionClient {
    client: Client,
    region: region::Region,
}

impl RekognitionClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> RekognitionClient {
        RekognitionClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> RekognitionClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        RekognitionClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Rekognition for RekognitionClient {
    /// <p>Compares a face in the <i>source</i> input image with each of the 100 largest faces detected in the <i>target</i> input image. </p> <note> <p> If the source image contains multiple faces, the service detects the largest face and compares it with each face detected in the target image. </p> </note> <p>You pass the input and target images either as base64-encoded image bytes or as references to images in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes isn't supported. The image must be formatted as a PNG or JPEG file. </p> <p>In response, the operation returns an array of face matches ordered by similarity score in descending order. For each face match, the response provides a bounding box of the face, facial landmarks, pose details (pitch, role, and yaw), quality (brightness and sharpness), and confidence value (indicating the level of confidence that the bounding box contains a face). The response also provides a similarity score, which indicates how closely the faces match. </p> <note> <p>By default, only faces with a similarity score of greater than or equal to 80% are returned in the response. You can change this value by specifying the <code>SimilarityThreshold</code> parameter.</p> </note> <p> <code>CompareFaces</code> also returns an array of faces that don't match the source image. For each face, it returns a bounding box, confidence value, landmarks, pose details, and quality. The response also returns information about the face in the source image, including the bounding box of the face and confidence value.</p> <p>If the image doesn't contain Exif metadata, <code>CompareFaces</code> returns orientation information for the source and target images. Use these values to display the images with the correct image orientation.</p> <p>If no faces are detected in the source or target images, <code>CompareFaces</code> returns an <code>InvalidParameterException</code> error. </p> <note> <p> This is a stateless API operation. That is, data returned by this operation doesn't persist.</p> </note> <p>For an example, see Comparing Faces in Images in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:CompareFaces</code> action.</p>
    fn compare_faces(
        &self,
        input: CompareFacesRequest,
    ) -> RusotoFuture<CompareFacesResponse, CompareFacesError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.CompareFaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CompareFacesResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CompareFacesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a collection in an AWS Region. You can add faces to the collection using the <a>IndexFaces</a> operation. </p> <p>For example, you might create collections, one for each of your application users. A user can then index faces using the <code>IndexFaces</code> operation and persist results in a specific collection. Then, a user can search the collection for faces in the user-specific container. </p> <p>When you create a collection, it is associated with the latest version of the face model version.</p> <note> <p>Collection names are case-sensitive.</p> </note> <p>This operation requires permissions to perform the <code>rekognition:CreateCollection</code> action.</p>
    fn create_collection(
        &self,
        input: CreateCollectionRequest,
    ) -> RusotoFuture<CreateCollectionResponse, CreateCollectionError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.CreateCollection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateCollectionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateCollectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an Amazon Rekognition stream processor that you can use to detect and recognize faces in a streaming video.</p> <p>Amazon Rekognition Video is a consumer of live video from Amazon Kinesis Video Streams. Amazon Rekognition Video sends analysis results to Amazon Kinesis Data Streams.</p> <p>You provide as input a Kinesis video stream (<code>Input</code>) and a Kinesis data stream (<code>Output</code>) stream. You also specify the face recognition criteria in <code>Settings</code>. For example, the collection containing faces that you want to recognize. Use <code>Name</code> to assign an identifier for the stream processor. You use <code>Name</code> to manage the stream processor. For example, you can start processing the source video by calling <a>StartStreamProcessor</a> with the <code>Name</code> field. </p> <p>After you have finished analyzing a streaming video, use <a>StopStreamProcessor</a> to stop processing. You can delete the stream processor by calling <a>DeleteStreamProcessor</a>.</p>
    fn create_stream_processor(
        &self,
        input: CreateStreamProcessorRequest,
    ) -> RusotoFuture<CreateStreamProcessorResponse, CreateStreamProcessorError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.CreateStreamProcessor");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateStreamProcessorResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateStreamProcessorError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes the specified collection. Note that this operation removes all faces in the collection. For an example, see <a>delete-collection-procedure</a>.</p> <p>This operation requires permissions to perform the <code>rekognition:DeleteCollection</code> action.</p>
    fn delete_collection(
        &self,
        input: DeleteCollectionRequest,
    ) -> RusotoFuture<DeleteCollectionResponse, DeleteCollectionError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DeleteCollection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteCollectionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteCollectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes faces from a collection. You specify a collection ID and an array of face IDs to remove from the collection.</p> <p>This operation requires permissions to perform the <code>rekognition:DeleteFaces</code> action.</p>
    fn delete_faces(
        &self,
        input: DeleteFacesRequest,
    ) -> RusotoFuture<DeleteFacesResponse, DeleteFacesError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DeleteFaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteFacesResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteFacesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the stream processor identified by <code>Name</code>. You assign the value for <code>Name</code> when you create the stream processor with <a>CreateStreamProcessor</a>. You might not be able to use the same name for a stream processor for a few seconds after calling <code>DeleteStreamProcessor</code>.</p>
    fn delete_stream_processor(
        &self,
        input: DeleteStreamProcessorRequest,
    ) -> RusotoFuture<DeleteStreamProcessorResponse, DeleteStreamProcessorError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DeleteStreamProcessor");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteStreamProcessorResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteStreamProcessorError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Describes the specified collection. You can use <code>DescribeCollection</code> to get information, such as the number of faces indexed into a collection and the version of the model used by the collection for face detection.</p> <p>For more information, see Describing a Collection in the Amazon Rekognition Developer Guide.</p>
    fn describe_collection(
        &self,
        input: DescribeCollectionRequest,
    ) -> RusotoFuture<DescribeCollectionResponse, DescribeCollectionError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DescribeCollection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeCollectionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeCollectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Provides information about a stream processor created by <a>CreateStreamProcessor</a>. You can get information about the input and output streams, the input parameters for the face recognition being performed, and the current status of the stream processor.</p>
    fn describe_stream_processor(
        &self,
        input: DescribeStreamProcessorRequest,
    ) -> RusotoFuture<DescribeStreamProcessorResponse, DescribeStreamProcessorError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DescribeStreamProcessor");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeStreamProcessorResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeStreamProcessorError::from_response(response))
                }))
            }
        })
    }

    /// <p>Detects faces within an image that is provided as input.</p> <p> <code>DetectFaces</code> detects the 100 largest faces in the image. For each face detected, the operation returns face details. These details include a bounding box of the face, a confidence value (that the bounding box contains a face), and a fixed set of attributes such as facial landmarks (for example, coordinates of eye and mouth), gender, presence of beard, sunglasses, and so on. </p> <p>The face-detection algorithm is most effective on frontal faces. For non-frontal or obscured faces, the algorithm might not detect the faces or might detect faces with lower confidence. </p> <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <note> <p>This is a stateless API operation. That is, the operation does not persist any data.</p> </note> <p>This operation requires permissions to perform the <code>rekognition:DetectFaces</code> action. </p>
    fn detect_faces(
        &self,
        input: DetectFacesRequest,
    ) -> RusotoFuture<DetectFacesResponse, DetectFacesError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DetectFaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DetectFacesResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DetectFacesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Detects instances of real-world entities within an image (JPEG or PNG) provided as input. This includes objects like flower, tree, and table; events like wedding, graduation, and birthday party; and concepts like landscape, evening, and nature. </p> <p>For an example, see Analyzing Images Stored in an Amazon S3 Bucket in the Amazon Rekognition Developer Guide.</p> <note> <p> <code>DetectLabels</code> does not support the detection of activities. However, activity detection is supported for label detection in videos. For more information, see StartLabelDetection in the Amazon Rekognition Developer Guide.</p> </note> <p>You pass the input image as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p> For each object, scene, and concept the API returns one or more labels. Each label provides the object name, and the level of confidence that the image contains the object. For example, suppose the input image has a lighthouse, the sea, and a rock. The response includes all three labels, one for each object. </p> <p> <code>{Name: lighthouse, Confidence: 98.4629}</code> </p> <p> <code>{Name: rock,Confidence: 79.2097}</code> </p> <p> <code> {Name: sea,Confidence: 75.061}</code> </p> <p>In the preceding example, the operation returns one label for each of the three objects. The operation can also return multiple labels for the same object in the image. For example, if the input image shows a flower (for example, a tulip), the operation might return the following three labels. </p> <p> <code>{Name: flower,Confidence: 99.0562}</code> </p> <p> <code>{Name: plant,Confidence: 99.0562}</code> </p> <p> <code>{Name: tulip,Confidence: 99.0562}</code> </p> <p>In this example, the detection algorithm more precisely identifies the flower as a tulip.</p> <p>In response, the API returns an array of labels. In addition, the response also includes the orientation correction. Optionally, you can specify <code>MinConfidence</code> to control the confidence threshold for the labels returned. The default is 55%. You can also add the <code>MaxLabels</code> parameter to limit the number of labels returned. </p> <note> <p>If the object detected is a person, the operation doesn't provide the same facial details that the <a>DetectFaces</a> operation provides.</p> </note> <p> <code>DetectLabels</code> returns bounding boxes for instances of common object labels in an array of <a>Instance</a> objects. An <code>Instance</code> object contains a <a>BoundingBox</a> object, for the location of the label on the image. It also includes the confidence by which the bounding box was detected.</p> <p> <code>DetectLabels</code> also returns a hierarchical taxonomy of detected labels. For example, a detected car might be assigned the label <i>car</i>. The label <i>car</i> has two parent labels: <i>Vehicle</i> (its parent) and <i>Transportation</i> (its grandparent). The response returns the entire list of ancestors for a label. Each ancestor is a unique label in the response. In the previous example, <i>Car</i>, <i>Vehicle</i>, and <i>Transportation</i> are returned as unique labels in the response. </p> <p>This is a stateless API operation. That is, the operation does not persist any data.</p> <p>This operation requires permissions to perform the <code>rekognition:DetectLabels</code> action. </p>
    fn detect_labels(
        &self,
        input: DetectLabelsRequest,
    ) -> RusotoFuture<DetectLabelsResponse, DetectLabelsError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DetectLabels");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DetectLabelsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DetectLabelsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Detects explicit or suggestive adult content in a specified JPEG or PNG format image. Use <code>DetectModerationLabels</code> to moderate images depending on your requirements. For example, you might want to filter images that contain nudity, but not images containing suggestive content.</p> <p>To filter images, use the labels returned by <code>DetectModerationLabels</code> to determine which types of content are appropriate.</p> <p>For information about moderation labels, see Detecting Unsafe Content in the Amazon Rekognition Developer Guide.</p> <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p>
    fn detect_moderation_labels(
        &self,
        input: DetectModerationLabelsRequest,
    ) -> RusotoFuture<DetectModerationLabelsResponse, DetectModerationLabelsError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DetectModerationLabels");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DetectModerationLabelsResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DetectModerationLabelsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Detects text in the input image and converts it into machine-readable text.</p> <p>Pass the input image as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, you must pass it as a reference to an image in an Amazon S3 bucket. For the AWS CLI, passing image bytes is not supported. The image must be either a .png or .jpeg formatted file. </p> <p>The <code>DetectText</code> operation returns text in an array of <a>TextDetection</a> elements, <code>TextDetections</code>. Each <code>TextDetection</code> element provides information about a single word or line of text that was detected in the image. </p> <p>A word is one or more ISO basic latin script characters that are not separated by spaces. <code>DetectText</code> can detect up to 50 words in an image.</p> <p>A line is a string of equally spaced words. A line isn't necessarily a complete sentence. For example, a driver's license number is detected as a line. A line ends when there is no aligned text after it. Also, a line ends when there is a large gap between words, relative to the length of the words. This means, depending on the gap between words, Amazon Rekognition may detect multiple lines in text aligned in the same direction. Periods don't represent the end of a line. If a sentence spans multiple lines, the <code>DetectText</code> operation returns multiple lines.</p> <p>To determine whether a <code>TextDetection</code> element is a line of text or a word, use the <code>TextDetection</code> object <code>Type</code> field. </p> <p>To be detected, text must be within +/- 90 degrees orientation of the horizontal axis.</p> <p>For more information, see DetectText in the Amazon Rekognition Developer Guide.</p>
    fn detect_text(
        &self,
        input: DetectTextRequest,
    ) -> RusotoFuture<DetectTextResponse, DetectTextError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.DetectText");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DetectTextResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DetectTextError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the name and additional information about a celebrity based on his or her Amazon Rekognition ID. The additional information is returned as an array of URLs. If there is no additional information about the celebrity, this list is empty.</p> <p>For more information, see Recognizing Celebrities in an Image in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:GetCelebrityInfo</code> action. </p>
    fn get_celebrity_info(
        &self,
        input: GetCelebrityInfoRequest,
    ) -> RusotoFuture<GetCelebrityInfoResponse, GetCelebrityInfoError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.GetCelebrityInfo");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCelebrityInfoResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetCelebrityInfoError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the celebrity recognition results for a Amazon Rekognition Video analysis started by <a>StartCelebrityRecognition</a>.</p> <p>Celebrity recognition in a video is an asynchronous operation. Analysis is started by a call to <a>StartCelebrityRecognition</a> which returns a job identifier (<code>JobId</code>). When the celebrity recognition operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartCelebrityRecognition</code>. To get the results of the celebrity recognition analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetCelebrityDetection</code> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartCelebrityDetection</code>. </p> <p>For more information, see Working With Stored Videos in the Amazon Rekognition Developer Guide.</p> <p> <code>GetCelebrityRecognition</code> returns detected celebrities and the time(s) they are detected in an array (<code>Celebrities</code>) of <a>CelebrityRecognition</a> objects. Each <code>CelebrityRecognition</code> contains information about the celebrity in a <a>CelebrityDetail</a> object and the time, <code>Timestamp</code>, the celebrity was detected. </p> <note> <p> <code>GetCelebrityRecognition</code> only returns the default facial attributes (<code>BoundingBox</code>, <code>Confidence</code>, <code>Landmarks</code>, <code>Pose</code>, and <code>Quality</code>). The other facial attributes listed in the <code>Face</code> object of the following response syntax are not returned. For more information, see FaceDetail in the Amazon Rekognition Developer Guide. </p> </note> <p>By default, the <code>Celebrities</code> array is sorted by time (milliseconds from the start of the video). You can also sort the array by celebrity by specifying the value <code>ID</code> in the <code>SortBy</code> input parameter.</p> <p>The <code>CelebrityDetail</code> object includes the celebrity identifer and additional information urls. If you don't store the additional information urls, you can get them later by calling <a>GetCelebrityInfo</a> with the celebrity identifer.</p> <p>No information is returned for faces not recognized as celebrities.</p> <p>Use MaxResults parameter to limit the number of labels returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetCelebrityDetection</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetCelebrityRecognition</code>.</p>
    fn get_celebrity_recognition(
        &self,
        input: GetCelebrityRecognitionRequest,
    ) -> RusotoFuture<GetCelebrityRecognitionResponse, GetCelebrityRecognitionError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.GetCelebrityRecognition");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetCelebrityRecognitionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetCelebrityRecognitionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets the content moderation analysis results for a Amazon Rekognition Video analysis started by <a>StartContentModeration</a>.</p> <p>Content moderation analysis of a video is an asynchronous operation. You start analysis by calling <a>StartContentModeration</a> which returns a job identifier (<code>JobId</code>). When analysis finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartContentModeration</code>. To get the results of the content moderation analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetContentModeration</code> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartContentModeration</code>. </p> <p>For more information, see Working with Stored Videos in the Amazon Rekognition Devlopers Guide.</p> <p> <code>GetContentModeration</code> returns detected content moderation labels, and the time they are detected, in an array, <code>ModerationLabels</code>, of <a>ContentModerationDetection</a> objects. </p> <p>By default, the moderated labels are returned sorted by time, in milliseconds from the start of the video. You can also sort them by moderated label by specifying <code>NAME</code> for the <code>SortBy</code> input parameter. </p> <p>Since video analysis can return a large number of results, use the <code>MaxResults</code> parameter to limit the number of labels returned in a single call to <code>GetContentModeration</code>. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetContentModeration</code> and populate the <code>NextToken</code> request parameter with the value of <code>NextToken</code> returned from the previous call to <code>GetContentModeration</code>.</p> <p>For more information, see Detecting Unsafe Content in the Amazon Rekognition Developer Guide.</p>
    fn get_content_moderation(
        &self,
        input: GetContentModerationRequest,
    ) -> RusotoFuture<GetContentModerationResponse, GetContentModerationError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.GetContentModeration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetContentModerationResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetContentModerationError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets face detection results for a Amazon Rekognition Video analysis started by <a>StartFaceDetection</a>.</p> <p>Face detection with Amazon Rekognition Video is an asynchronous operation. You start face detection by calling <a>StartFaceDetection</a> which returns a job identifier (<code>JobId</code>). When the face detection operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartFaceDetection</code>. To get the results of the face detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetFaceDetection</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceDetection</code>.</p> <p> <code>GetFaceDetection</code> returns an array of detected faces (<code>Faces</code>) sorted by the time the faces were detected. </p> <p>Use MaxResults parameter to limit the number of labels returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetFaceDetection</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetFaceDetection</code>.</p>
    fn get_face_detection(
        &self,
        input: GetFaceDetectionRequest,
    ) -> RusotoFuture<GetFaceDetectionResponse, GetFaceDetectionError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.GetFaceDetection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetFaceDetectionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetFaceDetectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the face search results for Amazon Rekognition Video face search started by <a>StartFaceSearch</a>. The search returns faces in a collection that match the faces of persons detected in a video. It also includes the time(s) that faces are matched in the video.</p> <p>Face search in a video is an asynchronous operation. You start face search by calling to <a>StartFaceSearch</a> which returns a job identifier (<code>JobId</code>). When the search operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartFaceSearch</code>. To get the search results, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetFaceSearch</code> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceSearch</code>.</p> <p>For more information, see Searching Faces in a Collection in the Amazon Rekognition Developer Guide.</p> <p>The search results are retured in an array, <code>Persons</code>, of <a>PersonMatch</a> objects. Each<code>PersonMatch</code> element contains details about the matching faces in the input collection, person information (facial attributes, bounding boxes, and person identifer) for the matched person, and the time the person was matched in the video.</p> <note> <p> <code>GetFaceSearch</code> only returns the default facial attributes (<code>BoundingBox</code>, <code>Confidence</code>, <code>Landmarks</code>, <code>Pose</code>, and <code>Quality</code>). The other facial attributes listed in the <code>Face</code> object of the following response syntax are not returned. For more information, see FaceDetail in the Amazon Rekognition Developer Guide. </p> </note> <p>By default, the <code>Persons</code> array is sorted by the time, in milliseconds from the start of the video, persons are matched. You can also sort by persons by specifying <code>INDEX</code> for the <code>SORTBY</code> input parameter.</p>
    fn get_face_search(
        &self,
        input: GetFaceSearchRequest,
    ) -> RusotoFuture<GetFaceSearchResponse, GetFaceSearchError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.GetFaceSearch");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetFaceSearchResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetFaceSearchError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the label detection results of a Amazon Rekognition Video analysis started by <a>StartLabelDetection</a>. </p> <p>The label detection operation is started by a call to <a>StartLabelDetection</a> which returns a job identifier (<code>JobId</code>). When the label detection operation finishes, Amazon Rekognition publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartlabelDetection</code>. To get the results of the label detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetLabelDetection</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartLabelDetection</code>.</p> <p> <code>GetLabelDetection</code> returns an array of detected labels (<code>Labels</code>) sorted by the time the labels were detected. You can also sort by the label name by specifying <code>NAME</code> for the <code>SortBy</code> input parameter.</p> <p>The labels returned include the label name, the percentage confidence in the accuracy of the detected label, and the time the label was detected in the video.</p> <p>The returned labels also include bounding box information for common objects, a hierarchical taxonomy of detected labels, and the version of the label model used for detection.</p> <p>Use MaxResults parameter to limit the number of labels returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetlabelDetection</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetLabelDetection</code>.</p>
    fn get_label_detection(
        &self,
        input: GetLabelDetectionRequest,
    ) -> RusotoFuture<GetLabelDetectionResponse, GetLabelDetectionError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.GetLabelDetection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetLabelDetectionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetLabelDetectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the path tracking results of a Amazon Rekognition Video analysis started by <a>StartPersonTracking</a>.</p> <p>The person path tracking operation is started by a call to <code>StartPersonTracking</code> which returns a job identifier (<code>JobId</code>). When the operation finishes, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic registered in the initial call to <code>StartPersonTracking</code>.</p> <p>To get the results of the person path tracking operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetPersonTracking</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartPersonTracking</code>.</p> <p> <code>GetPersonTracking</code> returns an array, <code>Persons</code>, of tracked persons and the time(s) their paths were tracked in the video. </p> <note> <p> <code>GetPersonTracking</code> only returns the default facial attributes (<code>BoundingBox</code>, <code>Confidence</code>, <code>Landmarks</code>, <code>Pose</code>, and <code>Quality</code>). The other facial attributes listed in the <code>Face</code> object of the following response syntax are not returned. </p> <p>For more information, see FaceDetail in the Amazon Rekognition Developer Guide.</p> </note> <p>By default, the array is sorted by the time(s) a person's path is tracked in the video. You can sort by tracked persons by specifying <code>INDEX</code> for the <code>SortBy</code> input parameter.</p> <p>Use the <code>MaxResults</code> parameter to limit the number of items returned. If there are more results than specified in <code>MaxResults</code>, the value of <code>NextToken</code> in the operation response contains a pagination token for getting the next set of results. To get the next page of results, call <code>GetPersonTracking</code> and populate the <code>NextToken</code> request parameter with the token value returned from the previous call to <code>GetPersonTracking</code>.</p>
    fn get_person_tracking(
        &self,
        input: GetPersonTrackingRequest,
    ) -> RusotoFuture<GetPersonTrackingResponse, GetPersonTrackingError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.GetPersonTracking");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetPersonTrackingResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetPersonTrackingError::from_response(response))),
                )
            }
        })
    }

    /// <p>Detects faces in the input image and adds them to the specified collection. </p> <p>Amazon Rekognition doesn't save the actual faces that are detected. Instead, the underlying detection algorithm first detects the faces in the input image. For each face, the algorithm extracts facial features into a feature vector, and stores it in the backend database. Amazon Rekognition uses feature vectors when it performs face match and search operations using the <a>SearchFaces</a> and <a>SearchFacesByImage</a> operations.</p> <p>For more information, see Adding Faces to a Collection in the Amazon Rekognition Developer Guide.</p> <p>To get the number of faces in a collection, call <a>DescribeCollection</a>. </p> <p>If you're using version 1.0 of the face detection model, <code>IndexFaces</code> indexes the 15 largest faces in the input image. Later versions of the face detection model index the 100 largest faces in the input image. </p> <p>If you're using version 4 or later of the face model, image orientation information is not returned in the <code>OrientationCorrection</code> field. </p> <p>To determine which version of the model you're using, call <a>DescribeCollection</a> and supply the collection ID. You can also get the model version from the value of <code>FaceModelVersion</code> in the response from <code>IndexFaces</code> </p> <p>For more information, see Model Versioning in the Amazon Rekognition Developer Guide.</p> <p>If you provide the optional <code>ExternalImageID</code> for the input image you provided, Amazon Rekognition associates this ID with all faces that it detects. When you call the <a>ListFaces</a> operation, the response returns the external ID. You can use this external image ID to create a client-side index to associate the faces with each image. You can then use the index to find all faces in an image.</p> <p>You can specify the maximum number of faces to index with the <code>MaxFaces</code> input parameter. This is useful when you want to index the largest faces in an image and don't want to index smaller faces, such as those belonging to people standing in the background.</p> <p>The <code>QualityFilter</code> input parameter allows you to filter out detected faces that don’t meet the required quality bar chosen by Amazon Rekognition. The quality bar is based on a variety of common use cases. By default, <code>IndexFaces</code> filters detected faces. You can also explicitly filter detected faces by specifying <code>AUTO</code> for the value of <code>QualityFilter</code>. If you do not want to filter detected faces, specify <code>NONE</code>. </p> <note> <p>To use quality filtering, you need a collection associated with version 3 of the face model. To get the version of the face model associated with a collection, call <a>DescribeCollection</a>. </p> </note> <p>Information about faces detected in an image, but not indexed, is returned in an array of <a>UnindexedFace</a> objects, <code>UnindexedFaces</code>. Faces aren't indexed for reasons such as:</p> <ul> <li> <p>The number of faces detected exceeds the value of the <code>MaxFaces</code> request parameter.</p> </li> <li> <p>The face is too small compared to the image dimensions.</p> </li> <li> <p>The face is too blurry.</p> </li> <li> <p>The image is too dark.</p> </li> <li> <p>The face has an extreme pose.</p> </li> </ul> <p>In response, the <code>IndexFaces</code> operation returns an array of metadata for all detected faces, <code>FaceRecords</code>. This includes: </p> <ul> <li> <p>The bounding box, <code>BoundingBox</code>, of the detected face. </p> </li> <li> <p>A confidence value, <code>Confidence</code>, which indicates the confidence that the bounding box contains a face.</p> </li> <li> <p>A face ID, <code>FaceId</code>, assigned by the service for each face that's detected and stored.</p> </li> <li> <p>An image ID, <code>ImageId</code>, assigned by the service for the input image.</p> </li> </ul> <p>If you request all facial attributes (by using the <code>detectionAttributes</code> parameter), Amazon Rekognition returns detailed facial attributes, such as facial landmarks (for example, location of eye and mouth) and other facial attributes like gender. If you provide the same image, specify the same collection, and use the same external ID in the <code>IndexFaces</code> operation, Amazon Rekognition doesn't save duplicate face metadata.</p> <p/> <p>The input image is passed either as base64-encoded image bytes, or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes isn't supported. The image must be formatted as a PNG or JPEG file. </p> <p>This operation requires permissions to perform the <code>rekognition:IndexFaces</code> action.</p>
    fn index_faces(
        &self,
        input: IndexFacesRequest,
    ) -> RusotoFuture<IndexFacesResponse, IndexFacesError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.IndexFaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<IndexFacesResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(IndexFacesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns list of collection IDs in your account. If the result is truncated, the response also provides a <code>NextToken</code> that you can use in the subsequent request to fetch the next set of collection IDs.</p> <p>For an example, see Listing Collections in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:ListCollections</code> action.</p>
    fn list_collections(
        &self,
        input: ListCollectionsRequest,
    ) -> RusotoFuture<ListCollectionsResponse, ListCollectionsError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.ListCollections");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListCollectionsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListCollectionsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns metadata for faces in the specified collection. This metadata includes information such as the bounding box coordinates, the confidence (that the bounding box contains a face), and face ID. For an example, see Listing Faces in a Collection in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:ListFaces</code> action.</p>
    fn list_faces(
        &self,
        input: ListFacesRequest,
    ) -> RusotoFuture<ListFacesResponse, ListFacesError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.ListFaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListFacesResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListFacesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets a list of stream processors that you have created with <a>CreateStreamProcessor</a>. </p>
    fn list_stream_processors(
        &self,
        input: ListStreamProcessorsRequest,
    ) -> RusotoFuture<ListStreamProcessorsResponse, ListStreamProcessorsError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.ListStreamProcessors");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListStreamProcessorsResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListStreamProcessorsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns an array of celebrities recognized in the input image. For more information, see Recognizing Celebrities in the Amazon Rekognition Developer Guide. </p> <p> <code>RecognizeCelebrities</code> returns the 100 largest faces in the image. It lists recognized celebrities in the <code>CelebrityFaces</code> array and unrecognized faces in the <code>UnrecognizedFaces</code> array. <code>RecognizeCelebrities</code> doesn't return celebrities whose faces aren't among the largest 100 faces in the image.</p> <p>For each celebrity recognized, <code>RecognizeCelebrities</code> returns a <code>Celebrity</code> object. The <code>Celebrity</code> object contains the celebrity name, ID, URL links to additional information, match confidence, and a <code>ComparedFace</code> object that you can use to locate the celebrity's face on the image.</p> <p>Amazon Rekognition doesn't retain information about which images a celebrity has been recognized in. Your application must store this information and use the <code>Celebrity</code> ID property as a unique identifier for the celebrity. If you don't store the celebrity name or additional information URLs returned by <code>RecognizeCelebrities</code>, you will need the ID to identify the celebrity in a call to the <a>GetCelebrityInfo</a> operation.</p> <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p>For an example, see Recognizing Celebrities in an Image in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:RecognizeCelebrities</code> operation.</p>
    fn recognize_celebrities(
        &self,
        input: RecognizeCelebritiesRequest,
    ) -> RusotoFuture<RecognizeCelebritiesResponse, RecognizeCelebritiesError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.RecognizeCelebrities");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RecognizeCelebritiesResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(RecognizeCelebritiesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>For a given input face ID, searches for matching faces in the collection the face belongs to. You get a face ID when you add a face to the collection using the <a>IndexFaces</a> operation. The operation compares the features of the input face with faces in the specified collection. </p> <note> <p>You can also search faces without indexing faces by using the <code>SearchFacesByImage</code> operation.</p> </note> <p> The operation response returns an array of faces that match, ordered by similarity score with the highest similarity first. More specifically, it is an array of metadata for each face match that is found. Along with the metadata, the response also includes a <code>confidence</code> value for each face match, indicating the confidence that the specific face matches the input face. </p> <p>For an example, see Searching for a Face Using Its Face ID in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:SearchFaces</code> action.</p>
    fn search_faces(
        &self,
        input: SearchFacesRequest,
    ) -> RusotoFuture<SearchFacesResponse, SearchFacesError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.SearchFaces");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<SearchFacesResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SearchFacesError::from_response(response))),
                )
            }
        })
    }

    /// <p>For a given input image, first detects the largest face in the image, and then searches the specified collection for matching faces. The operation compares the features of the input face with faces in the specified collection. </p> <note> <p>To search for all faces in an input image, you might first call the <a>IndexFaces</a> operation, and then use the face IDs returned in subsequent calls to the <a>SearchFaces</a> operation. </p> <p> You can also call the <code>DetectFaces</code> operation and use the bounding boxes in the response to make face crops, which then you can pass in to the <code>SearchFacesByImage</code> operation. </p> </note> <p>You pass the input image either as base64-encoded image bytes or as a reference to an image in an Amazon S3 bucket. If you use the AWS CLI to call Amazon Rekognition operations, passing image bytes is not supported. The image must be either a PNG or JPEG formatted file. </p> <p> The response returns an array of faces that match, ordered by similarity score with the highest similarity first. More specifically, it is an array of metadata for each face match found. Along with the metadata, the response also includes a <code>similarity</code> indicating how similar the face is to the input face. In the response, the operation also returns the bounding box (and a confidence level that the bounding box contains a face) of the face that Amazon Rekognition used for the input image. </p> <p>For an example, Searching for a Face Using an Image in the Amazon Rekognition Developer Guide.</p> <p>This operation requires permissions to perform the <code>rekognition:SearchFacesByImage</code> action.</p>
    fn search_faces_by_image(
        &self,
        input: SearchFacesByImageRequest,
    ) -> RusotoFuture<SearchFacesByImageResponse, SearchFacesByImageError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.SearchFacesByImage");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<SearchFacesByImageResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SearchFacesByImageError::from_response(response))),
                )
            }
        })
    }

    /// <p>Starts asynchronous recognition of celebrities in a stored video.</p> <p>Amazon Rekognition Video can detect celebrities in a video must be stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartCelebrityRecognition</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the analysis. When celebrity recognition analysis is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. To get the results of the celebrity recognition analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetCelebrityRecognition</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartCelebrityRecognition</code>. </p> <p>For more information, see Recognizing Celebrities in the Amazon Rekognition Developer Guide.</p>
    fn start_celebrity_recognition(
        &self,
        input: StartCelebrityRecognitionRequest,
    ) -> RusotoFuture<StartCelebrityRecognitionResponse, StartCelebrityRecognitionError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "RekognitionService.StartCelebrityRecognition",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartCelebrityRecognitionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartCelebrityRecognitionError::from_response(response))
                }))
            }
        })
    }

    /// <p> Starts asynchronous detection of explicit or suggestive adult content in a stored video.</p> <p>Amazon Rekognition Video can moderate content in a video stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartContentModeration</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the analysis. When content moderation analysis is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>.</p> <p>To get the results of the content moderation analysis, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetContentModeration</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartContentModeration</code>. </p> <p>For more information, see Detecting Unsafe Content in the Amazon Rekognition Developer Guide.</p>
    fn start_content_moderation(
        &self,
        input: StartContentModerationRequest,
    ) -> RusotoFuture<StartContentModerationResponse, StartContentModerationError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StartContentModeration");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartContentModerationResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartContentModerationError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Starts asynchronous detection of faces in a stored video.</p> <p>Amazon Rekognition Video can detect faces in a video stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartFaceDetection</code> returns a job identifier (<code>JobId</code>) that you use to get the results of the operation. When face detection is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. To get the results of the face detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetFaceDetection</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceDetection</code>.</p> <p>For more information, see Detecting Faces in a Stored Video in the Amazon Rekognition Developer Guide.</p>
    fn start_face_detection(
        &self,
        input: StartFaceDetectionRequest,
    ) -> RusotoFuture<StartFaceDetectionResponse, StartFaceDetectionError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StartFaceDetection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartFaceDetectionResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StartFaceDetectionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Starts the asynchronous search for faces in a collection that match the faces of persons detected in a stored video.</p> <p>The video must be stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartFaceSearch</code> returns a job identifier (<code>JobId</code>) which you use to get the search results once the search has completed. When searching is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. To get the search results, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetFaceSearch</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartFaceSearch</code>. For more information, see <a>procedure-person-search-videos</a>.</p>
    fn start_face_search(
        &self,
        input: StartFaceSearchRequest,
    ) -> RusotoFuture<StartFaceSearchResponse, StartFaceSearchError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StartFaceSearch");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartFaceSearchResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StartFaceSearchError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Starts asynchronous detection of labels in a stored video.</p> <p>Amazon Rekognition Video can detect labels in a video. Labels are instances of real-world entities. This includes objects like flower, tree, and table; events like wedding, graduation, and birthday party; concepts like landscape, evening, and nature; and activities like a person getting out of a car or a person skiing.</p> <p>The video must be stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartLabelDetection</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the operation. When label detection is finished, Amazon Rekognition Video publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>.</p> <p>To get the results of the label detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetLabelDetection</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartLabelDetection</code>.</p> <p/></p>
    fn start_label_detection(
        &self,
        input: StartLabelDetectionRequest,
    ) -> RusotoFuture<StartLabelDetectionResponse, StartLabelDetectionError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StartLabelDetection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartLabelDetectionResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartLabelDetectionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Starts the asynchronous tracking of a person's path in a stored video.</p> <p>Amazon Rekognition Video can track the path of people in a video stored in an Amazon S3 bucket. Use <a>Video</a> to specify the bucket name and the filename of the video. <code>StartPersonTracking</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the operation. When label detection is finished, Amazon Rekognition publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>. </p> <p>To get the results of the person detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <a>GetPersonTracking</a> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartPersonTracking</code>.</p>
    fn start_person_tracking(
        &self,
        input: StartPersonTrackingRequest,
    ) -> RusotoFuture<StartPersonTrackingResponse, StartPersonTrackingError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StartPersonTracking");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartPersonTrackingResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartPersonTrackingError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Starts processing a stream processor. You create a stream processor by calling <a>CreateStreamProcessor</a>. To tell <code>StartStreamProcessor</code> which stream processor to start, use the value of the <code>Name</code> field specified in the call to <code>CreateStreamProcessor</code>.</p>
    fn start_stream_processor(
        &self,
        input: StartStreamProcessorRequest,
    ) -> RusotoFuture<StartStreamProcessorResponse, StartStreamProcessorError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StartStreamProcessor");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartStreamProcessorResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartStreamProcessorError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Stops a running stream processor that was created by <a>CreateStreamProcessor</a>.</p>
    fn stop_stream_processor(
        &self,
        input: StopStreamProcessorRequest,
    ) -> RusotoFuture<StopStreamProcessorResponse, StopStreamProcessorError> {
        let mut request = SignedRequest::new("POST", "rekognition", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "RekognitionService.StopStreamProcessor");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StopStreamProcessorResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StopStreamProcessorError::from_response(response))
                    }),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
