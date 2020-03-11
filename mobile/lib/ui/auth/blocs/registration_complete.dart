import 'dart:async';
import 'package:bloom/core/users/messages.dart';
import 'package:bloom/core/users/methods.dart';
import 'package:bloom/ui/kernel/blocs/bloc_provider.dart';
import 'package:bloom/core/core.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

class RegistrationCompleteBloc extends BlocBase {
  RegistrationCompleteBloc();

  final StreamController<bool> _isLoadingController =
      StreamController<bool>.broadcast();
  StreamSink<bool> get _isLoadingStream => _isLoadingController.sink;
  Stream<bool> get isLoadingStream => _isLoadingController.stream;

  @override
  void dispose() {
    _isLoadingController.close();
  }

  Future<Map<String, dynamic>> complete(
    String pendingAccountId,
    String username,
    String password,
  ) async {
    debugPrint('RegistrationCompleteBloc.complete called');
    _isLoadingController.add(true);

    final AuthGuiCompleteRegistration message = AuthGuiCompleteRegistration(
      id: pendingAccountId,
      username: username,
      password: password,
    );

    Map<String, dynamic> json;

    try {
      json = await coreCall(AuthMethod.complete_registration, message);
    } catch (err) {
      rethrow;
    } finally {
      _isLoadingStream.add(false);
    }

    return json;
  }
}
