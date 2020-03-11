import 'package:bloom/ui/auth/blocs/registration_start.dart';
import 'package:bloom/core/users/messages.dart';
import 'package:bloom/ui/auth/views/registration_verify.dart';
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
  TextEditingController emailController = TextEditingController();
  TextEditingController displayNameController = TextEditingController();
  RegistrationStartBloc _bloc;

  @override
  void initState() {
    _bloc = RegistrationStartBloc();
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
                  _buildRegisterButton(context, isLoading),
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

  Material _buildRegisterButton(BuildContext context, bool isLoading) {
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
    final String email = emailController.text;
    final AuthRegistrationStarted messageRes =
        await _bloc.start(displayNameController.text, email);
    final RegistrationVerifyArguments args =
        RegistrationVerifyArguments(messageRes.id, email);

    Navigator.pushNamed(context, '/auth/registration/verify', arguments: args);
  }
}
