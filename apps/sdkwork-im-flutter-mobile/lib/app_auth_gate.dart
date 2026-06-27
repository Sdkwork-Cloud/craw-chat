import 'dart:async';

import 'package:app_links/app_links.dart';
import 'package:flutter/material.dart';
import 'package:sdkwork_im_flutter_mobile_core/sdkwork_im_flutter_mobile_core.dart';
import 'package:url_launcher/url_launcher.dart';

import 'bootstrap/app_auth.dart';
import 'bootstrap/environment.dart';
import 'bootstrap/sdk_clients.dart';
import 'chat/chat_home.dart';

class AppAuthGate extends StatefulWidget {
  const AppAuthGate({super.key});

  @override
  State<AppAuthGate> createState() => _AppAuthGateState();
}

class _AppAuthGateState extends State<AppAuthGate> {
  ImAppSession? _session = loadAppSession();
  late ImAppSession _form = _session ?? defaultAppSession;
  final _environment = resolveEnvironment();
  final _appLinks = AppLinks();
  StreamSubscription<Uri>? _appLinkSubscription;

  final _accessTokenController = TextEditingController();
  final _authTokenController = TextEditingController();
  final _tenantIdController = TextEditingController();
  final _organizationIdController = TextEditingController();
  final _userIdController = TextEditingController();

  @override
  void initState() {
    super.initState();
    _syncControllers();
    if (_session != null) {
      createSdkClients(session: _session);
    }
    _appLinkSubscription = _appLinks.uriLinkStream.listen((uri) async {
      final callbackSession = await consumeAppbaseCallbackSession(uri);
      if (callbackSession == null || !mounted) {
        return;
      }
      await resetSdkClients();
      createSdkClients(session: callbackSession);
      setState(() {
        _session = callbackSession;
        _form = callbackSession;
      });
    });
  }

  @override
  void dispose() {
    _appLinkSubscription?.cancel();
    _accessTokenController.dispose();
    _authTokenController.dispose();
    _tenantIdController.dispose();
    _organizationIdController.dispose();
    _userIdController.dispose();
    super.dispose();
  }

  void _syncControllers() {
    _accessTokenController.text = _form.accessToken;
    _authTokenController.text = _form.authToken;
    _tenantIdController.text = _form.tenantId;
    _organizationIdController.text = _form.organizationId;
    _userIdController.text = _form.userId;
  }

  Future<void> _handleSubmit() async {
    final nextSession = ImAppSession(
      accessToken: _accessTokenController.text.trim(),
      authToken: _authTokenController.text.trim().isEmpty
          ? _accessTokenController.text.trim()
          : _authTokenController.text.trim(),
      tenantId: _tenantIdController.text.trim().isEmpty
          ? defaultAppSession.tenantId
          : _tenantIdController.text.trim(),
      organizationId: _organizationIdController.text.trim().isEmpty
          ? defaultAppSession.organizationId
          : _organizationIdController.text.trim(),
      userId: _userIdController.text.trim().isEmpty
          ? defaultAppSession.userId
          : _userIdController.text.trim(),
    );
    if (nextSession.accessToken.isEmpty) {
      return;
    }
    await saveAppSession(nextSession);
    await resetSdkClients();
    createSdkClients(session: nextSession);
    setState(() {
      _session = nextSession;
      _form = nextSession;
    });
  }

  Future<void> _handleAppbaseLogin() async {
    final loginUri = buildAppbaseLoginUrl(
      loginUrl: _environment.appbaseLoginUrl,
      returnUrl: appbaseCallbackReturnUrl,
    );
    final launched = await launchUrl(loginUri, mode: LaunchMode.externalApplication);
    if (!launched && mounted) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('Unable to open Appbase login URL')),
      );
    }
  }

  Future<void> _handleSignOut() async {
    await clearAppSession();
    await resetSdkClients();
    setState(() {
      _session = null;
      _form = defaultAppSession;
      _syncControllers();
    });
  }

  @override
  Widget build(BuildContext context) {
    final session = _session;
    if (session != null) {
      return Column(
        children: [
          Material(
            color: const Color(0xFFE3F2FD),
            child: SafeArea(
              bottom: false,
              child: Padding(
                padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
                child: Row(
                  children: [
                    Expanded(child: Text('Signed in as ${session.userId}')),
                    TextButton(onPressed: _handleSignOut, child: const Text('Sign out')),
                  ],
                ),
              ),
            ),
          ),
          Expanded(child: ChatHome(session: session)),
        ],
      );
    }

    return Scaffold(
      body: SafeArea(
        child: Center(
          child: SingleChildScrollView(
            padding: const EdgeInsets.all(24),
            child: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 420),
              child: Card(
                child: Padding(
                  padding: const EdgeInsets.all(24),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.stretch,
                    children: [
                      Text(
                        'IM App Sign In',
                        style: Theme.of(context).textTheme.titleLarge,
                      ),
                      const SizedBox(height: 8),
                      Text(
                        'Sign in through appbase IAM or provide local IM credentials for development.',
                        style: Theme.of(context).textTheme.bodyMedium,
                      ),
                      const SizedBox(height: 16),
                      FilledButton(
                        onPressed: _handleAppbaseLogin,
                        child: const Text('Continue with Appbase'),
                      ),
                      const SizedBox(height: 16),
                      const Center(
                        child: Text(
                          'or use development credentials',
                          style: TextStyle(color: Colors.blueGrey),
                        ),
                      ),
                      const SizedBox(height: 16),
                      TextField(
                        controller: _accessTokenController,
                        decoration: const InputDecoration(labelText: 'Access Token'),
                      ),
                      const SizedBox(height: 12),
                      TextField(
                        controller: _authTokenController,
                        decoration: const InputDecoration(labelText: 'Auth Token'),
                      ),
                      const SizedBox(height: 12),
                      TextField(
                        controller: _tenantIdController,
                        decoration: const InputDecoration(labelText: 'Tenant ID'),
                      ),
                      const SizedBox(height: 12),
                      TextField(
                        controller: _organizationIdController,
                        decoration: const InputDecoration(labelText: 'Organization ID'),
                      ),
                      const SizedBox(height: 12),
                      TextField(
                        controller: _userIdController,
                        decoration: const InputDecoration(labelText: 'User ID'),
                      ),
                      const SizedBox(height: 20),
                      FilledButton(
                        onPressed: _handleSubmit,
                        child: const Text('Continue with Dev Credentials'),
                      ),
                    ],
                  ),
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}
