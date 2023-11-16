# SqlExternalLink

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**byte_count** | Option<**i64**> | Number of bytes in the result chunk. | [optional]
**chunk_index** | Option<**i32**> | Position within the sequence of result set chunks. | [optional]
**expiration** | Option<**String**> | Indicates date-time that the given external link will expire and become invalid, after which point a new `external_link` must be requested.  | [optional]
**external_link** | Option<**String**> | Pre-signed URL pointing to a chunk of result data, hosted by an external service, with a short expiration time (< 1 hour).  | [optional]
**next_chunk_index** | Option<**i32**> | When fetching, gives `chunk_index` for the _next_ chunk; if absent, indicates there are no more chunks. | [optional]
**next_chunk_internal_link** | Option<**String**> | When fetching, gives `internal_link` for the _next_ chunk; if absent, indicates there are no more chunks. | [optional]
**row_count** | Option<**i64**> | Number of rows within the result chunk. | [optional]
**row_offset** | Option<**i64**> | Starting row offset within the result set. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


