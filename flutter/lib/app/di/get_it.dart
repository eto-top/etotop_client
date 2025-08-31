import 'package:etotop_client/app/router/app_router.dart';
import 'package:etotop_client/app/web/http_client.dart';
import 'package:etotop_client/src/rust/api/auth/auth_api.dart';
import 'package:etotop_client/src/rust/api/core_bridge/core_bridge.dart';
import 'package:get_it/get_it.dart';
import 'package:http/http.dart' as http;
import 'package:shared_preferences/shared_preferences.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:etotop_client/app/storage/key_value_storage.dart';
import 'package:etotop_client/app/storage/key_value_secure_storage.dart';
import 'package:etotop_client/features/auth/bloc/auth_bloc.dart';
import 'package:flutter/foundation.dart'; // for kDebugMode

final coreConfig = "core_config";

final GetIt getIt = GetIt.instance;

Future<void> init() async {
  getIt.registerLazySingleton<SharedPreferencesAsync>(
    () => SharedPreferencesAsync(),
  );
  getIt.registerLazySingleton<FlutterSecureStorage>(
    () => FlutterSecureStorage(),
  );
  getIt.registerLazySingleton<http.Client>(() => http.Client());
  getIt.registerSingleton(HttpClient(client: getIt<http.Client>()));
  getIt.registerSingleton(KeyValueStorage(sharedPreferences: getIt()));
  getIt.registerSingleton(KeyValueSecureStorage(preferences: getIt()));
  getIt.registerLazySingleton<AuthBloc>(() => AuthBloc(authApi: getIt()));
  getIt.registerLazySingleton<AppRouter>(() => AppRouter(authBloc: getIt()));
  getIt.registerLazySingleton<AuthApi>(
    () => getAuthApi(bridge: getIt<CoreBridge>()),
  );

  if (kDebugMode) {
    final kv = getIt<KeyValueStorage>();
    Future<String> getConfig() async => kv.getStringValue(coreConfig);
    Future<dynamic> setConfig(String s) async =>
        kv.setStringValue(coreConfig, s);
    final httpClient = getIt<HttpClient>();
    Future<HttpResponseImpl> handleHttpRequest(HttpRequestArgsImpl r) async =>
        httpClient.handleRequest(r);
    getIt.registerLazySingleton<CoreBridge>(
      () => CoreBridge(
        getConfig: getConfig,
        saveConfig: setConfig,
        updateAuth: () {
          getIt<AuthBloc>().add(AppStarted());
        },
        httpRequest: handleHttpRequest,
      ),
    );
  } else {
    final kv = getIt<KeyValueSecureStorage>();
    Future<String> getConfig() async => kv.getStringValue(coreConfig);
    Future<dynamic> setConfig(String s) async =>
        kv.setStringValue(coreConfig, s);
    final httpClient = getIt<HttpClient>();
    Future<HttpResponseImpl> handleHttpRequest(HttpRequestArgsImpl r) async =>
        httpClient.handleRequest(r);
    getIt.registerLazySingleton<CoreBridge>(
      () => CoreBridge(
        getConfig: getConfig,
        saveConfig: setConfig,
        updateAuth: () {
          getIt<AuthBloc>().add(AppStarted());
        },
        httpRequest: handleHttpRequest,
      ),
    );
  }
}
