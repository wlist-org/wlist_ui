import 'package:flutter/material.dart';
import 'package:window_manager/window_manager.dart';
import 'package:wlist_ui/generated/rust/api/common.dart';
import 'package:wlist_ui/widgets/loading_text.dart';

import '../generated/l10n.dart';
import '../generated/rust/api/web/version.dart';
import '../generated/rust/frb_generated.dart';

class BootPage extends StatefulWidget {
  const BootPage({super.key});

  void initWindow() {
    windowManager.setMinimumSize(const Size(300, 400));
    windowManager.setSize(const Size(480, 600));
  }

  @override
  State<BootPage> createState() => _BootPageState();
}

bool initialized = false;
String? target;

class _BootPageState extends State<BootPage> {
  String? text;

  @override
  void initState() {
    super.initState();
    if (!initialized) {
      initialized = true;
      doInitialize(context);
    } else if (target != null) {
      Navigator.of(context).popAndPushNamed(target!);
    }
  }

  void doInitialize(BuildContext context) async {
    await RustLib.init();
    setState(() => text = S.of(context).boot);
    await initialize(dataDirectory: "run/data", cacheDirectory: "run/cache");
    setState(() => text = S.of(context).boot_check_version);
    var version = FVersionState.latest; // TODO: await checkVersion();
    if (version == FVersionState.unavailable) {
      target = "/version_unavailable";
    } else {
      // ignore difference between `latest` and `unavailable`.
      // TODO: login.
      target = "/login";
    }
    if (context.mounted) {
      Navigator.of(context).popAndPushNamed(target!);
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Center(
        child: LoadingText(text: text),
      )
    );
  }
}
