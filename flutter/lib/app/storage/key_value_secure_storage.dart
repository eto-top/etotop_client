import 'package:flutter_secure_storage/flutter_secure_storage.dart';

class KeyValueSecureStorage {
  final FlutterSecureStorage preferences;

  KeyValueSecureStorage({required this.preferences});

  Future setStringValue(String key, String value) async {
    await preferences.write(key: key, value: value);
  }

  Future<String> getStringValue(String key) async {
    return await preferences.read(key: key) ?? '';
  }
}
