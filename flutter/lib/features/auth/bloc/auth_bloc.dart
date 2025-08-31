import 'package:etotop_client/src/rust/api/auth/auth_api.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

part 'auth_events.dart';
part 'auth_state.dart';

class AuthBloc extends Bloc<AuthEvent, AuthState> {
  final AuthApi authApi;
  AuthBloc({required this.authApi}) : super(const AuthState.unknown()) {
    on<AppStarted>((event, emit) async {
      final authState = await authApi.getAuthState();
      switch (authState) {
        case 1:
          emit(const AuthState.noUrl());
          break;
        case 2:
          emit(const AuthState.unauthenticated());
          break;
        case 3:
          emit(const AuthState.authenticated());
          break;
        default:
          emit(const AuthState.noUrl());
      }
    });
    on<LoggedIn>((event, emit) => emit(const AuthState.authenticated()));
    on<LoggedOut>((event, emit) => emit(const AuthState.unauthenticated()));
  }
}
