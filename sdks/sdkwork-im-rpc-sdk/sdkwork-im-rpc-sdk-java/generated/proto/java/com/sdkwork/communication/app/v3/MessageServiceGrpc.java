package com.sdkwork.communication.app.v3;

import static io.grpc.MethodDescriptor.generateFullMethodName;

/**
 */
@io.grpc.stub.annotations.GrpcGenerated
public final class MessageServiceGrpc {

  private MessageServiceGrpc() {}

  public static final java.lang.String SERVICE_NAME = "sdkwork.communication.app.v3.MessageService";

  // Static method descriptors that strictly reflect the proto.
  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListConversationMessagesRequest,
      com.sdkwork.communication.app.v3.ListConversationMessagesResponse> getListConversationMessagesMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListConversationMessages",
      requestType = com.sdkwork.communication.app.v3.ListConversationMessagesRequest.class,
      responseType = com.sdkwork.communication.app.v3.ListConversationMessagesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListConversationMessagesRequest,
      com.sdkwork.communication.app.v3.ListConversationMessagesResponse> getListConversationMessagesMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListConversationMessagesRequest, com.sdkwork.communication.app.v3.ListConversationMessagesResponse> getListConversationMessagesMethod;
    if ((getListConversationMessagesMethod = MessageServiceGrpc.getListConversationMessagesMethod) == null) {
      synchronized (MessageServiceGrpc.class) {
        if ((getListConversationMessagesMethod = MessageServiceGrpc.getListConversationMessagesMethod) == null) {
          MessageServiceGrpc.getListConversationMessagesMethod = getListConversationMessagesMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.ListConversationMessagesRequest, com.sdkwork.communication.app.v3.ListConversationMessagesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListConversationMessages"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListConversationMessagesRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListConversationMessagesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new MessageServiceMethodDescriptorSupplier("ListConversationMessages"))
              .build();
        }
      }
    }
    return getListConversationMessagesMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateConversationMessageRequest,
      com.sdkwork.communication.app.v3.CreateConversationMessageResponse> getCreateConversationMessageMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateConversationMessage",
      requestType = com.sdkwork.communication.app.v3.CreateConversationMessageRequest.class,
      responseType = com.sdkwork.communication.app.v3.CreateConversationMessageResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateConversationMessageRequest,
      com.sdkwork.communication.app.v3.CreateConversationMessageResponse> getCreateConversationMessageMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateConversationMessageRequest, com.sdkwork.communication.app.v3.CreateConversationMessageResponse> getCreateConversationMessageMethod;
    if ((getCreateConversationMessageMethod = MessageServiceGrpc.getCreateConversationMessageMethod) == null) {
      synchronized (MessageServiceGrpc.class) {
        if ((getCreateConversationMessageMethod = MessageServiceGrpc.getCreateConversationMessageMethod) == null) {
          MessageServiceGrpc.getCreateConversationMessageMethod = getCreateConversationMessageMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CreateConversationMessageRequest, com.sdkwork.communication.app.v3.CreateConversationMessageResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateConversationMessage"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateConversationMessageRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateConversationMessageResponse.getDefaultInstance()))
              .setSchemaDescriptor(new MessageServiceMethodDescriptorSupplier("CreateConversationMessage"))
              .build();
        }
      }
    }
    return getCreateConversationMessageMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.PublishSystemChannelMessageRequest,
      com.sdkwork.communication.app.v3.PublishSystemChannelMessageResponse> getPublishSystemChannelMessageMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "PublishSystemChannelMessage",
      requestType = com.sdkwork.communication.app.v3.PublishSystemChannelMessageRequest.class,
      responseType = com.sdkwork.communication.app.v3.PublishSystemChannelMessageResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.PublishSystemChannelMessageRequest,
      com.sdkwork.communication.app.v3.PublishSystemChannelMessageResponse> getPublishSystemChannelMessageMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.PublishSystemChannelMessageRequest, com.sdkwork.communication.app.v3.PublishSystemChannelMessageResponse> getPublishSystemChannelMessageMethod;
    if ((getPublishSystemChannelMessageMethod = MessageServiceGrpc.getPublishSystemChannelMessageMethod) == null) {
      synchronized (MessageServiceGrpc.class) {
        if ((getPublishSystemChannelMessageMethod = MessageServiceGrpc.getPublishSystemChannelMessageMethod) == null) {
          MessageServiceGrpc.getPublishSystemChannelMessageMethod = getPublishSystemChannelMessageMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.PublishSystemChannelMessageRequest, com.sdkwork.communication.app.v3.PublishSystemChannelMessageResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "PublishSystemChannelMessage"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.PublishSystemChannelMessageRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.PublishSystemChannelMessageResponse.getDefaultInstance()))
              .setSchemaDescriptor(new MessageServiceMethodDescriptorSupplier("PublishSystemChannelMessage"))
              .build();
        }
      }
    }
    return getPublishSystemChannelMessageMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryRequest,
      com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryResponse> getRetrieveMessageInteractionSummaryMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RetrieveMessageInteractionSummary",
      requestType = com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryRequest.class,
      responseType = com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryRequest,
      com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryResponse> getRetrieveMessageInteractionSummaryMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryRequest, com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryResponse> getRetrieveMessageInteractionSummaryMethod;
    if ((getRetrieveMessageInteractionSummaryMethod = MessageServiceGrpc.getRetrieveMessageInteractionSummaryMethod) == null) {
      synchronized (MessageServiceGrpc.class) {
        if ((getRetrieveMessageInteractionSummaryMethod = MessageServiceGrpc.getRetrieveMessageInteractionSummaryMethod) == null) {
          MessageServiceGrpc.getRetrieveMessageInteractionSummaryMethod = getRetrieveMessageInteractionSummaryMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryRequest, com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RetrieveMessageInteractionSummary"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryResponse.getDefaultInstance()))
              .setSchemaDescriptor(new MessageServiceMethodDescriptorSupplier("RetrieveMessageInteractionSummary"))
              .build();
        }
      }
    }
    return getRetrieveMessageInteractionSummaryMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.EditMessageRequest,
      com.sdkwork.communication.app.v3.EditMessageResponse> getEditMessageMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "EditMessage",
      requestType = com.sdkwork.communication.app.v3.EditMessageRequest.class,
      responseType = com.sdkwork.communication.app.v3.EditMessageResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.EditMessageRequest,
      com.sdkwork.communication.app.v3.EditMessageResponse> getEditMessageMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.EditMessageRequest, com.sdkwork.communication.app.v3.EditMessageResponse> getEditMessageMethod;
    if ((getEditMessageMethod = MessageServiceGrpc.getEditMessageMethod) == null) {
      synchronized (MessageServiceGrpc.class) {
        if ((getEditMessageMethod = MessageServiceGrpc.getEditMessageMethod) == null) {
          MessageServiceGrpc.getEditMessageMethod = getEditMessageMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.EditMessageRequest, com.sdkwork.communication.app.v3.EditMessageResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "EditMessage"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.EditMessageRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.EditMessageResponse.getDefaultInstance()))
              .setSchemaDescriptor(new MessageServiceMethodDescriptorSupplier("EditMessage"))
              .build();
        }
      }
    }
    return getEditMessageMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RecallMessageRequest,
      com.sdkwork.communication.app.v3.RecallMessageResponse> getRecallMessageMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "RecallMessage",
      requestType = com.sdkwork.communication.app.v3.RecallMessageRequest.class,
      responseType = com.sdkwork.communication.app.v3.RecallMessageResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RecallMessageRequest,
      com.sdkwork.communication.app.v3.RecallMessageResponse> getRecallMessageMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.RecallMessageRequest, com.sdkwork.communication.app.v3.RecallMessageResponse> getRecallMessageMethod;
    if ((getRecallMessageMethod = MessageServiceGrpc.getRecallMessageMethod) == null) {
      synchronized (MessageServiceGrpc.class) {
        if ((getRecallMessageMethod = MessageServiceGrpc.getRecallMessageMethod) == null) {
          MessageServiceGrpc.getRecallMessageMethod = getRecallMessageMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.RecallMessageRequest, com.sdkwork.communication.app.v3.RecallMessageResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "RecallMessage"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RecallMessageRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.RecallMessageResponse.getDefaultInstance()))
              .setSchemaDescriptor(new MessageServiceMethodDescriptorSupplier("RecallMessage"))
              .build();
        }
      }
    }
    return getRecallMessageMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListFavoriteMessagesRequest,
      com.sdkwork.communication.app.v3.ListFavoriteMessagesResponse> getListFavoriteMessagesMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "ListFavoriteMessages",
      requestType = com.sdkwork.communication.app.v3.ListFavoriteMessagesRequest.class,
      responseType = com.sdkwork.communication.app.v3.ListFavoriteMessagesResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListFavoriteMessagesRequest,
      com.sdkwork.communication.app.v3.ListFavoriteMessagesResponse> getListFavoriteMessagesMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.ListFavoriteMessagesRequest, com.sdkwork.communication.app.v3.ListFavoriteMessagesResponse> getListFavoriteMessagesMethod;
    if ((getListFavoriteMessagesMethod = MessageServiceGrpc.getListFavoriteMessagesMethod) == null) {
      synchronized (MessageServiceGrpc.class) {
        if ((getListFavoriteMessagesMethod = MessageServiceGrpc.getListFavoriteMessagesMethod) == null) {
          MessageServiceGrpc.getListFavoriteMessagesMethod = getListFavoriteMessagesMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.ListFavoriteMessagesRequest, com.sdkwork.communication.app.v3.ListFavoriteMessagesResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "ListFavoriteMessages"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListFavoriteMessagesRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.ListFavoriteMessagesResponse.getDefaultInstance()))
              .setSchemaDescriptor(new MessageServiceMethodDescriptorSupplier("ListFavoriteMessages"))
              .build();
        }
      }
    }
    return getListFavoriteMessagesMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateMessageFavoriteRequest,
      com.sdkwork.communication.app.v3.CreateMessageFavoriteResponse> getCreateMessageFavoriteMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateMessageFavorite",
      requestType = com.sdkwork.communication.app.v3.CreateMessageFavoriteRequest.class,
      responseType = com.sdkwork.communication.app.v3.CreateMessageFavoriteResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateMessageFavoriteRequest,
      com.sdkwork.communication.app.v3.CreateMessageFavoriteResponse> getCreateMessageFavoriteMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateMessageFavoriteRequest, com.sdkwork.communication.app.v3.CreateMessageFavoriteResponse> getCreateMessageFavoriteMethod;
    if ((getCreateMessageFavoriteMethod = MessageServiceGrpc.getCreateMessageFavoriteMethod) == null) {
      synchronized (MessageServiceGrpc.class) {
        if ((getCreateMessageFavoriteMethod = MessageServiceGrpc.getCreateMessageFavoriteMethod) == null) {
          MessageServiceGrpc.getCreateMessageFavoriteMethod = getCreateMessageFavoriteMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CreateMessageFavoriteRequest, com.sdkwork.communication.app.v3.CreateMessageFavoriteResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateMessageFavorite"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateMessageFavoriteRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateMessageFavoriteResponse.getDefaultInstance()))
              .setSchemaDescriptor(new MessageServiceMethodDescriptorSupplier("CreateMessageFavorite"))
              .build();
        }
      }
    }
    return getCreateMessageFavoriteMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.DeleteMessageFavoriteRequest,
      com.sdkwork.communication.app.v3.DeleteMessageFavoriteResponse> getDeleteMessageFavoriteMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeleteMessageFavorite",
      requestType = com.sdkwork.communication.app.v3.DeleteMessageFavoriteRequest.class,
      responseType = com.sdkwork.communication.app.v3.DeleteMessageFavoriteResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.DeleteMessageFavoriteRequest,
      com.sdkwork.communication.app.v3.DeleteMessageFavoriteResponse> getDeleteMessageFavoriteMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.DeleteMessageFavoriteRequest, com.sdkwork.communication.app.v3.DeleteMessageFavoriteResponse> getDeleteMessageFavoriteMethod;
    if ((getDeleteMessageFavoriteMethod = MessageServiceGrpc.getDeleteMessageFavoriteMethod) == null) {
      synchronized (MessageServiceGrpc.class) {
        if ((getDeleteMessageFavoriteMethod = MessageServiceGrpc.getDeleteMessageFavoriteMethod) == null) {
          MessageServiceGrpc.getDeleteMessageFavoriteMethod = getDeleteMessageFavoriteMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.DeleteMessageFavoriteRequest, com.sdkwork.communication.app.v3.DeleteMessageFavoriteResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeleteMessageFavorite"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.DeleteMessageFavoriteRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.DeleteMessageFavoriteResponse.getDefaultInstance()))
              .setSchemaDescriptor(new MessageServiceMethodDescriptorSupplier("DeleteMessageFavorite"))
              .build();
        }
      }
    }
    return getDeleteMessageFavoriteMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.DeleteMessageVisibilityRequest,
      com.sdkwork.communication.app.v3.DeleteMessageVisibilityResponse> getDeleteMessageVisibilityMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeleteMessageVisibility",
      requestType = com.sdkwork.communication.app.v3.DeleteMessageVisibilityRequest.class,
      responseType = com.sdkwork.communication.app.v3.DeleteMessageVisibilityResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.DeleteMessageVisibilityRequest,
      com.sdkwork.communication.app.v3.DeleteMessageVisibilityResponse> getDeleteMessageVisibilityMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.DeleteMessageVisibilityRequest, com.sdkwork.communication.app.v3.DeleteMessageVisibilityResponse> getDeleteMessageVisibilityMethod;
    if ((getDeleteMessageVisibilityMethod = MessageServiceGrpc.getDeleteMessageVisibilityMethod) == null) {
      synchronized (MessageServiceGrpc.class) {
        if ((getDeleteMessageVisibilityMethod = MessageServiceGrpc.getDeleteMessageVisibilityMethod) == null) {
          MessageServiceGrpc.getDeleteMessageVisibilityMethod = getDeleteMessageVisibilityMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.DeleteMessageVisibilityRequest, com.sdkwork.communication.app.v3.DeleteMessageVisibilityResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeleteMessageVisibility"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.DeleteMessageVisibilityRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.DeleteMessageVisibilityResponse.getDefaultInstance()))
              .setSchemaDescriptor(new MessageServiceMethodDescriptorSupplier("DeleteMessageVisibility"))
              .build();
        }
      }
    }
    return getDeleteMessageVisibilityMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateMessageReactionRequest,
      com.sdkwork.communication.app.v3.CreateMessageReactionResponse> getCreateMessageReactionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "CreateMessageReaction",
      requestType = com.sdkwork.communication.app.v3.CreateMessageReactionRequest.class,
      responseType = com.sdkwork.communication.app.v3.CreateMessageReactionResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateMessageReactionRequest,
      com.sdkwork.communication.app.v3.CreateMessageReactionResponse> getCreateMessageReactionMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.CreateMessageReactionRequest, com.sdkwork.communication.app.v3.CreateMessageReactionResponse> getCreateMessageReactionMethod;
    if ((getCreateMessageReactionMethod = MessageServiceGrpc.getCreateMessageReactionMethod) == null) {
      synchronized (MessageServiceGrpc.class) {
        if ((getCreateMessageReactionMethod = MessageServiceGrpc.getCreateMessageReactionMethod) == null) {
          MessageServiceGrpc.getCreateMessageReactionMethod = getCreateMessageReactionMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.CreateMessageReactionRequest, com.sdkwork.communication.app.v3.CreateMessageReactionResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "CreateMessageReaction"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateMessageReactionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.CreateMessageReactionResponse.getDefaultInstance()))
              .setSchemaDescriptor(new MessageServiceMethodDescriptorSupplier("CreateMessageReaction"))
              .build();
        }
      }
    }
    return getCreateMessageReactionMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.DeleteMessageReactionRequest,
      com.sdkwork.communication.app.v3.DeleteMessageReactionResponse> getDeleteMessageReactionMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "DeleteMessageReaction",
      requestType = com.sdkwork.communication.app.v3.DeleteMessageReactionRequest.class,
      responseType = com.sdkwork.communication.app.v3.DeleteMessageReactionResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.DeleteMessageReactionRequest,
      com.sdkwork.communication.app.v3.DeleteMessageReactionResponse> getDeleteMessageReactionMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.DeleteMessageReactionRequest, com.sdkwork.communication.app.v3.DeleteMessageReactionResponse> getDeleteMessageReactionMethod;
    if ((getDeleteMessageReactionMethod = MessageServiceGrpc.getDeleteMessageReactionMethod) == null) {
      synchronized (MessageServiceGrpc.class) {
        if ((getDeleteMessageReactionMethod = MessageServiceGrpc.getDeleteMessageReactionMethod) == null) {
          MessageServiceGrpc.getDeleteMessageReactionMethod = getDeleteMessageReactionMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.DeleteMessageReactionRequest, com.sdkwork.communication.app.v3.DeleteMessageReactionResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "DeleteMessageReaction"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.DeleteMessageReactionRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.DeleteMessageReactionResponse.getDefaultInstance()))
              .setSchemaDescriptor(new MessageServiceMethodDescriptorSupplier("DeleteMessageReaction"))
              .build();
        }
      }
    }
    return getDeleteMessageReactionMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.PinMessageRequest,
      com.sdkwork.communication.app.v3.PinMessageResponse> getPinMessageMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "PinMessage",
      requestType = com.sdkwork.communication.app.v3.PinMessageRequest.class,
      responseType = com.sdkwork.communication.app.v3.PinMessageResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.PinMessageRequest,
      com.sdkwork.communication.app.v3.PinMessageResponse> getPinMessageMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.PinMessageRequest, com.sdkwork.communication.app.v3.PinMessageResponse> getPinMessageMethod;
    if ((getPinMessageMethod = MessageServiceGrpc.getPinMessageMethod) == null) {
      synchronized (MessageServiceGrpc.class) {
        if ((getPinMessageMethod = MessageServiceGrpc.getPinMessageMethod) == null) {
          MessageServiceGrpc.getPinMessageMethod = getPinMessageMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.PinMessageRequest, com.sdkwork.communication.app.v3.PinMessageResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "PinMessage"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.PinMessageRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.PinMessageResponse.getDefaultInstance()))
              .setSchemaDescriptor(new MessageServiceMethodDescriptorSupplier("PinMessage"))
              .build();
        }
      }
    }
    return getPinMessageMethod;
  }

  private static volatile io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.UnpinMessageRequest,
      com.sdkwork.communication.app.v3.UnpinMessageResponse> getUnpinMessageMethod;

  @io.grpc.stub.annotations.RpcMethod(
      fullMethodName = SERVICE_NAME + '/' + "UnpinMessage",
      requestType = com.sdkwork.communication.app.v3.UnpinMessageRequest.class,
      responseType = com.sdkwork.communication.app.v3.UnpinMessageResponse.class,
      methodType = io.grpc.MethodDescriptor.MethodType.UNARY)
  public static io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.UnpinMessageRequest,
      com.sdkwork.communication.app.v3.UnpinMessageResponse> getUnpinMessageMethod() {
    io.grpc.MethodDescriptor<com.sdkwork.communication.app.v3.UnpinMessageRequest, com.sdkwork.communication.app.v3.UnpinMessageResponse> getUnpinMessageMethod;
    if ((getUnpinMessageMethod = MessageServiceGrpc.getUnpinMessageMethod) == null) {
      synchronized (MessageServiceGrpc.class) {
        if ((getUnpinMessageMethod = MessageServiceGrpc.getUnpinMessageMethod) == null) {
          MessageServiceGrpc.getUnpinMessageMethod = getUnpinMessageMethod =
              io.grpc.MethodDescriptor.<com.sdkwork.communication.app.v3.UnpinMessageRequest, com.sdkwork.communication.app.v3.UnpinMessageResponse>newBuilder()
              .setType(io.grpc.MethodDescriptor.MethodType.UNARY)
              .setFullMethodName(generateFullMethodName(SERVICE_NAME, "UnpinMessage"))
              .setSampledToLocalTracing(true)
              .setRequestMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.UnpinMessageRequest.getDefaultInstance()))
              .setResponseMarshaller(io.grpc.protobuf.ProtoUtils.marshaller(
                  com.sdkwork.communication.app.v3.UnpinMessageResponse.getDefaultInstance()))
              .setSchemaDescriptor(new MessageServiceMethodDescriptorSupplier("UnpinMessage"))
              .build();
        }
      }
    }
    return getUnpinMessageMethod;
  }

  /**
   * Creates a new async stub that supports all call types for the service
   */
  public static MessageServiceStub newStub(io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<MessageServiceStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<MessageServiceStub>() {
        @java.lang.Override
        public MessageServiceStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new MessageServiceStub(channel, callOptions);
        }
      };
    return MessageServiceStub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports all types of calls on the service
   */
  public static MessageServiceBlockingV2Stub newBlockingV2Stub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<MessageServiceBlockingV2Stub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<MessageServiceBlockingV2Stub>() {
        @java.lang.Override
        public MessageServiceBlockingV2Stub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new MessageServiceBlockingV2Stub(channel, callOptions);
        }
      };
    return MessageServiceBlockingV2Stub.newStub(factory, channel);
  }

  /**
   * Creates a new blocking-style stub that supports unary and streaming output calls on the service
   */
  public static MessageServiceBlockingStub newBlockingStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<MessageServiceBlockingStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<MessageServiceBlockingStub>() {
        @java.lang.Override
        public MessageServiceBlockingStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new MessageServiceBlockingStub(channel, callOptions);
        }
      };
    return MessageServiceBlockingStub.newStub(factory, channel);
  }

  /**
   * Creates a new ListenableFuture-style stub that supports unary calls on the service
   */
  public static MessageServiceFutureStub newFutureStub(
      io.grpc.Channel channel) {
    io.grpc.stub.AbstractStub.StubFactory<MessageServiceFutureStub> factory =
      new io.grpc.stub.AbstractStub.StubFactory<MessageServiceFutureStub>() {
        @java.lang.Override
        public MessageServiceFutureStub newStub(io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
          return new MessageServiceFutureStub(channel, callOptions);
        }
      };
    return MessageServiceFutureStub.newStub(factory, channel);
  }

  /**
   */
  public interface AsyncService {

    /**
     */
    default void listConversationMessages(com.sdkwork.communication.app.v3.ListConversationMessagesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListConversationMessagesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListConversationMessagesMethod(), responseObserver);
    }

    /**
     */
    default void createConversationMessage(com.sdkwork.communication.app.v3.CreateConversationMessageRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateConversationMessageResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateConversationMessageMethod(), responseObserver);
    }

    /**
     */
    default void publishSystemChannelMessage(com.sdkwork.communication.app.v3.PublishSystemChannelMessageRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.PublishSystemChannelMessageResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getPublishSystemChannelMessageMethod(), responseObserver);
    }

    /**
     */
    default void retrieveMessageInteractionSummary(com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRetrieveMessageInteractionSummaryMethod(), responseObserver);
    }

    /**
     */
    default void editMessage(com.sdkwork.communication.app.v3.EditMessageRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.EditMessageResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getEditMessageMethod(), responseObserver);
    }

    /**
     */
    default void recallMessage(com.sdkwork.communication.app.v3.RecallMessageRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RecallMessageResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getRecallMessageMethod(), responseObserver);
    }

    /**
     */
    default void listFavoriteMessages(com.sdkwork.communication.app.v3.ListFavoriteMessagesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListFavoriteMessagesResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getListFavoriteMessagesMethod(), responseObserver);
    }

    /**
     */
    default void createMessageFavorite(com.sdkwork.communication.app.v3.CreateMessageFavoriteRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateMessageFavoriteResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateMessageFavoriteMethod(), responseObserver);
    }

    /**
     */
    default void deleteMessageFavorite(com.sdkwork.communication.app.v3.DeleteMessageFavoriteRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.DeleteMessageFavoriteResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeleteMessageFavoriteMethod(), responseObserver);
    }

    /**
     */
    default void deleteMessageVisibility(com.sdkwork.communication.app.v3.DeleteMessageVisibilityRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.DeleteMessageVisibilityResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeleteMessageVisibilityMethod(), responseObserver);
    }

    /**
     */
    default void createMessageReaction(com.sdkwork.communication.app.v3.CreateMessageReactionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateMessageReactionResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getCreateMessageReactionMethod(), responseObserver);
    }

    /**
     */
    default void deleteMessageReaction(com.sdkwork.communication.app.v3.DeleteMessageReactionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.DeleteMessageReactionResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getDeleteMessageReactionMethod(), responseObserver);
    }

    /**
     */
    default void pinMessage(com.sdkwork.communication.app.v3.PinMessageRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.PinMessageResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getPinMessageMethod(), responseObserver);
    }

    /**
     */
    default void unpinMessage(com.sdkwork.communication.app.v3.UnpinMessageRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.UnpinMessageResponse> responseObserver) {
      io.grpc.stub.ServerCalls.asyncUnimplementedUnaryCall(getUnpinMessageMethod(), responseObserver);
    }
  }

  /**
   * Base class for the server implementation of the service MessageService.
   */
  public static abstract class MessageServiceImplBase
      implements io.grpc.BindableService, AsyncService {

    @java.lang.Override public final io.grpc.ServerServiceDefinition bindService() {
      return MessageServiceGrpc.bindService(this);
    }
  }

  /**
   * A stub to allow clients to do asynchronous rpc calls to service MessageService.
   */
  public static final class MessageServiceStub
      extends io.grpc.stub.AbstractAsyncStub<MessageServiceStub> {
    private MessageServiceStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected MessageServiceStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new MessageServiceStub(channel, callOptions);
    }

    /**
     */
    public void listConversationMessages(com.sdkwork.communication.app.v3.ListConversationMessagesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListConversationMessagesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListConversationMessagesMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createConversationMessage(com.sdkwork.communication.app.v3.CreateConversationMessageRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateConversationMessageResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateConversationMessageMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void publishSystemChannelMessage(com.sdkwork.communication.app.v3.PublishSystemChannelMessageRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.PublishSystemChannelMessageResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getPublishSystemChannelMessageMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void retrieveMessageInteractionSummary(com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRetrieveMessageInteractionSummaryMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void editMessage(com.sdkwork.communication.app.v3.EditMessageRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.EditMessageResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getEditMessageMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void recallMessage(com.sdkwork.communication.app.v3.RecallMessageRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RecallMessageResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getRecallMessageMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void listFavoriteMessages(com.sdkwork.communication.app.v3.ListFavoriteMessagesRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListFavoriteMessagesResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getListFavoriteMessagesMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createMessageFavorite(com.sdkwork.communication.app.v3.CreateMessageFavoriteRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateMessageFavoriteResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateMessageFavoriteMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deleteMessageFavorite(com.sdkwork.communication.app.v3.DeleteMessageFavoriteRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.DeleteMessageFavoriteResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeleteMessageFavoriteMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deleteMessageVisibility(com.sdkwork.communication.app.v3.DeleteMessageVisibilityRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.DeleteMessageVisibilityResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeleteMessageVisibilityMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void createMessageReaction(com.sdkwork.communication.app.v3.CreateMessageReactionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateMessageReactionResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getCreateMessageReactionMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void deleteMessageReaction(com.sdkwork.communication.app.v3.DeleteMessageReactionRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.DeleteMessageReactionResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getDeleteMessageReactionMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void pinMessage(com.sdkwork.communication.app.v3.PinMessageRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.PinMessageResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getPinMessageMethod(), getCallOptions()), request, responseObserver);
    }

    /**
     */
    public void unpinMessage(com.sdkwork.communication.app.v3.UnpinMessageRequest request,
        io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.UnpinMessageResponse> responseObserver) {
      io.grpc.stub.ClientCalls.asyncUnaryCall(
          getChannel().newCall(getUnpinMessageMethod(), getCallOptions()), request, responseObserver);
    }
  }

  /**
   * A stub to allow clients to do synchronous rpc calls to service MessageService.
   */
  public static final class MessageServiceBlockingV2Stub
      extends io.grpc.stub.AbstractBlockingStub<MessageServiceBlockingV2Stub> {
    private MessageServiceBlockingV2Stub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected MessageServiceBlockingV2Stub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new MessageServiceBlockingV2Stub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListConversationMessagesResponse listConversationMessages(com.sdkwork.communication.app.v3.ListConversationMessagesRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListConversationMessagesMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateConversationMessageResponse createConversationMessage(com.sdkwork.communication.app.v3.CreateConversationMessageRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateConversationMessageMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.PublishSystemChannelMessageResponse publishSystemChannelMessage(com.sdkwork.communication.app.v3.PublishSystemChannelMessageRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getPublishSystemChannelMessageMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryResponse retrieveMessageInteractionSummary(com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRetrieveMessageInteractionSummaryMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.EditMessageResponse editMessage(com.sdkwork.communication.app.v3.EditMessageRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getEditMessageMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RecallMessageResponse recallMessage(com.sdkwork.communication.app.v3.RecallMessageRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getRecallMessageMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListFavoriteMessagesResponse listFavoriteMessages(com.sdkwork.communication.app.v3.ListFavoriteMessagesRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getListFavoriteMessagesMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateMessageFavoriteResponse createMessageFavorite(com.sdkwork.communication.app.v3.CreateMessageFavoriteRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateMessageFavoriteMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.DeleteMessageFavoriteResponse deleteMessageFavorite(com.sdkwork.communication.app.v3.DeleteMessageFavoriteRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDeleteMessageFavoriteMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.DeleteMessageVisibilityResponse deleteMessageVisibility(com.sdkwork.communication.app.v3.DeleteMessageVisibilityRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDeleteMessageVisibilityMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateMessageReactionResponse createMessageReaction(com.sdkwork.communication.app.v3.CreateMessageReactionRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getCreateMessageReactionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.DeleteMessageReactionResponse deleteMessageReaction(com.sdkwork.communication.app.v3.DeleteMessageReactionRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getDeleteMessageReactionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.PinMessageResponse pinMessage(com.sdkwork.communication.app.v3.PinMessageRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getPinMessageMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.UnpinMessageResponse unpinMessage(com.sdkwork.communication.app.v3.UnpinMessageRequest request) throws io.grpc.StatusException {
      return io.grpc.stub.ClientCalls.blockingV2UnaryCall(
          getChannel(), getUnpinMessageMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do limited synchronous rpc calls to service MessageService.
   */
  public static final class MessageServiceBlockingStub
      extends io.grpc.stub.AbstractBlockingStub<MessageServiceBlockingStub> {
    private MessageServiceBlockingStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected MessageServiceBlockingStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new MessageServiceBlockingStub(channel, callOptions);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListConversationMessagesResponse listConversationMessages(com.sdkwork.communication.app.v3.ListConversationMessagesRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListConversationMessagesMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateConversationMessageResponse createConversationMessage(com.sdkwork.communication.app.v3.CreateConversationMessageRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateConversationMessageMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.PublishSystemChannelMessageResponse publishSystemChannelMessage(com.sdkwork.communication.app.v3.PublishSystemChannelMessageRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getPublishSystemChannelMessageMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryResponse retrieveMessageInteractionSummary(com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRetrieveMessageInteractionSummaryMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.EditMessageResponse editMessage(com.sdkwork.communication.app.v3.EditMessageRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getEditMessageMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.RecallMessageResponse recallMessage(com.sdkwork.communication.app.v3.RecallMessageRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getRecallMessageMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.ListFavoriteMessagesResponse listFavoriteMessages(com.sdkwork.communication.app.v3.ListFavoriteMessagesRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getListFavoriteMessagesMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateMessageFavoriteResponse createMessageFavorite(com.sdkwork.communication.app.v3.CreateMessageFavoriteRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateMessageFavoriteMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.DeleteMessageFavoriteResponse deleteMessageFavorite(com.sdkwork.communication.app.v3.DeleteMessageFavoriteRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeleteMessageFavoriteMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.DeleteMessageVisibilityResponse deleteMessageVisibility(com.sdkwork.communication.app.v3.DeleteMessageVisibilityRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeleteMessageVisibilityMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.CreateMessageReactionResponse createMessageReaction(com.sdkwork.communication.app.v3.CreateMessageReactionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getCreateMessageReactionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.DeleteMessageReactionResponse deleteMessageReaction(com.sdkwork.communication.app.v3.DeleteMessageReactionRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getDeleteMessageReactionMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.PinMessageResponse pinMessage(com.sdkwork.communication.app.v3.PinMessageRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getPinMessageMethod(), getCallOptions(), request);
    }

    /**
     */
    public com.sdkwork.communication.app.v3.UnpinMessageResponse unpinMessage(com.sdkwork.communication.app.v3.UnpinMessageRequest request) {
      return io.grpc.stub.ClientCalls.blockingUnaryCall(
          getChannel(), getUnpinMessageMethod(), getCallOptions(), request);
    }
  }

  /**
   * A stub to allow clients to do ListenableFuture-style rpc calls to service MessageService.
   */
  public static final class MessageServiceFutureStub
      extends io.grpc.stub.AbstractFutureStub<MessageServiceFutureStub> {
    private MessageServiceFutureStub(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      super(channel, callOptions);
    }

    @java.lang.Override
    protected MessageServiceFutureStub build(
        io.grpc.Channel channel, io.grpc.CallOptions callOptions) {
      return new MessageServiceFutureStub(channel, callOptions);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.ListConversationMessagesResponse> listConversationMessages(
        com.sdkwork.communication.app.v3.ListConversationMessagesRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListConversationMessagesMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CreateConversationMessageResponse> createConversationMessage(
        com.sdkwork.communication.app.v3.CreateConversationMessageRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateConversationMessageMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.PublishSystemChannelMessageResponse> publishSystemChannelMessage(
        com.sdkwork.communication.app.v3.PublishSystemChannelMessageRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getPublishSystemChannelMessageMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryResponse> retrieveMessageInteractionSummary(
        com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRetrieveMessageInteractionSummaryMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.EditMessageResponse> editMessage(
        com.sdkwork.communication.app.v3.EditMessageRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getEditMessageMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.RecallMessageResponse> recallMessage(
        com.sdkwork.communication.app.v3.RecallMessageRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getRecallMessageMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.ListFavoriteMessagesResponse> listFavoriteMessages(
        com.sdkwork.communication.app.v3.ListFavoriteMessagesRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getListFavoriteMessagesMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CreateMessageFavoriteResponse> createMessageFavorite(
        com.sdkwork.communication.app.v3.CreateMessageFavoriteRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateMessageFavoriteMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.DeleteMessageFavoriteResponse> deleteMessageFavorite(
        com.sdkwork.communication.app.v3.DeleteMessageFavoriteRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeleteMessageFavoriteMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.DeleteMessageVisibilityResponse> deleteMessageVisibility(
        com.sdkwork.communication.app.v3.DeleteMessageVisibilityRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeleteMessageVisibilityMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.CreateMessageReactionResponse> createMessageReaction(
        com.sdkwork.communication.app.v3.CreateMessageReactionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getCreateMessageReactionMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.DeleteMessageReactionResponse> deleteMessageReaction(
        com.sdkwork.communication.app.v3.DeleteMessageReactionRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getDeleteMessageReactionMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.PinMessageResponse> pinMessage(
        com.sdkwork.communication.app.v3.PinMessageRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getPinMessageMethod(), getCallOptions()), request);
    }

    /**
     */
    public com.google.common.util.concurrent.ListenableFuture<com.sdkwork.communication.app.v3.UnpinMessageResponse> unpinMessage(
        com.sdkwork.communication.app.v3.UnpinMessageRequest request) {
      return io.grpc.stub.ClientCalls.futureUnaryCall(
          getChannel().newCall(getUnpinMessageMethod(), getCallOptions()), request);
    }
  }

  private static final int METHODID_LIST_CONVERSATION_MESSAGES = 0;
  private static final int METHODID_CREATE_CONVERSATION_MESSAGE = 1;
  private static final int METHODID_PUBLISH_SYSTEM_CHANNEL_MESSAGE = 2;
  private static final int METHODID_RETRIEVE_MESSAGE_INTERACTION_SUMMARY = 3;
  private static final int METHODID_EDIT_MESSAGE = 4;
  private static final int METHODID_RECALL_MESSAGE = 5;
  private static final int METHODID_LIST_FAVORITE_MESSAGES = 6;
  private static final int METHODID_CREATE_MESSAGE_FAVORITE = 7;
  private static final int METHODID_DELETE_MESSAGE_FAVORITE = 8;
  private static final int METHODID_DELETE_MESSAGE_VISIBILITY = 9;
  private static final int METHODID_CREATE_MESSAGE_REACTION = 10;
  private static final int METHODID_DELETE_MESSAGE_REACTION = 11;
  private static final int METHODID_PIN_MESSAGE = 12;
  private static final int METHODID_UNPIN_MESSAGE = 13;

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
        case METHODID_LIST_CONVERSATION_MESSAGES:
          serviceImpl.listConversationMessages((com.sdkwork.communication.app.v3.ListConversationMessagesRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListConversationMessagesResponse>) responseObserver);
          break;
        case METHODID_CREATE_CONVERSATION_MESSAGE:
          serviceImpl.createConversationMessage((com.sdkwork.communication.app.v3.CreateConversationMessageRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateConversationMessageResponse>) responseObserver);
          break;
        case METHODID_PUBLISH_SYSTEM_CHANNEL_MESSAGE:
          serviceImpl.publishSystemChannelMessage((com.sdkwork.communication.app.v3.PublishSystemChannelMessageRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.PublishSystemChannelMessageResponse>) responseObserver);
          break;
        case METHODID_RETRIEVE_MESSAGE_INTERACTION_SUMMARY:
          serviceImpl.retrieveMessageInteractionSummary((com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryResponse>) responseObserver);
          break;
        case METHODID_EDIT_MESSAGE:
          serviceImpl.editMessage((com.sdkwork.communication.app.v3.EditMessageRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.EditMessageResponse>) responseObserver);
          break;
        case METHODID_RECALL_MESSAGE:
          serviceImpl.recallMessage((com.sdkwork.communication.app.v3.RecallMessageRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.RecallMessageResponse>) responseObserver);
          break;
        case METHODID_LIST_FAVORITE_MESSAGES:
          serviceImpl.listFavoriteMessages((com.sdkwork.communication.app.v3.ListFavoriteMessagesRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.ListFavoriteMessagesResponse>) responseObserver);
          break;
        case METHODID_CREATE_MESSAGE_FAVORITE:
          serviceImpl.createMessageFavorite((com.sdkwork.communication.app.v3.CreateMessageFavoriteRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateMessageFavoriteResponse>) responseObserver);
          break;
        case METHODID_DELETE_MESSAGE_FAVORITE:
          serviceImpl.deleteMessageFavorite((com.sdkwork.communication.app.v3.DeleteMessageFavoriteRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.DeleteMessageFavoriteResponse>) responseObserver);
          break;
        case METHODID_DELETE_MESSAGE_VISIBILITY:
          serviceImpl.deleteMessageVisibility((com.sdkwork.communication.app.v3.DeleteMessageVisibilityRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.DeleteMessageVisibilityResponse>) responseObserver);
          break;
        case METHODID_CREATE_MESSAGE_REACTION:
          serviceImpl.createMessageReaction((com.sdkwork.communication.app.v3.CreateMessageReactionRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.CreateMessageReactionResponse>) responseObserver);
          break;
        case METHODID_DELETE_MESSAGE_REACTION:
          serviceImpl.deleteMessageReaction((com.sdkwork.communication.app.v3.DeleteMessageReactionRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.DeleteMessageReactionResponse>) responseObserver);
          break;
        case METHODID_PIN_MESSAGE:
          serviceImpl.pinMessage((com.sdkwork.communication.app.v3.PinMessageRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.PinMessageResponse>) responseObserver);
          break;
        case METHODID_UNPIN_MESSAGE:
          serviceImpl.unpinMessage((com.sdkwork.communication.app.v3.UnpinMessageRequest) request,
              (io.grpc.stub.StreamObserver<com.sdkwork.communication.app.v3.UnpinMessageResponse>) responseObserver);
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
          getListConversationMessagesMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.ListConversationMessagesRequest,
              com.sdkwork.communication.app.v3.ListConversationMessagesResponse>(
                service, METHODID_LIST_CONVERSATION_MESSAGES)))
        .addMethod(
          getCreateConversationMessageMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CreateConversationMessageRequest,
              com.sdkwork.communication.app.v3.CreateConversationMessageResponse>(
                service, METHODID_CREATE_CONVERSATION_MESSAGE)))
        .addMethod(
          getPublishSystemChannelMessageMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.PublishSystemChannelMessageRequest,
              com.sdkwork.communication.app.v3.PublishSystemChannelMessageResponse>(
                service, METHODID_PUBLISH_SYSTEM_CHANNEL_MESSAGE)))
        .addMethod(
          getRetrieveMessageInteractionSummaryMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryRequest,
              com.sdkwork.communication.app.v3.RetrieveMessageInteractionSummaryResponse>(
                service, METHODID_RETRIEVE_MESSAGE_INTERACTION_SUMMARY)))
        .addMethod(
          getEditMessageMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.EditMessageRequest,
              com.sdkwork.communication.app.v3.EditMessageResponse>(
                service, METHODID_EDIT_MESSAGE)))
        .addMethod(
          getRecallMessageMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.RecallMessageRequest,
              com.sdkwork.communication.app.v3.RecallMessageResponse>(
                service, METHODID_RECALL_MESSAGE)))
        .addMethod(
          getListFavoriteMessagesMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.ListFavoriteMessagesRequest,
              com.sdkwork.communication.app.v3.ListFavoriteMessagesResponse>(
                service, METHODID_LIST_FAVORITE_MESSAGES)))
        .addMethod(
          getCreateMessageFavoriteMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CreateMessageFavoriteRequest,
              com.sdkwork.communication.app.v3.CreateMessageFavoriteResponse>(
                service, METHODID_CREATE_MESSAGE_FAVORITE)))
        .addMethod(
          getDeleteMessageFavoriteMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.DeleteMessageFavoriteRequest,
              com.sdkwork.communication.app.v3.DeleteMessageFavoriteResponse>(
                service, METHODID_DELETE_MESSAGE_FAVORITE)))
        .addMethod(
          getDeleteMessageVisibilityMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.DeleteMessageVisibilityRequest,
              com.sdkwork.communication.app.v3.DeleteMessageVisibilityResponse>(
                service, METHODID_DELETE_MESSAGE_VISIBILITY)))
        .addMethod(
          getCreateMessageReactionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.CreateMessageReactionRequest,
              com.sdkwork.communication.app.v3.CreateMessageReactionResponse>(
                service, METHODID_CREATE_MESSAGE_REACTION)))
        .addMethod(
          getDeleteMessageReactionMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.DeleteMessageReactionRequest,
              com.sdkwork.communication.app.v3.DeleteMessageReactionResponse>(
                service, METHODID_DELETE_MESSAGE_REACTION)))
        .addMethod(
          getPinMessageMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.PinMessageRequest,
              com.sdkwork.communication.app.v3.PinMessageResponse>(
                service, METHODID_PIN_MESSAGE)))
        .addMethod(
          getUnpinMessageMethod(),
          io.grpc.stub.ServerCalls.asyncUnaryCall(
            new MethodHandlers<
              com.sdkwork.communication.app.v3.UnpinMessageRequest,
              com.sdkwork.communication.app.v3.UnpinMessageResponse>(
                service, METHODID_UNPIN_MESSAGE)))
        .build();
  }

  private static abstract class MessageServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoFileDescriptorSupplier, io.grpc.protobuf.ProtoServiceDescriptorSupplier {
    MessageServiceBaseDescriptorSupplier() {}

    @java.lang.Override
    public com.google.protobuf.Descriptors.FileDescriptor getFileDescriptor() {
      return com.sdkwork.communication.app.v3.MessageServiceOuterClass.getDescriptor();
    }

    @java.lang.Override
    public com.google.protobuf.Descriptors.ServiceDescriptor getServiceDescriptor() {
      return getFileDescriptor().findServiceByName("MessageService");
    }
  }

  private static final class MessageServiceFileDescriptorSupplier
      extends MessageServiceBaseDescriptorSupplier {
    MessageServiceFileDescriptorSupplier() {}
  }

  private static final class MessageServiceMethodDescriptorSupplier
      extends MessageServiceBaseDescriptorSupplier
      implements io.grpc.protobuf.ProtoMethodDescriptorSupplier {
    private final java.lang.String methodName;

    MessageServiceMethodDescriptorSupplier(java.lang.String methodName) {
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
      synchronized (MessageServiceGrpc.class) {
        result = serviceDescriptor;
        if (result == null) {
          serviceDescriptor = result = io.grpc.ServiceDescriptor.newBuilder(SERVICE_NAME)
              .setSchemaDescriptor(new MessageServiceFileDescriptorSupplier())
              .addMethod(getListConversationMessagesMethod())
              .addMethod(getCreateConversationMessageMethod())
              .addMethod(getPublishSystemChannelMessageMethod())
              .addMethod(getRetrieveMessageInteractionSummaryMethod())
              .addMethod(getEditMessageMethod())
              .addMethod(getRecallMessageMethod())
              .addMethod(getListFavoriteMessagesMethod())
              .addMethod(getCreateMessageFavoriteMethod())
              .addMethod(getDeleteMessageFavoriteMethod())
              .addMethod(getDeleteMessageVisibilityMethod())
              .addMethod(getCreateMessageReactionMethod())
              .addMethod(getDeleteMessageReactionMethod())
              .addMethod(getPinMessageMethod())
              .addMethod(getUnpinMessageMethod())
              .build();
        }
      }
    }
    return result;
  }
}
