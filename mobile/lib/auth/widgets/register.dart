import 'dart:ffi' as ffi;
// import 'package:bloom/chat/views/chat.dart';
import 'package:bloom/kernel/widgets/password_field.dart';
import 'package:ffi/ffi.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

typedef RusthandleMessageFunction = ffi.Pointer<Utf8> Function(
    ffi.Pointer<Utf8>);
// typedef NativeDoubleInputFunction= int Function(int);

class Register extends StatefulWidget {
  const Register({Key key}) : super(key: key);

  @override
  _RegisterState createState() => _RegisterState();
}

class _RegisterState extends State<Register> {
  TextStyle style = const TextStyle(fontFamily: 'Montserrat', fontSize: 20.0);
  bool isLoading = false;

  @override
  Widget build(BuildContext context) {
    return Container(
      child: Padding(
        padding: const EdgeInsets.all(36.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.center,
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            // SizedBox(
            //   height: 155.0,
            //   child: Image.asset(
            //     "assets/logo.png",
            //     fit: BoxFit.contain,
            //   ),
            // ),
            // SizedBox(height: 45.0),
            TextFormField(
              decoration: const InputDecoration(labelText: 'Full name'),
            ),
            const SizedBox(height: 25.0),
            TextFormField(
              decoration: const InputDecoration(labelText: 'Email'),
            ),
            const SizedBox(height: 25.0),
            const PasswordField(
              labelText: 'Password',
            ),
            const SizedBox(
              height: 35.0,
            ),
            _buildRegisterButton(context),
            const SizedBox(
              height: 15.0,
            ),
            isLoading
                ? const CircularProgressIndicator()
                : Container(
                    width: 0, height: 0), // TODO(z0mbie42): remove ugly hack
          ],
        ),
      ),
    );
  }

  Material _buildRegisterButton(BuildContext context) {
    return Material(
      elevation: 5.0,
      borderRadius: BorderRadius.circular(6.0),
      color: Colors.blue,
      child: MaterialButton(
        minWidth: MediaQuery.of(context).size.width,
        padding: const EdgeInsets.fromLTRB(20.0, 15.0, 20.0, 15.0),
        onPressed: () {
          _onRegisterButtonPressed();
        },
        child: Text('Register',
            textAlign: TextAlign.center,
            style: style.copyWith(
                color: Colors.white, fontWeight: FontWeight.bold)),
      ),
    );
  }

  Future<void> _onRegisterButtonPressed() async {
    setState(() {
      isLoading = true;
    });

    const String message = '''''';

    final String res = await compute(_RegisterState.lol, message);
    debugPrint(res);
    setState(() {
      isLoading = false;
    });
  }

  static String lol(String message) {
    final ffi.DynamicLibrary dylib = ffi.DynamicLibrary.open('libcore_ffi.so');
    final RusthandleMessageFunction handleMessage = dylib.lookupFunction<
        RusthandleMessageFunction, RusthandleMessageFunction>('handle_message');

    final ffi.Pointer<Utf8> cMessage = Utf8.toUtf8(message).cast();

    final ffi.Pointer<Utf8> res = handleMessage(cMessage);
    final String ret = Utf8.fromUtf8(res);
    res.free();
    return ret;
  }
}
