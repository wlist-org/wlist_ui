import 'dart:async';

import 'package:flutter/material.dart';
import 'package:path_provider/path_provider.dart';
import 'package:toastification/toastification.dart';
import 'package:window_manager/window_manager.dart';
import 'package:wlist_ui/main.dart';
import 'package:wlist_ui/utils/device_id.dart';
import 'package:wlist_ui/utils/utils.dart';
import 'package:wlist_ui/widgets/loading_text.dart';

import '../generated/l10n.dart';
import '../generated/rust/api/common.dart';
import '../generated/rust/api/common/exceptions.dart';
import '../generated/rust/api/web/account.dart';
import '../generated/rust/api/web/register.dart';
import '../generated/rust/api/web/version.dart';
import '../generated/rust/frb_generated.dart';

class BootPage extends StatefulWidget {
  const BootPage({super.key});

  void initWindow() {
    if (isDesktop) {
      windowManager.setMinimumSize(const Size(300, 400));
      windowManager.setSize(const Size(480, 600));
    }
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
      doInitialize(context).then((_) {
        if (target == null) {
          initialized = false;
          return;
        }
        var context = this.context;
        if (context.mounted) {
          Navigator.of(context).pushReplacementNamed(target!);
        }
      });
    } else if (target != null) {
      Navigator.of(context).popAndPushNamed(target!);
    }
  }

  Future<void> doInitialize(BuildContext context) async {
    await RustLib.init();
    setState(() => text = S.of(context).boot);
    var data = await getApplicationDocumentsDirectory();
    var cache = await getApplicationSupportDirectory(); // The native cache directory only contains support files now.
    await initialize(dataDirectory: data.path, cacheDirectory: cache.path);
    setState(() => text = S.of(context).boot_check_version);
    FVersionState version;
    try {
      version = await checkVersion();
    } on UniverseError catch (e) {
      if (e case UniverseError_Network()) {
        if (context.mounted) {
          toastification.show(
            context: context,
            title: Text(S.of(context).network_error),
            autoCloseDuration: toastShortTime,
          );
        }
        // TODO: offline mode
        version = FVersionState.unavailable;
      } else {
        rethrow;
      }
    }
    if (version == FVersionState.unavailable) {
      target = "/version_unavailable";
      return;
    }
    // ignore difference between `latest` and `unavailable`.
    var userId = await sharedPreferences.getString("web.user_id");
    var password = await sharedPreferences.getString("web.password");
    if (userId != null && password != null) {
      setState(() => text = S.of(context).boot_login);
      try {
        await login(userId: userId, password: password);
        target = "/main";
        return;
      } on UniverseError catch (e) {
        // TODO
        if (context.mounted) {
          toastification.show(
            context: context,
            title: Text(S.of(context).login_cached_password_mismatch),
            autoCloseDuration: toastShortTime,
          );
        }
      }
    }
    setState(() => text = S.of(context).boot_register);
    password = generateRandomString(6);
    var deviceId = await getDeviceId();
    userId = null;
    while (userId == null) {
      userId = await registerAsGuest(deviceId: deviceId, password: password);
      if (userId == null) {
        deviceId = await resetDeviceId();
      }
    }
    await sharedPreferences.setString("web.user_id", userId);
    await sharedPreferences.setString("web.password", password);
    target = "/main";
    return;
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
