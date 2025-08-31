import 'package:etotop_client/app/di/get_it.dart';
import 'package:etotop_client/app/router/router_refresh_notifier.dart';
import 'package:etotop_client/screens/login_screen.dart';
import 'package:etotop_client/screens/server_url_screen.dart';
import 'package:etotop_client/screens/splash_screen.dart';
import 'package:etotop_client/src/rust/api/auth/auth_api.dart';
import 'package:go_router/go_router.dart';
import 'package:etotop_client/features/auth/bloc/auth_bloc.dart';

class AppRouter {
  final AuthBloc authBloc;

  AppRouter({required this.authBloc});

  late final GoRouter router = GoRouter(
    refreshListenable: RouterRefreshNotifier(authBloc.stream),
    initialLocation: '/splash',
    routes: [
      GoRoute(
        path: '/splash',
        builder: (context, state) => const SplashScreen(),
      ),
      GoRoute(
        path: '/enter_url',
        builder: (context, state) => const ServerUrlScreen(),
      ),
      GoRoute(
        path: '/login',
        builder: (context, state) => LoginScreen(authApi: getIt<AuthApi>()),
      ),
      /*GoRoute(
        path: '/home',
        builder: (context, state) => const HomeScreen(),
      ),*/
    ],
    redirect: (context, state) {
      final authState = authBloc.state;
      final status = authState.status;

      if (status == AuthStatus.unknown) {
        return '/splash';
      }

      if (status == AuthStatus.noUrl) {
        return '/enter_url';
      }

      if (status == AuthStatus.unauthenticated) {
        return '/login';
      }

      if (status == AuthStatus.authenticated) {
        return '/home';
      }

      return null;
    },
  );
}
