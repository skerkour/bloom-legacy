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
      appBar: AppBar(
        title: const Text('Bloom'),
      ),
      body: _buildBody(context),
      bottomNavigationBar: _buildBottomNavigationBar(),
    );
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
      unselectedItemColor: Colors.black,
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
