import 'dart:convert';
import 'package:bloom/bloom/kernel/widgets/password_field.dart';
import 'package:bloom/native/core_ffi.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import '../messages.dart';

class SignIn extends StatefulWidget {
  const SignIn({Key key}) : super(key: key);

  @override
  _SignInState createState() => _SignInState();
}

class _SignInState extends State<SignIn> {
  TextStyle style = const TextStyle(fontFamily: 'Montserrat', fontSize: 20.0);
  bool isLoading = false;
  TextEditingController usernameController = TextEditingController();
  TextEditingController passwordController = TextEditingController();

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
              decoration: const InputDecoration(labelText: 'Username'),
              controller: usernameController,
            ),
            const SizedBox(height: 25.0),
            PasswordField(
              labelText: 'Password',
              controller: passwordController,
            ),
            const SizedBox(
              height: 35.0,
            ),
            _buildSignInButton(context),
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

  Material _buildSignInButton(BuildContext context) {
    return Material(
      elevation: 5.0,
      borderRadius: BorderRadius.circular(6.0),
      color: Colors.blue,
      child: MaterialButton(
        minWidth: MediaQuery.of(context).size.width,
        padding: const EdgeInsets.fromLTRB(20.0, 15.0, 20.0, 15.0),
        onPressed: isLoading ? null : _onSignInButtonPressed,
        child: Text(isLoading ? 'Signing in...' : 'Sign in',
            textAlign: TextAlign.center,
            style: style.copyWith(
                color: Colors.white, fontWeight: FontWeight.bold)),
      ),
    );
  }

  Future<void> _onSignInButtonPressed() async {
    setState(() {
      isLoading = true;
    });

    final String message = jsonEncode(AuthGuiSignIn(
      username: usernameController.text,
      password: passwordController.text,
    ));

    try {
      final Map<String, dynamic> json =
          await compute(_SignInState._nativeCall, message);
      debugPrint('$json');
    } catch (err) {
      Scaffold.of(context)
        ..removeCurrentSnackBar()
        ..showSnackBar(
          SnackBar(
              content: Text(err.toString(),
                  style: TextStyle(color: Colors.red[300]))),
        );
    } finally {
      setState(() {
        isLoading = false;
      });
    }
  }

  static Map<String, dynamic> _nativeCall(String message) {
    return coreFfi.call(message);
  }
}
