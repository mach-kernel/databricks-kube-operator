# MavenLibrary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**coordinates** | **String** | Gradle-style Maven coordinates. For example: `org.jsoup:jsoup:1.7.2`. This field is required. | 
**repo** | Option<**String**> | Maven repo to install the Maven package from. If omitted, both Maven Central Repository and Spark Packages are searched. | [optional]
**exclusions** | Option<**Vec<String>**> | List of dependences to exclude. For example: `[\"slf4j:slf4j\", \"*:hadoop-client\"]`.  Maven dependency exclusions: <https://maven.apache.org/guides/introduction/introduction-to-optional-and-excludes-dependencies.html>. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


