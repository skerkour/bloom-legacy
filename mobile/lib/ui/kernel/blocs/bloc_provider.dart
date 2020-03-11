// // Generic Interface for all BLoCs
// import 'package:flutter/material.dart';

// abstract class BlocBase {
//   void dispose();
// }

// // Generic BLoC provider
// class BlocProvider<T extends BlocBase> extends StatefulWidget {
//   const BlocProvider({
//     Key key,
//     @required this.child,
//     @required this.bloc,
//   }) : super(key: key);

//   final T bloc;
//   final Widget child;

//   @override
//   _BlocProviderState<T> createState() => _BlocProviderState<T>();

//   static T of<T extends BlocBase>(BuildContext context) {
//     final Type type = _typeOf<BlocProvider<T>>();
//     final BlocProvider<T> provider = context.ancestorWidgetOfExactType(type);
//     return provider.bloc;
//   }

//   static Type _typeOf<T>() => T;
// }

// class _BlocProviderState<T> extends State<BlocProvider<BlocBase>> {
//   @override
//   void dispose() {
//     widget.bloc.dispose();
//     super.dispose();
//   }

//   @override
//   Widget build(BuildContext context) {
//     return widget.child;
//   }
// }
import 'package:flutter/material.dart';

Type _typeOf<T>() => T;

abstract class BlocBase {
  void dispose();
}

class BlocProvider<T extends BlocBase> extends StatefulWidget {
  const BlocProvider({
    Key key,
    @required this.child,
    @required this.bloc,
  }) : super(key: key);

  final Widget child;
  final T bloc;

  @override
  _BlocProviderState<T> createState() => _BlocProviderState<T>();

  static T of<T extends BlocBase>(BuildContext context) {
    final Type type = _typeOf<_BlocProviderInherited<T>>();

    final _BlocProviderInherited<T> provider =
        // ignore: deprecated_member_use
        context.ancestorInheritedElementForWidgetOfExactType(type)?.widget;
    return provider?.bloc;
  }
}

class _BlocProviderState<T extends BlocBase> extends State<BlocProvider<T>> {
  @override
  void dispose() {
    widget.bloc?.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return _BlocProviderInherited<T>(
      bloc: widget.bloc,
      child: widget.child,
    );
  }
}

class _BlocProviderInherited<T> extends InheritedWidget {
  const _BlocProviderInherited({
    Key key,
    @required Widget child,
    @required this.bloc,
  }) : super(key: key, child: child);

  final T bloc;

  @override
  bool updateShouldNotify(_BlocProviderInherited<dynamic> oldWidget) => false;
}
