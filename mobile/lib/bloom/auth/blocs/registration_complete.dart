import 'dart:async';
import 'dart:convert';
import 'package:bloom/bloom/kernel/blocs/bloc_provider.dart';
import 'package:bloom/native/core_ffi.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

import '../messages.dart';

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

    final String message = jsonEncode(AuthGuiRegistrationComplete(
      id: pendingAccountId,
      username: username,
      password: password,
    ));

    Map<String, dynamic> json;

    try {
      json = await compute(RegistrationCompleteBloc._nativeCall, message);
    } catch (err) {
      rethrow;
    } finally {
      _isLoadingStream.add(false);
    }

    return json;
  }

  static Map<String, dynamic> _nativeCall<T>(T message) {
    final String jsonPayload = jsonEncode(message);
    debugPrint('input: $jsonPayload');
    final Map<String, dynamic> res = coreFfi.call(jsonPayload);
    debugPrint('output: $res');
    return res;
  }
}
