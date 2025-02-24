# VerifiableIdentityAddress

VerifiableAddress is an identity's verifiable address

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **str** |  | 
**status** | **str** | VerifiableAddressStatus must not exceed 16 characters as that is the limitation in the SQL Schema | 
**value** | **str** | The address value  example foo@user.com | 
**verified** | **bool** | Indicates if the address has already been verified | 
**via** | **str** | VerifiableAddressType must not exceed 16 characters as that is the limitation in the SQL Schema | 
**created_at** | **datetime** | When this entry was created | [optional] 
**updated_at** | **datetime** | When this entry was last updated | [optional] 
**verified_at** | **datetime** |  | [optional] 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


