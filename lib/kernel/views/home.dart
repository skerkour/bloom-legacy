import 'package:bloom/kernel/views/tab_chats.dart';
import 'package:bloom/kernel/views/tab_discover.dart';
import 'package:bloom/kernel/views/tab_groups.dart';
import 'package:bloom/kernel/views/tab_me.dart';
import 'package:bloom/kernel/widgets/bottom_sheet_chats.dart';
import 'package:flutter/material.dart';

class HomeView extends StatefulWidget {
  const HomeView({Key key}) : super(key: key);

  @override
  _HomeViewState createState() => _HomeViewState();
}

class _HomeViewState extends State<HomeView> {
  int _selectedIndex = 0;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: _buildAppBar(context),
      body: _buildBody(context),
      bottomNavigationBar: _buildBottomNavigationBar(),
      floatingActionButton: _buildFloatingActionButton(context),
    );
  }

  AppBar _buildAppBar(BuildContext context) {
    final ThemeData theme = Theme.of(context);
    return AppBar(
      elevation: 0,
      backgroundColor: theme.scaffoldBackgroundColor,
      iconTheme: IconThemeData(color: Colors.grey[700]),
      title: Text('Bloom', style: TextStyle(color: Colors.grey[700])),
      actions: <Widget>[
        IconButton(
          icon: Icon(Icons.search),
          onPressed: () => debugPrint('Search pressed'),
        ),
      ],
    );
  }

  FloatingActionButton _buildFloatingActionButton(BuildContext context) {
    switch (_selectedIndex) {
      case 0:
        return FloatingActionButton(
          onPressed: () => _fabChatsPressed(),
          child: Icon(Icons.add),
          backgroundColor: Colors.red,
        );
      case 1:
        return FloatingActionButton(
          onPressed: () => _fabGroupsPressed(),
          child: Icon(Icons.add),
          backgroundColor: Colors.red,
        );
      default:
        return null;
    }
  }

  void _fabChatsPressed() {
    showModalBottomSheet<ChatsBottomSheet>(
      context: context,
      builder: (BuildContext ctx) {
        return ChatsBottomSheet();
      },
    );
  }

  void _fabGroupsPressed() {
    debugPrint('FAB Groups pressed');
  }

  BottomNavigationBar _buildBottomNavigationBar() {
    return BottomNavigationBar(
      items: const <BottomNavigationBarItem>[
        BottomNavigationBarItem(
          icon: Icon(Icons.chat),
          title: Text('Chats'),
        ),
        BottomNavigationBarItem(
          icon: Icon(Icons.people),
          title: Text('Groups'),
        ),
        BottomNavigationBarItem(
          icon: Icon(Icons.explore),
          title: Text('Discover'),
        ),
        BottomNavigationBarItem(
          icon: Icon(Icons.person),
          title: Text('Me'),
        ),
      ],
      currentIndex: _selectedIndex,
      selectedItemColor: Colors.blue,
      onTap: _onItemTapped,
      type: BottomNavigationBarType.fixed,
    );
  }

  Widget _buildBody(BuildContext context) {
    switch (_selectedIndex) {
      case 0:
        return const TabChatsView();
      case 1:
        return const TabGroupsView();
      case 2:
        return const TabDiscoverView();
      default:
        return const TabMeView();
    }
  }

  void _onItemTapped(int index) {
    setState(() {
      _selectedIndex = index;
    });
  }
}
