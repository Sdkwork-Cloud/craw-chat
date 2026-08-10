import React from "react";
import { useLocation, useNavigate, useParams, useSearchParams } from "react-router";
import type { ImH5CapabilityModule } from "../contracts";
import { IM_H5_ROUTE_DEFINITIONS } from "../routeCatalog";
import { showToast } from "@sdkwork/im-h5-commons";
import type { AgentConfig } from "@sdkwork/agents-h5-agents";

const MyAgentsView = React.lazy(async () => {
  const mod = await import("@sdkwork/agents-h5-agents");
  return { default: mod.MyAgentsView };
});
const AgentChatView = React.lazy(async () => {
  const mod = await import("@sdkwork/agents-h5-agents");
  return { default: mod.AgentChatView };
});
const CreateAgentMobileView = React.lazy(async () => {
  const mod = await import("@sdkwork/agents-h5-agents");
  return { default: mod.CreateAgentMobileView };
});
const MyCharactersView = React.lazy(async () => {
  const mod = await import("@sdkwork/agents-h5-agents");
  return { default: mod.MyCharactersView };
});
const CreateCharacterMobileView = React.lazy(async () => {
  const mod = await import("@sdkwork/agents-h5-agents");
  return { default: mod.CreateCharacterMobileView };
});
const MyCharacterDetailView = React.lazy(async () => {
  const mod = await import("@sdkwork/agents-h5-agents");
  return { default: mod.MyCharacterDetailView };
});

/** Start an agent session: navigate to the agents chat route with display info. */
function useStartAgentChat() {
  const navigate = useNavigate();
  return (agent: AgentConfig) => {
    navigate(`/agent/chat/${encodeURIComponent(agent.id ?? "")}`, {
      state: {
        agent: {
          name: agent.name,
          welcomeMessage: agent.welcomeMessage,
        },
      },
    });
  };
}

const MyAgentsRoute: React.FC = () => {
  const navigate = useNavigate();
  const startChat = useStartAgentChat();
  return (
    <MyAgentsView
      onBack={() => navigate(-1)}
      onCreateAgent={() => navigate("/agent/create")}
      onEditAgent={(agentId) => navigate(`/agent/edit/${encodeURIComponent(agentId)}`)}
      onStartChat={startChat}
      notify={(message) => showToast(message)}
    />
  );
};

interface AgentChatRouteState {
  agent?: {
    name?: string;
    welcomeMessage?: string;
  };
}

const AgentChatRoute: React.FC = () => {
  const navigate = useNavigate();
  const { agentId = "" } = useParams<{ agentId?: string }>();
  const location = useLocation();
  const state = (location.state ?? {}) as AgentChatRouteState;
  return (
    <AgentChatView
      agentId={agentId}
      agentName={state.agent?.name}
      welcomeMessage={state.agent?.welcomeMessage}
      onBack={() => navigate(-1)}
    />
  );
};

const CreateAgentRoute: React.FC = () => {
  const navigate = useNavigate();
  const { id } = useParams<{ id?: string }>();
  return (
    <CreateAgentMobileView
      initialAgentId={id}
      onBack={() => navigate(-1)}
      notify={(message) => showToast(message)}
    />
  );
};

const MyCharactersRoute: React.FC = () => {
  const navigate = useNavigate();
  return (
    <MyCharactersView
      onBack={() => navigate(-1)}
      onCreateCharacter={() => navigate("/me/characters/create")}
      onEditCharacter={(characterId) =>
        navigate(`/me/characters/create?id=${encodeURIComponent(characterId)}`)
      }
      onViewDetail={(characterId) =>
        navigate(`/me/characters/${encodeURIComponent(characterId)}`)
      }
      notify={(message) => showToast(message)}
    />
  );
};

const CreateCharacterRoute: React.FC = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const editId = searchParams.get("id") ?? undefined;
  return (
    <CreateCharacterMobileView
      initialCharacterId={editId}
      onBack={() => navigate(-1)}
      onSaved={() => navigate("/me/characters", { replace: true })}
      notify={(message) => showToast(message)}
    />
  );
};

const MyCharacterDetailRoute: React.FC = () => {
  const navigate = useNavigate();
  const { id } = useParams<{ id?: string }>();
  return (
    <MyCharacterDetailView
      characterId={id}
      onBack={() => navigate(-1)}
      onStartChat={(character) =>
        navigate(`/chat/${encodeURIComponent(character.id)}`)
      }
      onEdit={(character) =>
        navigate(`/me/characters/create?id=${encodeURIComponent(character.id)}`)
      }
      notify={(message) => showToast(message)}
    />
  );
};

export const agentsModule: ImH5CapabilityModule = {
  id: "agents",
  routes: [
    { ...IM_H5_ROUTE_DEFINITIONS.userMyAgents, render: () => <MyAgentsRoute /> },
    { ...IM_H5_ROUTE_DEFINITIONS.agentsCreate, render: () => <CreateAgentRoute /> },
    { ...IM_H5_ROUTE_DEFINITIONS.agentsEdit, render: () => <CreateAgentRoute /> },
    { ...IM_H5_ROUTE_DEFINITIONS.agentsChat, render: () => <AgentChatRoute /> },
    { ...IM_H5_ROUTE_DEFINITIONS.agentsCharacters, render: () => <MyCharactersRoute /> },
    { ...IM_H5_ROUTE_DEFINITIONS.agentsCharactersCreate, render: () => <CreateCharacterRoute /> },
    { ...IM_H5_ROUTE_DEFINITIONS.agentsCharactersDetail, render: () => <MyCharacterDetailRoute /> },
  ],
};
