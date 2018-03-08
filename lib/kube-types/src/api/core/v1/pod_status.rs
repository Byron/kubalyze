// Generated from definition io.k8s.api.core.v1.PodStatus

/// PodStatus represents information about the status of a pod. Status may trail the actual state of a system.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PodStatus {
    /// Current service state of pod. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-conditions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<::api::core::v1::PodCondition>>,

    /// The list has one entry per container in the manifest. Each entry is currently the output of `docker inspect`. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status
    #[serde(rename = "containerStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_statuses: Option<Vec<::api::core::v1::ContainerStatus>>,

    /// IP address of the host to which the pod is assigned. Empty if not yet scheduled.
    #[serde(rename = "hostIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_ip: Option<String>,

    /// The list has one entry per init container in the manifest. The most recent successful init container will have ready = true, the most recently started container will have startTime set. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-and-container-status
    #[serde(rename = "initContainerStatuses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_container_statuses: Option<Vec<::api::core::v1::ContainerStatus>>,

    /// A human readable message indicating details about why the pod is in this condition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// nominatedNodeName is set only when this pod preempts other pods on the node, but it cannot be scheduled right away as preemption victims receive their graceful termination periods. This field does not guarantee that the pod will be scheduled on this node. Scheduler may decide to place the pod elsewhere if other nodes become available sooner. Scheduler may also decide to give the resources on this node to a higher priority pod that is created after preemption. As a result, this field may be different than PodSpec.nodeName when the pod is scheduled.
    #[serde(rename = "nominatedNodeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nominated_node_name: Option<String>,

    /// Current condition of the pod. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#pod-phase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,

    /// IP address allocated to the pod. Routable at least within the cluster. Empty if not yet allocated.
    #[serde(rename = "podIP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_ip: Option<String>,

    /// The Quality of Service (QOS) classification assigned to the pod based on resource requirements See PodQOSClass type for available QOS classes More info: https://git.k8s.io/community/contributors/design-proposals/node/resource-qos.md
    #[serde(rename = "qosClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos_class: Option<String>,

    /// A brief CamelCase message indicating details about why the pod is in this state. e.g. 'Evicted'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,

    /// RFC 3339 date and time at which the object was acknowledged by the Kubelet. This is before the Kubelet pulled the container image(s) for the pod.
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<::apimachinery::pkg::apis::meta::v1::Time>,
}
