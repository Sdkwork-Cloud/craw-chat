# Step 11 Capacity Tier artifact root

- artifactRoot: `artifacts/perf/step-11/capacity`
- gateTemplate: `tools/perf/step-11-capacity-tier-gate.json`
- evidenceIndex: `capacity-tier-evidence-index.json`
- schema: `../schemas/step-11-tier-evidence-index.schema.json`
- checksumManifestPath: `checksum-manifest.txt`
- artifactFileListPath: `artifact-file-list.txt`
- slot contract source: capacity-tier-evidence-index.json
- gate contract source: tools/perf/step-11-capacity-tier-gate.json
- scenario catalog source: tools/perf/step-11-scenario-catalog.json
- schema source: ../schemas/step-11-tier-evidence-index.schema.json
- current gate state: `evidence_collected_gate_passed`
- collected slots: `connection_capacity`, `message_capacity`, `stream_capacity`, `restore_recovery_recovery`, `failover_recovery`, `capacity_report`, `recovery_report`
- current evidence slot states: `collected`
- default naming rule: `artifactPath = artifactRoot + "/" + suggestedRelativePath`
- collected artifacts: `connection/capacity.json`, `message/capacity.json`, `stream/capacity.json`, `restore-recovery/recovery.json`, `failover/recovery.json`, `reports/capacity-report.md`, `reports/recovery-report.md`
- required fields: runId, peakActiveConnections, connectP95Ms, connectP99Ms, messageTps, fanoutP95Ms, fanoutP99Ms, streamFramesPerSecond, frameP95Ms, frameP99Ms, restoreRtoSeconds, dataLossRpoEvents, previewDiffAccuracy, takeoverDurationMs, ownerSwitchAccuracy, staleSessionRejectionRate
- required sections: input_scale, throughput_summary, tail_latency_summary, recovery_window, rto_rpo_summary, operator_follow_up
- boundary: this root now carries all seven truthful local Capacity Tier artifacts. `Capacity Tier` is `evidence_collected_gate_passed` after doc-captured backfill from published CI Smoke Tier / self-hosted.split-services.development evidence.
- refresh command: `pnpm run perf:refresh-step-11-capacity-evidence-index`
- exit 0: all required capacity evidence slots are collected and valid.
- exit 1: the refresh command failed to load, parse, or write the evidence index.
- exit 2: at least one required evidence slot is missing or invalid; this is the expected blocker before real `capacity-dedicated` evidence exists.

| slot id | suggestedRelativePath |
| --- | --- |
| `connection_capacity` | `connection/capacity.json` |
| `message_capacity` | `message/capacity.json` |
| `stream_capacity` | `stream/capacity.json` |
| `restore_recovery_recovery` | `restore-recovery/recovery.json` |
| `failover_recovery` | `failover/recovery.json` |
| `capacity_report` | `reports/capacity-report.md` |
| `recovery_report` | `reports/recovery-report.md` |
