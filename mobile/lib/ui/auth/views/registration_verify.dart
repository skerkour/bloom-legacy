import 'package:bloom/ui/auth/blocs/registration_verify.dart';
import 'package:bloom/ui/auth/views/registration_complete.dart';
import 'package:bloom/libs/masked_text_controller.dart';
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
  String pendingAccountEmail;
  String pendingAccountId;
  RegistrationVerifyBloc _bloc;

  @override
  void initState() {
    _bloc = RegistrationVerifyBloc();
    super.initState();
  }

  @override
  void dispose() {
    _bloc.dispose();
    super.dispose();
  }

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
                  TextField(
                    maxLength: 9,
                    controller: codeController,
                    decoration:
                        const InputDecoration(labelText: 'Verifycation code'),
                    keyboardType: TextInputType.number,
                  ),
                  const SizedBox(
                    height: 35.0,
                  ),
                  _buildVerifyButton(context, isLoading),
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

  Material _buildVerifyButton(BuildContext context, bool isLoading) {
    return Material(
      elevation: 5.0,
      borderRadius: BorderRadius.circular(6.0),
      color: Colors.blue,
      child: MaterialButton(
        minWidth: MediaQuery.of(context).size.width,
        padding: const EdgeInsets.fromLTRB(20.0, 15.0, 20.0, 15.0),
        onPressed: isLoading ? null : _onVerifyButtonPressed,
        child: Text(
          'Verify code',
          textAlign: TextAlign.center,
          style: style,
        ),
      ),
    );
  }

  Future<void> _onVerifyButtonPressed() async {
    await _bloc.verify(pendingAccountId, codeController.text);
    final RegistrationCompleteArguments args =
        RegistrationCompleteArguments(pendingAccountId);
    Navigator.pushNamed(context, '/auth/registration/complete',
        arguments: args);
  }
}
