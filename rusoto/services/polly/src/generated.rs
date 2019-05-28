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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteLexiconInput {
    /// <p>The name of the lexicon to delete. Must be an existing lexicon in the region.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteLexiconOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeVoicesInput {
    /// <p>Boolean value indicating whether to return any bilingual voices that use the specified language as an additional language. For instance, if you request all languages that use US English (es-US), and there is an Italian voice that speaks both Italian (it-IT) and US English, that voice will be included if you specify <code>yes</code> but not if you specify <code>no</code>.</p>
    #[serde(rename = "IncludeAdditionalLanguageCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_additional_language_codes: Option<bool>,
    /// <p> The language identification tag (ISO 639 code for the language name-ISO 3166 country code) for filtering the list of voices returned. If you don't specify this optional parameter, all available voices are returned. </p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>An opaque pagination token returned from the previous <code>DescribeVoices</code> operation. If present, this indicates where to continue the listing.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeVoicesOutput {
    /// <p>The pagination token to use in the next request to continue the listing of voices. <code>NextToken</code> is returned only if the response is truncated.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of voices with their properties.</p>
    #[serde(rename = "Voices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voices: Option<Vec<Voice>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetLexiconInput {
    /// <p>Name of the lexicon.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetLexiconOutput {
    /// <p>Lexicon object that provides name and the string content of the lexicon. </p>
    #[serde(rename = "Lexicon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexicon: Option<Lexicon>,
    /// <p>Metadata of the lexicon, including phonetic alphabetic used, language code, lexicon ARN, number of lexemes defined in the lexicon, and size of lexicon in bytes.</p>
    #[serde(rename = "LexiconAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexicon_attributes: Option<LexiconAttributes>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetSpeechSynthesisTaskInput {
    /// <p>The Amazon Polly generated identifier for a speech synthesis task.</p>
    #[serde(rename = "TaskId")]
    pub task_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetSpeechSynthesisTaskOutput {
    /// <p>SynthesisTask object that provides information from the requested task, including output format, creation time, task status, and so on.</p>
    #[serde(rename = "SynthesisTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synthesis_task: Option<SynthesisTask>,
}

/// <p>Provides lexicon name and lexicon content in string format. For more information, see <a href="https://www.w3.org/TR/pronunciation-lexicon/">Pronunciation Lexicon Specification (PLS) Version 1.0</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Lexicon {
    /// <p>Lexicon content in string format. The content of a lexicon must be in PLS format.</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// <p>Name of the lexicon.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Contains metadata describing the lexicon such as the number of lexemes, language code, and so on. For more information, see <a href="http://docs.aws.amazon.com/polly/latest/dg/managing-lexicons.html">Managing Lexicons</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LexiconAttributes {
    /// <p>Phonetic alphabet used in the lexicon. Valid values are <code>ipa</code> and <code>x-sampa</code>.</p>
    #[serde(rename = "Alphabet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alphabet: Option<String>,
    /// <p>Language code that the lexicon applies to. A lexicon with a language code such as "en" would be applied to all English languages (en-GB, en-US, en-AUS, en-WLS, and so on.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>Date lexicon was last modified (a timestamp value).</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    /// <p>Number of lexemes in the lexicon.</p>
    #[serde(rename = "LexemesCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexemes_count: Option<i64>,
    /// <p>Amazon Resource Name (ARN) of the lexicon.</p>
    #[serde(rename = "LexiconArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexicon_arn: Option<String>,
    /// <p>Total size of the lexicon, in characters.</p>
    #[serde(rename = "Size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

/// <p>Describes the content of the lexicon.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LexiconDescription {
    /// <p>Provides lexicon metadata.</p>
    #[serde(rename = "Attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<LexiconAttributes>,
    /// <p>Name of the lexicon.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListLexiconsInput {
    /// <p>An opaque pagination token returned from previous <code>ListLexicons</code> operation. If present, indicates where to continue the list of lexicons.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListLexiconsOutput {
    /// <p>A list of lexicon names and attributes.</p>
    #[serde(rename = "Lexicons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexicons: Option<Vec<LexiconDescription>>,
    /// <p>The pagination token to use in the next request to continue the listing of lexicons. <code>NextToken</code> is returned only if the response is truncated.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListSpeechSynthesisTasksInput {
    /// <p>Maximum number of speech synthesis tasks returned in a List operation.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The pagination token to use in the next request to continue the listing of speech synthesis tasks. </p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Status of the speech synthesis tasks returned in a List operation</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListSpeechSynthesisTasksOutput {
    /// <p>An opaque pagination token returned from the previous List operation in this request. If present, this indicates where to continue the listing.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of SynthesisTask objects that provides information from the specified task in the list request, including output format, creation time, task status, and so on.</p>
    #[serde(rename = "SynthesisTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synthesis_tasks: Option<Vec<SynthesisTask>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutLexiconInput {
    /// <p>Content of the PLS lexicon as string data.</p>
    #[serde(rename = "Content")]
    pub content: String,
    /// <p>Name of the lexicon. The name must follow the regular express format [0-9A-Za-z]{1,20}. That is, the name is a case-sensitive alphanumeric string up to 20 characters long. </p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutLexiconOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartSpeechSynthesisTaskInput {
    /// <p>Optional language code for the Speech Synthesis request. This is only necessary if using a bilingual voice, such as Aditi, which can be used for either Indian English (en-IN) or Hindi (hi-IN). </p> <p>If a bilingual voice is used and no language code is specified, Amazon Polly will use the default language of the bilingual voice. The default language for any voice is the one returned by the <a href="https://docs.aws.amazon.com/polly/latest/dg/API_DescribeVoices.html">DescribeVoices</a> operation for the <code>LanguageCode</code> parameter. For example, if no language code is specified, Aditi will use Indian English rather than Hindi.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>List of one or more pronunciation lexicon names you want the service to apply during synthesis. Lexicons are applied only if the language of the lexicon is the same as the language of the voice. </p>
    #[serde(rename = "LexiconNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexicon_names: Option<Vec<String>>,
    /// <p>The format in which the returned output will be encoded. For audio stream, this will be mp3, ogg_vorbis, or pcm. For speech marks, this will be json. </p>
    #[serde(rename = "OutputFormat")]
    pub output_format: String,
    /// <p>Amazon S3 bucket name to which the output file will be saved.</p>
    #[serde(rename = "OutputS3BucketName")]
    pub output_s3_bucket_name: String,
    /// <p>The Amazon S3 key prefix for the output speech file.</p>
    #[serde(rename = "OutputS3KeyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_key_prefix: Option<String>,
    /// <p>The audio frequency specified in Hz.</p> <p>The valid values for mp3 and ogg_vorbis are "8000", "16000", and "22050". The default value is "22050".</p> <p>Valid values for pcm are "8000" and "16000" The default value is "16000". </p>
    #[serde(rename = "SampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<String>,
    /// <p>ARN for the SNS topic optionally used for providing status notification for a speech synthesis task.</p>
    #[serde(rename = "SnsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    /// <p>The type of speech marks returned for the input text.</p>
    #[serde(rename = "SpeechMarkTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_mark_types: Option<Vec<String>>,
    /// <p>The input text to synthesize. If you specify ssml as the TextType, follow the SSML format for the input text. </p>
    #[serde(rename = "Text")]
    pub text: String,
    /// <p>Specifies whether the input text is plain text or SSML. The default value is plain text. </p>
    #[serde(rename = "TextType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_type: Option<String>,
    /// <p>Voice ID to use for the synthesis. </p>
    #[serde(rename = "VoiceId")]
    pub voice_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartSpeechSynthesisTaskOutput {
    /// <p>SynthesisTask object that provides information and attributes about a newly submitted speech synthesis task.</p>
    #[serde(rename = "SynthesisTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synthesis_task: Option<SynthesisTask>,
}

/// <p>SynthesisTask object that provides information about a speech synthesis task.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SynthesisTask {
    /// <p>Timestamp for the time the synthesis task was started.</p>
    #[serde(rename = "CreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>Optional language code for a synthesis task. This is only necessary if using a bilingual voice, such as Aditi, which can be used for either Indian English (en-IN) or Hindi (hi-IN). </p> <p>If a bilingual voice is used and no language code is specified, Amazon Polly will use the default language of the bilingual voice. The default language for any voice is the one returned by the <a href="https://docs.aws.amazon.com/polly/latest/dg/API_DescribeVoices.html">DescribeVoices</a> operation for the <code>LanguageCode</code> parameter. For example, if no language code is specified, Aditi will use Indian English rather than Hindi.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>List of one or more pronunciation lexicon names you want the service to apply during synthesis. Lexicons are applied only if the language of the lexicon is the same as the language of the voice. </p>
    #[serde(rename = "LexiconNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexicon_names: Option<Vec<String>>,
    /// <p>The format in which the returned output will be encoded. For audio stream, this will be mp3, ogg_vorbis, or pcm. For speech marks, this will be json. </p>
    #[serde(rename = "OutputFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    /// <p>Pathway for the output speech file.</p>
    #[serde(rename = "OutputUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_uri: Option<String>,
    /// <p>Number of billable characters synthesized.</p>
    #[serde(rename = "RequestCharacters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_characters: Option<i64>,
    /// <p>The audio frequency specified in Hz.</p> <p>The valid values for mp3 and ogg_vorbis are "8000", "16000", and "22050". The default value is "22050".</p> <p>Valid values for pcm are "8000" and "16000" The default value is "16000". </p>
    #[serde(rename = "SampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<String>,
    /// <p>ARN for the SNS topic optionally used for providing status notification for a speech synthesis task.</p>
    #[serde(rename = "SnsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    /// <p>The type of speech marks returned for the input text.</p>
    #[serde(rename = "SpeechMarkTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_mark_types: Option<Vec<String>>,
    /// <p>The Amazon Polly generated identifier for a speech synthesis task.</p>
    #[serde(rename = "TaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    /// <p>Current status of the individual speech synthesis task.</p>
    #[serde(rename = "TaskStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status: Option<String>,
    /// <p>Reason for the current status of a specific speech synthesis task, including errors if the task has failed.</p>
    #[serde(rename = "TaskStatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_status_reason: Option<String>,
    /// <p>Specifies whether the input text is plain text or SSML. The default value is plain text. </p>
    #[serde(rename = "TextType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_type: Option<String>,
    /// <p>Voice ID to use for the synthesis. </p>
    #[serde(rename = "VoiceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SynthesizeSpeechInput {
    /// <p>Optional language code for the Synthesize Speech request. This is only necessary if using a bilingual voice, such as Aditi, which can be used for either Indian English (en-IN) or Hindi (hi-IN). </p> <p>If a bilingual voice is used and no language code is specified, Amazon Polly will use the default language of the bilingual voice. The default language for any voice is the one returned by the <a href="https://docs.aws.amazon.com/polly/latest/dg/API_DescribeVoices.html">DescribeVoices</a> operation for the <code>LanguageCode</code> parameter. For example, if no language code is specified, Aditi will use Indian English rather than Hindi.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>List of one or more pronunciation lexicon names you want the service to apply during synthesis. Lexicons are applied only if the language of the lexicon is the same as the language of the voice. For information about storing lexicons, see <a href="http://docs.aws.amazon.com/polly/latest/dg/API_PutLexicon.html">PutLexicon</a>.</p>
    #[serde(rename = "LexiconNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexicon_names: Option<Vec<String>>,
    /// <p> The format in which the returned output will be encoded. For audio stream, this will be mp3, ogg_vorbis, or pcm. For speech marks, this will be json. </p> <p>When pcm is used, the content returned is audio/pcm in a signed 16-bit, 1 channel (mono), little-endian format. </p>
    #[serde(rename = "OutputFormat")]
    pub output_format: String,
    /// <p> The audio frequency specified in Hz. </p> <p>The valid values for <code>mp3</code> and <code>ogg_vorbis</code> are "8000", "16000", and "22050". The default value is "22050". </p> <p> Valid values for <code>pcm</code> are "8000" and "16000" The default value is "16000". </p>
    #[serde(rename = "SampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<String>,
    /// <p>The type of speech marks returned for the input text.</p>
    #[serde(rename = "SpeechMarkTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_mark_types: Option<Vec<String>>,
    /// <p> Input text to synthesize. If you specify <code>ssml</code> as the <code>TextType</code>, follow the SSML format for the input text. </p>
    #[serde(rename = "Text")]
    pub text: String,
    /// <p> Specifies whether the input text is plain text or SSML. The default value is plain text. For more information, see <a href="http://docs.aws.amazon.com/polly/latest/dg/ssml.html">Using SSML</a>.</p>
    #[serde(rename = "TextType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_type: Option<String>,
    /// <p> Voice ID to use for the synthesis. You can get a list of available voice IDs by calling the <a href="http://docs.aws.amazon.com/polly/latest/dg/API_DescribeVoices.html">DescribeVoices</a> operation. </p>
    #[serde(rename = "VoiceId")]
    pub voice_id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SynthesizeSpeechOutput {
    /// <p> Stream containing the synthesized speech. </p>
    pub audio_stream: Option<bytes::Bytes>,
    /// <p> Specifies the type audio stream. This should reflect the <code>OutputFormat</code> parameter in your request. </p> <ul> <li> <p> If you request <code>mp3</code> as the <code>OutputFormat</code>, the <code>ContentType</code> returned is audio/mpeg. </p> </li> <li> <p> If you request <code>ogg_vorbis</code> as the <code>OutputFormat</code>, the <code>ContentType</code> returned is audio/ogg. </p> </li> <li> <p> If you request <code>pcm</code> as the <code>OutputFormat</code>, the <code>ContentType</code> returned is audio/pcm in a signed 16-bit, 1 channel (mono), little-endian format. </p> </li> <li> <p>If you request <code>json</code> as the <code>OutputFormat</code>, the <code>ContentType</code> returned is audio/json.</p> </li> </ul> <p> </p>
    pub content_type: Option<String>,
    /// <p>Number of characters synthesized.</p>
    pub request_characters: Option<i64>,
}

/// <p>Description of the voice.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Voice {
    /// <p>Additional codes for languages available for the specified voice in addition to its default language. </p> <p>For example, the default language for Aditi is Indian English (en-IN) because it was first used for that language. Since Aditi is bilingual and fluent in both Indian English and Hindi, this parameter would show the code <code>hi-IN</code>.</p>
    #[serde(rename = "AdditionalLanguageCodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_language_codes: Option<Vec<String>>,
    /// <p>Gender of the voice.</p>
    #[serde(rename = "Gender")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// <p>Amazon Polly assigned voice ID. This is the ID that you specify when calling the <code>SynthesizeSpeech</code> operation.</p>
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Language code of the voice.</p>
    #[serde(rename = "LanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>Human readable name of the language in English.</p>
    #[serde(rename = "LanguageName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_name: Option<String>,
    /// <p>Name of the voice (for example, Salli, Kendra, etc.). This provides a human readable voice name that you might display in your application.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Errors returned by DeleteLexicon
#[derive(Debug, PartialEq)]
pub enum DeleteLexiconError {
    /// <p>Amazon Polly can't find the specified lexicon. This could be caused by a lexicon that is missing, its name is misspelled or specifying a lexicon that is in a different region.</p> <p>Verify that the lexicon exists, is in the region (see <a>ListLexicons</a>) and that you spelled its name is spelled correctly. Then try again.</p>
    LexiconNotFound(String),
    /// <p>An unknown condition has caused a service failure.</p>
    ServiceFailure(String),
}

impl DeleteLexiconError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLexiconError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "LexiconNotFoundException" => {
                    return RusotoError::Service(DeleteLexiconError::LexiconNotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(DeleteLexiconError::ServiceFailure(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteLexiconError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLexiconError {
    fn description(&self) -> &str {
        match *self {
            DeleteLexiconError::LexiconNotFound(ref cause) => cause,
            DeleteLexiconError::ServiceFailure(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeVoices
#[derive(Debug, PartialEq)]
pub enum DescribeVoicesError {
    /// <p>The NextToken is invalid. Verify that it's spelled correctly, and then try again.</p>
    InvalidNextToken(String),
    /// <p>An unknown condition has caused a service failure.</p>
    ServiceFailure(String),
}

impl DescribeVoicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeVoicesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(DescribeVoicesError::InvalidNextToken(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(DescribeVoicesError::ServiceFailure(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeVoicesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeVoicesError {
    fn description(&self) -> &str {
        match *self {
            DescribeVoicesError::InvalidNextToken(ref cause) => cause,
            DescribeVoicesError::ServiceFailure(ref cause) => cause,
        }
    }
}
/// Errors returned by GetLexicon
#[derive(Debug, PartialEq)]
pub enum GetLexiconError {
    /// <p>Amazon Polly can't find the specified lexicon. This could be caused by a lexicon that is missing, its name is misspelled or specifying a lexicon that is in a different region.</p> <p>Verify that the lexicon exists, is in the region (see <a>ListLexicons</a>) and that you spelled its name is spelled correctly. Then try again.</p>
    LexiconNotFound(String),
    /// <p>An unknown condition has caused a service failure.</p>
    ServiceFailure(String),
}

impl GetLexiconError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLexiconError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "LexiconNotFoundException" => {
                    return RusotoError::Service(GetLexiconError::LexiconNotFound(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(GetLexiconError::ServiceFailure(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetLexiconError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLexiconError {
    fn description(&self) -> &str {
        match *self {
            GetLexiconError::LexiconNotFound(ref cause) => cause,
            GetLexiconError::ServiceFailure(ref cause) => cause,
        }
    }
}
/// Errors returned by GetSpeechSynthesisTask
#[derive(Debug, PartialEq)]
pub enum GetSpeechSynthesisTaskError {
    /// <p>The provided Task ID is not valid. Please provide a valid Task ID and try again.</p>
    InvalidTaskId(String),
    /// <p>An unknown condition has caused a service failure.</p>
    ServiceFailure(String),
    /// <p>The Speech Synthesis task with requested Task ID cannot be found.</p>
    SynthesisTaskNotFound(String),
}

impl GetSpeechSynthesisTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSpeechSynthesisTaskError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidTaskIdException" => {
                    return RusotoError::Service(GetSpeechSynthesisTaskError::InvalidTaskId(
                        err.msg,
                    ))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(GetSpeechSynthesisTaskError::ServiceFailure(
                        err.msg,
                    ))
                }
                "SynthesisTaskNotFoundException" => {
                    return RusotoError::Service(
                        GetSpeechSynthesisTaskError::SynthesisTaskNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetSpeechSynthesisTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetSpeechSynthesisTaskError {
    fn description(&self) -> &str {
        match *self {
            GetSpeechSynthesisTaskError::InvalidTaskId(ref cause) => cause,
            GetSpeechSynthesisTaskError::ServiceFailure(ref cause) => cause,
            GetSpeechSynthesisTaskError::SynthesisTaskNotFound(ref cause) => cause,
        }
    }
}
/// Errors returned by ListLexicons
#[derive(Debug, PartialEq)]
pub enum ListLexiconsError {
    /// <p>The NextToken is invalid. Verify that it's spelled correctly, and then try again.</p>
    InvalidNextToken(String),
    /// <p>An unknown condition has caused a service failure.</p>
    ServiceFailure(String),
}

impl ListLexiconsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLexiconsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListLexiconsError::InvalidNextToken(err.msg))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(ListLexiconsError::ServiceFailure(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListLexiconsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListLexiconsError {
    fn description(&self) -> &str {
        match *self {
            ListLexiconsError::InvalidNextToken(ref cause) => cause,
            ListLexiconsError::ServiceFailure(ref cause) => cause,
        }
    }
}
/// Errors returned by ListSpeechSynthesisTasks
#[derive(Debug, PartialEq)]
pub enum ListSpeechSynthesisTasksError {
    /// <p>The NextToken is invalid. Verify that it's spelled correctly, and then try again.</p>
    InvalidNextToken(String),
    /// <p>An unknown condition has caused a service failure.</p>
    ServiceFailure(String),
}

impl ListSpeechSynthesisTasksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSpeechSynthesisTasksError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidNextTokenException" => {
                    return RusotoError::Service(ListSpeechSynthesisTasksError::InvalidNextToken(
                        err.msg,
                    ))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(ListSpeechSynthesisTasksError::ServiceFailure(
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
impl fmt::Display for ListSpeechSynthesisTasksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListSpeechSynthesisTasksError {
    fn description(&self) -> &str {
        match *self {
            ListSpeechSynthesisTasksError::InvalidNextToken(ref cause) => cause,
            ListSpeechSynthesisTasksError::ServiceFailure(ref cause) => cause,
        }
    }
}
/// Errors returned by PutLexicon
#[derive(Debug, PartialEq)]
pub enum PutLexiconError {
    /// <p>Amazon Polly can't find the specified lexicon. Verify that the lexicon's name is spelled correctly, and then try again.</p>
    InvalidLexicon(String),
    /// <p>The maximum size of the specified lexicon would be exceeded by this operation.</p>
    LexiconSizeExceeded(String),
    /// <p>The maximum size of the lexeme would be exceeded by this operation.</p>
    MaxLexemeLengthExceeded(String),
    /// <p>The maximum number of lexicons would be exceeded by this operation.</p>
    MaxLexiconsNumberExceeded(String),
    /// <p>An unknown condition has caused a service failure.</p>
    ServiceFailure(String),
    /// <p>The alphabet specified by the lexicon is not a supported alphabet. Valid values are <code>x-sampa</code> and <code>ipa</code>.</p>
    UnsupportedPlsAlphabet(String),
    /// <p>The language specified in the lexicon is unsupported. For a list of supported languages, see <a href="http://docs.aws.amazon.com/polly/latest/dg/API_LexiconAttributes.html">Lexicon Attributes</a>.</p>
    UnsupportedPlsLanguage(String),
}

impl PutLexiconError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutLexiconError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidLexiconException" => {
                    return RusotoError::Service(PutLexiconError::InvalidLexicon(err.msg))
                }
                "LexiconSizeExceededException" => {
                    return RusotoError::Service(PutLexiconError::LexiconSizeExceeded(err.msg))
                }
                "MaxLexemeLengthExceededException" => {
                    return RusotoError::Service(PutLexiconError::MaxLexemeLengthExceeded(err.msg))
                }
                "MaxLexiconsNumberExceededException" => {
                    return RusotoError::Service(PutLexiconError::MaxLexiconsNumberExceeded(
                        err.msg,
                    ))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(PutLexiconError::ServiceFailure(err.msg))
                }
                "UnsupportedPlsAlphabetException" => {
                    return RusotoError::Service(PutLexiconError::UnsupportedPlsAlphabet(err.msg))
                }
                "UnsupportedPlsLanguageException" => {
                    return RusotoError::Service(PutLexiconError::UnsupportedPlsLanguage(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutLexiconError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutLexiconError {
    fn description(&self) -> &str {
        match *self {
            PutLexiconError::InvalidLexicon(ref cause) => cause,
            PutLexiconError::LexiconSizeExceeded(ref cause) => cause,
            PutLexiconError::MaxLexemeLengthExceeded(ref cause) => cause,
            PutLexiconError::MaxLexiconsNumberExceeded(ref cause) => cause,
            PutLexiconError::ServiceFailure(ref cause) => cause,
            PutLexiconError::UnsupportedPlsAlphabet(ref cause) => cause,
            PutLexiconError::UnsupportedPlsLanguage(ref cause) => cause,
        }
    }
}
/// Errors returned by StartSpeechSynthesisTask
#[derive(Debug, PartialEq)]
pub enum StartSpeechSynthesisTaskError {
    /// <p>The provided Amazon S3 bucket name is invalid. Please check your input with S3 bucket naming requirements and try again.</p>
    InvalidS3Bucket(String),
    /// <p>The provided Amazon S3 key prefix is invalid. Please provide a valid S3 object key name.</p>
    InvalidS3Key(String),
    /// <p>The specified sample rate is not valid.</p>
    InvalidSampleRate(String),
    /// <p>The provided SNS topic ARN is invalid. Please provide a valid SNS topic ARN and try again.</p>
    InvalidSnsTopicArn(String),
    /// <p>The SSML you provided is invalid. Verify the SSML syntax, spelling of tags and values, and then try again.</p>
    InvalidSsml(String),
    /// <p>The language specified is not currently supported by Amazon Polly in this capacity.</p>
    LanguageNotSupported(String),
    /// <p>Amazon Polly can't find the specified lexicon. This could be caused by a lexicon that is missing, its name is misspelled or specifying a lexicon that is in a different region.</p> <p>Verify that the lexicon exists, is in the region (see <a>ListLexicons</a>) and that you spelled its name is spelled correctly. Then try again.</p>
    LexiconNotFound(String),
    /// <p>Speech marks are not supported for the <code>OutputFormat</code> selected. Speech marks are only available for content in <code>json</code> format.</p>
    MarksNotSupportedForFormat(String),
    /// <p>An unknown condition has caused a service failure.</p>
    ServiceFailure(String),
    /// <p>SSML speech marks are not supported for plain text-type input.</p>
    SsmlMarksNotSupportedForTextType(String),
    /// <p>The value of the "Text" parameter is longer than the accepted limits. For the <code>SynthesizeSpeech</code> API, the limit for input text is a maximum of 6000 characters total, of which no more than 3000 can be billed characters. For the <code>StartSpeechSynthesisTask</code> API, the maximum is 200,000 characters, of which no more than 100,000 can be billed characters. SSML tags are not counted as billed characters.</p>
    TextLengthExceeded(String),
}

impl StartSpeechSynthesisTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartSpeechSynthesisTaskError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidS3BucketException" => {
                    return RusotoError::Service(StartSpeechSynthesisTaskError::InvalidS3Bucket(
                        err.msg,
                    ))
                }
                "InvalidS3KeyException" => {
                    return RusotoError::Service(StartSpeechSynthesisTaskError::InvalidS3Key(
                        err.msg,
                    ))
                }
                "InvalidSampleRateException" => {
                    return RusotoError::Service(StartSpeechSynthesisTaskError::InvalidSampleRate(
                        err.msg,
                    ))
                }
                "InvalidSnsTopicArnException" => {
                    return RusotoError::Service(StartSpeechSynthesisTaskError::InvalidSnsTopicArn(
                        err.msg,
                    ))
                }
                "InvalidSsmlException" => {
                    return RusotoError::Service(StartSpeechSynthesisTaskError::InvalidSsml(
                        err.msg,
                    ))
                }
                "LanguageNotSupportedException" => {
                    return RusotoError::Service(
                        StartSpeechSynthesisTaskError::LanguageNotSupported(err.msg),
                    )
                }
                "LexiconNotFoundException" => {
                    return RusotoError::Service(StartSpeechSynthesisTaskError::LexiconNotFound(
                        err.msg,
                    ))
                }
                "MarksNotSupportedForFormatException" => {
                    return RusotoError::Service(
                        StartSpeechSynthesisTaskError::MarksNotSupportedForFormat(err.msg),
                    )
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(StartSpeechSynthesisTaskError::ServiceFailure(
                        err.msg,
                    ))
                }
                "SsmlMarksNotSupportedForTextTypeException" => {
                    return RusotoError::Service(
                        StartSpeechSynthesisTaskError::SsmlMarksNotSupportedForTextType(err.msg),
                    )
                }
                "TextLengthExceededException" => {
                    return RusotoError::Service(StartSpeechSynthesisTaskError::TextLengthExceeded(
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
impl fmt::Display for StartSpeechSynthesisTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartSpeechSynthesisTaskError {
    fn description(&self) -> &str {
        match *self {
            StartSpeechSynthesisTaskError::InvalidS3Bucket(ref cause) => cause,
            StartSpeechSynthesisTaskError::InvalidS3Key(ref cause) => cause,
            StartSpeechSynthesisTaskError::InvalidSampleRate(ref cause) => cause,
            StartSpeechSynthesisTaskError::InvalidSnsTopicArn(ref cause) => cause,
            StartSpeechSynthesisTaskError::InvalidSsml(ref cause) => cause,
            StartSpeechSynthesisTaskError::LanguageNotSupported(ref cause) => cause,
            StartSpeechSynthesisTaskError::LexiconNotFound(ref cause) => cause,
            StartSpeechSynthesisTaskError::MarksNotSupportedForFormat(ref cause) => cause,
            StartSpeechSynthesisTaskError::ServiceFailure(ref cause) => cause,
            StartSpeechSynthesisTaskError::SsmlMarksNotSupportedForTextType(ref cause) => cause,
            StartSpeechSynthesisTaskError::TextLengthExceeded(ref cause) => cause,
        }
    }
}
/// Errors returned by SynthesizeSpeech
#[derive(Debug, PartialEq)]
pub enum SynthesizeSpeechError {
    /// <p>The specified sample rate is not valid.</p>
    InvalidSampleRate(String),
    /// <p>The SSML you provided is invalid. Verify the SSML syntax, spelling of tags and values, and then try again.</p>
    InvalidSsml(String),
    /// <p>The language specified is not currently supported by Amazon Polly in this capacity.</p>
    LanguageNotSupported(String),
    /// <p>Amazon Polly can't find the specified lexicon. This could be caused by a lexicon that is missing, its name is misspelled or specifying a lexicon that is in a different region.</p> <p>Verify that the lexicon exists, is in the region (see <a>ListLexicons</a>) and that you spelled its name is spelled correctly. Then try again.</p>
    LexiconNotFound(String),
    /// <p>Speech marks are not supported for the <code>OutputFormat</code> selected. Speech marks are only available for content in <code>json</code> format.</p>
    MarksNotSupportedForFormat(String),
    /// <p>An unknown condition has caused a service failure.</p>
    ServiceFailure(String),
    /// <p>SSML speech marks are not supported for plain text-type input.</p>
    SsmlMarksNotSupportedForTextType(String),
    /// <p>The value of the "Text" parameter is longer than the accepted limits. For the <code>SynthesizeSpeech</code> API, the limit for input text is a maximum of 6000 characters total, of which no more than 3000 can be billed characters. For the <code>StartSpeechSynthesisTask</code> API, the maximum is 200,000 characters, of which no more than 100,000 can be billed characters. SSML tags are not counted as billed characters.</p>
    TextLengthExceeded(String),
}

impl SynthesizeSpeechError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SynthesizeSpeechError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidSampleRateException" => {
                    return RusotoError::Service(SynthesizeSpeechError::InvalidSampleRate(err.msg))
                }
                "InvalidSsmlException" => {
                    return RusotoError::Service(SynthesizeSpeechError::InvalidSsml(err.msg))
                }
                "LanguageNotSupportedException" => {
                    return RusotoError::Service(SynthesizeSpeechError::LanguageNotSupported(
                        err.msg,
                    ))
                }
                "LexiconNotFoundException" => {
                    return RusotoError::Service(SynthesizeSpeechError::LexiconNotFound(err.msg))
                }
                "MarksNotSupportedForFormatException" => {
                    return RusotoError::Service(SynthesizeSpeechError::MarksNotSupportedForFormat(
                        err.msg,
                    ))
                }
                "ServiceFailureException" => {
                    return RusotoError::Service(SynthesizeSpeechError::ServiceFailure(err.msg))
                }
                "SsmlMarksNotSupportedForTextTypeException" => {
                    return RusotoError::Service(
                        SynthesizeSpeechError::SsmlMarksNotSupportedForTextType(err.msg),
                    )
                }
                "TextLengthExceededException" => {
                    return RusotoError::Service(SynthesizeSpeechError::TextLengthExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SynthesizeSpeechError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SynthesizeSpeechError {
    fn description(&self) -> &str {
        match *self {
            SynthesizeSpeechError::InvalidSampleRate(ref cause) => cause,
            SynthesizeSpeechError::InvalidSsml(ref cause) => cause,
            SynthesizeSpeechError::LanguageNotSupported(ref cause) => cause,
            SynthesizeSpeechError::LexiconNotFound(ref cause) => cause,
            SynthesizeSpeechError::MarksNotSupportedForFormat(ref cause) => cause,
            SynthesizeSpeechError::ServiceFailure(ref cause) => cause,
            SynthesizeSpeechError::SsmlMarksNotSupportedForTextType(ref cause) => cause,
            SynthesizeSpeechError::TextLengthExceeded(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Polly API. Amazon Polly clients implement this trait.
pub trait Polly {
    /// <p>Deletes the specified pronunciation lexicon stored in an AWS Region. A lexicon which has been deleted is not available for speech synthesis, nor is it possible to retrieve it using either the <code>GetLexicon</code> or <code>ListLexicon</code> APIs.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/polly/latest/dg/managing-lexicons.html">Managing Lexicons</a>.</p>
    fn delete_lexicon(
        &self,
        input: DeleteLexiconInput,
    ) -> RusotoFuture<DeleteLexiconOutput, DeleteLexiconError>;

    /// <p>Returns the list of voices that are available for use when requesting speech synthesis. Each voice speaks a specified language, is either male or female, and is identified by an ID, which is the ASCII version of the voice name. </p> <p>When synthesizing speech ( <code>SynthesizeSpeech</code> ), you provide the voice ID for the voice you want from the list of voices returned by <code>DescribeVoices</code>.</p> <p>For example, you want your news reader application to read news in a specific language, but giving a user the option to choose the voice. Using the <code>DescribeVoices</code> operation you can provide the user with a list of available voices to select from.</p> <p> You can optionally specify a language code to filter the available voices. For example, if you specify <code>en-US</code>, the operation returns a list of all available US English voices. </p> <p>This operation requires permissions to perform the <code>polly:DescribeVoices</code> action.</p>
    fn describe_voices(
        &self,
        input: DescribeVoicesInput,
    ) -> RusotoFuture<DescribeVoicesOutput, DescribeVoicesError>;

    /// <p>Returns the content of the specified pronunciation lexicon stored in an AWS Region. For more information, see <a href="http://docs.aws.amazon.com/polly/latest/dg/managing-lexicons.html">Managing Lexicons</a>.</p>
    fn get_lexicon(
        &self,
        input: GetLexiconInput,
    ) -> RusotoFuture<GetLexiconOutput, GetLexiconError>;

    /// <p>Retrieves a specific SpeechSynthesisTask object based on its TaskID. This object contains information about the given speech synthesis task, including the status of the task, and a link to the S3 bucket containing the output of the task.</p>
    fn get_speech_synthesis_task(
        &self,
        input: GetSpeechSynthesisTaskInput,
    ) -> RusotoFuture<GetSpeechSynthesisTaskOutput, GetSpeechSynthesisTaskError>;

    /// <p>Returns a list of pronunciation lexicons stored in an AWS Region. For more information, see <a href="http://docs.aws.amazon.com/polly/latest/dg/managing-lexicons.html">Managing Lexicons</a>.</p>
    fn list_lexicons(
        &self,
        input: ListLexiconsInput,
    ) -> RusotoFuture<ListLexiconsOutput, ListLexiconsError>;

    /// <p>Returns a list of SpeechSynthesisTask objects ordered by their creation date. This operation can filter the tasks by their status, for example, allowing users to list only tasks that are completed.</p>
    fn list_speech_synthesis_tasks(
        &self,
        input: ListSpeechSynthesisTasksInput,
    ) -> RusotoFuture<ListSpeechSynthesisTasksOutput, ListSpeechSynthesisTasksError>;

    /// <p>Stores a pronunciation lexicon in an AWS Region. If a lexicon with the same name already exists in the region, it is overwritten by the new lexicon. Lexicon operations have eventual consistency, therefore, it might take some time before the lexicon is available to the SynthesizeSpeech operation.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/polly/latest/dg/managing-lexicons.html">Managing Lexicons</a>.</p>
    fn put_lexicon(
        &self,
        input: PutLexiconInput,
    ) -> RusotoFuture<PutLexiconOutput, PutLexiconError>;

    /// <p>Allows the creation of an asynchronous synthesis task, by starting a new <code>SpeechSynthesisTask</code>. This operation requires all the standard information needed for speech synthesis, plus the name of an Amazon S3 bucket for the service to store the output of the synthesis task and two optional parameters (OutputS3KeyPrefix and SnsTopicArn). Once the synthesis task is created, this operation will return a SpeechSynthesisTask object, which will include an identifier of this task as well as the current status.</p>
    fn start_speech_synthesis_task(
        &self,
        input: StartSpeechSynthesisTaskInput,
    ) -> RusotoFuture<StartSpeechSynthesisTaskOutput, StartSpeechSynthesisTaskError>;

    /// <p>Synthesizes UTF-8 input, plain text or SSML, to a stream of bytes. SSML input must be valid, well-formed SSML. Some alphabets might not be available with all the voices (for example, Cyrillic might not be read at all by English voices) unless phoneme mapping is used. For more information, see <a href="http://docs.aws.amazon.com/polly/latest/dg/how-text-to-speech-works.html">How it Works</a>.</p>
    fn synthesize_speech(
        &self,
        input: SynthesizeSpeechInput,
    ) -> RusotoFuture<SynthesizeSpeechOutput, SynthesizeSpeechError>;
}
/// A client for the Amazon Polly API.
#[derive(Clone)]
pub struct PollyClient {
    client: Client,
    region: region::Region,
}

impl PollyClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> PollyClient {
        PollyClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> PollyClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        PollyClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }
}

impl Polly for PollyClient {
    /// <p>Deletes the specified pronunciation lexicon stored in an AWS Region. A lexicon which has been deleted is not available for speech synthesis, nor is it possible to retrieve it using either the <code>GetLexicon</code> or <code>ListLexicon</code> APIs.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/polly/latest/dg/managing-lexicons.html">Managing Lexicons</a>.</p>
    fn delete_lexicon(
        &self,
        input: DeleteLexiconInput,
    ) -> RusotoFuture<DeleteLexiconOutput, DeleteLexiconError> {
        let request_uri = format!("/v1/lexicons/{lexicon_name}", lexicon_name = input.name);

        let mut request = SignedRequest::new("DELETE", "polly", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteLexiconOutput, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteLexiconError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the list of voices that are available for use when requesting speech synthesis. Each voice speaks a specified language, is either male or female, and is identified by an ID, which is the ASCII version of the voice name. </p> <p>When synthesizing speech ( <code>SynthesizeSpeech</code> ), you provide the voice ID for the voice you want from the list of voices returned by <code>DescribeVoices</code>.</p> <p>For example, you want your news reader application to read news in a specific language, but giving a user the option to choose the voice. Using the <code>DescribeVoices</code> operation you can provide the user with a list of available voices to select from.</p> <p> You can optionally specify a language code to filter the available voices. For example, if you specify <code>en-US</code>, the operation returns a list of all available US English voices. </p> <p>This operation requires permissions to perform the <code>polly:DescribeVoices</code> action.</p>
    fn describe_voices(
        &self,
        input: DescribeVoicesInput,
    ) -> RusotoFuture<DescribeVoicesOutput, DescribeVoicesError> {
        let request_uri = "/v1/voices";

        let mut request = SignedRequest::new("GET", "polly", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.include_additional_language_codes {
            params.put("IncludeAdditionalLanguageCodes", x);
        }
        if let Some(ref x) = input.language_code {
            params.put("LanguageCode", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeVoicesOutput, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeVoicesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the content of the specified pronunciation lexicon stored in an AWS Region. For more information, see <a href="http://docs.aws.amazon.com/polly/latest/dg/managing-lexicons.html">Managing Lexicons</a>.</p>
    fn get_lexicon(
        &self,
        input: GetLexiconInput,
    ) -> RusotoFuture<GetLexiconOutput, GetLexiconError> {
        let request_uri = format!("/v1/lexicons/{lexicon_name}", lexicon_name = input.name);

        let mut request = SignedRequest::new("GET", "polly", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetLexiconOutput, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetLexiconError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a specific SpeechSynthesisTask object based on its TaskID. This object contains information about the given speech synthesis task, including the status of the task, and a link to the S3 bucket containing the output of the task.</p>
    fn get_speech_synthesis_task(
        &self,
        input: GetSpeechSynthesisTaskInput,
    ) -> RusotoFuture<GetSpeechSynthesisTaskOutput, GetSpeechSynthesisTaskError> {
        let request_uri = format!("/v1/synthesisTasks/{task_id}", task_id = input.task_id);

        let mut request = SignedRequest::new("GET", "polly", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetSpeechSynthesisTaskOutput, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetSpeechSynthesisTaskError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns a list of pronunciation lexicons stored in an AWS Region. For more information, see <a href="http://docs.aws.amazon.com/polly/latest/dg/managing-lexicons.html">Managing Lexicons</a>.</p>
    fn list_lexicons(
        &self,
        input: ListLexiconsInput,
    ) -> RusotoFuture<ListLexiconsOutput, ListLexiconsError> {
        let request_uri = "/v1/lexicons";

        let mut request = SignedRequest::new("GET", "polly", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListLexiconsOutput, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListLexiconsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of SpeechSynthesisTask objects ordered by their creation date. This operation can filter the tasks by their status, for example, allowing users to list only tasks that are completed.</p>
    fn list_speech_synthesis_tasks(
        &self,
        input: ListSpeechSynthesisTasksInput,
    ) -> RusotoFuture<ListSpeechSynthesisTasksOutput, ListSpeechSynthesisTasksError> {
        let request_uri = "/v1/synthesisTasks";

        let mut request = SignedRequest::new("GET", "polly", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("MaxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("NextToken", x);
        }
        if let Some(ref x) = input.status {
            params.put("Status", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListSpeechSynthesisTasksOutput, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListSpeechSynthesisTasksError::from_response(response))
                }))
            }
        })
    }

    /// <p>Stores a pronunciation lexicon in an AWS Region. If a lexicon with the same name already exists in the region, it is overwritten by the new lexicon. Lexicon operations have eventual consistency, therefore, it might take some time before the lexicon is available to the SynthesizeSpeech operation.</p> <p>For more information, see <a href="http://docs.aws.amazon.com/polly/latest/dg/managing-lexicons.html">Managing Lexicons</a>.</p>
    fn put_lexicon(
        &self,
        input: PutLexiconInput,
    ) -> RusotoFuture<PutLexiconOutput, PutLexiconError> {
        let request_uri = format!("/v1/lexicons/{lexicon_name}", lexicon_name = input.name);

        let mut request = SignedRequest::new("PUT", "polly", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<PutLexiconOutput, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutLexiconError::from_response(response))),
                )
            }
        })
    }

    /// <p>Allows the creation of an asynchronous synthesis task, by starting a new <code>SpeechSynthesisTask</code>. This operation requires all the standard information needed for speech synthesis, plus the name of an Amazon S3 bucket for the service to store the output of the synthesis task and two optional parameters (OutputS3KeyPrefix and SnsTopicArn). Once the synthesis task is created, this operation will return a SpeechSynthesisTask object, which will include an identifier of this task as well as the current status.</p>
    fn start_speech_synthesis_task(
        &self,
        input: StartSpeechSynthesisTaskInput,
    ) -> RusotoFuture<StartSpeechSynthesisTaskOutput, StartSpeechSynthesisTaskError> {
        let request_uri = "/v1/synthesisTasks";

        let mut request = SignedRequest::new("POST", "polly", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartSpeechSynthesisTaskOutput, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartSpeechSynthesisTaskError::from_response(response))
                }))
            }
        })
    }

    /// <p>Synthesizes UTF-8 input, plain text or SSML, to a stream of bytes. SSML input must be valid, well-formed SSML. Some alphabets might not be available with all the voices (for example, Cyrillic might not be read at all by English voices) unless phoneme mapping is used. For more information, see <a href="http://docs.aws.amazon.com/polly/latest/dg/how-text-to-speech-works.html">How it Works</a>.</p>
    fn synthesize_speech(
        &self,
        input: SynthesizeSpeechInput,
    ) -> RusotoFuture<SynthesizeSpeechOutput, SynthesizeSpeechError> {
        let request_uri = "/v1/speech";

        let mut request = SignedRequest::new("POST", "polly", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let mut result = SynthesizeSpeechOutput::default();
                    result.audio_stream = Some(response.body);

                    if let Some(content_type) = response.headers.get("Content-Type") {
                        let value = content_type.to_owned();
                        result.content_type = Some(value)
                    };
                    if let Some(request_characters) =
                        response.headers.get("x-amzn-RequestCharacters")
                    {
                        let value = request_characters.to_owned();
                        result.request_characters = Some(value.parse::<i64>().unwrap())
                    };

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SynthesizeSpeechError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
