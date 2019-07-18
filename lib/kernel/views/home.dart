import 'package:bloom/kernel/blocs/app.dart';
import 'package:bloom/kernel/widgets/drawer.dart';
import 'package:flutter/material.dart';

class HomeView extends StatefulWidget {
  const HomeView({Key key, this.title}) : super(key: key);

  final String title;

  @override
  _HomeViewState createState() => _HomeViewState();
}

class _HomeViewState extends State<HomeView> {
  @override
  void initState() {
    appBloc.setCurrentApp(Apps.HOME);
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const BlmDrawer(),
      appBar: AppBar(
        title: Text(widget.title),
      ),
      body: Center(
        child: const Text('Home'),
      ),
    );
  }
}
