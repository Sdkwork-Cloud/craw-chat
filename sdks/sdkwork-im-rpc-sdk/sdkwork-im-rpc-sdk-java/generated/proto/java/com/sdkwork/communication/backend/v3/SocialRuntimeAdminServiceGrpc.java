package com.sdkwork.communication.backend.v3;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class SocialRuntimeAdminServiceGrpc {

  private SocialRuntimeAdminServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.backend.v3.SocialRuntimeAdminService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedRequest,
      com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedResponse> getClaimPendingSharedChannelSyncTargetedMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ClaimPendingSharedChannelSyncTargeted",
      requestType = com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedRequest.class,
      responseType = com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedRequest,
      com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedResponse> getClaimPendingSharedChannelSyncTargetedMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedRequest, com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedResponse> getClaimPendingSharedChannelSyncTargetedMethod;
    if ((getClaimPendingSharedChannelSyncTargetedMethod = SocialRuntimeAdminServiceGrpc.getClaimPendingSharedChannelSyncTargetedMethod) == null) {
      synchronized (SocialRuntimeAdminServiceGrpc.class) {
        if ((getClaimPendingSharedChannelSyncTargetedMethod = SocialRuntimeAdminServiceGrpc.getClaimPendingSharedChannelSyncTargetedMethod) == null) {
          SocialRuntimeAdminServiceGrpc.getClaimPendingSharedChannelSyncTargetedMethod = getClaimPendingSharedChannelSyncTargetedMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedRequest, com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ClaimPendingSharedChannelSyncTargeted"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialRuntimeAdminServiceMethodDescriptorSupplier("ClaimPendingSharedChannelSyncTargeted"))
              .build();
        }
      }
    }
    return getClaimPendingSharedChannelSyncTargetedMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncRequest,
      com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncResponse> getListDeadLetterSharedChannelSyncMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListDeadLetterSharedChannelSync",
      requestType = com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncRequest.class,
      responseType = com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncRequest,
      com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncResponse> getListDeadLetterSharedChannelSyncMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncRequest, com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncResponse> getListDeadLetterSharedChannelSyncMethod;
    if ((getListDeadLetterSharedChannelSyncMethod = SocialRuntimeAdminServiceGrpc.getListDeadLetterSharedChannelSyncMethod) == null) {
      synchronized (SocialRuntimeAdminServiceGrpc.class) {
        if ((getListDeadLetterSharedChannelSyncMethod = SocialRuntimeAdminServiceGrpc.getListDeadLetterSharedChannelSyncMethod) == null) {
          SocialRuntimeAdminServiceGrpc.getListDeadLetterSharedChannelSyncMethod = getListDeadLetterSharedChannelSyncMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncRequest, com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListDeadLetterSharedChannelSync"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialRuntimeAdminServiceMethodDescriptorSupplier("ListDeadLetterSharedChannelSync"))
              .build();
        }
      }
    }
    return getListDeadLetterSharedChannelSyncMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncRequest,
      com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncResponse> getListDeliveredSharedChannelSyncMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListDeliveredSharedChannelSync",
      requestType = com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncRequest.class,
      responseType = com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncRequest,
      com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncResponse> getListDeliveredSharedChannelSyncMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncRequest, com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncResponse> getListDeliveredSharedChannelSyncMethod;
    if ((getListDeliveredSharedChannelSyncMethod = SocialRuntimeAdminServiceGrpc.getListDeliveredSharedChannelSyncMethod) == null) {
      synchronized (SocialRuntimeAdminServiceGrpc.class) {
        if ((getListDeliveredSharedChannelSyncMethod = SocialRuntimeAdminServiceGrpc.getListDeliveredSharedChannelSyncMethod) == null) {
          SocialRuntimeAdminServiceGrpc.getListDeliveredSharedChannelSyncMethod = getListDeliveredSharedChannelSyncMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncRequest, com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListDeliveredSharedChannelSync"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialRuntimeAdminServiceMethodDescriptorSupplier("ListDeliveredSharedChannelSync"))
              .build();
        }
      }
    }
    return getListDeliveredSharedChannelSyncMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncRequest,
      com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncResponse> getListDeliveryStateSharedChannelSyncMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListDeliveryStateSharedChannelSync",
      requestType = com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncRequest.class,
      responseType = com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncRequest,
      com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncResponse> getListDeliveryStateSharedChannelSyncMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncRequest, com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncResponse> getListDeliveryStateSharedChannelSyncMethod;
    if ((getListDeliveryStateSharedChannelSyncMethod = SocialRuntimeAdminServiceGrpc.getListDeliveryStateSharedChannelSyncMethod) == null) {
      synchronized (SocialRuntimeAdminServiceGrpc.class) {
        if ((getListDeliveryStateSharedChannelSyncMethod = SocialRuntimeAdminServiceGrpc.getListDeliveryStateSharedChannelSyncMethod) == null) {
          SocialRuntimeAdminServiceGrpc.getListDeliveryStateSharedChannelSyncMethod = getListDeliveryStateSharedChannelSyncMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncRequest, com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListDeliveryStateSharedChannelSync"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialRuntimeAdminServiceMethodDescriptorSupplier("ListDeliveryStateSharedChannelSync"))
              .build();
        }
      }
    }
    return getListDeliveryStateSharedChannelSyncMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncRequest,
      com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncResponse> getListPendingSharedChannelSyncMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListPendingSharedChannelSync",
      requestType = com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncRequest.class,
      responseType = com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncRequest,
      com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncResponse> getListPendingSharedChannelSyncMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncRequest, com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncResponse> getListPendingSharedChannelSyncMethod;
    if ((getListPendingSharedChannelSyncMethod = SocialRuntimeAdminServiceGrpc.getListPendingSharedChannelSyncMethod) == null) {
      synchronized (SocialRuntimeAdminServiceGrpc.class) {
        if ((getListPendingSharedChannelSyncMethod = SocialRuntimeAdminServiceGrpc.getListPendingSharedChannelSyncMethod) == null) {
          SocialRuntimeAdminServiceGrpc.getListPendingSharedChannelSyncMethod = getListPendingSharedChannelSyncMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncRequest, com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListPendingSharedChannelSync"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialRuntimeAdminServiceMethodDescriptorSupplier("ListPendingSharedChannelSync"))
              .build();
        }
      }
    }
    return getListPendingSharedChannelSyncMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncRequest,
      com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncResponse> getReclaimStalePendingSharedChannelSyncMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ReclaimStalePendingSharedChannelSync",
      requestType = com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncRequest.class,
      responseType = com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncRequest,
      com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncResponse> getReclaimStalePendingSharedChannelSyncMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncRequest, com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncResponse> getReclaimStalePendingSharedChannelSyncMethod;
    if ((getReclaimStalePendingSharedChannelSyncMethod = SocialRuntimeAdminServiceGrpc.getReclaimStalePendingSharedChannelSyncMethod) == null) {
      synchronized (SocialRuntimeAdminServiceGrpc.class) {
        if ((getReclaimStalePendingSharedChannelSyncMethod = SocialRuntimeAdminServiceGrpc.getReclaimStalePendingSharedChannelSyncMethod) == null) {
          SocialRuntimeAdminServiceGrpc.getReclaimStalePendingSharedChannelSyncMethod = getReclaimStalePendingSharedChannelSyncMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncRequest, com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ReclaimStalePendingSharedChannelSync"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialRuntimeAdminServiceMethodDescriptorSupplier("ReclaimStalePendingSharedChannelSync"))
              .build();
        }
      }
    }
    return getReclaimStalePendingSharedChannelSyncMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedRequest,
      com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedResponse> getReleasePendingSharedChannelSyncTargetedMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ReleasePendingSharedChannelSyncTargeted",
      requestType = com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedRequest.class,
      responseType = com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedRequest,
      com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedResponse> getReleasePendingSharedChannelSyncTargetedMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedRequest, com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedResponse> getReleasePendingSharedChannelSyncTargetedMethod;
    if ((getReleasePendingSharedChannelSyncTargetedMethod = SocialRuntimeAdminServiceGrpc.getReleasePendingSharedChannelSyncTargetedMethod) == null) {
      synchronized (SocialRuntimeAdminServiceGrpc.class) {
        if ((getReleasePendingSharedChannelSyncTargetedMethod = SocialRuntimeAdminServiceGrpc.getReleasePendingSharedChannelSyncTargetedMethod) == null) {
          SocialRuntimeAdminServiceGrpc.getReleasePendingSharedChannelSyncTargetedMethod = getReleasePendingSharedChannelSyncTargetedMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedRequest, com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ReleasePendingSharedChannelSyncTargeted"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialRuntimeAdminServiceMethodDescriptorSupplier("ReleasePendingSharedChannelSyncTargeted"))
              .build();
        }
      }
    }
    return getReleasePendingSharedChannelSyncTargetedMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RepairDerivedSnapshotRequest,
      com.sdkwork.communication.backend.v3.RepairDerivedSnapshotResponse> getRepairDerivedSnapshotMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RepairDerivedSnapshot",
      requestType = com.sdkwork.communication.backend.v3.RepairDerivedSnapshotRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RepairDerivedSnapshotResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RepairDerivedSnapshotRequest,
      com.sdkwork.communication.backend.v3.RepairDerivedSnapshotResponse> getRepairDerivedSnapshotMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RepairDerivedSnapshotRequest, com.sdkwork.communication.backend.v3.RepairDerivedSnapshotResponse> getRepairDerivedSnapshotMethod;
    if ((getRepairDerivedSnapshotMethod = SocialRuntimeAdminServiceGrpc.getRepairDerivedSnapshotMethod) == null) {
      synchronized (SocialRuntimeAdminServiceGrpc.class) {
        if ((getRepairDerivedSnapshotMethod = SocialRuntimeAdminServiceGrpc.getRepairDerivedSnapshotMethod) == null) {
          SocialRuntimeAdminServiceGrpc.getRepairDerivedSnapshotMethod = getRepairDerivedSnapshotMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RepairDerivedSnapshotRequest, com.sdkwork.communication.backend.v3.RepairDerivedSnapshotResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RepairDerivedSnapshot"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RepairDerivedSnapshotRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RepairDerivedSnapshotResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialRuntimeAdminServiceMethodDescriptorSupplier("RepairDerivedSnapshot"))
              .build();
        }
      }
    }
    return getRepairDerivedSnapshotMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RepairSharedChannelSyncRequest,
      com.sdkwork.communication.backend.v3.RepairSharedChannelSyncResponse> getRepairSharedChannelSyncMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RepairSharedChannelSync",
      requestType = com.sdkwork.communication.backend.v3.RepairSharedChannelSyncRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RepairSharedChannelSyncResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RepairSharedChannelSyncRequest,
      com.sdkwork.communication.backend.v3.RepairSharedChannelSyncResponse> getRepairSharedChannelSyncMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RepairSharedChannelSyncRequest, com.sdkwork.communication.backend.v3.RepairSharedChannelSyncResponse> getRepairSharedChannelSyncMethod;
    if ((getRepairSharedChannelSyncMethod = SocialRuntimeAdminServiceGrpc.getRepairSharedChannelSyncMethod) == null) {
      synchronized (SocialRuntimeAdminServiceGrpc.class) {
        if ((getRepairSharedChannelSyncMethod = SocialRuntimeAdminServiceGrpc.getRepairSharedChannelSyncMethod) == null) {
          SocialRuntimeAdminServiceGrpc.getRepairSharedChannelSyncMethod = getRepairSharedChannelSyncMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RepairSharedChannelSyncRequest, com.sdkwork.communication.backend.v3.RepairSharedChannelSyncResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RepairSharedChannelSync"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RepairSharedChannelSyncRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RepairSharedChannelSyncResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialRuntimeAdminServiceMethodDescriptorSupplier("RepairSharedChannelSync"))
              .build();
        }
      }
    }
    return getRepairSharedChannelSyncMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedRequest,
      com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedResponse> getRepublishPendingSharedChannelSyncTargetedMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RepublishPendingSharedChannelSyncTargeted",
      requestType = com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedRequest,
      com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedResponse> getRepublishPendingSharedChannelSyncTargetedMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedRequest, com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedResponse> getRepublishPendingSharedChannelSyncTargetedMethod;
    if ((getRepublishPendingSharedChannelSyncTargetedMethod = SocialRuntimeAdminServiceGrpc.getRepublishPendingSharedChannelSyncTargetedMethod) == null) {
      synchronized (SocialRuntimeAdminServiceGrpc.class) {
        if ((getRepublishPendingSharedChannelSyncTargetedMethod = SocialRuntimeAdminServiceGrpc.getRepublishPendingSharedChannelSyncTargetedMethod) == null) {
          SocialRuntimeAdminServiceGrpc.getRepublishPendingSharedChannelSyncTargetedMethod = getRepublishPendingSharedChannelSyncTargetedMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedRequest, com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RepublishPendingSharedChannelSyncTargeted"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialRuntimeAdminServiceMethodDescriptorSupplier("RepublishPendingSharedChannelSyncTargeted"))
              .build();
        }
      }
    }
    return getRepublishPendingSharedChannelSyncTargetedMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncRequest,
      com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncResponse> getRequeueDeadLetterSharedChannelSyncMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RequeueDeadLetterSharedChannelSync",
      requestType = com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncRequest,
      com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncResponse> getRequeueDeadLetterSharedChannelSyncMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncRequest, com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncResponse> getRequeueDeadLetterSharedChannelSyncMethod;
    if ((getRequeueDeadLetterSharedChannelSyncMethod = SocialRuntimeAdminServiceGrpc.getRequeueDeadLetterSharedChannelSyncMethod) == null) {
      synchronized (SocialRuntimeAdminServiceGrpc.class) {
        if ((getRequeueDeadLetterSharedChannelSyncMethod = SocialRuntimeAdminServiceGrpc.getRequeueDeadLetterSharedChannelSyncMethod) == null) {
          SocialRuntimeAdminServiceGrpc.getRequeueDeadLetterSharedChannelSyncMethod = getRequeueDeadLetterSharedChannelSyncMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncRequest, com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RequeueDeadLetterSharedChannelSync"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialRuntimeAdminServiceMethodDescriptorSupplier("RequeueDeadLetterSharedChannelSync"))
              .build();
        }
      }
    }
    return getRequeueDeadLetterSharedChannelSyncMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedRequest,
      com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedResponse> getRequeueDeadLetterSharedChannelSyncTargetedMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RequeueDeadLetterSharedChannelSyncTargeted",
      requestType = com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedRequest,
      com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedResponse> getRequeueDeadLetterSharedChannelSyncTargetedMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedRequest, com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedResponse> getRequeueDeadLetterSharedChannelSyncTargetedMethod;
    if ((getRequeueDeadLetterSharedChannelSyncTargetedMethod = SocialRuntimeAdminServiceGrpc.getRequeueDeadLetterSharedChannelSyncTargetedMethod) == null) {
      synchronized (SocialRuntimeAdminServiceGrpc.class) {
        if ((getRequeueDeadLetterSharedChannelSyncTargetedMethod = SocialRuntimeAdminServiceGrpc.getRequeueDeadLetterSharedChannelSyncTargetedMethod) == null) {
          SocialRuntimeAdminServiceGrpc.getRequeueDeadLetterSharedChannelSyncTargetedMethod = getRequeueDeadLetterSharedChannelSyncTargetedMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedRequest, com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RequeueDeadLetterSharedChannelSyncTargeted"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialRuntimeAdminServiceMethodDescriptorSupplier("RequeueDeadLetterSharedChannelSyncTargeted"))
              .build();
        }
      }
    }
    return getRequeueDeadLetterSharedChannelSyncTargetedMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedRequest,
      com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedResponse> getTakeoverPendingSharedChannelSyncTargetedMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "TakeoverPendingSharedChannelSyncTargeted",
      requestType = com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedRequest.class,
      responseType = com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedRequest,
      com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedResponse> getTakeoverPendingSharedChannelSyncTargetedMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedRequest, com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedResponse> getTakeoverPendingSharedChannelSyncTargetedMethod;
    if ((getTakeoverPendingSharedChannelSyncTargetedMethod = SocialRuntimeAdminServiceGrpc.getTakeoverPendingSharedChannelSyncTargetedMethod) == null) {
      synchronized (SocialRuntimeAdminServiceGrpc.class) {
        if ((getTakeoverPendingSharedChannelSyncTargetedMethod = SocialRuntimeAdminServiceGrpc.getTakeoverPendingSharedChannelSyncTargetedMethod) == null) {
          SocialRuntimeAdminServiceGrpc.getTakeoverPendingSharedChannelSyncTargetedMethod = getTakeoverPendingSharedChannelSyncTargetedMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedRequest, com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "TakeoverPendingSharedChannelSyncTargeted"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedResponse.getDefaultInstance()))
              .setSchemaDescriptor(new SocialRuntimeAdminServiceMethodDescriptorSupplier("TakeoverPendingSharedChannelSyncTargeted"))
              .build();
        }
      }
    }
    return getTakeoverPendingSharedChannelSyncTargetedMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static SocialRuntimeAdminServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SocialRuntimeAdminServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SocialRuntimeAdminServiceStub>() {
        @java.lang.Override
        public SocialRuntimeAdminServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SocialRuntimeAdminServiceStub(channel, callOptions);
        }
      };
    return SocialRuntimeAdminServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static SocialRuntimeAdminServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SocialRuntimeAdminServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SocialRuntimeAdminServiceBlockingV2Stub>() {
        @java.lang.Override
        public SocialRuntimeAdminServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SocialRuntimeAdminServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return SocialRuntimeAdminServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static SocialRuntimeAdminServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SocialRuntimeAdminServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SocialRuntimeAdminServiceBlockingStub>() {
        @java.lang.Override
        public SocialRuntimeAdminServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SocialRuntimeAdminServiceBlockingStub(channel, callOptions);
        }
      };
    return SocialRuntimeAdminServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static SocialRuntimeAdminServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<SocialRuntimeAdminServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<SocialRuntimeAdminServiceFutureStub>() {
        @java.lang.Override
        public SocialRuntimeAdminServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new SocialRuntimeAdminServiceFutureStub(channel, callOptions);
        }
      };
    return SocialRuntimeAdminServiceFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void claimPendingSharedChannelSyncTargeted(com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getClaimPendingSharedChannelSyncTargetedMethod(), responseObserver);
    }

    /**
     */
    default void listDeadLetterSharedChannelSync(com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListDeadLetterSharedChannelSyncMethod(), responseObserver);
    }

    /**
     */
    default void listDeliveredSharedChannelSync(com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListDeliveredSharedChannelSyncMethod(), responseObserver);
    }

    /**
     */
    default void listDeliveryStateSharedChannelSync(com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListDeliveryStateSharedChannelSyncMethod(), responseObserver);
    }

    /**
     */
    default void listPendingSharedChannelSync(com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListPendingSharedChannelSyncMethod(), responseObserver);
    }

    /**
     */
    default void reclaimStalePendingSharedChannelSync(com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getReclaimStalePendingSharedChannelSyncMethod(), responseObserver);
    }

    /**
     */
    default void releasePendingSharedChannelSyncTargeted(com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getReleasePendingSharedChannelSyncTargetedMethod(), responseObserver);
    }

    /**
     */
    default void repairDerivedSnapshot(com.sdkwork.communication.backend.v3.RepairDerivedSnapshotRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RepairDerivedSnapshotResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRepairDerivedSnapshotMethod(), responseObserver);
    }

    /**
     */
    default void repairSharedChannelSync(com.sdkwork.communication.backend.v3.RepairSharedChannelSyncRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RepairSharedChannelSyncResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRepairSharedChannelSyncMethod(), responseObserver);
    }

    /**
     */
    default void republishPendingSharedChannelSyncTargeted(com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRepublishPendingSharedChannelSyncTargetedMethod(), responseObserver);
    }

    /**
     */
    default void requeueDeadLetterSharedChannelSync(com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRequeueDeadLetterSharedChannelSyncMethod(), responseObserver);
    }

    /**
     */
    default void requeueDeadLetterSharedChannelSyncTargeted(com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRequeueDeadLetterSharedChannelSyncTargetedMethod(), responseObserver);
    }

    /**
     */
    default void takeoverPendingSharedChannelSyncTargeted(com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getTakeoverPendingSharedChannelSyncTargetedMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service SocialRuntimeAdminService.
   */
  public static abstract class SocialRuntimeAdminServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return SocialRuntimeAdminServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service SocialRuntimeAdminService.
   */
  public static final class SocialRuntimeAdminServiceStub
      extends io.grpc.stub.AbstractAsyncStub<SocialRuntimeAdminServiceStub> {
    private SocialRuntimeAdminServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SocialRuntimeAdminServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SocialRuntimeAdminServiceStub(channel, callOptions);
    }

    /**
     */
    public void claimPendingSharedChannelSyncTargeted(com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getClaimPendingSharedChannelSyncTargetedMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void listDeadLetterSharedChannelSync(com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListDeadLetterSharedChannelSyncMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void listDeliveredSharedChannelSync(com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListDeliveredSharedChannelSyncMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void listDeliveryStateSharedChannelSync(com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListDeliveryStateSharedChannelSyncMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void listPendingSharedChannelSync(com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListPendingSharedChannelSyncMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void reclaimStalePendingSharedChannelSync(com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getReclaimStalePendingSharedChannelSyncMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void releasePendingSharedChannelSyncTargeted(com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getReleasePendingSharedChannelSyncTargetedMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void repairDerivedSnapshot(com.sdkwork.communication.backend.v3.RepairDerivedSnapshotRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RepairDerivedSnapshotResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRepairDerivedSnapshotMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void repairSharedChannelSync(com.sdkwork.communication.backend.v3.RepairSharedChannelSyncRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RepairSharedChannelSyncResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRepairSharedChannelSyncMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void republishPendingSharedChannelSyncTargeted(com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRepublishPendingSharedChannelSyncTargetedMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void requeueDeadLetterSharedChannelSync(com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRequeueDeadLetterSharedChannelSyncMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void requeueDeadLetterSharedChannelSyncTargeted(com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRequeueDeadLetterSharedChannelSyncTargetedMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void takeoverPendingSharedChannelSyncTargeted(com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getTakeoverPendingSharedChannelSyncTargetedMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service SocialRuntimeAdminService.
   */
  public static final class SocialRuntimeAdminServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<SocialRuntimeAdminServiceBlockingV2Stub> {
    private SocialRuntimeAdminServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SocialRuntimeAdminServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SocialRuntimeAdminServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedResponse claimPendingSharedChannelSyncTargeted(com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getClaimPendingSharedChannelSyncTargetedMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncResponse listDeadLetterSharedChannelSync(com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListDeadLetterSharedChannelSyncMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncResponse listDeliveredSharedChannelSync(com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListDeliveredSharedChannelSyncMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncResponse listDeliveryStateSharedChannelSync(com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListDeliveryStateSharedChannelSyncMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncResponse listPendingSharedChannelSync(com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListPendingSharedChannelSyncMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncResponse reclaimStalePendingSharedChannelSync(com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getReclaimStalePendingSharedChannelSyncMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedResponse releasePendingSharedChannelSyncTargeted(com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getReleasePendingSharedChannelSyncTargetedMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RepairDerivedSnapshotResponse repairDerivedSnapshot(com.sdkwork.communication.backend.v3.RepairDerivedSnapshotRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRepairDerivedSnapshotMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RepairSharedChannelSyncResponse repairSharedChannelSync(com.sdkwork.communication.backend.v3.RepairSharedChannelSyncRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRepairSharedChannelSyncMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedResponse republishPendingSharedChannelSyncTargeted(com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRepublishPendingSharedChannelSyncTargetedMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncResponse requeueDeadLetterSharedChannelSync(com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRequeueDeadLetterSharedChannelSyncMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedResponse requeueDeadLetterSharedChannelSyncTargeted(com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRequeueDeadLetterSharedChannelSyncTargetedMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedResponse takeoverPendingSharedChannelSyncTargeted(com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getTakeoverPendingSharedChannelSyncTargetedMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service SocialRuntimeAdminService.
   */
  public static final class SocialRuntimeAdminServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<SocialRuntimeAdminServiceBlockingStub> {
    private SocialRuntimeAdminServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SocialRuntimeAdminServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SocialRuntimeAdminServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedResponse claimPendingSharedChannelSyncTargeted(com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getClaimPendingSharedChannelSyncTargetedMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncResponse listDeadLetterSharedChannelSync(com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListDeadLetterSharedChannelSyncMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncResponse listDeliveredSharedChannelSync(com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListDeliveredSharedChannelSyncMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncResponse listDeliveryStateSharedChannelSync(com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListDeliveryStateSharedChannelSyncMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncResponse listPendingSharedChannelSync(com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListPendingSharedChannelSyncMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncResponse reclaimStalePendingSharedChannelSync(com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getReclaimStalePendingSharedChannelSyncMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedResponse releasePendingSharedChannelSyncTargeted(com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getReleasePendingSharedChannelSyncTargetedMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RepairDerivedSnapshotResponse repairDerivedSnapshot(com.sdkwork.communication.backend.v3.RepairDerivedSnapshotRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRepairDerivedSnapshotMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RepairSharedChannelSyncResponse repairSharedChannelSync(com.sdkwork.communication.backend.v3.RepairSharedChannelSyncRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRepairSharedChannelSyncMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedResponse republishPendingSharedChannelSyncTargeted(com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRepublishPendingSharedChannelSyncTargetedMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncResponse requeueDeadLetterSharedChannelSync(com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRequeueDeadLetterSharedChannelSyncMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedResponse requeueDeadLetterSharedChannelSyncTargeted(com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRequeueDeadLetterSharedChannelSyncTargetedMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedResponse takeoverPendingSharedChannelSyncTargeted(com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getTakeoverPendingSharedChannelSyncTargetedMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service SocialRuntimeAdminService.
   */
  public static final class SocialRuntimeAdminServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<SocialRuntimeAdminServiceFutureStub> {
    private SocialRuntimeAdminServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected SocialRuntimeAdminServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new SocialRuntimeAdminServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedResponse> claimPendingSharedChannelSyncTargeted(
        com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getClaimPendingSharedChannelSyncTargetedMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncResponse> listDeadLetterSharedChannelSync(
        com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListDeadLetterSharedChannelSyncMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncResponse> listDeliveredSharedChannelSync(
        com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListDeliveredSharedChannelSyncMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncResponse> listDeliveryStateSharedChannelSync(
        com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListDeliveryStateSharedChannelSyncMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncResponse> listPendingSharedChannelSync(
        com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListPendingSharedChannelSyncMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncResponse> reclaimStalePendingSharedChannelSync(
        com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getReclaimStalePendingSharedChannelSyncMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedResponse> releasePendingSharedChannelSyncTargeted(
        com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getReleasePendingSharedChannelSyncTargetedMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RepairDerivedSnapshotResponse> repairDerivedSnapshot(
        com.sdkwork.communication.backend.v3.RepairDerivedSnapshotRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRepairDerivedSnapshotMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RepairSharedChannelSyncResponse> repairSharedChannelSync(
        com.sdkwork.communication.backend.v3.RepairSharedChannelSyncRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRepairSharedChannelSyncMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedResponse> republishPendingSharedChannelSyncTargeted(
        com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRepublishPendingSharedChannelSyncTargetedMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncResponse> requeueDeadLetterSharedChannelSync(
        com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRequeueDeadLetterSharedChannelSyncMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedResponse> requeueDeadLetterSharedChannelSyncTargeted(
        com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRequeueDeadLetterSharedChannelSyncTargetedMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedResponse> takeoverPendingSharedChannelSyncTargeted(
        com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getTakeoverPendingSharedChannelSyncTargetedMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_CLAIM_PENDING_SHARED_CHANNEL_SYNC_TARGETED = 0;
  private static final int METHODID_LIST_DEAD_LETTER_SHARED_CHANNEL_SYNC = 1;
  private static final int METHODID_LIST_DELIVERED_SHARED_CHANNEL_SYNC = 2;
  private static final int METHODID_LIST_DELIVERY_STATE_SHARED_CHANNEL_SYNC = 3;
  private static final int METHODID_LIST_PENDING_SHARED_CHANNEL_SYNC = 4;
  private static final int METHODID_RECLAIM_STALE_PENDING_SHARED_CHANNEL_SYNC = 5;
  private static final int METHODID_RELEASE_PENDING_SHARED_CHANNEL_SYNC_TARGETED = 6;
  private static final int METHODID_REPAIR_DERIVED_SNAPSHOT = 7;
  private static final int METHODID_REPAIR_SHARED_CHANNEL_SYNC = 8;
  private static final int METHODID_REPUBLISH_PENDING_SHARED_CHANNEL_SYNC_TARGETED = 9;
  private static final int METHODID_REQUEUE_DEAD_LETTER_SHARED_CHANNEL_SYNC = 10;
  private static final int METHODID_REQUEUE_DEAD_LETTER_SHARED_CHANNEL_SYNC_TARGETED = 11;
  private static final int METHODID_TAKEOVER_PENDING_SHARED_CHANNEL_SYNC_TARGETED = 12;

  private static final class MethodHandlers<Req, Resp> implements
      io.grpc.stub.ServerCalls.UnaryMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.ServerStreamingMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.ClientStreamingMethod<Req, Resp>,
      io.grpc.stub.ServerCalls.BidiStreamingMethod<Req, Resp> {
    private final AsyncService serviceImpl;
    private final int methodId;

    MethodHandlers(AsyncService serviceImpl, int methodId) {
      this.serviceImpl = serviceImpl;
      this.methodId = methodId;
    }

    @java.lang.Override
    @java.lang.SuppressWarnings("unchecked")
    public void invoke(Req request, io.grpc.stub.StreamObserver<Resp> responseObserver) {
      switch (methodId) {
        case METHODID_CLAIM_PENDING_SHARED_CHANNEL_SYNC_TARGETED:
          serviceImpl.claimPendingSharedChannelSyncTargeted((com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedResponse>) responseObserver);
          break;
        case METHODID_LIST_DEAD_LETTER_SHARED_CHANNEL_SYNC:
          serviceImpl.listDeadLetterSharedChannelSync((com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncResponse>) responseObserver);
          break;
        case METHODID_LIST_DELIVERED_SHARED_CHANNEL_SYNC:
          serviceImpl.listDeliveredSharedChannelSync((com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncResponse>) responseObserver);
          break;
        case METHODID_LIST_DELIVERY_STATE_SHARED_CHANNEL_SYNC:
          serviceImpl.listDeliveryStateSharedChannelSync((com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncResponse>) responseObserver);
          break;
        case METHODID_LIST_PENDING_SHARED_CHANNEL_SYNC:
          serviceImpl.listPendingSharedChannelSync((com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncResponse>) responseObserver);
          break;
        case METHODID_RECLAIM_STALE_PENDING_SHARED_CHANNEL_SYNC:
          serviceImpl.reclaimStalePendingSharedChannelSync((com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncResponse>) responseObserver);
          break;
        case METHODID_RELEASE_PENDING_SHARED_CHANNEL_SYNC_TARGETED:
          serviceImpl.releasePendingSharedChannelSyncTargeted((com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedResponse>) responseObserver);
          break;
        case METHODID_REPAIR_DERIVED_SNAPSHOT:
          serviceImpl.repairDerivedSnapshot((com.sdkwork.communication.backend.v3.RepairDerivedSnapshotRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RepairDerivedSnapshotResponse>) responseObserver);
          break;
        case METHODID_REPAIR_SHARED_CHANNEL_SYNC:
          serviceImpl.repairSharedChannelSync((com.sdkwork.communication.backend.v3.RepairSharedChannelSyncRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RepairSharedChannelSyncResponse>) responseObserver);
          break;
        case METHODID_REPUBLISH_PENDING_SHARED_CHANNEL_SYNC_TARGETED:
          serviceImpl.republishPendingSharedChannelSyncTargeted((com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedResponse>) responseObserver);
          break;
        case METHODID_REQUEUE_DEAD_LETTER_SHARED_CHANNEL_SYNC:
          serviceImpl.requeueDeadLetterSharedChannelSync((com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncResponse>) responseObserver);
          break;
        case METHODID_REQUEUE_DEAD_LETTER_SHARED_CHANNEL_SYNC_TARGETED:
          serviceImpl.requeueDeadLetterSharedChannelSyncTargeted((com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedResponse>) responseObserver);
          break;
        case METHODID_TAKEOVER_PENDING_SHARED_CHANNEL_SYNC_TARGETED:
          serviceImpl.takeoverPendingSharedChannelSyncTargeted((com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedResponse>) responseObserver);
          break;
        default:
          throw new AssertionError();
      }
    }

    @java.lang.Override
    @java.lang.SuppressWarnings("unchecked")
    public io.grpc.stub.StreamObserver<Req> invoke(
        io.grpc.stub.StreamObserver<Resp> responseObserver) {
      switch (methodId) {
        default:
          throw new AssertionError();
      }
    }
  }

  public static final io.grpc.ServerServiceDefinition bindService(AsyncService service) {
    return io.grpc.ServerServiceDefinition.builder(getServiceDescriptor())
        .addMethod(
          getClaimPendingSharedChannelSyncTargetedMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedRequest,
              com.sdkwork.communication.backend.v3.ClaimPendingSharedChannelSyncTargetedResponse>(
                service, METHODID_CLAIM_PENDING_SHARED_CHANNEL_SYNC_TARGETED)))
        .addMethod(
          getListDeadLetterSharedChannelSyncMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncRequest,
              com.sdkwork.communication.backend.v3.ListDeadLetterSharedChannelSyncResponse>(
                service, METHODID_LIST_DEAD_LETTER_SHARED_CHANNEL_SYNC)))
        .addMethod(
          getListDeliveredSharedChannelSyncMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncRequest,
              com.sdkwork.communication.backend.v3.ListDeliveredSharedChannelSyncResponse>(
                service, METHODID_LIST_DELIVERED_SHARED_CHANNEL_SYNC)))
        .addMethod(
          getListDeliveryStateSharedChannelSyncMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncRequest,
              com.sdkwork.communication.backend.v3.ListDeliveryStateSharedChannelSyncResponse>(
                service, METHODID_LIST_DELIVERY_STATE_SHARED_CHANNEL_SYNC)))
        .addMethod(
          getListPendingSharedChannelSyncMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncRequest,
              com.sdkwork.communication.backend.v3.ListPendingSharedChannelSyncResponse>(
                service, METHODID_LIST_PENDING_SHARED_CHANNEL_SYNC)))
        .addMethod(
          getReclaimStalePendingSharedChannelSyncMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncRequest,
              com.sdkwork.communication.backend.v3.ReclaimStalePendingSharedChannelSyncResponse>(
                service, METHODID_RECLAIM_STALE_PENDING_SHARED_CHANNEL_SYNC)))
        .addMethod(
          getReleasePendingSharedChannelSyncTargetedMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedRequest,
              com.sdkwork.communication.backend.v3.ReleasePendingSharedChannelSyncTargetedResponse>(
                service, METHODID_RELEASE_PENDING_SHARED_CHANNEL_SYNC_TARGETED)))
        .addMethod(
          getRepairDerivedSnapshotMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RepairDerivedSnapshotRequest,
              com.sdkwork.communication.backend.v3.RepairDerivedSnapshotResponse>(
                service, METHODID_REPAIR_DERIVED_SNAPSHOT)))
        .addMethod(
          getRepairSharedChannelSyncMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RepairSharedChannelSyncRequest,
              com.sdkwork.communication.backend.v3.RepairSharedChannelSyncResponse>(
                service, METHODID_REPAIR_SHARED_CHANNEL_SYNC)))
        .addMethod(
          getRepublishPendingSharedChannelSyncTargetedMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedRequest,
              com.sdkwork.communication.backend.v3.RepublishPendingSharedChannelSyncTargetedResponse>(
                service, METHODID_REPUBLISH_PENDING_SHARED_CHANNEL_SYNC_TARGETED)))
        .addMethod(
          getRequeueDeadLetterSharedChannelSyncMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncRequest,
              com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncResponse>(
                service, METHODID_REQUEUE_DEAD_LETTER_SHARED_CHANNEL_SYNC)))
        .addMethod(
          getRequeueDeadLetterSharedChannelSyncTargetedMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedRequest,
              com.sdkwork.communication.backend.v3.RequeueDeadLetterSharedChannelSyncTargetedResponse>(
                service, METHODID_REQUEUE_DEAD_LETTER_SHARED_CHANNEL_SYNC_TARGETED)))
        .addMethod(
          getTakeoverPendingSharedChannelSyncTargetedMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedRequest,
              com.sdkwork.communication.backend.v3.TakeoverPendingSharedChannelSyncTargetedResponse>(
                service, METHODID_TAKEOVER_PENDING_SHARED_CHANNEL_SYNC_TARGETED)))
        .build();
  }

  private static abstract class SocialRuntimeAdminServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    SocialRuntimeAdminServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.backend.v3.AdminService.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("SocialRuntimeAdminService");
    }
  }

  private static final class SocialRuntimeAdminServiceFileDescriptorSupplier
      extends SocialRuntimeAdminServiceBaseDescriptorSupplier {
    SocialRuntimeAdminServiceFileDescriptorSupplier() {}
  }

  private static final class SocialRuntimeAdminServiceMethodDescriptorSupplier
      extends SocialRuntimeAdminServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    SocialRuntimeAdminServiceMethodDescriptorSupplier(java.lang.String methodName) {
      this.methodName = methodName;
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.MethodDescriptor getMethodDescriptor() {
      return getServiceDescriptor().findMethodByName(methodName);
    }
  }

  private static volatile io.grpc.ServiceDescriptor serviceDescriptor;

  public static io.grpc.ServiceDescriptor getServiceDescriptor() {
    io.grpc.ServiceDescriptor result = serviceDescriptor;
    if (result == null) {
      synchronized (SocialRuntimeAdminServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new SocialRuntimeAdminServiceFileDescriptorSupplier())
              .addMethod(getClaimPendingSharedChannelSyncTargetedMethod())
              .addMethod(getListDeadLetterSharedChannelSyncMethod())
              .addMethod(getListDeliveredSharedChannelSyncMethod())
              .addMethod(getListDeliveryStateSharedChannelSyncMethod())
              .addMethod(getListPendingSharedChannelSyncMethod())
              .addMethod(getReclaimStalePendingSharedChannelSyncMethod())
              .addMethod(getReleasePendingSharedChannelSyncTargetedMethod())
              .addMethod(getRepairDerivedSnapshotMethod())
              .addMethod(getRepairSharedChannelSyncMethod())
              .addMethod(getRepublishPendingSharedChannelSyncTargetedMethod())
              .addMethod(getRequeueDeadLetterSharedChannelSyncMethod())
              .addMethod(getRequeueDeadLetterSharedChannelSyncTargetedMethod())
              .addMethod(getTakeoverPendingSharedChannelSyncTargetedMethod())
              .build();
        }
      }
    }
    return result;
  }
}
