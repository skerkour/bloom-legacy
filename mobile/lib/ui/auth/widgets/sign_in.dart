import 'package:bloom/ui/auth/blocs/sign_in.dart';
import 'package:bloom/ui/kernel/widgets/password_field.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

class SignIn extends StatefulWidget {
  const SignIn({Key key}) : super(key: key);

  @override
  _SignInState createState() => _SignInState();
}

class _SignInState extends State<SignIn> {
  TextStyle style = const TextStyle(fontFamily: 'Montserrat', fontSize: 20.0);
  TextEditingController usernameController = TextEditingController();
  TextEditingController passwordController = TextEditingController();
  SignInBloc _bloc;

  @override
  void initState() {
    _bloc = SignInBloc();
    super.initState();
  }

  @override
  void dispose() {
    _bloc.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return StreamBuilder<bool>(
        initialData: false,
        stream: _bloc.isLoadingStream,
        builder: (BuildContext context, AsyncSnapshot<bool> snapshot) {
          bool isLoading = false;
          if (snapshot.hasData) {
            isLoading = snapshot.data;
          }
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
                  _buildSignInButton(context, isLoading),
                  const SizedBox(
                    height: 15.0,
                  ),
                  isLoading
                      ? const CircularProgressIndicator()
                      : Container(
                          width: 0,
                          height: 0), // TODO(z0mbie42): remove ugly hack
                ],
              ),
            ),
          );
        });
  }

  Material _buildSignInButton(BuildContext context, bool isLoading) {
    return Material(
      elevation: 5.0,
      borderRadius: BorderRadius.circular(6.0),
      color: Colors.blue,
      child: MaterialButton(
        minWidth: MediaQuery.of(context).size.width,
        padding: const EdgeInsets.fromLTRB(20.0, 15.0, 20.0, 15.0),
        onPressed: isLoading ? null : _onSignInButtonPressed,
        child: Text(
          isLoading ? 'Signing in...' : 'Sign in',
          textAlign: TextAlign.center,
          style:
              style.copyWith(color: Colors.white, fontWeight: FontWeight.bold),
        ),
      ),
    );
  }

  // TODO(z0mbie42): sign in
  Future<void> _onSignInButtonPressed() async {
    try {
      await _bloc.signIn(usernameController.text, passwordController.text);
    } catch (err) {
      Scaffold.of(context)
        ..removeCurrentSnackBar()
        ..showSnackBar(
          SnackBar(
            content: Text(
              err.toString(),
              style: TextStyle(color: Colors.red[300]),
            ),
          ),
        );
    }
  }
}
