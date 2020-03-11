import 'package:bloom/ui/auth/widgets/registration_start.dart';
import 'package:bloom/ui/auth/widgets/sign_in.dart';
import 'package:flutter/material.dart';

class AuthView extends StatefulWidget {
  const AuthView({Key key}) : super(key: key);

  @override
  _AuthViewState createState() => _AuthViewState();
}

class _AuthViewState extends State<AuthView> {
  @override
  Widget build(BuildContext context) {
    return DefaultTabController(
      length: 2,
      child: Scaffold(
        appBar: PreferredSize(
          preferredSize: Size.fromHeight(kToolbarHeight),
          child: Container(
            margin: const EdgeInsets.only(top: 25.0),
            height: 50.0,
            child: const TabBar(
              labelColor: Colors.black,
              tabs: <Widget>[
                Tab(text: 'Register'),
                Tab(text: 'Sign in'),
              ],
            ),
          ),
        ),
        body: _buildBody(),
        // resizeToAvoidBottomInset: false,
      ),
    );
  }

  Widget _buildBody() {
    return const TabBarView(
      children: <Widget>[
        RegistrationStart(),
        SignIn(),
      ],
    );
  }
}
