import 'dart:typed_data';

import 'package:crypto/crypto.dart';
import 'package:http/http.dart' as http;
import 'package:im_sdk_generated/src/http/client.dart';
import 'package:sdkwork_common_flutter/sdkwork_common_flutter.dart';

const _appApiPrefix = '/app/v3/api';
const _defaultChunkSizeBytes = 5 * 1024 * 1024;

class DriveUploadReference {
  const DriveUploadReference({
    required this.driveUri,
    required this.spaceId,
    required this.nodeId,
  });

  final String driveUri;
  final String spaceId;
  final String nodeId;
}

class DriveUploaderRequest {
  const DriveUploaderRequest({
    required this.bytes,
    required this.userId,
    required this.appResourceType,
    required this.appResourceId,
    required this.scene,
    required this.source,
    this.originalFileName,
    this.contentType,
    this.uploadProfileCode = 'attachment',
  });

  final Uint8List bytes;
  final String userId;
  final String appResourceType;
  final String appResourceId;
  final String scene;
  final String source;
  final String? originalFileName;
  final String? contentType;
  final String uploadProfileCode;
}

class DriveAppSdkClient {
  DriveAppSdkClient._(this._http);

  final HttpClient _http;

  static DriveAppSdkClient create({
    required String applicationPublicHttpUrl,
    String? accessToken,
    String? authToken,
  }) {
    final baseUrl = _resolveAppApiBaseUrl(applicationPublicHttpUrl);
    final client = HttpClient(
      config: SdkConfig(
        baseUrl: baseUrl,
        accessToken: accessToken,
        authToken: authToken,
        headers: const {
          'x-sdkwork-platform': 'mobile',
        },
      ),
    );
    return DriveAppSdkClient._(client);
  }

  Future<DriveUploadReference> uploadImage(DriveUploaderRequest request) {
    return _uploadByProfile('image', request);
  }

  Future<DriveUploadReference> uploadAttachment(DriveUploaderRequest request) {
    return _uploadByProfile('attachment', request);
  }

  Future<DriveUploadReference> _uploadByProfile(
    String profile,
    DriveUploaderRequest request,
  ) async {
    final contentType = request.contentType ?? 'application/octet-stream';
    final originalFileName = request.originalFileName ?? 'upload.bin';
    final checksumSha256Hex = 'sha256:${sha256.convert(request.bytes).toString()}';
    final uploadId = 'upload-${DateTime.now().millisecondsSinceEpoch}';
    final taskId = 'task-${DateTime.now().millisecondsSinceEpoch}';

    final prepared = _unwrapData(await _http.request(
      'POST',
      '$_appApiPrefix/drive/uploader/uploads',
      body: {
        'id': uploadId,
        'taskId': taskId,
        'appResourceType': request.appResourceType,
        'appResourceId': request.appResourceId,
        'scene': request.scene,
        'source': request.source,
        'uploadProfileCode': profile,
        'fileFingerprint':
            'name:$originalFileName:size:${request.bytes.length}:type:${contentType.replaceAll('/', '.')}',
        'originalFileName': originalFileName,
        'contentType': contentType,
        'contentLength': '${request.bytes.length}',
        'chunkSizeBytes': '$_defaultChunkSizeBytes',
        'checksumSha256Hex': checksumSha256Hex,
      },
    )) as Map<String, dynamic>;

    final uploadItem = Map<String, dynamic>.from(prepared['uploadItem'] as Map);
    final uploadSession = Map<String, dynamic>.from(prepared['uploadSession'] as Map);
    final uploadItemId = uploadItem['id']?.toString();
    final uploadSessionId =
        uploadItem['uploadSessionId']?.toString() ?? uploadSession['id']?.toString();
    final storageUploadId =
        uploadItem['storageUploadId']?.toString() ?? uploadSession['storageUploadId']?.toString();
    if (uploadItemId == null || uploadSessionId == null || storageUploadId == null) {
      throw StateError('Drive uploader prepare response is missing upload identifiers.');
    }

    const partNo = 1;
    final presigned = _unwrapData(await _http.request(
      'PUT',
      '$_appApiPrefix/drive/upload_sessions/$uploadSessionId/parts/$partNo',
      body: {
        'uploadId': storageUploadId,
      },
    )) as Map<String, dynamic>;

    final uploadUrl = presigned['uploadUrl']?.toString();
    final uploadHeaders = Map<String, String>.from(
      (presigned['headers'] as Map?)?.map((key, value) => MapEntry('$key', '$value')) ?? {},
    );
    if (uploadUrl == null || uploadUrl.isEmpty) {
      throw StateError('Drive uploader presign response is missing uploadUrl.');
    }

    final signedResponse = await http.put(
      Uri.parse(uploadUrl),
      headers: uploadHeaders,
      body: request.bytes,
    );
    if (signedResponse.statusCode < 200 || signedResponse.statusCode >= 300) {
      throw StateError(
        'Drive uploader signed upload failed with HTTP ${signedResponse.statusCode}.',
      );
    }
    final etag = signedResponse.headers['etag'] ?? signedResponse.headers['ETag'];
    if (etag == null || etag.isEmpty) {
      throw StateError('Drive uploader signed upload response did not return an ETag.');
    }

    await _http.request(
      'PUT',
      '$_appApiPrefix/drive/uploader/uploads/$uploadItemId/parts/$partNo',
      body: {
        'uploadSessionId': uploadSessionId,
        'offsetBytes': '0',
        'sizeBytes': '${request.bytes.length}',
        'etag': etag,
      },
    );

    final completed = _unwrapData(await _http.request(
      'POST',
      '$_appApiPrefix/drive/upload_sessions/$uploadSessionId/complete',
      body: {
        'uploadId': storageUploadId,
        'contentType': contentType,
        'contentLength': '${request.bytes.length}',
        'checksumSha256Hex': checksumSha256Hex,
        'parts': [
          {
            'partNo': partNo,
            'etag': etag,
          },
        ],
      },
    )) as Map<String, dynamic>;

    final spaceId =
        uploadItem['spaceId']?.toString()
        ?? completed['spaceId']?.toString()
        ?? uploadSession['spaceId']?.toString();
    final nodeId =
        uploadItem['nodeId']?.toString()
        ?? completed['nodeId']?.toString()
        ?? uploadSession['nodeId']?.toString();
    if (spaceId == null || nodeId == null) {
      throw StateError('Drive uploader completion response is missing spaceId or nodeId.');
    }

    return DriveUploadReference(
      driveUri: 'drive://spaces/$spaceId/nodes/$nodeId',
      spaceId: spaceId,
      nodeId: nodeId,
    );
  }
}

String _resolveAppApiBaseUrl(String configuredBaseUrl) {
  final trimmed = configuredBaseUrl.trim().replaceAll(RegExp(r'/+$'), '');
  if (trimmed.endsWith(_appApiPrefix)) {
    return trimmed.substring(0, trimmed.length - _appApiPrefix.length);
  }
  return trimmed;
}

dynamic _unwrapData(dynamic body) {
  if (body is Map<String, dynamic> && body['data'] is Map<String, dynamic>) {
    return body['data'];
  }
  if (body is Map && body['data'] != null) {
    return body['data'];
  }
  return body;
}
