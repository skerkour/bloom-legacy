import 'dart:async';
import 'package:bloom/core/users/messages.dart';
import 'package:bloom/core/users/methods.dart';
import 'package:bloom/ui/kernel/blocs/bloc_provider.dart';
import 'package:bloom/core/core.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

class RegistrationStartBloc extends BlocBase {
  RegistrationStartBloc();

  final StreamController<bool> _isLoadingController =
      StreamController<bool>.broadcast();
  StreamSink<bool> get _isLoadingStream => _isLoadingController.sink;
  Stream<bool> get isLoadingStream => _isLoadingController.stream;

  @override
  void dispose() {
    _isLoadingController.close();
  }

  Future<AuthRegistrationStarted> start(
    String displayName,
    String email,
  ) async {
    debugPrint('RegistrationStartBloc.start called');
    _isLoadingController.add(true);

    final AuthStartRegistration message = AuthStartRegistration(
      displayName: displayName,
      email: email,
    );

    Map<String, dynamic> json;
    AuthRegistrationStarted ret;

    try {
      json = await coreCall(AuthMethod.start_registration, message);
      ret = AuthRegistrationStarted.fromJson(json);
    } catch (err) {
      rethrow;
    } finally {
      _isLoadingStream.add(false);
    }

    return ret;
  }
}
