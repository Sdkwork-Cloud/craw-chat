class ImAppSession {
  const ImAppSession({
    required this.accessToken,
    required this.authToken,
    required this.tenantId,
    required this.organizationId,
    required this.userId,
  });

  final String accessToken;
  final String authToken;
  final String tenantId;
  final String organizationId;
  final String userId;

  bool get isComplete =>
      accessToken.isNotEmpty &&
      authToken.isNotEmpty &&
      tenantId.isNotEmpty &&
      organizationId.isNotEmpty &&
      userId.isNotEmpty;

  Map<String, dynamic> toJson() => {
        'accessToken': accessToken,
        'authToken': authToken,
        'tenantId': tenantId,
        'organizationId': organizationId,
        'userId': userId,
      };

  factory ImAppSession.fromJson(Map<String, dynamic> json) {
    final accessToken = json['accessToken']?.toString().trim() ?? '';
    final authToken = json['authToken']?.toString().trim() ?? '';
    return ImAppSession(
      accessToken: accessToken,
      authToken: authToken,
      tenantId: json['tenantId']?.toString().trim() ?? '',
      organizationId: json['organizationId']?.toString().trim() ?? '',
      userId: json['userId']?.toString().trim() ?? '',
    );
  }
}

const defaultAppSession = ImAppSession(
  accessToken: '',
  authToken: '',
  tenantId: '',
  organizationId: '',
  userId: '',
);

const imFlutterMobileSessionStorageKey = 'sdkwork-im-flutter-mobile:session:v1';
