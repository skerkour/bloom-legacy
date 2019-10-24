import 'dart:async';
import 'dart:convert';
import 'package:bloom/bloom/kernel/blocs/bloc_provider.dart';
import 'package:bloom/native/core_ffi.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

import '../messages.dart';

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

    final String message = jsonEncode(AuthGuiRegistrationVerify(
      id: pendingAccountId,
      code: code,
    ));

    Map<String, dynamic> json;

    try {
      json = await compute(RegistrationVerifyBloc._nativeCall, message);
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

  static Map<String, dynamic> _nativeCall<T>(T message) {
    final String jsonPayload = jsonEncode(message);
    debugPrint('input: $jsonPayload');
    final Map<String, dynamic> res = coreFfi.call(jsonPayload);
    debugPrint('output: $res');
    return res;
  }
}
