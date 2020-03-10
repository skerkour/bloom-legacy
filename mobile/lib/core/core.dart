import 'dart:convert';
import 'dart:ffi' as ffi;
import 'package:flutter/foundation.dart';
import 'package:ffi/ffi.dart';

class Message<P> {
  Message(this.method, this.params);
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

Message<P> _toMessage<P>(String method, P params) {
  return Message<P>(method, params);
}

Future<void> coreInit() async {
  return await coreCall('core.init', Empty());
}

Future<Map<String, dynamic>> coreCall<P>(String method, P params) async {
  final Message<P> message = _toMessage(method, params);
  return await compute(_coreCall, message);
}

Map<String, dynamic> _coreCall<P>(Message<P> message) {
  final String jsonMessage = jsonEncode(message.toJson());
  debugPrint('jsonMessage: $jsonMessage');
  final ffi.Pointer<Utf8> cMessage = Utf8.toUtf8(jsonMessage);

  if (_call == null) {
    final ffi.DynamicLibrary dylib = ffi.DynamicLibrary.open('libbloomcore.so');
    _call = dylib.lookupFunction<BlmCoreCall, BlmCoreCall>('blmcore_call');
  }

  final ffi.Pointer<Utf8> res = _call(cMessage);
  free(cMessage);
  debugPrint('res: $res');

  final String retMessage = Utf8.fromUtf8(res);
  free(res);
  debugPrint('retMessage: $retMessage');

  final Map<String, dynamic> ret = jsonDecode(retMessage);
  if (ret['error'] != null) {
    final String errorMessage = ret['error']['message'];
    throw errorMessage;
  }
  return ret['data'];
}
