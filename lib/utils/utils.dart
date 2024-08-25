import 'dart:math';

import 'package:shared_preferences/shared_preferences.dart';

const String currentVersion = "0.0.1";
const String updateUrl = "https://"; // TODO: update url

SharedPreferencesAsync sharedPreferences = SharedPreferencesAsync();

Duration? toastShortTime = const Duration(seconds: 2);
Duration? toastLongTime = const Duration(seconds: 5);

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
