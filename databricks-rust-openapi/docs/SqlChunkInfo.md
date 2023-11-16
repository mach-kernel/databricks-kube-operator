# SqlChunkInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**byte_count** | Option<**i64**> | Number of bytes in the result chunk. | [optional]
**chunk_index** | Option<**i32**> | Position within the sequence of result set chunks. | [optional]
**next_chunk_index** | Option<**i32**> | When fetching, gives `chunk_index` for the _next_ chunk; if absent, indicates there are no more chunks. | [optional]
**next_chunk_internal_link** | Option<**String**> | When fetching, gives `internal_link` for the _next_ chunk; if absent, indicates there are no more chunks. | [optional]
**row_count** | Option<**i64**> | Number of rows within the result chunk. | [optional]
**row_offset** | Option<**i64**> | Starting row offset within the result set. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


