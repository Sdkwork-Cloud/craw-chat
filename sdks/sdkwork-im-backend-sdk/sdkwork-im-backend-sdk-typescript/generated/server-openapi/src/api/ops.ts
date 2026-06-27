import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { ClusterRetrieveResponse, CommercialReadinessRetrieveResponse, DiagnosticsRetrieveResponse, HealthRetrieveResponse, LagRetrieveResponse, OpsProviderBindingsDriftRetrieveResponse, OpsProviderBindingsListResponse, ReplayStatusRetrieveResponse, RuntimeDirRetrieveResponse } from '../types';


export class OpsDiagnosticsApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve diagnostics */
  async retrieve(): Promise<DiagnosticsRetrieveResponse> {
    return this.client.get<DiagnosticsRetrieveResponse>(backendApiPath(`/ops/diagnostics`));
  }
}

export class OpsProviderBindingsDriftApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve provider binding drift */
  async retrieve(): Promise<OpsProviderBindingsDriftRetrieveResponse> {
    return this.client.get<OpsProviderBindingsDriftRetrieveResponse>(backendApiPath(`/ops/provider_bindings/drift`));
  }
}

export class OpsProviderBindingsApi {
  private client: HttpClient;
  public readonly drift: OpsProviderBindingsDriftApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.drift = new OpsProviderBindingsDriftApi(client);
  }


/** List provider bindings */
  async list(): Promise<OpsProviderBindingsListResponse> {
    return this.client.get<OpsProviderBindingsListResponse>(backendApiPath(`/ops/provider_bindings`));
  }
}

export class OpsRuntimeDirApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Inspect runtime directory */
  async retrieve(): Promise<RuntimeDirRetrieveResponse> {
    return this.client.get<RuntimeDirRetrieveResponse>(backendApiPath(`/ops/runtime_dir`));
  }
}

export class OpsCommercialReadinessApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve commercial readiness */
  async retrieve(): Promise<CommercialReadinessRetrieveResponse> {
    return this.client.get<CommercialReadinessRetrieveResponse>(backendApiPath(`/ops/commercial_readiness`));
  }
}

export class OpsReplayStatusApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve replay status */
  async retrieve(): Promise<ReplayStatusRetrieveResponse> {
    return this.client.get<ReplayStatusRetrieveResponse>(backendApiPath(`/ops/replay_status`));
  }
}

export class OpsLagApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve projection lag */
  async retrieve(): Promise<LagRetrieveResponse> {
    return this.client.get<LagRetrieveResponse>(backendApiPath(`/ops/lag`));
  }
}

export class OpsClusterApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve cluster state */
  async retrieve(): Promise<ClusterRetrieveResponse> {
    return this.client.get<ClusterRetrieveResponse>(backendApiPath(`/ops/cluster`));
  }
}

export class OpsHealthApi {
  private client: HttpClient;

  constructor(client: HttpClient) {
    this.client = client;
  }


/** Retrieve ops health */
  async retrieve(): Promise<HealthRetrieveResponse> {
    return this.client.get<HealthRetrieveResponse>(backendApiPath(`/ops/health`));
  }
}

export class OpsApi {
  private client: HttpClient;
  public readonly health: OpsHealthApi;
  public readonly cluster: OpsClusterApi;
  public readonly lag: OpsLagApi;
  public readonly replayStatus: OpsReplayStatusApi;
  public readonly commercialReadiness: OpsCommercialReadinessApi;
  public readonly runtimeDir: OpsRuntimeDirApi;
  public readonly providerBindings: OpsProviderBindingsApi;
  public readonly diagnostics: OpsDiagnosticsApi;

  constructor(client: HttpClient) {
    this.client = client;
    this.health = new OpsHealthApi(client);
    this.cluster = new OpsClusterApi(client);
    this.lag = new OpsLagApi(client);
    this.replayStatus = new OpsReplayStatusApi(client);
    this.commercialReadiness = new OpsCommercialReadinessApi(client);
    this.runtimeDir = new OpsRuntimeDirApi(client);
    this.providerBindings = new OpsProviderBindingsApi(client);
    this.diagnostics = new OpsDiagnosticsApi(client);
  }

}

export function createOpsApi(client: HttpClient): OpsApi {
  return new OpsApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}
