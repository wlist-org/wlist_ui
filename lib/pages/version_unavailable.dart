
import 'package:flutter/material.dart';
import 'package:window_manager/window_manager.dart';
import 'package:wlist_ui/main.dart';

import '../generated/l10n.dart';

class VersionUnavailablePage extends StatelessWidget {
  const VersionUnavailablePage({super.key});

  void initWindow() {
    windowManager.setMinimumSize(const Size(400, 200));
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: Text(
          S.of(context).version_unavailable(currentVersion, updateUrl),
          textAlign: TextAlign.center,
        ),
      ),
    );
  }
}
