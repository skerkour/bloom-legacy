import 'package:bloom/ui/home/views/tab_me.dart';
import 'package:flutter/material.dart';

class HomeView extends StatefulWidget {
  const HomeView({Key key}) : super(key: key);

  @override
  _HomeViewState createState() => _HomeViewState();
}

class _HomeViewState extends State<HomeView> {

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: _buildAppBar(context),
      body: const TabMeView(),
    );
  }

  AppBar _buildAppBar(BuildContext context) {
    final ThemeData theme = Theme.of(context);
    final List<IconButton> actions = <IconButton>[];

    // switch (_selectedIndex) {
    //   // case 0:
    //   //   actions.add(IconButton(
    //   //       icon: Icon(Icons.search),
    //   //       onPressed: () {
    //   //         showSearch<String>(
    //   //           context: context,
    //   //           delegate: ChatsSearchDelegate(),
    //   //         );
    //   //       }));
    //   //   break;
    //   case 1:
    //     actions.add(IconButton(
    //         icon: Icon(Icons.search),
    //         onPressed: () {
    //           showSearch<String>(
    //             context: context,
    //             delegate: ChatsSearchDelegate(),
    //           );
    //         }));
    //     break;
    // }

    return AppBar(
      elevation: 0,
      backgroundColor: theme.scaffoldBackgroundColor,
      iconTheme: IconThemeData(color: Colors.grey[700]),
      title: Text('Bloom', style: TextStyle(color: Colors.grey[700])),
      actions: actions,
    );
  }
}
