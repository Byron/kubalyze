// Generated from definition io.k8s.kube-aggregator.pkg.apis.apiregistration.v1beta1.APIServiceSpec

/// APIServiceSpec contains information for locating and communicating with a server. Only https is supported, though you are able to disable certificate verification.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct APIServiceSpec {
    /// CABundle is a PEM encoded CA bundle which will be used to validate an API server's serving certificate.
    #[serde(rename = "caBundle")]
    pub ca_bundle: ::ByteString,

    /// Group is the API group name this server hosts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    /// GroupPriorityMininum is the priority this group should have at least. Higher priority means that the group is preferred by clients over lower priority ones. Note that other versions of this group might specify even higher GroupPriorityMininum values such that the whole group gets a higher priority. The primary sort is based on GroupPriorityMinimum, ordered highest number to lowest (20 before 10). The secondary sort is based on the alphabetical comparison of the name of the object.  (v1.bar before v1.foo) We'd recommend something like: *.k8s.io (except extensions) at 18000 and PaaSes (OpenShift, Deis) are recommended to be in the 2000s
    #[serde(rename = "groupPriorityMinimum")]
    pub group_priority_minimum: i32,

    /// InsecureSkipTLSVerify disables TLS certificate verification when communicating with this server. This is strongly discouraged.  You should use the CABundle instead.
    #[serde(rename = "insecureSkipTLSVerify")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insecure_skip_tls_verify: Option<bool>,

    /// Service is a reference to the service for this API server.  It must communicate on port 443 If the Service is nil, that means the handling for the API groupversion is handled locally on this server. The call will simply delegate to the normal handler chain to be fulfilled.
    pub service: ::kube_aggregator::pkg::apis::apiregistration::v1beta1::ServiceReference,

    /// Version is the API version this server hosts.  For example, "v1"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// VersionPriority controls the ordering of this API version inside of its group.  Must be greater than zero. The primary sort is based on VersionPriority, ordered highest to lowest (20 before 10). The secondary sort is based on the alphabetical comparison of the name of the object.  (v1.bar before v1.foo) Since it's inside of a group, the number can be small, probably in the 10s.
    #[serde(rename = "versionPriority")]
    pub version_priority: i32,
}
