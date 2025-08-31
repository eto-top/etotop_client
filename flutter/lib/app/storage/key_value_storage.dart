import 'package:shared_preferences/shared_preferences.dart';

class KeyValueStorage {
  final SharedPreferencesAsync sharedPreferences;

  KeyValueStorage({required this.sharedPreferences});

  Future setStringValue(String key, String value) async {
    await sharedPreferences.setString(key, value);
  }

  Future<String> getStringValue(String key) async {
    return await sharedPreferences.getString(key) ?? '';
  }
}
