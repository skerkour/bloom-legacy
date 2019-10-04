import 'package:flutter/material.dart';

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
  TextEditingController codeController = TextEditingController();
  bool isLoading = false;

  @override
  Widget build(BuildContext context) {
    return Container(
      child: Padding(
        padding: const EdgeInsets.all(36.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.center,
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            TextFormField(
              controller: codeController,
              decoration: const InputDecoration(labelText: 'Username'),
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
        onPressed: () {
          _onVerifyButtonPressed();
        },
        child: Text(
          'Complete registration',
          textAlign: TextAlign.center,
          style: style,
        ),
      ),
    );
  }

  Future<void> _onVerifyButtonPressed() async {
    debugPrint('Verify pressed');
  }
}
