# Realtime

## What This Module Is For

This module covers realtime coordination through the standard WebSocket-first SDK model.

## Public Entrypoints

This page will document the realtime entrypoints for TypeScript, Flutter, and Rust.

## API Mapping

The primary App API alignment is the session and realtime domain.

## Common Workflows

Typical flows include live WebSocket receive, durable catch-up, subscription sync, and event
acknowledgements.

## Ownership and Status

This page separates semantic SDK capabilities from raw route-level transport operations. The
consumer standard is:

- live push through `sdk.connect(...)`
- durable replay through `sdk.sync.catchUp(...)`
- route-level HTTP control through `sdk.realtime.*` only when exact transport alignment is needed

## Example

Use this page together with the session bootstrap example.
