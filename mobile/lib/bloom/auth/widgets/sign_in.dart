import 'package:bloom/bloom/kernel/widgets/password_field.dart';
import 'package:flutter/material.dart';

class SignIn extends StatefulWidget {
  const SignIn({Key key}) : super(key: key);

  @override
  _SignInState createState() => _SignInState();
}

class _SignInState extends State<SignIn> {
  TextStyle style = const TextStyle(fontFamily: 'Montserrat', fontSize: 20.0);

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
            ),
            const SizedBox(height: 25.0),
            const PasswordField(
              labelText: 'Password',
            ),
            const SizedBox(
              height: 35.0,
            ),
            _buildSignInButton(context),
            const SizedBox(
              height: 15.0,
            ),
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
        onPressed: () {},
        child: Text('Sign in',
            textAlign: TextAlign.center,
            style: style.copyWith(
                color: Colors.white, fontWeight: FontWeight.bold)),
      ),
    );
  }
}
