import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:window_manager/window_manager.dart';
import 'package:wlist_ui/pages/login.dart';

import 'generated/l10n.dart';

void main() async {
  runApp(const WlistApp());
  try {
    if (Platform.isWindows || Platform.isLinux || Platform.isMacOS) {
      WidgetsFlutterBinding.ensureInitialized();
      await windowManager.ensureInitialized();
      WindowOptions windowOptions = WindowOptions(
        size: const Size(800, 600),
        center: true,
        minimumSize: const Size(400, 600),
        title: S.current.title,
      );
      windowManager.waitUntilReadyToShow(windowOptions, () async {
        await windowManager.show();
        await windowManager.focus();
      });
    }
  } catch (_) {
  }
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
