// Generated from definition io.k8s.api.core.v1.ResourceQuota

/// ResourceQuota sets aggregate quota restrictions enforced per namespace
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ResourceQuota {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Spec defines the desired quota. https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<::api::core::v1::ResourceQuotaSpec>,

    /// Status defines the actual enforced quota and its current usage. https://git.k8s.io/community/contributors/devel/api-conventions.md#spec-and-status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<::api::core::v1::ResourceQuotaStatus>,
}
