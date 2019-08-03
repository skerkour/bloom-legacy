import 'package:bloom/kernel/views/tab_chats.dart';
import 'package:bloom/kernel/views/tab_discover.dart';
import 'package:bloom/kernel/views/tab_groups.dart';
import 'package:bloom/kernel/views/tab_me.dart';
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
      appBar: _buildAppBar(),
      body: _buildBody(context),
      bottomNavigationBar: _buildBottomNavigationBar(),
      floatingActionButton: _buildFloatingActionButton(context),
    );
  }

  AppBar _buildAppBar() {
    String title = '';
    switch (_selectedIndex) {
      case 0:
        title = 'Chats';
        break;
      case 1:
        title = 'Groups';
        break;
      case 2:
        title = 'Discover';
        break;
      default:
        title = 'Me';
    }
    return AppBar(
      title: Text(title),
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
    debugPrint('FAB Chats pressed');
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
