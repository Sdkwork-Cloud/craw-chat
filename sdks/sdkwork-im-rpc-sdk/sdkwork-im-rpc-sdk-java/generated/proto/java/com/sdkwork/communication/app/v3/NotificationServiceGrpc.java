package com.sdkwork.communication.app.v3;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class NotificationServiceGrpc {

  private NotificationServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.app.v3.NotificationService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListNotificationsRequest,
      com.sdkwork.communication.app.v3.ListNotificationsResponse> getListNotificationsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListNotifications",
      requestType = com.sdkwork.communication.app.v3.ListNotificationsRequest.class,
      responseType = com.sdkwork.communication.app.v3.ListNotificationsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListNotificationsRequest,
      com.sdkwork.communication.app.v3.ListNotificationsResponse> getListNotificationsMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListNotificationsRequest, com.sdkwork.communication.app.v3.ListNotificationsResponse> getListNotificationsMethod;
    if ((getListNotificationsMethod = NotificationServiceGrpc.getListNotificationsMethod) == null) {
      synchronized (NotificationServiceGrpc.class) {
        if ((getListNotificationsMethod = NotificationServiceGrpc.getListNotificationsMethod) == null) {
          NotificationServiceGrpc.getListNotificationsMethod = getListNotificationsMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.ListNotificationsRequest, com.sdkwork.communication.app.v3.ListNotificationsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListNotifications"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListNotificationsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListNotificationsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new NotificationServiceMethodDescriptorSupplier("ListNotifications"))
              .build();
        }
      }
    }
    return getListNotificationsMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateNotificationRequestRequest,
      com.sdkwork.communication.app.v3.CreateNotificationRequestResponse> getCreateNotificationRequestMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateNotificationRequest",
      requestType = com.sdkwork.communication.app.v3.CreateNotificationRequestRequest.class,
      responseType = com.sdkwork.communication.app.v3.CreateNotificationRequestResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateNotificationRequestRequest,
      com.sdkwork.communication.app.v3.CreateNotificationRequestResponse> getCreateNotificationRequestMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateNotificationRequestRequest, com.sdkwork.communication.app.v3.CreateNotificationRequestResponse> getCreateNotificationRequestMethod;
    if ((getCreateNotificationRequestMethod = NotificationServiceGrpc.getCreateNotificationRequestMethod) == null) {
      synchronized (NotificationServiceGrpc.class) {
        if ((getCreateNotificationRequestMethod = NotificationServiceGrpc.getCreateNotificationRequestMethod) == null) {
          NotificationServiceGrpc.getCreateNotificationRequestMethod = getCreateNotificationRequestMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CreateNotificationRequestRequest, com.sdkwork.communication.app.v3.CreateNotificationRequestResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateNotificationRequest"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateNotificationRequestRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateNotificationRequestResponse.getDefaultInstance()))
              .setSchemaDescriptor(new NotificationServiceMethodDescriptorSupplier("CreateNotificationRequest"))
              .build();
        }
      }
    }
    return getCreateNotificationRequestMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveNotificationRequest,
      com.sdkwork.communication.app.v3.RetrieveNotificationResponse> getRetrieveNotificationMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveNotification",
      requestType = com.sdkwork.communication.app.v3.RetrieveNotificationRequest.class,
      responseType = com.sdkwork.communication.app.v3.RetrieveNotificationResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveNotificationRequest,
      com.sdkwork.communication.app.v3.RetrieveNotificationResponse> getRetrieveNotificationMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveNotificationRequest, com.sdkwork.communication.app.v3.RetrieveNotificationResponse> getRetrieveNotificationMethod;
    if ((getRetrieveNotificationMethod = NotificationServiceGrpc.getRetrieveNotificationMethod) == null) {
      synchronized (NotificationServiceGrpc.class) {
        if ((getRetrieveNotificationMethod = NotificationServiceGrpc.getRetrieveNotificationMethod) == null) {
          NotificationServiceGrpc.getRetrieveNotificationMethod = getRetrieveNotificationMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.RetrieveNotificationRequest, com.sdkwork.communication.app.v3.RetrieveNotificationResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveNotification"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveNotificationRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveNotificationResponse.getDefaultInstance()))
              .setSchemaDescriptor(new NotificationServiceMethodDescriptorSupplier("RetrieveNotification"))
              .build();
        }
      }
    }
    return getRetrieveNotificationMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.WatchNotificationsRequest,
      com.sdkwork.communication.app.v3.WatchNotificationsResponse> getWatchNotificationsMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "WatchNotifications",
      requestType = com.sdkwork.communication.app.v3.WatchNotificationsRequest.class,
      responseType = com.sdkwork.communication.app.v3.WatchNotificationsResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.WatchNotificationsRequest,
      com.sdkwork.communication.app.v3.WatchNotificationsResponse> getWatchNotificationsMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.WatchNotificationsRequest, com.sdkwork.communication.app.v3.WatchNotificationsResponse> getWatchNotificationsMethod;
    if ((getWatchNotificationsMethod = NotificationServiceGrpc.getWatchNotificationsMethod) == null) {
      synchronized (NotificationServiceGrpc.class) {
        if ((getWatchNotificationsMethod = NotificationServiceGrpc.getWatchNotificationsMethod) == null) {
          NotificationServiceGrpc.getWatchNotificationsMethod = getWatchNotificationsMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.WatchNotificationsRequest, com.sdkwork.communication.app.v3.WatchNotificationsResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.SERVER_STREAMING)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "WatchNotifications"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.WatchNotificationsRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.WatchNotificationsResponse.getDefaultInstance()))
              .setSchemaDescriptor(new NotificationServiceMethodDescriptorSupplier("WatchNotifications"))
              .build();
        }
      }
    }
    return getWatchNotificationsMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static NotificationServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<NotificationServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<NotificationServiceStub>() {
        @java.lang.Override
        public NotificationServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new NotificationServiceStub(channel, callOptions);
        }
      };
    return NotificationServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static NotificationServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<NotificationServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<NotificationServiceBlockingV2Stub>() {
        @java.lang.Override
        public NotificationServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new NotificationServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return NotificationServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static NotificationServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<NotificationServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<NotificationServiceBlockingStub>() {
        @java.lang.Override
        public NotificationServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new NotificationServiceBlockingStub(channel, callOptions);
        }
      };
    return NotificationServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static NotificationServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<NotificationServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<NotificationServiceFutureStub>() {
        @java.lang.Override
        public NotificationServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new NotificationServiceFutureStub(channel, callOptions);
        }
      };
    return NotificationServiceFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void listNotifications(com.sdkwork.communication.app.v3.ListNotificationsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListNotificationsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListNotificationsMethod(), responseObserver);
    }

    /**
     */
    default void createNotificationRequest(com.sdkwork.communication.app.v3.CreateNotificationRequestRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateNotificationRequestResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateNotificationRequestMethod(), responseObserver);
    }

    /**
     */
    default void retrieveNotification(com.sdkwork.communication.app.v3.RetrieveNotificationRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveNotificationResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveNotificationMethod(), responseObserver);
    }

    /**
     */
    default void watchNotifications(com.sdkwork.communication.app.v3.WatchNotificationsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.WatchNotificationsResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getWatchNotificationsMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service NotificationService.
   */
  public static abstract class NotificationServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return NotificationServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service NotificationService.
   */
  public static final class NotificationServiceStub
      extends io.grpc.stub.AbstractAsyncStub<NotificationServiceStub> {
    private NotificationServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected NotificationServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new NotificationServiceStub(channel, callOptions);
    }

    /**
     */
    public void listNotifications(com.sdkwork.communication.app.v3.ListNotificationsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListNotificationsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListNotificationsMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createNotificationRequest(com.sdkwork.communication.app.v3.CreateNotificationRequestRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateNotificationRequestResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateNotificationRequestMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveNotification(com.sdkwork.communication.app.v3.RetrieveNotificationRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveNotificationResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveNotificationMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void watchNotifications(com.sdkwork.communication.app.v3.WatchNotificationsRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.WatchNotificationsResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncServerStreamingCall(
          getChannel().newCall(getWatchNotificationsMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service NotificationService.
   */
  public static final class NotificationServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<NotificationServiceBlockingV2Stub> {
    private NotificationServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected NotificationServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new NotificationServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListNotificationsResponse listNotifications(com.sdkwork.communication.app.v3.ListNotificationsRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListNotificationsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateNotificationRequestResponse createNotificationRequest(com.sdkwork.communication.app.v3.CreateNotificationRequestRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateNotificationRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveNotificationResponse retrieveNotification(com.sdkwork.communication.app.v3.RetrieveNotificationRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveNotificationMethod(), getCallOptions(), request);
    }

    /**
     */
    @io.grpc.ExperimentalApi("https://github.com/grpc/grpc-java/issues/10918")
    public io.grpc.stub.BlockingClientCall<?, com.sdkwork.communication.app.v3.WatchNotificationsResponse>
        watchNotifications(com.sdkwork.communication.app.v3.WatchNotificationsRequest request) {
      return io.grpc.stub.ClientCalls.blockingV2ServerStreamingCall(
          getChannel(), getWatchNotificationsMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service NotificationService.
   */
  public static final class NotificationServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<NotificationServiceBlockingStub> {
    private NotificationServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected NotificationServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new NotificationServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListNotificationsResponse listNotifications(com.sdkwork.communication.app.v3.ListNotificationsRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListNotificationsMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateNotificationRequestResponse createNotificationRequest(com.sdkwork.communication.app.v3.CreateNotificationRequestRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateNotificationRequestMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveNotificationResponse retrieveNotification(com.sdkwork.communication.app.v3.RetrieveNotificationRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveNotificationMethod(), getCallOptions(), request);
    }

    /**
     */
    public java.util.Iterator<com.sdkwork.communication.app.v3.WatchNotificationsResponse> watchNotifications(
        com.sdkwork.communication.app.v3.WatchNotificationsRequest request) {
      return io.grpc.stub.ClientCalls.blockingServerStreamingCall(
          getChannel(), getWatchNotificationsMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service NotificationService.
   */
  public static final class NotificationServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<NotificationServiceFutureStub> {
    private NotificationServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected NotificationServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new NotificationServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.ListNotificationsResponse> listNotifications(
        com.sdkwork.communication.app.v3.ListNotificationsRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListNotificationsMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CreateNotificationRequestResponse> createNotificationRequest(
        com.sdkwork.communication.app.v3.CreateNotificationRequestRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateNotificationRequestMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.RetrieveNotificationResponse> retrieveNotification(
        com.sdkwork.communication.app.v3.RetrieveNotificationRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveNotificationMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_LIST_NOTIFICATIONS = 0;
  private static final int METHODID_CREATE_NOTIFICATION_REQUEST = 1;
  private static final int METHODID_RETRIEVE_NOTIFICATION = 2;
  private static final int METHODID_WATCH_NOTIFICATIONS = 3;

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
        case METHODID_LIST_NOTIFICATIONS:
          serviceImpl.listNotifications((com.sdkwork.communication.app.v3.ListNotificationsRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListNotificationsResponse>) responseObserver);
          break;
        case METHODID_CREATE_NOTIFICATION_REQUEST:
          serviceImpl.createNotificationRequest((com.sdkwork.communication.app.v3.CreateNotificationRequestRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateNotificationRequestResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_NOTIFICATION:
          serviceImpl.retrieveNotification((com.sdkwork.communication.app.v3.RetrieveNotificationRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveNotificationResponse>) responseObserver);
          break;
        case METHODID_WATCH_NOTIFICATIONS:
          serviceImpl.watchNotifications((com.sdkwork.communication.app.v3.WatchNotificationsRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.WatchNotificationsResponse>) responseObserver);
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
          getListNotificationsMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.ListNotificationsRequest,
              com.sdkwork.communication.app.v3.ListNotificationsResponse>(
                service, METHODID_LIST_NOTIFICATIONS)))
        .addMethod(
          getCreateNotificationRequestMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CreateNotificationRequestRequest,
              com.sdkwork.communication.app.v3.CreateNotificationRequestResponse>(
                service, METHODID_CREATE_NOTIFICATION_REQUEST)))
        .addMethod(
          getRetrieveNotificationMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.RetrieveNotificationRequest,
              com.sdkwork.communication.app.v3.RetrieveNotificationResponse>(
                service, METHODID_RETRIEVE_NOTIFICATION)))
        .addMethod(
          getWatchNotificationsMethod(),
          io.grpc.stub.ServerCalls.asyncServerStreamingCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.WatchNotificationsRequest,
              com.sdkwork.communication.app.v3.WatchNotificationsResponse>(
                service, METHODID_WATCH_NOTIFICATIONS)))
        .build();
  }

  private static abstract class NotificationServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    NotificationServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.app.v3.NotificationServiceOuterClass.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("NotificationService");
    }
  }

  private static final class NotificationServiceFileDescriptorSupplier
      extends NotificationServiceBaseDescriptorSupplier {
    NotificationServiceFileDescriptorSupplier() {}
  }

  private static final class NotificationServiceMethodDescriptorSupplier
      extends NotificationServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    NotificationServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (NotificationServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new NotificationServiceFileDescriptorSupplier())
              .addMethod(getListNotificationsMethod())
              .addMethod(getCreateNotificationRequestMethod())
              .addMethod(getRetrieveNotificationMethod())
              .addMethod(getWatchNotificationsMethod())
              .build();
        }
      }
    }
    return result;
  }
}
