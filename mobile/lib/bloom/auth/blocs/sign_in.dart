import 'dart:async';
import 'package:bloom/bloom/auth/core/messages.dart';
import 'package:bloom/bloom/auth/core/methods.dart';
import 'package:bloom/bloom/kernel/blocs/bloc_provider.dart';
import 'package:bloom/core.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

class SignInBloc extends BlocBase {
  SignInBloc();

  final StreamController<bool> _isLoadingController =
      StreamController<bool>.broadcast();
  StreamSink<bool> get _isLoadingStream => _isLoadingController.sink;
  Stream<bool> get isLoadingStream => _isLoadingController.stream;

  @override
  void dispose() {
    _isLoadingController.close();
  }

  Future<Map<String, dynamic>> signIn(String username, String password) async {
    debugPrint('SignInBloc.signIn called');
    _isLoadingController.add(true);

    final AuthSignIn message = AuthSignIn(
      username: username,
      password: password,
    );

    Map<String, dynamic> json;

    try {
      json = await coreCall(AuthMethod.sign_in, message);
    } catch (err) {
      rethrow;
    } finally {
      _isLoadingStream.add(false);
    }

    return json;
  }
}
