import 'dart:convert';
import 'package:bloom/bloom/auth/views/registration_verify.dart';
import 'package:bloom/native/core_ffi.dart';
import 'package:bloom/native/messages/auth.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

class RegistrationStart extends StatefulWidget {
  const RegistrationStart({Key key}) : super(key: key);

  @override
  _RegistrationStartState createState() => _RegistrationStartState();
}

class _RegistrationStartState extends State<RegistrationStart> {
  TextStyle style = const TextStyle(
      fontFamily: 'Montserrat',
      fontSize: 20.0,
      color: Colors.white,
      fontWeight: FontWeight.bold);
  bool isLoading = false;
  TextEditingController emailController = TextEditingController();
  TextEditingController displayNameController = TextEditingController();

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
              controller: displayNameController,
              decoration: const InputDecoration(labelText: 'Full name'),
            ),
            const SizedBox(height: 25.0),
            TextFormField(
              controller: emailController,
              decoration: const InputDecoration(labelText: 'Email'),
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
        onPressed: isLoading ? null : _onRegisterButtonPressed,
        child: Text(
          'Register',
          textAlign: TextAlign.center,
          style: style,
        ),
      ),
    );
  }

  Future<void> _onRegisterButtonPressed() async {
    setState(() {
      isLoading = true;
    });

    final String email = emailController.text;

    final String message = jsonEncode(AuthRegistrationStart(
      displayName: displayNameController.text,
      email: email,
    ));

    final String json =
        await compute(_RegistrationStartState._nativeCall, message);
    debugPrint(json);
    final Map<String, dynamic> jsonMap = jsonDecode(json);
    final AuthGuiRegistrationStarted messageRes =
        AuthGuiRegistrationStarted.fromJson(jsonMap);
    final RegistrationVerifyArguments args =
        RegistrationVerifyArguments(messageRes.id, email);

    setState(() {
      isLoading = false;
    });
    Navigator.pushNamed(context, '/auth/registration/verify', arguments: args);
  }

  static String _nativeCall(String message) {
    return coreFfi.call(message);
  }
}
