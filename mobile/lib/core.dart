import 'dart:convert';
import 'dart:ffi' as ffi;

import 'package:ffi/ffi.dart';
import 'package:flutter/foundation.dart';

class Payload<P> {
  Payload(this.method, this.params);
  String method;
  P params;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'method': method,
      'params': jsonEncode(params),
    };
    return data;
  }
}

class Empty {
  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
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
    _call = dylib.lookupFunction<BlmCoreCall, BlmCoreCall>('blmcore_call');
  }

  Payload<P> toPayload<M, P>(M method, P params) {
    return Payload<P>(method.toString().toLowerCase(), params);
  }

  Map<String, dynamic> call<P>(Payload<P> payload) {
    debugPrint('blmcore.call($payload)');

    final ffi.Pointer<Utf8> cPayload =
        Utf8.toUtf8(jsonEncode(payload.toJson()));
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
