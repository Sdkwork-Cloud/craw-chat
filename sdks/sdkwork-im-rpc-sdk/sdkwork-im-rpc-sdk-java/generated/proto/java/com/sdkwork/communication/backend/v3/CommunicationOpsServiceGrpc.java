package com.sdkwork.communication.backend.v3;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class CommunicationOpsServiceGrpc {

  private CommunicationOpsServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.backend.v3.CommunicationOpsService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveHealthRequest,
      com.sdkwork.communication.backend.v3.RetrieveHealthResponse> getRetrieveHealthMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveHealth",
      requestType = com.sdkwork.communication.backend.v3.RetrieveHealthRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RetrieveHealthResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveHealthRequest,
      com.sdkwork.communication.backend.v3.RetrieveHealthResponse> getRetrieveHealthMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveHealthRequest, com.sdkwork.communication.backend.v3.RetrieveHealthResponse> getRetrieveHealthMethod;
    if ((getRetrieveHealthMethod = CommunicationOpsServiceGrpc.getRetrieveHealthMethod) == null) {
      synchronized (CommunicationOpsServiceGrpc.class) {
        if ((getRetrieveHealthMethod = CommunicationOpsServiceGrpc.getRetrieveHealthMethod) == null) {
          CommunicationOpsServiceGrpc.getRetrieveHealthMethod = getRetrieveHealthMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RetrieveHealthRequest, com.sdkwork.communication.backend.v3.RetrieveHealthResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveHealth"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveHealthRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveHealthResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CommunicationOpsServiceMethodDescriptorSupplier("RetrieveHealth"))
              .build();
        }
      }
    }
    return getRetrieveHealthMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveClusterRequest,
      com.sdkwork.communication.backend.v3.RetrieveClusterResponse> getRetrieveClusterMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveCluster",
      requestType = com.sdkwork.communication.backend.v3.RetrieveClusterRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RetrieveClusterResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveClusterRequest,
      com.sdkwork.communication.backend.v3.RetrieveClusterResponse> getRetrieveClusterMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveClusterRequest, com.sdkwork.communication.backend.v3.RetrieveClusterResponse> getRetrieveClusterMethod;
    if ((getRetrieveClusterMethod = CommunicationOpsServiceGrpc.getRetrieveClusterMethod) == null) {
      synchronized (CommunicationOpsServiceGrpc.class) {
        if ((getRetrieveClusterMethod = CommunicationOpsServiceGrpc.getRetrieveClusterMethod) == null) {
          CommunicationOpsServiceGrpc.getRetrieveClusterMethod = getRetrieveClusterMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RetrieveClusterRequest, com.sdkwork.communication.backend.v3.RetrieveClusterResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveCluster"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveClusterRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveClusterResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CommunicationOpsServiceMethodDescriptorSupplier("RetrieveCluster"))
              .build();
        }
      }
    }
    return getRetrieveClusterMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveLagRequest,
      com.sdkwork.communication.backend.v3.RetrieveLagResponse> getRetrieveLagMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveLag",
      requestType = com.sdkwork.communication.backend.v3.RetrieveLagRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RetrieveLagResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveLagRequest,
      com.sdkwork.communication.backend.v3.RetrieveLagResponse> getRetrieveLagMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveLagRequest, com.sdkwork.communication.backend.v3.RetrieveLagResponse> getRetrieveLagMethod;
    if ((getRetrieveLagMethod = CommunicationOpsServiceGrpc.getRetrieveLagMethod) == null) {
      synchronized (CommunicationOpsServiceGrpc.class) {
        if ((getRetrieveLagMethod = CommunicationOpsServiceGrpc.getRetrieveLagMethod) == null) {
          CommunicationOpsServiceGrpc.getRetrieveLagMethod = getRetrieveLagMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RetrieveLagRequest, com.sdkwork.communication.backend.v3.RetrieveLagResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveLag"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveLagRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveLagResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CommunicationOpsServiceMethodDescriptorSupplier("RetrieveLag"))
              .build();
        }
      }
    }
    return getRetrieveLagMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveReplayStatusRequest,
      com.sdkwork.communication.backend.v3.RetrieveReplayStatusResponse> getRetrieveReplayStatusMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveReplayStatus",
      requestType = com.sdkwork.communication.backend.v3.RetrieveReplayStatusRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RetrieveReplayStatusResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveReplayStatusRequest,
      com.sdkwork.communication.backend.v3.RetrieveReplayStatusResponse> getRetrieveReplayStatusMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveReplayStatusRequest, com.sdkwork.communication.backend.v3.RetrieveReplayStatusResponse> getRetrieveReplayStatusMethod;
    if ((getRetrieveReplayStatusMethod = CommunicationOpsServiceGrpc.getRetrieveReplayStatusMethod) == null) {
      synchronized (CommunicationOpsServiceGrpc.class) {
        if ((getRetrieveReplayStatusMethod = CommunicationOpsServiceGrpc.getRetrieveReplayStatusMethod) == null) {
          CommunicationOpsServiceGrpc.getRetrieveReplayStatusMethod = getRetrieveReplayStatusMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RetrieveReplayStatusRequest, com.sdkwork.communication.backend.v3.RetrieveReplayStatusResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveReplayStatus"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveReplayStatusRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveReplayStatusResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CommunicationOpsServiceMethodDescriptorSupplier("RetrieveReplayStatus"))
              .build();
        }
      }
    }
    return getRetrieveReplayStatusMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessRequest,
      com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessResponse> getRetrieveCommercialReadinessMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveCommercialReadiness",
      requestType = com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessRequest,
      com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessResponse> getRetrieveCommercialReadinessMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessRequest, com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessResponse> getRetrieveCommercialReadinessMethod;
    if ((getRetrieveCommercialReadinessMethod = CommunicationOpsServiceGrpc.getRetrieveCommercialReadinessMethod) == null) {
      synchronized (CommunicationOpsServiceGrpc.class) {
        if ((getRetrieveCommercialReadinessMethod = CommunicationOpsServiceGrpc.getRetrieveCommercialReadinessMethod) == null) {
          CommunicationOpsServiceGrpc.getRetrieveCommercialReadinessMethod = getRetrieveCommercialReadinessMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessRequest, com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveCommercialReadiness"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CommunicationOpsServiceMethodDescriptorSupplier("RetrieveCommercialReadiness"))
              .build();
        }
      }
    }
    return getRetrieveCommercialReadinessMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveRuntimeDirRequest,
      com.sdkwork.communication.backend.v3.RetrieveRuntimeDirResponse> getRetrieveRuntimeDirMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveRuntimeDir",
      requestType = com.sdkwork.communication.backend.v3.RetrieveRuntimeDirRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RetrieveRuntimeDirResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveRuntimeDirRequest,
      com.sdkwork.communication.backend.v3.RetrieveRuntimeDirResponse> getRetrieveRuntimeDirMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveRuntimeDirRequest, com.sdkwork.communication.backend.v3.RetrieveRuntimeDirResponse> getRetrieveRuntimeDirMethod;
    if ((getRetrieveRuntimeDirMethod = CommunicationOpsServiceGrpc.getRetrieveRuntimeDirMethod) == null) {
      synchronized (CommunicationOpsServiceGrpc.class) {
        if ((getRetrieveRuntimeDirMethod = CommunicationOpsServiceGrpc.getRetrieveRuntimeDirMethod) == null) {
          CommunicationOpsServiceGrpc.getRetrieveRuntimeDirMethod = getRetrieveRuntimeDirMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RetrieveRuntimeDirRequest, com.sdkwork.communication.backend.v3.RetrieveRuntimeDirResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveRuntimeDir"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveRuntimeDirRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveRuntimeDirResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CommunicationOpsServiceMethodDescriptorSupplier("RetrieveRuntimeDir"))
              .build();
        }
      }
    }
    return getRetrieveRuntimeDirMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListOpsProviderBindingsRequest,
      com.sdkwork.communication.backend.v3.ListOpsProviderBindingsResponse> getListOpsProviderBindingsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListOpsProviderBindings",
      requestType = com.sdkwork.communication.backend.v3.ListOpsProviderBindingsRequest.class,
      responseType = com.sdkwork.communication.backend.v3.ListOpsProviderBindingsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListOpsProviderBindingsRequest,
      com.sdkwork.communication.backend.v3.ListOpsProviderBindingsResponse> getListOpsProviderBindingsMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListOpsProviderBindingsRequest, com.sdkwork.communication.backend.v3.ListOpsProviderBindingsResponse> getListOpsProviderBindingsMethod;
    if ((getListOpsProviderBindingsMethod = CommunicationOpsServiceGrpc.getListOpsProviderBindingsMethod) == null) {
      synchronized (CommunicationOpsServiceGrpc.class) {
        if ((getListOpsProviderBindingsMethod = CommunicationOpsServiceGrpc.getListOpsProviderBindingsMethod) == null) {
          CommunicationOpsServiceGrpc.getListOpsProviderBindingsMethod = getListOpsProviderBindingsMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.ListOpsProviderBindingsRequest, com.sdkwork.communication.backend.v3.ListOpsProviderBindingsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListOpsProviderBindings"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ListOpsProviderBindingsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ListOpsProviderBindingsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CommunicationOpsServiceMethodDescriptorSupplier("ListOpsProviderBindings"))
              .build();
        }
      }
    }
    return getListOpsProviderBindingsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftRequest,
      com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftResponse> getRetrieveProviderBindingDriftMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveProviderBindingDrift",
      requestType = com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftRequest,
      com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftResponse> getRetrieveProviderBindingDriftMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftRequest, com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftResponse> getRetrieveProviderBindingDriftMethod;
    if ((getRetrieveProviderBindingDriftMethod = CommunicationOpsServiceGrpc.getRetrieveProviderBindingDriftMethod) == null) {
      synchronized (CommunicationOpsServiceGrpc.class) {
        if ((getRetrieveProviderBindingDriftMethod = CommunicationOpsServiceGrpc.getRetrieveProviderBindingDriftMethod) == null) {
          CommunicationOpsServiceGrpc.getRetrieveProviderBindingDriftMethod = getRetrieveProviderBindingDriftMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftRequest, com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveProviderBindingDrift"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CommunicationOpsServiceMethodDescriptorSupplier("RetrieveProviderBindingDrift"))
              .build();
        }
      }
    }
    return getRetrieveProviderBindingDriftMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveDiagnosticsRequest,
      com.sdkwork.communication.backend.v3.RetrieveDiagnosticsResponse> getRetrieveDiagnosticsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveDiagnostics",
      requestType = com.sdkwork.communication.backend.v3.RetrieveDiagnosticsRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RetrieveDiagnosticsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveDiagnosticsRequest,
      com.sdkwork.communication.backend.v3.RetrieveDiagnosticsResponse> getRetrieveDiagnosticsMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveDiagnosticsRequest, com.sdkwork.communication.backend.v3.RetrieveDiagnosticsResponse> getRetrieveDiagnosticsMethod;
    if ((getRetrieveDiagnosticsMethod = CommunicationOpsServiceGrpc.getRetrieveDiagnosticsMethod) == null) {
      synchronized (CommunicationOpsServiceGrpc.class) {
        if ((getRetrieveDiagnosticsMethod = CommunicationOpsServiceGrpc.getRetrieveDiagnosticsMethod) == null) {
          CommunicationOpsServiceGrpc.getRetrieveDiagnosticsMethod = getRetrieveDiagnosticsMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RetrieveDiagnosticsRequest, com.sdkwork.communication.backend.v3.RetrieveDiagnosticsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveDiagnostics"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveDiagnosticsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveDiagnosticsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CommunicationOpsServiceMethodDescriptorSupplier("RetrieveDiagnostics"))
              .build();
        }
      }
    }
    return getRetrieveDiagnosticsMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static CommunicationOpsServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CommunicationOpsServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CommunicationOpsServiceStub>() {
        @java.lang.Override
        public CommunicationOpsServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CommunicationOpsServiceStub(channel, callOptions);
        }
      };
    return CommunicationOpsServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static CommunicationOpsServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CommunicationOpsServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CommunicationOpsServiceBlockingV2Stub>() {
        @java.lang.Override
        public CommunicationOpsServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CommunicationOpsServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return CommunicationOpsServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static CommunicationOpsServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CommunicationOpsServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CommunicationOpsServiceBlockingStub>() {
        @java.lang.Override
        public CommunicationOpsServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CommunicationOpsServiceBlockingStub(channel, callOptions);
        }
      };
    return CommunicationOpsServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static CommunicationOpsServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CommunicationOpsServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CommunicationOpsServiceFutureStub>() {
        @java.lang.Override
        public CommunicationOpsServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CommunicationOpsServiceFutureStub(channel, callOptions);
        }
      };
    return CommunicationOpsServiceFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void retrieveHealth(com.sdkwork.communication.backend.v3.RetrieveHealthRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveHealthResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveHealthMethod(), responseObserver);
    }

    /**
     */
    default void retrieveCluster(com.sdkwork.communication.backend.v3.RetrieveClusterRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveClusterResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveClusterMethod(), responseObserver);
    }

    /**
     */
    default void retrieveLag(com.sdkwork.communication.backend.v3.RetrieveLagRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveLagResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveLagMethod(), responseObserver);
    }

    /**
     */
    default void retrieveReplayStatus(com.sdkwork.communication.backend.v3.RetrieveReplayStatusRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveReplayStatusResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveReplayStatusMethod(), responseObserver);
    }

    /**
     */
    default void retrieveCommercialReadiness(com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveCommercialReadinessMethod(), responseObserver);
    }

    /**
     */
    default void retrieveRuntimeDir(com.sdkwork.communication.backend.v3.RetrieveRuntimeDirRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveRuntimeDirResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveRuntimeDirMethod(), responseObserver);
    }

    /**
     */
    default void listOpsProviderBindings(com.sdkwork.communication.backend.v3.ListOpsProviderBindingsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListOpsProviderBindingsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListOpsProviderBindingsMethod(), responseObserver);
    }

    /**
     */
    default void retrieveProviderBindingDrift(com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveProviderBindingDriftMethod(), responseObserver);
    }

    /**
     */
    default void retrieveDiagnostics(com.sdkwork.communication.backend.v3.RetrieveDiagnosticsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveDiagnosticsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveDiagnosticsMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service CommunicationOpsService.
   */
  public static abstract class CommunicationOpsServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return CommunicationOpsServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service CommunicationOpsService.
   */
  public static final class CommunicationOpsServiceStub
      extends io.grpc.stub.AbstractAsyncStub<CommunicationOpsServiceStub> {
    private CommunicationOpsServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CommunicationOpsServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CommunicationOpsServiceStub(channel, callOptions);
    }

    /**
     */
    public void retrieveHealth(com.sdkwork.communication.backend.v3.RetrieveHealthRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveHealthResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveHealthMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveCluster(com.sdkwork.communication.backend.v3.RetrieveClusterRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveClusterResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveClusterMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveLag(com.sdkwork.communication.backend.v3.RetrieveLagRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveLagResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveLagMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveReplayStatus(com.sdkwork.communication.backend.v3.RetrieveReplayStatusRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveReplayStatusResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveReplayStatusMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveCommercialReadiness(com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveCommercialReadinessMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveRuntimeDir(com.sdkwork.communication.backend.v3.RetrieveRuntimeDirRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveRuntimeDirResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveRuntimeDirMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void listOpsProviderBindings(com.sdkwork.communication.backend.v3.ListOpsProviderBindingsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListOpsProviderBindingsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListOpsProviderBindingsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveProviderBindingDrift(com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveProviderBindingDriftMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveDiagnostics(com.sdkwork.communication.backend.v3.RetrieveDiagnosticsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveDiagnosticsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveDiagnosticsMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service CommunicationOpsService.
   */
  public static final class CommunicationOpsServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<CommunicationOpsServiceBlockingV2Stub> {
    private CommunicationOpsServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CommunicationOpsServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CommunicationOpsServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveHealthResponse retrieveHealth(com.sdkwork.communication.backend.v3.RetrieveHealthRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveHealthMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveClusterResponse retrieveCluster(com.sdkwork.communication.backend.v3.RetrieveClusterRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveClusterMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveLagResponse retrieveLag(com.sdkwork.communication.backend.v3.RetrieveLagRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveLagMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveReplayStatusResponse retrieveReplayStatus(com.sdkwork.communication.backend.v3.RetrieveReplayStatusRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveReplayStatusMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessResponse retrieveCommercialReadiness(com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveCommercialReadinessMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveRuntimeDirResponse retrieveRuntimeDir(com.sdkwork.communication.backend.v3.RetrieveRuntimeDirRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveRuntimeDirMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ListOpsProviderBindingsResponse listOpsProviderBindings(com.sdkwork.communication.backend.v3.ListOpsProviderBindingsRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListOpsProviderBindingsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftResponse retrieveProviderBindingDrift(com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveProviderBindingDriftMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveDiagnosticsResponse retrieveDiagnostics(com.sdkwork.communication.backend.v3.RetrieveDiagnosticsRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveDiagnosticsMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service CommunicationOpsService.
   */
  public static final class CommunicationOpsServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<CommunicationOpsServiceBlockingStub> {
    private CommunicationOpsServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CommunicationOpsServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CommunicationOpsServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveHealthResponse retrieveHealth(com.sdkwork.communication.backend.v3.RetrieveHealthRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveHealthMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveClusterResponse retrieveCluster(com.sdkwork.communication.backend.v3.RetrieveClusterRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveClusterMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveLagResponse retrieveLag(com.sdkwork.communication.backend.v3.RetrieveLagRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveLagMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveReplayStatusResponse retrieveReplayStatus(com.sdkwork.communication.backend.v3.RetrieveReplayStatusRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveReplayStatusMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessResponse retrieveCommercialReadiness(com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveCommercialReadinessMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveRuntimeDirResponse retrieveRuntimeDir(com.sdkwork.communication.backend.v3.RetrieveRuntimeDirRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveRuntimeDirMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ListOpsProviderBindingsResponse listOpsProviderBindings(com.sdkwork.communication.backend.v3.ListOpsProviderBindingsRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListOpsProviderBindingsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftResponse retrieveProviderBindingDrift(com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveProviderBindingDriftMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveDiagnosticsResponse retrieveDiagnostics(com.sdkwork.communication.backend.v3.RetrieveDiagnosticsRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveDiagnosticsMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service CommunicationOpsService.
   */
  public static final class CommunicationOpsServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<CommunicationOpsServiceFutureStub> {
    private CommunicationOpsServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CommunicationOpsServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CommunicationOpsServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RetrieveHealthResponse> retrieveHealth(
        com.sdkwork.communication.backend.v3.RetrieveHealthRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveHealthMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RetrieveClusterResponse> retrieveCluster(
        com.sdkwork.communication.backend.v3.RetrieveClusterRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveClusterMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RetrieveLagResponse> retrieveLag(
        com.sdkwork.communication.backend.v3.RetrieveLagRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveLagMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RetrieveReplayStatusResponse> retrieveReplayStatus(
        com.sdkwork.communication.backend.v3.RetrieveReplayStatusRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveReplayStatusMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessResponse> retrieveCommercialReadiness(
        com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveCommercialReadinessMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RetrieveRuntimeDirResponse> retrieveRuntimeDir(
        com.sdkwork.communication.backend.v3.RetrieveRuntimeDirRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveRuntimeDirMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.ListOpsProviderBindingsResponse> listOpsProviderBindings(
        com.sdkwork.communication.backend.v3.ListOpsProviderBindingsRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListOpsProviderBindingsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftResponse> retrieveProviderBindingDrift(
        com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveProviderBindingDriftMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RetrieveDiagnosticsResponse> retrieveDiagnostics(
        com.sdkwork.communication.backend.v3.RetrieveDiagnosticsRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveDiagnosticsMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_RETRIEVE_HEALTH = 0;
  private static final int METHODID_RETRIEVE_CLUSTER = 1;
  private static final int METHODID_RETRIEVE_LAG = 2;
  private static final int METHODID_RETRIEVE_REPLAY_STATUS = 3;
  private static final int METHODID_RETRIEVE_COMMERCIAL_READINESS = 4;
  private static final int METHODID_RETRIEVE_RUNTIME_DIR = 5;
  private static final int METHODID_LIST_OPS_PROVIDER_BINDINGS = 6;
  private static final int METHODID_RETRIEVE_PROVIDER_BINDING_DRIFT = 7;
  private static final int METHODID_RETRIEVE_DIAGNOSTICS = 8;

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
        case METHODID_RETRIEVE_HEALTH:
          serviceImpl.retrieveHealth((com.sdkwork.communication.backend.v3.RetrieveHealthRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveHealthResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_CLUSTER:
          serviceImpl.retrieveCluster((com.sdkwork.communication.backend.v3.RetrieveClusterRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveClusterResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_LAG:
          serviceImpl.retrieveLag((com.sdkwork.communication.backend.v3.RetrieveLagRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveLagResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_REPLAY_STATUS:
          serviceImpl.retrieveReplayStatus((com.sdkwork.communication.backend.v3.RetrieveReplayStatusRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveReplayStatusResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_COMMERCIAL_READINESS:
          serviceImpl.retrieveCommercialReadiness((com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_RUNTIME_DIR:
          serviceImpl.retrieveRuntimeDir((com.sdkwork.communication.backend.v3.RetrieveRuntimeDirRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveRuntimeDirResponse>) responseObserver);
          break;
        case METHODID_LIST_OPS_PROVIDER_BINDINGS:
          serviceImpl.listOpsProviderBindings((com.sdkwork.communication.backend.v3.ListOpsProviderBindingsRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListOpsProviderBindingsResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_PROVIDER_BINDING_DRIFT:
          serviceImpl.retrieveProviderBindingDrift((com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_DIAGNOSTICS:
          serviceImpl.retrieveDiagnostics((com.sdkwork.communication.backend.v3.RetrieveDiagnosticsRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveDiagnosticsResponse>) responseObserver);
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
          getRetrieveHealthMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RetrieveHealthRequest,
              com.sdkwork.communication.backend.v3.RetrieveHealthResponse>(
                service, METHODID_RETRIEVE_HEALTH)))
        .addMethod(
          getRetrieveClusterMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RetrieveClusterRequest,
              com.sdkwork.communication.backend.v3.RetrieveClusterResponse>(
                service, METHODID_RETRIEVE_CLUSTER)))
        .addMethod(
          getRetrieveLagMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RetrieveLagRequest,
              com.sdkwork.communication.backend.v3.RetrieveLagResponse>(
                service, METHODID_RETRIEVE_LAG)))
        .addMethod(
          getRetrieveReplayStatusMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RetrieveReplayStatusRequest,
              com.sdkwork.communication.backend.v3.RetrieveReplayStatusResponse>(
                service, METHODID_RETRIEVE_REPLAY_STATUS)))
        .addMethod(
          getRetrieveCommercialReadinessMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessRequest,
              com.sdkwork.communication.backend.v3.RetrieveCommercialReadinessResponse>(
                service, METHODID_RETRIEVE_COMMERCIAL_READINESS)))
        .addMethod(
          getRetrieveRuntimeDirMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RetrieveRuntimeDirRequest,
              com.sdkwork.communication.backend.v3.RetrieveRuntimeDirResponse>(
                service, METHODID_RETRIEVE_RUNTIME_DIR)))
        .addMethod(
          getListOpsProviderBindingsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.ListOpsProviderBindingsRequest,
              com.sdkwork.communication.backend.v3.ListOpsProviderBindingsResponse>(
                service, METHODID_LIST_OPS_PROVIDER_BINDINGS)))
        .addMethod(
          getRetrieveProviderBindingDriftMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftRequest,
              com.sdkwork.communication.backend.v3.RetrieveProviderBindingDriftResponse>(
                service, METHODID_RETRIEVE_PROVIDER_BINDING_DRIFT)))
        .addMethod(
          getRetrieveDiagnosticsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RetrieveDiagnosticsRequest,
              com.sdkwork.communication.backend.v3.RetrieveDiagnosticsResponse>(
                service, METHODID_RETRIEVE_DIAGNOSTICS)))
        .build();
  }

  private static abstract class CommunicationOpsServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    CommunicationOpsServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.backend.v3.AdminService.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("CommunicationOpsService");
    }
  }

  private static final class CommunicationOpsServiceFileDescriptorSupplier
      extends CommunicationOpsServiceBaseDescriptorSupplier {
    CommunicationOpsServiceFileDescriptorSupplier() {}
  }

  private static final class CommunicationOpsServiceMethodDescriptorSupplier
      extends CommunicationOpsServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    CommunicationOpsServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (CommunicationOpsServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new CommunicationOpsServiceFileDescriptorSupplier())
              .addMethod(getRetrieveHealthMethod())
              .addMethod(getRetrieveClusterMethod())
              .addMethod(getRetrieveLagMethod())
              .addMethod(getRetrieveReplayStatusMethod())
              .addMethod(getRetrieveCommercialReadinessMethod())
              .addMethod(getRetrieveRuntimeDirMethod())
              .addMethod(getListOpsProviderBindingsMethod())
              .addMethod(getRetrieveProviderBindingDriftMethod())
              .addMethod(getRetrieveDiagnosticsMethod())
              .build();
        }
      }
    }
    return result;
  }
}
