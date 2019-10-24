import 'dart:async';
import 'dart:convert';
import 'package:bloom/bloom/kernel/blocs/bloc_provider.dart';
import 'package:bloom/native/core_ffi.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

import '../messages.dart';

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

  Future<AuthGuiRegistrationStarted> start(
      String displayName, String email) async {
    debugPrint('RegistrationStartBloc.signIn called');
    _isLoadingController.add(true);

    final String message = jsonEncode(AuthRegistrationStart(
      displayName: displayName,
      email: email,
    ));

    Map<String, dynamic> json;
    AuthGuiRegistrationStarted ret;

    try {
      json = await compute(RegistrationStartBloc._nativeCall, message);
      ret = AuthGuiRegistrationStarted.fromJson(json);
    } catch (err) {
      rethrow;
    } finally {
      _isLoadingStream.add(false);
    }

    return ret;
  }

  static Map<String, dynamic> _nativeCall<T>(T message) {
    final String jsonPayload = jsonEncode(message);
    debugPrint('input: $jsonPayload');
    final Map<String, dynamic> res = coreFfi.call(jsonPayload);
    debugPrint('output: $res');
    return res;
  }
}
