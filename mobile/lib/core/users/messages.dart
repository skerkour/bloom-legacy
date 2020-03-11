import 'package:flutter/material.dart';

class AuthStartRegistration {
  AuthStartRegistration({
    @required this.displayName,
    @required this.email,
  });

  final String displayName;
  final String email;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'displayName': displayName,
      'email': email,
    };
    return data;
  }
}

class AuthRegistrationStarted {
  AuthRegistrationStarted({
    @required this.id,
  });

  final String id;

  static AuthRegistrationStarted fromJson(Map<String, dynamic> json) {
    return AuthRegistrationStarted(id: json['data']['id']);
  }
}

class AuthGuiVerifyRegistration {
  AuthGuiVerifyRegistration({
    @required this.id,
    @required this.code,
  });

  final String id;
  final String code;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'id': id,
      'code': code,
    };
    return data;
  }
}

class AuthGuiCompleteRegistration {
  AuthGuiCompleteRegistration({
    @required this.id,
    @required this.username,
    @required this.password,
  });

  final String id;
  final String username;
  final String password;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'id': id,
      'username': username,
      'password': password,
    };
    return data;
  }
}

class AuthSignIn {
  AuthSignIn({
    @required this.username,
    @required this.password,
  });

  final String username;
  final String password;

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'username': username,
      'password': password,
    };
    return data;
  }
}

class AuthSignOut {
  AuthSignOut();

  Map<String, dynamic> toJson() {
    final Map<String, dynamic> data = <String, dynamic>{
      'type': 'auth.signOut',
      'data': <String, dynamic>{},
    };
    return data;
  }
}
