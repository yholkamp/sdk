# ory_kratos_client.model.GenericError

## Load the model package
```dart
import 'package:ory_kratos_client/api.dart';
```

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **int** | The status code | [optional] 
**debug** | **String** | Debug information  This field is often not exposed to protect against leaking sensitive information. | [optional] 
**details** | [**Map<String, Object>**](Object.md) | Further error details | [optional] [default to const {}]
**message** | **String** | Error message  The error's message. | 
**reason** | **String** | A human-readable reason for the error | [optional] 
**request** | **String** | The request ID  The request ID is often exposed internally in order to trace errors across service architectures. This is often a UUID. | [optional] 
**status** | **String** | The status description | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


