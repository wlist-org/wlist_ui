import 'package:wlist_ui/utils/utils.dart';

Future<String> _generateDeviceId() async {
  // TODO: device id.
  return generateRandomString(128);
}

Future<String> getDeviceId() async {
  var cached = await sharedPreferences.getString("web.user_id");
  if (cached != null) {
    return cached;
  }
  return await resetDeviceId();
}

Future<String> resetDeviceId() async {
  var id = await _generateDeviceId();
  await sharedPreferences.setString("web.user_id", id);
  return id;
}
