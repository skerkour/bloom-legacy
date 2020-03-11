import 'dart:async';

import 'package:bloom/ui/kernel/blocs/bloc_provider.dart';
import 'package:flutter/services.dart';
import 'package:barcode_scan/barcode_scan.dart';

class ChatTabBloc extends BlocBase {
  ChatTabBloc();

  @override
  void dispose() {}

  static Future<String> scan() async {
    try {
      final String barcode = await BarcodeScanner.scan();
      return barcode;
    } on PlatformException catch (e) {
      if (e.code == BarcodeScanner.CameraAccessDenied) {
        return Future<String>.error('Error accessing camera');
      } else {
        return Future<String>.error('Unknown error');
      }
    } on FormatException {
      return Future<String>.error('Error: incorrect format');
    } catch (e) {
      return Future<String>.error('Unknown error');
    }
  }
}
