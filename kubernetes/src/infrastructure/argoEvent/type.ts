export type MySchema =
    | IoArgoprojEventbusV1Alpha1EventBus
    | IoArgoprojEventsourceV1Alpha1EventSource
    | IoArgoprojSensorV1Alpha1Sensor
/**
 * Quantity is a fixed-point representation of a number. It provides convenient marshaling/unmarshaling in JSON and YAML, in addition to String() and AsInt64() accessors.
 *
 * The serialization format is:
 *
 * <quantity>        ::= <signedNumber><suffix>
 *   (Note that <suffix> may be empty, from the "" case in <decimalSI>.)
 * <digit>           ::= 0 | 1 | ... | 9 <digits>          ::= <digit> | <digit><digits> <number>          ::= <digits> | <digits>.<digits> | <digits>. | .<digits> <sign>            ::= "+" | "-" <signedNumber>    ::= <number> | <sign><number> <suffix>          ::= <binarySI> | <decimalExponent> | <decimalSI> <binarySI>        ::= Ki | Mi | Gi | Ti | Pi | Ei
 *   (International System of units; See: http://physics.nist.gov/cuu/Units/binary.html)
 * <decimalSI>       ::= m | "" | k | M | G | T | P | E
 *   (Note that 1024 = 1Ki but 1000 = 1k; I didn't choose the capitalization.)
 * <decimalExponent> ::= "e" <signedNumber> | "E" <signedNumber>
 *
 * No matter which of the three exponent forms is used, no quantity may represent a number greater than 2^63-1 in magnitude, nor may it have more than 3 decimal places. Numbers larger or more precise will be capped or rounded up. (E.g.: 0.1m will rounded up to 1m.) This may be extended in the future if we require larger or smaller quantities.
 *
 * When a Quantity is parsed from a string, it will remember the type of suffix it had, and will use the same type again when it is serialized.
 *
 * Before serializing, Quantity will be put in "canonical form". This means that Exponent/suffix will be adjusted up or down (with a corresponding increase or decrease in Mantissa) such that:
 *   a. No precision is lost
 *   b. No fractional digits will be emitted
 *   c. The exponent (or suffix) is as large as possible.
 * The sign will be omitted unless the number is negative.
 *
 * Examples:
 *   1.5 will be serialized as "1500m"
 *   1.5Gi will be serialized as "1536Mi"
 *
 * Note that the quantity will NEVER be internally represented by a floating point number. That is the whole point of this exercise.
 *
 * Non-canonical values will still parse as long as they are well formed, but will be re-emitted in their canonical form. (So always use canonical form, or don't diff.)
 *
 * This format is intended to make it difficult to use these numbers without writing some sort of special handling code in the hopes that that will cause implementors to also use a fixed point implementation.
 */
export type IoK8SApimachineryPkgApiResourceQuantity = string

/**
 * EventBus is the definition of a eventbus resource
 */
export interface IoArgoprojEventbusV1Alpha1EventBus {
    /**
     * APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
     */
    apiVersion?: "argoproj.io/v1alpha1"
    /**
     * Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
     */
    kind?: "EventBus"
    metadata: IoK8SApimachineryPkgApisMetaV1ObjectMeta
    spec: IoArgoprojEventbusV1Alpha1EventBusSpec
    status?: IoArgoprojEventbusV1Alpha1EventBusStatus
    [k: string]: unknown
}
/**
 * ObjectMeta is metadata that all persisted resources must have, which includes all objects users must create.
 */
export interface IoK8SApimachineryPkgApisMetaV1ObjectMeta {
    /**
     * Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: http://kubernetes.io/docs/user-guide/annotations
     */
    annotations?: {
        [k: string]: string
    }
    /**
     * The name of the cluster which the object belongs to. This is used to distinguish resources with same name and namespace in different clusters. This field is not set anywhere right now and apiserver is going to ignore it if set in create or update request.
     */
    clusterName?: string
    /**
     * CreationTimestamp is a timestamp representing the server time when this object was created. It is not guaranteed to be set in happens-before order across separate operations. Clients may not set this value. It is represented in RFC3339 form and is in UTC.
     *
     * Populated by the system. Read-only. Null for lists. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
     */
    creationTimestamp?: string
    /**
     * Number of seconds allowed for this object to gracefully terminate before it will be removed from the system. Only set when deletionTimestamp is also set. May only be shortened. Read-only.
     */
    deletionGracePeriodSeconds?: number
    /**
     * DeletionTimestamp is RFC 3339 date and time at which this resource will be deleted. This field is set by the server when a graceful deletion is requested by the user, and is not directly settable by a client. The resource is expected to be deleted (no longer visible from resource lists, and not reachable by name) after the time in this field, once the finalizers list is empty. As long as the finalizers list contains items, deletion is blocked. Once the deletionTimestamp is set, this value may not be unset or be set further into the future, although it may be shortened or the resource may be deleted prior to this time. For example, a user may request that a pod is deleted in 30 seconds. The Kubelet will react by sending a graceful termination signal to the containers in the pod. After that 30 seconds, the Kubelet will send a hard termination signal (SIGKILL) to the container and after cleanup, remove the pod from the API. In the presence of network partitions, this object may still exist after this timestamp, until an administrator or automated process can determine the resource is fully terminated. If not set, graceful deletion of the object has not been requested.
     *
     * Populated by the system when a graceful deletion is requested. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
     */
    deletionTimestamp?: string
    /**
     * Must be empty before the object is deleted from the registry. Each entry is an identifier for the responsible component that will remove the entry from the list. If the deletionTimestamp of the object is non-nil, entries in this list can only be removed. Finalizers may be processed and removed in any order.  Order is NOT enforced because it introduces significant risk of stuck finalizers. finalizers is a shared field, any actor with permission can reorder it. If the finalizer list is processed in order, then this can lead to a situation in which the component responsible for the first finalizer in the list is waiting for a signal (field value, external system, or other) produced by a component responsible for a finalizer later in the list, resulting in a deadlock. Without enforced ordering finalizers are free to order amongst themselves and are not vulnerable to ordering changes in the list.
     */
    finalizers?: string[]
    /**
     * GenerateName is an optional prefix, used by the server, to generate a unique name ONLY IF the Name field has not been provided. If this field is used, the name returned to the client will be different than the name passed. This value will also be combined with a unique suffix. The provided value has the same validation rules as the Name field, and may be truncated by the length of the suffix required to make the value unique on the server.
     *
     * If this field is specified and the generated name exists, the server will NOT return a 409 - instead, it will either return 201 Created or 500 with Reason ServerTimeout indicating a unique name could not be found in the time allotted, and the client should retry (optionally after the time indicated in the Retry-After header).
     *
     * Applied only if Name is not specified. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#idempotency
     */
    generateName?: string
    /**
     * A sequence number representing a specific generation of the desired state. Populated by the system. Read-only.
     */
    generation?: number
    /**
     * Map of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and services. More info: http://kubernetes.io/docs/user-guide/labels
     */
    labels?: {
        [k: string]: string
    }
    /**
     * ManagedFields maps workflow-id and version to the set of fields that are managed by that workflow. This is mostly for internal housekeeping, and users typically shouldn't need to set or understand this field. A workflow can be the user's name, a controller's name, or the name of a specific apply path like "ci-cd". The set of fields is always in the version that the workflow used when modifying the object.
     */
    managedFields?: IoK8SApimachineryPkgApisMetaV1ManagedFieldsEntry[]
    /**
     * Name must be unique within a namespace. Is required when creating resources, although some resources may allow a client to request the generation of an appropriate name automatically. Name is primarily intended for creation idempotence and configuration definition. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/identifiers#names
     */
    name?: string
    /**
     * Namespace defines the space within which each name must be unique. An empty namespace is equivalent to the "default" namespace, but "default" is the canonical representation. Not all objects are required to be scoped to a namespace - the value of this field for those objects will be empty.
     *
     * Must be a DNS_LABEL. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/namespaces
     */
    namespace?: string
    /**
     * List of objects depended by this object. If ALL objects in the list have been deleted, this object will be garbage collected. If this object is managed by a controller, then an entry in this list will point to this controller, with the controller field set to true. There cannot be more than one managing controller.
     */
    ownerReferences?: IoK8SApimachineryPkgApisMetaV1OwnerReference[]
    /**
     * An opaque value that represents the internal version of this object that can be used by clients to determine when objects have changed. May be used for optimistic concurrency, change detection, and the watch operation on a resource or set of resources. Clients must treat these values as opaque and passed unmodified back to the server. They may only be valid for a particular resource or set of resources.
     *
     * Populated by the system. Read-only. Value must be treated as opaque by clients and . More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
     */
    resourceVersion?: string
    /**
     * SelfLink is a URL representing this object. Populated by the system. Read-only.
     *
     * DEPRECATED Kubernetes will stop propagating this field in 1.20 release and the field is planned to be removed in 1.21 release.
     */
    selfLink?: string
    /**
     * UID is the unique in time and space value for this object. It is typically generated by the server on successful creation of a resource and is not allowed to change on PUT operations.
     *
     * Populated by the system. Read-only. More info: http://kubernetes.io/docs/user-guide/identifiers#uids
     */
    uid?: string
    [k: string]: unknown
}
/**
 * ManagedFieldsEntry is a workflow-id, a FieldSet and the group version of the resource that the fieldset applies to.
 */
export interface IoK8SApimachineryPkgApisMetaV1ManagedFieldsEntry {
    /**
     * APIVersion defines the version of this resource that this field set applies to. The format is "group/version" just like the top-level APIVersion field. It is necessary to track the version of a field set because it cannot be automatically converted.
     */
    apiVersion?: string
    /**
     * FieldsType is the discriminator for the different fields format and version. There is currently only one possible value: "FieldsV1"
     */
    fieldsType?: string
    /**
     * FieldsV1 holds the first JSON version format as described in the "FieldsV1" type.
     */
    fieldsV1?: {
        [k: string]: unknown
    }
    /**
     * Manager is an identifier of the workflow managing these fields.
     */
    manager?: string
    /**
     * Operation is the type of operation which lead to this ManagedFieldsEntry being created. The only valid values for this field are 'Apply' and 'Update'.
     */
    operation?: string
    /**
     * Time is timestamp of when these fields were set. It should always be empty if Operation is 'Apply'
     */
    time?: string
    [k: string]: unknown
}
/**
 * OwnerReference contains enough information to let you identify an owning object. An owning object must be in the same namespace as the dependent, or be cluster-scoped, so there is no namespace field.
 */
export interface IoK8SApimachineryPkgApisMetaV1OwnerReference {
    /**
     * API version of the referent.
     */
    apiVersion: string
    /**
     * If true, AND if the owner has the "foregroundDeletion" finalizer, then the owner cannot be deleted from the key-value store until this reference is removed. Defaults to false. To set this field, a user needs "delete" permission of the owner, otherwise 422 (Unprocessable Entity) will be returned.
     */
    blockOwnerDeletion?: boolean
    /**
     * If true, this reference points to the managing controller.
     */
    controller?: boolean
    /**
     * Kind of the referent. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
     */
    kind: string
    /**
     * Name of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#names
     */
    name: string
    /**
     * UID of the referent. More info: http://kubernetes.io/docs/user-guide/identifiers#uids
     */
    uid: string
    [k: string]: unknown
}
/**
 * EventBusSpec refers to specification of eventbus resource
 */
export interface IoArgoprojEventbusV1Alpha1EventBusSpec {
    jetstream?: IoArgoprojEventbusV1Alpha1JetStreamBus
    /**
     * NATS eventbus
     */
    nats?: {
        /**
         * NATSConfig holds the config of NATS
         */
        exotic?: {
            /**
             * SecretKeySelector selects a key of a Secret.
             */
            accessSecret?: {
                /**
                 * The key of the secret to select from.  Must be a valid secret key.
                 */
                key: string
                /**
                 * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                 */
                name?: string
                /**
                 * Specify whether the Secret or its key must be defined
                 */
                optional?: boolean
                [k: string]: unknown
            }
            /**
             * Auth strategy, default to AuthStrategyNone
             */
            auth?: string
            /**
             * Cluster ID for nats streaming
             */
            clusterID?: string
            /**
             * NATS streaming url
             */
            url?: string
            [k: string]: unknown
        }
        /**
         * Native means to bring up a native NATS service
         */
        native?: {
            /**
             * The pod's scheduling constraints More info: https://kubernetes.io/docs/concepts/scheduling-eviction/assign-pod-node/
             */
            affinity?: {
                /**
                 * Describes node affinity scheduling rules for the pod.
                 */
                nodeAffinity?: {
                    /**
                     * The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node matches the corresponding matchExpressions; the node(s) with the highest sum are the most preferred.
                     */
                    preferredDuringSchedulingIgnoredDuringExecution?: IoK8SApiCoreV1PreferredSchedulingTerm[]
                    /**
                     * If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to an update), the system may or may not try to eventually evict the pod from its node.
                     */
                    requiredDuringSchedulingIgnoredDuringExecution?: {
                        /**
                         * Required. A list of node selector terms. The terms are ORed.
                         */
                        nodeSelectorTerms: IoK8SApiCoreV1NodeSelectorTerm[]
                        [k: string]: unknown
                    }
                    [k: string]: unknown
                }
                /**
                 * Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
                 */
                podAffinity?: {
                    /**
                     * The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
                     */
                    preferredDuringSchedulingIgnoredDuringExecution?: IoK8SApiCoreV1WeightedPodAffinityTerm[]
                    /**
                     * If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
                     */
                    requiredDuringSchedulingIgnoredDuringExecution?: IoK8SApiCoreV1PodAffinityTerm[]
                    [k: string]: unknown
                }
                /**
                 * Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
                 */
                podAntiAffinity?: {
                    /**
                     * The scheduler will prefer to schedule pods to nodes that satisfy the anti-affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling anti-affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
                     */
                    preferredDuringSchedulingIgnoredDuringExecution?: IoK8SApiCoreV1WeightedPodAffinityTerm[]
                    /**
                     * If the anti-affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the anti-affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
                     */
                    requiredDuringSchedulingIgnoredDuringExecution?: IoK8SApiCoreV1PodAffinityTerm[]
                    [k: string]: unknown
                }
                [k: string]: unknown
            }
            auth?: string
            /**
             * ContainerTemplate contains customized spec for NATS container
             */
            containerTemplate?: {
                imagePullPolicy?: string
                resources?: IoK8SApiCoreV1ResourceRequirements
                securityContext?: IoK8SApiCoreV1SecurityContext
                [k: string]: unknown
            }
            /**
             * ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec. If specified, these secrets will be passed to individual puller implementations for them to use. For example, in the case of docker, only DockerConfig type secrets are honored. More info: https://kubernetes.io/docs/concepts/containers/images#specifying-imagepullsecrets-on-a-pod
             */
            imagePullSecrets?: IoK8SApiCoreV1LocalObjectReference[]
            /**
             * Max Age of existing messages, i.e. "72h", “4h35m”
             */
            maxAge?: string
            /**
             * Total size of messages per channel, 0 means unlimited. Defaults to 1GB
             */
            maxBytes?: string
            /**
             * Maximum number of messages per channel, 0 means unlimited. Defaults to 1000000
             */
            maxMsgs?: number
            /**
             * Maximum number of bytes in a message payload, 0 means unlimited. Defaults to 1MB
             */
            maxPayload?: string
            /**
             * Maximum number of subscriptions per channel, 0 means unlimited. Defaults to 1000
             */
            maxSubs?: number
            /**
             * Metadata sets the pods's metadata, i.e. annotations and labels
             */
            metadata?: {
                annotations?: {
                    [k: string]: string
                }
                labels?: {
                    [k: string]: string
                }
                [k: string]: unknown
            }
            /**
             * MetricsContainerTemplate contains customized spec for metrics container
             */
            metricsContainerTemplate?: {
                imagePullPolicy?: string
                resources?: IoK8SApiCoreV1ResourceRequirements
                securityContext?: IoK8SApiCoreV1SecurityContext
                [k: string]: unknown
            }
            /**
             * NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
             */
            nodeSelector?: {
                [k: string]: string
            }
            persistence?: IoArgoprojEventbusV1Alpha1PersistenceStrategy
            /**
             * The priority value. Various system components use this field to find the priority of the EventSource pod. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority. More info: https://kubernetes.io/docs/concepts/configuration/pod-priority-preemption/
             */
            priority?: number
            /**
             * If specified, indicates the EventSource pod's priority. "system-node-critical" and "system-cluster-critical" are two special keywords which indicate the highest priorities with the former being the highest priority. Any other name must be defined by creating a PriorityClass object with that name. If not specified, the pod priority will be default or zero if there is no default. More info: https://kubernetes.io/docs/concepts/configuration/pod-priority-preemption/
             */
            priorityClassName?: string
            /**
             * Specifies the time without an Apply() operation before sending an heartbeat to ensure timely commit, i.e. "72h", “4h35m”. Defaults to 100ms
             */
            raftCommitTimeout?: string
            /**
             * Specifies the time in candidate state without a leader before attempting an election, i.e. "72h", “4h35m”. Defaults to 2s
             */
            raftElectionTimeout?: string
            /**
             * Specifies the time in follower state without a leader before attempting an election, i.e. "72h", “4h35m”. Defaults to 2s
             */
            raftHeartbeatTimeout?: string
            /**
             * Specifies how long a leader waits without being able to contact a quorum of nodes before stepping down as leader, i.e. "72h", “4h35m”. Defaults to 1s
             */
            raftLeaseTimeout?: string
            /**
             * Size is the NATS StatefulSet size
             */
            replicas?: number
            /**
             * SecurityContext holds pod-level security attributes and common container settings. Optional: Defaults to empty.  See type description for default values of each field.
             */
            securityContext?: {
                /**
                 * A special supplemental group that applies to all containers in a pod. Some volume types allow the Kubelet to change the ownership of that volume to be owned by the pod:
                 *
                 * 1. The owning GID will be the FSGroup 2. The setgid bit is set (new files created in the volume will be owned by FSGroup) 3. The permission bits are OR'd with rw-rw----
                 *
                 * If unset, the Kubelet will not modify the ownership and permissions of any volume.
                 */
                fsGroup?: number
                /**
                 * fsGroupChangePolicy defines behavior of changing ownership and permission of the volume before being exposed inside Pod. This field will only apply to volume types which support fsGroup based ownership(and permissions). It will have no effect on ephemeral volume types such as: secret, configmaps and emptydir. Valid values are "OnRootMismatch" and "Always". If not specified, "Always" is used.
                 */
                fsGroupChangePolicy?: string
                /**
                 * The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container.
                 */
                runAsGroup?: number
                /**
                 * Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
                 */
                runAsNonRoot?: boolean
                /**
                 * The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container.
                 */
                runAsUser?: number
                /**
                 * The SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container.
                 */
                seLinuxOptions?: {
                    /**
                     * Level is SELinux level label that applies to the container.
                     */
                    level?: string
                    /**
                     * Role is a SELinux role label that applies to the container.
                     */
                    role?: string
                    /**
                     * Type is a SELinux type label that applies to the container.
                     */
                    type?: string
                    /**
                     * User is a SELinux user label that applies to the container.
                     */
                    user?: string
                    [k: string]: unknown
                }
                /**
                 * The seccomp options to use by the containers in this pod.
                 */
                seccompProfile?: {
                    /**
                     * localhostProfile indicates a profile defined in a file on the node should be used. The profile must be preconfigured on the node to work. Must be a descending path, relative to the kubelet's configured seccomp profile location. Must only be set if type is "Localhost".
                     */
                    localhostProfile?: string
                    /**
                     * type indicates which kind of seccomp profile will be applied. Valid options are:
                     *
                     * Localhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.
                     */
                    type: string
                    [k: string]: unknown
                }
                /**
                 * A list of groups applied to the first process run in each container, in addition to the container's primary GID.  If unspecified, no groups will be added to any container.
                 */
                supplementalGroups?: number[]
                /**
                 * Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported sysctls (by the container runtime) might fail to launch.
                 */
                sysctls?: IoK8SApiCoreV1Sysctl[]
                /**
                 * The Windows specific settings applied to all containers. If unspecified, the options within a container's SecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
                 */
                windowsOptions?: {
                    /**
                     * GMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field.
                     */
                    gmsaCredentialSpec?: string
                    /**
                     * GMSACredentialSpecName is the name of the GMSA credential spec to use.
                     */
                    gmsaCredentialSpecName?: string
                    /**
                     * The UserName in Windows to run the entrypoint of the container process. Defaults to the user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
                     */
                    runAsUserName?: string
                    [k: string]: unknown
                }
                [k: string]: unknown
            }
            /**
             * ServiceAccountName to apply to NATS StatefulSet
             */
            serviceAccountName?: string
            /**
             * If specified, the pod's tolerations.
             */
            tolerations?: IoK8SApiCoreV1Toleration[]
            [k: string]: unknown
        }
        [k: string]: unknown
    }
    [k: string]: unknown
}
/**
 * JetStreamBus holds the JetStream EventBus information
 */
export interface IoArgoprojEventbusV1Alpha1JetStreamBus {
    /**
     * The pod's scheduling constraints More info: https://kubernetes.io/docs/concepts/scheduling-eviction/assign-pod-node/
     */
    affinity?: {
        /**
         * Describes node affinity scheduling rules for the pod.
         */
        nodeAffinity?: {
            /**
             * The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node matches the corresponding matchExpressions; the node(s) with the highest sum are the most preferred.
             */
            preferredDuringSchedulingIgnoredDuringExecution?: IoK8SApiCoreV1PreferredSchedulingTerm[]
            /**
             * If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to an update), the system may or may not try to eventually evict the pod from its node.
             */
            requiredDuringSchedulingIgnoredDuringExecution?: {
                /**
                 * Required. A list of node selector terms. The terms are ORed.
                 */
                nodeSelectorTerms: IoK8SApiCoreV1NodeSelectorTerm[]
                [k: string]: unknown
            }
            [k: string]: unknown
        }
        /**
         * Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
         */
        podAffinity?: {
            /**
             * The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
             */
            preferredDuringSchedulingIgnoredDuringExecution?: IoK8SApiCoreV1WeightedPodAffinityTerm[]
            /**
             * If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
             */
            requiredDuringSchedulingIgnoredDuringExecution?: IoK8SApiCoreV1PodAffinityTerm[]
            [k: string]: unknown
        }
        /**
         * Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
         */
        podAntiAffinity?: {
            /**
             * The scheduler will prefer to schedule pods to nodes that satisfy the anti-affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling anti-affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
             */
            preferredDuringSchedulingIgnoredDuringExecution?: IoK8SApiCoreV1WeightedPodAffinityTerm[]
            /**
             * If the anti-affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the anti-affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
             */
            requiredDuringSchedulingIgnoredDuringExecution?: IoK8SApiCoreV1PodAffinityTerm[]
            [k: string]: unknown
        }
        [k: string]: unknown
    }
    /**
     * ContainerTemplate contains customized spec for Nats JetStream container
     */
    containerTemplate?: {
        imagePullPolicy?: string
        resources?: IoK8SApiCoreV1ResourceRequirements
        securityContext?: IoK8SApiCoreV1SecurityContext
        [k: string]: unknown
    }
    /**
     * ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec. If specified, these secrets will be passed to individual puller implementations for them to use. For example, in the case of docker, only DockerConfig type secrets are honored. More info: https://kubernetes.io/docs/concepts/containers/images#specifying-imagepullsecrets-on-a-pod
     */
    imagePullSecrets?: IoK8SApiCoreV1LocalObjectReference[]
    /**
     * Maximum number of bytes in a message payload, 0 means unlimited. Defaults to 1MB
     */
    maxPayload?: string
    /**
     * Metadata sets the pods's metadata, i.e. annotations and labels
     */
    metadata?: {
        annotations?: {
            [k: string]: string
        }
        labels?: {
            [k: string]: string
        }
        [k: string]: unknown
    }
    /**
     * MetricsContainerTemplate contains customized spec for metrics container
     */
    metricsContainerTemplate?: {
        imagePullPolicy?: string
        resources?: IoK8SApiCoreV1ResourceRequirements
        securityContext?: IoK8SApiCoreV1SecurityContext
        [k: string]: unknown
    }
    /**
     * NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
     */
    nodeSelector?: {
        [k: string]: string
    }
    persistence?: IoArgoprojEventbusV1Alpha1PersistenceStrategy
    /**
     * The priority value. Various system components use this field to find the priority of the Redis pod. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority. More info: https://kubernetes.io/docs/concepts/configuration/pod-priority-preemption/
     */
    priority?: number
    /**
     * If specified, indicates the Redis pod's priority. "system-node-critical" and "system-cluster-critical" are two special keywords which indicate the highest priorities with the former being the highest priority. Any other name must be defined by creating a PriorityClass object with that name. If not specified, the pod priority will be default or zero if there is no default. More info: https://kubernetes.io/docs/concepts/configuration/pod-priority-preemption/
     */
    priorityClassName?: string
    /**
     * ReloaderContainerTemplate contains customized spec for config reloader container
     */
    reloaderContainerTemplate?: {
        imagePullPolicy?: string
        resources?: IoK8SApiCoreV1ResourceRequirements
        securityContext?: IoK8SApiCoreV1SecurityContext
        [k: string]: unknown
    }
    /**
     * Redis StatefulSet size
     */
    replicas?: number
    /**
     * SecurityContext holds pod-level security attributes and common container settings. Optional: Defaults to empty.  See type description for default values of each field.
     */
    securityContext?: {
        /**
         * A special supplemental group that applies to all containers in a pod. Some volume types allow the Kubelet to change the ownership of that volume to be owned by the pod:
         *
         * 1. The owning GID will be the FSGroup 2. The setgid bit is set (new files created in the volume will be owned by FSGroup) 3. The permission bits are OR'd with rw-rw----
         *
         * If unset, the Kubelet will not modify the ownership and permissions of any volume.
         */
        fsGroup?: number
        /**
         * fsGroupChangePolicy defines behavior of changing ownership and permission of the volume before being exposed inside Pod. This field will only apply to volume types which support fsGroup based ownership(and permissions). It will have no effect on ephemeral volume types such as: secret, configmaps and emptydir. Valid values are "OnRootMismatch" and "Always". If not specified, "Always" is used.
         */
        fsGroupChangePolicy?: string
        /**
         * The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container.
         */
        runAsGroup?: number
        /**
         * Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
         */
        runAsNonRoot?: boolean
        /**
         * The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container.
         */
        runAsUser?: number
        /**
         * The SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container.
         */
        seLinuxOptions?: {
            /**
             * Level is SELinux level label that applies to the container.
             */
            level?: string
            /**
             * Role is a SELinux role label that applies to the container.
             */
            role?: string
            /**
             * Type is a SELinux type label that applies to the container.
             */
            type?: string
            /**
             * User is a SELinux user label that applies to the container.
             */
            user?: string
            [k: string]: unknown
        }
        /**
         * The seccomp options to use by the containers in this pod.
         */
        seccompProfile?: {
            /**
             * localhostProfile indicates a profile defined in a file on the node should be used. The profile must be preconfigured on the node to work. Must be a descending path, relative to the kubelet's configured seccomp profile location. Must only be set if type is "Localhost".
             */
            localhostProfile?: string
            /**
             * type indicates which kind of seccomp profile will be applied. Valid options are:
             *
             * Localhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.
             */
            type: string
            [k: string]: unknown
        }
        /**
         * A list of groups applied to the first process run in each container, in addition to the container's primary GID.  If unspecified, no groups will be added to any container.
         */
        supplementalGroups?: number[]
        /**
         * Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported sysctls (by the container runtime) might fail to launch.
         */
        sysctls?: IoK8SApiCoreV1Sysctl[]
        /**
         * The Windows specific settings applied to all containers. If unspecified, the options within a container's SecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
         */
        windowsOptions?: {
            /**
             * GMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field.
             */
            gmsaCredentialSpec?: string
            /**
             * GMSACredentialSpecName is the name of the GMSA credential spec to use.
             */
            gmsaCredentialSpecName?: string
            /**
             * The UserName in Windows to run the entrypoint of the container process. Defaults to the user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
             */
            runAsUserName?: string
            [k: string]: unknown
        }
        [k: string]: unknown
    }
    /**
     * ServiceAccountName to apply to the StatefulSet
     */
    serviceAccountName?: string
    /**
     * JetStream configuration, if not specified, global settings in controller-config will be used. See https://docs.nats.io/running-a-nats-service/configuration#jetstream. Only configure "max_memory_store" or "max_file_store", do not set "store_dir" as it has been hardcoded.
     */
    settings?: string
    /**
     * Optional arguments to start nats-server. For example, "-D" to enable debugging output, "-DV" to enable debugging and tracing. Check https://docs.nats.io/ for all the available arguments.
     */
    startArgs?: string[]
    /**
     * Optional configuration for the streams to be created in this JetStream service, if specified, it will be merged with the default configuration in controller-config. It accepts a YAML format configuration, available fields include, "maxBytes", "maxMsgs", "maxAge" (e.g. 72h), "replicas" (1, 3, 5), "duplicates" (e.g. 5m).
     */
    streamConfig?: string
    /**
     * If specified, the pod's tolerations.
     */
    tolerations?: IoK8SApiCoreV1Toleration[]
    /**
     * JetStream version, such as "2.7.3"
     */
    version?: string
    [k: string]: unknown
}
/**
 * An empty preferred scheduling term matches all objects with implicit weight 0 (i.e. it's a no-op). A null preferred scheduling term matches no objects (i.e. is also a no-op).
 */
export interface IoK8SApiCoreV1PreferredSchedulingTerm {
    /**
     * A node selector term, associated with the corresponding weight.
     */
    preference: {
        /**
         * A list of node selector requirements by node's labels.
         */
        matchExpressions?: IoK8SApiCoreV1NodeSelectorRequirement[]
        /**
         * A list of node selector requirements by node's fields.
         */
        matchFields?: IoK8SApiCoreV1NodeSelectorRequirement[]
        [k: string]: unknown
    }
    /**
     * Weight associated with matching the corresponding nodeSelectorTerm, in the range 1-100.
     */
    weight: number
    [k: string]: unknown
}
/**
 * A node selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
 */
export interface IoK8SApiCoreV1NodeSelectorRequirement {
    /**
     * The label key that the selector applies to.
     */
    key: string
    /**
     * Represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists, DoesNotExist. Gt, and Lt.
     */
    operator: string
    /**
     * An array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. If the operator is Gt or Lt, the values array must have a single element, which will be interpreted as an integer. This array is replaced during a strategic merge patch.
     */
    values?: string[]
    [k: string]: unknown
}
/**
 * A null or empty node selector term matches no objects. The requirements of them are ANDed. The TopologySelectorTerm type implements a subset of the NodeSelectorTerm.
 */
export interface IoK8SApiCoreV1NodeSelectorTerm {
    /**
     * A list of node selector requirements by node's labels.
     */
    matchExpressions?: IoK8SApiCoreV1NodeSelectorRequirement[]
    /**
     * A list of node selector requirements by node's fields.
     */
    matchFields?: IoK8SApiCoreV1NodeSelectorRequirement[]
    [k: string]: unknown
}
/**
 * The weights of all of the matched WeightedPodAffinityTerm fields are added per-node to find the most preferred node(s)
 */
export interface IoK8SApiCoreV1WeightedPodAffinityTerm {
    /**
     * Required. A pod affinity term, associated with the corresponding weight.
     */
    podAffinityTerm: {
        /**
         * A label query over a set of resources, in this case pods.
         */
        labelSelector?: {
            /**
             * matchExpressions is a list of label selector requirements. The requirements are ANDed.
             */
            matchExpressions?: IoK8SApimachineryPkgApisMetaV1LabelSelectorRequirement[]
            /**
             * matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
             */
            matchLabels?: {
                [k: string]: string
            }
            [k: string]: unknown
        }
        /**
         * namespaces specifies which namespaces the labelSelector applies to (matches against); null or empty list means "this pod's namespace"
         */
        namespaces?: string[]
        /**
         * This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
         */
        topologyKey: string
        [k: string]: unknown
    }
    /**
     * weight associated with matching the corresponding podAffinityTerm, in the range 1-100.
     */
    weight: number
    [k: string]: unknown
}
/**
 * A label selector requirement is a selector that contains values, a key, and an operator that relates the key and values.
 */
export interface IoK8SApimachineryPkgApisMetaV1LabelSelectorRequirement {
    /**
     * key is the label key that the selector applies to.
     */
    key: string
    /**
     * operator represents a key's relationship to a set of values. Valid operators are In, NotIn, Exists and DoesNotExist.
     */
    operator: string
    /**
     * values is an array of string values. If the operator is In or NotIn, the values array must be non-empty. If the operator is Exists or DoesNotExist, the values array must be empty. This array is replaced during a strategic merge patch.
     */
    values?: string[]
    [k: string]: unknown
}
/**
 * Defines a set of pods (namely those matching the labelSelector relative to the given namespace(s)) that this pod should be co-located (affinity) or not co-located (anti-affinity) with, where co-located is defined as running on a node whose value of the label with key <topologyKey> matches that of any node on which a pod of the set of pods is running
 */
export interface IoK8SApiCoreV1PodAffinityTerm {
    /**
     * A label query over a set of resources, in this case pods.
     */
    labelSelector?: {
        /**
         * matchExpressions is a list of label selector requirements. The requirements are ANDed.
         */
        matchExpressions?: IoK8SApimachineryPkgApisMetaV1LabelSelectorRequirement[]
        /**
         * matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
         */
        matchLabels?: {
            [k: string]: string
        }
        [k: string]: unknown
    }
    /**
     * namespaces specifies which namespaces the labelSelector applies to (matches against); null or empty list means "this pod's namespace"
     */
    namespaces?: string[]
    /**
     * This pod should be co-located (affinity) or not co-located (anti-affinity) with the pods matching the labelSelector in the specified namespaces, where co-located is defined as running on a node whose value of the label with key topologyKey matches that of any node on which any of the selected pods is running. Empty topologyKey is not allowed.
     */
    topologyKey: string
    [k: string]: unknown
}
/**
 * ResourceRequirements describes the compute resource requirements.
 */
export interface IoK8SApiCoreV1ResourceRequirements {
    /**
     * Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/
     */
    limits?: {
        [k: string]: IoK8SApimachineryPkgApiResourceQuantity
    }
    /**
     * Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/
     */
    requests?: {
        [k: string]: IoK8SApimachineryPkgApiResourceQuantity
    }
    [k: string]: unknown
}
/**
 * SecurityContext holds security configuration that will be applied to a container. Some fields are present in both SecurityContext and PodSecurityContext.  When both are set, the values in SecurityContext take precedence.
 */
export interface IoK8SApiCoreV1SecurityContext {
    /**
     * AllowPrivilegeEscalation controls whether a process can gain more privileges than its parent process. This bool directly controls if the no_new_privs flag will be set on the container process. AllowPrivilegeEscalation is true always when the container is: 1) run as Privileged 2) has CAP_SYS_ADMIN
     */
    allowPrivilegeEscalation?: boolean
    /**
     * The capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime.
     */
    capabilities?: {
        /**
         * Added capabilities
         */
        add?: string[]
        /**
         * Removed capabilities
         */
        drop?: string[]
        [k: string]: unknown
    }
    /**
     * Run container in privileged mode. Processes in privileged containers are essentially equivalent to root on the host. Defaults to false.
     */
    privileged?: boolean
    /**
     * procMount denotes the type of proc mount to use for the containers. The default is DefaultProcMount which uses the container runtime defaults for readonly paths and masked paths. This requires the ProcMountType feature flag to be enabled.
     */
    procMount?: string
    /**
     * Whether this container has a read-only root filesystem. Default is false.
     */
    readOnlyRootFilesystem?: boolean
    /**
     * The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
     */
    runAsGroup?: number
    /**
     * Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
     */
    runAsNonRoot?: boolean
    /**
     * The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
     */
    runAsUser?: number
    /**
     * The SELinux context to be applied to the container. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
     */
    seLinuxOptions?: {
        /**
         * Level is SELinux level label that applies to the container.
         */
        level?: string
        /**
         * Role is a SELinux role label that applies to the container.
         */
        role?: string
        /**
         * Type is a SELinux type label that applies to the container.
         */
        type?: string
        /**
         * User is a SELinux user label that applies to the container.
         */
        user?: string
        [k: string]: unknown
    }
    /**
     * The seccomp options to use by this container. If seccomp options are provided at both the pod & container level, the container options override the pod options.
     */
    seccompProfile?: {
        /**
         * localhostProfile indicates a profile defined in a file on the node should be used. The profile must be preconfigured on the node to work. Must be a descending path, relative to the kubelet's configured seccomp profile location. Must only be set if type is "Localhost".
         */
        localhostProfile?: string
        /**
         * type indicates which kind of seccomp profile will be applied. Valid options are:
         *
         * Localhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.
         */
        type: string
        [k: string]: unknown
    }
    /**
     * The Windows specific settings applied to all containers. If unspecified, the options from the PodSecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
     */
    windowsOptions?: {
        /**
         * GMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field.
         */
        gmsaCredentialSpec?: string
        /**
         * GMSACredentialSpecName is the name of the GMSA credential spec to use.
         */
        gmsaCredentialSpecName?: string
        /**
         * The UserName in Windows to run the entrypoint of the container process. Defaults to the user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
         */
        runAsUserName?: string
        [k: string]: unknown
    }
    [k: string]: unknown
}
/**
 * LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
 */
export interface IoK8SApiCoreV1LocalObjectReference {
    /**
     * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
     */
    name?: string
    [k: string]: unknown
}
/**
 * PersistenceStrategy defines the strategy of persistence
 */
export interface IoArgoprojEventbusV1Alpha1PersistenceStrategy {
    /**
     * Available access modes such as ReadWriteOnce, ReadWriteMany https://kubernetes.io/docs/concepts/storage/persistent-volumes/#access-modes
     */
    accessMode?: string
    /**
     * Name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1
     */
    storageClassName?: string
    /**
     * Quantity is a fixed-point representation of a number. It provides convenient marshaling/unmarshaling in JSON and YAML, in addition to String() and AsInt64() accessors.
     *
     * The serialization format is:
     *
     * <quantity>        ::= <signedNumber><suffix>
     *   (Note that <suffix> may be empty, from the "" case in <decimalSI>.)
     * <digit>           ::= 0 | 1 | ... | 9 <digits>          ::= <digit> | <digit><digits> <number>          ::= <digits> | <digits>.<digits> | <digits>. | .<digits> <sign>            ::= "+" | "-" <signedNumber>    ::= <number> | <sign><number> <suffix>          ::= <binarySI> | <decimalExponent> | <decimalSI> <binarySI>        ::= Ki | Mi | Gi | Ti | Pi | Ei
     *   (International System of units; See: http://physics.nist.gov/cuu/Units/binary.html)
     * <decimalSI>       ::= m | "" | k | M | G | T | P | E
     *   (Note that 1024 = 1Ki but 1000 = 1k; I didn't choose the capitalization.)
     * <decimalExponent> ::= "e" <signedNumber> | "E" <signedNumber>
     *
     * No matter which of the three exponent forms is used, no quantity may represent a number greater than 2^63-1 in magnitude, nor may it have more than 3 decimal places. Numbers larger or more precise will be capped or rounded up. (E.g.: 0.1m will rounded up to 1m.) This may be extended in the future if we require larger or smaller quantities.
     *
     * When a Quantity is parsed from a string, it will remember the type of suffix it had, and will use the same type again when it is serialized.
     *
     * Before serializing, Quantity will be put in "canonical form". This means that Exponent/suffix will be adjusted up or down (with a corresponding increase or decrease in Mantissa) such that:
     *   a. No precision is lost
     *   b. No fractional digits will be emitted
     *   c. The exponent (or suffix) is as large as possible.
     * The sign will be omitted unless the number is negative.
     *
     * Examples:
     *   1.5 will be serialized as "1500m"
     *   1.5Gi will be serialized as "1536Mi"
     *
     * Note that the quantity will NEVER be internally represented by a floating point number. That is the whole point of this exercise.
     *
     * Non-canonical values will still parse as long as they are well formed, but will be re-emitted in their canonical form. (So always use canonical form, or don't diff.)
     *
     * This format is intended to make it difficult to use these numbers without writing some sort of special handling code in the hopes that that will cause implementors to also use a fixed point implementation.
     */
    volumeSize?: string
    [k: string]: unknown
}
/**
 * Sysctl defines a kernel parameter to be set
 */
export interface IoK8SApiCoreV1Sysctl {
    /**
     * Name of a property to set
     */
    name: string
    /**
     * Value of a property to set
     */
    value: string
    [k: string]: unknown
}
/**
 * The pod this Toleration is attached to tolerates any taint that matches the triple <key,value,effect> using the matching operator <operator>.
 */
export interface IoK8SApiCoreV1Toleration {
    /**
     * Effect indicates the taint effect to match. Empty means match all taint effects. When specified, allowed values are NoSchedule, PreferNoSchedule and NoExecute.
     */
    effect?: string
    /**
     * Key is the taint key that the toleration applies to. Empty means match all taint keys. If the key is empty, operator must be Exists; this combination means to match all values and all keys.
     */
    key?: string
    /**
     * Operator represents a key's relationship to the value. Valid operators are Exists and Equal. Defaults to Equal. Exists is equivalent to wildcard for value, so that a pod can tolerate all taints of a particular category.
     */
    operator?: string
    /**
     * TolerationSeconds represents the period of time the toleration (which must be of effect NoExecute, otherwise this field is ignored) tolerates the taint. By default, it is not set, which means tolerate the taint forever (do not evict). Zero and negative values will be treated as 0 (evict immediately) by the system.
     */
    tolerationSeconds?: number
    /**
     * Value is the taint value the toleration matches to. If the operator is Exists, the value should be empty, otherwise just a regular string.
     */
    value?: string
    [k: string]: unknown
}
/**
 * EventBusStatus holds the status of the eventbus resource
 */
export interface IoArgoprojEventbusV1Alpha1EventBusStatus {
    /**
     * Conditions are the latest available observations of a resource's current state.
     */
    conditions?: IoArgoprojCommonCondition[]
    /**
     * Config holds the fininalized configuration of EventBus
     */
    config?: {
        jetstream?: IoArgoprojEventbusV1Alpha1JetStreamConfig
        nats?: IoArgoprojEventbusV1Alpha1NATSConfig
        [k: string]: unknown
    }
    [k: string]: unknown
}
/**
 * Condition contains details about resource state
 */
export interface IoArgoprojCommonCondition {
    /**
     * Last time the condition transitioned from one status to another.
     */
    lastTransitionTime?: string
    /**
     * Human-readable message indicating details about last transition.
     */
    message?: string
    /**
     * Unique, this should be a short, machine understandable string that gives the reason for condition's last transition. For example, "ImageNotFound"
     */
    reason?: string
    /**
     * Condition status, True, False or Unknown.
     */
    status: string
    /**
     * Condition type.
     */
    type: string
    [k: string]: unknown
}
export interface IoArgoprojEventbusV1Alpha1JetStreamConfig {
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    accessSecret?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    streamConfig?: string
    /**
     * JetStream (Nats) URL
     */
    url?: string
    [k: string]: unknown
}
/**
 * NATSConfig holds the config of NATS
 */
export interface IoArgoprojEventbusV1Alpha1NATSConfig {
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    accessSecret?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * Auth strategy, default to AuthStrategyNone
     */
    auth?: string
    /**
     * Cluster ID for nats streaming
     */
    clusterID?: string
    /**
     * NATS streaming url
     */
    url?: string
    [k: string]: unknown
}
/**
 * EventSource is the definition of a eventsource resource
 */
export interface IoArgoprojEventsourceV1Alpha1EventSource {
    /**
     * APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
     */
    apiVersion?: "argoproj.io/v1alpha1"
    /**
     * Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
     */
    kind?: "EventSource"
    metadata: IoK8SApimachineryPkgApisMetaV1ObjectMeta
    spec: IoArgoprojEventsourceV1Alpha1EventSourceSpec
    status?: IoArgoprojEventsourceV1Alpha1EventSourceStatus
    [k: string]: unknown
}
/**
 * EventSourceSpec refers to specification of event-source resource
 */
export interface IoArgoprojEventsourceV1Alpha1EventSourceSpec {
    /**
     * AMQP event sources
     */
    amqp?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1AMQPEventSource
    }
    /**
     * AzureEventsHub event sources
     */
    azureEventsHub?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1AzureEventsHubEventSource
    }
    /**
     * Azure Service Bus event source
     */
    azureServiceBus?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1AzureServiceBusEventSource
    }
    /**
     * Bitbucket event sources
     */
    bitbucket?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1BitbucketEventSource
    }
    /**
     * Bitbucket Server event sources
     */
    bitbucketserver?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1BitbucketServerEventSource
    }
    /**
     * Calendar event sources
     */
    calendar?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1CalendarEventSource
    }
    /**
     * Emitter event source
     */
    emitter?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1EmitterEventSource
    }
    /**
     * EventBusName references to a EventBus name. By default the value is "default"
     */
    eventBusName?: string
    /**
     * File event sources
     */
    file?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1FileEventSource
    }
    /**
     * Generic event source
     */
    generic?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1GenericEventSource
    }
    /**
     * Github event sources
     */
    github?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1GithubEventSource
    }
    /**
     * Gitlab event sources
     */
    gitlab?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1GitlabEventSource
    }
    /**
     * HDFS event sources
     */
    hdfs?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1HDFSEventSource
    }
    /**
     * Kafka event sources
     */
    kafka?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1KafkaEventSource
    }
    /**
     * Minio event sources
     */
    minio?: {
        [k: string]: IoArgoprojCommonS3Artifact
    }
    /**
     * MQTT event sources
     */
    mqtt?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1MQTTEventSource
    }
    /**
     * NATS event sources
     */
    nats?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1NATSEventsSource
    }
    /**
     * NSQ event source
     */
    nsq?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1NSQEventSource
    }
    /**
     * PubSub event sources
     */
    pubSub?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1PubSubEventSource
    }
    /**
     * Pulsar event source
     */
    pulsar?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1PulsarEventSource
    }
    /**
     * Redis event source
     */
    redis?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1RedisEventSource
    }
    /**
     * Redis stream source
     */
    redisStream?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1RedisStreamEventSource
    }
    /**
     * Replicas is the event source deployment replicas
     */
    replicas?: number
    /**
     * Resource event sources
     */
    resource?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1ResourceEventSource
    }
    /**
     * Service is the specifications of the service to expose the event source
     */
    service?: {
        /**
         * clusterIP is the IP address of the service and is usually assigned randomly by the master. If an address is specified manually and is not in use by others, it will be allocated to the service; otherwise, creation of the service will fail. This field can not be changed through updates. Valid values are "None", empty string (""), or a valid IP address. "None" can be specified for headless services when proxying is not required. More info: https://kubernetes.io/docs/concepts/services-networking/service/#virtual-ips-and-service-proxies
         */
        clusterIP?: string
        /**
         * The list of ports that are exposed by this ClusterIP service.
         */
        ports?: IoK8SApiCoreV1ServicePort[]
        [k: string]: unknown
    }
    /**
     * Slack event sources
     */
    slack?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1SlackEventSource
    }
    /**
     * SNS event sources
     */
    sns?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1SNSEventSource
    }
    /**
     * SQS event sources
     */
    sqs?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1SQSEventSource
    }
    /**
     * StorageGrid event sources
     */
    storageGrid?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1StorageGridEventSource
    }
    /**
     * Stripe event sources
     */
    stripe?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1StripeEventSource
    }
    /**
     * Template is the pod specification for the event source
     */
    template?: {
        /**
         * If specified, the pod's scheduling constraints
         */
        affinity?: {
            /**
             * Describes node affinity scheduling rules for the pod.
             */
            nodeAffinity?: {
                /**
                 * The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node matches the corresponding matchExpressions; the node(s) with the highest sum are the most preferred.
                 */
                preferredDuringSchedulingIgnoredDuringExecution?: IoK8SApiCoreV1PreferredSchedulingTerm[]
                /**
                 * If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to an update), the system may or may not try to eventually evict the pod from its node.
                 */
                requiredDuringSchedulingIgnoredDuringExecution?: {
                    /**
                     * Required. A list of node selector terms. The terms are ORed.
                     */
                    nodeSelectorTerms: IoK8SApiCoreV1NodeSelectorTerm[]
                    [k: string]: unknown
                }
                [k: string]: unknown
            }
            /**
             * Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
             */
            podAffinity?: {
                /**
                 * The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
                 */
                preferredDuringSchedulingIgnoredDuringExecution?: IoK8SApiCoreV1WeightedPodAffinityTerm[]
                /**
                 * If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
                 */
                requiredDuringSchedulingIgnoredDuringExecution?: IoK8SApiCoreV1PodAffinityTerm[]
                [k: string]: unknown
            }
            /**
             * Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
             */
            podAntiAffinity?: {
                /**
                 * The scheduler will prefer to schedule pods to nodes that satisfy the anti-affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling anti-affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
                 */
                preferredDuringSchedulingIgnoredDuringExecution?: IoK8SApiCoreV1WeightedPodAffinityTerm[]
                /**
                 * If the anti-affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the anti-affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
                 */
                requiredDuringSchedulingIgnoredDuringExecution?: IoK8SApiCoreV1PodAffinityTerm[]
                [k: string]: unknown
            }
            [k: string]: unknown
        }
        /**
         * Container is the main container image to run in the event source pod
         */
        container?: {
            /**
             * Arguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
             */
            args?: string[]
            /**
             * Entrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
             */
            command?: string[]
            /**
             * List of environment variables to set in the container. Cannot be updated.
             */
            env?: IoK8SApiCoreV1EnvVar[]
            /**
             * List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
             */
            envFrom?: IoK8SApiCoreV1EnvFromSource[]
            /**
             * Docker image name. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets.
             */
            image?: string
            /**
             * Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
             */
            imagePullPolicy?: string
            /**
             * Actions that the management system should take in response to container lifecycle events. Cannot be updated.
             */
            lifecycle?: {
                /**
                 * PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
                 */
                postStart?: {
                    /**
                     * One and only one of the following should be specified. Exec specifies the action to take.
                     */
                    exec?: {
                        /**
                         * Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
                         */
                        command?: string[]
                        [k: string]: unknown
                    }
                    /**
                     * HTTPGet specifies the http request to perform.
                     */
                    httpGet?: {
                        /**
                         * Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
                         */
                        host?: string
                        /**
                         * Custom headers to set in the request. HTTP allows repeated headers.
                         */
                        httpHeaders?: IoK8SApiCoreV1HTTPHeader[]
                        /**
                         * Path to access on the HTTP server.
                         */
                        path?: string
                        /**
                         * Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
                         */
                        port: number | string
                        /**
                         * Scheme to use for connecting to the host. Defaults to HTTP.
                         */
                        scheme?: string
                        [k: string]: unknown
                    }
                    /**
                     * TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported
                     */
                    tcpSocket?: {
                        /**
                         * Optional: Host name to connect to, defaults to the pod IP.
                         */
                        host?: string
                        /**
                         * Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
                         */
                        port: number | string
                        [k: string]: unknown
                    }
                    [k: string]: unknown
                }
                /**
                 * PreStop is called immediately before a container is terminated due to an API request or management event such as liveness/startup probe failure, preemption, resource contention, etc. The handler is not called if the container crashes or exits. The reason for termination is passed to the handler. The Pod's termination grace period countdown begins before the PreStop hooked is executed. Regardless of the outcome of the handler, the container will eventually terminate within the Pod's termination grace period. Other management of the container blocks until the hook completes or until the termination grace period is reached. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
                 */
                preStop?: {
                    /**
                     * One and only one of the following should be specified. Exec specifies the action to take.
                     */
                    exec?: {
                        /**
                         * Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
                         */
                        command?: string[]
                        [k: string]: unknown
                    }
                    /**
                     * HTTPGet specifies the http request to perform.
                     */
                    httpGet?: {
                        /**
                         * Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
                         */
                        host?: string
                        /**
                         * Custom headers to set in the request. HTTP allows repeated headers.
                         */
                        httpHeaders?: IoK8SApiCoreV1HTTPHeader[]
                        /**
                         * Path to access on the HTTP server.
                         */
                        path?: string
                        /**
                         * Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
                         */
                        port: number | string
                        /**
                         * Scheme to use for connecting to the host. Defaults to HTTP.
                         */
                        scheme?: string
                        [k: string]: unknown
                    }
                    /**
                     * TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported
                     */
                    tcpSocket?: {
                        /**
                         * Optional: Host name to connect to, defaults to the pod IP.
                         */
                        host?: string
                        /**
                         * Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
                         */
                        port: number | string
                        [k: string]: unknown
                    }
                    [k: string]: unknown
                }
                [k: string]: unknown
            }
            /**
             * Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
             */
            livenessProbe?: {
                /**
                 * One and only one of the following should be specified. Exec specifies the action to take.
                 */
                exec?: {
                    /**
                     * Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
                     */
                    command?: string[]
                    [k: string]: unknown
                }
                /**
                 * Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
                 */
                failureThreshold?: number
                /**
                 * HTTPGet specifies the http request to perform.
                 */
                httpGet?: {
                    /**
                     * Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
                     */
                    host?: string
                    /**
                     * Custom headers to set in the request. HTTP allows repeated headers.
                     */
                    httpHeaders?: IoK8SApiCoreV1HTTPHeader[]
                    /**
                     * Path to access on the HTTP server.
                     */
                    path?: string
                    /**
                     * Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
                     */
                    port: number | string
                    /**
                     * Scheme to use for connecting to the host. Defaults to HTTP.
                     */
                    scheme?: string
                    [k: string]: unknown
                }
                /**
                 * Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
                 */
                initialDelaySeconds?: number
                /**
                 * How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
                 */
                periodSeconds?: number
                /**
                 * Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
                 */
                successThreshold?: number
                /**
                 * TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported
                 */
                tcpSocket?: {
                    /**
                     * Optional: Host name to connect to, defaults to the pod IP.
                     */
                    host?: string
                    /**
                     * Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
                     */
                    port: number | string
                    [k: string]: unknown
                }
                /**
                 * Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
                 */
                timeoutSeconds?: number
                [k: string]: unknown
            }
            /**
             * Name of the container specified as a DNS_LABEL. Each container in a pod must have a unique name (DNS_LABEL). Cannot be updated.
             */
            name: string
            /**
             * List of ports to expose from the container. Exposing a port here gives the system additional information about the network connections a container uses, but is primarily informational. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default "0.0.0.0" address inside a container will be accessible from the network. Cannot be updated.
             */
            ports?: IoK8SApiCoreV1ContainerPort[]
            /**
             * Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
             */
            readinessProbe?: {
                /**
                 * One and only one of the following should be specified. Exec specifies the action to take.
                 */
                exec?: {
                    /**
                     * Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
                     */
                    command?: string[]
                    [k: string]: unknown
                }
                /**
                 * Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
                 */
                failureThreshold?: number
                /**
                 * HTTPGet specifies the http request to perform.
                 */
                httpGet?: {
                    /**
                     * Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
                     */
                    host?: string
                    /**
                     * Custom headers to set in the request. HTTP allows repeated headers.
                     */
                    httpHeaders?: IoK8SApiCoreV1HTTPHeader[]
                    /**
                     * Path to access on the HTTP server.
                     */
                    path?: string
                    /**
                     * Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
                     */
                    port: number | string
                    /**
                     * Scheme to use for connecting to the host. Defaults to HTTP.
                     */
                    scheme?: string
                    [k: string]: unknown
                }
                /**
                 * Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
                 */
                initialDelaySeconds?: number
                /**
                 * How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
                 */
                periodSeconds?: number
                /**
                 * Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
                 */
                successThreshold?: number
                /**
                 * TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported
                 */
                tcpSocket?: {
                    /**
                     * Optional: Host name to connect to, defaults to the pod IP.
                     */
                    host?: string
                    /**
                     * Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
                     */
                    port: number | string
                    [k: string]: unknown
                }
                /**
                 * Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
                 */
                timeoutSeconds?: number
                [k: string]: unknown
            }
            /**
             * ResourceRequirements describes the compute resource requirements.
             */
            resources?: {
                /**
                 * Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/
                 */
                limits?: {
                    [k: string]: IoK8SApimachineryPkgApiResourceQuantity
                }
                /**
                 * Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/
                 */
                requests?: {
                    [k: string]: IoK8SApimachineryPkgApiResourceQuantity
                }
                [k: string]: unknown
            }
            /**
             * SecurityContext holds security configuration that will be applied to a container. Some fields are present in both SecurityContext and PodSecurityContext.  When both are set, the values in SecurityContext take precedence.
             */
            securityContext?: {
                /**
                 * AllowPrivilegeEscalation controls whether a process can gain more privileges than its parent process. This bool directly controls if the no_new_privs flag will be set on the container process. AllowPrivilegeEscalation is true always when the container is: 1) run as Privileged 2) has CAP_SYS_ADMIN
                 */
                allowPrivilegeEscalation?: boolean
                /**
                 * The capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime.
                 */
                capabilities?: {
                    /**
                     * Added capabilities
                     */
                    add?: string[]
                    /**
                     * Removed capabilities
                     */
                    drop?: string[]
                    [k: string]: unknown
                }
                /**
                 * Run container in privileged mode. Processes in privileged containers are essentially equivalent to root on the host. Defaults to false.
                 */
                privileged?: boolean
                /**
                 * procMount denotes the type of proc mount to use for the containers. The default is DefaultProcMount which uses the container runtime defaults for readonly paths and masked paths. This requires the ProcMountType feature flag to be enabled.
                 */
                procMount?: string
                /**
                 * Whether this container has a read-only root filesystem. Default is false.
                 */
                readOnlyRootFilesystem?: boolean
                /**
                 * The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
                 */
                runAsGroup?: number
                /**
                 * Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
                 */
                runAsNonRoot?: boolean
                /**
                 * The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
                 */
                runAsUser?: number
                /**
                 * The SELinux context to be applied to the container. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
                 */
                seLinuxOptions?: {
                    /**
                     * Level is SELinux level label that applies to the container.
                     */
                    level?: string
                    /**
                     * Role is a SELinux role label that applies to the container.
                     */
                    role?: string
                    /**
                     * Type is a SELinux type label that applies to the container.
                     */
                    type?: string
                    /**
                     * User is a SELinux user label that applies to the container.
                     */
                    user?: string
                    [k: string]: unknown
                }
                /**
                 * The seccomp options to use by this container. If seccomp options are provided at both the pod & container level, the container options override the pod options.
                 */
                seccompProfile?: {
                    /**
                     * localhostProfile indicates a profile defined in a file on the node should be used. The profile must be preconfigured on the node to work. Must be a descending path, relative to the kubelet's configured seccomp profile location. Must only be set if type is "Localhost".
                     */
                    localhostProfile?: string
                    /**
                     * type indicates which kind of seccomp profile will be applied. Valid options are:
                     *
                     * Localhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.
                     */
                    type: string
                    [k: string]: unknown
                }
                /**
                 * The Windows specific settings applied to all containers. If unspecified, the options from the PodSecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
                 */
                windowsOptions?: {
                    /**
                     * GMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field.
                     */
                    gmsaCredentialSpec?: string
                    /**
                     * GMSACredentialSpecName is the name of the GMSA credential spec to use.
                     */
                    gmsaCredentialSpecName?: string
                    /**
                     * The UserName in Windows to run the entrypoint of the container process. Defaults to the user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
                     */
                    runAsUserName?: string
                    [k: string]: unknown
                }
                [k: string]: unknown
            }
            /**
             * StartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
             */
            startupProbe?: {
                /**
                 * One and only one of the following should be specified. Exec specifies the action to take.
                 */
                exec?: {
                    /**
                     * Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
                     */
                    command?: string[]
                    [k: string]: unknown
                }
                /**
                 * Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
                 */
                failureThreshold?: number
                /**
                 * HTTPGet specifies the http request to perform.
                 */
                httpGet?: {
                    /**
                     * Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
                     */
                    host?: string
                    /**
                     * Custom headers to set in the request. HTTP allows repeated headers.
                     */
                    httpHeaders?: IoK8SApiCoreV1HTTPHeader[]
                    /**
                     * Path to access on the HTTP server.
                     */
                    path?: string
                    /**
                     * Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
                     */
                    port: number | string
                    /**
                     * Scheme to use for connecting to the host. Defaults to HTTP.
                     */
                    scheme?: string
                    [k: string]: unknown
                }
                /**
                 * Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
                 */
                initialDelaySeconds?: number
                /**
                 * How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
                 */
                periodSeconds?: number
                /**
                 * Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
                 */
                successThreshold?: number
                /**
                 * TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported
                 */
                tcpSocket?: {
                    /**
                     * Optional: Host name to connect to, defaults to the pod IP.
                     */
                    host?: string
                    /**
                     * Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
                     */
                    port: number | string
                    [k: string]: unknown
                }
                /**
                 * Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
                 */
                timeoutSeconds?: number
                [k: string]: unknown
            }
            /**
             * Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false.
             */
            stdin?: boolean
            /**
             * Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false
             */
            stdinOnce?: boolean
            /**
             * Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.
             */
            terminationMessagePath?: string
            /**
             * Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.
             */
            terminationMessagePolicy?: string
            /**
             * Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.
             */
            tty?: boolean
            /**
             * volumeDevices is the list of block devices to be used by the container.
             */
            volumeDevices?: IoK8SApiCoreV1VolumeDevice[]
            /**
             * Pod volumes to mount into the container's filesystem. Cannot be updated.
             */
            volumeMounts?: IoK8SApiCoreV1VolumeMount[]
            /**
             * Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.
             */
            workingDir?: string
            [k: string]: unknown
        }
        /**
         * ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec. If specified, these secrets will be passed to individual puller implementations for them to use. For example, in the case of docker, only DockerConfig type secrets are honored. More info: https://kubernetes.io/docs/concepts/containers/images#specifying-imagepullsecrets-on-a-pod
         */
        imagePullSecrets?: IoK8SApiCoreV1LocalObjectReference[]
        /**
         * Metadata sets the pods's metadata, i.e. annotations and labels
         */
        metadata?: {
            annotations?: {
                [k: string]: string
            }
            labels?: {
                [k: string]: string
            }
            [k: string]: unknown
        }
        /**
         * NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
         */
        nodeSelector?: {
            [k: string]: string
        }
        /**
         * The priority value. Various system components use this field to find the priority of the EventSource pod. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority. More info: https://kubernetes.io/docs/concepts/configuration/pod-priority-preemption/
         */
        priority?: number
        /**
         * If specified, indicates the EventSource pod's priority. "system-node-critical" and "system-cluster-critical" are two special keywords which indicate the highest priorities with the former being the highest priority. Any other name must be defined by creating a PriorityClass object with that name. If not specified, the pod priority will be default or zero if there is no default. More info: https://kubernetes.io/docs/concepts/configuration/pod-priority-preemption/
         */
        priorityClassName?: string
        /**
         * SecurityContext holds pod-level security attributes and common container settings. Optional: Defaults to empty.  See type description for default values of each field.
         */
        securityContext?: {
            /**
             * A special supplemental group that applies to all containers in a pod. Some volume types allow the Kubelet to change the ownership of that volume to be owned by the pod:
             *
             * 1. The owning GID will be the FSGroup 2. The setgid bit is set (new files created in the volume will be owned by FSGroup) 3. The permission bits are OR'd with rw-rw----
             *
             * If unset, the Kubelet will not modify the ownership and permissions of any volume.
             */
            fsGroup?: number
            /**
             * fsGroupChangePolicy defines behavior of changing ownership and permission of the volume before being exposed inside Pod. This field will only apply to volume types which support fsGroup based ownership(and permissions). It will have no effect on ephemeral volume types such as: secret, configmaps and emptydir. Valid values are "OnRootMismatch" and "Always". If not specified, "Always" is used.
             */
            fsGroupChangePolicy?: string
            /**
             * The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container.
             */
            runAsGroup?: number
            /**
             * Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
             */
            runAsNonRoot?: boolean
            /**
             * The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container.
             */
            runAsUser?: number
            /**
             * The SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container.
             */
            seLinuxOptions?: {
                /**
                 * Level is SELinux level label that applies to the container.
                 */
                level?: string
                /**
                 * Role is a SELinux role label that applies to the container.
                 */
                role?: string
                /**
                 * Type is a SELinux type label that applies to the container.
                 */
                type?: string
                /**
                 * User is a SELinux user label that applies to the container.
                 */
                user?: string
                [k: string]: unknown
            }
            /**
             * The seccomp options to use by the containers in this pod.
             */
            seccompProfile?: {
                /**
                 * localhostProfile indicates a profile defined in a file on the node should be used. The profile must be preconfigured on the node to work. Must be a descending path, relative to the kubelet's configured seccomp profile location. Must only be set if type is "Localhost".
                 */
                localhostProfile?: string
                /**
                 * type indicates which kind of seccomp profile will be applied. Valid options are:
                 *
                 * Localhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.
                 */
                type: string
                [k: string]: unknown
            }
            /**
             * A list of groups applied to the first process run in each container, in addition to the container's primary GID.  If unspecified, no groups will be added to any container.
             */
            supplementalGroups?: number[]
            /**
             * Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported sysctls (by the container runtime) might fail to launch.
             */
            sysctls?: IoK8SApiCoreV1Sysctl[]
            /**
             * The Windows specific settings applied to all containers. If unspecified, the options within a container's SecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
             */
            windowsOptions?: {
                /**
                 * GMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field.
                 */
                gmsaCredentialSpec?: string
                /**
                 * GMSACredentialSpecName is the name of the GMSA credential spec to use.
                 */
                gmsaCredentialSpecName?: string
                /**
                 * The UserName in Windows to run the entrypoint of the container process. Defaults to the user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
                 */
                runAsUserName?: string
                [k: string]: unknown
            }
            [k: string]: unknown
        }
        /**
         * ServiceAccountName is the name of the ServiceAccount to use to run event source pod. More info: https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/
         */
        serviceAccountName?: string
        /**
         * If specified, the pod's tolerations.
         */
        tolerations?: IoK8SApiCoreV1Toleration[]
        /**
         * Volumes is a list of volumes that can be mounted by containers in an eventsource.
         */
        volumes?: IoK8SApiCoreV1Volume[]
        [k: string]: unknown
    }
    /**
     * Webhook event sources
     */
    webhook?: {
        [k: string]: IoArgoprojEventsourceV1Alpha1WebhookEventSource
    }
    [k: string]: unknown
}
/**
 * AMQPEventSource refers to an event-source for AMQP stream events
 */
export interface IoArgoprojEventsourceV1Alpha1AMQPEventSource {
    /**
     * Auth hosts secret selectors for username and password
     */
    auth?: {
        /**
         * Password refers to the Kubernetes secret that holds the password required for basic auth.
         */
        password?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * Username refers to the Kubernetes secret that holds the username required for basic auth.
         */
        username?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        [k: string]: unknown
    }
    /**
     * Backoff holds parameters applied to connection.
     */
    connectionBackoff?: {
        /**
         * The initial duration in nanoseconds or strings like "1s", "3m"
         */
        duration?: number | string
        /**
         * Duration is multiplied by factor each iteration
         */
        factor?: number
        /**
         * The amount of jitter applied each iteration
         */
        jitter?: number
        /**
         * Exit with error after this many steps
         */
        steps?: number
        [k: string]: unknown
    }
    /**
     * Consume holds the configuration to immediately starts delivering queued messages For more information, visit https://pkg.go.dev/github.com/rabbitmq/amqp091-go#Channel.Consume
     */
    consume?: {
        /**
         * AutoAck when true, the server will acknowledge deliveries to this consumer prior to writing the delivery to the network
         */
        autoAck?: boolean
        /**
         * ConsumerTag is the identity of the consumer included in every delivery
         */
        consumerTag?: string
        /**
         * Exclusive when true, the server will ensure that this is the sole consumer from this queue
         */
        exclusive?: boolean
        /**
         * NoLocal flag is not supported by RabbitMQ
         */
        noLocal?: boolean
        /**
         * NowWait when true, do not wait for the server to confirm the request and immediately begin deliveries
         */
        noWait?: boolean
        [k: string]: unknown
    }
    /**
     * ExchangeDeclare holds the configuration for the exchange on the server For more information, visit https://pkg.go.dev/github.com/rabbitmq/amqp091-go#Channel.ExchangeDeclare
     */
    exchangeDeclare?: {
        /**
         * AutoDelete removes the exchange when no bindings are active
         */
        autoDelete?: boolean
        /**
         * Durable keeps the exchange also after the server restarts
         */
        durable?: boolean
        /**
         * Internal when true does not accept publishings
         */
        internal?: boolean
        /**
         * NowWait when true does not wait for a confirmation from the server
         */
        noWait?: boolean
        [k: string]: unknown
    }
    /**
     * ExchangeName is the exchange name For more information, visit https://www.rabbitmq.com/tutorials/amqp-concepts.html
     */
    exchangeName: string
    /**
     * ExchangeType is rabbitmq exchange type
     */
    exchangeType: string
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * JSONBody specifies that all event body payload coming from this source will be JSON
     */
    jsonBody?: boolean
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * QueueBind holds the configuration that binds an exchange to a queue so that publishings to the exchange will be routed to the queue when the publishing routing key matches the binding routing key For more information, visit https://pkg.go.dev/github.com/rabbitmq/amqp091-go#Channel.QueueBind
     */
    queueBind?: {
        /**
         * NowWait false and the queue could not be bound, the channel will be closed with an error
         */
        noWait?: boolean
        [k: string]: unknown
    }
    /**
     * QueueDeclare holds the configuration of a queue to hold messages and deliver to consumers. Declaring creates a queue if it doesn't already exist, or ensures that an existing queue matches the same parameters For more information, visit https://pkg.go.dev/github.com/rabbitmq/amqp091-go#Channel.QueueDeclare
     */
    queueDeclare?: {
        /**
         * Arguments of a queue (also known as "x-arguments") used for optional features and plugins
         */
        arguments?: string
        /**
         * AutoDelete removes the queue when no consumers are active
         */
        autoDelete?: boolean
        /**
         * Durable keeps the queue also after the server restarts
         */
        durable?: boolean
        /**
         * Exclusive sets the queues to be accessible only by the connection that declares them and will be deleted wgen the connection closes
         */
        exclusive?: boolean
        /**
         * Name of the queue. If empty the server auto-generates a unique name for this queue
         */
        name?: string
        /**
         * NowWait when true, the queue assumes to be declared on the server
         */
        noWait?: boolean
        [k: string]: unknown
    }
    /**
     * Routing key for bindings
     */
    routingKey: string
    /**
     * TLS configuration for the amqp client.
     */
    tls?: {
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        caCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        clientCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        clientKeySecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * If true, skips creation of TLSConfig with certs and creates an empty TLSConfig. (Defaults to false)
         */
        insecureSkipVerify?: boolean
        [k: string]: unknown
    }
    /**
     * URL for rabbitmq service
     */
    url?: string
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    urlSecret?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    [k: string]: unknown
}
/**
 * AzureEventsHubEventSource describes the event source for azure events hub More info at https://docs.microsoft.com/en-us/azure/event-hubs/
 */
export interface IoArgoprojEventsourceV1Alpha1AzureEventsHubEventSource {
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * FQDN of the EventHubs namespace you created More info at https://docs.microsoft.com/en-us/azure/event-hubs/event-hubs-get-connection-string
     */
    fqdn: string
    /**
     * Event Hub path/name
     */
    hubName: string
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    sharedAccessKey?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    sharedAccessKeyName?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    [k: string]: unknown
}
/**
 * AzureServiceBusEventSource describes the event source for azure service bus More info at https://docs.microsoft.com/en-us/azure/service-bus-messaging/
 */
export interface IoArgoprojEventsourceV1Alpha1AzureServiceBusEventSource {
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    connectionString?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * JSONBody specifies that all event body payload coming from this source will be JSON
     */
    jsonBody?: boolean
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * QueueName is the name of the Azure Service Bus Queue
     */
    queueName: string
    /**
     * SubscriptionName is the name of the Azure Service Bus Topic Subscription
     */
    subscriptionName: string
    /**
     * TLS configuration for the service bus client
     */
    tls?: {
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        caCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        clientCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        clientKeySecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * If true, skips creation of TLSConfig with certs and creates an empty TLSConfig. (Defaults to false)
         */
        insecureSkipVerify?: boolean
        [k: string]: unknown
    }
    /**
     * TopicName is the name of the Azure Service Bus Topic
     */
    topicName: string
    [k: string]: unknown
}
/**
 * BitbucketEventSource describes the event source for Bitbucket
 */
export interface IoArgoprojEventsourceV1Alpha1BitbucketEventSource {
    /**
     * Auth information required to connect to Bitbucket.
     */
    auth: {
        /**
         * Basic is BasicAuth auth strategy.
         */
        basic?: {
            /**
             * SecretKeySelector selects a key of a Secret.
             */
            password: {
                /**
                 * The key of the secret to select from.  Must be a valid secret key.
                 */
                key: string
                /**
                 * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                 */
                name?: string
                /**
                 * Specify whether the Secret or its key must be defined
                 */
                optional?: boolean
                [k: string]: unknown
            }
            /**
             * SecretKeySelector selects a key of a Secret.
             */
            username: {
                /**
                 * The key of the secret to select from.  Must be a valid secret key.
                 */
                key: string
                /**
                 * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                 */
                name?: string
                /**
                 * Specify whether the Secret or its key must be defined
                 */
                optional?: boolean
                [k: string]: unknown
            }
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        oauthToken?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        [k: string]: unknown
    }
    /**
     * DeleteHookOnFinish determines whether to delete the defined Bitbucket hook once the event source is stopped.
     */
    deleteHookOnFinish?: boolean
    /**
     * Events this webhook is subscribed to.
     */
    events: string[]
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * Metadata holds the user defined metadata which will be passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * DeprecatedOwner is the owner of the repository. Deprecated: use Repositories instead. Will be unsupported in v1.9
     */
    owner?: string
    /**
     * DeprecatedProjectKey is the key of the project to which the repository relates Deprecated: use Repositories instead. Will be unsupported in v1.9
     */
    projectKey?: string
    /**
     * Repositories holds a list of repositories for which integration needs to set up
     */
    repositories?: IoArgoprojEventsourceV1Alpha1BitbucketRepository[]
    /**
     * DeprecatedRepositorySlug is a URL-friendly version of a repository name, automatically generated by Bitbucket for use in the URL Deprecated: use Repositories instead. Will be unsupported in v1.9
     */
    repositorySlug?: string
    /**
     * Webhook refers to the configuration required to run an http server
     */
    webhook: {
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        authSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * REST API endpoint
         */
        endpoint: string
        /**
         * MaxPayloadSize is the maximum webhook payload size that the server will accept. Requests exceeding that limit will be rejected with "request too large" response. Default value: 1048576 (1MB).
         */
        maxPayloadSize?: number
        /**
         * Metadata holds the user defined metadata which will passed along the event payload.
         */
        metadata?: {
            [k: string]: string
        }
        /**
         * Method is HTTP request method that indicates the desired action to be performed for a given resource. See RFC7231 Hypertext Transfer Protocol (HTTP/1.1): Semantics and Content
         */
        method: string
        /**
         * Port on which HTTP server is listening for incoming events.
         */
        port: string
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        serverCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        serverKeySecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * URL is the url of the server.
         */
        url: string
        [k: string]: unknown
    }
    [k: string]: unknown
}
export interface IoArgoprojEventsourceV1Alpha1BitbucketRepository {
    /**
     * Owner is the owner of the repository
     */
    owner: string
    /**
     * RepositorySlug is a URL-friendly version of a repository name, automatically generated by Bitbucket for use in the URL
     */
    repositorySlug: string
    [k: string]: unknown
}
/**
 * BitbucketServerEventSource refers to event-source related to Bitbucket Server events
 */
export interface IoArgoprojEventsourceV1Alpha1BitbucketServerEventSource {
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    accessToken?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * BitbucketServerBaseURL is the base URL for API requests to a custom endpoint
     */
    bitbucketserverBaseURL: string
    /**
     * DeleteHookOnFinish determines whether to delete the Bitbucket Server hook for the project once the event source is stopped.
     */
    deleteHookOnFinish?: boolean
    /**
     * Events are bitbucket event to listen to. Refer https://confluence.atlassian.com/bitbucketserver/event-payload-938025882.html
     */
    events: string[]
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * DeprecatedProjectKey is the key of project for which integration needs to set up Deprecated: use Repositories instead. Will be unsupported in v1.8
     */
    projectKey?: string
    /**
     * Repositories holds a list of repositories for which integration needs to set up
     */
    repositories?: IoArgoprojEventsourceV1Alpha1BitbucketServerRepository[]
    /**
     * DeprecatedRepositorySlug is the slug of the repository for which integration needs to set up Deprecated: use Repositories instead. Will be unsupported in v1.8
     */
    repositorySlug?: string
    /**
     * Webhook holds configuration to run a http server
     */
    webhook?: {
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        authSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * REST API endpoint
         */
        endpoint: string
        /**
         * MaxPayloadSize is the maximum webhook payload size that the server will accept. Requests exceeding that limit will be rejected with "request too large" response. Default value: 1048576 (1MB).
         */
        maxPayloadSize?: number
        /**
         * Metadata holds the user defined metadata which will passed along the event payload.
         */
        metadata?: {
            [k: string]: string
        }
        /**
         * Method is HTTP request method that indicates the desired action to be performed for a given resource. See RFC7231 Hypertext Transfer Protocol (HTTP/1.1): Semantics and Content
         */
        method: string
        /**
         * Port on which HTTP server is listening for incoming events.
         */
        port: string
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        serverCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        serverKeySecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * URL is the url of the server.
         */
        url: string
        [k: string]: unknown
    }
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    webhookSecret?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    [k: string]: unknown
}
export interface IoArgoprojEventsourceV1Alpha1BitbucketServerRepository {
    /**
     * ProjectKey is the key of project for which integration needs to set up
     */
    projectKey: string
    /**
     * RepositorySlug is the slug of the repository for which integration needs to set up
     */
    repositorySlug: string
    [k: string]: unknown
}
/**
 * CalendarEventSource describes a time based dependency. One of the fields (schedule, interval, or recurrence) must be passed. Schedule takes precedence over interval; interval takes precedence over recurrence
 */
export interface IoArgoprojEventsourceV1Alpha1CalendarEventSource {
    /**
     * ExclusionDates defines the list of DATE-TIME exceptions for recurring events.
     */
    exclusionDates?: string[]
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * Interval is a string that describes an interval duration, e.g. 1s, 30m, 2h...
     */
    interval?: string
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * Persistence hold the configuration for event persistence
     */
    persistence?: {
        /**
         * Catchup enables to triggered the missed schedule when eventsource restarts
         */
        catchup?: {
            /**
             * Enabled enables to triggered the missed schedule when eventsource restarts
             */
            enabled?: boolean
            /**
             * MaxDuration holds max catchup duration
             */
            maxDuration?: string
            [k: string]: unknown
        }
        /**
         * ConfigMap holds configmap details for persistence
         */
        configMap?: {
            /**
             * CreateIfNotExist will create configmap if it doesn't exists
             */
            createIfNotExist?: boolean
            /**
             * Name of the configmap
             */
            name?: string
            [k: string]: unknown
        }
        [k: string]: unknown
    }
    /**
     * Schedule is a cron-like expression. For reference, see: https://en.wikipedia.org/wiki/Cron
     */
    schedule?: string
    /**
     * Timezone in which to run the schedule
     */
    timezone?: string
    [k: string]: unknown
}
/**
 * EmitterEventSource describes the event source for emitter More info at https://emitter.io/develop/getting-started/
 */
export interface IoArgoprojEventsourceV1Alpha1EmitterEventSource {
    /**
     * Broker URI to connect to.
     */
    broker: string
    /**
     * ChannelKey refers to the channel key
     */
    channelKey: string
    /**
     * ChannelName refers to the channel name
     */
    channelName: string
    /**
     * Backoff holds parameters applied to connection.
     */
    connectionBackoff?: {
        /**
         * The initial duration in nanoseconds or strings like "1s", "3m"
         */
        duration?: number | string
        /**
         * Duration is multiplied by factor each iteration
         */
        factor?: number
        /**
         * The amount of jitter applied each iteration
         */
        jitter?: number
        /**
         * Exit with error after this many steps
         */
        steps?: number
        [k: string]: unknown
    }
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * JSONBody specifies that all event body payload coming from this source will be JSON
     */
    jsonBody?: boolean
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    password?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * TLS configuration for the emitter client.
     */
    tls?: {
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        caCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        clientCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        clientKeySecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * If true, skips creation of TLSConfig with certs and creates an empty TLSConfig. (Defaults to false)
         */
        insecureSkipVerify?: boolean
        [k: string]: unknown
    }
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    username?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    [k: string]: unknown
}
/**
 * FileEventSource describes an event-source for file related events.
 */
export interface IoArgoprojEventsourceV1Alpha1FileEventSource {
    /**
     * Type of file operations to watch Refer https://github.com/fsnotify/fsnotify/blob/master/fsnotify.go for more information
     */
    eventType: string
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * Use polling instead of inotify
     */
    polling?: boolean
    /**
     * WatchPathConfig contains configuration about the file path to watch
     */
    watchPathConfig: {
        /**
         * Directory to watch for events
         */
        directory: string
        /**
         * Path is relative path of object to watch with respect to the directory
         */
        path?: string
        /**
         * PathRegexp is regexp of relative path of object to watch with respect to the directory
         */
        pathRegexp?: string
        [k: string]: unknown
    }
    [k: string]: unknown
}
/**
 * GenericEventSource refers to a generic event source. It can be used to implement a custom event source.
 */
export interface IoArgoprojEventsourceV1Alpha1GenericEventSource {
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    authSecret?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * Config is the event source configuration
     */
    config: string
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * Insecure determines the type of connection.
     */
    insecure?: boolean
    /**
     * JSONBody specifies that all event body payload coming from this source will be JSON
     */
    jsonBody?: boolean
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * URL of the gRPC server that implements the event source.
     */
    url: string
    [k: string]: unknown
}
/**
 * GithubEventSource refers to event-source for github related events
 */
export interface IoArgoprojEventsourceV1Alpha1GithubEventSource {
    /**
     * Active refers to status of the webhook for event deliveries. https://developer.github.com/webhooks/creating/#active
     */
    active?: boolean
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    apiToken?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * ContentType of the event delivery
     */
    contentType?: string
    /**
     * DeleteHookOnFinish determines whether to delete the GitHub hook for the repository once the event source is stopped.
     */
    deleteHookOnFinish?: boolean
    /**
     * Events refer to Github events to which the event source will subscribe
     */
    events: string[]
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * GitHubApp holds the GitHub app credentials
     */
    githubApp?: {
        /**
         * AppID refers to the GitHub App ID for the application you created
         */
        appID: number
        /**
         * InstallationID refers to the Installation ID of the GitHub app you created and installed
         */
        installationID: number
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        privateKey: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        [k: string]: unknown
    }
    /**
     * GitHub base URL (for GitHub Enterprise)
     */
    githubBaseURL?: string
    /**
     * GitHub upload URL (for GitHub Enterprise)
     */
    githubUploadURL?: string
    /**
     * Id is the webhook's id Deprecated: This is not used at all, will be removed in v1.6
     */
    id?: number
    /**
     * Insecure tls verification
     */
    insecure?: boolean
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * Organizations holds the names of organizations (used for organization level webhooks). Not required if Repositories is set.
     */
    organizations?: string[]
    /**
     * DeprecatedOwner refers to GitHub owner name i.e. argoproj Deprecated: use Repositories instead. Will be unsupported in v 1.6
     */
    owner?: string
    /**
     * Repositories holds the information of repositories, which uses repo owner as the key, and list of repo names as the value. Not required if Organizations is set.
     */
    repositories?: IoArgoprojEventsourceV1Alpha1OwnedRepositories[]
    /**
     * DeprecatedRepository refers to GitHub repo name i.e. argo-events Deprecated: use Repositories instead. Will be unsupported in v 1.6
     */
    repository?: string
    /**
     * Webhook refers to the configuration required to run a http server
     */
    webhook?: {
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        authSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * REST API endpoint
         */
        endpoint: string
        /**
         * MaxPayloadSize is the maximum webhook payload size that the server will accept. Requests exceeding that limit will be rejected with "request too large" response. Default value: 1048576 (1MB).
         */
        maxPayloadSize?: number
        /**
         * Metadata holds the user defined metadata which will passed along the event payload.
         */
        metadata?: {
            [k: string]: string
        }
        /**
         * Method is HTTP request method that indicates the desired action to be performed for a given resource. See RFC7231 Hypertext Transfer Protocol (HTTP/1.1): Semantics and Content
         */
        method: string
        /**
         * Port on which HTTP server is listening for incoming events.
         */
        port: string
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        serverCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        serverKeySecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * URL is the url of the server.
         */
        url: string
        [k: string]: unknown
    }
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    webhookSecret?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    [k: string]: unknown
}
export interface IoArgoprojEventsourceV1Alpha1OwnedRepositories {
    /**
     * Repository names
     */
    names?: string[]
    /**
     * Organization or user name
     */
    owner?: string
    [k: string]: unknown
}
/**
 * GitlabEventSource refers to event-source related to Gitlab events
 */
export interface IoArgoprojEventsourceV1Alpha1GitlabEventSource {
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    accessToken?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * DeleteHookOnFinish determines whether to delete the GitLab hook for the project once the event source is stopped.
     */
    deleteHookOnFinish?: boolean
    /**
     * EnableSSLVerification to enable ssl verification
     */
    enableSSLVerification?: boolean
    /**
     * Events are gitlab event to listen to. Refer https://github.com/xanzy/go-gitlab/blob/bf34eca5d13a9f4c3f501d8a97b8ac226d55e4d9/projects.go#L794.
     */
    events: string[]
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * GitlabBaseURL is the base URL for API requests to a custom endpoint
     */
    gitlabBaseURL: string
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * DeprecatedProjectID is the id of project for which integration needs to setup Deprecated: use Projects instead. Will be unsupported in v 1.7
     */
    projectID?: string
    /**
     * List of project IDs or project namespace paths like "whynowy/test"
     */
    projects?: string[]
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    secretToken?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * Webhook holds configuration to run a http server
     */
    webhook?: {
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        authSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * REST API endpoint
         */
        endpoint: string
        /**
         * MaxPayloadSize is the maximum webhook payload size that the server will accept. Requests exceeding that limit will be rejected with "request too large" response. Default value: 1048576 (1MB).
         */
        maxPayloadSize?: number
        /**
         * Metadata holds the user defined metadata which will passed along the event payload.
         */
        metadata?: {
            [k: string]: string
        }
        /**
         * Method is HTTP request method that indicates the desired action to be performed for a given resource. See RFC7231 Hypertext Transfer Protocol (HTTP/1.1): Semantics and Content
         */
        method: string
        /**
         * Port on which HTTP server is listening for incoming events.
         */
        port: string
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        serverCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        serverKeySecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * URL is the url of the server.
         */
        url: string
        [k: string]: unknown
    }
    [k: string]: unknown
}
/**
 * HDFSEventSource refers to event-source for HDFS related events
 */
export interface IoArgoprojEventsourceV1Alpha1HDFSEventSource {
    addresses: string[]
    /**
     * CheckInterval is a string that describes an interval duration to check the directory state, e.g. 1s, 30m, 2h... (defaults to 1m)
     */
    checkInterval?: string
    /**
     * Directory to watch for events
     */
    directory: string
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * HDFSUser is the user to access HDFS file system. It is ignored if either ccache or keytab is used.
     */
    hdfsUser?: string
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    krbCCacheSecret?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * Selects a key from a ConfigMap.
     */
    krbConfigConfigMap?: {
        /**
         * The key to select.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the ConfigMap or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    krbKeytabSecret?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * KrbRealm is the Kerberos realm used with Kerberos keytab It must be set if keytab is used.
     */
    krbRealm?: string
    /**
     * KrbServicePrincipalName is the principal name of Kerberos service It must be set if either ccache or keytab is used.
     */
    krbServicePrincipalName?: string
    /**
     * KrbUsername is the Kerberos username used with Kerberos keytab It must be set if keytab is used.
     */
    krbUsername?: string
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * Path is relative path of object to watch with respect to the directory
     */
    path?: string
    /**
     * PathRegexp is regexp of relative path of object to watch with respect to the directory
     */
    pathRegexp?: string
    /**
     * Type of file operations to watch
     */
    type: string
    [k: string]: unknown
}
/**
 * KafkaEventSource refers to event-source for Kafka related events
 */
export interface IoArgoprojEventsourceV1Alpha1KafkaEventSource {
    /**
     * Yaml format Sarama config for Kafka connection. It follows the struct of sarama.Config. See https://github.com/Shopify/sarama/blob/main/config.go e.g.
     *
     * consumer:
     *   fetch:
     *     min: 1
     * net:
     *   MaxOpenRequests: 5
     */
    config?: string
    /**
     * Backoff holds parameters applied to connection.
     */
    connectionBackoff?: {
        /**
         * The initial duration in nanoseconds or strings like "1s", "3m"
         */
        duration?: number | string
        /**
         * Duration is multiplied by factor each iteration
         */
        factor?: number
        /**
         * The amount of jitter applied each iteration
         */
        jitter?: number
        /**
         * Exit with error after this many steps
         */
        steps?: number
        [k: string]: unknown
    }
    /**
     * Consumer group for kafka client
     */
    consumerGroup?: {
        /**
         * The name for the consumer group to use
         */
        groupName: string
        /**
         * When starting up a new group do we want to start from the oldest event (true) or the newest event (false), defaults to false
         */
        oldest?: boolean
        /**
         * Rebalance strategy can be one of: sticky, roundrobin, range. Range is the default.
         */
        rebalanceStrategy?: string
        [k: string]: unknown
    }
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * JSONBody specifies that all event body payload coming from this source will be JSON
     */
    jsonBody?: boolean
    /**
     * Sets a limit on how many events get read from kafka per second.
     */
    limitEventsPerSecond?: number
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * Partition name
     */
    partition?: string
    /**
     * SASL configuration for the kafka client
     */
    sasl?: {
        /**
         * SASLMechanism is the name of the enabled SASL mechanism. Possible values: OAUTHBEARER, PLAIN (defaults to PLAIN).
         */
        mechanism?: string
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        passwordSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        userSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        [k: string]: unknown
    }
    /**
     * TLS configuration for the kafka client.
     */
    tls?: {
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        caCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        clientCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        clientKeySecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * If true, skips creation of TLSConfig with certs and creates an empty TLSConfig. (Defaults to false)
         */
        insecureSkipVerify?: boolean
        [k: string]: unknown
    }
    /**
     * Topic name
     */
    topic: string
    /**
     * URL to kafka cluster, multiple URLs separated by comma
     */
    url: string
    /**
     * Specify what kafka version is being connected to enables certain features in sarama, defaults to 1.0.0
     */
    version?: string
    [k: string]: unknown
}
/**
 * S3Artifact contains information about an S3 connection and bucket
 */
export interface IoArgoprojCommonS3Artifact {
    accessKey: IoK8SApiCoreV1SecretKeySelector
    bucket: IoArgoprojCommonS3Bucket
    endpoint: string
    events?: string[]
    filter?: IoArgoprojCommonS3Filter
    insecure?: boolean
    metadata?: {
        [k: string]: string
    }
    region?: string
    secretKey: IoK8SApiCoreV1SecretKeySelector
    [k: string]: unknown
}
/**
 * SecretKeySelector selects a key of a Secret.
 */
export interface IoK8SApiCoreV1SecretKeySelector {
    /**
     * The key of the secret to select from.  Must be a valid secret key.
     */
    key: string
    /**
     * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
     */
    name?: string
    /**
     * Specify whether the Secret or its key must be defined
     */
    optional?: boolean
    [k: string]: unknown
}
/**
 * S3Bucket contains information to describe an S3 Bucket
 */
export interface IoArgoprojCommonS3Bucket {
    key?: string
    name: string
    [k: string]: unknown
}
/**
 * S3Filter represents filters to apply to bucket notifications for specifying constraints on objects
 */
export interface IoArgoprojCommonS3Filter {
    prefix: string
    suffix: string
    [k: string]: unknown
}
/**
 * MQTTEventSource refers to event-source for MQTT related events
 */
export interface IoArgoprojEventsourceV1Alpha1MQTTEventSource {
    /**
     * ClientID is the id of the client
     */
    clientId: string
    /**
     * ConnectionBackoff holds backoff applied to connection.
     */
    connectionBackoff?: {
        /**
         * The initial duration in nanoseconds or strings like "1s", "3m"
         */
        duration?: number | string
        /**
         * Duration is multiplied by factor each iteration
         */
        factor?: number
        /**
         * The amount of jitter applied each iteration
         */
        jitter?: number
        /**
         * Exit with error after this many steps
         */
        steps?: number
        [k: string]: unknown
    }
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * JSONBody specifies that all event body payload coming from this source will be JSON
     */
    jsonBody?: boolean
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * TLS configuration for the mqtt client.
     */
    tls?: {
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        caCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        clientCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        clientKeySecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * If true, skips creation of TLSConfig with certs and creates an empty TLSConfig. (Defaults to false)
         */
        insecureSkipVerify?: boolean
        [k: string]: unknown
    }
    /**
     * Topic name
     */
    topic: string
    /**
     * URL to connect to broker
     */
    url: string
    [k: string]: unknown
}
/**
 * NATSEventsSource refers to event-source for NATS related events
 */
export interface IoArgoprojEventsourceV1Alpha1NATSEventsSource {
    /**
     * Auth information
     */
    auth?: {
        /**
         * Baisc auth with username and password
         */
        basic?: {
            /**
             * Password refers to the Kubernetes secret that holds the password required for basic auth.
             */
            password?: {
                /**
                 * The key of the secret to select from.  Must be a valid secret key.
                 */
                key: string
                /**
                 * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                 */
                name?: string
                /**
                 * Specify whether the Secret or its key must be defined
                 */
                optional?: boolean
                [k: string]: unknown
            }
            /**
             * Username refers to the Kubernetes secret that holds the username required for basic auth.
             */
            username?: {
                /**
                 * The key of the secret to select from.  Must be a valid secret key.
                 */
                key: string
                /**
                 * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                 */
                name?: string
                /**
                 * Specify whether the Secret or its key must be defined
                 */
                optional?: boolean
                [k: string]: unknown
            }
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        credential?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        nkey?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        token?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        [k: string]: unknown
    }
    /**
     * ConnectionBackoff holds backoff applied to connection.
     */
    connectionBackoff?: {
        /**
         * The initial duration in nanoseconds or strings like "1s", "3m"
         */
        duration?: number | string
        /**
         * Duration is multiplied by factor each iteration
         */
        factor?: number
        /**
         * The amount of jitter applied each iteration
         */
        jitter?: number
        /**
         * Exit with error after this many steps
         */
        steps?: number
        [k: string]: unknown
    }
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * JSONBody specifies that all event body payload coming from this source will be JSON
     */
    jsonBody?: boolean
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * Subject holds the name of the subject onto which messages are published
     */
    subject: string
    /**
     * TLS configuration for the nats client.
     */
    tls?: {
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        caCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        clientCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        clientKeySecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * If true, skips creation of TLSConfig with certs and creates an empty TLSConfig. (Defaults to false)
         */
        insecureSkipVerify?: boolean
        [k: string]: unknown
    }
    /**
     * URL to connect to NATS cluster
     */
    url: string
    [k: string]: unknown
}
/**
 * NSQEventSource describes the event source for NSQ PubSub More info at https://godoc.org/github.com/nsqio/go-nsq
 */
export interface IoArgoprojEventsourceV1Alpha1NSQEventSource {
    /**
     * Channel used for subscription
     */
    channel: string
    /**
     * Backoff holds parameters applied to connection.
     */
    connectionBackoff?: {
        /**
         * The initial duration in nanoseconds or strings like "1s", "3m"
         */
        duration?: number | string
        /**
         * Duration is multiplied by factor each iteration
         */
        factor?: number
        /**
         * The amount of jitter applied each iteration
         */
        jitter?: number
        /**
         * Exit with error after this many steps
         */
        steps?: number
        [k: string]: unknown
    }
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * HostAddress is the address of the host for NSQ lookup
     */
    hostAddress: string
    /**
     * JSONBody specifies that all event body payload coming from this source will be JSON
     */
    jsonBody?: boolean
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * TLS configuration for the nsq client.
     */
    tls?: {
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        caCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        clientCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        clientKeySecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * If true, skips creation of TLSConfig with certs and creates an empty TLSConfig. (Defaults to false)
         */
        insecureSkipVerify?: boolean
        [k: string]: unknown
    }
    /**
     * Topic to subscribe to.
     */
    topic: string
    [k: string]: unknown
}
/**
 * PubSubEventSource refers to event-source for GCP PubSub related events.
 */
export interface IoArgoprojEventsourceV1Alpha1PubSubEventSource {
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    credentialSecret?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * DeleteSubscriptionOnFinish determines whether to delete the GCP PubSub subscription once the event source is stopped.
     */
    deleteSubscriptionOnFinish?: boolean
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * JSONBody specifies that all event body payload coming from this source will be JSON
     */
    jsonBody?: boolean
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * ProjectID is GCP project ID for the subscription. Required if you run Argo Events outside of GKE/GCE. (otherwise, the default value is its project)
     */
    projectID?: string
    /**
     * SubscriptionID is ID of subscription. Required if you use existing subscription. The default value will be auto generated hash based on this eventsource setting, so the subscription might be recreated every time you update the setting, which has a possibility of event loss.
     */
    subscriptionID?: string
    /**
     * Topic to which the subscription should belongs. Required if you want the eventsource to create a new subscription. If you specify this field along with an existing subscription, it will be verified whether it actually belongs to the specified topic.
     */
    topic?: string
    /**
     * TopicProjectID is GCP project ID for the topic. By default, it is same as ProjectID.
     */
    topicProjectID?: string
    [k: string]: unknown
}
/**
 * PulsarEventSource describes the event source for Apache Pulsar
 */
export interface IoArgoprojEventsourceV1Alpha1PulsarEventSource {
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    authTokenSecret?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * Backoff holds parameters applied to connection.
     */
    connectionBackoff?: {
        /**
         * The initial duration in nanoseconds or strings like "1s", "3m"
         */
        duration?: number | string
        /**
         * Duration is multiplied by factor each iteration
         */
        factor?: number
        /**
         * The amount of jitter applied each iteration
         */
        jitter?: number
        /**
         * Exit with error after this many steps
         */
        steps?: number
        [k: string]: unknown
    }
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * JSONBody specifies that all event body payload coming from this source will be JSON
     */
    jsonBody?: boolean
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * TLS configuration for the pulsar client.
     */
    tls?: {
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        caCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        clientCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        clientKeySecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * If true, skips creation of TLSConfig with certs and creates an empty TLSConfig. (Defaults to false)
         */
        insecureSkipVerify?: boolean
        [k: string]: unknown
    }
    /**
     * Whether the Pulsar client accept untrusted TLS certificate from broker.
     */
    tlsAllowInsecureConnection?: boolean
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    tlsTrustCertsSecret?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * Whether the Pulsar client verify the validity of the host name from broker.
     */
    tlsValidateHostname?: boolean
    /**
     * Name of the topics to subscribe to.
     */
    topics: string[]
    /**
     * Type of the subscription. Only "exclusive" and "shared" is supported. Defaults to exclusive.
     */
    type?: string
    /**
     * Configure the service URL for the Pulsar service.
     */
    url: string
    [k: string]: unknown
}
/**
 * RedisEventSource describes an event source for the Redis PubSub. More info at https://godoc.org/github.com/go-redis/redis#example-PubSub
 */
export interface IoArgoprojEventsourceV1Alpha1RedisEventSource {
    channels: string[]
    /**
     * DB to use. If not specified, default DB 0 will be used.
     */
    db?: number
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * HostAddress refers to the address of the Redis host/server
     */
    hostAddress: string
    /**
     * JSONBody specifies that all event body payload coming from this source will be JSON
     */
    jsonBody?: boolean
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * Namespace to use to retrieve the password from. It should only be specified if password is declared
     */
    namespace?: string
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    password?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * TLS configuration for the redis client.
     */
    tls?: {
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        caCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        clientCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        clientKeySecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * If true, skips creation of TLSConfig with certs and creates an empty TLSConfig. (Defaults to false)
         */
        insecureSkipVerify?: boolean
        [k: string]: unknown
    }
    /**
     * Username required for ACL style authentication if any.
     */
    username?: string
    [k: string]: unknown
}
/**
 * RedisStreamEventSource describes an event source for Redis streams (https://redis.io/topics/streams-intro)
 */
export interface IoArgoprojEventsourceV1Alpha1RedisStreamEventSource {
    /**
     * ConsumerGroup refers to the Redis stream consumer group that will be created on all redis streams. Messages are read through this group. Defaults to 'argo-events-cg'
     */
    consumerGroup?: string
    /**
     * DB to use. If not specified, default DB 0 will be used.
     */
    db?: number
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * HostAddress refers to the address of the Redis host/server (master instance)
     */
    hostAddress: string
    /**
     * MaxMsgCountPerRead holds the maximum number of messages per stream that will be read in each XREADGROUP of all streams Example: if there are 2 streams and MaxMsgCountPerRead=10, then each XREADGROUP may read upto a total of 20 messages. Same as COUNT option in XREADGROUP(https://redis.io/topics/streams-intro). Defaults to 10
     */
    maxMsgCountPerRead?: number
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    password?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * Streams to look for entries. XREADGROUP is used on all streams using a single consumer group.
     */
    streams: string[]
    /**
     * TLS configuration for the redis client.
     */
    tls?: {
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        caCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        clientCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        clientKeySecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * If true, skips creation of TLSConfig with certs and creates an empty TLSConfig. (Defaults to false)
         */
        insecureSkipVerify?: boolean
        [k: string]: unknown
    }
    /**
     * Username required for ACL style authentication if any.
     */
    username?: string
    [k: string]: unknown
}
/**
 * ResourceEventSource refers to a event-source for K8s resource related events.
 */
export interface IoArgoprojEventsourceV1Alpha1ResourceEventSource {
    /**
     * EventTypes is the list of event type to watch. Possible values are - ADD, UPDATE and DELETE.
     */
    eventTypes: string[]
    /**
     * Filter is applied on the metadata of the resource If you apply filter, then the internal event informer will only monitor objects that pass the filter.
     */
    filter?: {
        /**
         * If the resource is created after the start time then the event is treated as valid.
         */
        afterStart?: boolean
        /**
         * If resource is created before the specified time then the event is treated as valid.
         */
        createdBy?: string
        /**
         * Fields provide field filters similar to K8s field selector (see https://kubernetes.io/docs/concepts/overview/working-with-objects/field-selectors/). Unlike K8s field selector, it supports arbitrary fileds like "spec.serviceAccountName", and the value could be a string or a regex. Same as K8s field selector, operator "=", "==" and "!=" are supported.
         */
        fields?: IoArgoprojEventsourceV1Alpha1Selector[]
        /**
         * Labels provide listing options to K8s API to watch resource/s. Refer https://kubernetes.io/docs/concepts/overview/working-with-objects/label-selectors/ for more info.
         */
        labels?: IoArgoprojEventsourceV1Alpha1Selector[]
        /**
         * Prefix filter is applied on the resource name.
         */
        prefix?: string
        [k: string]: unknown
    }
    group: string
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * Namespace where resource is deployed
     */
    namespace: string
    resource: string
    version: string
    [k: string]: unknown
}
/**
 * Selector represents conditional operation to select K8s objects.
 */
export interface IoArgoprojEventsourceV1Alpha1Selector {
    /**
     * Key name
     */
    key: string
    /**
     * Supported operations like ==, !=, <=, >= etc. Defaults to ==. Refer https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/#label-selectors for more info.
     */
    operation?: string
    /**
     * Value
     */
    value: string
    [k: string]: unknown
}
/**
 * ServicePort contains information on service's port.
 */
export interface IoK8SApiCoreV1ServicePort {
    /**
     * The application protocol for this port. This field follows standard Kubernetes label syntax. Un-prefixed names are reserved for IANA standard service names (as per RFC-6335 and http://www.iana.org/assignments/service-names). Non-standard protocols should use prefixed names such as mycompany.com/my-custom-protocol. This is a beta field that is guarded by the ServiceAppProtocol feature gate and enabled by default.
     */
    appProtocol?: string
    /**
     * The name of this port within the service. This must be a DNS_LABEL. All ports within a ServiceSpec must have unique names. When considering the endpoints for a Service, this must match the 'name' field in the EndpointPort. Optional if only one ServicePort is defined on this service.
     */
    name?: string
    /**
     * The port on each node on which this service is exposed when type is NodePort or LoadBalancer.  Usually assigned by the system. If a value is specified, in-range, and not in use it will be used, otherwise the operation will fail.  If not specified, a port will be allocated if this Service requires one.  If this field is specified when creating a Service which does not need it, creation will fail. This field will be wiped when updating a Service to no longer need it (e.g. changing type from NodePort to ClusterIP). More info: https://kubernetes.io/docs/concepts/services-networking/service/#type-nodeport
     */
    nodePort?: number
    /**
     * The port that will be exposed by this service.
     */
    port: number
    /**
     * The IP protocol for this port. Supports "TCP", "UDP", and "SCTP". Default is TCP.
     */
    protocol?: string
    /**
     * Number or name of the port to access on the pods targeted by the service. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME. If this is a string, it will be looked up as a named port in the target Pod's container ports. If this is not specified, the value of the 'port' field is used (an identity map). This field is ignored for services with clusterIP=None, and should be omitted or set equal to the 'port' field. More info: https://kubernetes.io/docs/concepts/services-networking/service/#defining-a-service
     */
    targetPort?: number | string
    [k: string]: unknown
}
/**
 * SlackEventSource refers to event-source for Slack related events
 */
export interface IoArgoprojEventsourceV1Alpha1SlackEventSource {
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    signingSecret?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    token?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * Webhook holds configuration for a REST endpoint
     */
    webhook?: {
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        authSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * REST API endpoint
         */
        endpoint: string
        /**
         * MaxPayloadSize is the maximum webhook payload size that the server will accept. Requests exceeding that limit will be rejected with "request too large" response. Default value: 1048576 (1MB).
         */
        maxPayloadSize?: number
        /**
         * Metadata holds the user defined metadata which will passed along the event payload.
         */
        metadata?: {
            [k: string]: string
        }
        /**
         * Method is HTTP request method that indicates the desired action to be performed for a given resource. See RFC7231 Hypertext Transfer Protocol (HTTP/1.1): Semantics and Content
         */
        method: string
        /**
         * Port on which HTTP server is listening for incoming events.
         */
        port: string
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        serverCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        serverKeySecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * URL is the url of the server.
         */
        url: string
        [k: string]: unknown
    }
    [k: string]: unknown
}
/**
 * SNSEventSource refers to event-source for AWS SNS related events
 */
export interface IoArgoprojEventsourceV1Alpha1SNSEventSource {
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    accessKey?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * Endpoint configures connection to a specific SNS endpoint instead of Amazons servers
     */
    endpoint?: string
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * Region is AWS region
     */
    region: string
    /**
     * RoleARN is the Amazon Resource Name (ARN) of the role to assume.
     */
    roleARN?: string
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    secretKey?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * TopicArn
     */
    topicArn: string
    /**
     * ValidateSignature is boolean that can be set to true for SNS signature verification
     */
    validateSignature?: boolean
    /**
     * Webhook configuration for http server
     */
    webhook?: {
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        authSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * REST API endpoint
         */
        endpoint: string
        /**
         * MaxPayloadSize is the maximum webhook payload size that the server will accept. Requests exceeding that limit will be rejected with "request too large" response. Default value: 1048576 (1MB).
         */
        maxPayloadSize?: number
        /**
         * Metadata holds the user defined metadata which will passed along the event payload.
         */
        metadata?: {
            [k: string]: string
        }
        /**
         * Method is HTTP request method that indicates the desired action to be performed for a given resource. See RFC7231 Hypertext Transfer Protocol (HTTP/1.1): Semantics and Content
         */
        method: string
        /**
         * Port on which HTTP server is listening for incoming events.
         */
        port: string
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        serverCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        serverKeySecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * URL is the url of the server.
         */
        url: string
        [k: string]: unknown
    }
    [k: string]: unknown
}
/**
 * SQSEventSource refers to event-source for AWS SQS related events
 */
export interface IoArgoprojEventsourceV1Alpha1SQSEventSource {
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    accessKey?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * DLQ specifies if a dead-letter queue is configured for messages that can't be processed successfully. If set to true, messages with invalid payload won't be acknowledged to allow to forward them farther to the dead-letter queue. The default value is false.
     */
    dlq?: boolean
    /**
     * Endpoint configures connection to a specific SQS endpoint instead of Amazons servers
     */
    endpoint?: string
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * JSONBody specifies that all event body payload coming from this source will be JSON
     */
    jsonBody?: boolean
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * Queue is AWS SQS queue to listen to for messages
     */
    queue: string
    /**
     * QueueAccountID is the ID of the account that created the queue to monitor
     */
    queueAccountId?: string
    /**
     * Region is AWS region
     */
    region: string
    /**
     * RoleARN is the Amazon Resource Name (ARN) of the role to assume.
     */
    roleARN?: string
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    secretKey?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    sessionToken?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * WaitTimeSeconds is The duration (in seconds) for which the call waits for a message to arrive in the queue before returning.
     */
    waitTimeSeconds: number
    [k: string]: unknown
}
/**
 * StorageGridEventSource refers to event-source for StorageGrid related events
 */
export interface IoArgoprojEventsourceV1Alpha1StorageGridEventSource {
    /**
     * APIURL is the url of the storagegrid api.
     */
    apiURL: string
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    authToken: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * Name of the bucket to register notifications for.
     */
    bucket: string
    events?: string[]
    /**
     * Filter on object key which caused the notification.
     */
    filter?: {
        prefix: string
        suffix: string
        [k: string]: unknown
    }
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * S3 region. Defaults to us-east-1
     */
    region?: string
    /**
     * TopicArn
     */
    topicArn: string
    /**
     * Webhook holds configuration for a REST endpoint
     */
    webhook?: {
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        authSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * REST API endpoint
         */
        endpoint: string
        /**
         * MaxPayloadSize is the maximum webhook payload size that the server will accept. Requests exceeding that limit will be rejected with "request too large" response. Default value: 1048576 (1MB).
         */
        maxPayloadSize?: number
        /**
         * Metadata holds the user defined metadata which will passed along the event payload.
         */
        metadata?: {
            [k: string]: string
        }
        /**
         * Method is HTTP request method that indicates the desired action to be performed for a given resource. See RFC7231 Hypertext Transfer Protocol (HTTP/1.1): Semantics and Content
         */
        method: string
        /**
         * Port on which HTTP server is listening for incoming events.
         */
        port: string
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        serverCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        serverKeySecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * URL is the url of the server.
         */
        url: string
        [k: string]: unknown
    }
    [k: string]: unknown
}
/**
 * StripeEventSource describes the event source for stripe webhook notifications More info at https://stripe.com/docs/webhooks
 */
export interface IoArgoprojEventsourceV1Alpha1StripeEventSource {
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    apiKey?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * CreateWebhook if specified creates a new webhook programmatically.
     */
    createWebhook?: boolean
    /**
     * EventFilter describes the type of events to listen to. If not specified, all types of events will be processed. More info at https://stripe.com/docs/api/events/list
     */
    eventFilter?: string[]
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * Webhook holds configuration for a REST endpoint
     */
    webhook?: {
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        authSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * REST API endpoint
         */
        endpoint: string
        /**
         * MaxPayloadSize is the maximum webhook payload size that the server will accept. Requests exceeding that limit will be rejected with "request too large" response. Default value: 1048576 (1MB).
         */
        maxPayloadSize?: number
        /**
         * Metadata holds the user defined metadata which will passed along the event payload.
         */
        metadata?: {
            [k: string]: string
        }
        /**
         * Method is HTTP request method that indicates the desired action to be performed for a given resource. See RFC7231 Hypertext Transfer Protocol (HTTP/1.1): Semantics and Content
         */
        method: string
        /**
         * Port on which HTTP server is listening for incoming events.
         */
        port: string
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        serverCertSecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        serverKeySecret?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * URL is the url of the server.
         */
        url: string
        [k: string]: unknown
    }
    [k: string]: unknown
}
/**
 * EnvVar represents an environment variable present in a Container.
 */
export interface IoK8SApiCoreV1EnvVar {
    /**
     * Name of the environment variable. Must be a C_IDENTIFIER.
     */
    name: string
    /**
     * Variable references $(VAR_NAME) are expanded using the previous defined environment variables in the container and any service environment variables. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. Defaults to "".
     */
    value?: string
    /**
     * Source for the environment variable's value. Cannot be used if value is not empty.
     */
    valueFrom?: {
        /**
         * Selects a key from a ConfigMap.
         */
        configMapKeyRef?: {
            /**
             * The key to select.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the ConfigMap or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        /**
         * Selects a field of the pod: supports metadata.name, metadata.namespace, `metadata.labels['<KEY>']`, `metadata.annotations['<KEY>']`, spec.nodeName, spec.serviceAccountName, status.hostIP, status.podIP, status.podIPs.
         */
        fieldRef?: {
            /**
             * Version of the schema the FieldPath is written in terms of, defaults to "v1".
             */
            apiVersion?: string
            /**
             * Path of the field to select in the specified API version.
             */
            fieldPath: string
            [k: string]: unknown
        }
        /**
         * Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, limits.ephemeral-storage, requests.cpu, requests.memory and requests.ephemeral-storage) are currently supported.
         */
        resourceFieldRef?: {
            /**
             * Container name: required for volumes, optional for env vars
             */
            containerName?: string
            /**
             * Quantity is a fixed-point representation of a number. It provides convenient marshaling/unmarshaling in JSON and YAML, in addition to String() and AsInt64() accessors.
             *
             * The serialization format is:
             *
             * <quantity>        ::= <signedNumber><suffix>
             *   (Note that <suffix> may be empty, from the "" case in <decimalSI>.)
             * <digit>           ::= 0 | 1 | ... | 9 <digits>          ::= <digit> | <digit><digits> <number>          ::= <digits> | <digits>.<digits> | <digits>. | .<digits> <sign>            ::= "+" | "-" <signedNumber>    ::= <number> | <sign><number> <suffix>          ::= <binarySI> | <decimalExponent> | <decimalSI> <binarySI>        ::= Ki | Mi | Gi | Ti | Pi | Ei
             *   (International System of units; See: http://physics.nist.gov/cuu/Units/binary.html)
             * <decimalSI>       ::= m | "" | k | M | G | T | P | E
             *   (Note that 1024 = 1Ki but 1000 = 1k; I didn't choose the capitalization.)
             * <decimalExponent> ::= "e" <signedNumber> | "E" <signedNumber>
             *
             * No matter which of the three exponent forms is used, no quantity may represent a number greater than 2^63-1 in magnitude, nor may it have more than 3 decimal places. Numbers larger or more precise will be capped or rounded up. (E.g.: 0.1m will rounded up to 1m.) This may be extended in the future if we require larger or smaller quantities.
             *
             * When a Quantity is parsed from a string, it will remember the type of suffix it had, and will use the same type again when it is serialized.
             *
             * Before serializing, Quantity will be put in "canonical form". This means that Exponent/suffix will be adjusted up or down (with a corresponding increase or decrease in Mantissa) such that:
             *   a. No precision is lost
             *   b. No fractional digits will be emitted
             *   c. The exponent (or suffix) is as large as possible.
             * The sign will be omitted unless the number is negative.
             *
             * Examples:
             *   1.5 will be serialized as "1500m"
             *   1.5Gi will be serialized as "1536Mi"
             *
             * Note that the quantity will NEVER be internally represented by a floating point number. That is the whole point of this exercise.
             *
             * Non-canonical values will still parse as long as they are well formed, but will be re-emitted in their canonical form. (So always use canonical form, or don't diff.)
             *
             * This format is intended to make it difficult to use these numbers without writing some sort of special handling code in the hopes that that will cause implementors to also use a fixed point implementation.
             */
            divisor?: string
            /**
             * Required: resource to select
             */
            resource: string
            [k: string]: unknown
        }
        /**
         * SecretKeySelector selects a key of a Secret.
         */
        secretKeyRef?: {
            /**
             * The key of the secret to select from.  Must be a valid secret key.
             */
            key: string
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            /**
             * Specify whether the Secret or its key must be defined
             */
            optional?: boolean
            [k: string]: unknown
        }
        [k: string]: unknown
    }
    [k: string]: unknown
}
/**
 * EnvFromSource represents the source of a set of ConfigMaps
 */
export interface IoK8SApiCoreV1EnvFromSource {
    /**
     * The ConfigMap to select from
     */
    configMapRef?: {
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the ConfigMap must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * An optional identifier to prepend to each key in the ConfigMap. Must be a C_IDENTIFIER.
     */
    prefix?: string
    /**
     * The Secret to select from
     */
    secretRef?: {
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    [k: string]: unknown
}
/**
 * HTTPHeader describes a custom header to be used in HTTP probes
 */
export interface IoK8SApiCoreV1HTTPHeader {
    /**
     * The header field name
     */
    name: string
    /**
     * The header field value
     */
    value: string
    [k: string]: unknown
}
/**
 * ContainerPort represents a network port in a single container.
 */
export interface IoK8SApiCoreV1ContainerPort {
    /**
     * Number of port to expose on the pod's IP address. This must be a valid port number, 0 < x < 65536.
     */
    containerPort: number
    /**
     * What host IP to bind the external port to.
     */
    hostIP?: string
    /**
     * Number of port to expose on the host. If specified, this must be a valid port number, 0 < x < 65536. If HostNetwork is specified, this must match ContainerPort. Most containers do not need this.
     */
    hostPort?: number
    /**
     * If specified, this must be an IANA_SVC_NAME and unique within the pod. Each named port in a pod must have a unique name. Name for the port that can be referred to by services.
     */
    name?: string
    /**
     * Protocol for port. Must be UDP, TCP, or SCTP. Defaults to "TCP".
     */
    protocol?: string
    [k: string]: unknown
}
/**
 * volumeDevice describes a mapping of a raw block device within a container.
 */
export interface IoK8SApiCoreV1VolumeDevice {
    /**
     * devicePath is the path inside of the container that the device will be mapped to.
     */
    devicePath: string
    /**
     * name must match the name of a persistentVolumeClaim in the pod
     */
    name: string
    [k: string]: unknown
}
/**
 * VolumeMount describes a mounting of a Volume within a container.
 */
export interface IoK8SApiCoreV1VolumeMount {
    /**
     * Path within the container at which the volume should be mounted.  Must not contain ':'.
     */
    mountPath: string
    /**
     * mountPropagation determines how mounts are propagated from the host to container and the other way around. When not set, MountPropagationNone is used. This field is beta in 1.10.
     */
    mountPropagation?: string
    /**
     * This must match the Name of a Volume.
     */
    name: string
    /**
     * Mounted read-only if true, read-write otherwise (false or unspecified). Defaults to false.
     */
    readOnly?: boolean
    /**
     * Path within the volume from which the container's volume should be mounted. Defaults to "" (volume's root).
     */
    subPath?: string
    /**
     * Expanded path within the volume from which the container's volume should be mounted. Behaves similarly to SubPath but environment variable references $(VAR_NAME) are expanded using the container's environment. Defaults to "" (volume's root). SubPathExpr and SubPath are mutually exclusive.
     */
    subPathExpr?: string
    [k: string]: unknown
}
/**
 * Volume represents a named volume in a pod that may be accessed by any container in the pod.
 */
export interface IoK8SApiCoreV1Volume {
    /**
     * AWSElasticBlockStore represents an AWS Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
     */
    awsElasticBlockStore?: {
        /**
         * Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
         */
        fsType?: string
        /**
         * The partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as "1". Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty).
         */
        partition?: number
        /**
         * Specify "true" to force and set the ReadOnly property in VolumeMounts to "true". If omitted, the default is "false". More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
         */
        readOnly?: boolean
        /**
         * Unique ID of the persistent disk resource in AWS (Amazon EBS volume). More info: https://kubernetes.io/docs/concepts/storage/volumes#awselasticblockstore
         */
        volumeID: string
        [k: string]: unknown
    }
    /**
     * AzureDisk represents an Azure Data Disk mount on the host and bind mount to the pod.
     */
    azureDisk?: {
        /**
         * Host Caching mode: None, Read Only, Read Write.
         */
        cachingMode?: string
        /**
         * The Name of the data disk in the blob storage
         */
        diskName: string
        /**
         * The URI the data disk in the blob storage
         */
        diskURI: string
        /**
         * Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
         */
        fsType?: string
        /**
         * Expected values Shared: multiple blob disks per storage account  Dedicated: single blob disk per storage account  Managed: azure managed data disk (only in managed availability set). defaults to shared
         */
        kind?: string
        /**
         * Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
         */
        readOnly?: boolean
        [k: string]: unknown
    }
    /**
     * AzureFile represents an Azure File Service mount on the host and bind mount to the pod.
     */
    azureFile?: {
        /**
         * Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
         */
        readOnly?: boolean
        /**
         * the name of secret that contains Azure Storage Account Name and Key
         */
        secretName: string
        /**
         * Share Name
         */
        shareName: string
        [k: string]: unknown
    }
    /**
     * CephFS represents a Ceph FS mount on the host that shares a pod's lifetime
     */
    cephfs?: {
        /**
         * Required: Monitors is a collection of Ceph monitors More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
         */
        monitors: string[]
        /**
         * Optional: Used as the mounted root, rather than the full Ceph tree, default is /
         */
        path?: string
        /**
         * Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
         */
        readOnly?: boolean
        /**
         * Optional: SecretFile is the path to key ring for User, default is /etc/ceph/user.secret More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
         */
        secretFile?: string
        /**
         * LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
         */
        secretRef?: {
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            [k: string]: unknown
        }
        /**
         * Optional: User is the rados user name, default is admin More info: https://examples.k8s.io/volumes/cephfs/README.md#how-to-use-it
         */
        user?: string
        [k: string]: unknown
    }
    /**
     * Cinder represents a cinder volume attached and mounted on kubelets host machine. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
     */
    cinder?: {
        /**
         * Filesystem type to mount. Must be a filesystem type supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
         */
        fsType?: string
        /**
         * Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
         */
        readOnly?: boolean
        /**
         * LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
         */
        secretRef?: {
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            [k: string]: unknown
        }
        /**
         * volume id used to identify the volume in cinder. More info: https://examples.k8s.io/mysql-cinder-pd/README.md
         */
        volumeID: string
        [k: string]: unknown
    }
    /**
     * ConfigMap represents a configMap that should populate this volume
     */
    configMap?: {
        /**
         * Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
         */
        defaultMode?: number
        /**
         * If unspecified, each key-value pair in the Data field of the referenced ConfigMap will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the ConfigMap, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
         */
        items?: IoK8SApiCoreV1KeyToPath[]
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the ConfigMap or its keys must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * CSI (Container Storage Interface) represents ephemeral storage that is handled by certain external CSI drivers (Beta feature).
     */
    csi?: {
        /**
         * Driver is the name of the CSI driver that handles this volume. Consult with your admin for the correct name as registered in the cluster.
         */
        driver: string
        /**
         * Filesystem type to mount. Ex. "ext4", "xfs", "ntfs". If not provided, the empty value is passed to the associated CSI driver which will determine the default filesystem to apply.
         */
        fsType?: string
        /**
         * LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
         */
        nodePublishSecretRef?: {
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            [k: string]: unknown
        }
        /**
         * Specifies a read-only configuration for the volume. Defaults to false (read/write).
         */
        readOnly?: boolean
        /**
         * VolumeAttributes stores driver-specific properties that are passed to the CSI driver. Consult your driver's documentation for supported values.
         */
        volumeAttributes?: {
            [k: string]: string
        }
        [k: string]: unknown
    }
    /**
     * DownwardAPI represents downward API about the pod that should populate this volume
     */
    downwardAPI?: {
        /**
         * Optional: mode bits to use on created files by default. Must be a Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
         */
        defaultMode?: number
        /**
         * Items is a list of downward API volume file
         */
        items?: IoK8SApiCoreV1DownwardAPIVolumeFile[]
        [k: string]: unknown
    }
    /**
     * EmptyDir represents a temporary directory that shares a pod's lifetime. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
     */
    emptyDir?: {
        /**
         * What type of storage medium should back this directory. The default is "" which means to use the node's default medium. Must be an empty string (default) or Memory. More info: https://kubernetes.io/docs/concepts/storage/volumes#emptydir
         */
        medium?: string
        /**
         * Quantity is a fixed-point representation of a number. It provides convenient marshaling/unmarshaling in JSON and YAML, in addition to String() and AsInt64() accessors.
         *
         * The serialization format is:
         *
         * <quantity>        ::= <signedNumber><suffix>
         *   (Note that <suffix> may be empty, from the "" case in <decimalSI>.)
         * <digit>           ::= 0 | 1 | ... | 9 <digits>          ::= <digit> | <digit><digits> <number>          ::= <digits> | <digits>.<digits> | <digits>. | .<digits> <sign>            ::= "+" | "-" <signedNumber>    ::= <number> | <sign><number> <suffix>          ::= <binarySI> | <decimalExponent> | <decimalSI> <binarySI>        ::= Ki | Mi | Gi | Ti | Pi | Ei
         *   (International System of units; See: http://physics.nist.gov/cuu/Units/binary.html)
         * <decimalSI>       ::= m | "" | k | M | G | T | P | E
         *   (Note that 1024 = 1Ki but 1000 = 1k; I didn't choose the capitalization.)
         * <decimalExponent> ::= "e" <signedNumber> | "E" <signedNumber>
         *
         * No matter which of the three exponent forms is used, no quantity may represent a number greater than 2^63-1 in magnitude, nor may it have more than 3 decimal places. Numbers larger or more precise will be capped or rounded up. (E.g.: 0.1m will rounded up to 1m.) This may be extended in the future if we require larger or smaller quantities.
         *
         * When a Quantity is parsed from a string, it will remember the type of suffix it had, and will use the same type again when it is serialized.
         *
         * Before serializing, Quantity will be put in "canonical form". This means that Exponent/suffix will be adjusted up or down (with a corresponding increase or decrease in Mantissa) such that:
         *   a. No precision is lost
         *   b. No fractional digits will be emitted
         *   c. The exponent (or suffix) is as large as possible.
         * The sign will be omitted unless the number is negative.
         *
         * Examples:
         *   1.5 will be serialized as "1500m"
         *   1.5Gi will be serialized as "1536Mi"
         *
         * Note that the quantity will NEVER be internally represented by a floating point number. That is the whole point of this exercise.
         *
         * Non-canonical values will still parse as long as they are well formed, but will be re-emitted in their canonical form. (So always use canonical form, or don't diff.)
         *
         * This format is intended to make it difficult to use these numbers without writing some sort of special handling code in the hopes that that will cause implementors to also use a fixed point implementation.
         */
        sizeLimit?: string
        [k: string]: unknown
    }
    /**
     * Ephemeral represents a volume that is handled by a cluster storage driver (Alpha feature). The volume's lifecycle is tied to the pod that defines it - it will be created before the pod starts, and deleted when the pod is removed.
     *
     * Use this if: a) the volume is only needed while the pod runs, b) features of normal volumes like restoring from snapshot or capacity
     *    tracking are needed,
     * c) the storage driver is specified through a storage class, and d) the storage driver supports dynamic volume provisioning through
     *    a PersistentVolumeClaim (see EphemeralVolumeSource for more
     *    information on the connection between this volume type
     *    and PersistentVolumeClaim).
     *
     * Use PersistentVolumeClaim or one of the vendor-specific APIs for volumes that persist for longer than the lifecycle of an individual pod.
     *
     * Use CSI for light-weight local ephemeral volumes if the CSI driver is meant to be used that way - see the documentation of the driver for more information.
     *
     * A pod can use both types of ephemeral volumes and persistent volumes at the same time.
     */
    ephemeral?: {
        /**
         * Specifies a read-only configuration for the volume. Defaults to false (read/write).
         */
        readOnly?: boolean
        /**
         * Will be used to create a stand-alone PVC to provision the volume. The pod in which this EphemeralVolumeSource is embedded will be the owner of the PVC, i.e. the PVC will be deleted together with the pod.  The name of the PVC will be `<pod name>-<volume name>` where `<volume name>` is the name from the `PodSpec.Volumes` array entry. Pod validation will reject the pod if the concatenated name is not valid for a PVC (for example, too long).
         *
         * An existing PVC with that name that is not owned by the pod will *not* be used for the pod to avoid using an unrelated volume by mistake. Starting the pod is then blocked until the unrelated PVC is removed. If such a pre-created PVC is meant to be used by the pod, the PVC has to updated with an owner reference to the pod once the pod exists. Normally this should not be necessary, but it may be useful when manually reconstructing a broken cluster.
         *
         * This field is read-only and no changes will be made by Kubernetes to the PVC after it has been created.
         *
         * Required, must not be nil.
         */
        volumeClaimTemplate?: {
            /**
             * ObjectMeta is metadata that all persisted resources must have, which includes all objects users must create.
             */
            metadata?: {
                /**
                 * Annotations is an unstructured key value map stored with a resource that may be set by external tools to store and retrieve arbitrary metadata. They are not queryable and should be preserved when modifying objects. More info: http://kubernetes.io/docs/user-guide/annotations
                 */
                annotations?: {
                    [k: string]: string
                }
                /**
                 * The name of the cluster which the object belongs to. This is used to distinguish resources with same name and namespace in different clusters. This field is not set anywhere right now and apiserver is going to ignore it if set in create or update request.
                 */
                clusterName?: string
                /**
                 * CreationTimestamp is a timestamp representing the server time when this object was created. It is not guaranteed to be set in happens-before order across separate operations. Clients may not set this value. It is represented in RFC3339 form and is in UTC.
                 *
                 * Populated by the system. Read-only. Null for lists. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
                 */
                creationTimestamp?: string
                /**
                 * Number of seconds allowed for this object to gracefully terminate before it will be removed from the system. Only set when deletionTimestamp is also set. May only be shortened. Read-only.
                 */
                deletionGracePeriodSeconds?: number
                /**
                 * DeletionTimestamp is RFC 3339 date and time at which this resource will be deleted. This field is set by the server when a graceful deletion is requested by the user, and is not directly settable by a client. The resource is expected to be deleted (no longer visible from resource lists, and not reachable by name) after the time in this field, once the finalizers list is empty. As long as the finalizers list contains items, deletion is blocked. Once the deletionTimestamp is set, this value may not be unset or be set further into the future, although it may be shortened or the resource may be deleted prior to this time. For example, a user may request that a pod is deleted in 30 seconds. The Kubelet will react by sending a graceful termination signal to the containers in the pod. After that 30 seconds, the Kubelet will send a hard termination signal (SIGKILL) to the container and after cleanup, remove the pod from the API. In the presence of network partitions, this object may still exist after this timestamp, until an administrator or automated process can determine the resource is fully terminated. If not set, graceful deletion of the object has not been requested.
                 *
                 * Populated by the system when a graceful deletion is requested. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
                 */
                deletionTimestamp?: string
                /**
                 * Must be empty before the object is deleted from the registry. Each entry is an identifier for the responsible component that will remove the entry from the list. If the deletionTimestamp of the object is non-nil, entries in this list can only be removed. Finalizers may be processed and removed in any order.  Order is NOT enforced because it introduces significant risk of stuck finalizers. finalizers is a shared field, any actor with permission can reorder it. If the finalizer list is processed in order, then this can lead to a situation in which the component responsible for the first finalizer in the list is waiting for a signal (field value, external system, or other) produced by a component responsible for a finalizer later in the list, resulting in a deadlock. Without enforced ordering finalizers are free to order amongst themselves and are not vulnerable to ordering changes in the list.
                 */
                finalizers?: string[]
                /**
                 * GenerateName is an optional prefix, used by the server, to generate a unique name ONLY IF the Name field has not been provided. If this field is used, the name returned to the client will be different than the name passed. This value will also be combined with a unique suffix. The provided value has the same validation rules as the Name field, and may be truncated by the length of the suffix required to make the value unique on the server.
                 *
                 * If this field is specified and the generated name exists, the server will NOT return a 409 - instead, it will either return 201 Created or 500 with Reason ServerTimeout indicating a unique name could not be found in the time allotted, and the client should retry (optionally after the time indicated in the Retry-After header).
                 *
                 * Applied only if Name is not specified. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#idempotency
                 */
                generateName?: string
                /**
                 * A sequence number representing a specific generation of the desired state. Populated by the system. Read-only.
                 */
                generation?: number
                /**
                 * Map of string keys and values that can be used to organize and categorize (scope and select) objects. May match selectors of replication controllers and services. More info: http://kubernetes.io/docs/user-guide/labels
                 */
                labels?: {
                    [k: string]: string
                }
                /**
                 * ManagedFields maps workflow-id and version to the set of fields that are managed by that workflow. This is mostly for internal housekeeping, and users typically shouldn't need to set or understand this field. A workflow can be the user's name, a controller's name, or the name of a specific apply path like "ci-cd". The set of fields is always in the version that the workflow used when modifying the object.
                 */
                managedFields?: IoK8SApimachineryPkgApisMetaV1ManagedFieldsEntry[]
                /**
                 * Name must be unique within a namespace. Is required when creating resources, although some resources may allow a client to request the generation of an appropriate name automatically. Name is primarily intended for creation idempotence and configuration definition. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/identifiers#names
                 */
                name?: string
                /**
                 * Namespace defines the space within which each name must be unique. An empty namespace is equivalent to the "default" namespace, but "default" is the canonical representation. Not all objects are required to be scoped to a namespace - the value of this field for those objects will be empty.
                 *
                 * Must be a DNS_LABEL. Cannot be updated. More info: http://kubernetes.io/docs/user-guide/namespaces
                 */
                namespace?: string
                /**
                 * List of objects depended by this object. If ALL objects in the list have been deleted, this object will be garbage collected. If this object is managed by a controller, then an entry in this list will point to this controller, with the controller field set to true. There cannot be more than one managing controller.
                 */
                ownerReferences?: IoK8SApimachineryPkgApisMetaV1OwnerReference[]
                /**
                 * An opaque value that represents the internal version of this object that can be used by clients to determine when objects have changed. May be used for optimistic concurrency, change detection, and the watch operation on a resource or set of resources. Clients must treat these values as opaque and passed unmodified back to the server. They may only be valid for a particular resource or set of resources.
                 *
                 * Populated by the system. Read-only. Value must be treated as opaque by clients and . More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#concurrency-control-and-consistency
                 */
                resourceVersion?: string
                /**
                 * SelfLink is a URL representing this object. Populated by the system. Read-only.
                 *
                 * DEPRECATED Kubernetes will stop propagating this field in 1.20 release and the field is planned to be removed in 1.21 release.
                 */
                selfLink?: string
                /**
                 * UID is the unique in time and space value for this object. It is typically generated by the server on successful creation of a resource and is not allowed to change on PUT operations.
                 *
                 * Populated by the system. Read-only. More info: http://kubernetes.io/docs/user-guide/identifiers#uids
                 */
                uid?: string
                [k: string]: unknown
            }
            /**
             * The specification for the PersistentVolumeClaim. The entire content is copied unchanged into the PVC that gets created from this template. The same fields as in a PersistentVolumeClaim are also valid here.
             */
            spec: {
                /**
                 * AccessModes contains the desired access modes the volume should have. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#access-modes-1
                 */
                accessModes?: string[]
                /**
                 * This field can be used to specify either: * An existing VolumeSnapshot object (snapshot.storage.k8s.io/VolumeSnapshot) * An existing PVC (PersistentVolumeClaim) * An existing custom resource that implements data population (Alpha) In order to use custom resource types that implement data population, the AnyVolumeDataSource feature gate must be enabled. If the provisioner or an external controller can support the specified data source, it will create a new volume based on the contents of the specified data source.
                 */
                dataSource?: {
                    /**
                     * APIGroup is the group for the resource being referenced. If APIGroup is not specified, the specified Kind must be in the core API group. For any other third-party types, APIGroup is required.
                     */
                    apiGroup?: string
                    /**
                     * Kind is the type of resource being referenced
                     */
                    kind: string
                    /**
                     * Name is the name of resource being referenced
                     */
                    name: string
                    [k: string]: unknown
                }
                /**
                 * ResourceRequirements describes the compute resource requirements.
                 */
                resources?: {
                    /**
                     * Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/
                     */
                    limits?: {
                        [k: string]: IoK8SApimachineryPkgApiResourceQuantity
                    }
                    /**
                     * Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/
                     */
                    requests?: {
                        [k: string]: IoK8SApimachineryPkgApiResourceQuantity
                    }
                    [k: string]: unknown
                }
                /**
                 * A label query over volumes to consider for binding.
                 */
                selector?: {
                    /**
                     * matchExpressions is a list of label selector requirements. The requirements are ANDed.
                     */
                    matchExpressions?: IoK8SApimachineryPkgApisMetaV1LabelSelectorRequirement[]
                    /**
                     * matchLabels is a map of {key,value} pairs. A single {key,value} in the matchLabels map is equivalent to an element of matchExpressions, whose key field is "key", the operator is "In", and the values array contains only "value". The requirements are ANDed.
                     */
                    matchLabels?: {
                        [k: string]: string
                    }
                    [k: string]: unknown
                }
                /**
                 * Name of the StorageClass required by the claim. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#class-1
                 */
                storageClassName?: string
                /**
                 * volumeMode defines what type of volume is required by the claim. Value of Filesystem is implied when not included in claim spec.
                 */
                volumeMode?: string
                /**
                 * VolumeName is the binding reference to the PersistentVolume backing this claim.
                 */
                volumeName?: string
                [k: string]: unknown
            }
            [k: string]: unknown
        }
        [k: string]: unknown
    }
    /**
     * FC represents a Fibre Channel resource that is attached to a kubelet's host machine and then exposed to the pod.
     */
    fc?: {
        /**
         * Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
         */
        fsType?: string
        /**
         * Optional: FC target lun number
         */
        lun?: number
        /**
         * Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
         */
        readOnly?: boolean
        /**
         * Optional: FC target worldwide names (WWNs)
         */
        targetWWNs?: string[]
        /**
         * Optional: FC volume world wide identifiers (wwids) Either wwids or combination of targetWWNs and lun must be set, but not both simultaneously.
         */
        wwids?: string[]
        [k: string]: unknown
    }
    /**
     * FlexVolume represents a generic volume resource that is provisioned/attached using an exec based plugin.
     */
    flexVolume?: {
        /**
         * Driver is the name of the driver to use for this volume.
         */
        driver: string
        /**
         * Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". The default filesystem depends on FlexVolume script.
         */
        fsType?: string
        /**
         * Optional: Extra command options if any.
         */
        options?: {
            [k: string]: string
        }
        /**
         * Optional: Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
         */
        readOnly?: boolean
        /**
         * LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
         */
        secretRef?: {
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            [k: string]: unknown
        }
        [k: string]: unknown
    }
    /**
     * Flocker represents a Flocker volume attached to a kubelet's host machine. This depends on the Flocker control service being running
     */
    flocker?: {
        /**
         * Name of the dataset stored as metadata -> name on the dataset for Flocker should be considered as deprecated
         */
        datasetName?: string
        /**
         * UUID of the dataset. This is unique identifier of a Flocker dataset
         */
        datasetUUID?: string
        [k: string]: unknown
    }
    /**
     * GCEPersistentDisk represents a GCE Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
     */
    gcePersistentDisk?: {
        /**
         * Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
         */
        fsType?: string
        /**
         * The partition in the volume that you want to mount. If omitted, the default is to mount by volume name. Examples: For volume /dev/sda1, you specify the partition as "1". Similarly, the volume partition for /dev/sda is "0" (or you can leave the property empty). More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
         */
        partition?: number
        /**
         * Unique name of the PD resource in GCE. Used to identify the disk in GCE. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
         */
        pdName: string
        /**
         * ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#gcepersistentdisk
         */
        readOnly?: boolean
        [k: string]: unknown
    }
    /**
     * GitRepo represents a git repository at a particular revision. DEPRECATED: GitRepo is deprecated. To provision a container with a git repo, mount an EmptyDir into an InitContainer that clones the repo using git, then mount the EmptyDir into the Pod's container.
     */
    gitRepo?: {
        /**
         * Target directory name. Must not contain or start with '..'.  If '.' is supplied, the volume directory will be the git repository.  Otherwise, if specified, the volume will contain the git repository in the subdirectory with the given name.
         */
        directory?: string
        /**
         * Repository URL
         */
        repository: string
        /**
         * Commit hash for the specified revision.
         */
        revision?: string
        [k: string]: unknown
    }
    /**
     * Glusterfs represents a Glusterfs mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/glusterfs/README.md
     */
    glusterfs?: {
        /**
         * EndpointsName is the endpoint name that details Glusterfs topology. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
         */
        endpoints: string
        /**
         * Path is the Glusterfs volume path. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
         */
        path: string
        /**
         * ReadOnly here will force the Glusterfs volume to be mounted with read-only permissions. Defaults to false. More info: https://examples.k8s.io/volumes/glusterfs/README.md#create-a-pod
         */
        readOnly?: boolean
        [k: string]: unknown
    }
    /**
     * HostPath represents a pre-existing file or directory on the host machine that is directly exposed to the container. This is generally used for system agents or other privileged things that are allowed to see the host machine. Most containers will NOT need this. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
     */
    hostPath?: {
        /**
         * Path of the directory on the host. If the path is a symlink, it will follow the link to the real path. More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
         */
        path: string
        /**
         * Type for HostPath Volume Defaults to "" More info: https://kubernetes.io/docs/concepts/storage/volumes#hostpath
         */
        type?: string
        [k: string]: unknown
    }
    /**
     * ISCSI represents an ISCSI Disk resource that is attached to a kubelet's host machine and then exposed to the pod. More info: https://examples.k8s.io/volumes/iscsi/README.md
     */
    iscsi?: {
        /**
         * whether support iSCSI Discovery CHAP authentication
         */
        chapAuthDiscovery?: boolean
        /**
         * whether support iSCSI Session CHAP authentication
         */
        chapAuthSession?: boolean
        /**
         * Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#iscsi
         */
        fsType?: string
        /**
         * Custom iSCSI Initiator Name. If initiatorName is specified with iscsiInterface simultaneously, new iSCSI interface <target portal>:<volume name> will be created for the connection.
         */
        initiatorName?: string
        /**
         * Target iSCSI Qualified Name.
         */
        iqn: string
        /**
         * iSCSI Interface Name that uses an iSCSI transport. Defaults to 'default' (tcp).
         */
        iscsiInterface?: string
        /**
         * iSCSI Target Lun number.
         */
        lun: number
        /**
         * iSCSI Target Portal List. The portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
         */
        portals?: string[]
        /**
         * ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false.
         */
        readOnly?: boolean
        /**
         * LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
         */
        secretRef?: {
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            [k: string]: unknown
        }
        /**
         * iSCSI Target Portal. The Portal is either an IP or ip_addr:port if the port is other than default (typically TCP ports 860 and 3260).
         */
        targetPortal: string
        [k: string]: unknown
    }
    /**
     * Volume's name. Must be a DNS_LABEL and unique within the pod. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
     */
    name: string
    /**
     * NFS represents an NFS mount on the host that shares a pod's lifetime More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
     */
    nfs?: {
        /**
         * Path that is exported by the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
         */
        path: string
        /**
         * ReadOnly here will force the NFS export to be mounted with read-only permissions. Defaults to false. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
         */
        readOnly?: boolean
        /**
         * Server is the hostname or IP address of the NFS server. More info: https://kubernetes.io/docs/concepts/storage/volumes#nfs
         */
        server: string
        [k: string]: unknown
    }
    /**
     * PersistentVolumeClaimVolumeSource represents a reference to a PersistentVolumeClaim in the same namespace. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
     */
    persistentVolumeClaim?: {
        /**
         * ClaimName is the name of a PersistentVolumeClaim in the same namespace as the pod using this volume. More info: https://kubernetes.io/docs/concepts/storage/persistent-volumes#persistentvolumeclaims
         */
        claimName: string
        /**
         * Will force the ReadOnly setting in VolumeMounts. Default false.
         */
        readOnly?: boolean
        [k: string]: unknown
    }
    /**
     * PhotonPersistentDisk represents a PhotonController persistent disk attached and mounted on kubelets host machine
     */
    photonPersistentDisk?: {
        /**
         * Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
         */
        fsType?: string
        /**
         * ID that identifies Photon Controller persistent disk
         */
        pdID: string
        [k: string]: unknown
    }
    /**
     * PortworxVolume represents a portworx volume attached and mounted on kubelets host machine
     */
    portworxVolume?: {
        /**
         * FSType represents the filesystem type to mount Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs". Implicitly inferred to be "ext4" if unspecified.
         */
        fsType?: string
        /**
         * Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
         */
        readOnly?: boolean
        /**
         * VolumeID uniquely identifies a Portworx volume
         */
        volumeID: string
        [k: string]: unknown
    }
    /**
     * Items for all in one resources secrets, configmaps, and downward API
     */
    projected?: {
        /**
         * Mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
         */
        defaultMode?: number
        /**
         * list of volume projections
         */
        sources?: IoK8SApiCoreV1VolumeProjection[]
        [k: string]: unknown
    }
    /**
     * Quobyte represents a Quobyte mount on the host that shares a pod's lifetime
     */
    quobyte?: {
        /**
         * Group to map volume access to Default is no group
         */
        group?: string
        /**
         * ReadOnly here will force the Quobyte volume to be mounted with read-only permissions. Defaults to false.
         */
        readOnly?: boolean
        /**
         * Registry represents a single or multiple Quobyte Registry services specified as a string as host:port pair (multiple entries are separated with commas) which acts as the central registry for volumes
         */
        registry: string
        /**
         * Tenant owning the given Quobyte volume in the Backend Used with dynamically provisioned Quobyte volumes, value is set by the plugin
         */
        tenant?: string
        /**
         * User to map volume access to Defaults to serivceaccount user
         */
        user?: string
        /**
         * Volume is a string that references an already created Quobyte volume by name.
         */
        volume: string
        [k: string]: unknown
    }
    /**
     * RBD represents a Rados Block Device mount on the host that shares a pod's lifetime. More info: https://examples.k8s.io/volumes/rbd/README.md
     */
    rbd?: {
        /**
         * Filesystem type of the volume that you want to mount. Tip: Ensure that the filesystem type is supported by the host operating system. Examples: "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified. More info: https://kubernetes.io/docs/concepts/storage/volumes#rbd
         */
        fsType?: string
        /**
         * The rados image name. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
         */
        image: string
        /**
         * Keyring is the path to key ring for RBDUser. Default is /etc/ceph/keyring. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
         */
        keyring?: string
        /**
         * A collection of Ceph monitors. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
         */
        monitors: string[]
        /**
         * The rados pool name. Default is rbd. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
         */
        pool?: string
        /**
         * ReadOnly here will force the ReadOnly setting in VolumeMounts. Defaults to false. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
         */
        readOnly?: boolean
        /**
         * LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
         */
        secretRef?: {
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            [k: string]: unknown
        }
        /**
         * The rados user name. Default is admin. More info: https://examples.k8s.io/volumes/rbd/README.md#how-to-use-it
         */
        user?: string
        [k: string]: unknown
    }
    /**
     * ScaleIO represents a ScaleIO persistent volume attached and mounted on Kubernetes nodes.
     */
    scaleIO?: {
        /**
         * Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Default is "xfs".
         */
        fsType?: string
        /**
         * The host address of the ScaleIO API Gateway.
         */
        gateway: string
        /**
         * The name of the ScaleIO Protection Domain for the configured storage.
         */
        protectionDomain?: string
        /**
         * Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
         */
        readOnly?: boolean
        /**
         * LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
         */
        secretRef: {
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            [k: string]: unknown
        }
        /**
         * Flag to enable/disable SSL communication with Gateway, default false
         */
        sslEnabled?: boolean
        /**
         * Indicates whether the storage for a volume should be ThickProvisioned or ThinProvisioned. Default is ThinProvisioned.
         */
        storageMode?: string
        /**
         * The ScaleIO Storage Pool associated with the protection domain.
         */
        storagePool?: string
        /**
         * The name of the storage system as configured in ScaleIO.
         */
        system: string
        /**
         * The name of a volume already created in the ScaleIO system that is associated with this volume source.
         */
        volumeName?: string
        [k: string]: unknown
    }
    /**
     * Secret represents a secret that should populate this volume. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
     */
    secret?: {
        /**
         * Optional: mode bits used to set permissions on created files by default. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. Defaults to 0644. Directories within the path are not affected by this setting. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
         */
        defaultMode?: number
        /**
         * If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
         */
        items?: IoK8SApiCoreV1KeyToPath[]
        /**
         * Specify whether the Secret or its keys must be defined
         */
        optional?: boolean
        /**
         * Name of the secret in the pod's namespace to use. More info: https://kubernetes.io/docs/concepts/storage/volumes#secret
         */
        secretName?: string
        [k: string]: unknown
    }
    /**
     * StorageOS represents a StorageOS volume attached and mounted on Kubernetes nodes.
     */
    storageos?: {
        /**
         * Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
         */
        fsType?: string
        /**
         * Defaults to false (read/write). ReadOnly here will force the ReadOnly setting in VolumeMounts.
         */
        readOnly?: boolean
        /**
         * LocalObjectReference contains enough information to let you locate the referenced object inside the same namespace.
         */
        secretRef?: {
            /**
             * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
             */
            name?: string
            [k: string]: unknown
        }
        /**
         * VolumeName is the human-readable name of the StorageOS volume.  Volume names are only unique within a namespace.
         */
        volumeName?: string
        /**
         * VolumeNamespace specifies the scope of the volume within StorageOS.  If no namespace is specified then the Pod's namespace will be used.  This allows the Kubernetes name scoping to be mirrored within StorageOS for tighter integration. Set VolumeName to any name to override the default behaviour. Set to "default" if you are not using namespaces within StorageOS. Namespaces that do not pre-exist within StorageOS will be created.
         */
        volumeNamespace?: string
        [k: string]: unknown
    }
    /**
     * VsphereVolume represents a vSphere volume attached and mounted on kubelets host machine
     */
    vsphereVolume?: {
        /**
         * Filesystem type to mount. Must be a filesystem type supported by the host operating system. Ex. "ext4", "xfs", "ntfs". Implicitly inferred to be "ext4" if unspecified.
         */
        fsType?: string
        /**
         * Storage Policy Based Management (SPBM) profile ID associated with the StoragePolicyName.
         */
        storagePolicyID?: string
        /**
         * Storage Policy Based Management (SPBM) profile name.
         */
        storagePolicyName?: string
        /**
         * Path that identifies vSphere volume vmdk
         */
        volumePath: string
        [k: string]: unknown
    }
    [k: string]: unknown
}
/**
 * Maps a string key to a path within a volume.
 */
export interface IoK8SApiCoreV1KeyToPath {
    /**
     * The key to project.
     */
    key: string
    /**
     * Optional: mode bits used to set permissions on this file. Must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
     */
    mode?: number
    /**
     * The relative path of the file to map the key to. May not be an absolute path. May not contain the path element '..'. May not start with the string '..'.
     */
    path: string
    [k: string]: unknown
}
/**
 * DownwardAPIVolumeFile represents information to create the file containing the pod field
 */
export interface IoK8SApiCoreV1DownwardAPIVolumeFile {
    /**
     * Required: Selects a field of the pod: only annotations, labels, name and namespace are supported.
     */
    fieldRef?: {
        /**
         * Version of the schema the FieldPath is written in terms of, defaults to "v1".
         */
        apiVersion?: string
        /**
         * Path of the field to select in the specified API version.
         */
        fieldPath: string
        [k: string]: unknown
    }
    /**
     * Optional: mode bits used to set permissions on this file, must be an octal value between 0000 and 0777 or a decimal value between 0 and 511. YAML accepts both octal and decimal values, JSON requires decimal values for mode bits. If not specified, the volume defaultMode will be used. This might be in conflict with other options that affect the file mode, like fsGroup, and the result can be other mode bits set.
     */
    mode?: number
    /**
     * Required: Path is  the relative path name of the file to be created. Must not be absolute or contain the '..' path. Must be utf-8 encoded. The first item of the relative path must not start with '..'
     */
    path: string
    /**
     * Selects a resource of the container: only resources limits and requests (limits.cpu, limits.memory, requests.cpu and requests.memory) are currently supported.
     */
    resourceFieldRef?: {
        /**
         * Container name: required for volumes, optional for env vars
         */
        containerName?: string
        /**
         * Quantity is a fixed-point representation of a number. It provides convenient marshaling/unmarshaling in JSON and YAML, in addition to String() and AsInt64() accessors.
         *
         * The serialization format is:
         *
         * <quantity>        ::= <signedNumber><suffix>
         *   (Note that <suffix> may be empty, from the "" case in <decimalSI>.)
         * <digit>           ::= 0 | 1 | ... | 9 <digits>          ::= <digit> | <digit><digits> <number>          ::= <digits> | <digits>.<digits> | <digits>. | .<digits> <sign>            ::= "+" | "-" <signedNumber>    ::= <number> | <sign><number> <suffix>          ::= <binarySI> | <decimalExponent> | <decimalSI> <binarySI>        ::= Ki | Mi | Gi | Ti | Pi | Ei
         *   (International System of units; See: http://physics.nist.gov/cuu/Units/binary.html)
         * <decimalSI>       ::= m | "" | k | M | G | T | P | E
         *   (Note that 1024 = 1Ki but 1000 = 1k; I didn't choose the capitalization.)
         * <decimalExponent> ::= "e" <signedNumber> | "E" <signedNumber>
         *
         * No matter which of the three exponent forms is used, no quantity may represent a number greater than 2^63-1 in magnitude, nor may it have more than 3 decimal places. Numbers larger or more precise will be capped or rounded up. (E.g.: 0.1m will rounded up to 1m.) This may be extended in the future if we require larger or smaller quantities.
         *
         * When a Quantity is parsed from a string, it will remember the type of suffix it had, and will use the same type again when it is serialized.
         *
         * Before serializing, Quantity will be put in "canonical form". This means that Exponent/suffix will be adjusted up or down (with a corresponding increase or decrease in Mantissa) such that:
         *   a. No precision is lost
         *   b. No fractional digits will be emitted
         *   c. The exponent (or suffix) is as large as possible.
         * The sign will be omitted unless the number is negative.
         *
         * Examples:
         *   1.5 will be serialized as "1500m"
         *   1.5Gi will be serialized as "1536Mi"
         *
         * Note that the quantity will NEVER be internally represented by a floating point number. That is the whole point of this exercise.
         *
         * Non-canonical values will still parse as long as they are well formed, but will be re-emitted in their canonical form. (So always use canonical form, or don't diff.)
         *
         * This format is intended to make it difficult to use these numbers without writing some sort of special handling code in the hopes that that will cause implementors to also use a fixed point implementation.
         */
        divisor?: string
        /**
         * Required: resource to select
         */
        resource: string
        [k: string]: unknown
    }
    [k: string]: unknown
}
/**
 * Projection that may be projected along with other supported volume types
 */
export interface IoK8SApiCoreV1VolumeProjection {
    /**
     * information about the configMap data to project
     */
    configMap?: {
        /**
         * If unspecified, each key-value pair in the Data field of the referenced ConfigMap will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the ConfigMap, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
         */
        items?: IoK8SApiCoreV1KeyToPath[]
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the ConfigMap or its keys must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * information about the downwardAPI data to project
     */
    downwardAPI?: {
        /**
         * Items is a list of DownwardAPIVolume file
         */
        items?: IoK8SApiCoreV1DownwardAPIVolumeFile[]
        [k: string]: unknown
    }
    /**
     * information about the secret data to project
     */
    secret?: {
        /**
         * If unspecified, each key-value pair in the Data field of the referenced Secret will be projected into the volume as a file whose name is the key and content is the value. If specified, the listed keys will be projected into the specified paths, and unlisted keys will not be present. If a key is specified which is not present in the Secret, the volume setup will error unless it is marked optional. Paths must be relative and may not contain the '..' path or start with '..'.
         */
        items?: IoK8SApiCoreV1KeyToPath[]
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * information about the serviceAccountToken data to project
     */
    serviceAccountToken?: {
        /**
         * Audience is the intended audience of the token. A recipient of a token must identify itself with an identifier specified in the audience of the token, and otherwise should reject the token. The audience defaults to the identifier of the apiserver.
         */
        audience?: string
        /**
         * ExpirationSeconds is the requested duration of validity of the service account token. As the token approaches expiration, the kubelet volume plugin will proactively rotate the service account token. The kubelet will start trying to rotate the token if the token is older than 80 percent of its time to live or if the token is older than 24 hours.Defaults to 1 hour and must be at least 10 minutes.
         */
        expirationSeconds?: number
        /**
         * Path is the path relative to the mount point of the file to project the token into.
         */
        path: string
        [k: string]: unknown
    }
    [k: string]: unknown
}
/**
 * CalendarEventSource describes an HTTP based EventSource
 */
export interface IoArgoprojEventsourceV1Alpha1WebhookEventSource {
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    authSecret?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * REST API endpoint
     */
    endpoint: string
    /**
     * Filter
     */
    filter?: {
        expression?: string
        [k: string]: unknown
    }
    /**
     * MaxPayloadSize is the maximum webhook payload size that the server will accept. Requests exceeding that limit will be rejected with "request too large" response. Default value: 1048576 (1MB).
     */
    maxPayloadSize?: number
    /**
     * Metadata holds the user defined metadata which will passed along the event payload.
     */
    metadata?: {
        [k: string]: string
    }
    /**
     * Method is HTTP request method that indicates the desired action to be performed for a given resource. See RFC7231 Hypertext Transfer Protocol (HTTP/1.1): Semantics and Content
     */
    method: string
    /**
     * Port on which HTTP server is listening for incoming events.
     */
    port: string
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    serverCertSecret?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * SecretKeySelector selects a key of a Secret.
     */
    serverKeySecret?: {
        /**
         * The key of the secret to select from.  Must be a valid secret key.
         */
        key: string
        /**
         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
         */
        name?: string
        /**
         * Specify whether the Secret or its key must be defined
         */
        optional?: boolean
        [k: string]: unknown
    }
    /**
     * URL is the url of the server.
     */
    url: string
    [k: string]: unknown
}
/**
 * EventSourceStatus holds the status of the event-source resource
 */
export interface IoArgoprojEventsourceV1Alpha1EventSourceStatus {
    /**
     * Conditions are the latest available observations of a resource's current state.
     */
    conditions?: IoArgoprojCommonCondition[]
    [k: string]: unknown
}
/**
 * Sensor is the definition of a sensor resource
 */
export interface IoArgoprojSensorV1Alpha1Sensor {
    /**
     * APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources
     */
    apiVersion?: "argoproj.io/v1alpha1"
    /**
     * Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds
     */
    kind?: "Sensor"
    metadata: IoK8SApimachineryPkgApisMetaV1ObjectMeta
    spec: IoArgoprojSensorV1Alpha1SensorSpec
    status?: IoArgoprojSensorV1Alpha1SensorStatus
    [k: string]: unknown
}
/**
 * SensorSpec represents desired sensor state
 */
export interface IoArgoprojSensorV1Alpha1SensorSpec {
    /**
     * Dependencies is a list of the events that this sensor is dependent on.
     */
    dependencies: IoArgoprojSensorV1Alpha1EventDependency[]
    /**
     * ErrorOnFailedRound if set to true, marks sensor state as `error` if the previous trigger round fails. Once sensor state is set to `error`, no further triggers will be processed.
     */
    errorOnFailedRound?: boolean
    /**
     * EventBusName references to a EventBus name. By default the value is "default"
     */
    eventBusName?: string
    /**
     * Replicas is the sensor deployment replicas
     */
    replicas?: number
    /**
     * RevisionHistoryLimit specifies how many old deployment revisions to retain
     */
    revisionHistoryLimit?: number
    /**
     * Template is the pod specification for the sensor
     */
    template?: {
        /**
         * If specified, the pod's scheduling constraints
         */
        affinity?: {
            /**
             * Describes node affinity scheduling rules for the pod.
             */
            nodeAffinity?: {
                /**
                 * The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node matches the corresponding matchExpressions; the node(s) with the highest sum are the most preferred.
                 */
                preferredDuringSchedulingIgnoredDuringExecution?: IoK8SApiCoreV1PreferredSchedulingTerm[]
                /**
                 * If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to an update), the system may or may not try to eventually evict the pod from its node.
                 */
                requiredDuringSchedulingIgnoredDuringExecution?: {
                    /**
                     * Required. A list of node selector terms. The terms are ORed.
                     */
                    nodeSelectorTerms: IoK8SApiCoreV1NodeSelectorTerm[]
                    [k: string]: unknown
                }
                [k: string]: unknown
            }
            /**
             * Describes pod affinity scheduling rules (e.g. co-locate this pod in the same node, zone, etc. as some other pod(s)).
             */
            podAffinity?: {
                /**
                 * The scheduler will prefer to schedule pods to nodes that satisfy the affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
                 */
                preferredDuringSchedulingIgnoredDuringExecution?: IoK8SApiCoreV1WeightedPodAffinityTerm[]
                /**
                 * If the affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
                 */
                requiredDuringSchedulingIgnoredDuringExecution?: IoK8SApiCoreV1PodAffinityTerm[]
                [k: string]: unknown
            }
            /**
             * Describes pod anti-affinity scheduling rules (e.g. avoid putting this pod in the same node, zone, etc. as some other pod(s)).
             */
            podAntiAffinity?: {
                /**
                 * The scheduler will prefer to schedule pods to nodes that satisfy the anti-affinity expressions specified by this field, but it may choose a node that violates one or more of the expressions. The node that is most preferred is the one with the greatest sum of weights, i.e. for each node that meets all of the scheduling requirements (resource request, requiredDuringScheduling anti-affinity expressions, etc.), compute a sum by iterating through the elements of this field and adding "weight" to the sum if the node has pods which matches the corresponding podAffinityTerm; the node(s) with the highest sum are the most preferred.
                 */
                preferredDuringSchedulingIgnoredDuringExecution?: IoK8SApiCoreV1WeightedPodAffinityTerm[]
                /**
                 * If the anti-affinity requirements specified by this field are not met at scheduling time, the pod will not be scheduled onto the node. If the anti-affinity requirements specified by this field cease to be met at some point during pod execution (e.g. due to a pod label update), the system may or may not try to eventually evict the pod from its node. When there are multiple elements, the lists of nodes corresponding to each podAffinityTerm are intersected, i.e. all terms must be satisfied.
                 */
                requiredDuringSchedulingIgnoredDuringExecution?: IoK8SApiCoreV1PodAffinityTerm[]
                [k: string]: unknown
            }
            [k: string]: unknown
        }
        /**
         * Container is the main container image to run in the sensor pod
         */
        container?: {
            /**
             * Arguments to the entrypoint. The docker image's CMD is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
             */
            args?: string[]
            /**
             * Entrypoint array. Not executed within a shell. The docker image's ENTRYPOINT is used if this is not provided. Variable references $(VAR_NAME) are expanded using the container's environment. If a variable cannot be resolved, the reference in the input string will be unchanged. The $(VAR_NAME) syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped references will never be expanded, regardless of whether the variable exists or not. Cannot be updated. More info: https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell
             */
            command?: string[]
            /**
             * List of environment variables to set in the container. Cannot be updated.
             */
            env?: IoK8SApiCoreV1EnvVar[]
            /**
             * List of sources to populate environment variables in the container. The keys defined within a source must be a C_IDENTIFIER. All invalid keys will be reported as an event when the container is starting. When a key exists in multiple sources, the value associated with the last source will take precedence. Values defined by an Env with a duplicate key will take precedence. Cannot be updated.
             */
            envFrom?: IoK8SApiCoreV1EnvFromSource[]
            /**
             * Docker image name. More info: https://kubernetes.io/docs/concepts/containers/images This field is optional to allow higher level config management to default or override container images in workload controllers like Deployments and StatefulSets.
             */
            image?: string
            /**
             * Image pull policy. One of Always, Never, IfNotPresent. Defaults to Always if :latest tag is specified, or IfNotPresent otherwise. Cannot be updated. More info: https://kubernetes.io/docs/concepts/containers/images#updating-images
             */
            imagePullPolicy?: string
            /**
             * Actions that the management system should take in response to container lifecycle events. Cannot be updated.
             */
            lifecycle?: {
                /**
                 * PostStart is called immediately after a container is created. If the handler fails, the container is terminated and restarted according to its restart policy. Other management of the container blocks until the hook completes. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
                 */
                postStart?: {
                    /**
                     * One and only one of the following should be specified. Exec specifies the action to take.
                     */
                    exec?: {
                        /**
                         * Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
                         */
                        command?: string[]
                        [k: string]: unknown
                    }
                    /**
                     * HTTPGet specifies the http request to perform.
                     */
                    httpGet?: {
                        /**
                         * Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
                         */
                        host?: string
                        /**
                         * Custom headers to set in the request. HTTP allows repeated headers.
                         */
                        httpHeaders?: IoK8SApiCoreV1HTTPHeader[]
                        /**
                         * Path to access on the HTTP server.
                         */
                        path?: string
                        /**
                         * Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
                         */
                        port: number | string
                        /**
                         * Scheme to use for connecting to the host. Defaults to HTTP.
                         */
                        scheme?: string
                        [k: string]: unknown
                    }
                    /**
                     * TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported
                     */
                    tcpSocket?: {
                        /**
                         * Optional: Host name to connect to, defaults to the pod IP.
                         */
                        host?: string
                        /**
                         * Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
                         */
                        port: number | string
                        [k: string]: unknown
                    }
                    [k: string]: unknown
                }
                /**
                 * PreStop is called immediately before a container is terminated due to an API request or management event such as liveness/startup probe failure, preemption, resource contention, etc. The handler is not called if the container crashes or exits. The reason for termination is passed to the handler. The Pod's termination grace period countdown begins before the PreStop hooked is executed. Regardless of the outcome of the handler, the container will eventually terminate within the Pod's termination grace period. Other management of the container blocks until the hook completes or until the termination grace period is reached. More info: https://kubernetes.io/docs/concepts/containers/container-lifecycle-hooks/#container-hooks
                 */
                preStop?: {
                    /**
                     * One and only one of the following should be specified. Exec specifies the action to take.
                     */
                    exec?: {
                        /**
                         * Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
                         */
                        command?: string[]
                        [k: string]: unknown
                    }
                    /**
                     * HTTPGet specifies the http request to perform.
                     */
                    httpGet?: {
                        /**
                         * Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
                         */
                        host?: string
                        /**
                         * Custom headers to set in the request. HTTP allows repeated headers.
                         */
                        httpHeaders?: IoK8SApiCoreV1HTTPHeader[]
                        /**
                         * Path to access on the HTTP server.
                         */
                        path?: string
                        /**
                         * Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
                         */
                        port: number | string
                        /**
                         * Scheme to use for connecting to the host. Defaults to HTTP.
                         */
                        scheme?: string
                        [k: string]: unknown
                    }
                    /**
                     * TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported
                     */
                    tcpSocket?: {
                        /**
                         * Optional: Host name to connect to, defaults to the pod IP.
                         */
                        host?: string
                        /**
                         * Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
                         */
                        port: number | string
                        [k: string]: unknown
                    }
                    [k: string]: unknown
                }
                [k: string]: unknown
            }
            /**
             * Periodic probe of container liveness. Container will be restarted if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
             */
            livenessProbe?: {
                /**
                 * One and only one of the following should be specified. Exec specifies the action to take.
                 */
                exec?: {
                    /**
                     * Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
                     */
                    command?: string[]
                    [k: string]: unknown
                }
                /**
                 * Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
                 */
                failureThreshold?: number
                /**
                 * HTTPGet specifies the http request to perform.
                 */
                httpGet?: {
                    /**
                     * Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
                     */
                    host?: string
                    /**
                     * Custom headers to set in the request. HTTP allows repeated headers.
                     */
                    httpHeaders?: IoK8SApiCoreV1HTTPHeader[]
                    /**
                     * Path to access on the HTTP server.
                     */
                    path?: string
                    /**
                     * Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
                     */
                    port: number | string
                    /**
                     * Scheme to use for connecting to the host. Defaults to HTTP.
                     */
                    scheme?: string
                    [k: string]: unknown
                }
                /**
                 * Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
                 */
                initialDelaySeconds?: number
                /**
                 * How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
                 */
                periodSeconds?: number
                /**
                 * Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
                 */
                successThreshold?: number
                /**
                 * TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported
                 */
                tcpSocket?: {
                    /**
                     * Optional: Host name to connect to, defaults to the pod IP.
                     */
                    host?: string
                    /**
                     * Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
                     */
                    port: number | string
                    [k: string]: unknown
                }
                /**
                 * Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
                 */
                timeoutSeconds?: number
                [k: string]: unknown
            }
            /**
             * Name of the container specified as a DNS_LABEL. Each container in a pod must have a unique name (DNS_LABEL). Cannot be updated.
             */
            name: string
            /**
             * List of ports to expose from the container. Exposing a port here gives the system additional information about the network connections a container uses, but is primarily informational. Not specifying a port here DOES NOT prevent that port from being exposed. Any port which is listening on the default "0.0.0.0" address inside a container will be accessible from the network. Cannot be updated.
             */
            ports?: IoK8SApiCoreV1ContainerPort[]
            /**
             * Periodic probe of container service readiness. Container will be removed from service endpoints if the probe fails. Cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
             */
            readinessProbe?: {
                /**
                 * One and only one of the following should be specified. Exec specifies the action to take.
                 */
                exec?: {
                    /**
                     * Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
                     */
                    command?: string[]
                    [k: string]: unknown
                }
                /**
                 * Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
                 */
                failureThreshold?: number
                /**
                 * HTTPGet specifies the http request to perform.
                 */
                httpGet?: {
                    /**
                     * Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
                     */
                    host?: string
                    /**
                     * Custom headers to set in the request. HTTP allows repeated headers.
                     */
                    httpHeaders?: IoK8SApiCoreV1HTTPHeader[]
                    /**
                     * Path to access on the HTTP server.
                     */
                    path?: string
                    /**
                     * Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
                     */
                    port: number | string
                    /**
                     * Scheme to use for connecting to the host. Defaults to HTTP.
                     */
                    scheme?: string
                    [k: string]: unknown
                }
                /**
                 * Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
                 */
                initialDelaySeconds?: number
                /**
                 * How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
                 */
                periodSeconds?: number
                /**
                 * Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
                 */
                successThreshold?: number
                /**
                 * TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported
                 */
                tcpSocket?: {
                    /**
                     * Optional: Host name to connect to, defaults to the pod IP.
                     */
                    host?: string
                    /**
                     * Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
                     */
                    port: number | string
                    [k: string]: unknown
                }
                /**
                 * Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
                 */
                timeoutSeconds?: number
                [k: string]: unknown
            }
            /**
             * ResourceRequirements describes the compute resource requirements.
             */
            resources?: {
                /**
                 * Limits describes the maximum amount of compute resources allowed. More info: https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/
                 */
                limits?: {
                    [k: string]: IoK8SApimachineryPkgApiResourceQuantity
                }
                /**
                 * Requests describes the minimum amount of compute resources required. If Requests is omitted for a container, it defaults to Limits if that is explicitly specified, otherwise to an implementation-defined value. More info: https://kubernetes.io/docs/concepts/configuration/manage-compute-resources-container/
                 */
                requests?: {
                    [k: string]: IoK8SApimachineryPkgApiResourceQuantity
                }
                [k: string]: unknown
            }
            /**
             * SecurityContext holds security configuration that will be applied to a container. Some fields are present in both SecurityContext and PodSecurityContext.  When both are set, the values in SecurityContext take precedence.
             */
            securityContext?: {
                /**
                 * AllowPrivilegeEscalation controls whether a process can gain more privileges than its parent process. This bool directly controls if the no_new_privs flag will be set on the container process. AllowPrivilegeEscalation is true always when the container is: 1) run as Privileged 2) has CAP_SYS_ADMIN
                 */
                allowPrivilegeEscalation?: boolean
                /**
                 * The capabilities to add/drop when running containers. Defaults to the default set of capabilities granted by the container runtime.
                 */
                capabilities?: {
                    /**
                     * Added capabilities
                     */
                    add?: string[]
                    /**
                     * Removed capabilities
                     */
                    drop?: string[]
                    [k: string]: unknown
                }
                /**
                 * Run container in privileged mode. Processes in privileged containers are essentially equivalent to root on the host. Defaults to false.
                 */
                privileged?: boolean
                /**
                 * procMount denotes the type of proc mount to use for the containers. The default is DefaultProcMount which uses the container runtime defaults for readonly paths and masked paths. This requires the ProcMountType feature flag to be enabled.
                 */
                procMount?: string
                /**
                 * Whether this container has a read-only root filesystem. Default is false.
                 */
                readOnlyRootFilesystem?: boolean
                /**
                 * The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
                 */
                runAsGroup?: number
                /**
                 * Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
                 */
                runAsNonRoot?: boolean
                /**
                 * The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
                 */
                runAsUser?: number
                /**
                 * The SELinux context to be applied to the container. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in PodSecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
                 */
                seLinuxOptions?: {
                    /**
                     * Level is SELinux level label that applies to the container.
                     */
                    level?: string
                    /**
                     * Role is a SELinux role label that applies to the container.
                     */
                    role?: string
                    /**
                     * Type is a SELinux type label that applies to the container.
                     */
                    type?: string
                    /**
                     * User is a SELinux user label that applies to the container.
                     */
                    user?: string
                    [k: string]: unknown
                }
                /**
                 * The seccomp options to use by this container. If seccomp options are provided at both the pod & container level, the container options override the pod options.
                 */
                seccompProfile?: {
                    /**
                     * localhostProfile indicates a profile defined in a file on the node should be used. The profile must be preconfigured on the node to work. Must be a descending path, relative to the kubelet's configured seccomp profile location. Must only be set if type is "Localhost".
                     */
                    localhostProfile?: string
                    /**
                     * type indicates which kind of seccomp profile will be applied. Valid options are:
                     *
                     * Localhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.
                     */
                    type: string
                    [k: string]: unknown
                }
                /**
                 * The Windows specific settings applied to all containers. If unspecified, the options from the PodSecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
                 */
                windowsOptions?: {
                    /**
                     * GMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field.
                     */
                    gmsaCredentialSpec?: string
                    /**
                     * GMSACredentialSpecName is the name of the GMSA credential spec to use.
                     */
                    gmsaCredentialSpecName?: string
                    /**
                     * The UserName in Windows to run the entrypoint of the container process. Defaults to the user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
                     */
                    runAsUserName?: string
                    [k: string]: unknown
                }
                [k: string]: unknown
            }
            /**
             * StartupProbe indicates that the Pod has successfully initialized. If specified, no other probes are executed until this completes successfully. If this probe fails, the Pod will be restarted, just as if the livenessProbe failed. This can be used to provide different probe parameters at the beginning of a Pod's lifecycle, when it might take a long time to load data or warm a cache, than during steady-state operation. This cannot be updated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
             */
            startupProbe?: {
                /**
                 * One and only one of the following should be specified. Exec specifies the action to take.
                 */
                exec?: {
                    /**
                     * Command is the command line to execute inside the container, the working directory for the command  is root ('/') in the container's filesystem. The command is simply exec'd, it is not run inside a shell, so traditional shell instructions ('|', etc) won't work. To use a shell, you need to explicitly call out to that shell. Exit status of 0 is treated as live/healthy and non-zero is unhealthy.
                     */
                    command?: string[]
                    [k: string]: unknown
                }
                /**
                 * Minimum consecutive failures for the probe to be considered failed after having succeeded. Defaults to 3. Minimum value is 1.
                 */
                failureThreshold?: number
                /**
                 * HTTPGet specifies the http request to perform.
                 */
                httpGet?: {
                    /**
                     * Host name to connect to, defaults to the pod IP. You probably want to set "Host" in httpHeaders instead.
                     */
                    host?: string
                    /**
                     * Custom headers to set in the request. HTTP allows repeated headers.
                     */
                    httpHeaders?: IoK8SApiCoreV1HTTPHeader[]
                    /**
                     * Path to access on the HTTP server.
                     */
                    path?: string
                    /**
                     * Name or number of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
                     */
                    port: number | string
                    /**
                     * Scheme to use for connecting to the host. Defaults to HTTP.
                     */
                    scheme?: string
                    [k: string]: unknown
                }
                /**
                 * Number of seconds after the container has started before liveness probes are initiated. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
                 */
                initialDelaySeconds?: number
                /**
                 * How often (in seconds) to perform the probe. Default to 10 seconds. Minimum value is 1.
                 */
                periodSeconds?: number
                /**
                 * Minimum consecutive successes for the probe to be considered successful after having failed. Defaults to 1. Must be 1 for liveness and startup. Minimum value is 1.
                 */
                successThreshold?: number
                /**
                 * TCPSocket specifies an action involving a TCP port. TCP hooks not yet supported
                 */
                tcpSocket?: {
                    /**
                     * Optional: Host name to connect to, defaults to the pod IP.
                     */
                    host?: string
                    /**
                     * Number or name of the port to access on the container. Number must be in the range 1 to 65535. Name must be an IANA_SVC_NAME.
                     */
                    port: number | string
                    [k: string]: unknown
                }
                /**
                 * Number of seconds after which the probe times out. Defaults to 1 second. Minimum value is 1. More info: https://kubernetes.io/docs/concepts/workloads/pods/pod-lifecycle#container-probes
                 */
                timeoutSeconds?: number
                [k: string]: unknown
            }
            /**
             * Whether this container should allocate a buffer for stdin in the container runtime. If this is not set, reads from stdin in the container will always result in EOF. Default is false.
             */
            stdin?: boolean
            /**
             * Whether the container runtime should close the stdin channel after it has been opened by a single attach. When stdin is true the stdin stream will remain open across multiple attach sessions. If stdinOnce is set to true, stdin is opened on container start, is empty until the first client attaches to stdin, and then remains open and accepts data until the client disconnects, at which time stdin is closed and remains closed until the container is restarted. If this flag is false, a container processes that reads from stdin will never receive an EOF. Default is false
             */
            stdinOnce?: boolean
            /**
             * Optional: Path at which the file to which the container's termination message will be written is mounted into the container's filesystem. Message written is intended to be brief final status, such as an assertion failure message. Will be truncated by the node if greater than 4096 bytes. The total message length across all containers will be limited to 12kb. Defaults to /dev/termination-log. Cannot be updated.
             */
            terminationMessagePath?: string
            /**
             * Indicate how the termination message should be populated. File will use the contents of terminationMessagePath to populate the container status message on both success and failure. FallbackToLogsOnError will use the last chunk of container log output if the termination message file is empty and the container exited with an error. The log output is limited to 2048 bytes or 80 lines, whichever is smaller. Defaults to File. Cannot be updated.
             */
            terminationMessagePolicy?: string
            /**
             * Whether this container should allocate a TTY for itself, also requires 'stdin' to be true. Default is false.
             */
            tty?: boolean
            /**
             * volumeDevices is the list of block devices to be used by the container.
             */
            volumeDevices?: IoK8SApiCoreV1VolumeDevice[]
            /**
             * Pod volumes to mount into the container's filesystem. Cannot be updated.
             */
            volumeMounts?: IoK8SApiCoreV1VolumeMount[]
            /**
             * Container's working directory. If not specified, the container runtime's default will be used, which might be configured in the container image. Cannot be updated.
             */
            workingDir?: string
            [k: string]: unknown
        }
        /**
         * ImagePullSecrets is an optional list of references to secrets in the same namespace to use for pulling any of the images used by this PodSpec. If specified, these secrets will be passed to individual puller implementations for them to use. For example, in the case of docker, only DockerConfig type secrets are honored. More info: https://kubernetes.io/docs/concepts/containers/images#specifying-imagepullsecrets-on-a-pod
         */
        imagePullSecrets?: IoK8SApiCoreV1LocalObjectReference[]
        /**
         * Metadata sets the pods's metadata, i.e. annotations and labels
         */
        metadata?: {
            annotations?: {
                [k: string]: string
            }
            labels?: {
                [k: string]: string
            }
            [k: string]: unknown
        }
        /**
         * NodeSelector is a selector which must be true for the pod to fit on a node. Selector which must match a node's labels for the pod to be scheduled on that node. More info: https://kubernetes.io/docs/concepts/configuration/assign-pod-node/
         */
        nodeSelector?: {
            [k: string]: string
        }
        /**
         * The priority value. Various system components use this field to find the priority of the EventSource pod. When Priority Admission Controller is enabled, it prevents users from setting this field. The admission controller populates this field from PriorityClassName. The higher the value, the higher the priority. More info: https://kubernetes.io/docs/concepts/configuration/pod-priority-preemption/
         */
        priority?: number
        /**
         * If specified, indicates the EventSource pod's priority. "system-node-critical" and "system-cluster-critical" are two special keywords which indicate the highest priorities with the former being the highest priority. Any other name must be defined by creating a PriorityClass object with that name. If not specified, the pod priority will be default or zero if there is no default. More info: https://kubernetes.io/docs/concepts/configuration/pod-priority-preemption/
         */
        priorityClassName?: string
        /**
         * SecurityContext holds pod-level security attributes and common container settings. Optional: Defaults to empty.  See type description for default values of each field.
         */
        securityContext?: {
            /**
             * A special supplemental group that applies to all containers in a pod. Some volume types allow the Kubelet to change the ownership of that volume to be owned by the pod:
             *
             * 1. The owning GID will be the FSGroup 2. The setgid bit is set (new files created in the volume will be owned by FSGroup) 3. The permission bits are OR'd with rw-rw----
             *
             * If unset, the Kubelet will not modify the ownership and permissions of any volume.
             */
            fsGroup?: number
            /**
             * fsGroupChangePolicy defines behavior of changing ownership and permission of the volume before being exposed inside Pod. This field will only apply to volume types which support fsGroup based ownership(and permissions). It will have no effect on ephemeral volume types such as: secret, configmaps and emptydir. Valid values are "OnRootMismatch" and "Always". If not specified, "Always" is used.
             */
            fsGroupChangePolicy?: string
            /**
             * The GID to run the entrypoint of the container process. Uses runtime default if unset. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container.
             */
            runAsGroup?: number
            /**
             * Indicates that the container must run as a non-root user. If true, the Kubelet will validate the image at runtime to ensure that it does not run as UID 0 (root) and fail to start the container if it does. If unset or false, no such validation will be performed. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
             */
            runAsNonRoot?: boolean
            /**
             * The UID to run the entrypoint of the container process. Defaults to user specified in image metadata if unspecified. May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container.
             */
            runAsUser?: number
            /**
             * The SELinux context to be applied to all containers. If unspecified, the container runtime will allocate a random SELinux context for each container.  May also be set in SecurityContext.  If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence for that container.
             */
            seLinuxOptions?: {
                /**
                 * Level is SELinux level label that applies to the container.
                 */
                level?: string
                /**
                 * Role is a SELinux role label that applies to the container.
                 */
                role?: string
                /**
                 * Type is a SELinux type label that applies to the container.
                 */
                type?: string
                /**
                 * User is a SELinux user label that applies to the container.
                 */
                user?: string
                [k: string]: unknown
            }
            /**
             * The seccomp options to use by the containers in this pod.
             */
            seccompProfile?: {
                /**
                 * localhostProfile indicates a profile defined in a file on the node should be used. The profile must be preconfigured on the node to work. Must be a descending path, relative to the kubelet's configured seccomp profile location. Must only be set if type is "Localhost".
                 */
                localhostProfile?: string
                /**
                 * type indicates which kind of seccomp profile will be applied. Valid options are:
                 *
                 * Localhost - a profile defined in a file on the node should be used. RuntimeDefault - the container runtime default profile should be used. Unconfined - no profile should be applied.
                 */
                type: string
                [k: string]: unknown
            }
            /**
             * A list of groups applied to the first process run in each container, in addition to the container's primary GID.  If unspecified, no groups will be added to any container.
             */
            supplementalGroups?: number[]
            /**
             * Sysctls hold a list of namespaced sysctls used for the pod. Pods with unsupported sysctls (by the container runtime) might fail to launch.
             */
            sysctls?: IoK8SApiCoreV1Sysctl[]
            /**
             * The Windows specific settings applied to all containers. If unspecified, the options within a container's SecurityContext will be used. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
             */
            windowsOptions?: {
                /**
                 * GMSACredentialSpec is where the GMSA admission webhook (https://github.com/kubernetes-sigs/windows-gmsa) inlines the contents of the GMSA credential spec named by the GMSACredentialSpecName field.
                 */
                gmsaCredentialSpec?: string
                /**
                 * GMSACredentialSpecName is the name of the GMSA credential spec to use.
                 */
                gmsaCredentialSpecName?: string
                /**
                 * The UserName in Windows to run the entrypoint of the container process. Defaults to the user specified in image metadata if unspecified. May also be set in PodSecurityContext. If set in both SecurityContext and PodSecurityContext, the value specified in SecurityContext takes precedence.
                 */
                runAsUserName?: string
                [k: string]: unknown
            }
            [k: string]: unknown
        }
        /**
         * ServiceAccountName is the name of the ServiceAccount to use to run sensor pod. More info: https://kubernetes.io/docs/tasks/configure-pod-container/configure-service-account/
         */
        serviceAccountName?: string
        /**
         * If specified, the pod's tolerations.
         */
        tolerations?: IoK8SApiCoreV1Toleration[]
        /**
         * Volumes is a list of volumes that can be mounted by containers in a workflow.
         */
        volumes?: IoK8SApiCoreV1Volume[]
        [k: string]: unknown
    }
    /**
     * Triggers is a list of the things that this sensor evokes. These are the outputs from this sensor.
     */
    triggers: IoArgoprojSensorV1Alpha1Trigger[]
    [k: string]: unknown
}
/**
 * EventDependency describes a dependency
 */
export interface IoArgoprojSensorV1Alpha1EventDependency {
    /**
     * EventName is the name of the event
     */
    eventName: string
    /**
     * EventSourceName is the name of EventSource that Sensor depends on
     */
    eventSourceName: string
    /**
     * Filters and rules governing toleration of success and constraints on the context and data of an event
     */
    filters?: {
        /**
         * EventContext holds the context of the cloudevent received from an event source.
         */
        context?: {
            /**
             * DataContentType - A MIME (RFC2046) string describing the media type of `data`.
             */
            datacontenttype: string
            /**
             * ID of the event; must be non-empty and unique within the scope of the producer.
             */
            id: string
            /**
             * Source - A URI describing the event producer.
             */
            source: string
            /**
             * SpecVersion - The version of the CloudEvents specification used by the event.
             */
            specversion: string
            /**
             * Subject - The subject of the event in the context of the event producer
             */
            subject: string
            /**
             * Time - A Timestamp when the event happened.
             */
            time: string
            /**
             * Type - The type of the occurrence which has happened.
             */
            type: string
            [k: string]: unknown
        }
        /**
         * Data filter constraints with escalation
         */
        data?: IoArgoprojSensorV1Alpha1DataFilter[]
        /**
         * DataLogicalOperator defines how multiple Data filters (if defined) are evaluated together. Available values: and (&&), or (||) Is optional and if left blank treated as and (&&).
         */
        dataLogicalOperator?: string
        /**
         * ExprLogicalOperator defines how multiple Exprs filters (if defined) are evaluated together. Available values: and (&&), or (||) Is optional and if left blank treated as and (&&).
         */
        exprLogicalOperator?: string
        /**
         * Exprs contains the list of expressions evaluated against the event payload.
         */
        exprs?: IoArgoprojSensorV1Alpha1ExprFilter[]
        /**
         * Script refers to a Lua script evaluated to determine the validity of an event.
         */
        script?: string
        /**
         * Time filter on the event with escalation
         */
        time?: {
            /**
             * Start is the beginning of a time window in UTC. Before this time, events for this dependency are ignored. Format is hh:mm:ss.
             */
            start: string
            /**
             * Stop is the end of a time window in UTC. After or equal to this time, events for this dependency are ignored and Format is hh:mm:ss. If it is smaller than Start, it is treated as next day of Start (e.g.: 22:00:00-01:00:00 means 22:00:00-25:00:00).
             */
            stop: string
            [k: string]: unknown
        }
        [k: string]: unknown
    }
    /**
     * FiltersLogicalOperator defines how different filters are evaluated together. Available values: and (&&), or (||) Is optional and if left blank treated as and (&&).
     */
    filtersLogicalOperator?: string
    /**
     * Name is a unique name of this dependency
     */
    name: string
    /**
     * Transform transforms the event data
     */
    transform?: {
        /**
         * JQ holds the jq command applied for transformation
         */
        jq?: string
        /**
         * Script refers to a Lua script used to transform the event
         */
        script?: string
        [k: string]: unknown
    }
    [k: string]: unknown
}
/**
 * DataFilter describes constraints and filters for event data Regular Expressions are purposefully not a feature as they are overkill for our uses here See Rob Pike's Post: https://commandcenter.blogspot.com/2011/08/regular-expressions-in-lexing-and.html
 */
export interface IoArgoprojSensorV1Alpha1DataFilter {
    /**
     * Comparator compares the event data with a user given value. Can be ">=", ">", "=", "!=", "<", or "<=". Is optional, and if left blank treated as equality "=".
     */
    comparator?: string
    /**
     * Path is the JSONPath of the event's (JSON decoded) data key Path is a series of keys separated by a dot. A key may contain wildcard characters '*' and '?'. To access an array value use the index as the key. The dot and wildcard characters can be escaped with '\'. See https://github.com/tidwall/gjson#path-syntax for more information on how to use this.
     */
    path: string
    /**
     * Template is a go-template for extracting a string from the event's data. A Template is evaluated with provided path, type and value. The templating follows the standard go-template syntax as well as sprig's extra functions. See https://pkg.go.dev/text/template and https://masterminds.github.io/sprig/
     */
    template?: string
    /**
     * Type contains the JSON type of the data
     */
    type: string
    /**
     * Value is the allowed string values for this key Booleans are passed using strconv.ParseBool() Numbers are parsed using as float64 using strconv.ParseFloat() Strings are taken as is Nils this value is ignored
     */
    value: string[]
    [k: string]: unknown
}
export interface IoArgoprojSensorV1Alpha1ExprFilter {
    /**
     * Expr refers to the expression that determines the outcome of the filter.
     */
    expr: string
    /**
     * Fields refers to set of keys that refer to the paths within event payload.
     */
    fields: IoArgoprojSensorV1Alpha1PayloadField[]
    [k: string]: unknown
}
/**
 * PayloadField binds a value at path within the event payload against a name.
 */
export interface IoArgoprojSensorV1Alpha1PayloadField {
    /**
     * Name acts as key that holds the value at the path.
     */
    name: string
    /**
     * Path is the JSONPath of the event's (JSON decoded) data key Path is a series of keys separated by a dot. A key may contain wildcard characters '*' and '?'. To access an array value use the index as the key. The dot and wildcard characters can be escaped with '\'. See https://github.com/tidwall/gjson#path-syntax for more information on how to use this.
     */
    path: string
    [k: string]: unknown
}
/**
 * Trigger is an action taken, output produced, an event created, a message sent
 */
export interface IoArgoprojSensorV1Alpha1Trigger {
    /**
     * Parameters is the list of parameters applied to the trigger template definition
     */
    parameters?: IoArgoprojSensorV1Alpha1TriggerParameter[]
    /**
     * Policy to configure backoff and execution criteria for the trigger
     */
    policy?: {
        /**
         * K8SResourcePolicy refers to the policy used to check the state of K8s based triggers using using labels
         */
        k8s?: {
            /**
             * Backoff before checking resource state
             */
            backoff: {
                /**
                 * The initial duration in nanoseconds or strings like "1s", "3m"
                 */
                duration?: number | string
                /**
                 * Duration is multiplied by factor each iteration
                 */
                factor?: number
                /**
                 * The amount of jitter applied each iteration
                 */
                jitter?: number
                /**
                 * Exit with error after this many steps
                 */
                steps?: number
                [k: string]: unknown
            }
            /**
             * ErrorOnBackoffTimeout determines whether sensor should transition to error state if the trigger policy is unable to determine the state of the resource
             */
            errorOnBackoffTimeout: boolean
            /**
             * Labels required to identify whether a resource is in success state
             */
            labels?: {
                [k: string]: string
            }
            [k: string]: unknown
        }
        /**
         * Status refers to the policy used to check the state of the trigger using response status
         */
        status?: {
            allow: number[]
            [k: string]: unknown
        }
        [k: string]: unknown
    }
    /**
     * Rate limit, default unit is Second
     */
    rateLimit?: {
        requestsPerUnit?: number
        /**
         * Defaults to Second
         */
        unit?: string
        [k: string]: unknown
    }
    /**
     * Retry strategy, defaults to no retry
     */
    retryStrategy?: {
        /**
         * The initial duration in nanoseconds or strings like "1s", "3m"
         */
        duration?: number | string
        /**
         * Duration is multiplied by factor each iteration
         */
        factor?: number
        /**
         * The amount of jitter applied each iteration
         */
        jitter?: number
        /**
         * Exit with error after this many steps
         */
        steps?: number
        [k: string]: unknown
    }
    /**
     * Template describes the trigger specification.
     */
    template?: {
        /**
         * ArgoWorkflow refers to the trigger that can perform various operations on an Argo workflow.
         */
        argoWorkflow?: {
            /**
             * Args is the list of arguments to pass to the argo CLI
             */
            args?: string[]
            /**
             * Operation refers to the type of operation performed on the argo workflow resource. Default value is Submit.
             */
            operation?: string
            /**
             * Parameters is the list of parameters to pass to resolved Argo Workflow object
             */
            parameters?: IoArgoprojSensorV1Alpha1TriggerParameter[]
            /**
             * Source of the K8s resource file(s)
             */
            source?: {
                /**
                 * Selects a key from a ConfigMap.
                 */
                configmap?: {
                    /**
                     * The key to select.
                     */
                    key: string
                    /**
                     * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                     */
                    name?: string
                    /**
                     * Specify whether the ConfigMap or its key must be defined
                     */
                    optional?: boolean
                    [k: string]: unknown
                }
                /**
                 * File artifact is artifact stored in a file
                 */
                file?: {
                    path?: string
                    [k: string]: unknown
                }
                /**
                 * Git repository hosting the artifact
                 */
                git?: {
                    /**
                     * Branch to use to pull trigger resource
                     */
                    branch?: string
                    /**
                     * Directory to clone the repository. We clone complete directory because GitArtifact is not limited to any specific Git service providers. Hence we don't use any specific git provider client.
                     */
                    cloneDirectory: string
                    /**
                     * Creds contain reference to git username and password
                     */
                    creds?: {
                        password?: IoK8SApiCoreV1SecretKeySelector
                        username?: IoK8SApiCoreV1SecretKeySelector
                        [k: string]: unknown
                    }
                    /**
                     * Path to file that contains trigger resource definition
                     */
                    filePath: string
                    /**
                     * Whether to ignore host key
                     */
                    insecureIgnoreHostKey?: boolean
                    /**
                     * Ref to use to pull trigger resource. Will result in a shallow clone and fetch.
                     */
                    ref?: string
                    /**
                     * Remote to manage set of tracked repositories. Defaults to "origin". Refer https://git-scm.com/docs/git-remote
                     */
                    remote?: {
                        /**
                         * Name of the remote to fetch from.
                         */
                        name: string
                        /**
                         * URLs the URLs of a remote repository. It must be non-empty. Fetch will always use the first URL, while push will use all of them.
                         */
                        urls: string[]
                        [k: string]: unknown
                    }
                    /**
                     * SecretKeySelector selects a key of a Secret.
                     */
                    sshKeySecret?: {
                        /**
                         * The key of the secret to select from.  Must be a valid secret key.
                         */
                        key: string
                        /**
                         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                         */
                        name?: string
                        /**
                         * Specify whether the Secret or its key must be defined
                         */
                        optional?: boolean
                        [k: string]: unknown
                    }
                    /**
                     * Tag to use to pull trigger resource
                     */
                    tag?: string
                    /**
                     * Git URL
                     */
                    url: string
                    [k: string]: unknown
                }
                /**
                 * Inline artifact is embedded in sensor spec as a string
                 */
                inline?: string
                /**
                 * Resource is generic template for K8s resource
                 */
                resource?: {
                    [k: string]: unknown
                }
                /**
                 * S3Artifact contains information about an S3 connection and bucket
                 */
                s3?: {
                    accessKey: IoK8SApiCoreV1SecretKeySelector
                    bucket: IoArgoprojCommonS3Bucket
                    endpoint: string
                    events?: string[]
                    filter?: IoArgoprojCommonS3Filter
                    insecure?: boolean
                    metadata?: {
                        [k: string]: string
                    }
                    region?: string
                    secretKey: IoK8SApiCoreV1SecretKeySelector
                    [k: string]: unknown
                }
                /**
                 * URL to fetch the artifact from
                 */
                url?: {
                    /**
                     * Path is the complete URL
                     */
                    path: string
                    /**
                     * VerifyCert decides whether the connection is secure or not
                     */
                    verifyCert?: boolean
                    [k: string]: unknown
                }
                [k: string]: unknown
            }
            [k: string]: unknown
        }
        /**
         * AWSLambda refers to the trigger designed to invoke AWS Lambda function with with on-the-fly constructable payload.
         */
        awsLambda?: {
            /**
             * SecretKeySelector selects a key of a Secret.
             */
            accessKey?: {
                /**
                 * The key of the secret to select from.  Must be a valid secret key.
                 */
                key: string
                /**
                 * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                 */
                name?: string
                /**
                 * Specify whether the Secret or its key must be defined
                 */
                optional?: boolean
                [k: string]: unknown
            }
            /**
             * FunctionName refers to the name of the function to invoke.
             */
            functionName: string
            /**
             * Choose from the following options.
             *
             *    * RequestResponse (default) - Invoke the function synchronously. Keep
             *    the connection open until the function returns a response or times out.
             *    The API response includes the function response and additional data.
             *
             *    * Event - Invoke the function asynchronously. Send events that fail multiple
             *    times to the function's dead-letter queue (if it's configured). The API
             *    response only includes a status code.
             *
             *    * DryRun - Validate parameter values and verify that the user or role
             *    has permission to invoke the function.
             */
            invocationType?: string
            /**
             * Parameters is the list of key-value extracted from event's payload that are applied to the trigger resource.
             */
            parameters?: IoArgoprojSensorV1Alpha1TriggerParameter[]
            /**
             * Payload is the list of key-value extracted from an event payload to construct the request payload.
             */
            payload: IoArgoprojSensorV1Alpha1TriggerParameter[]
            /**
             * Region is AWS region
             */
            region: string
            /**
             * RoleARN is the Amazon Resource Name (ARN) of the role to assume.
             */
            roleARN?: string
            /**
             * SecretKeySelector selects a key of a Secret.
             */
            secretKey?: {
                /**
                 * The key of the secret to select from.  Must be a valid secret key.
                 */
                key: string
                /**
                 * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                 */
                name?: string
                /**
                 * Specify whether the Secret or its key must be defined
                 */
                optional?: boolean
                [k: string]: unknown
            }
            [k: string]: unknown
        }
        /**
         * AzureEventHubs refers to the trigger send an event to an Azure Event Hub.
         */
        azureEventHubs?: {
            /**
             * FQDN refers to the namespace dns of Azure Event Hubs to be used i.e. <namespace>.servicebus.windows.net
             */
            fqdn: string
            /**
             * HubName refers to the Azure Event Hub to send events to
             */
            hubName: string
            /**
             * Parameters is the list of key-value extracted from event's payload that are applied to the trigger resource.
             */
            parameters?: IoArgoprojSensorV1Alpha1TriggerParameter[]
            /**
             * Payload is the list of key-value extracted from an event payload to construct the request payload.
             */
            payload: IoArgoprojSensorV1Alpha1TriggerParameter[]
            /**
             * SecretKeySelector selects a key of a Secret.
             */
            sharedAccessKey?: {
                /**
                 * The key of the secret to select from.  Must be a valid secret key.
                 */
                key: string
                /**
                 * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                 */
                name?: string
                /**
                 * Specify whether the Secret or its key must be defined
                 */
                optional?: boolean
                [k: string]: unknown
            }
            /**
             * SecretKeySelector selects a key of a Secret.
             */
            sharedAccessKeyName: {
                /**
                 * The key of the secret to select from.  Must be a valid secret key.
                 */
                key: string
                /**
                 * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                 */
                name?: string
                /**
                 * Specify whether the Secret or its key must be defined
                 */
                optional?: boolean
                [k: string]: unknown
            }
            [k: string]: unknown
        }
        /**
         * Conditions is the conditions to execute the trigger. For example: "(dep01 || dep02) && dep04"
         */
        conditions?: string
        /**
         * Criteria to reset the conditons
         */
        conditionsReset?: IoArgoprojSensorV1Alpha1ConditionsResetCriteria[]
        /**
         * CustomTrigger refers to the trigger designed to connect to a gRPC trigger server and execute a custom trigger.
         */
        custom?: {
            /**
             * SecretKeySelector selects a key of a Secret.
             */
            certSecret?: {
                /**
                 * The key of the secret to select from.  Must be a valid secret key.
                 */
                key: string
                /**
                 * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                 */
                name?: string
                /**
                 * Specify whether the Secret or its key must be defined
                 */
                optional?: boolean
                [k: string]: unknown
            }
            /**
             * Parameters is the list of parameters that is applied to resolved custom trigger trigger object.
             */
            parameters?: IoArgoprojSensorV1Alpha1TriggerParameter[]
            /**
             * Payload is the list of key-value extracted from an event payload to construct the request payload.
             */
            payload: IoArgoprojSensorV1Alpha1TriggerParameter[]
            /**
             * Secure refers to type of the connection between sensor to custom trigger gRPC
             */
            secure: boolean
            /**
             * ServerNameOverride for the secure connection between sensor and custom trigger gRPC server.
             */
            serverNameOverride?: string
            /**
             * ServerURL is the url of the gRPC server that executes custom trigger
             */
            serverURL: string
            /**
             * Spec is the custom trigger resource specification that custom trigger gRPC server knows how to interpret.
             */
            spec: {
                [k: string]: string
            }
            [k: string]: unknown
        }
        /**
         * HTTP refers to the trigger designed to dispatch a HTTP request with on-the-fly constructable payload.
         */
        http?: {
            /**
             * BasicAuth configuration for the http request.
             */
            basicAuth?: {
                /**
                 * Password refers to the Kubernetes secret that holds the password required for basic auth.
                 */
                password?: {
                    /**
                     * The key of the secret to select from.  Must be a valid secret key.
                     */
                    key: string
                    /**
                     * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                     */
                    name?: string
                    /**
                     * Specify whether the Secret or its key must be defined
                     */
                    optional?: boolean
                    [k: string]: unknown
                }
                /**
                 * Username refers to the Kubernetes secret that holds the username required for basic auth.
                 */
                username?: {
                    /**
                     * The key of the secret to select from.  Must be a valid secret key.
                     */
                    key: string
                    /**
                     * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                     */
                    name?: string
                    /**
                     * Specify whether the Secret or its key must be defined
                     */
                    optional?: boolean
                    [k: string]: unknown
                }
                [k: string]: unknown
            }
            /**
             * Headers for the HTTP request.
             */
            headers?: {
                [k: string]: string
            }
            /**
             * Method refers to the type of the HTTP request. Refer https://golang.org/src/net/http/method.go for more info. Default value is POST.
             */
            method?: string
            /**
             * Parameters is the list of key-value extracted from event's payload that are applied to the HTTP trigger resource.
             */
            parameters?: IoArgoprojSensorV1Alpha1TriggerParameter[]
            payload: IoArgoprojSensorV1Alpha1TriggerParameter[]
            /**
             * Secure Headers stored in Kubernetes Secrets for the HTTP requests.
             */
            secureHeaders?: IoArgoprojCommonSecureHeader[]
            /**
             * Timeout refers to the HTTP request timeout in seconds. Default value is 60 seconds.
             */
            timeout?: number
            /**
             * TLS configuration for the HTTP client.
             */
            tls?: {
                /**
                 * SecretKeySelector selects a key of a Secret.
                 */
                caCertSecret?: {
                    /**
                     * The key of the secret to select from.  Must be a valid secret key.
                     */
                    key: string
                    /**
                     * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                     */
                    name?: string
                    /**
                     * Specify whether the Secret or its key must be defined
                     */
                    optional?: boolean
                    [k: string]: unknown
                }
                /**
                 * SecretKeySelector selects a key of a Secret.
                 */
                clientCertSecret?: {
                    /**
                     * The key of the secret to select from.  Must be a valid secret key.
                     */
                    key: string
                    /**
                     * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                     */
                    name?: string
                    /**
                     * Specify whether the Secret or its key must be defined
                     */
                    optional?: boolean
                    [k: string]: unknown
                }
                /**
                 * SecretKeySelector selects a key of a Secret.
                 */
                clientKeySecret?: {
                    /**
                     * The key of the secret to select from.  Must be a valid secret key.
                     */
                    key: string
                    /**
                     * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                     */
                    name?: string
                    /**
                     * Specify whether the Secret or its key must be defined
                     */
                    optional?: boolean
                    [k: string]: unknown
                }
                /**
                 * If true, skips creation of TLSConfig with certs and creates an empty TLSConfig. (Defaults to false)
                 */
                insecureSkipVerify?: boolean
                [k: string]: unknown
            }
            /**
             * URL refers to the URL to send HTTP request to.
             */
            url: string
            [k: string]: unknown
        }
        /**
         * StandardK8STrigger refers to the trigger designed to create or update a generic Kubernetes resource.
         */
        k8s?: {
            /**
             * LiveObject specifies whether the resource should be directly fetched from K8s instead of being marshaled from the resource artifact. If set to true, the resource artifact must contain the information required to uniquely identify the resource in the cluster, that is, you must specify "apiVersion", "kind" as well as "name" and "namespace" meta data. Only valid for operation type `update`
             */
            liveObject?: boolean
            /**
             * Operation refers to the type of operation performed on the k8s resource. Default value is Create.
             */
            operation?: string
            /**
             * Parameters is the list of parameters that is applied to resolved K8s trigger object.
             */
            parameters?: IoArgoprojSensorV1Alpha1TriggerParameter[]
            /**
             * PatchStrategy controls the K8s object patching strategy when the trigger operation is specified as patch. possible values: "application/json-patch+json" "application/merge-patch+json" "application/strategic-merge-patch+json" "application/apply-patch+yaml". Defaults to "application/merge-patch+json"
             */
            patchStrategy?: string
            /**
             * Source of the K8s resource file(s)
             */
            source?: {
                /**
                 * Selects a key from a ConfigMap.
                 */
                configmap?: {
                    /**
                     * The key to select.
                     */
                    key: string
                    /**
                     * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                     */
                    name?: string
                    /**
                     * Specify whether the ConfigMap or its key must be defined
                     */
                    optional?: boolean
                    [k: string]: unknown
                }
                /**
                 * File artifact is artifact stored in a file
                 */
                file?: {
                    path?: string
                    [k: string]: unknown
                }
                /**
                 * Git repository hosting the artifact
                 */
                git?: {
                    /**
                     * Branch to use to pull trigger resource
                     */
                    branch?: string
                    /**
                     * Directory to clone the repository. We clone complete directory because GitArtifact is not limited to any specific Git service providers. Hence we don't use any specific git provider client.
                     */
                    cloneDirectory: string
                    /**
                     * Creds contain reference to git username and password
                     */
                    creds?: {
                        password?: IoK8SApiCoreV1SecretKeySelector
                        username?: IoK8SApiCoreV1SecretKeySelector
                        [k: string]: unknown
                    }
                    /**
                     * Path to file that contains trigger resource definition
                     */
                    filePath: string
                    /**
                     * Whether to ignore host key
                     */
                    insecureIgnoreHostKey?: boolean
                    /**
                     * Ref to use to pull trigger resource. Will result in a shallow clone and fetch.
                     */
                    ref?: string
                    /**
                     * Remote to manage set of tracked repositories. Defaults to "origin". Refer https://git-scm.com/docs/git-remote
                     */
                    remote?: {
                        /**
                         * Name of the remote to fetch from.
                         */
                        name: string
                        /**
                         * URLs the URLs of a remote repository. It must be non-empty. Fetch will always use the first URL, while push will use all of them.
                         */
                        urls: string[]
                        [k: string]: unknown
                    }
                    /**
                     * SecretKeySelector selects a key of a Secret.
                     */
                    sshKeySecret?: {
                        /**
                         * The key of the secret to select from.  Must be a valid secret key.
                         */
                        key: string
                        /**
                         * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                         */
                        name?: string
                        /**
                         * Specify whether the Secret or its key must be defined
                         */
                        optional?: boolean
                        [k: string]: unknown
                    }
                    /**
                     * Tag to use to pull trigger resource
                     */
                    tag?: string
                    /**
                     * Git URL
                     */
                    url: string
                    [k: string]: unknown
                }
                /**
                 * Inline artifact is embedded in sensor spec as a string
                 */
                inline?: string
                /**
                 * Resource is generic template for K8s resource
                 */
                resource?: {
                    [k: string]: unknown
                }
                /**
                 * S3Artifact contains information about an S3 connection and bucket
                 */
                s3?: {
                    accessKey: IoK8SApiCoreV1SecretKeySelector
                    bucket: IoArgoprojCommonS3Bucket
                    endpoint: string
                    events?: string[]
                    filter?: IoArgoprojCommonS3Filter
                    insecure?: boolean
                    metadata?: {
                        [k: string]: string
                    }
                    region?: string
                    secretKey: IoK8SApiCoreV1SecretKeySelector
                    [k: string]: unknown
                }
                /**
                 * URL to fetch the artifact from
                 */
                url?: {
                    /**
                     * Path is the complete URL
                     */
                    path: string
                    /**
                     * VerifyCert decides whether the connection is secure or not
                     */
                    verifyCert?: boolean
                    [k: string]: unknown
                }
                [k: string]: unknown
            }
            [k: string]: unknown
        }
        /**
         * Kafka refers to the trigger designed to place messages on Kafka topic.
         */
        kafka?: {
            /**
             * Compress determines whether to compress message or not. Defaults to false. If set to true, compresses message using snappy compression.
             */
            compress?: boolean
            /**
             * FlushFrequency refers to the frequency in milliseconds to flush batches. Defaults to 500 milliseconds.
             */
            flushFrequency?: number
            /**
             * Parameters is the list of parameters that is applied to resolved Kafka trigger object.
             */
            parameters?: IoArgoprojSensorV1Alpha1TriggerParameter[]
            /**
             * Partition to write data to.
             */
            partition: number
            /**
             * The partitioning key for the messages put on the Kafka topic. Defaults to broker url.
             */
            partitioningKey?: string
            /**
             * Payload is the list of key-value extracted from an event payload to construct the request payload.
             */
            payload: IoArgoprojSensorV1Alpha1TriggerParameter[]
            /**
             * RequiredAcks used in producer to tell the broker how many replica acknowledgements Defaults to 1 (Only wait for the leader to ack).
             */
            requiredAcks?: number
            /**
             * SASL configuration for the kafka client
             */
            sasl?: {
                /**
                 * SASLMechanism is the name of the enabled SASL mechanism. Possible values: OAUTHBEARER, PLAIN (defaults to PLAIN).
                 */
                mechanism?: string
                /**
                 * SecretKeySelector selects a key of a Secret.
                 */
                passwordSecret?: {
                    /**
                     * The key of the secret to select from.  Must be a valid secret key.
                     */
                    key: string
                    /**
                     * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                     */
                    name?: string
                    /**
                     * Specify whether the Secret or its key must be defined
                     */
                    optional?: boolean
                    [k: string]: unknown
                }
                /**
                 * SecretKeySelector selects a key of a Secret.
                 */
                userSecret?: {
                    /**
                     * The key of the secret to select from.  Must be a valid secret key.
                     */
                    key: string
                    /**
                     * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                     */
                    name?: string
                    /**
                     * Specify whether the Secret or its key must be defined
                     */
                    optional?: boolean
                    [k: string]: unknown
                }
                [k: string]: unknown
            }
            /**
             * TLS configuration for the Kafka producer.
             */
            tls?: {
                /**
                 * SecretKeySelector selects a key of a Secret.
                 */
                caCertSecret?: {
                    /**
                     * The key of the secret to select from.  Must be a valid secret key.
                     */
                    key: string
                    /**
                     * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                     */
                    name?: string
                    /**
                     * Specify whether the Secret or its key must be defined
                     */
                    optional?: boolean
                    [k: string]: unknown
                }
                /**
                 * SecretKeySelector selects a key of a Secret.
                 */
                clientCertSecret?: {
                    /**
                     * The key of the secret to select from.  Must be a valid secret key.
                     */
                    key: string
                    /**
                     * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                     */
                    name?: string
                    /**
                     * Specify whether the Secret or its key must be defined
                     */
                    optional?: boolean
                    [k: string]: unknown
                }
                /**
                 * SecretKeySelector selects a key of a Secret.
                 */
                clientKeySecret?: {
                    /**
                     * The key of the secret to select from.  Must be a valid secret key.
                     */
                    key: string
                    /**
                     * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                     */
                    name?: string
                    /**
                     * Specify whether the Secret or its key must be defined
                     */
                    optional?: boolean
                    [k: string]: unknown
                }
                /**
                 * If true, skips creation of TLSConfig with certs and creates an empty TLSConfig. (Defaults to false)
                 */
                insecureSkipVerify?: boolean
                [k: string]: unknown
            }
            /**
             * Name of the topic. More info at https://kafka.apache.org/documentation/#intro_topics
             */
            topic: string
            /**
             * URL of the Kafka broker, multiple URLs separated by comma.
             */
            url: string
            /**
             * Specify what kafka version is being connected to enables certain features in sarama, defaults to 1.0.0
             */
            version?: string
            [k: string]: unknown
        }
        /**
         * Log refers to the trigger designed to invoke log the event.
         */
        log?: {
            /**
             * Only print messages every interval. Useful to prevent logging too much data for busy events.
             */
            intervalSeconds?: number
            [k: string]: unknown
        }
        /**
         * Name is a unique name of the action to take.
         */
        name: string
        /**
         * NATS refers to the trigger designed to place message on NATS subject.
         */
        nats?: {
            parameters?: IoArgoprojSensorV1Alpha1TriggerParameter[]
            payload: IoArgoprojSensorV1Alpha1TriggerParameter[]
            /**
             * Name of the subject to put message on.
             */
            subject: string
            /**
             * TLS configuration for the NATS producer.
             */
            tls?: {
                /**
                 * SecretKeySelector selects a key of a Secret.
                 */
                caCertSecret?: {
                    /**
                     * The key of the secret to select from.  Must be a valid secret key.
                     */
                    key: string
                    /**
                     * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                     */
                    name?: string
                    /**
                     * Specify whether the Secret or its key must be defined
                     */
                    optional?: boolean
                    [k: string]: unknown
                }
                /**
                 * SecretKeySelector selects a key of a Secret.
                 */
                clientCertSecret?: {
                    /**
                     * The key of the secret to select from.  Must be a valid secret key.
                     */
                    key: string
                    /**
                     * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                     */
                    name?: string
                    /**
                     * Specify whether the Secret or its key must be defined
                     */
                    optional?: boolean
                    [k: string]: unknown
                }
                /**
                 * SecretKeySelector selects a key of a Secret.
                 */
                clientKeySecret?: {
                    /**
                     * The key of the secret to select from.  Must be a valid secret key.
                     */
                    key: string
                    /**
                     * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                     */
                    name?: string
                    /**
                     * Specify whether the Secret or its key must be defined
                     */
                    optional?: boolean
                    [k: string]: unknown
                }
                /**
                 * If true, skips creation of TLSConfig with certs and creates an empty TLSConfig. (Defaults to false)
                 */
                insecureSkipVerify?: boolean
                [k: string]: unknown
            }
            /**
             * URL of the NATS cluster.
             */
            url: string
            [k: string]: unknown
        }
        /**
         * OpenWhisk refers to the trigger designed to invoke OpenWhisk action.
         */
        openWhisk?: {
            /**
             * Name of the action/function.
             */
            actionName: string
            /**
             * SecretKeySelector selects a key of a Secret.
             */
            authToken?: {
                /**
                 * The key of the secret to select from.  Must be a valid secret key.
                 */
                key: string
                /**
                 * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                 */
                name?: string
                /**
                 * Specify whether the Secret or its key must be defined
                 */
                optional?: boolean
                [k: string]: unknown
            }
            /**
             * Host URL of the OpenWhisk.
             */
            host: string
            /**
             * Namespace for the action. Defaults to "_".
             */
            namespace?: string
            /**
             * Parameters is the list of key-value extracted from event's payload that are applied to the trigger resource.
             */
            parameters?: IoArgoprojSensorV1Alpha1TriggerParameter[]
            /**
             * Payload is the list of key-value extracted from an event payload to construct the request payload.
             */
            payload: IoArgoprojSensorV1Alpha1TriggerParameter[]
            /**
             * Version for the API. Defaults to v1.
             */
            version?: string
            [k: string]: unknown
        }
        /**
         * Pulsar refers to the trigger designed to place messages on Pulsar topic.
         */
        pulsar?: {
            /**
             * SecretKeySelector selects a key of a Secret.
             */
            authTokenSecret?: {
                /**
                 * The key of the secret to select from.  Must be a valid secret key.
                 */
                key: string
                /**
                 * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                 */
                name?: string
                /**
                 * Specify whether the Secret or its key must be defined
                 */
                optional?: boolean
                [k: string]: unknown
            }
            /**
             * Backoff holds parameters applied to connection.
             */
            connectionBackoff?: {
                /**
                 * The initial duration in nanoseconds or strings like "1s", "3m"
                 */
                duration?: number | string
                /**
                 * Duration is multiplied by factor each iteration
                 */
                factor?: number
                /**
                 * The amount of jitter applied each iteration
                 */
                jitter?: number
                /**
                 * Exit with error after this many steps
                 */
                steps?: number
                [k: string]: unknown
            }
            /**
             * Parameters is the list of parameters that is applied to resolved Kafka trigger object.
             */
            parameters?: IoArgoprojSensorV1Alpha1TriggerParameter[]
            /**
             * Payload is the list of key-value extracted from an event payload to construct the request payload.
             */
            payload: IoArgoprojSensorV1Alpha1TriggerParameter[]
            /**
             * TLS configuration for the pulsar client.
             */
            tls?: {
                /**
                 * SecretKeySelector selects a key of a Secret.
                 */
                caCertSecret?: {
                    /**
                     * The key of the secret to select from.  Must be a valid secret key.
                     */
                    key: string
                    /**
                     * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                     */
                    name?: string
                    /**
                     * Specify whether the Secret or its key must be defined
                     */
                    optional?: boolean
                    [k: string]: unknown
                }
                /**
                 * SecretKeySelector selects a key of a Secret.
                 */
                clientCertSecret?: {
                    /**
                     * The key of the secret to select from.  Must be a valid secret key.
                     */
                    key: string
                    /**
                     * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                     */
                    name?: string
                    /**
                     * Specify whether the Secret or its key must be defined
                     */
                    optional?: boolean
                    [k: string]: unknown
                }
                /**
                 * SecretKeySelector selects a key of a Secret.
                 */
                clientKeySecret?: {
                    /**
                     * The key of the secret to select from.  Must be a valid secret key.
                     */
                    key: string
                    /**
                     * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                     */
                    name?: string
                    /**
                     * Specify whether the Secret or its key must be defined
                     */
                    optional?: boolean
                    [k: string]: unknown
                }
                /**
                 * If true, skips creation of TLSConfig with certs and creates an empty TLSConfig. (Defaults to false)
                 */
                insecureSkipVerify?: boolean
                [k: string]: unknown
            }
            /**
             * Whether the Pulsar client accept untrusted TLS certificate from broker.
             */
            tlsAllowInsecureConnection?: boolean
            /**
             * SecretKeySelector selects a key of a Secret.
             */
            tlsTrustCertsSecret?: {
                /**
                 * The key of the secret to select from.  Must be a valid secret key.
                 */
                key: string
                /**
                 * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                 */
                name?: string
                /**
                 * Specify whether the Secret or its key must be defined
                 */
                optional?: boolean
                [k: string]: unknown
            }
            /**
             * Whether the Pulsar client verify the validity of the host name from broker.
             */
            tlsValidateHostname?: boolean
            /**
             * Name of the topic. See https://pulsar.apache.org/docs/en/concepts-messaging/
             */
            topic: string
            /**
             * Configure the service URL for the Pulsar service.
             */
            url: string
            [k: string]: unknown
        }
        /**
         * Slack refers to the trigger designed to send slack notification message.
         */
        slack?: {
            /**
             * Channel refers to which Slack channel to send slack message.
             */
            channel?: string
            /**
             * Message refers to the message to send to the Slack channel.
             */
            message?: string
            /**
             * Parameters is the list of key-value extracted from event's payload that are applied to the trigger resource.
             */
            parameters?: IoArgoprojSensorV1Alpha1TriggerParameter[]
            /**
             * SecretKeySelector selects a key of a Secret.
             */
            slackToken?: {
                /**
                 * The key of the secret to select from.  Must be a valid secret key.
                 */
                key: string
                /**
                 * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
                 */
                name?: string
                /**
                 * Specify whether the Secret or its key must be defined
                 */
                optional?: boolean
                [k: string]: unknown
            }
            [k: string]: unknown
        }
        [k: string]: unknown
    }
    [k: string]: unknown
}
/**
 * TriggerParameter indicates a passed parameter to a service template
 */
export interface IoArgoprojSensorV1Alpha1TriggerParameter {
    /**
     * Dest is the JSONPath of a resource key. A path is a series of keys separated by a dot. The colon character can be escaped with '.' The -1 key can be used to append a value to an existing array. See https://github.com/tidwall/sjson#path-syntax for more information about how this is used.
     */
    dest: string
    /**
     * Operation is what to do with the existing value at Dest, whether to 'prepend', 'overwrite', or 'append' it.
     */
    operation?: string
    /**
     * Src contains a source reference to the value of the parameter from a dependency
     */
    src?: {
        /**
         * ContextKey is the JSONPath of the event's (JSON decoded) context key ContextKey is a series of keys separated by a dot. A key may contain wildcard characters '*' and '?'. To access an array value use the index as the key. The dot and wildcard characters can be escaped with '\'. See https://github.com/tidwall/gjson#path-syntax for more information on how to use this.
         */
        contextKey?: string
        /**
         * ContextTemplate is a go-template for extracting a string from the event's context. If a ContextTemplate is provided with a ContextKey, the template will be evaluated first and fallback to the ContextKey. The templating follows the standard go-template syntax as well as sprig's extra functions. See https://pkg.go.dev/text/template and https://masterminds.github.io/sprig/
         */
        contextTemplate?: string
        /**
         * DataKey is the JSONPath of the event's (JSON decoded) data key DataKey is a series of keys separated by a dot. A key may contain wildcard characters '*' and '?'. To access an array value use the index as the key. The dot and wildcard characters can be escaped with '\'. See https://github.com/tidwall/gjson#path-syntax for more information on how to use this.
         */
        dataKey?: string
        /**
         * DataTemplate is a go-template for extracting a string from the event's data. If a DataTemplate is provided with a DataKey, the template will be evaluated first and fallback to the DataKey. The templating follows the standard go-template syntax as well as sprig's extra functions. See https://pkg.go.dev/text/template and https://masterminds.github.io/sprig/
         */
        dataTemplate?: string
        /**
         * DependencyName refers to the name of the dependency. The event which is stored for this dependency is used as payload for the parameterization. Make sure to refer to one of the dependencies you have defined under Dependencies list.
         */
        dependencyName: string
        /**
         * Value is the default literal value to use for this parameter source This is only used if the DataKey is invalid. If the DataKey is invalid and this is not defined, this param source will produce an error.
         */
        value?: string
        [k: string]: unknown
    }
    [k: string]: unknown
}
export interface IoArgoprojSensorV1Alpha1ConditionsResetCriteria {
    /**
     * Schedule is a cron-like expression. For reference, see: https://en.wikipedia.org/wiki/Cron
     */
    byTime?: {
        /**
         * Cron is a cron-like expression. For reference, see: https://en.wikipedia.org/wiki/Cron
         */
        cron?: string
        timezone?: string
        [k: string]: unknown
    }
    [k: string]: unknown
}
/**
 * SecureHeader refers to HTTP Headers with auth tokens as values
 */
export interface IoArgoprojCommonSecureHeader {
    name?: string
    /**
     * Values can be read from either secrets or configmaps
     */
    valueFrom?: {
        configMapKeyRef?: IoK8SApiCoreV1ConfigMapKeySelector
        secretKeyRef?: IoK8SApiCoreV1SecretKeySelector
        [k: string]: unknown
    }
    [k: string]: unknown
}
/**
 * Selects a key from a ConfigMap.
 */
export interface IoK8SApiCoreV1ConfigMapKeySelector {
    /**
     * The key to select.
     */
    key: string
    /**
     * Name of the referent. More info: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
     */
    name?: string
    /**
     * Specify whether the ConfigMap or its key must be defined
     */
    optional?: boolean
    [k: string]: unknown
}
/**
 * SensorStatus contains information about the status of a sensor.
 */
export interface IoArgoprojSensorV1Alpha1SensorStatus {
    /**
     * Conditions are the latest available observations of a resource's current state.
     */
    conditions?: IoArgoprojCommonCondition[]
    [k: string]: unknown
}
