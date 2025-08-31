part of 'auth_bloc.dart';

enum AuthStatus { unknown, noUrl, authenticated, unauthenticated }

class AuthState {
  final AuthStatus status;
  const AuthState._(this.status);
  const AuthState.unknown() : this._(AuthStatus.unknown);
  const AuthState.noUrl() : this._(AuthStatus.noUrl);
  const AuthState.unauthenticated() : this._(AuthStatus.unauthenticated);
  const AuthState.authenticated() : this._(AuthStatus.authenticated);
}
