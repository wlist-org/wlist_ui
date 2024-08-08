import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:window_manager/window_manager.dart';
import 'package:wlist_ui/pages/boot.dart';
import 'package:wlist_ui/pages/login.dart';
import 'package:wlist_ui/pages/version_unavailable.dart';

import 'generated/l10n.dart';

bool isDesktop = !kIsWeb && (Platform.isWindows || Platform.isLinux || Platform.isMacOS);

const String currentVersion = "0.0.1";
const String updateUrl = "https://"; // TODO: update url

void main() async {
  runApp(const WlistApp());
  if (isDesktop) {
    await windowManager.ensureInitialized();
    await windowManager.waitUntilReadyToShow(WindowOptions(
      center: true,
      title: S.current.title,
    ));
    await windowManager.show();
    await windowManager.focus();
  }
}

class WlistApp extends StatelessWidget {
  const WlistApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      themeMode: ThemeMode.system,
      theme: ThemeData(colorSchemeSeed: Colors.lightBlue, brightness: Brightness.light),
      darkTheme: ThemeData(colorSchemeSeed: Colors.indigo, brightness: Brightness.dark),
      localizationsDelegates: const [
        S.delegate,
        GlobalMaterialLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
      ],
      supportedLocales: S.delegate.supportedLocales,
      routes: {
        "/": (context) => const BootPage()..initWindow(),
        "/version_unavailable": (context) => const VersionUnavailablePage()..initWindow(),
        "/login": (context) => const LoginPage()..initWindow(),
      },
    );
  }
}
