import 'dart:async';
import 'dart:convert';
import 'package:bloom/bloom/calendar/models/event.dart';
import 'package:bloom/bloom/kernel/blocs/bloc_provider.dart';
import 'package:bloom/native/core_ffi.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

import '../messages.dart';

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

    final String message = jsonEncode(AuthGuiSignIn(
      username: username,
      password: password,
    ));

    Map<String, dynamic> json;

    try {
      json = await compute(SignInBloc._nativeCall, message);
    } catch (err) {
      rethrow;
    } finally {
      _isLoadingStream.add(false);
    }

    return json;
  }

  static Map<String, dynamic> _nativeCall(String message) {
    return coreFfi.call(message);
  }
}
