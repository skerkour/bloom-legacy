import 'dart:convert';

import 'package:bloom/bloom/auth/views/registration_complete.dart';
import 'package:bloom/libs/masked_text_controller.dart';
import 'package:bloom/native/core_ffi.dart';
import 'package:bloom/native/messages/auth.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

class RegistrationVerifyArguments {
  RegistrationVerifyArguments(this.id, this.email);

  final String id;
  final String email;
}

class RegistrationVerifyView extends StatefulWidget {
  const RegistrationVerifyView({Key key}) : super(key: key);

  @override
  _RegistrationVerifyViewState createState() => _RegistrationVerifyViewState();
}

class _RegistrationVerifyViewState extends State<RegistrationVerifyView> {
  TextStyle style = const TextStyle(
      fontFamily: 'Montserrat',
      fontSize: 20.0,
      color: Colors.white,
      fontWeight: FontWeight.bold);
  TextEditingController codeController =
      MaskedTextController(mask: '0000-0000');
  bool isLoading = false;
  String pendingAccountEmail;
  String pendingAccountId;

  @override
  Widget build(BuildContext context) {
    final RegistrationVerifyArguments args =
        ModalRoute.of(context).settings.arguments;
    pendingAccountId = args.id;
    pendingAccountEmail = args.email;

    return Scaffold(
      body: _buildBody(context),
    );
  }

  Container _buildBody(BuildContext context) {
    return Container(
      child: Padding(
        padding: const EdgeInsets.all(36.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.center,
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            TextField(
              maxLength: 9,
              controller: codeController,
              decoration: const InputDecoration(labelText: 'Verifycation code'),
              keyboardType: TextInputType.number,
            ),
            const SizedBox(
              height: 35.0,
            ),
            _buildVerifyButton(context),
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

  Material _buildVerifyButton(BuildContext context) {
    return Material(
      elevation: 5.0,
      borderRadius: BorderRadius.circular(6.0),
      color: Colors.blue,
      child: MaterialButton(
        minWidth: MediaQuery.of(context).size.width,
        padding: const EdgeInsets.fromLTRB(20.0, 15.0, 20.0, 15.0),
        onPressed: _onVerifyButtonPressed,
        child: Text(
          'Verify code',
          textAlign: TextAlign.center,
          style: style,
        ),
      ),
    );
  }

  Future<void> _onVerifyButtonPressed() async {
    setState(() {
      isLoading = true;
    });

    final String code = _cleanCode(codeController.text);

    final String message = jsonEncode(AuthGuiRegistrationVerify(
      id: pendingAccountId,
      code: code,
    ));

    final String res =
        await compute(_RegistrationVerifyViewState._nativeCall, message);
    debugPrint(res);
    final RegistrationCompleteArguments args =
        RegistrationCompleteArguments(pendingAccountId);

    setState(() {
      isLoading = false;
    });
    Navigator.pushNamed(context, '/auth/registration/complete',
        arguments: args);
  }

  static String _nativeCall(String message) {
    return coreFfi.call(message);
  }

  String _cleanCode(String code) {
    return code.substring(0, 4) + code.substring(5);
  }
}
