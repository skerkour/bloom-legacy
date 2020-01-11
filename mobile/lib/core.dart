import 'dart:convert';
import 'dart:ffi' as ffi;
import 'package:flutter/foundation.dart';
import 'package:ffi/ffi.dart';

class Payload<P> {
  Payload(this.method, this.params);
  String method;
  P params;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'method': method,
      'params': params,
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

Payload<P> _toPayload<P>(String method, P params) {
  return Payload<P>(method, params);
}

Future<Map<String, dynamic>> coreCall<P>(String method, P params) async {
  final Payload<P> payload = _toPayload(method, params);
  return await compute(_coreCall, payload);
}

Map<String, dynamic> _coreCall<P>(Payload<P> payload) {
  final String jsonPayload = jsonEncode(payload.toJson());
  debugPrint('jsonPayload: $jsonPayload');
  final ffi.Pointer<Utf8> cPayload = Utf8.toUtf8(jsonPayload);

  if (_call == null) {
    final ffi.DynamicLibrary dylib = ffi.DynamicLibrary.open('libblmcore.so');
    _call = dylib.lookupFunction<BlmCoreCall, BlmCoreCall>('blmcore_call');
  }

  final ffi.Pointer<Utf8> res = _call(cPayload);
  free(cPayload);
  debugPrint('res: $res');

  final String retPayload = Utf8.fromUtf8(res);
  free(res);
  debugPrint('retPayload: $retPayload');

  final Map<String, dynamic> ret = jsonDecode(retPayload);
  if (ret['type'] == 'error') {
    final String errorMessage = ret['data']['message'];
    throw errorMessage;
  }
  return ret;
}
