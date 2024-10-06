import 'dart:math';

import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:window_manager/window_manager.dart';

import '../main.dart';

Future<void> setWindowSize({Size? min, Size? max, Size? current}) async {
  if (isDesktop) {
    if (min != null) { windowManager.setMinimumSize(min); }
    if (max != null) { windowManager.setMaximumSize(max); }
    if (current != null && !await windowManager.isFullScreen()) {
      await windowManager.setSize(current, animate: true);
      await windowManager.center(animate: true);
    }
  }
}

const String currentVersion = "0.0.1";
const String updateUrl = "https://"; // TODO: update url

SharedPreferencesAsync sharedPreferences = SharedPreferencesAsync();

Duration? toastShortTime = const Duration(seconds: 2);
Duration? toastLongTime = const Duration(seconds: 5);

Color themeWhite(BuildContext context) => Theme.of(context).brightness == Brightness.light ? Colors.white : Colors.black;

String generateRandomString(int length) {
  const all = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
  var random = Random.secure();
  var string = StringBuffer();
  for (int i = 0; i < length; i++) {
    var char = all[random.nextInt(all.length)];
    string.write(char);
  }
  return string.toString();
}
