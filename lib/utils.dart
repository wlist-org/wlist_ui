import 'package:shared_preferences/shared_preferences.dart';

const String currentVersion = "0.0.1";
const String updateUrl = "https://"; // TODO: update url

SharedPreferencesAsync sharedPreferences = SharedPreferencesAsync();

Duration? toastShortTime = const Duration(seconds: 2);
Duration? toastLongTime = const Duration(seconds: 5);
