import 'dart:convert';
import 'dart:ffi' as ffi;

import 'package:ffi/ffi.dart';
import 'package:flutter/foundation.dart';

typedef RustHandleMessage = ffi.Pointer<Utf8> Function(ffi.Pointer<Utf8>);

typedef RustFreeFunction = ffi.Void Function(ffi.Pointer<Utf8>);
typedef RustFree = void Function(ffi.Pointer<Utf8>);

RustFree _coreFree;
RustHandleMessage _handleMessage;

class _Native {
  _Native() {
    final ffi.DynamicLibrary dylib = ffi.DynamicLibrary.open('libcore_ffi.so');
    _handleMessage = dylib.lookupFunction<RustHandleMessage, RustHandleMessage>(
        'core_handle_message');
    _coreFree = dylib
        .lookup<ffi.NativeFunction<RustFreeFunction>>('core_free')
        .asFunction();
  }

  Map<String, dynamic> call(String message) {
    final ffi.Pointer<Utf8> cMessage = Utf8.toUtf8(message);
    debugPrint('cmessage: $cMessage');

    final ffi.Pointer<Utf8> res = _handleMessage(cMessage);
    free(cMessage);

    final String retStr = Utf8.fromUtf8(res);
    _coreFree(res);

    final Map<String, dynamic> ret = jsonDecode(retStr);
    if (ret['type'] == 'error') {
      final String errorMessage = ret['data']['message'];
      throw errorMessage;
    }
    return ret;
  }
}

_Native coreFfi = _Native();
