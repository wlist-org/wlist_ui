import 'package:flutter/material.dart';
import 'package:wlist_ui/pages/login.dart';

import 'generated/l10n.dart';

void main() {
  runApp(const WlistApp());
}

class WlistApp extends StatelessWidget {
  const WlistApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      theme: ThemeData(
        colorSchemeSeed: Colors.red,
      ),
      localizationsDelegates: const [S.delegate],
      supportedLocales: S.delegate.supportedLocales,
      initialRoute: "/login",
      routes: {
        "/login": (context) => const LoginPage(),
      },
    );
  }
}
