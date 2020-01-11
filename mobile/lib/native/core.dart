import 'dart:convert';
import 'dart:ffi' as ffi;

import 'package:ffi/ffi.dart';
import 'package:flutter/foundation.dart';



class Payload<T> {
  Payload(this.method, this.params);
  String method;
  T params;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'method': method,
      'params': jsonEncode(params),
    };
    return data;
  }
}

typedef BlmCoreCall = ffi.Pointer<Utf8> Function(ffi.Pointer<Utf8>);

typedef RustFreeFunction = ffi.Void Function(ffi.Pointer<Utf8>);
// typedef RustFree = void Function(ffi.Pointer<Utf8>);

// RustFree _coreFree;
BlmCoreCall _call;

class BlmCore {
  BlmCore() {
    final ffi.DynamicLibrary dylib = ffi.DynamicLibrary.open('libblmcore.so');
    _call = dylib.lookupFunction<BlmCoreCall, BlmCoreCall>(
        'blmcore_call');
    // _coreFree = dylib
    //     .lookup<ffi.NativeFunction<RustFreeFunction>>('blmcore_free')
    //     .asFunction();
  }

  Map<String, dynamic> call<T>(String method, T params) {
    debugPrint('blmcore.call($method, $params)');

    final Payload<T> paylaod = Payload<T>(method, params);
    final ffi.Pointer<Utf8> cPayload = Utf8.toUtf8(jsonEncode(paylaod.toJson()));
    debugPrint('cPayload: $cPayload');

    final ffi.Pointer<Utf8> res = _call(cPayload);
    free(cPayload);

    final String retStr = Utf8.fromUtf8(res);
    free(res);

    final Map<String, dynamic> ret = jsonDecode(retStr);
    if (ret['type'] == 'error') {
      final String errorMessage = ret['data']['message'];
      throw errorMessage;
    }
    return ret;
  }
}

BlmCore core = BlmCore();
