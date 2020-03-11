import 'package:bloom/ui/auth/blocs/registration_complete.dart';
import 'package:bloom/ui/kernel/widgets/password_field.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

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
  String pendingAccountId;
  RegistrationCompleteBloc _bloc;

  @override
  void initState() {
    _bloc = RegistrationCompleteBloc();
    super.initState();
  }

  @override
  void dispose() {
    _bloc.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final RegistrationCompleteArguments args =
        ModalRoute.of(context).settings.arguments;
    pendingAccountId = args.id;

    return Scaffold(
      body: _buildBody(context),
    );
  }

  Widget _buildBody(BuildContext context) {
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
                  _buildCompleteButton(context, isLoading),
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

  Material _buildCompleteButton(BuildContext context, bool isLoading) {
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
    _bloc.complete(
      pendingAccountId,
      usernameController.text,
      passwordController.text,
    );

    Navigator.pushNamedAndRemoveUntil(
      context,
      '/',
      (Route<dynamic> route) => false,
    );
  }
}
