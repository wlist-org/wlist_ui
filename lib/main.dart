import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:window_manager/window_manager.dart';
import 'package:wlist_ui/pages/login.dart';

import 'generated/l10n.dart';

void main() async {
  runApp(const WlistApp());
  if (Platform.isWindows || Platform.isLinux || Platform.isMacOS) {
    WidgetsFlutterBinding.ensureInitialized();
    windowManager.ensureInitialized().then((_) => {
      windowManager.waitUntilReadyToShow(WindowOptions(
        size: const Size(800, 600),
        center: true,
        minimumSize: const Size(300, 400),
        title: S.current.title,
      ))
    })
    .then((_) => windowManager.show())
    .then((_) => windowManager.focus());
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
      initialRoute: "/login",
      routes: {
        "/login": (context) => const LoginPage(),
      },
    );
  }
}
