import 'dart:convert';
import 'package:bloom/bloom/kernel/widgets/password_field.dart';
import 'package:bloom/native/core_ffi.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import '../messages.dart';

class RegistrationCompleteArguments {
  RegistrationCompleteArguments(this.id);

  final String id;
}

class RegistrationCompleteView extends StatefulWidget {
  const RegistrationCompleteView({Key key}) : super(key: key);

  @override
  _RegistrationCompleteViewState createState() =>
      _RegistrationCompleteViewState();
}

class _RegistrationCompleteViewState extends State<RegistrationCompleteView> {
  TextStyle style = const TextStyle(
      fontFamily: 'Montserrat',
      fontSize: 20.0,
      color: Colors.white,
      fontWeight: FontWeight.bold);
  TextEditingController usernameController = TextEditingController();
  TextEditingController passwordController = TextEditingController();
  bool isLoading = false;
  String pendingAccountId;

  @override
  Widget build(BuildContext context) {
    final RegistrationCompleteArguments args =
        ModalRoute.of(context).settings.arguments;
    pendingAccountId = args.id;

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
            TextFormField(
              controller: usernameController,
              decoration: const InputDecoration(labelText: 'Username'),
            ),
            const SizedBox(
              height: 35.0,
            ),
            PasswordField(
              controller: passwordController,
              labelText: 'Password',
            ),
            const SizedBox(
              height: 35.0,
            ),
            _buildCompleteButton(context),
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

  Material _buildCompleteButton(BuildContext context) {
    return Material(
      elevation: 5.0,
      borderRadius: BorderRadius.circular(6.0),
      color: Colors.blue,
      child: MaterialButton(
        minWidth: MediaQuery.of(context).size.width,
        padding: const EdgeInsets.fromLTRB(20.0, 15.0, 20.0, 15.0),
        onPressed: isLoading ? null : _onCompleteButtonPressed,
        child: Text(
          'Complete registration',
          textAlign: TextAlign.center,
          style: style,
        ),
      ),
    );
  }

  Future<void> _onCompleteButtonPressed() async {
    setState(() {
      isLoading = true;
    });

    final String message = jsonEncode(AuthGuiRegistrationComplete(
      id: pendingAccountId,
      username: usernameController.text,
      password: passwordController.text,
    ));

    final Map<String, dynamic> res =
        await compute(_RegistrationCompleteViewState._nativeCall, message);
    debugPrint('$res');

    setState(() {
      isLoading = false;
    });
    Navigator.pushNamedAndRemoveUntil(
      context,
      '/',
      (Route<dynamic> route) => false,
    );
  }

  static Map<String, dynamic> _nativeCall(String message) {
    return coreFfi.call(message);
  }
}
