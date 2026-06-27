package com.sdkwork.communication.app.v3;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class CallServiceGrpc {

  private CallServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.app.v3.CallService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateCallSessionRequest,
      com.sdkwork.communication.app.v3.CreateCallSessionResponse> getCreateCallSessionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateCallSession",
      requestType = com.sdkwork.communication.app.v3.CreateCallSessionRequest.class,
      responseType = com.sdkwork.communication.app.v3.CreateCallSessionResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateCallSessionRequest,
      com.sdkwork.communication.app.v3.CreateCallSessionResponse> getCreateCallSessionMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateCallSessionRequest, com.sdkwork.communication.app.v3.CreateCallSessionResponse> getCreateCallSessionMethod;
    if ((getCreateCallSessionMethod = CallServiceGrpc.getCreateCallSessionMethod) == null) {
      synchronized (CallServiceGrpc.class) {
        if ((getCreateCallSessionMethod = CallServiceGrpc.getCreateCallSessionMethod) == null) {
          CallServiceGrpc.getCreateCallSessionMethod = getCreateCallSessionMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CreateCallSessionRequest, com.sdkwork.communication.app.v3.CreateCallSessionResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateCallSession"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateCallSessionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateCallSessionResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CallServiceMethodDescriptorSupplier("CreateCallSession"))
              .build();
        }
      }
    }
    return getCreateCallSessionMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveCallSessionRequest,
      com.sdkwork.communication.app.v3.RetrieveCallSessionResponse> getRetrieveCallSessionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveCallSession",
      requestType = com.sdkwork.communication.app.v3.RetrieveCallSessionRequest.class,
      responseType = com.sdkwork.communication.app.v3.RetrieveCallSessionResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveCallSessionRequest,
      com.sdkwork.communication.app.v3.RetrieveCallSessionResponse> getRetrieveCallSessionMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveCallSessionRequest, com.sdkwork.communication.app.v3.RetrieveCallSessionResponse> getRetrieveCallSessionMethod;
    if ((getRetrieveCallSessionMethod = CallServiceGrpc.getRetrieveCallSessionMethod) == null) {
      synchronized (CallServiceGrpc.class) {
        if ((getRetrieveCallSessionMethod = CallServiceGrpc.getRetrieveCallSessionMethod) == null) {
          CallServiceGrpc.getRetrieveCallSessionMethod = getRetrieveCallSessionMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.RetrieveCallSessionRequest, com.sdkwork.communication.app.v3.RetrieveCallSessionResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveCallSession"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveCallSessionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveCallSessionResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CallServiceMethodDescriptorSupplier("RetrieveCallSession"))
              .build();
        }
      }
    }
    return getRetrieveCallSessionMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.InviteCallSessionRequest,
      com.sdkwork.communication.app.v3.InviteCallSessionResponse> getInviteCallSessionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "InviteCallSession",
      requestType = com.sdkwork.communication.app.v3.InviteCallSessionRequest.class,
      responseType = com.sdkwork.communication.app.v3.InviteCallSessionResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.InviteCallSessionRequest,
      com.sdkwork.communication.app.v3.InviteCallSessionResponse> getInviteCallSessionMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.InviteCallSessionRequest, com.sdkwork.communication.app.v3.InviteCallSessionResponse> getInviteCallSessionMethod;
    if ((getInviteCallSessionMethod = CallServiceGrpc.getInviteCallSessionMethod) == null) {
      synchronized (CallServiceGrpc.class) {
        if ((getInviteCallSessionMethod = CallServiceGrpc.getInviteCallSessionMethod) == null) {
          CallServiceGrpc.getInviteCallSessionMethod = getInviteCallSessionMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.InviteCallSessionRequest, com.sdkwork.communication.app.v3.InviteCallSessionResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "InviteCallSession"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.InviteCallSessionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.InviteCallSessionResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CallServiceMethodDescriptorSupplier("InviteCallSession"))
              .build();
        }
      }
    }
    return getInviteCallSessionMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.AcceptCallSessionRequest,
      com.sdkwork.communication.app.v3.AcceptCallSessionResponse> getAcceptCallSessionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "AcceptCallSession",
      requestType = com.sdkwork.communication.app.v3.AcceptCallSessionRequest.class,
      responseType = com.sdkwork.communication.app.v3.AcceptCallSessionResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.AcceptCallSessionRequest,
      com.sdkwork.communication.app.v3.AcceptCallSessionResponse> getAcceptCallSessionMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.AcceptCallSessionRequest, com.sdkwork.communication.app.v3.AcceptCallSessionResponse> getAcceptCallSessionMethod;
    if ((getAcceptCallSessionMethod = CallServiceGrpc.getAcceptCallSessionMethod) == null) {
      synchronized (CallServiceGrpc.class) {
        if ((getAcceptCallSessionMethod = CallServiceGrpc.getAcceptCallSessionMethod) == null) {
          CallServiceGrpc.getAcceptCallSessionMethod = getAcceptCallSessionMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.AcceptCallSessionRequest, com.sdkwork.communication.app.v3.AcceptCallSessionResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "AcceptCallSession"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.AcceptCallSessionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.AcceptCallSessionResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CallServiceMethodDescriptorSupplier("AcceptCallSession"))
              .build();
        }
      }
    }
    return getAcceptCallSessionMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RejectCallSessionRequest,
      com.sdkwork.communication.app.v3.RejectCallSessionResponse> getRejectCallSessionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RejectCallSession",
      requestType = com.sdkwork.communication.app.v3.RejectCallSessionRequest.class,
      responseType = com.sdkwork.communication.app.v3.RejectCallSessionResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RejectCallSessionRequest,
      com.sdkwork.communication.app.v3.RejectCallSessionResponse> getRejectCallSessionMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RejectCallSessionRequest, com.sdkwork.communication.app.v3.RejectCallSessionResponse> getRejectCallSessionMethod;
    if ((getRejectCallSessionMethod = CallServiceGrpc.getRejectCallSessionMethod) == null) {
      synchronized (CallServiceGrpc.class) {
        if ((getRejectCallSessionMethod = CallServiceGrpc.getRejectCallSessionMethod) == null) {
          CallServiceGrpc.getRejectCallSessionMethod = getRejectCallSessionMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.RejectCallSessionRequest, com.sdkwork.communication.app.v3.RejectCallSessionResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RejectCallSession"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RejectCallSessionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RejectCallSessionResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CallServiceMethodDescriptorSupplier("RejectCallSession"))
              .build();
        }
      }
    }
    return getRejectCallSessionMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.EndCallSessionRequest,
      com.sdkwork.communication.app.v3.EndCallSessionResponse> getEndCallSessionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "EndCallSession",
      requestType = com.sdkwork.communication.app.v3.EndCallSessionRequest.class,
      responseType = com.sdkwork.communication.app.v3.EndCallSessionResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.EndCallSessionRequest,
      com.sdkwork.communication.app.v3.EndCallSessionResponse> getEndCallSessionMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.EndCallSessionRequest, com.sdkwork.communication.app.v3.EndCallSessionResponse> getEndCallSessionMethod;
    if ((getEndCallSessionMethod = CallServiceGrpc.getEndCallSessionMethod) == null) {
      synchronized (CallServiceGrpc.class) {
        if ((getEndCallSessionMethod = CallServiceGrpc.getEndCallSessionMethod) == null) {
          CallServiceGrpc.getEndCallSessionMethod = getEndCallSessionMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.EndCallSessionRequest, com.sdkwork.communication.app.v3.EndCallSessionResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "EndCallSession"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.EndCallSessionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.EndCallSessionResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CallServiceMethodDescriptorSupplier("EndCallSession"))
              .build();
        }
      }
    }
    return getEndCallSessionMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateCallSignalRequest,
      com.sdkwork.communication.app.v3.CreateCallSignalResponse> getCreateCallSignalMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateCallSignal",
      requestType = com.sdkwork.communication.app.v3.CreateCallSignalRequest.class,
      responseType = com.sdkwork.communication.app.v3.CreateCallSignalResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateCallSignalRequest,
      com.sdkwork.communication.app.v3.CreateCallSignalResponse> getCreateCallSignalMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateCallSignalRequest, com.sdkwork.communication.app.v3.CreateCallSignalResponse> getCreateCallSignalMethod;
    if ((getCreateCallSignalMethod = CallServiceGrpc.getCreateCallSignalMethod) == null) {
      synchronized (CallServiceGrpc.class) {
        if ((getCreateCallSignalMethod = CallServiceGrpc.getCreateCallSignalMethod) == null) {
          CallServiceGrpc.getCreateCallSignalMethod = getCreateCallSignalMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CreateCallSignalRequest, com.sdkwork.communication.app.v3.CreateCallSignalResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateCallSignal"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateCallSignalRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateCallSignalResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CallServiceMethodDescriptorSupplier("CreateCallSignal"))
              .build();
        }
      }
    }
    return getCreateCallSignalMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateCallCredentialRequest,
      com.sdkwork.communication.app.v3.CreateCallCredentialResponse> getCreateCallCredentialMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateCallCredential",
      requestType = com.sdkwork.communication.app.v3.CreateCallCredentialRequest.class,
      responseType = com.sdkwork.communication.app.v3.CreateCallCredentialResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateCallCredentialRequest,
      com.sdkwork.communication.app.v3.CreateCallCredentialResponse> getCreateCallCredentialMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateCallCredentialRequest, com.sdkwork.communication.app.v3.CreateCallCredentialResponse> getCreateCallCredentialMethod;
    if ((getCreateCallCredentialMethod = CallServiceGrpc.getCreateCallCredentialMethod) == null) {
      synchronized (CallServiceGrpc.class) {
        if ((getCreateCallCredentialMethod = CallServiceGrpc.getCreateCallCredentialMethod) == null) {
          CallServiceGrpc.getCreateCallCredentialMethod = getCreateCallCredentialMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CreateCallCredentialRequest, com.sdkwork.communication.app.v3.CreateCallCredentialResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateCallCredential"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateCallCredentialRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateCallCredentialResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CallServiceMethodDescriptorSupplier("CreateCallCredential"))
              .build();
        }
      }
    }
    return getCreateCallCredentialMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.WatchCallSignalsRequest,
      com.sdkwork.communication.app.v3.WatchCallSignalsResponse> getWatchCallSignalsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "WatchCallSignals",
      requestType = com.sdkwork.communication.app.v3.WatchCallSignalsRequest.class,
      responseType = com.sdkwork.communication.app.v3.WatchCallSignalsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.WatchCallSignalsRequest,
      com.sdkwork.communication.app.v3.WatchCallSignalsResponse> getWatchCallSignalsMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.WatchCallSignalsRequest, com.sdkwork.communication.app.v3.WatchCallSignalsResponse> getWatchCallSignalsMethod;
    if ((getWatchCallSignalsMethod = CallServiceGrpc.getWatchCallSignalsMethod) == null) {
      synchronized (CallServiceGrpc.class) {
        if ((getWatchCallSignalsMethod = CallServiceGrpc.getWatchCallSignalsMethod) == null) {
          CallServiceGrpc.getWatchCallSignalsMethod = getWatchCallSignalsMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.WatchCallSignalsRequest, com.sdkwork.communication.app.v3.WatchCallSignalsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "WatchCallSignals"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.WatchCallSignalsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.WatchCallSignalsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new CallServiceMethodDescriptorSupplier("WatchCallSignals"))
              .build();
        }
      }
    }
    return getWatchCallSignalsMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static CallServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CallServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CallServiceStub>() {
        @java.lang.Override
        public CallServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CallServiceStub(channel, callOptions);
        }
      };
    return CallServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static CallServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CallServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CallServiceBlockingV2Stub>() {
        @java.lang.Override
        public CallServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CallServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return CallServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static CallServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CallServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CallServiceBlockingStub>() {
        @java.lang.Override
        public CallServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CallServiceBlockingStub(channel, callOptions);
        }
      };
    return CallServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static CallServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<CallServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<CallServiceFutureStub>() {
        @java.lang.Override
        public CallServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new CallServiceFutureStub(channel, callOptions);
        }
      };
    return CallServiceFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void createCallSession(com.sdkwork.communication.app.v3.CreateCallSessionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateCallSessionResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateCallSessionMethod(), responseObserver);
    }

    /**
     */
    default void retrieveCallSession(com.sdkwork.communication.app.v3.RetrieveCallSessionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveCallSessionResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveCallSessionMethod(), responseObserver);
    }

    /**
     */
    default void inviteCallSession(com.sdkwork.communication.app.v3.InviteCallSessionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.InviteCallSessionResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getInviteCallSessionMethod(), responseObserver);
    }

    /**
     */
    default void acceptCallSession(com.sdkwork.communication.app.v3.AcceptCallSessionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.AcceptCallSessionResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getAcceptCallSessionMethod(), responseObserver);
    }

    /**
     */
    default void rejectCallSession(com.sdkwork.communication.app.v3.RejectCallSessionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RejectCallSessionResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRejectCallSessionMethod(), responseObserver);
    }

    /**
     */
    default void endCallSession(com.sdkwork.communication.app.v3.EndCallSessionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.EndCallSessionResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getEndCallSessionMethod(), responseObserver);
    }

    /**
     */
    default void createCallSignal(com.sdkwork.communication.app.v3.CreateCallSignalRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateCallSignalResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateCallSignalMethod(), responseObserver);
    }

    /**
     */
    default void createCallCredential(com.sdkwork.communication.app.v3.CreateCallCredentialRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateCallCredentialResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateCallCredentialMethod(), responseObserver);
    }

    /**
     */
    default void watchCallSignals(com.sdkwork.communication.app.v3.WatchCallSignalsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.WatchCallSignalsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getWatchCallSignalsMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service CallService.
   */
  public static abstract class CallServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return CallServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service CallService.
   */
  public static final class CallServiceStub
      extends io.grpc.stub.AbstractAsyncStub<CallServiceStub> {
    private CallServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CallServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CallServiceStub(channel, callOptions);
    }

    /**
     */
    public void createCallSession(com.sdkwork.communication.app.v3.CreateCallSessionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateCallSessionResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateCallSessionMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveCallSession(com.sdkwork.communication.app.v3.RetrieveCallSessionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveCallSessionResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveCallSessionMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void inviteCallSession(com.sdkwork.communication.app.v3.InviteCallSessionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.InviteCallSessionResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getInviteCallSessionMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void acceptCallSession(com.sdkwork.communication.app.v3.AcceptCallSessionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.AcceptCallSessionResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getAcceptCallSessionMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void rejectCallSession(com.sdkwork.communication.app.v3.RejectCallSessionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RejectCallSessionResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRejectCallSessionMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void endCallSession(com.sdkwork.communication.app.v3.EndCallSessionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.EndCallSessionResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getEndCallSessionMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createCallSignal(com.sdkwork.communication.app.v3.CreateCallSignalRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateCallSignalResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateCallSignalMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createCallCredential(com.sdkwork.communication.app.v3.CreateCallCredentialRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateCallCredentialResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateCallCredentialMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void watchCallSignals(com.sdkwork.communication.app.v3.WatchCallSignalsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.WatchCallSignalsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncServerStreamingCall(
          getChannel().newCall(getWatchCallSignalsMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service CallService.
   */
  public static final class CallServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<CallServiceBlockingV2Stub> {
    private CallServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CallServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CallServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateCallSessionResponse createCallSession(com.sdkwork.communication.app.v3.CreateCallSessionRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateCallSessionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveCallSessionResponse retrieveCallSession(com.sdkwork.communication.app.v3.RetrieveCallSessionRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveCallSessionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.InviteCallSessionResponse inviteCallSession(com.sdkwork.communication.app.v3.InviteCallSessionRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getInviteCallSessionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.AcceptCallSessionResponse acceptCallSession(com.sdkwork.communication.app.v3.AcceptCallSessionRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getAcceptCallSessionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RejectCallSessionResponse rejectCallSession(com.sdkwork.communication.app.v3.RejectCallSessionRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRejectCallSessionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.EndCallSessionResponse endCallSession(com.sdkwork.communication.app.v3.EndCallSessionRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getEndCallSessionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateCallSignalResponse createCallSignal(com.sdkwork.communication.app.v3.CreateCallSignalRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateCallSignalMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateCallCredentialResponse createCallCredential(com.sdkwork.communication.app.v3.CreateCallCredentialRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateCallCredentialMethod(), getCallOptions(), request);
    }

    /**
     */
    @io.grpc.ExperimentalApi("https://github.com/grpc/grpc-java/issues/10918")
    public io.grpc.stub.BlockingClientCall<?, com.sdkwork.communication.app.v3.WatchCallSignalsResponse>
        watchCallSignals(com.sdkwork.communication.app.v3.WatchCallSignalsRequest request) {
      return io.grpc.stub.ClientCalls.blockingV2ServerStreamingCall(
          getChannel(), getWatchCallSignalsMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service CallService.
   */
  public static final class CallServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<CallServiceBlockingStub> {
    private CallServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CallServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CallServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateCallSessionResponse createCallSession(com.sdkwork.communication.app.v3.CreateCallSessionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateCallSessionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveCallSessionResponse retrieveCallSession(com.sdkwork.communication.app.v3.RetrieveCallSessionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveCallSessionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.InviteCallSessionResponse inviteCallSession(com.sdkwork.communication.app.v3.InviteCallSessionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getInviteCallSessionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.AcceptCallSessionResponse acceptCallSession(com.sdkwork.communication.app.v3.AcceptCallSessionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getAcceptCallSessionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RejectCallSessionResponse rejectCallSession(com.sdkwork.communication.app.v3.RejectCallSessionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRejectCallSessionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.EndCallSessionResponse endCallSession(com.sdkwork.communication.app.v3.EndCallSessionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getEndCallSessionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateCallSignalResponse createCallSignal(com.sdkwork.communication.app.v3.CreateCallSignalRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateCallSignalMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateCallCredentialResponse createCallCredential(com.sdkwork.communication.app.v3.CreateCallCredentialRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateCallCredentialMethod(), getCallOptions(), request);
    }

    /**
     */
    public java.util.Iterator<com.sdkwork.communication.app.v3.WatchCallSignalsResponse> watchCallSignals(
        com.sdkwork.communication.app.v3.WatchCallSignalsRequest request) {
      return io.grpc.stub.ClientCalls.blockingServerStreamingCall(
          getChannel(), getWatchCallSignalsMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service CallService.
   */
  public static final class CallServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<CallServiceFutureStub> {
    private CallServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected CallServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new CallServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CreateCallSessionResponse> createCallSession(
        com.sdkwork.communication.app.v3.CreateCallSessionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateCallSessionMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.RetrieveCallSessionResponse> retrieveCallSession(
        com.sdkwork.communication.app.v3.RetrieveCallSessionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveCallSessionMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.InviteCallSessionResponse> inviteCallSession(
        com.sdkwork.communication.app.v3.InviteCallSessionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getInviteCallSessionMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.AcceptCallSessionResponse> acceptCallSession(
        com.sdkwork.communication.app.v3.AcceptCallSessionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getAcceptCallSessionMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.RejectCallSessionResponse> rejectCallSession(
        com.sdkwork.communication.app.v3.RejectCallSessionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRejectCallSessionMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.EndCallSessionResponse> endCallSession(
        com.sdkwork.communication.app.v3.EndCallSessionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getEndCallSessionMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CreateCallSignalResponse> createCallSignal(
        com.sdkwork.communication.app.v3.CreateCallSignalRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateCallSignalMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CreateCallCredentialResponse> createCallCredential(
        com.sdkwork.communication.app.v3.CreateCallCredentialRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateCallCredentialMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_CREATE_CALL_SESSION = 0;
  private static final int METHODID_RETRIEVE_CALL_SESSION = 1;
  private static final int METHODID_INVITE_CALL_SESSION = 2;
  private static final int METHODID_ACCEPT_CALL_SESSION = 3;
  private static final int METHODID_REJECT_CALL_SESSION = 4;
  private static final int METHODID_END_CALL_SESSION = 5;
  private static final int METHODID_CREATE_CALL_SIGNAL = 6;
  private static final int METHODID_CREATE_CALL_CREDENTIAL = 7;
  private static final int METHODID_WATCH_CALL_SIGNALS = 8;

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
        case METHODID_CREATE_CALL_SESSION:
          serviceImpl.createCallSession((com.sdkwork.communication.app.v3.CreateCallSessionRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateCallSessionResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_CALL_SESSION:
          serviceImpl.retrieveCallSession((com.sdkwork.communication.app.v3.RetrieveCallSessionRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveCallSessionResponse>) responseObserver);
          break;
        case METHODID_INVITE_CALL_SESSION:
          serviceImpl.inviteCallSession((com.sdkwork.communication.app.v3.InviteCallSessionRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.InviteCallSessionResponse>) responseObserver);
          break;
        case METHODID_ACCEPT_CALL_SESSION:
          serviceImpl.acceptCallSession((com.sdkwork.communication.app.v3.AcceptCallSessionRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.AcceptCallSessionResponse>) responseObserver);
          break;
        case METHODID_REJECT_CALL_SESSION:
          serviceImpl.rejectCallSession((com.sdkwork.communication.app.v3.RejectCallSessionRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RejectCallSessionResponse>) responseObserver);
          break;
        case METHODID_END_CALL_SESSION:
          serviceImpl.endCallSession((com.sdkwork.communication.app.v3.EndCallSessionRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.EndCallSessionResponse>) responseObserver);
          break;
        case METHODID_CREATE_CALL_SIGNAL:
          serviceImpl.createCallSignal((com.sdkwork.communication.app.v3.CreateCallSignalRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateCallSignalResponse>) responseObserver);
          break;
        case METHODID_CREATE_CALL_CREDENTIAL:
          serviceImpl.createCallCredential((com.sdkwork.communication.app.v3.CreateCallCredentialRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateCallCredentialResponse>) responseObserver);
          break;
        case METHODID_WATCH_CALL_SIGNALS:
          serviceImpl.watchCallSignals((com.sdkwork.communication.app.v3.WatchCallSignalsRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.WatchCallSignalsResponse>) responseObserver);
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
          getCreateCallSessionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CreateCallSessionRequest,
              com.sdkwork.communication.app.v3.CreateCallSessionResponse>(
                service, METHODID_CREATE_CALL_SESSION)))
        .addMethod(
          getRetrieveCallSessionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.RetrieveCallSessionRequest,
              com.sdkwork.communication.app.v3.RetrieveCallSessionResponse>(
                service, METHODID_RETRIEVE_CALL_SESSION)))
        .addMethod(
          getInviteCallSessionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.InviteCallSessionRequest,
              com.sdkwork.communication.app.v3.InviteCallSessionResponse>(
                service, METHODID_INVITE_CALL_SESSION)))
        .addMethod(
          getAcceptCallSessionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.AcceptCallSessionRequest,
              com.sdkwork.communication.app.v3.AcceptCallSessionResponse>(
                service, METHODID_ACCEPT_CALL_SESSION)))
        .addMethod(
          getRejectCallSessionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.RejectCallSessionRequest,
              com.sdkwork.communication.app.v3.RejectCallSessionResponse>(
                service, METHODID_REJECT_CALL_SESSION)))
        .addMethod(
          getEndCallSessionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.EndCallSessionRequest,
              com.sdkwork.communication.app.v3.EndCallSessionResponse>(
                service, METHODID_END_CALL_SESSION)))
        .addMethod(
          getCreateCallSignalMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CreateCallSignalRequest,
              com.sdkwork.communication.app.v3.CreateCallSignalResponse>(
                service, METHODID_CREATE_CALL_SIGNAL)))
        .addMethod(
          getCreateCallCredentialMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CreateCallCredentialRequest,
              com.sdkwork.communication.app.v3.CreateCallCredentialResponse>(
                service, METHODID_CREATE_CALL_CREDENTIAL)))
        .addMethod(
          getWatchCallSignalsMethod(),
          io.grpc.stub.ServerCalls.asyncServerStreamingCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.WatchCallSignalsRequest,
              com.sdkwork.communication.app.v3.WatchCallSignalsResponse>(
                service, METHODID_WATCH_CALL_SIGNALS)))
        .build();
  }

  private static abstract class CallServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    CallServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.app.v3.CallServiceOuterClass.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("CallService");
    }
  }

  private static final class CallServiceFileDescriptorSupplier
      extends CallServiceBaseDescriptorSupplier {
    CallServiceFileDescriptorSupplier() {}
  }

  private static final class CallServiceMethodDescriptorSupplier
      extends CallServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    CallServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (CallServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new CallServiceFileDescriptorSupplier())
              .addMethod(getCreateCallSessionMethod())
              .addMethod(getRetrieveCallSessionMethod())
              .addMethod(getInviteCallSessionMethod())
              .addMethod(getAcceptCallSessionMethod())
              .addMethod(getRejectCallSessionMethod())
              .addMethod(getEndCallSessionMethod())
              .addMethod(getCreateCallSignalMethod())
              .addMethod(getCreateCallCredentialMethod())
              .addMethod(getWatchCallSignalsMethod())
              .build();
        }
      }
    }
    return result;
  }
}
