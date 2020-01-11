import 'dart:async';
import 'package:bloom/bloom/auth/core/messages.dart';
import 'package:bloom/bloom/auth/core/methods.dart';
import 'package:bloom/bloom/kernel/blocs/bloc_provider.dart';
import 'package:bloom/core.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

class RegistrationVerifyBloc extends BlocBase {
  RegistrationVerifyBloc();

  final StreamController<bool> _isLoadingController =
      StreamController<bool>.broadcast();
  StreamSink<bool> get _isLoadingStream => _isLoadingController.sink;
  Stream<bool> get isLoadingStream => _isLoadingController.stream;

  @override
  void dispose() {
    _isLoadingController.close();
  }

  Future<Map<String, dynamic>> verify(
    String pendingAccountId,
    String code,
  ) async {
    debugPrint('RegistrationVerifyBloc.complete called');
    _isLoadingController.add(true);

    code = _cleanCode(code);

    final AuthGuiVerifyRegistration message = AuthGuiVerifyRegistration(
      id: pendingAccountId,
      code: code,
    );

    Map<String, dynamic> json;

    try {
      json = await coreCall(AuthMethod.verify_registration, message);
    } catch (err) {
      rethrow;
    } finally {
      _isLoadingStream.add(false);
    }

    return json;
  }

  String _cleanCode(String code) {
    return code.substring(0, 4) + code.substring(5);
  }
}
