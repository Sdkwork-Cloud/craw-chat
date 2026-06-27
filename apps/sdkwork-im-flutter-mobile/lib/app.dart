import 'package:flutter/material.dart';

import 'auth_gate.dart';

class ImApp extends StatelessWidget {
  const ImApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'SDKWork IM',
      theme: ThemeData(
        colorSchemeSeed: const Color(0xFF17202A),
        useMaterial3: true,
      ),
      home: const AuthGate(),
    );
  }
}
