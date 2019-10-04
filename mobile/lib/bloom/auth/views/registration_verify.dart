import 'package:flutter/material.dart';

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
  TextEditingController codeController = TextEditingController();
  bool isLoading = false;

  @override
  Widget build(BuildContext context) {
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
              controller: codeController,
              decoration: const InputDecoration(labelText: 'Verifycation code'),
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
    debugPrint('Verify pressed');
    Navigator.pushNamed(context, '/auth/registration/complete');
  }
}
