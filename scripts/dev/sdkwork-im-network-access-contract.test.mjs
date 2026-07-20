import assert from 'node:assert/strict';
import test from 'node:test';

import {
  createSdkworkChatPcAccessUrls,
  formatSdkworkChatPcAccessLinks,
  resolveLocalNetworkHosts,
  resolveSdkworkChatAccessNetworkHosts,
} from '../lib/im-pc-dev.mjs';

const networkInterfaces = {
  Ethernet: [
    { address: '192.168.1.25', family: 'IPv4', internal: false },
    { address: '203.0.113.10', family: 'IPv4', internal: false },
    { address: '127.0.0.1', family: 'IPv4', internal: true },
  ],
  Virtual: [
    { address: '10.8.0.4', family: 4, internal: false },
    { address: '169.254.30.58', family: 4, internal: false },
    { address: '198.18.0.1', family: 'IPv4', internal: false },
    { address: '192.168.1.25', family: 'IPv4', internal: false },
    { address: 'fd00::4', family: 'IPv6', internal: false },
    { address: 'fe80::4', family: 'IPv6', internal: false },
  ],
};

test('IM reuses shared IPv4 discovery while retaining private-network policy', () => {
  assert.deepEqual(resolveLocalNetworkHosts({ networkInterfaces }), [
    '10.8.0.4',
    '192.168.1.25',
    'fd00::4',
  ]);
});

test('IM access output includes every non-loopback IPv4 without widening CORS policy', () => {
  assert.deepEqual(resolveSdkworkChatAccessNetworkHosts({ networkInterfaces }), [
    '10.8.0.4',
    '169.254.30.58',
    '192.168.1.25',
    '198.18.0.1',
    '203.0.113.10',
    'fd00::4',
  ]);
});

test('IM startup formats every selected network URL through the shared formatter', () => {
  const accessUrls = createSdkworkChatPcAccessUrls({
    networkHosts: resolveSdkworkChatAccessNetworkHosts({ networkInterfaces }),
    port: 4188,
  });
  assert.equal(formatSdkworkChatPcAccessLinks(accessUrls), [
    '[sdkwork-im] application started successfully',
    '[sdkwork-im] Local: http://localhost:4188',
    '[sdkwork-im] Network: http://10.8.0.4:4188',
    '[sdkwork-im] Network: http://169.254.30.58:4188',
    '[sdkwork-im] Network: http://192.168.1.25:4188',
    '[sdkwork-im] Network: http://198.18.0.1:4188',
    '[sdkwork-im] Network: http://203.0.113.10:4188',
    '[sdkwork-im] Network: http://[fd00::4]:4188',
  ].join('\n'));
});

test('IM prints an explicit unavailable state without inventing network URLs', () => {
  assert.equal(formatSdkworkChatPcAccessLinks(
    createSdkworkChatPcAccessUrls({ networkHosts: [], port: 4188 }),
  ), [
    '[sdkwork-im] application started successfully',
    '[sdkwork-im] Local: http://localhost:4188',
    '[sdkwork-im] Network: no private IPv4 LAN address detected',
  ].join('\n'));
});
