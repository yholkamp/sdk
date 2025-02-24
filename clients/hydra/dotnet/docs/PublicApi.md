# Ory.Hydra.Client.Api.PublicApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**DisconnectUser**](PublicApi.md#disconnectuser) | **GET** /oauth2/sessions/logout | OpenID Connect Front-Backchannel Enabled Logout
[**DiscoverOpenIDConfiguration**](PublicApi.md#discoveropenidconfiguration) | **GET** /.well-known/openid-configuration | OpenID Connect Discovery
[**IsInstanceReady**](PublicApi.md#isinstanceready) | **GET** /health/ready | Check Readiness Status
[**Oauth2Token**](PublicApi.md#oauth2token) | **POST** /oauth2/token | The OAuth 2.0 Token Endpoint
[**OauthAuth**](PublicApi.md#oauthauth) | **GET** /oauth2/auth | The OAuth 2.0 Authorize Endpoint
[**RevokeOAuth2Token**](PublicApi.md#revokeoauth2token) | **POST** /oauth2/revoke | Revoke OAuth2 Tokens
[**Userinfo**](PublicApi.md#userinfo) | **GET** /userinfo | OpenID Connect Userinfo
[**WellKnown**](PublicApi.md#wellknown) | **GET** /.well-known/jwks.json | JSON Web Keys Discovery


<a name="disconnectuser"></a>
# **DisconnectUser**
> void DisconnectUser ()

OpenID Connect Front-Backchannel Enabled Logout

This endpoint initiates and completes user logout at ORY Hydra and initiates OpenID Connect Front-/Back-channel logout:  https://openid.net/specs/openid-connect-frontchannel-1_0.html https://openid.net/specs/openid-connect-backchannel-1_0.html

### Example
```csharp
using System.Collections.Generic;
using System.Diagnostics;
using Ory.Hydra.Client.Api;
using Ory.Hydra.Client.Client;
using Ory.Hydra.Client.Model;

namespace Example
{
    public class DisconnectUserExample
    {
        public static void Main()
        {
            Configuration config = new Configuration();
            config.BasePath = "http://localhost";
            var apiInstance = new PublicApi(config);

            try
            {
                // OpenID Connect Front-Backchannel Enabled Logout
                apiInstance.DisconnectUser();
            }
            catch (ApiException  e)
            {
                Debug.Print("Exception when calling PublicApi.DisconnectUser: " + e.Message );
                Debug.Print("Status Code: "+ e.ErrorCode);
                Debug.Print(e.StackTrace);
            }
        }
    }
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **302** | Empty responses are sent when, for example, resources are deleted. The HTTP status code for empty responses is typically 201. |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

<a name="discoveropenidconfiguration"></a>
# **DiscoverOpenIDConfiguration**
> HydraWellKnown DiscoverOpenIDConfiguration ()

OpenID Connect Discovery

The well known endpoint an be used to retrieve information for OpenID Connect clients. We encourage you to not roll your own OpenID Connect client but to use an OpenID Connect client library instead. You can learn more on this flow at https://openid.net/specs/openid-connect-discovery-1_0.html .  Popular libraries for OpenID Connect clients include oidc-client-js (JavaScript), go-oidc (Golang), and others. For a full list of clients go here: https://openid.net/developers/certified/

### Example
```csharp
using System.Collections.Generic;
using System.Diagnostics;
using Ory.Hydra.Client.Api;
using Ory.Hydra.Client.Client;
using Ory.Hydra.Client.Model;

namespace Example
{
    public class DiscoverOpenIDConfigurationExample
    {
        public static void Main()
        {
            Configuration config = new Configuration();
            config.BasePath = "http://localhost";
            var apiInstance = new PublicApi(config);

            try
            {
                // OpenID Connect Discovery
                HydraWellKnown result = apiInstance.DiscoverOpenIDConfiguration();
                Debug.WriteLine(result);
            }
            catch (ApiException  e)
            {
                Debug.Print("Exception when calling PublicApi.DiscoverOpenIDConfiguration: " + e.Message );
                Debug.Print("Status Code: "+ e.ErrorCode);
                Debug.Print(e.StackTrace);
            }
        }
    }
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

[**HydraWellKnown**](HydraWellKnown.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | wellKnown |  -  |
| **401** | jsonError |  -  |
| **500** | jsonError |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

<a name="isinstanceready"></a>
# **IsInstanceReady**
> HydraHealthStatus IsInstanceReady ()

Check Readiness Status

This endpoint returns a 200 status code when the HTTP server is up running and the environment dependencies (e.g. the database) are responsive as well.  If the service supports TLS Edge Termination, this endpoint does not require the `X-Forwarded-Proto` header to be set.  Be aware that if you are running multiple nodes of this service, the health status will never refer to the cluster state, only to a single instance.

### Example
```csharp
using System.Collections.Generic;
using System.Diagnostics;
using Ory.Hydra.Client.Api;
using Ory.Hydra.Client.Client;
using Ory.Hydra.Client.Model;

namespace Example
{
    public class IsInstanceReadyExample
    {
        public static void Main()
        {
            Configuration config = new Configuration();
            config.BasePath = "http://localhost";
            var apiInstance = new PublicApi(config);

            try
            {
                // Check Readiness Status
                HydraHealthStatus result = apiInstance.IsInstanceReady();
                Debug.WriteLine(result);
            }
            catch (ApiException  e)
            {
                Debug.Print("Exception when calling PublicApi.IsInstanceReady: " + e.Message );
                Debug.Print("Status Code: "+ e.ErrorCode);
                Debug.Print(e.StackTrace);
            }
        }
    }
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

[**HydraHealthStatus**](HydraHealthStatus.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | healthStatus |  -  |
| **503** | healthNotReadyStatus |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

<a name="oauth2token"></a>
# **Oauth2Token**
> HydraOauth2TokenResponse Oauth2Token (string grantType, string code = null, string refreshToken = null, string redirectUri = null, string clientId = null)

The OAuth 2.0 Token Endpoint

The client makes a request to the token endpoint by sending the following parameters using the \"application/x-www-form-urlencoded\" HTTP request entity-body.  > Do not implement a client for this endpoint yourself. Use a library. There are many libraries > available for any programming language. You can find a list of libraries here: https://oauth.net/code/ > > Do note that Hydra SDK does not implement this endpoint properly. Use one of the libraries listed above!

### Example
```csharp
using System.Collections.Generic;
using System.Diagnostics;
using Ory.Hydra.Client.Api;
using Ory.Hydra.Client.Client;
using Ory.Hydra.Client.Model;

namespace Example
{
    public class Oauth2TokenExample
    {
        public static void Main()
        {
            Configuration config = new Configuration();
            config.BasePath = "http://localhost";
            // Configure HTTP basic authorization: basic
            config.Username = "YOUR_USERNAME";
            config.Password = "YOUR_PASSWORD";
            // Configure OAuth2 access token for authorization: oauth2
            config.AccessToken = "YOUR_ACCESS_TOKEN";

            var apiInstance = new PublicApi(config);
            var grantType = grantType_example;  // string | 
            var code = code_example;  // string |  (optional) 
            var refreshToken = refreshToken_example;  // string |  (optional) 
            var redirectUri = redirectUri_example;  // string |  (optional) 
            var clientId = clientId_example;  // string |  (optional) 

            try
            {
                // The OAuth 2.0 Token Endpoint
                HydraOauth2TokenResponse result = apiInstance.Oauth2Token(grantType, code, refreshToken, redirectUri, clientId);
                Debug.WriteLine(result);
            }
            catch (ApiException  e)
            {
                Debug.Print("Exception when calling PublicApi.Oauth2Token: " + e.Message );
                Debug.Print("Status Code: "+ e.ErrorCode);
                Debug.Print(e.StackTrace);
            }
        }
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **grantType** | **string**|  | 
 **code** | **string**|  | [optional] 
 **refreshToken** | **string**|  | [optional] 
 **redirectUri** | **string**|  | [optional] 
 **clientId** | **string**|  | [optional] 

### Return type

[**HydraOauth2TokenResponse**](HydraOauth2TokenResponse.md)

### Authorization

[basic](../README.md#basic), [oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | oauth2TokenResponse |  -  |
| **400** | jsonError |  -  |
| **401** | jsonError |  -  |
| **500** | jsonError |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

<a name="oauthauth"></a>
# **OauthAuth**
> void OauthAuth ()

The OAuth 2.0 Authorize Endpoint

This endpoint is not documented here because you should never use your own implementation to perform OAuth2 flows. OAuth2 is a very popular protocol and a library for your programming language will exists.  To learn more about this flow please refer to the specification: https://tools.ietf.org/html/rfc6749

### Example
```csharp
using System.Collections.Generic;
using System.Diagnostics;
using Ory.Hydra.Client.Api;
using Ory.Hydra.Client.Client;
using Ory.Hydra.Client.Model;

namespace Example
{
    public class OauthAuthExample
    {
        public static void Main()
        {
            Configuration config = new Configuration();
            config.BasePath = "http://localhost";
            var apiInstance = new PublicApi(config);

            try
            {
                // The OAuth 2.0 Authorize Endpoint
                apiInstance.OauthAuth();
            }
            catch (ApiException  e)
            {
                Debug.Print("Exception when calling PublicApi.OauthAuth: " + e.Message );
                Debug.Print("Status Code: "+ e.ErrorCode);
                Debug.Print(e.StackTrace);
            }
        }
    }
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **302** | Empty responses are sent when, for example, resources are deleted. The HTTP status code for empty responses is typically 201. |  -  |
| **401** | jsonError |  -  |
| **500** | jsonError |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

<a name="revokeoauth2token"></a>
# **RevokeOAuth2Token**
> void RevokeOAuth2Token (string token)

Revoke OAuth2 Tokens

Revoking a token (both access and refresh) means that the tokens will be invalid. A revoked access token can no longer be used to make access requests, and a revoked refresh token can no longer be used to refresh an access token. Revoking a refresh token also invalidates the access token that was created with it. A token may only be revoked by the client the token was generated for.

### Example
```csharp
using System.Collections.Generic;
using System.Diagnostics;
using Ory.Hydra.Client.Api;
using Ory.Hydra.Client.Client;
using Ory.Hydra.Client.Model;

namespace Example
{
    public class RevokeOAuth2TokenExample
    {
        public static void Main()
        {
            Configuration config = new Configuration();
            config.BasePath = "http://localhost";
            // Configure HTTP basic authorization: basic
            config.Username = "YOUR_USERNAME";
            config.Password = "YOUR_PASSWORD";
            // Configure OAuth2 access token for authorization: oauth2
            config.AccessToken = "YOUR_ACCESS_TOKEN";

            var apiInstance = new PublicApi(config);
            var token = token_example;  // string | 

            try
            {
                // Revoke OAuth2 Tokens
                apiInstance.RevokeOAuth2Token(token);
            }
            catch (ApiException  e)
            {
                Debug.Print("Exception when calling PublicApi.RevokeOAuth2Token: " + e.Message );
                Debug.Print("Status Code: "+ e.ErrorCode);
                Debug.Print(e.StackTrace);
            }
        }
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **token** | **string**|  | 

### Return type

void (empty response body)

### Authorization

[basic](../README.md#basic), [oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: application/x-www-form-urlencoded
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | Empty responses are sent when, for example, resources are deleted. The HTTP status code for empty responses is typically 201. |  -  |
| **401** | jsonError |  -  |
| **500** | jsonError |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

<a name="userinfo"></a>
# **Userinfo**
> HydraUserinfoResponse Userinfo ()

OpenID Connect Userinfo

This endpoint returns the payload of the ID Token, including the idTokenExtra values, of the provided OAuth 2.0 Access Token.  For more information please [refer to the spec](http://openid.net/specs/openid-connect-core-1_0.html#UserInfo).  In the case of authentication error, a WWW-Authenticate header might be set in the response with more information about the error. See [the spec](https://datatracker.ietf.org/doc/html/rfc6750#section-3) for more details about header format.

### Example
```csharp
using System.Collections.Generic;
using System.Diagnostics;
using Ory.Hydra.Client.Api;
using Ory.Hydra.Client.Client;
using Ory.Hydra.Client.Model;

namespace Example
{
    public class UserinfoExample
    {
        public static void Main()
        {
            Configuration config = new Configuration();
            config.BasePath = "http://localhost";
            // Configure OAuth2 access token for authorization: oauth2
            config.AccessToken = "YOUR_ACCESS_TOKEN";

            var apiInstance = new PublicApi(config);

            try
            {
                // OpenID Connect Userinfo
                HydraUserinfoResponse result = apiInstance.Userinfo();
                Debug.WriteLine(result);
            }
            catch (ApiException  e)
            {
                Debug.Print("Exception when calling PublicApi.Userinfo: " + e.Message );
                Debug.Print("Status Code: "+ e.ErrorCode);
                Debug.Print(e.StackTrace);
            }
        }
    }
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

[**HydraUserinfoResponse**](HydraUserinfoResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | userinfoResponse |  -  |
| **401** | jsonError |  -  |
| **500** | jsonError |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

<a name="wellknown"></a>
# **WellKnown**
> HydraJSONWebKeySet WellKnown ()

JSON Web Keys Discovery

This endpoint returns JSON Web Keys to be used as public keys for verifying OpenID Connect ID Tokens and, if enabled, OAuth 2.0 JWT Access Tokens. This endpoint can be used with client libraries like [node-jwks-rsa](https://github.com/auth0/node-jwks-rsa) among others.

### Example
```csharp
using System.Collections.Generic;
using System.Diagnostics;
using Ory.Hydra.Client.Api;
using Ory.Hydra.Client.Client;
using Ory.Hydra.Client.Model;

namespace Example
{
    public class WellKnownExample
    {
        public static void Main()
        {
            Configuration config = new Configuration();
            config.BasePath = "http://localhost";
            var apiInstance = new PublicApi(config);

            try
            {
                // JSON Web Keys Discovery
                HydraJSONWebKeySet result = apiInstance.WellKnown();
                Debug.WriteLine(result);
            }
            catch (ApiException  e)
            {
                Debug.Print("Exception when calling PublicApi.WellKnown: " + e.Message );
                Debug.Print("Status Code: "+ e.ErrorCode);
                Debug.Print(e.StackTrace);
            }
        }
    }
}
```

### Parameters
This endpoint does not need any parameter.

### Return type

[**HydraJSONWebKeySet**](HydraJSONWebKeySet.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
| **200** | JSONWebKeySet |  -  |
| **500** | jsonError |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

