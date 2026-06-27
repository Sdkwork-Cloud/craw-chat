import 'package:im_app_api_generated/im_app_api_generated.dart';

import 'context.dart';

class ImAppAutomationModule {
  final ImAppSdkContext context;

  ImAppAutomationModule(this.context);

  Future<StreamSession?> startAgentResponse(StartAgentResponseRequest body) {
    return context.transportClient.automation.agentResponsesCreate(body);
  }

  Future<StreamSession?> completeAgentResponse(
    String streamId,
    CompleteAgentResponseRequest body,
  ) {
    return context.transportClient.automation.agentResponsesComplete(
      streamId,
      body,
    );
  }

  Future<StreamFrame?> appendAgentResponseFrame(
    String streamId,
    AppendAgentResponseDeltaRequest body,
  ) {
    return context.transportClient.automation.agentResponsesFramesCreate(
      streamId,
      body,
    );
  }

  Future<AgentToolCall?> requestAgentToolCall(RequestAgentToolCallRequest body) {
    return context.transportClient.automation.agentToolCallsCreate(body);
  }

  Future<AgentToolCall?> completeAgentToolCall(
    String executionId,
    String toolCallId,
    CompleteAgentToolCallRequest body,
  ) {
    return context.transportClient.automation.agentToolCallsComplete(
      executionId,
      toolCallId,
      body,
    );
  }

  Future<AutomationExecutionRequestResponse?> requestExecution(
    RequestAutomationExecution body,
  ) {
    return context.transportClient.automation.executionsCreate(body);
  }

  Future<AutomationExecution?> getExecution(String executionId) {
    return context.transportClient.automation.executionsRetrieve(executionId);
  }
}
