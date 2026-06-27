package com.sdkwork.communication.backend.v3;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class CommunicationControlServiceGrpc {

  private CommunicationControlServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.backend.v3.CommunicationControlService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceRequest,
      com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceResponse> getRetrieveProtocolGovernanceMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveProtocolGovernance",
      requestType = com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceRequest,
      com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceResponse> getRetrieveProtocolGovernanceMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceRequest, com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceResponse> getRetrieveProtocolGovernanceMethod;
    if ((getRetrieveProtocolGovernanceMethod = CommunicationControlServiceGrpc.getRetrieveProtocolGovernanceMethod) == null) {
      synchronized (CommunicationControlServiceGrpc.class) {
        if ((getRetrieveProtocolGovernanceMethod = CommunicationControlServiceGrpc.getRetrieveProtocolGovernanceMethod) == null) {
          CommunicationControlServiceGrpc.getRetrieveProtocolGovernanceMethod = getRetrieveProtocolGovernanceMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceRequest, com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveProtocolGovernance"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CommunicationControlServiceMethodDescriptorSupplier("RetrieveProtocolGovernance"))
              .build();
        }
      }
    }
    return getRetrieveProtocolGovernanceMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryRequest,
      com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryResponse> getRetrieveProtocolRegistryMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveProtocolRegistry",
      requestType = com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryRequest,
      com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryResponse> getRetrieveProtocolRegistryMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryRequest, com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryResponse> getRetrieveProtocolRegistryMethod;
    if ((getRetrieveProtocolRegistryMethod = CommunicationControlServiceGrpc.getRetrieveProtocolRegistryMethod) == null) {
      synchronized (CommunicationControlServiceGrpc.class) {
        if ((getRetrieveProtocolRegistryMethod = CommunicationControlServiceGrpc.getRetrieveProtocolRegistryMethod) == null) {
          CommunicationControlServiceGrpc.getRetrieveProtocolRegistryMethod = getRetrieveProtocolRegistryMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryRequest, com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveProtocolRegistry"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CommunicationControlServiceMethodDescriptorSupplier("RetrieveProtocolRegistry"))
              .build();
        }
      }
    }
    return getRetrieveProtocolRegistryMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListProviderPoliciesRequest,
      com.sdkwork.communication.backend.v3.ListProviderPoliciesResponse> getListProviderPoliciesMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListProviderPolicies",
      requestType = com.sdkwork.communication.backend.v3.ListProviderPoliciesRequest.class,
      responseType = com.sdkwork.communication.backend.v3.ListProviderPoliciesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListProviderPoliciesRequest,
      com.sdkwork.communication.backend.v3.ListProviderPoliciesResponse> getListProviderPoliciesMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListProviderPoliciesRequest, com.sdkwork.communication.backend.v3.ListProviderPoliciesResponse> getListProviderPoliciesMethod;
    if ((getListProviderPoliciesMethod = CommunicationControlServiceGrpc.getListProviderPoliciesMethod) == null) {
      synchronized (CommunicationControlServiceGrpc.class) {
        if ((getListProviderPoliciesMethod = CommunicationControlServiceGrpc.getListProviderPoliciesMethod) == null) {
          CommunicationControlServiceGrpc.getListProviderPoliciesMethod = getListProviderPoliciesMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.ListProviderPoliciesRequest, com.sdkwork.communication.backend.v3.ListProviderPoliciesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListProviderPolicies"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ListProviderPoliciesRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ListProviderPoliciesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CommunicationControlServiceMethodDescriptorSupplier("ListProviderPolicies"))
              .build();
        }
      }
    }
    return getListProviderPoliciesMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.PreviewProviderPolicyRequest,
      com.sdkwork.communication.backend.v3.PreviewProviderPolicyResponse> getPreviewProviderPolicyMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "PreviewProviderPolicy",
      requestType = com.sdkwork.communication.backend.v3.PreviewProviderPolicyRequest.class,
      responseType = com.sdkwork.communication.backend.v3.PreviewProviderPolicyResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.PreviewProviderPolicyRequest,
      com.sdkwork.communication.backend.v3.PreviewProviderPolicyResponse> getPreviewProviderPolicyMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.PreviewProviderPolicyRequest, com.sdkwork.communication.backend.v3.PreviewProviderPolicyResponse> getPreviewProviderPolicyMethod;
    if ((getPreviewProviderPolicyMethod = CommunicationControlServiceGrpc.getPreviewProviderPolicyMethod) == null) {
      synchronized (CommunicationControlServiceGrpc.class) {
        if ((getPreviewProviderPolicyMethod = CommunicationControlServiceGrpc.getPreviewProviderPolicyMethod) == null) {
          CommunicationControlServiceGrpc.getPreviewProviderPolicyMethod = getPreviewProviderPolicyMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.PreviewProviderPolicyRequest, com.sdkwork.communication.backend.v3.PreviewProviderPolicyResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "PreviewProviderPolicy"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.PreviewProviderPolicyRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.PreviewProviderPolicyResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CommunicationControlServiceMethodDescriptorSupplier("PreviewProviderPolicy"))
              .build();
        }
      }
    }
    return getPreviewProviderPolicyMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RollbackProviderPolicyRequest,
      com.sdkwork.communication.backend.v3.RollbackProviderPolicyResponse> getRollbackProviderPolicyMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RollbackProviderPolicy",
      requestType = com.sdkwork.communication.backend.v3.RollbackProviderPolicyRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RollbackProviderPolicyResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RollbackProviderPolicyRequest,
      com.sdkwork.communication.backend.v3.RollbackProviderPolicyResponse> getRollbackProviderPolicyMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RollbackProviderPolicyRequest, com.sdkwork.communication.backend.v3.RollbackProviderPolicyResponse> getRollbackProviderPolicyMethod;
    if ((getRollbackProviderPolicyMethod = CommunicationControlServiceGrpc.getRollbackProviderPolicyMethod) == null) {
      synchronized (CommunicationControlServiceGrpc.class) {
        if ((getRollbackProviderPolicyMethod = CommunicationControlServiceGrpc.getRollbackProviderPolicyMethod) == null) {
          CommunicationControlServiceGrpc.getRollbackProviderPolicyMethod = getRollbackProviderPolicyMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RollbackProviderPolicyRequest, com.sdkwork.communication.backend.v3.RollbackProviderPolicyResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RollbackProviderPolicy"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RollbackProviderPolicyRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RollbackProviderPolicyResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CommunicationControlServiceMethodDescriptorSupplier("RollbackProviderPolicy"))
              .build();
        }
      }
    }
    return getRollbackProviderPolicyMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveProviderRegistryRequest,
      com.sdkwork.communication.backend.v3.RetrieveProviderRegistryResponse> getRetrieveProviderRegistryMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveProviderRegistry",
      requestType = com.sdkwork.communication.backend.v3.RetrieveProviderRegistryRequest.class,
      responseType = com.sdkwork.communication.backend.v3.RetrieveProviderRegistryResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveProviderRegistryRequest,
      com.sdkwork.communication.backend.v3.RetrieveProviderRegistryResponse> getRetrieveProviderRegistryMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.RetrieveProviderRegistryRequest, com.sdkwork.communication.backend.v3.RetrieveProviderRegistryResponse> getRetrieveProviderRegistryMethod;
    if ((getRetrieveProviderRegistryMethod = CommunicationControlServiceGrpc.getRetrieveProviderRegistryMethod) == null) {
      synchronized (CommunicationControlServiceGrpc.class) {
        if ((getRetrieveProviderRegistryMethod = CommunicationControlServiceGrpc.getRetrieveProviderRegistryMethod) == null) {
          CommunicationControlServiceGrpc.getRetrieveProviderRegistryMethod = getRetrieveProviderRegistryMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.RetrieveProviderRegistryRequest, com.sdkwork.communication.backend.v3.RetrieveProviderRegistryResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveProviderRegistry"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveProviderRegistryRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.RetrieveProviderRegistryResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CommunicationControlServiceMethodDescriptorSupplier("RetrieveProviderRegistry"))
              .build();
        }
      }
    }
    return getRetrieveProviderRegistryMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListControlProviderBindingsRequest,
      com.sdkwork.communication.backend.v3.ListControlProviderBindingsResponse> getListControlProviderBindingsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListControlProviderBindings",
      requestType = com.sdkwork.communication.backend.v3.ListControlProviderBindingsRequest.class,
      responseType = com.sdkwork.communication.backend.v3.ListControlProviderBindingsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListControlProviderBindingsRequest,
      com.sdkwork.communication.backend.v3.ListControlProviderBindingsResponse> getListControlProviderBindingsMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.ListControlProviderBindingsRequest, com.sdkwork.communication.backend.v3.ListControlProviderBindingsResponse> getListControlProviderBindingsMethod;
    if ((getListControlProviderBindingsMethod = CommunicationControlServiceGrpc.getListControlProviderBindingsMethod) == null) {
      synchronized (CommunicationControlServiceGrpc.class) {
        if ((getListControlProviderBindingsMethod = CommunicationControlServiceGrpc.getListControlProviderBindingsMethod) == null) {
          CommunicationControlServiceGrpc.getListControlProviderBindingsMethod = getListControlProviderBindingsMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.ListControlProviderBindingsRequest, com.sdkwork.communication.backend.v3.ListControlProviderBindingsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListControlProviderBindings"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ListControlProviderBindingsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.ListControlProviderBindingsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CommunicationControlServiceMethodDescriptorSupplier("ListControlProviderBindings"))
              .build();
        }
      }
    }
    return getListControlProviderBindingsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateControlProviderBindingRequest,
      com.sdkwork.communication.backend.v3.CreateControlProviderBindingResponse> getCreateControlProviderBindingMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateControlProviderBinding",
      requestType = com.sdkwork.communication.backend.v3.CreateControlProviderBindingRequest.class,
      responseType = com.sdkwork.communication.backend.v3.CreateControlProviderBindingResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateControlProviderBindingRequest,
      com.sdkwork.communication.backend.v3.CreateControlProviderBindingResponse> getCreateControlProviderBindingMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.backend.v3.CreateControlProviderBindingRequest, com.sdkwork.communication.backend.v3.CreateControlProviderBindingResponse> getCreateControlProviderBindingMethod;
    if ((getCreateControlProviderBindingMethod = CommunicationControlServiceGrpc.getCreateControlProviderBindingMethod) == null) {
      synchronized (CommunicationControlServiceGrpc.class) {
        if ((getCreateControlProviderBindingMethod = CommunicationControlServiceGrpc.getCreateControlProviderBindingMethod) == null) {
          CommunicationControlServiceGrpc.getCreateControlProviderBindingMethod = getCreateControlProviderBindingMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.backend.v3.CreateControlProviderBindingRequest, com.sdkwork.communication.backend.v3.CreateControlProviderBindingResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateControlProviderBinding"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.CreateControlProviderBindingRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.backend.v3.CreateControlProviderBindingResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CommunicationControlServiceMethodDescriptorSupplier("CreateControlProviderBinding"))
              .build();
        }
      }
    }
    return getCreateControlProviderBindingMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static CommunicationControlServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CommunicationControlServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CommunicationControlServiceStub>() {
        @java.lang.Override
        public CommunicationControlServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CommunicationControlServiceStub(channel, callOptions);
        }
      };
    return CommunicationControlServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static CommunicationControlServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CommunicationControlServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CommunicationControlServiceBlockingV2Stub>() {
        @java.lang.Override
        public CommunicationControlServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CommunicationControlServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return CommunicationControlServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static CommunicationControlServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CommunicationControlServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CommunicationControlServiceBlockingStub>() {
        @java.lang.Override
        public CommunicationControlServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CommunicationControlServiceBlockingStub(channel, callOptions);
        }
      };
    return CommunicationControlServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static CommunicationControlServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CommunicationControlServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CommunicationControlServiceFutureStub>() {
        @java.lang.Override
        public CommunicationControlServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CommunicationControlServiceFutureStub(channel, callOptions);
        }
      };
    return CommunicationControlServiceFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void retrieveProtocolGovernance(com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveProtocolGovernanceMethod(), responseObserver);
    }

    /**
     */
    default void retrieveProtocolRegistry(com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveProtocolRegistryMethod(), responseObserver);
    }

    /**
     */
    default void listProviderPolicies(com.sdkwork.communication.backend.v3.ListProviderPoliciesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListProviderPoliciesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListProviderPoliciesMethod(), responseObserver);
    }

    /**
     */
    default void previewProviderPolicy(com.sdkwork.communication.backend.v3.PreviewProviderPolicyRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.PreviewProviderPolicyResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getPreviewProviderPolicyMethod(), responseObserver);
    }

    /**
     */
    default void rollbackProviderPolicy(com.sdkwork.communication.backend.v3.RollbackProviderPolicyRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RollbackProviderPolicyResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRollbackProviderPolicyMethod(), responseObserver);
    }

    /**
     */
    default void retrieveProviderRegistry(com.sdkwork.communication.backend.v3.RetrieveProviderRegistryRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveProviderRegistryResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveProviderRegistryMethod(), responseObserver);
    }

    /**
     */
    default void listControlProviderBindings(com.sdkwork.communication.backend.v3.ListControlProviderBindingsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListControlProviderBindingsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListControlProviderBindingsMethod(), responseObserver);
    }

    /**
     */
    default void createControlProviderBinding(com.sdkwork.communication.backend.v3.CreateControlProviderBindingRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateControlProviderBindingResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateControlProviderBindingMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service CommunicationControlService.
   */
  public static abstract class CommunicationControlServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return CommunicationControlServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service CommunicationControlService.
   */
  public static final class CommunicationControlServiceStub
      extends io.grpc.stub.AbstractAsyncStub<CommunicationControlServiceStub> {
    private CommunicationControlServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CommunicationControlServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CommunicationControlServiceStub(channel, callOptions);
    }

    /**
     */
    public void retrieveProtocolGovernance(com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveProtocolGovernanceMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveProtocolRegistry(com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveProtocolRegistryMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void listProviderPolicies(com.sdkwork.communication.backend.v3.ListProviderPoliciesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListProviderPoliciesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListProviderPoliciesMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void previewProviderPolicy(com.sdkwork.communication.backend.v3.PreviewProviderPolicyRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.PreviewProviderPolicyResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getPreviewProviderPolicyMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void rollbackProviderPolicy(com.sdkwork.communication.backend.v3.RollbackProviderPolicyRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RollbackProviderPolicyResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRollbackProviderPolicyMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveProviderRegistry(com.sdkwork.communication.backend.v3.RetrieveProviderRegistryRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveProviderRegistryResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveProviderRegistryMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void listControlProviderBindings(com.sdkwork.communication.backend.v3.ListControlProviderBindingsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListControlProviderBindingsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListControlProviderBindingsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createControlProviderBinding(com.sdkwork.communication.backend.v3.CreateControlProviderBindingRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateControlProviderBindingResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateControlProviderBindingMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service CommunicationControlService.
   */
  public static final class CommunicationControlServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<CommunicationControlServiceBlockingV2Stub> {
    private CommunicationControlServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CommunicationControlServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CommunicationControlServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceResponse retrieveProtocolGovernance(com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveProtocolGovernanceMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryResponse retrieveProtocolRegistry(com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveProtocolRegistryMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ListProviderPoliciesResponse listProviderPolicies(com.sdkwork.communication.backend.v3.ListProviderPoliciesRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListProviderPoliciesMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.PreviewProviderPolicyResponse previewProviderPolicy(com.sdkwork.communication.backend.v3.PreviewProviderPolicyRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getPreviewProviderPolicyMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RollbackProviderPolicyResponse rollbackProviderPolicy(com.sdkwork.communication.backend.v3.RollbackProviderPolicyRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRollbackProviderPolicyMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveProviderRegistryResponse retrieveProviderRegistry(com.sdkwork.communication.backend.v3.RetrieveProviderRegistryRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveProviderRegistryMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ListControlProviderBindingsResponse listControlProviderBindings(com.sdkwork.communication.backend.v3.ListControlProviderBindingsRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListControlProviderBindingsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.CreateControlProviderBindingResponse createControlProviderBinding(com.sdkwork.communication.backend.v3.CreateControlProviderBindingRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateControlProviderBindingMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service CommunicationControlService.
   */
  public static final class CommunicationControlServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<CommunicationControlServiceBlockingStub> {
    private CommunicationControlServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CommunicationControlServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CommunicationControlServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceResponse retrieveProtocolGovernance(com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveProtocolGovernanceMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryResponse retrieveProtocolRegistry(com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveProtocolRegistryMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ListProviderPoliciesResponse listProviderPolicies(com.sdkwork.communication.backend.v3.ListProviderPoliciesRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListProviderPoliciesMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.PreviewProviderPolicyResponse previewProviderPolicy(com.sdkwork.communication.backend.v3.PreviewProviderPolicyRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getPreviewProviderPolicyMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RollbackProviderPolicyResponse rollbackProviderPolicy(com.sdkwork.communication.backend.v3.RollbackProviderPolicyRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRollbackProviderPolicyMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.RetrieveProviderRegistryResponse retrieveProviderRegistry(com.sdkwork.communication.backend.v3.RetrieveProviderRegistryRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveProviderRegistryMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.ListControlProviderBindingsResponse listControlProviderBindings(com.sdkwork.communication.backend.v3.ListControlProviderBindingsRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListControlProviderBindingsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.backend.v3.CreateControlProviderBindingResponse createControlProviderBinding(com.sdkwork.communication.backend.v3.CreateControlProviderBindingRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateControlProviderBindingMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service CommunicationControlService.
   */
  public static final class CommunicationControlServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<CommunicationControlServiceFutureStub> {
    private CommunicationControlServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CommunicationControlServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CommunicationControlServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceResponse> retrieveProtocolGovernance(
        com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveProtocolGovernanceMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryResponse> retrieveProtocolRegistry(
        com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveProtocolRegistryMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.ListProviderPoliciesResponse> listProviderPolicies(
        com.sdkwork.communication.backend.v3.ListProviderPoliciesRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListProviderPoliciesMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.PreviewProviderPolicyResponse> previewProviderPolicy(
        com.sdkwork.communication.backend.v3.PreviewProviderPolicyRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getPreviewProviderPolicyMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RollbackProviderPolicyResponse> rollbackProviderPolicy(
        com.sdkwork.communication.backend.v3.RollbackProviderPolicyRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRollbackProviderPolicyMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.RetrieveProviderRegistryResponse> retrieveProviderRegistry(
        com.sdkwork.communication.backend.v3.RetrieveProviderRegistryRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveProviderRegistryMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.ListControlProviderBindingsResponse> listControlProviderBindings(
        com.sdkwork.communication.backend.v3.ListControlProviderBindingsRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListControlProviderBindingsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.backend.v3.CreateControlProviderBindingResponse> createControlProviderBinding(
        com.sdkwork.communication.backend.v3.CreateControlProviderBindingRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateControlProviderBindingMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_RETRIEVE_PROTOCOL_GOVERNANCE = 0;
  private static final int METHODID_RETRIEVE_PROTOCOL_REGISTRY = 1;
  private static final int METHODID_LIST_PROVIDER_POLICIES = 2;
  private static final int METHODID_PREVIEW_PROVIDER_POLICY = 3;
  private static final int METHODID_ROLLBACK_PROVIDER_POLICY = 4;
  private static final int METHODID_RETRIEVE_PROVIDER_REGISTRY = 5;
  private static final int METHODID_LIST_CONTROL_PROVIDER_BINDINGS = 6;
  private static final int METHODID_CREATE_CONTROL_PROVIDER_BINDING = 7;

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
        case METHODID_RETRIEVE_PROTOCOL_GOVERNANCE:
          serviceImpl.retrieveProtocolGovernance((com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_PROTOCOL_REGISTRY:
          serviceImpl.retrieveProtocolRegistry((com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryResponse>) responseObserver);
          break;
        case METHODID_LIST_PROVIDER_POLICIES:
          serviceImpl.listProviderPolicies((com.sdkwork.communication.backend.v3.ListProviderPoliciesRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListProviderPoliciesResponse>) responseObserver);
          break;
        case METHODID_PREVIEW_PROVIDER_POLICY:
          serviceImpl.previewProviderPolicy((com.sdkwork.communication.backend.v3.PreviewProviderPolicyRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.PreviewProviderPolicyResponse>) responseObserver);
          break;
        case METHODID_ROLLBACK_PROVIDER_POLICY:
          serviceImpl.rollbackProviderPolicy((com.sdkwork.communication.backend.v3.RollbackProviderPolicyRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RollbackProviderPolicyResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_PROVIDER_REGISTRY:
          serviceImpl.retrieveProviderRegistry((com.sdkwork.communication.backend.v3.RetrieveProviderRegistryRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.RetrieveProviderRegistryResponse>) responseObserver);
          break;
        case METHODID_LIST_CONTROL_PROVIDER_BINDINGS:
          serviceImpl.listControlProviderBindings((com.sdkwork.communication.backend.v3.ListControlProviderBindingsRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.ListControlProviderBindingsResponse>) responseObserver);
          break;
        case METHODID_CREATE_CONTROL_PROVIDER_BINDING:
          serviceImpl.createControlProviderBinding((com.sdkwork.communication.backend.v3.CreateControlProviderBindingRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.backend.v3.CreateControlProviderBindingResponse>) responseObserver);
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
          getRetrieveProtocolGovernanceMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceRequest,
              com.sdkwork.communication.backend.v3.RetrieveProtocolGovernanceResponse>(
                service, METHODID_RETRIEVE_PROTOCOL_GOVERNANCE)))
        .addMethod(
          getRetrieveProtocolRegistryMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryRequest,
              com.sdkwork.communication.backend.v3.RetrieveProtocolRegistryResponse>(
                service, METHODID_RETRIEVE_PROTOCOL_REGISTRY)))
        .addMethod(
          getListProviderPoliciesMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.ListProviderPoliciesRequest,
              com.sdkwork.communication.backend.v3.ListProviderPoliciesResponse>(
                service, METHODID_LIST_PROVIDER_POLICIES)))
        .addMethod(
          getPreviewProviderPolicyMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.PreviewProviderPolicyRequest,
              com.sdkwork.communication.backend.v3.PreviewProviderPolicyResponse>(
                service, METHODID_PREVIEW_PROVIDER_POLICY)))
        .addMethod(
          getRollbackProviderPolicyMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RollbackProviderPolicyRequest,
              com.sdkwork.communication.backend.v3.RollbackProviderPolicyResponse>(
                service, METHODID_ROLLBACK_PROVIDER_POLICY)))
        .addMethod(
          getRetrieveProviderRegistryMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.RetrieveProviderRegistryRequest,
              com.sdkwork.communication.backend.v3.RetrieveProviderRegistryResponse>(
                service, METHODID_RETRIEVE_PROVIDER_REGISTRY)))
        .addMethod(
          getListControlProviderBindingsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.ListControlProviderBindingsRequest,
              com.sdkwork.communication.backend.v3.ListControlProviderBindingsResponse>(
                service, METHODID_LIST_CONTROL_PROVIDER_BINDINGS)))
        .addMethod(
          getCreateControlProviderBindingMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.backend.v3.CreateControlProviderBindingRequest,
              com.sdkwork.communication.backend.v3.CreateControlProviderBindingResponse>(
                service, METHODID_CREATE_CONTROL_PROVIDER_BINDING)))
        .build();
  }

  private static abstract class CommunicationControlServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    CommunicationControlServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.backend.v3.AdminService.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("CommunicationControlService");
    }
  }

  private static final class CommunicationControlServiceFileDescriptorSupplier
      extends CommunicationControlServiceBaseDescriptorSupplier {
    CommunicationControlServiceFileDescriptorSupplier() {}
  }

  private static final class CommunicationControlServiceMethodDescriptorSupplier
      extends CommunicationControlServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    CommunicationControlServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (CommunicationControlServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new CommunicationControlServiceFileDescriptorSupplier())
              .addMethod(getRetrieveProtocolGovernanceMethod())
              .addMethod(getRetrieveProtocolRegistryMethod())
              .addMethod(getListProviderPoliciesMethod())
              .addMethod(getPreviewProviderPolicyMethod())
              .addMethod(getRollbackProviderPolicyMethod())
              .addMethod(getRetrieveProviderRegistryMethod())
              .addMethod(getListControlProviderBindingsMethod())
              .addMethod(getCreateControlProviderBindingMethod())
              .build();
        }
      }
    }
    return result;
  }
}
