import 'package:etotop_client/app/router/app_router.dart';
import 'package:etotop_client/features/auth/bloc/auth_bloc.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter/foundation.dart'; // for kDebugMode
import 'package:etotop_client/app/di/get_it.dart';
import 'package:etotop_client/app/theme/theme_bloc.dart';
import 'package:etotop_client/app/theme/app_theme.dart';
import 'l10n/app_localizations.dart';
import 'package:etotop_client/src/rust/api/core_bridge/core_bridge.dart';
import 'package:etotop_client/src/rust/frb_generated.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await init();
  await RustLib.init();

  final coreBridge = getIt<CoreBridge>();
  await coreBridge.init();
  getIt<AuthBloc>().add(AppStarted());
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  late final AuthBloc _authBloc;
  late final AppRouter _appRouter;

  @override
  void initState() {
    super.initState();
    _authBloc = getIt<AuthBloc>();
    _appRouter = getIt<AppRouter>();
  }

  @override
  Widget build(BuildContext context) {
    return MultiBlocProvider(
      providers: [
        BlocProvider<AuthBloc>.value(value: _authBloc),
        BlocProvider<ThemeBloc>(
          create: (context) => ThemeBloc()..add(LoadThemeEvent()),
        ),
      ],
      child: BlocBuilder<ThemeBloc, ThemeState>(
        builder: (context, state) {
          return MaterialApp.router(
            routerConfig: _appRouter.router,
            localizationsDelegates: AppLocalizations.localizationsDelegates,
            supportedLocales: AppLocalizations.supportedLocales,
            locale: kDebugMode ? const Locale('ru') : null,
            themeMode: state.themeMode,
            theme: AppTheme.getTheme(ThemeMode.light, context),
            darkTheme: AppTheme.getTheme(ThemeMode.dark, context),
          );
        },
      ),
    );
  }

  @override
  void dispose() {
    _authBloc.close();
    super.dispose();
  }
}
